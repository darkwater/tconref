use core::fmt::Display;

use super::materials::{Base, FlatMod, Multiplier};

pub struct Tool {
    pub name: &'static str,
    pub description: &'static str,
    pub parts: &'static [Weighted<ToolPart>],
    pub base: ToolStats<Base>,
    pub flat: ToolStats<FlatMod>,
    pub multiplier: ToolStats<Multiplier>,
    pub traits: &'static [&'static str],
}

#[derive(Clone, Copy)]
pub struct Weighted<T> {
    pub inner: T,
    pub weight: usize,
}

impl<T> Weighted<T> {
    pub const fn single(inner: T) -> Self {
        Self { inner, weight: 1 }
    }

    #[expect(clippy::self_named_constructors)]
    pub const fn weighted(inner: T, weight: usize) -> Self {
        Self { inner, weight }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Weighted<U> {
        Weighted {
            inner: f(self.inner),
            weight: self.weight,
        }
    }
}

impl<T> const From<T> for Weighted<T> {
    fn from(inner: T) -> Self {
        Self::single(inner)
    }
}

#[derive(Clone, Copy)]
pub enum ToolPart {
    Bowstring,
    ToolBinding,
    ToolHandle,
    BowGrip,
    BowLimb,
    PickHead,
    RoundPlate,
    SmallAxeHead,
    SmallBlade,
    ToughHandle,
    LargePlate,
    BroadAxeHead,
    BroadBlade,
    HammerHead,
}

impl Display for ToolPart {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bowstring => "Bowstring",
                Self::ToolBinding => "Tool Binding",
                Self::ToolHandle => "Tool Handle",
                Self::BowGrip => "Bow Grip",
                Self::BowLimb => "Bow Limb",
                Self::PickHead => "Pick Head",
                Self::RoundPlate => "Round Plate",
                Self::SmallAxeHead => "Small Axe Head",
                Self::SmallBlade => "Small Blade",
                Self::ToughHandle => "Tough Handle",
                Self::LargePlate => "Large Plate",
                Self::BroadAxeHead => "Broad Axe Head",
                Self::BroadBlade => "Broad Blade",
                Self::HammerHead => "Hammer Head",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PartType {
    Head,
    Handle,
    Binding,
    Limb,
    Grip,
    Bowstring,
}

impl ToolPart {
    pub const fn part_type(&self) -> PartType {
        match self {
            Self::PickHead
            | Self::RoundPlate
            | Self::SmallAxeHead
            | Self::SmallBlade
            | Self::LargePlate
            | Self::BroadAxeHead
            | Self::BroadBlade
            | Self::HammerHead => PartType::Head,
            Self::ToolHandle | Self::ToughHandle => PartType::Handle,
            Self::ToolBinding => PartType::Binding,
            Self::BowLimb => PartType::Limb,
            Self::BowGrip => PartType::Grip,
            Self::Bowstring => PartType::Bowstring,
        }
    }

    pub const fn material_cost(&self) -> usize {
        match self {
            Self::Bowstring | Self::ToolBinding | Self::ToolHandle => 1,
            Self::BowGrip
            | Self::BowLimb
            | Self::PickHead
            | Self::RoundPlate
            | Self::SmallAxeHead
            | Self::SmallBlade => 2,
            Self::ToughHandle => 3,
            Self::LargePlate => 4,
            Self::BroadAxeHead | Self::BroadBlade | Self::HammerHead => 8,
        }
    }
}

pub struct ToolStats<T> {
    pub durability: T,
    pub attack_damage: T,
    pub attack_speed: T,
    pub mining_speed: T,
    pub draw_speed: T,
    pub arrow_velocity: T,
    pub accuracy: T,
}

impl const Default for ToolStats<Base> {
    fn default() -> Self {
        Self {
            durability: Base(0.),
            attack_damage: Base(0.),
            attack_speed: Base(0.),
            mining_speed: Base(0.),
            draw_speed: Base(0.),
            arrow_velocity: Base(0.),
            accuracy: Base(0.),
        }
    }
}

impl const Default for ToolStats<FlatMod> {
    fn default() -> Self {
        Self {
            durability: FlatMod(0.),
            attack_damage: FlatMod(0.),
            attack_speed: FlatMod(0.),
            mining_speed: FlatMod(0.),
            draw_speed: FlatMod(0.),
            arrow_velocity: FlatMod(0.),
            accuracy: FlatMod(0.),
        }
    }
}

impl const Default for ToolStats<Multiplier> {
    fn default() -> Self {
        Self {
            durability: Multiplier(1.),
            attack_damage: Multiplier(1.),
            attack_speed: Multiplier(1.),
            mining_speed: Multiplier(1.),
            draw_speed: Multiplier(1.),
            arrow_velocity: Multiplier(1.),
            accuracy: Multiplier(1.),
        }
    }
}

const PICKAXE: Tool = Tool {
    name: "Pickaxe",
    description: "The Pickaxe is a precise mining tool. It is effective on stone and ores.\nIt breaks blocks, OK?",
    parts: &[
        ToolPart::PickHead.into(),
        ToolPart::ToolHandle.into(),
        ToolPart::ToolBinding.into(),
    ],
    base: ToolStats {
        attack_speed: Base(1.2),
        ..Default::default()
    },
    flat: ToolStats {
        attack_damage: FlatMod(0.5),
        ..Default::default()
    },
    multiplier: Default::default(),
    traits: &["Piercing I"],
};

const SLEDGE_HAMMER: Tool = Tool {
    name: "Sledge Hammer",
    description: "The Sledge Hammer is a broad mining tool. It harvests blocks in a wide range.\nAlso effective against undead.",
    parts: &[
        Weighted::weighted(ToolPart::HammerHead, 2),
        ToolPart::ToughHandle.into(),
        Weighted::weighted(ToolPart::LargePlate, 1),
        Weighted::weighted(ToolPart::LargePlate, 1),
    ],
    base: ToolStats {
        attack_speed: Base(0.75),
        ..Default::default()
    },
    flat: ToolStats {
        attack_damage: FlatMod(3.),
        ..Default::default()
    },
    multiplier: ToolStats {
        durability: Multiplier(4.),
        attack_damage: Multiplier(1.35),
        mining_speed: Multiplier(0.4),
        ..Default::default()
    },
    traits: &["Smite II"],
};

const VEIN_HAMMER: Tool = Tool {
    name: "Vein Hammer",
    description: "The Vein Hammer is a broad mining tool. It harvests many connected blocks of the same type, perfect for ores.",
    parts: &[
        Weighted::weighted(ToolPart::HammerHead, 2),
        ToolPart::ToughHandle.into(),
        Weighted::weighted(ToolPart::PickHead, 1),
        ToolPart::LargePlate.into(),
    ],
    base: ToolStats {
        attack_speed: Base(0.85),
        ..Default::default()
    },
    flat: ToolStats {
        attack_damage: FlatMod(3.),
        ..Default::default()
    },
    multiplier: ToolStats {
        durability: Multiplier(5.),
        attack_damage: Multiplier(1.25),
        mining_speed: Multiplier(0.3),
        ..Default::default()
    },
    traits: &["Piercing II"],
};

const MATTOCK: Tool = Tool {
    name: "Mattock",
    description: "The Mattock is a farming tool, effective on logs, dirt, gravel, and sand. It is also faster on wood.\nJust don't dig your own grave!",
    parts: &[
        ToolPart::SmallAxeHead.into(),
        ToolPart::ToolHandle.into(),
        ToolPart::RoundPlate.into(),
    ],
    base: ToolStats {
        attack_speed: Base(0.9),
        ..Default::default()
    },
    flat: ToolStats {
        attack_damage: FlatMod(1.5),
        ..Default::default()
    },
    multiplier: ToolStats {
        durability: Multiplier(1.25),
        attack_damage: Multiplier(1.1),
        mining_speed: Multiplier(1.1),
        ..Default::default()
    },
    traits: &["Sticky I", "Tilling"],
};

const PICKADZE: Tool = Tool {
    name: "Pickadze",
    description: "The Pickadze is a versatile mining tool. It is effective on rock, dirt, sand and gravel, but is not sharp enough to mine tougher stone blocks like many ores.",
    parts: &[
        ToolPart::PickHead.into(),
        ToolPart::ToolHandle.into(),
        ToolPart::RoundPlate.into(),
    ],
    base: ToolStats {
        attack_speed: Base(1.3),
        ..Default::default()
    },
    flat: ToolStats {
        attack_damage: FlatMod(0.5),
        ..Default::default()
    },
    multiplier: ToolStats {
        durability: Multiplier(1.3),
        attack_damage: Multiplier(1.15),
        mining_speed: Multiplier(0.75),
        ..Default::default()
    },
    traits: &["Pathing", "Bane of Sssss"],
};

const EXCAVATOR: Tool = Tool {
    name: "Excavator",
    description: "The Excavator is a broad digging tool. It digs up large areas of soil and snow in a wide range.\nTerraforming!",
    parts: &[
        ToolPart::LargePlate.into(),
        ToolPart::ToughHandle.into(),
        ToolPart::LargePlate.into(),
        ToolPart::ToughHandle.into(),
    ],
    base: ToolStats {
        attack_speed: Base(1.),
        ..Default::default()
    },
    flat: ToolStats {
        attack_damage: FlatMod(1.5),
        ..Default::default()
    },
    multiplier: ToolStats {
        durability: Multiplier(3.75),
        attack_damage: Multiplier(1.2),
        mining_speed: Multiplier(0.3),
        ..Default::default()
    },
    traits: &["Knockback II", "Makes path"],
};

const HAND_AXE: Tool = Tool {
    name: "Hand Axe",
    description: "The Hand Axe chops up wood and makes short work of leaves. It also makes for an effective weapon.\nChop chop!",
    parts: &[
        ToolPart::SmallAxeHead.into(),
        ToolPart::ToolHandle.into(),
        ToolPart::ToolBinding.into(),
    ],
    base: ToolStats {
        attack_speed: Base(0.9),
        ..Default::default()
    },
    flat: ToolStats {
        attack_damage: FlatMod(6.),
        ..Default::default()
    },
    multiplier: Default::default(),
    traits: &["Axe Scrape", "Stripping", "Axe Wax Off"],
};

const BROAD_AXE: Tool = Tool {
    name: "Broad Axe",
    description: "The Broad Axe fells small trees in a single swing, and makes quick work of large trees. It makes for a powerful heavy weapon.\nTimber!",
    parts: &[
        Weighted::weighted(ToolPart::BroadAxeHead, 2),
        ToolPart::ToughHandle.into(),
        ToolPart::PickHead.into(),
        ToolPart::ToolBinding.into(),
    ],
    base: ToolStats {
        attack_damage: Base(5.),
        attack_speed: Base(0.6),
        ..Default::default()
    },
    flat: ToolStats {
        attack_damage: FlatMod(5.),
        ..Default::default()
    },
    multiplier: ToolStats {
        durability: Multiplier(4.25),
        attack_damage: Multiplier(1.65),
        mining_speed: Multiplier(0.3),
        ..Default::default()
    },
    traits: &["Axe Scrape", "Stripping", "Axe Wax Off"],
};

const KAMA: Tool = Tool {
    name: "Kama",
    description: "The Kama is a precision reaping tool, mowing down plants and shearing animals.\nRight Click: Harvest and replant crops",
    parts: &[
        ToolPart::SmallBlade.into(),
        ToolPart::ToolHandle.into(),
        ToolPart::ToolBinding.into(),
    ],
    base: ToolStats {
        attack_speed: Base(1.6),
        ..Default::default()
    },
    flat: ToolStats {
        attack_damage: FlatMod(1.),
        ..Default::default()
    },
    multiplier: ToolStats {
        attack_damage: Multiplier(0.5),
        ..Default::default()
    },
    traits: &["Shearing", "Harvest"],
};

const SCYTHE: Tool = Tool {
    name: "Scythe",
    description: "The Scythe is a broad reaping tool, mowing down plants in a wide area.\nRight Click: Harvest and replant crops",
    parts: &[
        ToolPart::BroadBlade.into(),
        ToolPart::ToughHandle.into(),
        ToolPart::ToolBinding.into(),
        ToolPart::ToughHandle.into(),
    ],
    base: ToolStats {
        attack_speed: Base(0.7),
        ..Default::default()
    },
    flat: ToolStats {
        attack_damage: FlatMod(1.),
        ..Default::default()
    },
    multiplier: ToolStats {
        durability: Multiplier(2.5),
        mining_speed: Multiplier(0.45),
        ..Default::default()
    },
    traits: &["AOE Attack", "Tills dirt", "Harvests plants"],
};

const DAGGER: Tool = Tool {
    name: "Dagger",
    description: "The Dagger is a light weapon, capabile of quick strikes from either hand.",
    parts: &[ToolPart::SmallBlade.into(), ToolPart::ToolHandle.into()],
    base: ToolStats {
        attack_speed: Base(2.),
        ..Default::default()
    },
    flat: ToolStats {
        attack_damage: FlatMod(3.),
        ..Default::default()
    },
    multiplier: ToolStats {
        durability: Multiplier(0.75),
        attack_damage: Multiplier(0.65),
        mining_speed: Multiplier(0.75),
        ..Default::default()
    },
    traits: &["Padded I", "Offhand Attack", "Silky Shears"],
};

const SWORD: Tool = Tool {
    name: "Sword",
    description: "The Sword is a universal weapon. Sweep attacks keep enemy hordes at bay.\nAlso good against cobwebs!",
    parts: &[
        ToolPart::SmallBlade.into(),
        ToolPart::ToolHandle.into(),
        ToolPart::ToolHandle.into(),
    ],
    base: ToolStats {
        attack_speed: Base(1.6),
        ..Default::default()
    },
    flat: ToolStats {
        attack_damage: FlatMod(3.),
        ..Default::default()
    },
    multiplier: ToolStats {
        durability: Multiplier(1.1),
        mining_speed: Multiplier(0.5),
        ..Default::default()
    },
    traits: &["Silky Shears", "Sweep Attack"],
};

const CLEAVER: Tool = Tool {
    name: "Cleaver",
    description: "The Cleaver is a weapon for a smeltery master. High range attacks keep cut through the toughest of foes.",
    parts: &[
        ToolPart::BroadBlade.into(),
        ToolPart::ToughHandle.into(),
        ToolPart::ToughHandle.into(),
        ToolPart::LargePlate.into(),
    ],
    base: ToolStats {
        attack_speed: Base(1.),
        ..Default::default()
    },
    flat: ToolStats {
        attack_damage: FlatMod(3.),
        ..Default::default()
    },
    multiplier: ToolStats {
        durability: Multiplier(3.5),
        attack_damage: Multiplier(1.5),
        mining_speed: Multiplier(0.25),
        ..Default::default()
    },
    traits: &["Beheading II"],
};

const CROSSBOW: Tool = Tool {
    name: "Crossbow",
    description: "The Crossbow is a light ranged weapon, capable of quickly firing arrows and fireworks.",
    parts: &[
        ToolPart::BowLimb.into(),
        ToolPart::BowGrip.into(),
        ToolPart::Bowstring.into(),
    ],
    base: ToolStats {
        attack_speed: Base(1.),
        draw_speed: Base(1.),
        arrow_velocity: Base(1.),
        accuracy: Base(0.75),
        ..Default::default()
    },
    flat: ToolStats::default(),
    multiplier: ToolStats {
        durability: Multiplier(2.),
        ..Default::default()
    },
    traits: &[],
};

const LONGBOW: Tool = Tool {
    name: "Longbow",
    description: "The Longbow is a broad ranged weapon, capable of firing arrows with precision over long distances.",
    parts: &[
        ToolPart::BowLimb.into(),
        ToolPart::BowLimb.into(),
        ToolPart::BowGrip.into(),
        ToolPart::Bowstring.into(),
    ],
    base: ToolStats {
        attack_speed: Base(1.),
        draw_speed: Base(1.),
        arrow_velocity: Base(1.),
        accuracy: Base(0.75),
        ..Default::default()
    },
    flat: ToolStats {
        durability: FlatMod(120.),
        ..Default::default()
    },
    multiplier: ToolStats {
        durability: Multiplier(1.5),
        ..Default::default()
    },
    traits: &[],
};

pub const TOOLS: &[Tool] = &[
    PICKAXE,
    SLEDGE_HAMMER,
    VEIN_HAMMER,
    MATTOCK,
    PICKADZE,
    EXCAVATOR,
    HAND_AXE,
    BROAD_AXE,
    KAMA,
    SCYTHE,
    DAGGER,
    SWORD,
    CLEAVER,
    CROSSBOW,
    LONGBOW,
];
