#![expect(dead_code)] // TODO: remove

pub struct Material {
    pub name: &'static str,
    pub head: Option<HeadStats>,
    pub handle: Option<HandleStats>,
    pub binding: Option<BindingStats>,
    pub limb: Option<LimbStats>,
    pub grip: Option<GripStats>,
    pub melee_ability: &'static [Ability],
    pub ranged_ability: &'static [Ability],
    /// Material is provided by a mod and may not be available
    pub mod_compat: bool,
}

pub struct HeadStats {
    pub durability: Base,
    pub harvest_tier: HarvestTier,
    pub mining_speed: Base,
    pub attack_damage: Base,
}

pub struct HandleStats {
    pub durability: Multiplier,
    pub attack_damage: Multiplier,
    pub attack_speed: Multiplier,
    pub mining_speed: Multiplier,
}

impl const Default for HandleStats {
    fn default() -> Self {
        Self {
            durability: 1.0.into(),
            attack_damage: 1.0.into(),
            attack_speed: 1.0.into(),
            mining_speed: 1.0.into(),
        }
    }
}

pub struct BindingStats {}

pub struct LimbStats {
    pub durability: Base,
    pub draw_speed: FlatMod,
    pub velocity: FlatMod,
    pub accuracy: FlatMod,
}

pub struct GripStats {
    pub durability: Multiplier,
    pub accuracy: FlatMod,
    pub attack_damage: Base,
}

pub struct Base(pub f32);
pub struct Multiplier(pub f32);
pub struct FlatMod(pub f32);
impl const From<f32> for Base {
    fn from(value: f32) -> Self {
        Self(value)
    }
}
impl const From<f32> for Multiplier {
    fn from(value: f32) -> Self {
        Self(value)
    }
}
impl const From<f32> for FlatMod {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

pub enum HarvestTier {
    Wood,
    Gold,
    Stone,
    Iron,
    Diamond,
    Netherite,
}

pub struct Ability {
    pub name: &'static str,
    pub flavor: &'static str,
    pub tooltip: &'static str,
    pub explanation: &'static str,
}

pub mod tier1 {
    use super::*;

    const WOOD: Material = Material {
        name: "Wood",
        head: Some(HeadStats {
            durability: 60.0.into(),
            harvest_tier: HarvestTier::Wood,
            mining_speed: 2.0.into(),
            attack_damage: 0.0.into(),
        }),
        handle: Some(HandleStats::default()),
        binding: Some(BindingStats {}),
        limb: Some(LimbStats {
            durability: 60.0.into(),
            draw_speed: 0.0.into(),
            velocity: 0.0.into(),
            accuracy: 0.0.into(),
        }),
        grip: Some(GripStats {
            durability: 1.0.into(),
            accuracy: 0.0.into(),
            attack_damage: 0.0.into(),
        }),
        melee_ability: &[CULTIVATED],
        ranged_ability: &[CULTIVATED],
        mod_compat: false,
    };

    const CULTIVATED: Ability = Ability {
        name: "Cultivated",
        flavor: "Economical!",
        tooltip: "Tool practically grows more material when repairing",
        explanation: "Makes repairs 50% more effective per part",
    };

    const ROCK: () = ();
    const FLINT: () = ();
    const BONE: () = ();
    const COPPER: () = ();
    const CHORUS: () = ();
    const LEATHER: () = ();
    const STRING: () = ();
    const VINE: () = ();
}

pub mod tier2 {
    use super::*;

    const IRON: () = ();
    const SEARED_STONE: () = ();
    const BLOODBONE: () = ();
    const SLIMEWOOD: () = ();
    const NECROTIC_BONE: () = ();
    const SCORCHED_STONE: () = ();
    const WHITESTONE: () = ();
    const CHAIN: () = ();
    const SLIMY_VINE: () = ();
}

pub mod tier3 {
    use super::*;

    const SLIMESTEEL: () = ();
    const AMETHYST_BRONZE: () = ();
    const NAHUATL: () = ();
    const PIG_IRON: () = ();
    const ROSE_GOLD: () = ();
    const ELECTRUM: () = ();
    const PLATED_SLIMEWOOD: () = ();
    const COBALT: () = ();
    const DARKTHREAD: () = ();
}

pub mod tier4 {
    use super::*;

    const QUEENS_SLIME: () = ();
    const HEPATIZON: () = ();
    const MANYULLYN: () = ();
    const BLAZING_BONE: () = ();
    const ANCIENT_HIDE: () = ();
    const SLIMY_VINE: () = ();
}
