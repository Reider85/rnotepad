use eframe::egui;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

struct NotepadApp {
    tabs: Vec<Tab>,
    current_tab: usize,
    next_tab_id: usize,
}

struct Tab {
    id: usize,
    title: String,
    content: String,
    file_path: Option<PathBuf>,
    modified: bool,
}

impl Tab {
    fn save(&mut self) -> std::io::Result<bool> {
        if let Some(path) = &self.file_path {
            let mut file = File::create(path)?;
            file.write_all(self.content.as_bytes())?;
            self.modified = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn save_as(&mut self) -> std::io::Result<bool> {
        if let Some(path) = rfd::FileDialog::new().save_file() {
            let mut file = File::create(&path)?;
            file.write_all(self.content.as_bytes())?;
            self.file_path = Some(path.clone());
            self.title = path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned();
            self.modified = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl NotepadApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        NotepadApp {
            tabs: vec![Tab {
                id: 0,
                title: "Untitled".to_string(),
                content: "".to_string(),
                file_path: None,
                modified: false,
            }],
            current_tab: 0,
            next_tab_id: 1,
        }
    }

    fn open_file(&mut self, path: PathBuf) -> std::io::Result<()> {
        let mut file_content = String::new();
        File::open(&path)?.read_to_string(&mut file_content)?;
        self.tabs.push(Tab {
            id: self.next_tab_id,
            title: path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            content: file_content,
            file_path: Some(path),
            modified: false,
        });
        self.next_tab_id += 1;
        self.current_tab = self.tabs.len() - 1;
        Ok(())
    }

    fn new_tab(&mut self) {
        self.tabs.push(Tab {
            id: self.next_tab_id,
            title: "Untitled".to_string(),
            content: "".to_string(),
            file_path: None,
            modified: false,
        });
        self.next_tab_id += 1;
        self.current_tab = self.tabs.len() - 1;
    }

    fn close_tab(&mut self, index: usize) {
        self.tabs.remove(index);
        if self.tabs.is_empty() {
            self.new_tab();
        } else if self.current_tab >= index && self.current_tab > 0 {
            self.current_tab -= 1;
        }
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.new_tab();
                    }
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let _ = self.open_file(path);
                        }
                    }
                    if ui.button("Save").clicked() && !self.tabs.is_empty() {
                        let tab = &mut self.tabs[self.current_tab];
                        if let Ok(saved) = tab.save() {
                            if !saved {
                                let _ = tab.save_as();
                            }
                        }
                    }
                    if ui.button("Save As").clicked() && !self.tabs.is_empty() {
                        let tab = &mut self.tabs[self.current_tab];
                        let _ = tab.save_as();
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        // Undo functionality placeholder
                    }
                    if ui.button("Redo").clicked() {
                        // Redo functionality placeholder
                    }
                });
            });
        });

        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut to_close = None;
                for (i, tab) in self.tabs.iter().enumerate() {
                    let label = if tab.modified {
                        format!("{} *", tab.title)
                    } else {
                        tab.title.clone()
                    };
                    if ui.selectable_label(i == self.current_tab, label).clicked() {
                        self.current_tab = i;
                    }
                    if ui.button("x").clicked() {
                        to_close = Some(i);
                    }
                }
                if let Some(index) = to_close {
                    self.close_tab(index);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.tabs.is_empty() {
                let tab = &mut self.tabs[self.current_tab];
                // Use scroll area to ensure the text edit can grow vertically
                egui::ScrollArea::both()
                    .show(ui, |ui| {
                        let response = ui.add(
                            egui::TextEdit::multiline(&mut tab.content)
                                .desired_rows(1) // Let it grow dynamically
                                .desired_width(f32::INFINITY) // Full width
                                .min_size(ui.available_size()), // Fill available space
                        );
                        if response.changed() {
                            tab.modified = true;
                        }
                    });
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Notepad++ Clone",
        native_options,
        Box::new(|cc| Box::new(NotepadApp::new(cc))),
    )
}