use crate::error::{Diagnostic, Result};
use std::path::Path;

pub trait Rule: Send + Sync {
    fn name(&self) -> &str;
    fn check(&self, file_path: &Path, content: &str) -> Result<Vec<Diagnostic>>;
}
