mod arpg_core;
mod ui;
mod view;

use crate::arpg_core::item::{ArmourType, EquipmentType, JewelleryType, WeaponType};
use crate::arpg_core::modifier::{BasicStatModifier, FrontStatModifier, ModifierKind, ModifierPass, ModifierTargetKind, RequirementModifier};
use crate::arpg_core::requirement::{RequirementBlock, StatRequirement};
use crate::arpg_core::stat::{Stat, StatBlock, StatType};
use crate::view::item_view::ItemView;
use arpg_core::item::{Item, ItemClass, ItemRarity};
use std::any::Any;
use ui::console::console_ui::ConsoleUI;
use ui::ui::UI;

fn main() {
    let player_stats = StatBlock {
        stats: vec![
            Stat {
                stat_type: StatType::Strength,
                value: 15,
            },
            Stat {
                stat_type: StatType::Dexterity,
                value: 13,
            },
            Stat {
                stat_type: StatType::Intelligence,
                value: 8,
            },
            Stat {
                stat_type: StatType::Level,
                value: 10,
            },
        ],
    };

    let mut items = Vec::<ItemView>::new();

    let item1 = Item {
        name: None,
        item_base: String::from("Hand Axe"),
        rarity: ItemRarity::Normal,
        item_class: ItemClass::Equipment(EquipmentType::Weapon(WeaponType::Axe)),
        requirements: RequirementBlock {
            requirements: vec![
                StatRequirement {
                    stat_type: StatType::Strength,
                    amount: 15,
                },
                StatRequirement {
                    stat_type: StatType::Dexterity,
                    amount: 10,
                },
            ],
        },
        base_stats: StatBlock {
            stats: vec![
                Stat {
                    stat_type: StatType::MinimumDamage,
                    value: 2,
                },
                Stat {
                    stat_type: StatType::MaximumDamage,
                    value: 6,
                },
            ],
        },
        modifiers: vec![],
    };

    items.push(view::item_view::from_item(&item1.present(&player_stats)));

    items.push(view::item_view::from_item(
        &Item {
            name: None,
            item_base: String::from("Short Sword"),
            rarity: ItemRarity::Magic,
            item_class: ItemClass::Equipment(EquipmentType::Weapon(WeaponType::Sword)),
            requirements: RequirementBlock {
                requirements: vec![StatRequirement {
                    stat_type: StatType::Strength,
                    amount: 1000,
                }],
            },
            base_stats: StatBlock {
                stats: vec![
                    Stat {
                        stat_type: StatType::MinimumDamage,
                        value: 3,
                    },
                    Stat {
                        stat_type: StatType::MaximumDamage,
                        value: 5,
                    },
                ],
            },
            modifiers: vec![
                Box::new(BasicStatModifier {
                    value: 5,
                    stat: StatType::Dexterity,
                    modifier_kind: ModifierKind::Flat,
                    modifier_pass: ModifierPass::Flat,
                    target: ModifierTargetKind::Item
                }),
                Box::new(BasicStatModifier {
                    value: 2,
                    stat: StatType::MinimumDamage,
                    modifier_kind: ModifierKind::Flat,
                    modifier_pass: ModifierPass::Flat,
                    target: ModifierTargetKind::Item
                }),
                Box::new(BasicStatModifier {
                    value: 10,
                    stat: StatType::MaximumDamage,
                    modifier_kind: ModifierKind::Flat,
                    modifier_pass: ModifierPass::Flat,
                    target: ModifierTargetKind::Item
                }),
                Box::new(FrontStatModifier {
                    value: 50,
                    front: StatType::IncreasedDamage,
                    stats: vec![StatType::MinimumDamage, StatType::MaximumDamage],
                    modifier_kind: ModifierKind::Percent,
                    modifier_pass: ModifierPass::Increased,
                    target: ModifierTargetKind::Item
                }),
                Box::new(RequirementModifier {
                    value: -30,
                }),
            ],
        }.present(&player_stats),
    ));



    items.push(view::item_view::from_item(
        &Item {
            item_base: String::from("Kris"),
            name: Some(String::from("Death's Kiss")),
            rarity: ItemRarity::Rare,
            item_class: ItemClass::Equipment(EquipmentType::Weapon(WeaponType::Dagger)),
            requirements: RequirementBlock {
                requirements: vec![
                    StatRequirement {
                        stat_type: StatType::Dexterity,
                        amount: 15,
                    },
                    StatRequirement {
                        stat_type: StatType::Intelligence,
                        amount: 10,
                    },
                ],
            },
            base_stats: StatBlock {
                stats: vec![
                    Stat {
                        stat_type: StatType::MinimumDamage,
                        value: 1,
                    },
                    Stat {
                        stat_type: StatType::MaximumDamage,
                        value: 4,
                    },
                ],
            },
            modifiers: vec![],
        }.present(&player_stats),
    ));

    items.push(view::item_view::from_item(
        &Item {
            item_base: String::from("Shako"),
            name: Some(String::from("Harlequin's Crest")),
            rarity: ItemRarity::Unique,
            item_class: ItemClass::Equipment(EquipmentType::Armour(ArmourType::Helmet)),
            requirements: RequirementBlock {
                requirements: vec![StatRequirement {
                    stat_type: StatType::Level,
                    amount: 43,
                }],
            },
            base_stats: StatBlock {
                stats: vec![Stat {
                    stat_type: StatType::Defense,
                    value: 40,
                }],
            },
            modifiers: vec![],
        }.present(&player_stats),
    ));

    items.push(view::item_view::from_item(
        &Item {
            item_base: String::from("Leather Belt"),
            name: Some(String::from("Headhunter")),
            rarity: ItemRarity::Unique,
            item_class: ItemClass::Equipment(EquipmentType::Jewellery(JewelleryType::Belt)),
            requirements: RequirementBlock {
                requirements: vec![StatRequirement {
                    stat_type: StatType::Level,
                    amount: 65,
                }],
            },
            base_stats: StatBlock::default(),
            modifiers: vec![
                Box::new(RequirementModifier {
                    value: -30,
                }),
            ],
        }.present(&player_stats),
    ));

    let ui = ConsoleUI::default();

    for item in items {
        ui.display_item_view(&item);
        println!();
    }
}
