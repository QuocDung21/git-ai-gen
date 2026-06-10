use std::collections::HashMap;
use std::path::Path;
use crate::models::LanguageStat;

pub fn detect_project_languages<P: AsRef<Path>>(dir: P) -> Vec<LanguageStat> {
    let mut ext_map = HashMap::new();
    let mut total_bytes = 0u64;

    fn traverse_dir(path: &Path, ext_map: &mut HashMap<String, u64>, total_bytes: &mut u64) {
        let entries = match std::fs::read_dir(path) {
            Ok(entries) => entries,
            Err(_) => return,
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };
            let file_type = match entry.file_type() {
                Ok(ft) => ft,
                Err(_) => continue,
            };

            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if name_str.starts_with('.') {
                continue;
            }

            if file_type.is_dir() {
                if name_str == "node_modules"
                    || name_str == "target"
                    || name_str == "build"
                    || name_str == "dist"
                    || name_str == "venv"
                    || name_str == "env"
                    || name_str == "Pods"
                    || name_str == "pods"
                {
                    continue;
                }
                traverse_dir(&entry.path(), ext_map, total_bytes);
            } else if file_type.is_file() {
                if name_str == "Cargo.lock"
                    || name_str == "package-lock.json"
                    || name_str == "pnpm-lock.yaml"
                    || name_str == "yarn.lock"
                {
                    continue;
                }

                let metadata = match entry.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };
                let size = metadata.len();
                if size == 0 {
                    continue;
                }

                if let Some(ext) = entry.path().extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    let is_binary_or_asset = matches!(
                        ext_str.as_str(),
                        "a" | "lib" | "so" | "dylib" | "dll" | "exe" | "bin" | "o" | "obj" | "out" |
                        "png" | "jpg" | "jpeg" | "gif" | "ico" | "svg" | "webp" | "bmp" | "tiff" |
                        "zip" | "tar" | "gz" | "rar" | "7z" | "bz2" | "xz" |
                        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" |
                        "ttf" | "otf" | "woff" | "woff2" | "eot" |
                        "mp3" | "mp4" | "wav" | "avi" | "mov" | "mkv" | "flac" |
                        "db" | "sqlite" | "sqlite3"
                    );
                    if !is_binary_or_asset {
                        *ext_map.entry(ext_str).or_insert(0) += size;
                        *total_bytes += size;
                    }
                }
            }
        }
    }

    traverse_dir(dir.as_ref(), &mut ext_map, &mut total_bytes);

    if total_bytes == 0 {
        return Vec::new();
    }

    let lang_map = [
        ("rs", ("Rust", (222, 90, 56))),
        ("js", ("JavaScript", (241, 224, 90))),
        ("mjs", ("JavaScript", (241, 224, 90))),
        ("cjs", ("JavaScript", (241, 224, 90))),
        ("ts", ("TypeScript", (49, 120, 198))),
        ("mts", ("TypeScript", (49, 120, 198))),
        ("cts", ("TypeScript", (49, 120, 198))),
        ("tsx", ("TypeScript", (49, 120, 198))),
        ("py", ("Python", (53, 114, 165))),
        ("pyw", ("Python", (53, 114, 165))),
        ("go", ("Go", (0, 173, 216))),
        ("c", ("C", (85, 85, 85))),
        ("h", ("C", (85, 85, 85))),
        ("cpp", ("C++", (243, 75, 125))),
        ("hpp", ("C++", (243, 75, 125))),
        ("cc", ("C++", (243, 75, 125))),
        ("cxx", ("C++", (243, 75, 125))),
        ("java", ("Java", (176, 114, 25))),
        ("kt", ("Kotlin", (241, 142, 4))),
        ("kts", ("Kotlin", (241, 142, 4))),
        ("swift", ("Swift", (240, 81, 43))),
        ("html", ("HTML", (227, 76, 38))),
        ("htm", ("HTML", (227, 76, 38))),
        ("css", ("CSS", (86, 61, 124))),
        ("sh", ("Shell", (137, 224, 81))),
        ("bash", ("Shell", (137, 224, 81))),
        ("yml", ("YAML", (203, 76, 96))),
        ("yaml", ("YAML", (203, 76, 96))),
        ("json", ("JSON", (41, 186, 156))),
        ("md", ("Markdown", (8, 63, 116))),
        ("sql", ("SQL", (227, 111, 30))),
        ("php", ("PHP", (79, 91, 147))),
        ("rb", ("Ruby", (112, 21, 22))),
        ("cs", ("C#", (23, 134, 0))),
    ];

    let mut grouped_stats: HashMap<String, (u64, (u8, u8, u8))> = HashMap::new();
    for (ext, bytes) in ext_map {
        let mut matched = false;
        for (pattern, (name, color)) in &lang_map {
            if ext == *pattern {
                let entry = grouped_stats.entry(name.to_string()).or_insert((0, *color));
                entry.0 += bytes;
                matched = true;
                break;
            }
        }
        if !matched {
            let entry = grouped_stats.entry("Other".to_string()).or_insert((0, (140, 140, 140)));
            entry.0 += bytes;
        }
    }

    let mut stats: Vec<LanguageStat> = grouped_stats
        .into_iter()
        .map(|(name, (bytes, color_code))| {
            let percentage = (bytes as f64 / total_bytes as f64) * 100.0;
            LanguageStat {
                name,
                bytes,
                percentage,
                color_code,
            }
        })
        .collect();

    stats.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());
    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_project_languages() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_path = temp_dir.path();

        let rs_file = project_path.join("main.rs");
        let md_file = project_path.join("README.md");
        let a_file = project_path.join("lib.a");
        let node_dir = project_path.join("node_modules");
        std::fs::create_dir(&node_dir).unwrap();
        let js_file = node_dir.join("index.js");

        std::fs::write(&rs_file, "fn main() {}").unwrap();
        std::fs::write(&md_file, "# Readme").unwrap();
        std::fs::write(&a_file, "binary data here which is huge").unwrap();
        std::fs::write(&js_file, "console.log()").unwrap();

        let stats = detect_project_languages(project_path);

        assert_eq!(stats.len(), 2);

        let rust_stat = stats.iter().find(|s| s.name == "Rust").unwrap();
        let md_stat = stats.iter().find(|s| s.name == "Markdown").unwrap();

        assert_eq!(rust_stat.bytes, 12);
        assert_eq!(md_stat.bytes, 8);

        assert!((rust_stat.percentage - 60.0).abs() < 0.001);
        assert!((md_stat.percentage - 40.0).abs() < 0.001);
    }
}
