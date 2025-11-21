use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::Serialize;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Clone, Serialize)]
pub struct SearchResult {
    pub name: String,
    pub path: String,
    pub kind: String,
}

static INDEX: Lazy<RwLock<Vec<SearchResult>>> = Lazy::new(|| RwLock::new(Vec::new()));
static MATCHER: Lazy<SkimMatcherV2> = Lazy::new(SkimMatcherV2::default);

// File extensions to index
const DOCUMENT_EXTS: &[&str] = &["txt", "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "rtf"];
const IMAGE_EXTS: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "svg", "webp", "ico"];
const VIDEO_EXTS: &[&str] = &["mp4", "avi", "mkv", "mov", "wmv", "flv", "webm"];
const AUDIO_EXTS: &[&str] = &["mp3", "wav", "flac", "aac", "ogg", "wma", "m4a"];
const CODE_EXTS: &[&str] = &["rs", "js", "ts", "py", "java", "c", "cpp", "h", "cs", "go", "html", "css", "json", "xml", "yaml", "toml"];
const ARCHIVE_EXTS: &[&str] = &["zip", "rar", "7z", "tar", "gz", "bz2"];

fn get_file_kind(extension: &str) -> &'static str {
    let ext = extension.to_lowercase();
    if DOCUMENT_EXTS.contains(&ext.as_str()) {
        "document"
    } else if IMAGE_EXTS.contains(&ext.as_str()) {
        "image"
    } else if VIDEO_EXTS.contains(&ext.as_str()) {
        "video"
    } else if AUDIO_EXTS.contains(&ext.as_str()) {
        "audio"
    } else if CODE_EXTS.contains(&ext.as_str()) {
        "code"
    } else if ARCHIVE_EXTS.contains(&ext.as_str()) {
        "archive"
    } else {
        "file"
    }
}

fn should_index_file(extension: &str) -> bool {
    let ext = extension.to_lowercase();
    DOCUMENT_EXTS.contains(&ext.as_str())
        || IMAGE_EXTS.contains(&ext.as_str())
        || VIDEO_EXTS.contains(&ext.as_str())
        || AUDIO_EXTS.contains(&ext.as_str())
        || CODE_EXTS.contains(&ext.as_str())
        || ARCHIVE_EXTS.contains(&ext.as_str())
}

pub fn init_index() {
    let mut entries = Vec::new();

    // Index Start Menu shortcuts (apps)
    let start_menu_paths = [
        dirs::data_dir().map(|p| p.join("Microsoft\\Windows\\Start Menu\\Programs")),
        Some(PathBuf::from("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs")),
    ];

    for path in start_menu_paths.into_iter().flatten() {
        if path.exists() {
            for entry in WalkDir::new(&path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "lnk" || ext == "exe" {
                        if let Some(name) = path.file_stem() {
                            entries.push(SearchResult {
                                name: name.to_string_lossy().to_string(),
                                path: path.to_string_lossy().to_string(),
                                kind: "app".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    // Index common user directories for files
    let user_dirs = [
        dirs::document_dir(),
        dirs::download_dir(),
        dirs::desktop_dir(),
        dirs::picture_dir(),
        dirs::video_dir(),
        dirs::audio_dir(),
    ];

    for dir in user_dirs.into_iter().flatten() {
        if dir.exists() {
            for entry in WalkDir::new(&dir)
                .max_depth(3)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                let path = entry.path();

                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy();

                    if should_index_file(&ext_str) {
                        if let Some(name) = path.file_stem() {
                            let kind = get_file_kind(&ext_str);
                            entries.push(SearchResult {
                                name: name.to_string_lossy().to_string(),
                                path: path.to_string_lossy().to_string(),
                                kind: kind.to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    *INDEX.write() = entries;
}

pub fn search_apps(query: &str) -> Vec<SearchResult> {
    if query.is_empty() {
        return Vec::new();
    }

    let index = INDEX.read();

    let query_lower = query.to_lowercase();
    let query_normalized = query_lower.replace("_", " ").replace("-", " ");

    let mut results: Vec<(i64, SearchResult)> = index
        .iter()
        .filter_map(|item| {
            let name_lower = item.name.to_lowercase();
            let name_normalized = name_lower.replace("_", " ").replace("-", " ");

            // 1. Check for exact substring match (highest priority)
            let substring_score = if name_lower.contains(&query_lower) {
                Some(10000i64)
            } else if name_normalized.contains(&query_normalized) {
                Some(9000i64)
            } else {
                None
            };

            if let Some(score) = substring_score {
                return Some((score, item.clone()));
            }

            // 2. Try fuzzy match on original name
            let score1 = MATCHER.fuzzy_match(&name_lower, &query_lower);

            // 3. Try fuzzy match on normalized name
            let score2 = MATCHER.fuzzy_match(&name_normalized, &query_normalized);

            // Use the best fuzzy score
            score1.max(score2).map(|score| (score, item.clone()))
        })
        .collect();

    results.sort_by(|a, b| b.0.cmp(&a.0));
    results.into_iter().take(8).map(|(_, r)| r).collect()
}

pub fn get_index_info() -> String {
    let index = INDEX.read();
    format!("Total items: {}\nApps: {}\nFiles: {}",
        index.len(),
        index.iter().filter(|i| i.kind == "app").count(),
        index.iter().filter(|i| i.kind != "app").count()
    )
}

pub fn debug_search(query: &str) -> Vec<SearchResult> {
    let index = INDEX.read();

    // Case-insensitive contains search for debugging
    index
        .iter()
        .filter(|item| {
            item.name.to_lowercase().contains(&query.to_lowercase()) ||
            item.path.to_lowercase().contains(&query.to_lowercase())
        })
        .take(20)
        .cloned()
        .collect()
}
