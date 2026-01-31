use crate::model::item::{EquipmentType, Item, ItemClass, ItemRarity};
use crate::model::requirement::StatRequirement;
use crate::model::stat::{StatBlock, StatType};

pub struct ItemViewModel {
    pub item_base: String,
    pub name: Option<String>,
    pub rarity: ItemRarity,

    pub damage: Option<DamageLine>,
    pub requirements: Vec<RequirementLine>,
    pub item_class: String,
    pub modifiers: Vec<String>,
}

impl ItemViewModel {
    pub fn from(item: &Item, player_stats: &StatBlock) -> ItemViewModel {
        let derived = item.get_derived_stats();
        let reqs = item.get_derived_requirements();

        let damage = match item.item_class {
            ItemClass::Equipment(EquipmentType::Weapon(_))
            if derived.has(StatType::MinimumDamage) && item.base_stats.has(StatType::MinimumDamage)
                && derived.has(StatType::MaximumDamage) && item.base_stats.has(StatType::MinimumDamage)=>
                {
                    Some(DamageLine {
                        min: derived.get(StatType::MinimumDamage).unwrap().value,
                        max: derived.get(StatType::MaximumDamage).unwrap().value,
                        is_modified: item.modifiers.iter().any(|m|
                            matches!(
                            m.get_affected_stat(),
                            StatType::IncreasedDamage
                                | StatType::MinimumDamage
                                | StatType::MaximumDamage
                        )
                        ),
                    })
                }
            _ => None,
        };

        let requirements = reqs.requirements.into_iter().map(|r| {
            let met = player_stats
                .get(r.stat_type)
                .map(|s| r.is_met(s))
                .unwrap_or(false);

            RequirementLine {
                requirement: r,
                is_met: met,
                is_modified: item.modifiers.iter().any(|m| m.get_affected_stat() == StatType::Requirements)
            }
        }).collect();

        ItemViewModel {
            name: item.name.clone(),
            item_base: item.item_base.clone(),
            rarity: item.rarity,
            item_class: item.item_class.to_string(),
            damage,
            requirements,
            modifiers: item.modifiers.iter().map(|m| m.description()).collect(),
        }
    }
}



pub struct DamageLine {
    pub min: i32,
    pub max: i32,
    pub is_modified: bool,
}

pub struct RequirementLine {
    pub requirement: StatRequirement,
    pub is_met: bool,
    pub is_modified: bool,
}