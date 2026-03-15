use egui::RichText;

use crate::data::{
    materials::{
        Base, FlatMod, GripStats, HandleStats, HeadStats, LimbStats, Material, Multiplier,
    },
    tools::{PartType, Tool, Weighted},
};

pub struct Assembly {
    pub tool: &'static Tool,
    pub part_materials: Vec<&'static Material>,
}

impl Assembly {
    pub fn new(tool: &'static Tool) -> Self {
        Self {
            tool,
            part_materials: vec![],
        }
    }

    pub fn next_part_type(&self) -> Option<PartType> {
        let next_part_index = self.part_materials.len();
        self.tool
            .parts
            .get(next_part_index)
            .map(|part| part.inner.part_type())
    }

    pub fn weighted_materials_of_type(&self, ty: PartType) -> Vec<&'static Material> {
        self.tool
            .parts
            .iter()
            .zip(&self.part_materials)
            .filter(|(part, _)| part.inner.part_type() == ty)
            .flat_map(|(part, material)| std::iter::repeat_n(*material, part.weight))
            .collect()
    }

    #[expect(clippy::unwrap_used)]
    pub fn weighted_head_stats(&self) -> Vec<HeadStats> {
        self.weighted_materials_of_type(PartType::Head)
            .iter()
            .map(|material| material.head.unwrap())
            .collect()
    }

    #[expect(clippy::unwrap_used)]
    pub fn weighted_handle_stats(&self) -> Vec<HandleStats> {
        self.weighted_materials_of_type(PartType::Handle)
            .iter()
            .map(|material| material.handle.unwrap())
            .collect()
    }

    pub fn materials_of_type(&self, ty: PartType) -> Vec<Weighted<&'static Material>> {
        self.tool
            .parts
            .iter()
            .zip(&self.part_materials)
            .filter(|(part, _)| part.inner.part_type() == ty)
            .map(|(part, material)| Weighted::weighted(*material, part.weight))
            .collect()
    }

    #[expect(clippy::unwrap_used)]
    pub fn head_stats(&self) -> Vec<Weighted<HeadStats>> {
        self.materials_of_type(PartType::Head)
            .iter()
            .map(|material| material.map(|m| m.head.unwrap()))
            .collect()
    }

    #[expect(clippy::unwrap_used)]
    pub fn handle_stats(&self) -> Vec<Weighted<HandleStats>> {
        self.materials_of_type(PartType::Handle)
            .iter()
            .map(|material| material.map(|m| m.handle.unwrap()))
            .collect()
    }

    #[expect(clippy::unwrap_used)]
    pub fn limb_stats(&self) -> Vec<Weighted<LimbStats>> {
        self.materials_of_type(PartType::Limb)
            .iter()
            .map(|material| material.map(|m| m.limb.unwrap()))
            .collect()
    }

    #[expect(clippy::unwrap_used)]
    pub fn grip_stats(&self) -> Vec<Weighted<GripStats>> {
        self.materials_of_type(PartType::Grip)
            .iter()
            .map(|material| material.map(|m| m.grip.unwrap()))
            .collect()
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        ui.heading(self.tool.name);
        for (idx, part) in self.tool.parts.iter().enumerate() {
            let material = self.part_materials.get(idx).copied();

            ui.horizontal(|ui| {
                ui.label(part.inner.to_string());
                if let Some(material) = material {
                    ui.label(format!("({})", material.name));
                } else {
                    ui.label("(None)");
                }
            });
        }

        ui.separator();

        ui.label(
            RichText::new("⚠ Calculations are unfinished and incorrect ⚠")
                .color(ui.visuals().warn_fg_color),
        );

        let head_stats = self.weighted_head_stats();
        let handle_stats = self.weighted_handle_stats();

        let durability = (self.tool.base.durability.0
            + nan_zero(
                head_stats.iter().map(|s| s.durability.0).sum::<f32>() / head_stats.len() as f32,
            ))
            * handle_stats.iter().map(|s| s.durability.0).product::<f32>()
            * self.tool.multiplier.durability.0;

        let attack_damage = (self.tool.base.attack_damage.0
            + nan_zero(
                head_stats.iter().map(|s| s.attack_damage.0).sum::<f32>() / head_stats.len() as f32,
            ))
            * handle_stats
                .iter()
                .map(|s| s.attack_damage.0)
                .product::<f32>()
            * self.tool.multiplier.attack_damage.0;

        let attack_speed = self.tool.base.attack_speed.0
            * handle_stats
                .iter()
                .map(|s| s.attack_speed.0)
                .product::<f32>()
            * self.tool.multiplier.attack_speed.0;

        let mining_speed = (self.tool.base.mining_speed.0
            + nan_zero(
                head_stats.iter().map(|s| s.mining_speed.0).sum::<f32>() / head_stats.len() as f32,
            ))
            * handle_stats
                .iter()
                .map(|s| s.mining_speed.0)
                .product::<f32>()
            * self.tool.multiplier.mining_speed.0;

        ui.label(format!("Durability: {durability}"));
        ui.label(format!("Attack Damage: {attack_damage}"));
        ui.label(format!("Attack Speed: {attack_speed}"));
        ui.label(format!("Mining Speed: {mining_speed}"));

        ui.separator();

        let mut durability = Calculation::new();
        durability.add_flat(Weighted::single(self.tool.flat.durability));
        durability.add_mult(Weighted::single(self.tool.multiplier.durability));
        for head_stat in self.head_stats() {
            durability.add_base(head_stat.map(|s| s.durability));
        }
        for limb_stat in self.limb_stats() {
            durability.add_flat(limb_stat.map(|s| s.durability));
        }
        for handle_stat in self.handle_stats() {
            durability.add_mult(handle_stat.map(|s| s.durability));
        }
        for grip_stat in self.grip_stats() {
            durability.add_mult(grip_stat.map(|s| s.durability));
        }
        ui.collapsing(format!("Durability: {}", durability.result()), |ui| {
            durability.ui(ui);
        });

        let mut attack_damage = Calculation::new();
        attack_damage.add_flat(Weighted::single(FlatMod(1.)));
        attack_damage.add_flat(Weighted::single(self.tool.flat.attack_damage));
        attack_damage.add_mult(Weighted::single(self.tool.multiplier.attack_damage));
        for head_stat in self.head_stats() {
            attack_damage.add_base(head_stat.map(|s| s.attack_damage));
        }
        for handle_stat in self.handle_stats() {
            attack_damage.add_mult(handle_stat.map(|s| s.attack_damage));
        }
        for grip_stat in self.grip_stats() {
            attack_damage.add_base(grip_stat.map(|s| s.attack_damage));
        }
        ui.collapsing(format!("Attack Damage: {}", attack_damage.result()), |ui| {
            attack_damage.ui(ui);
        });

        let mut attack_speed = Calculation::new();
        attack_speed.add_flat(Weighted::single(FlatMod(1.)));
        attack_speed.add_mult(Weighted::single(self.tool.multiplier.attack_speed));
        for handle_stat in self.handle_stats() {
            attack_speed.add_mult(handle_stat.map(|s| s.attack_speed));
        }
        ui.collapsing(format!("Attack Speed: {}", attack_speed.result()), |ui| {
            attack_speed.ui(ui);
        });
    }
}

fn nan_zero(x: f32) -> f32 {
    if x.is_nan() { 0.0 } else { x }
}

struct Calculation {
    pub base: Vec<Weighted<Base>>,
    pub flat: Vec<Weighted<FlatMod>>,
    pub mult: Vec<Weighted<Multiplier>>,
}

impl Calculation {
    pub fn new() -> Self {
        Self {
            base: vec![],
            flat: vec![],
            mult: vec![],
        }
    }

    pub fn add_base(&mut self, base: Weighted<Base>) {
        self.base.push(base);
    }

    pub fn add_flat(&mut self, flat: Weighted<FlatMod>) {
        self.flat.push(flat);
    }

    pub fn add_mult(&mut self, mult: Weighted<Multiplier>) {
        self.mult.push(mult);
    }

    pub fn base(&self) -> f32 {
        let base = self
            .base
            .iter()
            .filter(|b| b.inner.0 != 0.)
            .map(|b| b.inner.0 * b.weight as f32)
            .sum::<f32>()
            / self
                .base
                .iter()
                .filter(|b| b.inner.0 != 0.)
                .map(|b| b.weight)
                .sum::<usize>() as f32;

        if base.is_nan() { 0. } else { base }
    }

    pub fn flat(&self) -> f32 {
        self.flat
            .iter()
            .map(|b| b.inner.0 * b.weight as f32)
            .sum::<f32>()
    }

    pub fn mult(&self) -> f32 {
        self.mult
            .iter()
            .map(|b| b.inner.0 * b.weight as f32)
            .product::<f32>()
    }

    pub fn result(&self) -> f32 {
        (self.base() + self.flat()) * self.mult()
    }
}

impl Calculation {
    fn ui(self, ui: &mut egui::Ui) {
        match self.base.as_slice() {
            [] => {
                ui.collapsing("Base: 0", |_ui| {});
            }
            // [_single] => {
            //     ui.label(format!("Base: {}", self.base()));
            // }
            multiple => {
                ui.collapsing(format!("Base: {} (average)", self.base()), |ui| {
                    for base in multiple {
                        match base.weight {
                            1 => ui.label(format!("{}", base.inner.0)),
                            w => ui.label(format!("{} (x{w})", base.inner.0)),
                        };
                    }
                });
            }
        }

        match self.flat.as_slice() {
            [] => {}
            // [_single] => {
            //     ui.label(format!("Flat: {}", self.base()));
            // }
            multiple => {
                ui.collapsing(format!("Flat: {}", self.flat()), |ui| {
                    for flat in multiple {
                        match flat.weight {
                            1 => ui.label(format!("{:+}", flat.inner.0)),
                            w => ui.label(format!("{:+} (x{w})", flat.inner.0)),
                        };
                    }
                });
            }
        }

        match self.mult.as_slice() {
            [] => {}
            // [_single] => {
            //     ui.label(format!("Mult: {}", self.mult()));
            // }
            multiple => {
                ui.collapsing(format!("Multiplier: {}", self.mult()), |ui| {
                    for mult in multiple {
                        match mult.weight {
                            1 => ui.label(format!("{}x", mult.inner.0)),
                            w => ui.label(format!("{}x (x{w})", mult.inner.0)),
                        };
                    }
                });
            }
        }
    }
}
