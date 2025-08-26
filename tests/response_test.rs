#[cfg(test)]
mod tests {
    use afaf_rest_rust::core::rest::handler::response::{build_error_response, build_success_response};
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    struct TestData {
        id: i32,
        name: String,
    }

    #[test]
    fn test_build_error_response() {
        let error_response = build_error_response("not_found", "Resource not found");
        
        assert_eq!(error_response.error, "not_found");
        assert_eq!(error_response.message, "Resource not found");
        assert_eq!(error_response.meta.app, "afaf-rest-rust");
        assert_eq!(error_response.meta.version, "1.0.0");
    }

    #[test]
    fn test_build_success_response() {
        let test_data = TestData {
            id: 1,
            name: String::from("test"),
        };
        
        let success_response = build_success_response(test_data, "Operation successful");
        
        assert_eq!(success_response.message, "Operation successful");
        assert_eq!(success_response.meta.app, "afaf-rest-rust");
        assert_eq!(success_response.meta.version, "1.0.0");
    }
} 