pub fn resolve(
    path: &str,
    apps: &[(String /* app name */, String /* app path */)],
) -> Option<String> {
    apps.iter()
        .find(|(_, app_path)| path.starts_with(app_path))
        .map(|(app_name, _)| app_name.to_string())
}

#[cfg(test)]
mod tests {
    use crate::app_resolver::resolve;

    #[test]
    fn test_resolve_app() {
        let path = "/awesome/app/v1/hello/world?lang=en";
        let apps = [
            ("app-1".to_string(), "/awesome/app/v1".to_string()),
            ("app-2".to_string(), "/awesome/app/v2".to_string()),
        ];

        assert_eq!("app-1", resolve(path, &apps).unwrap().as_str());
    }
}
