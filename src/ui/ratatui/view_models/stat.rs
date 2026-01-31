use crate::model::stat::Stat;

pub struct StatViewModel {
    pub value: String,
}

impl From<&Stat> for StatViewModel {
    fn from(value: &Stat) -> Self {
        Self {
            value: format!("{}: {}", value.stat_type.to_string(), value.value),
        }
    }
}