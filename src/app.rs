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
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            files: Vec::new(),
            selected_file: None,
            file_content: String::new(),
            formatted_content: None,
            selected_index: 0,
            notification: None,
        };
        app.update_file_list();
        app
    }

    fn update_file_list(&mut self) {
        self.files = fs::read_dir(".")
            .unwrap()
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();
    }

    pub fn select_file(&mut self, index: usize) {
        if index < self.files.len() {
            self.selected_file = Some(self.files[index].clone());
            self.read_selected_file();
        }
    }

    fn read_selected_file(&mut self) {
        if let Some(ref path) = self.selected_file {
            if path.is_file() {
                self.file_content =
                    fs::read_to_string(path).unwrap_or_else(|_| "Failed to read file".to_string());
                self.formatted_content = None;
            } else {
                self.file_content = "Selected item is not a file".to_string();
            }
        }
    }

    pub fn format_sql(&mut self) {
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
                self.formatted_content = Some(formatted_sql);
                self.notification = Some("SQL formatted successfully".to_string());
            } else {
                self.notification = Some("Selected file is not an SQL file".to_string());
            }
        }
    }

    pub fn save_formatted_file(&mut self) {
        if let Some(ref path) = self.selected_file {
            if let Some(ref formatted_content) = self.formatted_content {
                if path.extension().and_then(|s| s.to_str()) == Some("sql") {
                    let mut file = File::create(path).expect("Failed to create file");
                    file.write_all(formatted_content.as_bytes()).expect("Failed to write to file");
                    self.notification = Some("File saved successfully".to_string());
                }
            } else {
                self.notification = Some("No formatted content to save".to_string());
            }
        }
    }
}