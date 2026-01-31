use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Stat {
    pub stat_type: StatType,
    pub value: i32,
}

impl Stat {
    pub fn new(stat_type: StatType, value: i32) -> Stat {
        Stat { stat_type, value }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StatType {
    Strength,
    Intelligence,
    Dexterity,
    Level,
    Requirements,
    IncreasedDamage,
    IncreasedAttackSpeed,
    MinimumDamage,
    MaximumDamage,
    Defense,
    Life,
}

impl Display for StatType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StatType::Strength => write!(f, "Strength"),
            StatType::Intelligence => write!(f, "Intelligence"),
            StatType::Dexterity => write!(f, "Dexterity"),
            StatType::Level => write!(f, "Level"),
            StatType::Requirements => write!(f, "Requirements"),
            StatType::IncreasedDamage => write!(f, "Increased Damage"),
            StatType::MinimumDamage => write!(f, "Minimum Damage"),
            StatType::MaximumDamage => write!(f, "Maximum Damage"),
            StatType::Defense => write!(f, "Defence"),
            StatType::IncreasedAttackSpeed => write!(f, "Increased Attack Speed"),
            StatType::Life => write!(f, "Life"),
        }
    }
}

impl PartialEq for StatType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StatType::Strength, StatType::Strength) => true,
            (StatType::Dexterity, StatType::Dexterity) => true,
            (StatType::Intelligence, StatType::Intelligence) => true,
            (StatType::Level, StatType::Level) => true,
            (StatType::MinimumDamage, StatType::MinimumDamage) => true,
            (StatType::MaximumDamage, StatType::MaximumDamage) => true,
            (StatType::IncreasedDamage, StatType::IncreasedDamage) => true,
            (StatType::Requirements, StatType::Requirements) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatBlock {
    pub stats: Vec<Stat>,
}

impl StatBlock {
    pub fn get(&self, stat_type: StatType) -> Option<&Stat> {
        self.stats.iter().find(|&x| x.stat_type == stat_type)
    }

    pub fn get_mut(&mut self, stat_type: StatType) -> Option<&mut Stat> {
        self.stats.iter_mut().find(|x| x.stat_type == stat_type)
    }
    pub fn has(&self, stat_type: StatType) -> bool {
        self.stats.iter().any(|x| x.stat_type == stat_type)
    }

    pub fn add(&mut self, stat: Stat) {
        if self.stats.iter().any(|x| x.stat_type == stat.stat_type) {
            return;
        }

        self.stats.push(stat);
    }
}

impl Default for StatBlock {
    fn default() -> Self {
        Self { stats: vec![] }
    }
}
