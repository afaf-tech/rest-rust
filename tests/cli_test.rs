#[cfg(test)]
mod tests {
    use afaf_rest_rust::cmd::cli::{run, CliArgs};

    #[test]
    fn test_cli_run() {
        let args = CliArgs {
            task: String::from("test_task"),
        };
        
        let result = run(args);
        assert!(result.is_ok());
    }
} 