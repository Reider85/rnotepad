use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Notepad",
        options,
        Box::new(|_cc| Box::new(NotepadApp::default())),
    );
}

#[derive(Default)]
struct NotepadApp {
    text: String,
    file_path: Option<String>,
    saved: bool,
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Файл", |ui| {
                    if ui.button("Новый").clicked() {
                        self.new_file();
                    }
                    if ui.button("Открыть...").clicked() {
                        self.open_file();
                    }
                    if ui.button("Сохранить").clicked() {
                        self.save_file();
                    }
                    if ui.button("Сохранить как...").clicked() {
                        self.save_file_as();
                    }
                    ui.separator();
                    if ui.button("Выход").clicked() {
                        _frame.close();
                    }
                });

                let status = if self.saved { "Сохранено" } else { "Не сохранено" };
                let file_name = self.file_path.as_ref().map_or("Безымянный".to_string(), |p| {
                    std::path::Path::new(p).file_name().unwrap().to_string_lossy().into_owned()
                });
                ui.label(format!("{} - {}", file_name, status));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.text)
                        .desired_width(f32::INFINITY)
                        .desired_rows(20)
                        .font(egui::TextStyle::Monospace),
                );
            });
        });
    }
}

impl NotepadApp {
    fn new_file(&mut self) {
        if !self.saved {
            // Здесь можно добавить проверку на необходимость сохранения
        }
        self.text.clear();
        self.file_path = None;
        self.saved = true;
    }

    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                self.text = content;
                self.file_path = Some(path.to_string_lossy().into_owned());
                self.saved = true;
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(path) = &self.file_path {
            if let Err(e) = std::fs::write(path, &self.text) {
                eprintln!("Ошибка сохранения файла: {}", e);
            } else {
                self.saved = true;
            }
        } else {
            self.save_file_as();
        }
    }

    fn save_file_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new().save_file() {
            if let Err(e) = std::fs::write(&path, &self.text) {
                eprintln!("Ошибка сохранения файла: {}", e);
            } else {
                self.file_path = Some(path.to_string_lossy().into_owned());
                self.saved = true;
            }
        }
    }
}