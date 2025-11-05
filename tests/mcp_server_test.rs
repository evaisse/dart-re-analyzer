use dart_re_analyzer::error::{Diagnostic, Location, RuleCategory, Severity};
use dart_re_analyzer::mcp::{DiagnosticsStats, ErrorsQuery, McpRequest, McpServer};
use serde_json::json;

#[tokio::test]
async fn test_mcp_server_creation() {
    let mcp = McpServer::new();
    let diagnostics = mcp.get_all_diagnostics().await;
    assert_eq!(diagnostics.len(), 0);
}

#[tokio::test]
async fn test_mcp_update_diagnostics() {
    let mcp = McpServer::new();

    let diag = Diagnostic {
        location: Location {
            file: "test.dart".to_string(),
            line: 10,
            column: 5,
            end_line: None,
            end_column: None,
        },
        severity: Severity::Error,
        category: RuleCategory::Runtime,
        rule_id: "test_rule".to_string(),
        message: "Test error".to_string(),
        suggestion: Some("Fix it".to_string()),
    };

    mcp.update_diagnostics(vec![diag.clone()]).await;

    let diagnostics = mcp.get_all_diagnostics().await;
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id, "test_rule");
}

#[tokio::test]
async fn test_mcp_get_all_errors_request() {
    let mcp = McpServer::new();

    let diag1 = create_test_diagnostic(
        "test1.dart",
        Severity::Error,
        RuleCategory::Runtime,
        "rule1",
    );
    let diag2 = create_test_diagnostic(
        "test2.dart",
        Severity::Warning,
        RuleCategory::Style,
        "rule2",
    );

    mcp.update_diagnostics(vec![diag1, diag2]).await;

    let request = McpRequest {
        method: "get_all_errors".to_string(),
        params: json!({}),
    };

    let response = mcp.handle_request(request).await;
    assert!(response.success);
    assert!(response.error.is_none());

    let diagnostics: Vec<Diagnostic> = serde_json::from_value(response.data).unwrap();
    assert_eq!(diagnostics.len(), 2);
}

#[tokio::test]
async fn test_mcp_get_errors_filtered_by_category() {
    let mcp = McpServer::new();

    let diag1 = create_test_diagnostic(
        "test1.dart",
        Severity::Error,
        RuleCategory::Runtime,
        "rule1",
    );
    let diag2 = create_test_diagnostic(
        "test2.dart",
        Severity::Warning,
        RuleCategory::Style,
        "rule2",
    );
    let diag3 = create_test_diagnostic(
        "test3.dart",
        Severity::Error,
        RuleCategory::Runtime,
        "rule3",
    );

    mcp.update_diagnostics(vec![diag1, diag2, diag3]).await;

    let request = McpRequest {
        method: "get_errors".to_string(),
        params: json!({
            "category": "runtime"
        }),
    };

    let response = mcp.handle_request(request).await;
    assert!(response.success);

    let diagnostics: Vec<Diagnostic> = serde_json::from_value(response.data).unwrap();
    assert_eq!(diagnostics.len(), 2);
    assert!(diagnostics
        .iter()
        .all(|d| matches!(d.category, RuleCategory::Runtime)));
}

#[tokio::test]
async fn test_mcp_get_errors_filtered_by_severity() {
    let mcp = McpServer::new();

    let diag1 = create_test_diagnostic(
        "test1.dart",
        Severity::Error,
        RuleCategory::Runtime,
        "rule1",
    );
    let diag2 = create_test_diagnostic(
        "test2.dart",
        Severity::Warning,
        RuleCategory::Style,
        "rule2",
    );
    let diag3 = create_test_diagnostic("test3.dart", Severity::Info, RuleCategory::Style, "rule3");

    mcp.update_diagnostics(vec![diag1, diag2, diag3]).await;

    let request = McpRequest {
        method: "get_errors".to_string(),
        params: json!({
            "severity": "warning"
        }),
    };

    let response = mcp.handle_request(request).await;
    assert!(response.success);

    let diagnostics: Vec<Diagnostic> = serde_json::from_value(response.data).unwrap();
    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics
        .iter()
        .all(|d| matches!(d.severity, Severity::Warning)));
}

#[tokio::test]
async fn test_mcp_get_errors_filtered_by_file() {
    let mcp = McpServer::new();

    let diag1 = create_test_diagnostic(
        "lib/main.dart",
        Severity::Error,
        RuleCategory::Runtime,
        "rule1",
    );
    let diag2 = create_test_diagnostic(
        "lib/utils.dart",
        Severity::Warning,
        RuleCategory::Style,
        "rule2",
    );
    let diag3 = create_test_diagnostic(
        "lib/main.dart",
        Severity::Error,
        RuleCategory::Runtime,
        "rule3",
    );

    mcp.update_diagnostics(vec![diag1, diag2, diag3]).await;

    let request = McpRequest {
        method: "get_errors".to_string(),
        params: json!({
            "file": "main.dart"
        }),
    };

    let response = mcp.handle_request(request).await;
    assert!(response.success);

    let diagnostics: Vec<Diagnostic> = serde_json::from_value(response.data).unwrap();
    assert_eq!(diagnostics.len(), 2);
    assert!(diagnostics
        .iter()
        .all(|d| d.location.file.contains("main.dart")));
}

#[tokio::test]
async fn test_mcp_get_errors_multiple_filters() {
    let mcp = McpServer::new();

    let diag1 = create_test_diagnostic(
        "lib/main.dart",
        Severity::Error,
        RuleCategory::Runtime,
        "rule1",
    );
    let diag2 = create_test_diagnostic(
        "lib/main.dart",
        Severity::Warning,
        RuleCategory::Runtime,
        "rule2",
    );
    let diag3 = create_test_diagnostic(
        "lib/utils.dart",
        Severity::Error,
        RuleCategory::Runtime,
        "rule3",
    );
    let diag4 = create_test_diagnostic(
        "lib/main.dart",
        Severity::Error,
        RuleCategory::Style,
        "rule4",
    );

    mcp.update_diagnostics(vec![diag1, diag2, diag3, diag4])
        .await;

    let request = McpRequest {
        method: "get_errors".to_string(),
        params: json!({
            "category": "runtime",
            "severity": "error",
            "file": "main.dart"
        }),
    };

    let response = mcp.handle_request(request).await;
    assert!(response.success);

    let diagnostics: Vec<Diagnostic> = serde_json::from_value(response.data).unwrap();
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id, "rule1");
}

#[tokio::test]
async fn test_mcp_get_stats() {
    let mcp = McpServer::new();

    let diag1 = create_test_diagnostic(
        "test1.dart",
        Severity::Error,
        RuleCategory::Runtime,
        "rule1",
    );
    let diag2 = create_test_diagnostic(
        "test2.dart",
        Severity::Warning,
        RuleCategory::Style,
        "rule2",
    );
    let diag3 = create_test_diagnostic("test2.dart", Severity::Info, RuleCategory::Style, "rule3");
    let diag4 = create_test_diagnostic(
        "test3.dart",
        Severity::Error,
        RuleCategory::Runtime,
        "rule4",
    );

    mcp.update_diagnostics(vec![diag1, diag2, diag3, diag4])
        .await;

    let request = McpRequest {
        method: "get_stats".to_string(),
        params: json!({}),
    };

    let response = mcp.handle_request(request).await;
    assert!(response.success);

    let stats: DiagnosticsStats = serde_json::from_value(response.data).unwrap();
    assert_eq!(stats.total, 4);
    assert_eq!(stats.errors, 2);
    assert_eq!(stats.warnings, 1);
    assert_eq!(stats.info, 1);
    assert_eq!(stats.runtime_issues, 2);
    assert_eq!(stats.style_issues, 2);
    assert_eq!(stats.files_with_issues, 3);
}

#[tokio::test]
async fn test_mcp_unknown_method() {
    let mcp = McpServer::new();

    let request = McpRequest {
        method: "unknown_method".to_string(),
        params: json!({}),
    };

    let response = mcp.handle_request(request).await;
    assert!(!response.success);
    assert!(response.error.is_some());
    assert!(response.error.unwrap().contains("Unknown method"));
}

#[tokio::test]
async fn test_mcp_invalid_query_params() {
    let mcp = McpServer::new();

    let request = McpRequest {
        method: "get_errors".to_string(),
        params: json!("invalid"),
    };

    let response = mcp.handle_request(request).await;
    assert!(!response.success);
    assert!(response.error.is_some());
    assert!(response.error.unwrap().contains("Invalid query parameters"));
}

#[tokio::test]
async fn test_diagnostics_stats_from_empty() {
    let diagnostics: Vec<Diagnostic> = vec![];
    let stats = DiagnosticsStats::from_diagnostics(&diagnostics);

    assert_eq!(stats.total, 0);
    assert_eq!(stats.errors, 0);
    assert_eq!(stats.warnings, 0);
    assert_eq!(stats.info, 0);
    assert_eq!(stats.runtime_issues, 0);
    assert_eq!(stats.style_issues, 0);
    assert_eq!(stats.files_with_issues, 0);
}

#[tokio::test]
async fn test_filtered_diagnostics_query() {
    let mcp = McpServer::new();

    let diag1 =
        create_test_diagnostic("test.dart", Severity::Error, RuleCategory::Runtime, "rule1");
    let diag2 =
        create_test_diagnostic("test.dart", Severity::Warning, RuleCategory::Style, "rule2");

    mcp.update_diagnostics(vec![diag1, diag2]).await;

    let query = ErrorsQuery {
        category: Some("runtime".to_string()),
        severity: None,
        file: None,
    };

    let filtered = mcp.get_filtered_diagnostics(&query).await;
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].rule_id, "rule1");
}

// Helper function to create test diagnostics
fn create_test_diagnostic(
    file: &str,
    severity: Severity,
    category: RuleCategory,
    rule_id: &str,
) -> Diagnostic {
    Diagnostic {
        location: Location {
            file: file.to_string(),
            line: 10,
            column: 5,
            end_line: None,
            end_column: None,
        },
        severity,
        category,
        rule_id: rule_id.to_string(),
        message: format!("Test message for {}", rule_id),
        suggestion: Some("Fix it".to_string()),
    }
}
