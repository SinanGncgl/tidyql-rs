use anyhow::{Context, Result};
use difference::{Changeset, Difference};
use sqlformat::{format, FormatOptions, Indent, QueryParams};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub struct App {
    pub files: Vec<PathBuf>,
    pub selected_file: Option<PathBuf>,
    pub file_content: String,
    pub selected_index: usize,
    pub notification: Option<String>,
    pub formatted_content: Option<String>,
    pub diff_content: Option<String>,
    pub search_query: String,
    pub search_results: Vec<PathBuf>,
    pub is_searching: bool,
    pub current_dir: PathBuf,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut app = App {
            files: Vec::new(),
            selected_file: None,
            file_content: String::new(),
            formatted_content: None,
            selected_index: 0,
            notification: None,
            diff_content: None,
            search_query: String::new(),
            search_results: Vec::new(),
            is_searching: false,
            current_dir: PathBuf::from("."),
        };
        app.update_file_list()?;
        Ok(app)
    }

    fn update_file_list(&mut self) -> Result<()> {
        self.files = fs::read_dir(&self.current_dir)
            .context("Failed to read directory")?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();
        Ok(())
    }

    pub fn search_files(&mut self) -> Result<()> {
        self.search_results = self
            .files
            .iter()
            .filter(|file| file.to_string_lossy().contains(&self.search_query))
            .cloned()
            .collect();
        Ok(())
    }

    pub fn select_file(&mut self, index: usize) -> Result<()> {
        if index < self.files.len() {
            self.selected_file = Some(self.files[index].clone());
            self.read_selected_file()?;
        }
        Ok(())
    }

    fn read_selected_file(&mut self) -> Result<()> {
        if let Some(ref path) = self.selected_file {
            if path.is_file() {
                self.file_content = fs::read_to_string(path)
                    .with_context(|| format!("Failed to read file: {:?}", path))?;
                self.formatted_content = None;
                self.diff_content = None;
            } else {
                self.file_content = "Selected item is not a file".to_string();
            }
        }
        Ok(())
    }

    pub fn format_sql(&mut self) -> Result<()> {
        if let Some(ref path) = self.selected_file {
            if path.extension().and_then(|s| s.to_str()) == Some("sql") {
                let formatted_sql = format(
                    &self.file_content,
                    &QueryParams::None,
                    &FormatOptions {
                        indent: Indent::Spaces(4),
                        uppercase: Some(false),
                        lines_between_queries: 1,
                        ignore_case_convert: None,
                    },
                );
                self.formatted_content = Some(formatted_sql.clone());
                self.notification = Some("SQL formatted successfully".to_string());

                // Generate the diff content
                let changeset = Changeset::new(&self.file_content, &formatted_sql, "\n");
                let mut diff_content = String::new();
                for diff in changeset.diffs {
                    match diff {
                        Difference::Same(ref x) => diff_content.push_str(&format!(" {}\n", x)),
                        Difference::Add(ref x) => diff_content.push_str(&format!("+{}\n", x)),
                        Difference::Rem(ref x) => diff_content.push_str(&format!("-{}\n", x)),
                    }
                }
                self.diff_content = Some(diff_content);
            } else {
                self.notification = Some("Selected file is not an SQL file".to_string());
            }
        }
        Ok(())
    }

    pub fn save_formatted_file(&mut self) -> Result<()> {
        if let Some(ref path) = self.selected_file {
            if let Some(ref formatted_content) = self.formatted_content {
                if path.extension().and_then(|s| s.to_str()) == Some("sql") {
                    let mut file = File::create(path)
                        .with_context(|| format!("Failed to create file: {:?}", path))?;
                    file.write_all(formatted_content.as_bytes())
                        .with_context(|| format!("Failed to write to file: {:?}", path))?;
                    self.notification = Some("File saved successfully".to_string());
                    self.diff_content = None;
                }
            } else {
                self.notification = Some("No formatted content to save".to_string());
            }
        }
        Ok(())
    }

    pub fn navigate_into_folder(&mut self) -> Result<()> {
        if let Some(ref path) = self.selected_file {
            if path.is_dir() {
                self.current_dir = path.clone();
                self.update_file_list()?;
                self.selected_index = 0;
                self.selected_file = None;
            }
        }
        Ok(())
    }

    pub fn navigate_back(&mut self) -> Result<()> {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.update_file_list()?;
            self.selected_index = 0;
            self.selected_file = None;
        }
        Ok(())
    }
}
