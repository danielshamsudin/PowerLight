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

pub fn init_index() {
    let mut entries = Vec::new();

    // Index Start Menu shortcuts
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

    *INDEX.write() = entries;
}

pub fn search_apps(query: &str) -> Vec<SearchResult> {
    if query.is_empty() {
        return Vec::new();
    }

    let index = INDEX.read();
    let mut results: Vec<(i64, SearchResult)> = index
        .iter()
        .filter_map(|item| {
            MATCHER
                .fuzzy_match(&item.name, query)
                .map(|score| (score, item.clone()))
        })
        .collect();

    results.sort_by(|a, b| b.0.cmp(&a.0));
    results.into_iter().take(8).map(|(_, r)| r).collect()
}
