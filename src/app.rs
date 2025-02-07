use std::fs;
use std::path::PathBuf;

pub struct App {
    pub files: Vec<PathBuf>,
    pub selected_file: Option<PathBuf>,
    pub file_content: String,
    pub selected_index: usize,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            files: Vec::new(),
            selected_file: None,
            file_content: String::new(),
            selected_index: 0,
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
            } else {
                self.file_content = "Selected item is not a file".to_string();
            }
        }
    }
}
