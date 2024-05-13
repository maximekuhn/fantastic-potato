use clap::Parser;
use config::Config;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::{error, info};
use uuid::Uuid;

use crate::{args::Args, state::State};

mod app_resolver;
mod args;
mod config;
mod load_balancer;
mod logger;
mod request_parser;
mod request_sender;
mod response_bytes;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init logger
    logger::init();

    // parse args from CLI
    let args = Args::parse();

    // parse and validate config
    let config = Config::new_from_file(&args.config_file_path)?;
    info!(?config);

    // create TCP listener based on the provided config
    let tcp_listener = TcpListener::bind(format!(
        "{}:{}",
        config.server.listen_addr, config.server.listen_port
    ))
    .await?;
    info!("Listening on {:?}", tcp_listener.local_addr()?);

    // create the state using the provided config
    let state = State::from(config);

    // main loop to handle incoming requests
    loop {
        let (tcp_stream, _) = match tcp_listener.accept().await {
            Ok((stream, socket)) => (stream, socket),
            Err(err) => {
                error!("failed to accept incoming TCP stream: {}", err);
                continue;
            }
        };

        // TODO: use a semaphore or any other mechanisms to limit the number of spawned tasks
        // spawn a new tokio task to avoid blocking the main loop
        tokio::spawn(process_request(tcp_stream, state.clone()));
    }
}

#[tracing::instrument(skip_all, fields(req_id = %Uuid::new_v4()))]
async fn process_request(mut tcp_stream: TcpStream, state: State) {
    const BUFFER_SIZE: usize = 1024;

    // read incoming stream into a buffer
    let mut buffer = [0; BUFFER_SIZE];
    let n_read = match tcp_stream.read(&mut buffer).await {
        Ok(n_read) => n_read,
        Err(err) => {
            error!("failed to reay bytes from TCP stream: {}", err);
            return;
        }
    };

    info!("read {} bytes from TCP stream", n_read);

    // XXX: maybe include n_read (0..=n_read)
    let raw_bytes = &buffer[0..n_read];

    // try to parse the HTTP request (if it's actually one)
    let request = match request_parser::parse(raw_bytes) {
        Ok(request) => request,
        Err(err) => {
            error!("failed to parse raw bytes into HTTP request: {}", err);
            return;
        }
    };

    info!(method = %request.method(), uri = %request.uri());

    // resolve the app the request should be redirected to
    let apps = state.apps.read().await;
    let Some(app_name) = app_resolver::resolve(request.uri().to_string().as_str(), &apps) else {
        error!("No app found for given URI");
        return;
    };
    drop(apps);

    info!(%app_name);

    // apply load balancing
    // TODO: if the app only has 1 backend, no need to lock anything
    let Some(load_balancer) = state.load_balancers.get(&app_name) else {
        error!("No load balancer found for given app");
        return;
    };
    let backend_addr = {
        let mut load_balancer = load_balancer.lock().await;
        load_balancer.choose_one()
    };
    info!(%backend_addr);

    // request the backend
    let response = match request_sender::send_request(request, backend_addr).await {
        Ok(response) => response,
        Err(err) => {
            error!("failed to send request to backend: {}", err);
            return;
        }
    };

    // TODO: check if we need to cache the response

    // Send the response to the client
    let raw_response = response_bytes::to_bytes(response);
    if let Err(err) = tcp_stream.write_all(&raw_response).await {
        error!("failed to write response to the client: {}", err);
    }

    info!("sent response to client");
}
