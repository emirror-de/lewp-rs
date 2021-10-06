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
    exclude_files: Vec<PathBuf>,
    id: String,
    level: Level,
}

impl Css {
    fn collect_files(&self) -> Vec<PathBuf> {
        let path = self.fh.folder(&self.id, FileType::CSS, self.level.clone());
        let path = match path.to_str() {
            Some(p) => p,
            None => {
                log::error!("Path does not contain valid unicode!");
                return vec![];
            }
        };
        let mut css_files = self.fh.collect_filenames(path);
        css_files.retain(|e| {
            if self.exclude_files.contains(e) {
                return false;
            }
            match &e.extension() {
                Some(s) => match s.to_str() {
                    None => false,
                    Some(v) => v == self.fh.extension(FileType::CSS),
                },
                None => false,
            }
        });
        css_files
    }

    fn combine(&self, css_files: Vec<PathBuf>) -> Result<String, Error> {
        let mut css_combined = String::new();
        for css_file_name in css_files {
            let mut css = String::new();
            let file_path = self
                .fh
                .folder(&self.id, FileType::CSS, self.level.clone())
                .join(&css_file_name);
            let mut css_file = match std::fs::File::open(&file_path) {
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

    /// Prepares and processes CSS files for given id and level. Returns the
    /// processed CSS as String.
    pub fn process(&self) -> Result<String, Error> {
        let files = self.collect_files();
        let css_raw = self.combine(files)?;
        Ok(String::new())
    }
}

#[test]
fn process_css_files() {
    let dir = tempfile::tempdir().unwrap();
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.copy_inside = true;
    match fs_extra::dir::copy(
        "testfiles/modules",
        dir.path().join("modules"),
        &copy_options,
    ) {
        Err(msg) => panic!("{}", msg.to_string()),
        Ok(_) => (),
    };

    let fh = crate::fh::FileHierarchyBuilder::new()
        .base_directory(dir.path().to_path_buf())
        .build();
    let css = Css {
        fh,
        exclude_files: vec![],
        level: Level::Module,
        id: String::from("hello-world"),
    };
    assert_eq!(css.process().unwrap(), String::new());

    match dir.close() {
        Err(msg) => panic!("{}", msg.to_string()),
        Ok(_) => (),
    }
}
