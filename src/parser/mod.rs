use crate::error::Result;
use std::path::Path;

pub struct DartFile {
    pub path: String,
    pub content: String,
}

impl DartFile {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(Self {
            path: path.to_string_lossy().to_string(),
            content,
        })
    }
}

pub fn is_dart_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext == "dart")
        .unwrap_or(false)
}

pub fn find_dart_files(root: &Path) -> Result<Vec<DartFile>> {
    use walkdir::WalkDir;

    let mut files = Vec::new();

    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && is_dart_file(path) {
            // Skip common excluded directories
            let path_str = path.to_string_lossy();
            if path_str.contains("/.dart_tool/")
                || path_str.contains("/build/")
                || path_str.contains("/.pub/")
                || path_str.contains("/packages/")
            {
                continue;
            }

            files.push(DartFile::load(path)?);
        }
    }

    Ok(files)
}
