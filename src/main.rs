mod arpg_core;
mod ui;
mod view;

use crate::arpg_core::item::{ArmourType, EquipmentType, ItemPresentation, JewelleryType, WeaponType};
use crate::arpg_core::item_builder::{ItemBuilder, ItemCreationError};
use crate::arpg_core::modifier::{
    BasicStatModifier, CompositeStatModifier, FlatStatModifier, FrontStatModifier, ModifierKind,
    ModifierPass, ModifierTargetKind, RequirementModifier,
};
use crate::arpg_core::player::Player;
use crate::arpg_core::stat::{Stat, StatBlock, StatType};
use crate::ui::ratatui::ratatui_app::Inventory;
use crate::view::item_view::ItemView;
use crate::view::stat_view::{PlayerView, StatsView};
use arpg_core::item::{Item, ItemClass, ItemRarity};
use ui::console::console_ui::ConsoleUI;
use ui::ui::UI;
use ui::ratatui::ratatui_app::RatatuiApp;

fn add_item_to_list(
    item: Result<Item, ItemCreationError>,
    stats: &StatBlock,
    items: &mut Vec<ItemPresentation>,
) {
    match item {
        Ok(item) => items.push(item.present(stats)),
        Err(error) => println!("{}", error),
    }
}

fn main() {
    let equippable_item =ItemBuilder::new()
        .base(String::from("Claymore"))
        .name(String::from("Big Long Sword"))
        .rarity(ItemRarity::Rare)
        .with_stat(StatType::MinimumDamage, 9)
        .with_stat( StatType::MaximumDamage, 17)
        .with_modifier(FlatStatModifier{value: 5, stat: StatType::Dexterity, target: ModifierTargetKind::Character})
        .with_modifier(FrontStatModifier {
            front: StatType::IncreasedDamage,
            stats: vec![StatType::MinimumDamage,StatType::MaximumDamage],
            modifier_kind: ModifierKind::Percent,
            modifier_pass: ModifierPass::Increased,
            value: 80,

            target: ModifierTargetKind::Item,
        })
        .with_modifier(FrontStatModifier {
            front: StatType::IncreasedDamage,
            stats: vec![StatType::MinimumDamage,StatType::MaximumDamage],
            modifier_kind: ModifierKind::Percent,
            modifier_pass: ModifierPass::Increased,
            value: 10,
            target: ModifierTargetKind::Character,
        })
        .build().unwrap();

    let mut player = Player {
        base_stats: StatBlock {
            stats: vec![
                Stat::new(StatType::Strength, 15),
                Stat::new(StatType::Dexterity, 13),
                Stat::new(StatType::Intelligence, 8),
                Stat::new(StatType::Level, 10),
            ],
        },
        equipped_items: vec![],
    };

    player.equip(&equippable_item);

    let mut items = Vec::<ItemPresentation>::new();

    let item = ItemBuilder::new()
        .base(String::from("Hand Axe"))
        .rarity(ItemRarity::Normal)
        .class(ItemClass::Equipment(EquipmentType::Weapon(WeaponType::Axe)))
        .with_requirement(StatType::Strength, 15)
        .with_requirement(StatType::Dexterity, 12)
        .with_stat(StatType::MinimumDamage, 2)
        .with_stat(StatType::MaximumDamage, 6)
        .build();

    add_item_to_list(item, &player.base_stats, &mut items);

    let item = ItemBuilder::new()
        .name(String::from("Excalibur"))
        .base(String::from("Short Sword"))
        .rarity(ItemRarity::Magic)
        .class(ItemClass::Equipment(EquipmentType::Weapon(
            WeaponType::Sword,
        )))
        .with_requirement(StatType::Strength, 1000)
        .with_stat(StatType::MinimumDamage, 3)
        .with_stat(StatType::MaximumDamage, 5)
        .with_modifier(FlatStatModifier {
            value: 5,
            stat: StatType::Dexterity,
            target: ModifierTargetKind::Item,
        })
        .with_modifier(FlatStatModifier {
            value: 5,
            stat: StatType::MinimumDamage,
            target: ModifierTargetKind::Item,
        })
        .with_modifier(FlatStatModifier {
            value: 10,
            stat: StatType::MaximumDamage,
            target: ModifierTargetKind::Item,
        })
        .with_modifier(FrontStatModifier {
            value: 50,
            front: StatType::IncreasedDamage,
            stats: vec![StatType::MinimumDamage, StatType::MaximumDamage],
            modifier_kind: ModifierKind::Percent,
            modifier_pass: ModifierPass::Increased,
            target: ModifierTargetKind::Item,
        })
        .with_modifier(RequirementModifier { value: -98 })
        .build();

    add_item_to_list(item, &player.base_stats, &mut items);

    let item = ItemBuilder::new()
        .base(String::from("Kris"))
        .name(String::from("Death's Kiss"))
        .rarity(ItemRarity::Rare)
        .class(ItemClass::Equipment(EquipmentType::Weapon(
            WeaponType::Dagger,
        )))
        .with_requirement(StatType::Dexterity, 15)
        .with_requirement(StatType::Intelligence, 10)
        .with_stat(StatType::MinimumDamage, 1)
        .with_stat(StatType::MaximumDamage, 4)
        .with_modifier(BasicStatModifier {
            value: 35,
            stat: StatType::IncreasedAttackSpeed,
            modifier_kind: ModifierKind::Percent,
            modifier_pass: ModifierPass::Increased,
            target: ModifierTargetKind::Item,
        })
        .build();

    add_item_to_list(item, &player.base_stats, &mut items);

    let item = ItemBuilder::new()
        .base(String::from("Shako"))
        .name(String::from("Harlequin's Crest"))
        .rarity(ItemRarity::Unique)
        .class(ItemClass::Equipment(EquipmentType::Armour(
            ArmourType::Helmet,
        )))
        .with_requirement(StatType::Level, 43)
        .with_modifier(CompositeStatModifier {
            stats: vec![
                StatType::Strength,
                StatType::Dexterity,
                StatType::Intelligence,
            ],
            modifier_kind: ModifierKind::Flat,
            modifier_pass: ModifierPass::Flat,
            values: vec![20, 20, 20],
            target: ModifierTargetKind::Character,
        })
        .build();

    add_item_to_list(item, &player.base_stats, &mut items);

    let item = ItemBuilder::new()
        .name(String::from("Headhunter"))
        .base(String::from("Leather Belt"))
        .rarity(ItemRarity::Unique)
        .with_requirement(StatType::Level, 68)
        .with_modifier(RequirementModifier { value: -30 })
        .with_stat(StatType::Life, 40)
        .build();

    add_item_to_list(item, &player.base_stats, &mut items);

    let ui = ConsoleUI::default();
    
    for item in items.iter() {
        ui.display_item_view(&ItemView::from(item));
        println!();
    }
    
    ui.display_player_view(&PlayerView::from(player));

    let mut app = RatatuiApp{
        exit: false,
        inventory: Inventory {items},
    };

    let mut terminal = ratatui::init();
    let _res = app.run(&mut terminal);

    ratatui::restore();
}
