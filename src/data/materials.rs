use core::fmt::Display;

use super::tools::PartType;

pub struct Material {
    pub name: &'static str,
    pub tier: u8,
    pub head: Option<HeadStats>,
    pub handle: Option<HandleStats>,
    pub binding: Option<BindingStats>,
    pub bowstring: Option<BowstringStats>,
    pub limb: Option<LimbStats>,
    pub grip: Option<GripStats>,
    pub melee_trait: Option<Trait>,
    pub ranged_trait: Option<Trait>,
    pub overslime: bool,
    /// Material is provided by a mod and may not be available
    pub mod_compat: bool,
}

impl Material {
    pub fn trait_(&self) -> Option<Trait> {
        self.melee_trait.or(self.ranged_trait)
    }

    pub fn valid_for(&self, part_type: PartType) -> bool {
        match part_type {
            PartType::Head => self.head.is_some(),
            PartType::Handle => self.handle.is_some(),
            PartType::Binding => self.binding.is_some(),
            PartType::Limb => self.limb.is_some(),
            PartType::Grip => self.grip.is_some(),
            PartType::Bowstring => self.bowstring.is_some(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HeadStats {
    pub durability: Base,
    pub harvest_tier: HarvestTier,
    pub mining_speed: Base,
    pub attack_damage: Base,
}

#[derive(Debug, Clone, Copy)]
pub struct HandleStats {
    pub durability: Multiplier,
    pub attack_damage: Multiplier,
    pub attack_speed: Multiplier,
    pub mining_speed: Multiplier,
}

impl const Default for HandleStats {
    fn default() -> Self {
        Self {
            durability: Multiplier(1.0),
            attack_damage: Multiplier(1.0),
            attack_speed: Multiplier(1.0),
            mining_speed: Multiplier(1.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BindingStats {}

#[derive(Debug, Clone, Copy)]
pub struct BowstringStats {}

#[derive(Debug, Clone, Copy)]
pub struct LimbStats {
    pub durability: FlatMod,
    pub draw_speed: FlatMod,
    pub velocity: FlatMod,
    pub accuracy: FlatMod,
}

#[derive(Debug, Clone, Copy)]
pub struct GripStats {
    pub durability: Multiplier,
    pub accuracy: FlatMod,
    pub attack_damage: Base,
}

#[derive(Debug, Clone, Copy)]
pub struct Base(pub f32);
#[derive(Debug, Clone, Copy)]
pub struct Multiplier(pub f32);
#[derive(Debug, Clone, Copy)]
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

impl Display for Base {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self(v) = self;
        write!(f, "{v}")
    }
}
impl Display for Multiplier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self(v) = self;
        write!(f, "{v}x")
    }
}
impl Display for FlatMod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self(v) = self;
        let sign = match *v {
            v if v > 0.0 => "+",
            v if v < 0.0 => "-",
            _ => "",
        };
        write!(f, "{sign}{v}")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HarvestTier {
    Wood,
    Gold,
    Stone,
    Iron,
    Diamond,
    Netherite,
}
impl Display for HarvestTier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Trait {
    pub name: &'static str,
    pub flavor: &'static str,
    pub tooltip: &'static str,
    pub explanation: &'static str,
}

pub mod tier1 {
    use super::*;

    const WOOD: Material = Material {
        name: "Wood",
        tier: 1,
        head: Some(HeadStats {
            durability: Base(60.0),
            harvest_tier: HarvestTier::Wood,
            mining_speed: Base(2.0),
            attack_damage: Base(0.0),
        }),
        handle: Some(HandleStats::default()),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(60.0),
            draw_speed: FlatMod(0.0),
            velocity: FlatMod(0.0),
            accuracy: FlatMod(0.0),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.0),
            accuracy: FlatMod(0.0),
            attack_damage: Base(0.0),
        }),
        melee_trait: Some(CULTIVATED),
        ranged_trait: Some(CULTIVATED),
        overslime: false,
        mod_compat: false,
    };

    const CULTIVATED: Trait = Trait {
        name: "Cultivated",
        flavor: "Economical!",
        tooltip: "Tool practically grows more material when repairing",
        explanation: "Makes repairs 50% more effective per part",
    };

    const BAMBOO: Material = Material {
        name: "Bamboo",
        tier: 1,
        head: None,
        handle: None,
        binding: None,
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(70.0),
            draw_speed: FlatMod(0.1),
            velocity: FlatMod(-0.05),
            accuracy: FlatMod(-0.05),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.95),
            accuracy: FlatMod(0.05),
            attack_damage: Base(0.75),
        }),
        melee_trait: None,
        ranged_trait: Some(FLEXIBLE),
        overslime: false,
        mod_compat: false,
    };

    const FLEXIBLE: Trait = Trait {
        name: "Flexible",
        flavor: "Twang",
        tooltip: "Your arrows fly faster, but do the same damage",
        explanation: "Arrows have 110% velocity, but deal 90% damage. Effectively, they go farther while dealing about the same damage",
    };

    const ROCK: Material = Material {
        name: "Rock",
        tier: 1,
        head: Some(HeadStats {
            durability: Base(130.0),
            harvest_tier: HarvestTier::Stone,
            mining_speed: Base(4.0),
            attack_damage: Base(1.0),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.9),
            mining_speed: Multiplier(1.05),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: None,
        grip: None,
        melee_trait: Some(STONEBOUND),
        ranged_trait: Some(STONEBOUND),
        overslime: false,
        mod_compat: false,
    };

    const STONEBOUND: Trait = Trait {
        name: "Stonebound",
        flavor: "Your tool absolutely loves stone!",
        tooltip: "The tool mines faster as it wears out, but does less damage",
        explanation: "Causes the tool to mine faster by half of the square root of the trait level times lost durability",
    };

    const FLINT: Material = Material {
        name: "Flint",
        tier: 1,
        head: Some(HeadStats {
            durability: Base(85.0),
            harvest_tier: HarvestTier::Stone,
            mining_speed: Base(3.5),
            attack_damage: Base(1.25),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.85),
            attack_damage: Multiplier(1.1),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: None,
        grip: None,
        melee_trait: Some(JAGGED),
        ranged_trait: Some(JAGGED),
        overslime: false,
        mod_compat: false,
    };

    const JAGGED: Trait = Trait {
        name: "Jagged",
        flavor: "Broken edges make it sharp!",
        tooltip: "Every point durability lost increases damage, but decreases mining speed",
        explanation: "Increases damage by a quarter of the square root of the trait level times lost durability",
    };

    const BONE: Material = Material {
        name: "Bone",
        tier: 1,
        head: Some(HeadStats {
            durability: Base(100.0),
            harvest_tier: HarvestTier::Stone,
            mining_speed: Base(2.5),
            attack_damage: Base(1.25),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.75),
            attack_speed: Multiplier(1.1),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(100.0),
            draw_speed: FlatMod(0.05),
            velocity: FlatMod(-0.05),
            accuracy: FlatMod(0.05),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.75),
            accuracy: FlatMod(0.05),
            attack_damage: Base(1.25),
        }),
        melee_trait: Some(PIERCING),
        ranged_trait: Some(PIERCING_RANGED),
        overslime: false,
        mod_compat: false,
    };

    const PIERCING: Trait = Trait {
        name: "Piercing",
        flavor: "Become the cactus!",
        tooltip: "Tool deals bonus damage which attacks through armor!",
        explanation: "Converts 0.5 damage into 1 piercing damage per level",
    };

    const PIERCING_RANGED: Trait = Trait {
        name: "Piercing",
        flavor: "Become the cactus!",
        tooltip: "Tool deals bonus damage which attacks through armor!",
        explanation: "Causes arrows to deal 1 piercing damage per level in addition to regular damage",
    };

    const COPPER: Material = Material {
        name: "Copper",
        tier: 1,
        head: Some(HeadStats {
            durability: Base(210.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(5.0),
            attack_damage: Base(0.5),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.8),
            mining_speed: Multiplier(1.1),
            attack_damage: Multiplier(1.05),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(210.0),
            draw_speed: FlatMod(-0.1),
            velocity: FlatMod(0.05),
            accuracy: FlatMod(0.0),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.8),
            accuracy: FlatMod(0.0),
            attack_damage: Base(0.5),
        }),
        melee_trait: Some(DWARVEN),
        ranged_trait: Some(DWARVEN_RANGED),
        overslime: false,
        mod_compat: false,
    };

    const DWARVEN: Trait = Trait {
        name: "Dwarven",
        flavor: "Miner's Friend!",
        tooltip: "Tool mines faster and has more projectile velocity faster the deeper you mine",
        explanation: "Grants bonus mining speed at lower depth, +6 per 64 blocks below 64",
    };

    const DWARVEN_RANGED: Trait = Trait {
        name: "Dwarven",
        flavor: "Miner's Friend!",
        tooltip: "Tool mines faster and has more projectile velocity faster the deeper you mine",
        explanation: "Grants bonus velocity at lower depth, +0.05 per 64 blocks below 64",
    };

    const CHORUS: Material = Material {
        name: "Chorus",
        tier: 1,
        head: Some(HeadStats {
            durability: Base(180.0),
            harvest_tier: HarvestTier::Stone,
            mining_speed: Base(3.0),
            attack_damage: Base(1.0),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.1),
            mining_speed: Multiplier(0.95),
            attack_speed: Multiplier(0.9),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(180.0),
            draw_speed: FlatMod(0.1),
            velocity: FlatMod(0.0),
            accuracy: FlatMod(0.1),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.1),
            accuracy: FlatMod(-0.1),
            attack_damage: Base(1.0),
        }),
        melee_trait: Some(ENDERFERENCE),
        ranged_trait: Some(ENDERFERENCE_RANGED),
        overslime: false,
        mod_compat: false,
    };

    const ENDERFERENCE: Trait = Trait {
        name: "Enderference",
        flavor: "Really quite distracting",
        tooltip: "Prevents target from teleporting for a short time",
        explanation: "Prevents the target from teleporting for a 5 seconds per effect level",
    };

    const ENDERFERENCE_RANGED: Trait = Trait {
        name: "Enderference",
        flavor: "Really quite distracting",
        tooltip: "Prevents target from teleporting for a short time",
        explanation: "Prevents the target from teleporting for a 5 seconds per effect level and allows damaging endermen with arrows",
    };

    const LEATHER: Material = Material {
        name: "Leather",
        tier: 1,
        head: None,
        handle: None,
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: None,
        grip: None,
        melee_trait: Some(TANNED),
        ranged_trait: Some(TANNED),
        overslime: false,
        mod_compat: false,
    };

    const TANNED: Trait = Trait {
        name: "Tanned",
        flavor: "Become a master leatherworker",
        tooltip: "Tool no longer takes double or more damage from certain actions",
        explanation: "Prevents the tool from taking more than 1 damage in an action",
    };

    const STRING: Material = Material {
        name: "String",
        tier: 1,
        head: None,
        handle: None,
        binding: Some(BindingStats {}),
        bowstring: Some(BowstringStats {}),
        limb: None,
        grip: None,
        melee_trait: Some(STRINGY),
        ranged_trait: Some(STRINGY),
        overslime: false,
        mod_compat: false,
    };

    const STRINGY: Trait = Trait {
        name: "Stringy",
        flavor: "Like cheese, but less tasty",
        tooltip: "Tool can be repaired using string",
        explanation: "Allows the tool to be repaired using string in the tinker station or crafting table",
    };

    const VINE: Material = Material {
        name: "Vine",
        tier: 1,
        head: None,
        handle: None,
        binding: Some(BindingStats {}),
        bowstring: Some(BowstringStats {}),
        limb: None,
        grip: None,
        melee_trait: Some(SOLAR_POWERED),
        ranged_trait: Some(SOLAR_POWERED),
        overslime: false,
        mod_compat: false,
    };

    const SOLAR_POWERED: Trait = Trait {
        name: "Solar Powered",
        flavor: "Good for the environment",
        tooltip: "Tool resists damage in sunlight",
        explanation: "Tool has up to a 75% chance to resist damage in full sunlight",
    };

    pub const MATERIALS: &[&Material] = &[
        &WOOD, &BAMBOO, &ROCK, &FLINT, &BONE, &COPPER, &CHORUS, &LEATHER, &STRING, &VINE,
    ];
}

pub mod tier2 {
    use super::*;

    const IRON: Material = Material {
        name: "Iron",
        tier: 2,
        head: Some(HeadStats {
            durability: Base(250.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(6.0),
            attack_damage: Base(2.0),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.1),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(250.0),
            draw_speed: FlatMod(-0.2),
            velocity: FlatMod(0.1),
            accuracy: FlatMod(0.0),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.1),
            accuracy: FlatMod(0.0),
            attack_damage: Base(2.0),
        }),
        melee_trait: Some(MAGNETIC),
        ranged_trait: Some(MAGNETIC),
        overslime: false,
        mod_compat: false,
    };

    const MAGNETIC: Trait = Trait {
        name: "Magnetic",
        flavor: "Magnetic Personality!",
        tooltip: "Hitting things attracts nearby items?!",
        explanation: "Gives an extra 15% durability per part",
    };

    const SEARED_STONE: Material = Material {
        name: "Seared Stone",
        tier: 2,
        head: Some(HeadStats {
            durability: Base(225.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(6.5),
            attack_damage: Base(1.5),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.85),
            mining_speed: Multiplier(1.1),
            attack_damage: Multiplier(1.05),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: None,
        grip: None,
        melee_trait: Some(SEARING),
        ranged_trait: Some(SEARING),
        overslime: false,
        mod_compat: false,
    };

    const SEARING: Trait = Trait {
        name: "Searing",
        flavor: "Harness the Smeltery!",
        tooltip: "Tool mines meltable blocks such as metal ores faster",
        explanation: "Gives +6 mining speed against blocks that can be melted in a melter",
    };

    const BLOODBONE: Material = Material {
        name: "Bloodbone",
        tier: 2,
        head: Some(HeadStats {
            durability: Base(175.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(4.5),
            attack_damage: Base(2.25),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.9),
            attack_speed: Multiplier(1.1),
            attack_damage: Multiplier(1.05),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(175.0),
            draw_speed: FlatMod(0.1),
            velocity: FlatMod(-0.1),
            accuracy: FlatMod(0.05),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.9),
            accuracy: FlatMod(-0.1),
            attack_damage: Base(2.25),
        }),
        melee_trait: Some(RAGING),
        ranged_trait: Some(RAGING_RANGED),
        overslime: false,
        mod_compat: false,
    };

    const RAGING: Trait = Trait {
        name: "Raging",
        flavor: "A desperate solution!",
        tooltip: "Tool does more damage and draws faster when the holder's health is low",
        explanation: "Boosts damage at low health, up to +4 damage per level at 1 heart",
    };

    const RAGING_RANGED: Trait = Trait {
        name: "Raging",
        flavor: "A desperate solution!",
        tooltip: "Tool does more damage and draws faster when the holder's health is low",
        explanation: "Boosts drawspeed at low health, up to +0.25 drawspeed per level at 1 heart",
    };

    const SLIMEWOOD: Material = Material {
        name: "Slimewood",
        tier: 2,
        head: Some(HeadStats {
            durability: Base(375.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(4.0),
            attack_damage: Base(1.0),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.3),
            mining_speed: Multiplier(0.85),
            attack_damage: Multiplier(0.85),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(375.0),
            draw_speed: FlatMod(0.0),
            velocity: FlatMod(-0.05),
            accuracy: FlatMod(0.1),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.4),
            accuracy: FlatMod(-0.2),
            attack_damage: Base(1.0),
        }),
        melee_trait: Some(OVERGROWTH),
        ranged_trait: Some(OVERGROWTH),
        overslime: true,
        mod_compat: false,
    };

    const OVERGROWTH: Trait = Trait {
        name: "Overgrowth",
        flavor: "Fungal!",
        tooltip: "Tool slowly grows overslime",
        explanation: "Gives a 5% chance per level of regenerating overslime each second",
    };

    const NECROTIC_BONE: Material = Material {
        name: "Necrotic Bone",
        tier: 2,
        head: Some(HeadStats {
            durability: Base(125.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(4.0),
            attack_damage: Base(2.25),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.7),
            attack_speed: Multiplier(1.15),
            attack_damage: Multiplier(1.05),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(125.0),
            draw_speed: FlatMod(0.05),
            velocity: FlatMod(0.05),
            accuracy: FlatMod(-0.15),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.7),
            accuracy: FlatMod(0.1),
            attack_damage: Base(2.25),
        }),
        melee_trait: Some(NECROTIC),
        ranged_trait: Some(NECROTIC_RANGED),
        overslime: false,
        mod_compat: false,
    };

    const NECROTIC: Trait = Trait {
        name: "Necrotic",
        flavor: "Vampiric!",
        tooltip: "Critical hits on enemies drains their life to heal you",
        explanation: "Heals a percentage of damage dealt from critical hits",
    };

    const NECROTIC_RANGED: Trait = Trait {
        name: "Necrotic",
        flavor: "Vampiric!",
        tooltip: "Critical hits on enemies drains their life to heal you",
        explanation: "Heals a percentage of damage dealt from successful arrow hits",
    };

    const SCORCHED_STONE: Material = Material {
        name: "Scorched Stone",
        tier: 2,
        head: Some(HeadStats {
            durability: Base(120.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(4.5),
            attack_damage: Base(2.5),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.8),
            attack_speed: Multiplier(1.05),
            attack_damage: Multiplier(1.1),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: None,
        grip: None,
        melee_trait: Some(SCORCHING),
        ranged_trait: Some(SCORCHING),
        overslime: false,
        mod_compat: false,
    };

    const SCORCHING: Trait = Trait {
        name: "Scorching",
        flavor: "Harness the Foundry!",
        tooltip: "Tool does bonus damage to targets that are on fire",
        explanation: "Gives +2 damage per level against burning targets",
    };

    const WHITESTONE: Material = Material {
        name: "Whitestone",
        tier: 2,
        head: Some(HeadStats {
            durability: Base(275.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(6.0),
            attack_damage: Base(1.25),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.95),
            mining_speed: Multiplier(1.1),
            attack_speed: Multiplier(0.95),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: None,
        grip: None,
        melee_trait: Some(STONESHIELD),
        ranged_trait: Some(STONESHIELD),
        overslime: false,
        mod_compat: false,
    };

    const STONESHIELD: Trait = Trait {
        name: "Stoneshield",
        flavor: "Masterful stonebending",
        tooltip: "Mined stone has a chance to protect the tool from future damage",
        explanation: "Mining stone has a 20% per level of granting +3 stoneshield",
    };

    const OSMIUM: Material = Material {
        name: "Osmium",
        tier: 2,
        head: Some(HeadStats {
            durability: Base(500.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(4.5),
            attack_damage: Base(2.0),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.2),
            attack_speed: Multiplier(0.9),
            mining_speed: Multiplier(0.9),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: None,
        grip: None,
        melee_trait: Some(DENSE),
        ranged_trait: Some(DENSE),
        overslime: false,
        mod_compat: true,
    };

    const DENSE: Trait = Trait {
        name: "Dense",
        flavor: "Barely loses its shape",
        tooltip: "Tool lasts longer, but is harder to repair",
        explanation: "Prevents 33% of damage at the cost of an extra 25% repair cost at first level",
    };

    const TUNGSTEN: Material = Material {
        name: "Tungsten",
        tier: 2,
        head: Some(HeadStats {
            durability: Base(350.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(6.5),
            attack_damage: Base(1.75),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.9),
            mining_speed: Multiplier(1.1),
            attack_speed: Multiplier(0.9),
            attack_damage: Multiplier(1.1),
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(350.0),
            draw_speed: FlatMod(0.2),
            velocity: FlatMod(-0.3),
            accuracy: FlatMod(0.0),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.9),
            accuracy: FlatMod(0.1),
            attack_damage: Base(1.75),
        }),
        melee_trait: Some(SHARPWEIGHT),
        ranged_trait: Some(SHARPWEIGHT),
        overslime: false,
        mod_compat: true,
    };

    const SHARPWEIGHT: Trait = Trait {
        name: "Sharpweight",
        flavor: "As opposed to a dumbweight",
        tooltip: "Mines faster, but is extremely hard to move",
        explanation: "Grants +10% mining speed and -10% movement speed per level",
    };

    const PLATINUM: Material = Material {
        name: "Platinum",
        tier: 2,
        head: Some(HeadStats {
            durability: Base(400.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(7.0),
            attack_damage: Base(1.5),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.05),
            mining_speed: Multiplier(1.05),
            attack_speed: Multiplier(0.95),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(400.0),
            draw_speed: FlatMod(-0.05),
            velocity: FlatMod(0.0),
            accuracy: FlatMod(0.1),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.05),
            accuracy: FlatMod(0.05),
            attack_damage: Base(1.5),
        }),
        melee_trait: Some(LUSTROUS),
        ranged_trait: Some(OLYMPIC),
        overslime: false,
        mod_compat: true,
    };

    const LUSTROUS: Trait = Trait {
        name: "Lustrous",
        flavor: "The shiniest of ores!",
        tooltip: "Ores drop bonus nuggets",
        explanation: "Grants 2-6 bonus nugget drops when mining ore, boosted at higher levels",
    };

    const OLYMPIC: Trait = Trait {
        name: "Olympic",
        flavor: "The shiniest of medals!",
        tooltip: "Earn medals by hitting far away targets",
        explanation: "Hitting a distant monster has a 5% chance per level to drop nuggets. 50 blocks or more drops platinum, 40 blocks gold, 30 blocks iron, and 20 blocks copper.",
    };

    const ALUMINUM: Material = Material {
        name: "Aluminum",
        tier: 2,
        head: None,
        handle: None,
        binding: None,
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(225.0),
            draw_speed: FlatMod(0.15),
            velocity: FlatMod(-0.15),
            accuracy: FlatMod(-0.05),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.85),
            accuracy: FlatMod(0.15),
            attack_damage: Base(2.0),
        }),
        melee_trait: None,
        ranged_trait: Some(FEATHERWEIGHT),
        overslime: false,
        mod_compat: true,
    };

    const FEATHERWEIGHT: Trait = Trait {
        name: "Featherweight",
        flavor: "Light as a feather",
        tooltip: "Tool is so easy to use, it has higher drawspeed and accuracy",
        explanation: "Grants +7% drawspeed and accuracy",
    };

    const SILVER: Material = Material {
        name: "Silver",
        tier: 2,
        head: Some(HeadStats {
            durability: Base(300.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(5.5),
            attack_damage: Base(2.25),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.9),
            mining_speed: Multiplier(1.05),
            attack_speed: Multiplier(1.1),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(300.0),
            draw_speed: FlatMod(-0.15),
            velocity: FlatMod(0.1),
            accuracy: FlatMod(-0.1),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.9),
            accuracy: FlatMod(-0.05),
            attack_damage: Base(2.25),
        }),
        melee_trait: Some(SMITE),
        ranged_trait: Some(HOLY),
        overslime: false,
        mod_compat: true,
    };

    const SMITE: Trait = Trait {
        name: "Smite",
        flavor: "Die, Foul Creature!",
        tooltip: "Undead fear the might of the melon!",
        explanation: "Gives +2.5 damage against undead per level",
    };

    const HOLY: Trait = Trait {
        name: "Holy",
        flavor: "Die, Fowl Creature!",
        tooltip: "Arrows deal bonus damage against undead",
        explanation: "Gives +0.5 projectile damage against undead per level",
    };

    const LEAD: Material = Material {
        name: "Lead",
        tier: 2,
        head: Some(HeadStats {
            durability: Base(200.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(5.0),
            attack_damage: Base(2.5),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.9),
            attack_speed: Multiplier(0.9),
            attack_damage: Multiplier(1.2),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(200.0),
            draw_speed: FlatMod(-0.3),
            velocity: FlatMod(0.15),
            accuracy: FlatMod(-0.05),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.9),
            accuracy: FlatMod(-0.1),
            attack_damage: Base(2.5),
        }),
        melee_trait: Some(HEAVY),
        ranged_trait: Some(HEAVY),
        overslime: false,
        mod_compat: true,
    };

    const HEAVY: Trait = Trait {
        name: "Heavy",
        flavor: "Dense!",
        tooltip: "Hard to move while wielding, but it packs a heavy punch",
        explanation: "Grants +10% attack and -10% movement speed per level",
    };

    const CHAIN: Material = Material {
        name: "Chain",
        tier: 2,
        head: None,
        handle: None,
        binding: Some(BindingStats {}),
        bowstring: Some(BowstringStats {}),
        limb: None,
        grip: None,
        melee_trait: Some(REINFORCED),
        ranged_trait: Some(REINFORCED),
        overslime: false,
        mod_compat: false,
    };

    const REINFORCED: Trait = Trait {
        name: "Reinforced",
        flavor: "Long Lasting!",
        tooltip: "Tool is less likely to take damage",
        explanation: "Tool takes 25% less damage",
    };

    const SLIMY_VINE: Material = Material {
        name: "Slimy Vine",
        tier: 2,
        head: None,
        handle: None,
        binding: Some(BindingStats {}),
        bowstring: Some(BowstringStats {}),
        limb: None,
        grip: None,
        melee_trait: Some(AIRBORNE),
        ranged_trait: Some(AIRBORNE_RANGED),
        overslime: false,
        mod_compat: false,
    };

    const AIRBORNE: Trait = Trait {
        name: "Airborne",
        flavor: "Take to the skies!",
        tooltip: "Tool no longer is penalized when mining in the air and gains a projectile accuracy bonus",
        explanation: "Cancels out the mining speed reduction when not on the ground",
    };

    const AIRBORNE_RANGED: Trait = Trait {
        name: "Airborne",
        flavor: "Take to the skies!",
        tooltip: "Tool no longer is penalized when mining in the air and gains a projectile accuracy bonus",
        explanation: "Grants +0.5 accuracy when not on the ground or a ladder",
    };

    pub const MATERIALS: &[&Material] = &[
        &IRON,
        &SEARED_STONE,
        &BLOODBONE,
        &SLIMEWOOD,
        &NECROTIC_BONE,
        &SCORCHED_STONE,
        &WHITESTONE,
        &OSMIUM,
        &TUNGSTEN,
        &PLATINUM,
        &ALUMINUM,
        &SILVER,
        &LEAD,
        &CHAIN,
        &SLIMY_VINE,
    ];
}

pub mod tier3 {
    use super::*;

    const SLIMESTEEL: Material = Material {
        name: "Slimesteel",
        tier: 3,
        head: Some(HeadStats {
            durability: Base(1040.0),
            harvest_tier: HarvestTier::Diamond,
            mining_speed: Base(6.0),
            attack_damage: Base(2.5),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.2),
            attack_speed: Multiplier(0.95),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(1040.0),
            draw_speed: FlatMod(-0.1),
            velocity: FlatMod(-0.05),
            accuracy: FlatMod(0.15),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.2),
            accuracy: FlatMod(-0.1),
            attack_damage: Base(2.5),
        }),
        melee_trait: Some(OVERCAST),
        ranged_trait: Some(OVERCAST),
        overslime: true,
        mod_compat: false,
    };

    const OVERCAST: Trait = Trait {
        name: "Overcast",
        flavor: "Thick Coverage!",
        tooltip: "Overslime can be made extra thick on this tool",
        explanation: "Grants 25 extra overslime, +50% of the total per level",
    };

    const AMETHYST_BRONZE: Material = Material {
        name: "Amethyst Bronze",
        tier: 3,
        head: Some(HeadStats {
            durability: Base(720.0),
            harvest_tier: HarvestTier::Diamond,
            mining_speed: Base(7.0),
            attack_damage: Base(1.5),
        }),
        handle: Some(HandleStats {
            mining_speed: Multiplier(1.1),
            attack_speed: Multiplier(1.05),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(720.0),
            draw_speed: FlatMod(-0.25),
            velocity: FlatMod(0.15),
            accuracy: FlatMod(-0.1),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.0),
            accuracy: FlatMod(0.1),
            attack_damage: Base(1.5),
        }),
        melee_trait: Some(CRUMBLING),
        ranged_trait: Some(CRYSTALBOUND),
        overslime: false,
        mod_compat: false,
    };

    const CRUMBLING: Trait = Trait {
        name: "Crumbling",
        flavor: "Hold yourself together!",
        tooltip: "Tool mines blocks that don't require a tool faster",
        explanation: "Grants +0.5 mining speed per level to blocks that don't require tools, even ones where the tool is not effective",
    };

    const CRYSTALBOUND: Trait = Trait {
        name: "Crystalbound",
        flavor: "The way of the crystal is straight",
        tooltip: "Tool gains bonus velocity, but arrow angle is restricted",
        explanation: "Adds +0.1 velocity per level at the cost of limiting arrow angles to 32, 16, or 8 directions at the first 3 levels respectively",
    };

    const NAHUATL: Material = Material {
        name: "Nahuatl",
        tier: 3,
        head: Some(HeadStats {
            durability: Base(350.0),
            harvest_tier: HarvestTier::Diamond,
            mining_speed: Base(4.5),
            attack_damage: Base(3.0),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.9),
            attack_speed: Multiplier(0.9),
            attack_damage: Multiplier(1.3),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(350.0),
            draw_speed: FlatMod(0.2),
            velocity: FlatMod(-0.15),
            accuracy: FlatMod(0.1),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.9),
            accuracy: FlatMod(-0.15),
            attack_damage: Base(3.0),
        }),
        melee_trait: Some(LACERATING),
        ranged_trait: Some(LACERATING),
        overslime: false,
        mod_compat: false,
    };

    const LACERATING: Trait = Trait {
        name: "Lacerating",
        flavor: "Sharp Edges!",
        tooltip: "Causes the target to bleed, dealing additional damage",
        explanation: "Has a 50% chance of doing up to 5 damage over time, more damage at higher levels",
    };

    const PIG_IRON: Material = Material {
        name: "Pig Iron",
        tier: 3,
        head: Some(HeadStats {
            durability: Base(580.0),
            harvest_tier: HarvestTier::Diamond,
            mining_speed: Base(6.0),
            attack_damage: Base(2.5),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.1),
            mining_speed: Multiplier(0.85),
            attack_damage: Multiplier(1.1),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: None,
        grip: None,
        melee_trait: Some(TASTY),
        ranged_trait: Some(TASTY),
        overslime: false,
        mod_compat: false,
    };

    const TASTY: Trait = Trait {
        name: "Tasty",
        flavor: "Om Nom Nom",
        tooltip: "Smells so good, you cannot resist taking a few bites of your tool",
        explanation: "Can be eaten to restore 1 hunger and 0.1 saturation per level",
    };

    const ROSE_GOLD: Material = Material {
        name: "Rose Gold",
        tier: 3,
        head: Some(HeadStats {
            durability: Base(175.0),
            harvest_tier: HarvestTier::Gold,
            mining_speed: Base(9.0),
            attack_damage: Base(1.0),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.6),
            mining_speed: Multiplier(1.2),
            attack_speed: Multiplier(1.2),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(175.0),
            draw_speed: FlatMod(0.15),
            velocity: FlatMod(-0.25),
            accuracy: FlatMod(0.15),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.6),
            accuracy: FlatMod(0.25),
            attack_damage: Base(1.0),
        }),
        melee_trait: Some(ENHANCED),
        ranged_trait: Some(ENHANCED),
        overslime: false,
        mod_compat: false,
    };

    const ENHANCED: Trait = Trait {
        name: "Enhanced",
        flavor: "Flashy!",
        tooltip: "Rose gold goes great with a bonus upgrade!",
        explanation: "Grants 1 extra upgrade slot per part",
    };

    const ELECTRUM: Material = Material {
        name: "Electrum",
        tier: 3,
        head: Some(HeadStats {
            durability: Base(225.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(8.5),
            attack_damage: Base(1.5),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.8),
            attack_speed: Multiplier(1.15),
            mining_speed: Multiplier(1.15),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(225.0),
            draw_speed: FlatMod(-0.25),
            velocity: FlatMod(0.1),
            accuracy: FlatMod(0.15),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.8),
            accuracy: FlatMod(0.2),
            attack_damage: Base(1.5),
        }),
        melee_trait: Some(EXPERIENCED),
        ranged_trait: Some(EXPERIENCED_RANGED),
        overslime: false,
        mod_compat: true,
    };

    const EXPERIENCED: Trait = Trait {
        name: "Experienced",
        flavor: "Fast Learner!",
        tooltip: "Gain bonus XP from killing mobs and mining blocks",
        explanation: "Grants +50% XP per level from blocks and monsters",
    };

    const EXPERIENCED_RANGED: Trait = Trait {
        name: "Experienced",
        flavor: "Fast Learner!",
        tooltip: "Gain bonus XP from killing mobs and mining blocks",
        explanation: "Grants +50% XP per level from monsters killed by arrows",
    };

    const PLATED_SLIMEWOOD: Material = Material {
        name: "Plated Slimewood",
        tier: 3,
        head: Some(HeadStats {
            durability: Base(595.0),
            harvest_tier: HarvestTier::Diamond,
            mining_speed: Base(5.0),
            attack_damage: Base(2.0),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.25),
            mining_speed: Multiplier(0.9),
            attack_speed: Multiplier(0.9),
            attack_damage: Multiplier(1.05),
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(595.0),
            draw_speed: FlatMod(0.15),
            velocity: FlatMod(-0.15),
            accuracy: FlatMod(0.0),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.25),
            accuracy: FlatMod(-0.1),
            attack_damage: Base(2.0),
        }),
        melee_trait: Some(OVERWORKED),
        ranged_trait: Some(OVERWORKED),
        overslime: true,
        mod_compat: true,
    };

    const OVERWORKED: Trait = Trait {
        name: "Overworked",
        flavor: "Underpaid",
        tooltip: "Increases amount of overslime restored from all sources",
        explanation: "Grants +100% to all methods of restoring overslime",
    };

    const STEEL: Material = Material {
        name: "Steel",
        tier: 3,
        head: Some(HeadStats {
            durability: Base(775.0),
            harvest_tier: HarvestTier::Diamond,
            mining_speed: Base(6.0),
            attack_damage: Base(2.75),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.05),
            mining_speed: Multiplier(1.05),
            attack_speed: Multiplier(1.05),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(775.0),
            draw_speed: FlatMod(-0.3),
            velocity: FlatMod(0.2),
            accuracy: FlatMod(-0.1),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.05),
            accuracy: FlatMod(-0.05),
            attack_damage: Base(2.75),
        }),
        melee_trait: Some(DUCTILE),
        ranged_trait: Some(DUCTILE),
        overslime: false,
        mod_compat: true,
    };

    const DUCTILE: Trait = Trait {
        name: "Ductile",
        flavor: "Reshapes well!",
        tooltip: "Tool is longer lasting, mines faster, and damages more",
        explanation: "Grants +4% durability, damage, and speed per level",
    };

    const BRONZE: Material = Material {
        name: "Bronze",
        tier: 3,
        head: Some(HeadStats {
            durability: Base(760.0),
            harvest_tier: HarvestTier::Diamond,
            mining_speed: Base(6.5),
            attack_damage: Base(2.25),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.1),
            mining_speed: Multiplier(1.05),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(760.0),
            draw_speed: FlatMod(-0.2),
            velocity: FlatMod(0.15),
            accuracy: FlatMod(-0.2),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.1),
            accuracy: FlatMod(0.0),
            attack_damage: Base(2.25),
        }),
        melee_trait: Some(MAINTAINED),
        ranged_trait: Some(MAINTAINED),
        overslime: false,
        mod_compat: true,
    };

    const MAINTAINED: Trait = Trait {
        name: "Maintained",
        flavor: "Stays Sharper, Longer!",
        tooltip: "Tool mines faster when at higher durability",
        explanation: "Grants +6 mining speed per level at full durability, 0 boost below 50% durability",
    };

    const CONSTANTAN: Material = Material {
        name: "Constantan",
        tier: 3,
        head: Some(HeadStats {
            durability: Base(675.0),
            harvest_tier: HarvestTier::Diamond,
            mining_speed: Base(7.5),
            attack_damage: Base(1.75),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.95),
            mining_speed: Multiplier(1.15),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(675.0),
            draw_speed: FlatMod(0.2),
            velocity: FlatMod(-0.05),
            accuracy: FlatMod(-0.25),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.95),
            accuracy: FlatMod(0.1),
            attack_damage: Base(1.75),
        }),
        melee_trait: Some(TEMPERATE),
        ranged_trait: Some(TEMPERATE),
        overslime: false,
        mod_compat: true,
    };

    const TEMPERATE: Trait = Trait {
        name: "Temperate",
        flavor: "Extreme Mining!",
        tooltip: "Tool mines faster when in extreme temperatures",
        explanation: "Grants +7.5 mining speed in extreme cold and twice reinforced in extreme heat",
    };

    const INVAR: Material = Material {
        name: "Invar",
        tier: 3,
        head: Some(HeadStats {
            durability: Base(630.0),
            harvest_tier: HarvestTier::Diamond,
            mining_speed: Base(5.5),
            attack_damage: Base(2.5),
        }),
        handle: Some(HandleStats {
            mining_speed: Multiplier(0.9),
            attack_damage: Multiplier(1.2),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(630.0),
            draw_speed: FlatMod(-0.15),
            velocity: FlatMod(-0.1),
            accuracy: FlatMod(0.2),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.0),
            accuracy: FlatMod(0.05),
            attack_damage: Base(2.5),
        }),
        melee_trait: Some(INVARIANT),
        ranged_trait: Some(INVARIANT),
        overslime: false,
        mod_compat: true,
    };

    const INVARIANT: Trait = Trait {
        name: "Invariant",
        flavor: "Couldn't care less",
        tooltip: "Tool does more damage in neutral temperatures",
        explanation: "Grants +2.5 damage in biomes with neutral temperatures",
    };

    const NECRONIUM: Material = Material {
        name: "Necronium",
        tier: 3,
        head: Some(HeadStats {
            durability: Base(357.0),
            harvest_tier: HarvestTier::Diamond,
            mining_speed: Base(4.0),
            attack_damage: Base(2.75),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.8),
            attack_speed: Multiplier(1.15),
            attack_damage: Multiplier(1.1),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(357.0),
            draw_speed: FlatMod(0.15),
            velocity: FlatMod(-0.1),
            accuracy: FlatMod(-0.05),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.8),
            accuracy: FlatMod(0.15),
            attack_damage: Base(2.75),
        }),
        melee_trait: Some(DECAY),
        ranged_trait: Some(DECAY),
        overslime: false,
        mod_compat: true,
    };

    const DECAY: Trait = Trait {
        name: "Decay",
        flavor: "Radioactive Withering",
        tooltip: "Causes targets to wither away, but may also cause the holder to wither",
        explanation: "Causes at least 5 seconds of wither, to your target and at a 25% chance to you",
    };

    const COBALT: Material = Material {
        name: "Cobalt",
        tier: 3,
        head: Some(HeadStats {
            durability: Base(800.0),
            harvest_tier: HarvestTier::Diamond,
            mining_speed: Base(6.5),
            attack_damage: Base(2.25),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.05),
            mining_speed: Multiplier(1.05),
            attack_speed: Multiplier(1.05),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(800.0),
            draw_speed: FlatMod(0.05),
            velocity: FlatMod(0.05),
            accuracy: FlatMod(0.05),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.05),
            accuracy: FlatMod(0.05),
            attack_damage: Base(2.25),
        }),
        melee_trait: Some(LIGHTWEIGHT),
        ranged_trait: Some(LIGHTWEIGHT_RANGED),
        overslime: false,
        mod_compat: false,
    };

    const LIGHTWEIGHT: Trait = Trait {
        name: "Lightweight",
        flavor: "Like a Shardblade!",
        tooltip: "Tool is incredibly light for its size, allowing you to attack, mine, and launch projectiles faster",
        explanation: "Boosts mining and attack speed by 7% per level",
    };

    const LIGHTWEIGHT_RANGED: Trait = Trait {
        name: "Lightweight",
        flavor: "Like a Shardblade!",
        tooltip: "Tool is incredibly light for its size, allowing you to attack, mine, and launch projectiles faster",
        explanation: "Boosts drawspeed and velocity by 5% per level",
    };

    const DARKTHREAD: Material = Material {
        name: "Darkthread",
        tier: 3,
        head: None,
        handle: None,
        binding: Some(BindingStats {}),
        bowstring: Some(BowstringStats {}),
        limb: None,
        grip: None,
        melee_trait: Some(LOOTING),
        ranged_trait: Some(LOOTING_RANGED),
        overslime: false,
        mod_compat: false,
    };

    const LOOTING: Trait = Trait {
        name: "Looting",
        flavor: "Luck of the Mobs!",
        tooltip: "Gives you more nice things when killing mobs!",
        explanation: "Increases drop chances from mobs, stacking with luck",
    };

    const LOOTING_RANGED: Trait = Trait {
        name: "Looting",
        flavor: "Luck of the Mobs!",
        tooltip: "Gives you more nice things when killing mobs!",
        explanation: "Increases drop chances from mobs killed by arrows",
    };

    pub const MATERIALS: &[&Material] = &[
        &SLIMESTEEL,
        &AMETHYST_BRONZE,
        &NAHUATL,
        &PIG_IRON,
        &ROSE_GOLD,
        &ELECTRUM,
        &PLATED_SLIMEWOOD,
        &STEEL,
        &BRONZE,
        &CONSTANTAN,
        &INVAR,
        &NECRONIUM,
        &COBALT,
        &DARKTHREAD,
    ];
}

pub mod tier4 {
    use super::*;

    const QUEENS_SLIME: Material = Material {
        name: "Queen's Slime",
        tier: 4,
        head: Some(HeadStats {
            durability: Base(1650.0),
            harvest_tier: HarvestTier::Netherite,
            mining_speed: Base(6.0),
            attack_damage: Base(2.0),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.35),
            mining_speed: Multiplier(0.9),
            attack_speed: Multiplier(0.95),
            attack_damage: Multiplier(0.95),
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(1650.0),
            draw_speed: FlatMod(0.0),
            velocity: FlatMod(-0.15),
            accuracy: FlatMod(0.2),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.35),
            accuracy: FlatMod(-0.15),
            attack_damage: Base(2.0),
        }),
        melee_trait: Some(OVERLORD),
        ranged_trait: Some(OVERLORD),
        overslime: true,
        mod_compat: false,
    };

    const OVERLORD: Trait = Trait {
        name: "Overlord",
        flavor: "Regal!",
        tooltip: "Tool takes some of the durability as new overslime subjects",
        explanation: "Reduces durability by 15% per level, 66% of that going towards overslime",
    };

    const HEPATIZON: Material = Material {
        name: "Hepatizon",
        tier: 4,
        head: Some(HeadStats {
            durability: Base(975.0),
            harvest_tier: HarvestTier::Netherite,
            mining_speed: Base(8.0),
            attack_damage: Base(2.5),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.1),
            mining_speed: Multiplier(1.2),
            attack_damage: Multiplier(0.9),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(975.0),
            draw_speed: FlatMod(0.25),
            velocity: FlatMod(-0.05),
            accuracy: FlatMod(-0.1),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.1),
            accuracy: FlatMod(0.15),
            attack_damage: Base(2.5),
        }),
        melee_trait: Some(MOMENTUM),
        ranged_trait: Some(MOMENTUM_RANGED),
        overslime: false,
        mod_compat: false,
    };

    const MOMENTUM: Trait = Trait {
        name: "Momentum",
        flavor: "Not Fast Enough, Gotta Go Faster!",
        tooltip: "Mining blocks and firing arrows increases your speed, as long as you keep going",
        explanation: "Boosts mining speed by 25% per level after mining 32 consecutive blocks",
    };

    const MOMENTUM_RANGED: Trait = Trait {
        name: "Momentum",
        flavor: "Not Fast Enough, Gotta Go Faster!",
        tooltip: "Mining blocks and firing arrows increases your speed, as long as you keep going",
        explanation: "Boosts drawspeed by 25% per level after firing 16 consecutive arrows",
    };

    const MANYULLYN: Material = Material {
        name: "Manyullyn",
        tier: 4,
        head: Some(HeadStats {
            durability: Base(1250.0),
            harvest_tier: HarvestTier::Netherite,
            mining_speed: Base(6.5),
            attack_damage: Base(3.5),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(1.1),
            mining_speed: Multiplier(0.9),
            attack_speed: Multiplier(0.95),
            attack_damage: Multiplier(1.25),
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(1250.0),
            draw_speed: FlatMod(-0.35),
            velocity: FlatMod(0.25),
            accuracy: FlatMod(-0.15),
        }),
        grip: Some(GripStats {
            durability: Multiplier(1.1),
            accuracy: FlatMod(-0.2),
            attack_damage: Base(3.5),
        }),
        melee_trait: Some(INSATIABLE),
        ranged_trait: Some(INSATIABLE_RANGED),
        overslime: false,
        mod_compat: false,
    };

    const INSATIABLE: Trait = Trait {
        name: "Insatiable",
        flavor: "Taste For Blood!",
        tooltip: "During combat you deal more and more damage every time you hit an enemy",
        explanation: "Grants +2 damage per level after 8 consecutive hits",
    };

    const INSATIABLE_RANGED: Trait = Trait {
        name: "Insatiable",
        flavor: "Taste For Blood!",
        tooltip: "During combat you deal more and more damage every time you hit an enemy",
        explanation: "Arrows gain +1 damage per level (before velocity) after 8 consecutive arrow hits",
    };

    const BLAZING_BONE: Material = Material {
        name: "Blazing Bone",
        tier: 4,
        head: Some(HeadStats {
            durability: Base(530.0),
            harvest_tier: HarvestTier::Iron,
            mining_speed: Base(6.0),
            attack_damage: Base(3.0),
        }),
        handle: Some(HandleStats {
            durability: Multiplier(0.85),
            attack_damage: Multiplier(1.05),
            attack_speed: Multiplier(1.2),
            ..HandleStats::default()
        }),
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: Some(LimbStats {
            durability: FlatMod(530.0),
            draw_speed: FlatMod(-0.3),
            velocity: FlatMod(0.2),
            accuracy: FlatMod(-0.15),
        }),
        grip: Some(GripStats {
            durability: Multiplier(0.85),
            accuracy: FlatMod(-0.1),
            attack_damage: Base(3.0),
        }),
        melee_trait: Some(CONDUCTING),
        ranged_trait: Some(CONDUCTING_RANGED),
        overslime: false,
        mod_compat: false,
    };

    const CONDUCTING: Trait = Trait {
        name: "Conducting",
        flavor: "Thermodynamic!",
        tooltip: "Tool deals multiplied damage when you are on fire",
        explanation: "Grants +1% damage per second of fire, up to +15% from lava's 15 seconds. Bonus reduced by fire resistance.",
    };

    const CONDUCTING_RANGED: Trait = Trait {
        name: "Conducting",
        flavor: "Thermodynamic!",
        tooltip: "Tool deals multiplied damage when you are on fire",
        explanation: "Grants +1% arrow damage per second of fire, up to +15% from lava's 15 seconds. Bonus reduced by fire resistance.",
    };

    const ANCIENT_HIDE: Material = Material {
        name: "Ancient Hide",
        tier: 4,
        head: None,
        handle: None,
        binding: Some(BindingStats {}),
        bowstring: None,
        limb: None,
        grip: None,
        melee_trait: Some(FORTUNE),
        ranged_trait: Some(FORTUNE),
        overslime: false,
        mod_compat: false,
    };

    const FORTUNE: Trait = Trait {
        name: "Fortune",
        flavor: "Luck of the Ores!",
        tooltip: "Gives you more nice things when mining!",
        explanation: "Increases drop chances from blocks, stacking with luck",
    };

    const SLIMY_VINE: Material = Material {
        name: "Slimy Vine",
        tier: 4,
        head: None,
        handle: None,
        binding: Some(BindingStats {}),
        bowstring: Some(BowstringStats {}),
        limb: None,
        grip: None,
        melee_trait: Some(ENDERPORTING),
        ranged_trait: Some(ENDERPORTING_RANGED),
        overslime: false,
        mod_compat: false,
    };

    const ENDERPORTING: Trait = Trait {
        name: "Enderporting",
        flavor: "Harness the power of the enderslime",
        tooltip: "Tool teleports the holder to mined blocks or killed entities",
        explanation: "Teleports the holder to the location of mined blocks or swaps the holder with attacked monsters",
    };

    const ENDERPORTING_RANGED: Trait = Trait {
        name: "Enderporting",
        flavor: "Harness the power of the enderslime",
        tooltip: "Tool teleports the holder to mined blocks or killed entities",
        explanation: "Teleports the holder to the location of arrows that land in blocks or swaps the holder with monsters hit by arrows",
    };

    pub const MATERIALS: &[&Material] = &[
        &QUEENS_SLIME,
        &HEPATIZON,
        &MANYULLYN,
        &BLAZING_BONE,
        &ANCIENT_HIDE,
        &SLIMY_VINE,
    ];
}
