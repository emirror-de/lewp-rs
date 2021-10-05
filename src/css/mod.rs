//! CSS modification functions especially required by lewp.

use {
    crate::{
        fh::{FileHierarchy, FileType, Level},
        Error,
    },
    std::{io::Read, path::PathBuf},
};

mod builder;

pub use builder::CssBuilder;

/// Handles CSS specific procedures of [lewp](crate).
pub struct Css {
    fh: FileHierarchy,
    excluded_files: Vec<PathBuf>,
    id: String,
    level: Level,
}

impl Css {
    fn collect_files(&self) -> Vec<PathBuf> {
        let mut css_files = vec![];
        for entry in
            walkdir::WalkDir::new(&self.fh.folder(&self.id, FileType::CSS, self.level.clone()))
        {
            let entry = match entry {
                Ok(v) => v,
                Err(_) => continue,
            };
            let entry = entry.into_path();
            if self.excluded_files.contains(&entry) {
                continue;
            }
            let ext_is_css = match &entry.extension() {
                Some(s) => match s.to_str() {
                    None => false,
                    Some(v) => v == self.fh.extension(FileType::CSS),
                },
                None => false,
            };
            if !ext_is_css {
                continue;
            }
            css_files.push(entry);
        }
        css_files
    }

    fn combine(&self, css_files: Vec<PathBuf>) -> Result<String, Error> {
        let mut css_combined = String::new();
        for css_file_name in css_files {
            let mut css = String::new();
            let mut css_file = match std::fs::File::open(&css_file_name) {
                Ok(c) => c,
                Err(msg) => {
                    return Err(Error::Css(
                        self.level.clone(),
                        self.id.clone(),
                        format!(
                            "Error opening file {}: {}",
                            css_file_name.to_str().unwrap(),
                            msg.to_string()
                        ),
                    ))
                }
            };
            match css_file.read_to_string(&mut css) {
                Ok(_) => (),
                Err(msg) => {
                    return Err(Error::Css(
                        self.level.clone(),
                        self.id.clone(),
                        format!("Error loading stylesheet: {}", msg.to_string(),),
                    ))
                }
            };
            css_combined.push_str(&css);
        }
        Ok(css_combined)
    }
}
