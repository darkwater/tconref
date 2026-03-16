use crate::data::{materials::Material, tools::PartType};
use egui::{RichText, Sense};
use egui_table::{HeaderCellInfo, HeaderRow, Table, TableDelegate};

use super::assembly::Assembly;

pub struct MaterialsTable<'a> {
    pub rows: &'a [&'static Material],
    pub settings: MaterialsTableSettings,
    pub assembly: Option<&'a mut Assembly>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct MaterialsTableSettings {
    pub columns: Vec<Column>,
}
impl Default for MaterialsTableSettings {
    fn default() -> Self {
        Self::all()
    }
}
impl MaterialsTableSettings {
    pub fn general() -> Self {
        Self {
            columns: vec![
                Column::Material,
                Column::Tier,
                Column::ModCompat,
                Column::Overslime,
                Column::Trait,
            ],
        }
    }

    pub fn trailing() -> Self {
        Self {
            columns: vec![Column::TraitExplanation],
        }
    }

    pub fn for_part_type(part_type: PartType) -> Self {
        let columns = match part_type {
            PartType::Head => {
                vec![
                    Column::Value(ValueColumn::HeadDurability),
                    Column::HarvestTier,
                    Column::Value(ValueColumn::HeadMiningSpeed),
                    Column::Value(ValueColumn::HeadAttackDamage),
                ]
            }
            PartType::Handle => {
                vec![
                    Column::Value(ValueColumn::HandleDurability),
                    Column::Value(ValueColumn::HandleAttackDamage),
                    Column::Value(ValueColumn::HandleAttackSpeed),
                    Column::Value(ValueColumn::HandleMiningSpeed),
                ]
            }
            PartType::Limb => {
                vec![
                    Column::Value(ValueColumn::LimbDurability),
                    Column::Value(ValueColumn::LimbDrawSpeed),
                    Column::Value(ValueColumn::LimbVelocity),
                    Column::Value(ValueColumn::LimbAccuracy),
                ]
            }
            PartType::Grip => {
                vec![
                    Column::Value(ValueColumn::GripDurability),
                    Column::Value(ValueColumn::GripAccuracy),
                    Column::Value(ValueColumn::GripAttackDamage),
                ]
            }
            PartType::Binding | PartType::Bowstring => vec![],
        };

        Self { columns }
    }

    pub fn all() -> Self {
        Self::general()
            .combine(Self::for_part_type(PartType::Head))
            .combine(Self::for_part_type(PartType::Handle))
            .combine(Self::for_part_type(PartType::Limb))
            .combine(Self::for_part_type(PartType::Grip))
            .combine(Self::trailing())
    }

    pub fn combine(mut self, other: Self) -> Self {
        self.columns.extend(other.columns);
        self
    }

    pub fn maybe_combine(self, other: Option<Self>) -> Self {
        if let Some(other) = other {
            self.combine(other)
        } else {
            self
        }
    }
}

impl MaterialsTable<'_> {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        #[expect(clippy::single_range_in_vec_init)]
        Table::new()
            .id_salt("materials_table")
            .columns(
                self.settings
                    .columns
                    .iter()
                    .map(|col| {
                        egui_table::Column::new(col.width())
                            .id(egui::Id::new(col))
                            .resizable(false)
                    })
                    .collect::<Vec<_>>(),
            )
            .num_sticky_cols(1)
            .headers([
                HeaderRow {
                    height: 24.0,
                    groups: self.settings.columns.iter().enumerate().fold(vec![0..0], {
                        let mut last = None;
                        move |mut groups, (i, col)| {
                            let category = col.category();
                            if category != last {
                                groups.push(i..(i + 1));
                                last = category;
                            } else if let Some(last_group) = groups.last_mut() {
                                last_group.end += 1;
                            }
                            groups
                        }
                    }),
                },
                HeaderRow {
                    height: 24.0,
                    groups: vec![],
                },
            ])
            .num_rows(self.rows.len() as u64)
            .show(ui, self);
    }
}

impl TableDelegate for MaterialsTable<'_> {
    fn header_cell_ui(&mut self, ui: &mut egui::Ui, cell: &HeaderCellInfo) {
        let rect = ui.clip_rect();
        ui.set_clip_rect(rect.expand(2.));

        ui.painter().line_segment(
            [rect.right_top(), rect.right_bottom()],
            ui.visuals().widgets.noninteractive.bg_stroke,
        );

        ui.set_clip_rect(rect);

        egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(8, 4))
            .show(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    let Some(column) = &self.settings.columns.get(cell.col_range.start) else {
                        return;
                    };

                    match cell.row_nr {
                        0 => ui.label(
                            column
                                .category()
                                .map_or(String::new(), |cat| format!("{cat:?}")),
                        ),
                        1 => ui.label(column.header()),
                        _ => unreachable!(),
                    };
                });
            });
    }

    fn cell_ui(&mut self, ui: &mut egui::Ui, cell: &egui_table::CellInfo) {
        let Some(material) = self.rows.get(cell.row_nr as usize) else {
            return;
        };

        let Some(column) = self.settings.columns.get(cell.col_nr) else {
            return;
        };

        let res = ui
            .with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                column.frame().show(ui, |ui| {
                    column.contents(ui, material);
                });
            })
            .response;

        column.on_res(material, res);
    }

    fn row_ui(&mut self, ui: &mut egui::Ui, row_nr: u64) {
        let Some(material) = self.rows.get(row_nr as usize) else {
            return;
        };

        let odd_row = row_nr % 2 == 1;

        let response = ui.interact(
            ui.min_rect(),
            ui.unique_id(),
            if self.assembly.is_some() {
                Sense::click()
            } else {
                Sense::hover()
            },
        );

        if let Some(assembly) = self.assembly.as_mut() {
            if let Some(next_part_type) = assembly.next_part_type()
                && !material.valid_for(next_part_type)
            {
                ui.disable();

                if odd_row {
                    ui.painter()
                        .rect_filled(ui.max_rect(), 0.0, ui.visuals().faint_bg_color);
                }

                return;
            }

            if response.clicked() {
                assembly.part_materials.push(material);
            }
        }

        if self.assembly.is_some() && response.hovered() {
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, ui.visuals().code_bg_color);
        } else if odd_row {
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, ui.visuals().faint_bg_color);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Column {
    Material,
    Tier,
    ModCompat,
    Overslime,
    Trait,
    HarvestTier,
    Value(ValueColumn),
    TraitExplanation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum ValueColumn {
    HeadDurability,
    HeadMiningSpeed,
    HeadAttackDamage,
    HandleDurability,
    HandleAttackDamage,
    HandleAttackSpeed,
    HandleMiningSpeed,
    LimbDurability,
    LimbDrawSpeed,
    LimbVelocity,
    LimbAccuracy,
    GripDurability,
    GripAccuracy,
    GripAttackDamage,
}

impl Column {
    pub fn width(&self) -> f32 {
        match self {
            Self::Material => 180.0,
            Self::Tier => 48.0,
            Self::ModCompat | Self::Overslime => 84.0,
            Self::Trait | Self::HarvestTier => 140.0,
            Self::Value(_) => 96.0,
            Self::TraitExplanation => 720.0,
        }
    }

    pub fn header(&self) -> &'static str {
        match self {
            Self::Material => "Material",
            Self::Tier => "Tier",
            Self::ModCompat => "Mod Compat",
            Self::Overslime => "Overslime",
            Self::Trait => "Trait",
            Self::HarvestTier => "Harvest Tier",
            Self::Value(value) =>
            {
                #[expect(clippy::match_same_arms)]
                match value {
                    ValueColumn::HeadDurability => "Durability",
                    ValueColumn::HeadMiningSpeed => "Mining Speed",
                    ValueColumn::HeadAttackDamage => "Attack Damage",
                    ValueColumn::HandleDurability => "Durability",
                    ValueColumn::HandleAttackDamage => "Attack Damage",
                    ValueColumn::HandleAttackSpeed => "Attack Speed",
                    ValueColumn::HandleMiningSpeed => "Mining Speed",
                    ValueColumn::LimbDurability => "Durability",
                    ValueColumn::LimbDrawSpeed => "Draw Speed",
                    ValueColumn::LimbVelocity => "Velocity",
                    ValueColumn::LimbAccuracy => "Accuracy",
                    ValueColumn::GripDurability => "Durability",
                    ValueColumn::GripAccuracy => "Accuracy",
                    ValueColumn::GripAttackDamage => "Attack Damage",
                }
            }
            Self::TraitExplanation => "Trait Explanation",
        }
    }

    pub fn category(&self) -> Option<PartType> {
        match self {
            Self::HarvestTier
            | Self::Value(
                ValueColumn::HeadDurability
                | ValueColumn::HeadMiningSpeed
                | ValueColumn::HeadAttackDamage,
            ) => Some(PartType::Head),
            Self::Value(
                ValueColumn::HandleDurability
                | ValueColumn::HandleAttackDamage
                | ValueColumn::HandleAttackSpeed
                | ValueColumn::HandleMiningSpeed,
            ) => Some(PartType::Handle),
            Self::Value(
                ValueColumn::LimbDurability
                | ValueColumn::LimbDrawSpeed
                | ValueColumn::LimbVelocity
                | ValueColumn::LimbAccuracy,
            ) => Some(PartType::Limb),
            Self::Value(
                ValueColumn::GripDurability
                | ValueColumn::GripAccuracy
                | ValueColumn::GripAttackDamage,
            ) => Some(PartType::Grip),
            _ => None,
        }
    }

    pub fn contents(&self, ui: &mut egui::Ui, material: &Material) {
        match self {
            Self::Material => {
                ui.label(material.name);
            }
            Self::Tier => {
                ui.label(material.tier.to_string());
            }
            Self::ModCompat => {
                ui.label(if material.mod_compat { "yes" } else { "no" });
            }
            Self::Overslime => {
                ui.label(if material.overslime { "yes" } else { "no" });
            }
            Self::Trait => {
                if let Some(tr) = material.melee_trait.or(material.ranged_trait) {
                    ui.label(tr.name);
                } else {
                    ui.label("-");
                }
            }
            Self::HarvestTier => {
                if let Some(harvest_tier) = material.head.map(|head| head.harvest_tier) {
                    ui.label(harvest_tier.to_string());
                } else {
                    ui.label("-");
                }
            }
            Self::Value(value_column) => {
                if let Some(value) = value_column.format(material) {
                    ui.label(value);
                } else {
                    ui.label("-");
                }
            }
            Self::TraitExplanation => {
                if let Some(tr) = material.trait_() {
                    ui.label(tr.explanation);
                } else {
                    ui.label("-");
                }
            }
        }
    }

    #[expect(clippy::unused_self)]
    pub fn frame(&self) -> egui::Frame {
        egui::Frame::new().inner_margin(egui::Margin::symmetric(6, 3))
    }

    pub fn on_res(&self, material: &Material, res: egui::Response) {
        if let Self::Trait = self
            && let Some(tr) = material.trait_()
        {
            res.on_hover_ui(|ui| {
                ui.label(RichText::new(tr.flavor).italics());
                ui.label(tr.tooltip);
                ui.separator();
                ui.label(tr.explanation);
            });
        }
    }
}

impl ValueColumn {
    pub fn format(&self, material: &Material) -> Option<String> {
        match self {
            Self::HeadDurability => material.head.map(|head| head.durability.to_string()),
            Self::HeadMiningSpeed => material.head.map(|head| head.mining_speed.to_string()),
            Self::HeadAttackDamage => material.head.map(|head| head.attack_damage.to_string()),
            Self::HandleDurability => material.handle.map(|handle| handle.durability.to_string()),
            Self::HandleAttackDamage => material
                .handle
                .map(|handle| handle.attack_damage.to_string()),
            Self::HandleAttackSpeed => material
                .handle
                .map(|handle| handle.attack_speed.to_string()),
            Self::HandleMiningSpeed => material
                .handle
                .map(|handle| handle.mining_speed.to_string()),
            Self::LimbDurability => material.limb.map(|limb| limb.durability.to_string()),
            Self::LimbDrawSpeed => material.limb.map(|limb| limb.draw_speed.to_string()),
            Self::LimbVelocity => material.limb.map(|limb| limb.velocity.to_string()),
            Self::LimbAccuracy => material.limb.map(|limb| limb.accuracy.to_string()),
            Self::GripDurability => material.grip.map(|grip| grip.durability.to_string()),
            Self::GripAccuracy => material.grip.map(|grip| grip.accuracy.to_string()),
            Self::GripAttackDamage => material.grip.map(|grip| grip.attack_damage.to_string()),
        }
    }
}
