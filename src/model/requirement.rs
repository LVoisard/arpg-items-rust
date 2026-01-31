use crate::model::stat::{Stat, StatType};

#[derive(Debug, Clone)]
pub struct StatRequirement {
    pub stat_type: StatType,
    pub amount: i32,
}

impl StatRequirement {
    pub fn is_met(&self, stat: &Stat) -> bool {
        self.stat_type == stat.stat_type && self.amount <= stat.value
    }
}

#[derive(Debug, Clone)]
pub struct RequirementBlock {
    pub requirements: Vec<StatRequirement>
}

impl Default for RequirementBlock {
    fn default() -> Self {
        RequirementBlock {
            requirements: Vec::new(),
        }
    }
}