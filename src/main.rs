use eframe::egui;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        maximized: true,
        ..Default::default()
    };

    eframe::run_native(
        "Rust Notepad with Line Numbers",
        options,
        Box::new(|_cc| Box::<NotepadApp>::default()),
    )
}

#[derive(Default)]
struct NotepadApp {
    text: String,
    file_path: Option<String>,
    saved: bool,
    line_numbers: Arc<Vec<String>>,
}

impl NotepadApp {
    fn update_line_numbers(&mut self) {
        let line_count = self.text.lines().count().max(1);
        self.line_numbers = Arc::new(
            (1..=line_count).map(|i| i.to_string()).collect()
        );
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| !i.raw.events.is_empty()) {
            self.update_line_numbers();
            self.saved = false;
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Файл", |ui| {
                    if ui.button("Новый").clicked() {
                        self.text.clear();
                        self.file_path = None;
                        self.saved = true;
                    }
                    if ui.button("Открыть...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                self.text = content;
                                self.file_path = Some(path.display().to_string());
                                self.saved = true;
                            }
                        }
                    }
                    if ui.button("Сохранить").clicked() {
                        if let Some(path) = &self.file_path {
                            if std::fs::write(path, &self.text).is_ok() {
                                self.saved = true;
                            }
                        }
                    }
                    ui.separator();
                    if ui.button("Выход").clicked() {
                        _frame.close();
                    }
                });

                ui.label(format!(
                    "{} - {}",
                    self.file_path.as_ref().map_or("Новый файл", |p| {
                        std::path::Path::new(p).file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Файл")
                    }),
                    if self.saved { "Сохранено" } else { "Не сохранено" }
                ));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
            let line_count = self.text.lines().count().max(1);
            let total_height = row_height * line_count as f32;
            let available_height = ui.available_height();

            ui.horizontal(|ui| {
                // Колонка с номерами строк (без скролла)
                ui.vertical(|ui| {
                    ui.set_width(40.0);
                    for line in self.line_numbers.iter() {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(line)
                                    .text_style(egui::TextStyle::Monospace) // Используем напрямую
                                    .size(row_height - 2.0)
                            )
                                .wrap(false)
                        );
                    }
                });

                // Основное текстовое поле со скроллом при необходимости
                if total_height > available_height {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.text)
                                .desired_width(f32::INFINITY)
                                .font(egui::TextStyle::Monospace) // Используем напрямую
                                .frame(false)
                        );
                    });
                } else {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.text)
                            .desired_width(f32::INFINITY)
                            .font(egui::TextStyle::Monospace)
                            .frame(false)
                    );
                }
            });
        });
    }
}