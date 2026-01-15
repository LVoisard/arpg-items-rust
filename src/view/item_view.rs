use std::cmp::PartialEq;
use crate::arpg_core::item::{EquipmentType, Item, ItemClass, ItemPresentation, ItemRarity};
use crate::arpg_core::requirement;
use crate::arpg_core::stat::{Stat, StatBlock, StatType};
use crate::view::text_view::{TextStyle, TextView};

#[derive(Debug)]
pub struct ItemView {
    pub item_name: Option<TextView>, // white items are not normally named, blue, yellow and orange are
    pub item_base: TextView,
    pub damage: Option<TextView>,
    pub requirements: Vec<TextView>,
    pub item_class: TextView,
    pub description: Vec<TextView>,
}

impl From<&ItemPresentation> for ItemView {
    fn from(item_presentation: &ItemPresentation) -> Self {
        let rarity_style = TextStyle::from_rarity(item_presentation.rarity);

        ItemView {
            item_name: item_presentation.name.as_ref().map(|n| TextView { value: n.clone(), style: rarity_style.clone() }),
            item_base: TextView { value: item_presentation.item_base.clone(), style: rarity_style.clone() },

            damage: item_presentation.damage.as_ref().map(|d| TextView {
                value: format!(
                    "Damage: {}{} to {}{}",
                    if d.is_modified { "%mod_start" } else { "" },
                    d.min,
                    d.max,
                    if d.is_modified { "%mod_end" } else { "" },
                ),
                style: TextStyle::ColorWhenModified,
            }),

            requirements: item_presentation.requirements.iter().map(|r| TextView {
                value: format!("Required {}: {}{}{}",
                               r.requirement.stat_type,
                               if r.is_modified {"%mod_start"} else {""},
                               r.requirement.amount,
                               if r.is_modified { "%mod_end" } else { "" }),
                style: if r.is_met {
                    TextStyle::Normal
                } else {
                    TextStyle::UnfulfilledRequirement
                },
            }).collect(),

            item_class: TextView {
                value: item_presentation.item_class.clone(),
                style: rarity_style,
            },

            description: item_presentation.modifiers.iter().map(|m| TextView {
                value: m.clone(),
                style: TextStyle::Magic,
            }).collect(),
        }
    }
}
