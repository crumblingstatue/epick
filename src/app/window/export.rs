use crate::{
    app::window::{self, WINDOW_X_OFFSET, WINDOW_Y_OFFSET},
    color::{NamedPalette, PaletteFormat},
    context::FrameCtx,
};

use anyhow::Result;
use egui::{Color32, ComboBox, CursorIcon, TextEdit, Window};
use std::{env, fs, path::PathBuf};

#[derive(Debug)]
pub struct ExportWindow {
    pub show: bool,
    pub path: String,
    pub export_status: Result<String, String>,
    pub format: PaletteFormat,
    pub export_path_editable: bool,
    pub export_palette: Option<NamedPalette>,
}

impl Default for ExportWindow {
    fn default() -> Self {
        Self {
            show: false,
            format: PaletteFormat::default(),
            export_status: Ok("".to_string()),
            path: env::current_dir()
                .map(|d| d.to_string_lossy().to_string())
                .unwrap_or_default(),
            export_path_editable: true,
            export_palette: None,
        }
    }
}

impl ExportWindow {
    pub fn display(&mut self, ctx: &FrameCtx) -> Result<()> {
        if self.show {
            let offset = ctx.egui.style().spacing.slider_width * WINDOW_X_OFFSET;
            let mut show = true;
            let is_dark_mode = ctx.egui.style().visuals.dark_mode;
            Window::new("export")
                .frame(window::default_frame(is_dark_mode))
                .open(&mut show)
                .default_pos((offset, WINDOW_Y_OFFSET))
                .show(ctx.egui, |ui| {
                    window::apply_default_style(ui, is_dark_mode);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ComboBox::from_label("format")
                                .selected_text(self.format.as_ref())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.format,
                                        PaletteFormat::Gimp,
                                        PaletteFormat::Gimp.as_ref(),
                                    );
                                    ui.selectable_value(
                                        &mut self.format,
                                        PaletteFormat::HexList,
                                        PaletteFormat::HexList.as_ref(),
                                    );
                                });
                        });
                        if let Some(palette) = &self.export_palette {
                            ui.scope(|ui| {
                                ui.label("Name: ");
                                ui.label(egui::RichText::new(&palette.name).italics());
                            });

                            ui.label("Export path:");
                            ui.add(
                                TextEdit::singleline(&mut self.path)
                                    .interactive(self.export_path_editable),
                            );

                            match &self.export_status {
                                Ok(msg) => ui.colored_label(Color32::GREEN, msg),
                                Err(msg) => ui.colored_label(Color32::RED, msg),
                            };

                            if ui
                                .button("export")
                                .on_hover_cursor(CursorIcon::PointingHand)
                                .clicked()
                            {
                                let generated_palette = palette.display(
                                    &self.format,
                                    ctx.app.settings.rgb_working_space,
                                    ctx.app.settings.illuminant,
                                );
                                let p = PathBuf::from(&self.path);
                                let filename =
                                    format!("{}.{}", &palette.name, self.format.extension());
                                if let Err(e) = fs::write(p.join(filename), generated_palette) {
                                    self.export_status = Err(e.to_string());
                                } else {
                                    self.export_status = Ok("export succesful".to_string());
                                }
                            }
                        }
                    });
                });

            if !show {
                self.show = false;
            }
        }

        Ok(())
    }
}
