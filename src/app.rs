use egui::RichText;

use crate::data::materials::{self, Material};

use self::{
    assembly::Assembly,
    table::{MaterialsTable, MaterialsTableSettings},
};

mod assembly;
mod table;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct TemplateApp {
    #[serde(skip)]
    rows: Vec<&'static Material>,
    #[serde(skip)]
    assembly: Option<Assembly>,

    materials_table_settings: MaterialsTableSettings,
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.style_mut(|s| {
            s.interaction.selectable_labels = false;
            s.interaction.tooltip_delay = 0.1;
            s.interaction.show_tooltips_only_when_still = false;
        });

        let mut this: Self = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or_default();

        this.rows = Self::rows();

        this
    }

    fn rows() -> Vec<&'static Material> {
        let mut rows = Vec::new();
        rows.extend(materials::tier1::MATERIALS.iter().copied());
        rows.extend(materials::tier2::MATERIALS.iter().copied());
        rows.extend(materials::tier3::MATERIALS.iter().copied());
        rows.extend(materials::tier4::MATERIALS.iter().copied());
        rows
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let _ = ui.selectable_label(true, "Modifiers");

                ui.menu_button("Assemble", |ui| {
                    for tool in crate::data::tools::TOOLS {
                        if ui.button(tool.name).clicked() {
                            self.assembly = Some(Assembly::new(tool));
                        }
                    }
                });

                ui.label(RichText::new("⚠ Unvalidated ⚠").color(ui.visuals().warn_fg_color))
                    .on_hover_text(
                        "Data may very well be inaccurate. Report issues for any mismatches.",
                    );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Reset").clicked() {
                        self.materials_table_settings = MaterialsTableSettings::default();
                    }
                });
            });
        });

        if let Some(assembly) = &mut self.assembly {
            egui::SidePanel::right("assembly").show(ctx, |ui| {
                assembly.ui(ui);
                ui.take_available_space();
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let settings = if let Some(assembly) = &self.assembly {
                MaterialsTableSettings::general().maybe_combine(
                    assembly
                        .next_part_type()
                        .map(MaterialsTableSettings::for_part_type),
                )
            } else {
                self.materials_table_settings.clone()
            };

            MaterialsTable {
                rows: &self.rows,
                settings,
                assembly: self.assembly.as_mut(),
            }
            .show(ui);
        });
    }
}
