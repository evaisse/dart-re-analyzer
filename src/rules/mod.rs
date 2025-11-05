pub mod style;
pub mod runtime;

use crate::analyzer::Rule;
use std::sync::Arc;

pub fn get_all_rules() -> Vec<Arc<dyn Rule>> {
    let mut rules: Vec<Arc<dyn Rule>> = Vec::new();
    
    // Style rules
    rules.push(Arc::new(style::CamelCaseClassNameRule));
    rules.push(Arc::new(style::SnakeCaseFileNameRule));
    rules.push(Arc::new(style::PrivateFieldUnderscoreRule));
    rules.push(Arc::new(style::LineLengthRule::new(120)));
    
    // Runtime rules
    rules.push(Arc::new(runtime::AvoidDynamicRule));
    rules.push(Arc::new(runtime::AvoidEmptyCatchRule));
    rules.push(Arc::new(runtime::UnusedImportRule));
    rules.push(Arc::new(runtime::AvoidPrintRule));
    rules.push(Arc::new(runtime::AvoidNullCheckOnNullableRule));
    
    rules
}

pub fn get_style_rules() -> Vec<Arc<dyn Rule>> {
    vec![
        Arc::new(style::CamelCaseClassNameRule),
        Arc::new(style::SnakeCaseFileNameRule),
        Arc::new(style::PrivateFieldUnderscoreRule),
        Arc::new(style::LineLengthRule::new(120)),
    ]
}

pub fn get_runtime_rules() -> Vec<Arc<dyn Rule>> {
    vec![
        Arc::new(runtime::AvoidDynamicRule),
        Arc::new(runtime::AvoidEmptyCatchRule),
        Arc::new(runtime::UnusedImportRule),
        Arc::new(runtime::AvoidPrintRule),
        Arc::new(runtime::AvoidNullCheckOnNullableRule),
    ]
}
