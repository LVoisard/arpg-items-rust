mod model;
mod ui;
mod input;

use crate::model::inventory::Inventory;
use crate::model::item::{ArmourType, EquipmentType, WeaponType};
use crate::model::modifier::{
    BasicStatModifier, CompositeStatModifier, FlatStatModifier, FrontStatModifier, ModifierKind,
    ModifierPass, ModifierTargetKind, RequirementModifier,
};
use crate::model::player::Player;
use crate::model::stat::{Stat, StatBlock, StatType};
use crate::ui::ratatui::state::player::PlayerState;
use model::item::{Item, ItemClass, ItemRarity};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, SetSize, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;
use std::io::{IsTerminal, stdout};
use std::process::Command;
use ui::ratatui::ratatui_app::RatatuiApp;

fn main() -> Result<(), std::io::Error> {
    if !std::io::stdout().is_terminal() {
        spawn_terminal_and_exit();
        return Ok(())
    }

    let equippable_item = Item::builder()
        .base(String::from("Claymore"))
        .name(String::from("Big Long Sword"))
        .rarity(ItemRarity::Rare)
        .with_stat(StatType::MinimumDamage, 9)
        .with_stat(StatType::MaximumDamage, 17)
        .with_modifier(FlatStatModifier {
            value: 5,
            stat: StatType::Dexterity,
            target: ModifierTargetKind::Character,
        })
        .with_modifier(FrontStatModifier {
            front: StatType::IncreasedDamage,
            stats: vec![StatType::MinimumDamage, StatType::MaximumDamage],
            modifier_kind: ModifierKind::Percent,
            modifier_pass: ModifierPass::Increased,
            value: 80,

            target: ModifierTargetKind::Item,
        })
        .with_modifier(FrontStatModifier {
            front: StatType::IncreasedDamage,
            stats: vec![StatType::MinimumDamage, StatType::MaximumDamage],
            modifier_kind: ModifierKind::Percent,
            modifier_pass: ModifierPass::Increased,
            value: 10,
            target: ModifierTargetKind::Character,
        })
        .build()
        .unwrap();

    let mut player = Player {
        base_stats: StatBlock {
            stats: vec![
                Stat::new(StatType::Strength, 15),
                Stat::new(StatType::Dexterity, 13),
                Stat::new(StatType::Intelligence, 8),
                Stat::new(StatType::Level, 10),
            ],
        },
        equippement: Inventory::new(),
        inventory: Inventory::new(),
    };

    player.equip(equippable_item);

    let item = Item::builder()
        .base(String::from("Hand Axe"))
        .rarity(ItemRarity::Normal)
        .class(ItemClass::Equipment(EquipmentType::Weapon(WeaponType::Axe)))
        .with_requirement(StatType::Strength, 15)
        .with_requirement(StatType::Dexterity, 12)
        .with_stat(StatType::MinimumDamage, 2)
        .with_stat(StatType::MaximumDamage, 6)
        .build();

    player.pickup(item.unwrap());


    let item = Item::builder()
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

    player.pickup(item.unwrap());

    let item = Item::builder()
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

    player.pickup(item.unwrap());

    let item = Item::builder()
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
    player.pickup(item.unwrap());


    let item = Item::builder()
        .name(String::from("Headhunter"))
        .base(String::from("Leather Belt"))
        .rarity(ItemRarity::Unique)
        .with_requirement(StatType::Level, 68)
        .with_modifier(RequirementModifier { value: -30 })
        .with_stat(StatType::Life, 40)
        .build();
    player.pickup(item.unwrap());

    // let ui = ConsoleUI::default();
    //
    // for item in items.iter() {
    //     ui.display_item_view(&ItemView::from(item));
    //     println!();
    // }
    //
    // ui.display_player_view(&PlayerView::from(player));

    let mut app = RatatuiApp::new(PlayerState::from(player));

    let mut terminal = init_terminal()?;    
    app.run(&mut terminal)?;
    restore_terminal()
}

fn spawn_terminal_and_exit() {
    let exe = std::env::current_exe().unwrap();

    let candidates = [
        ("x-terminal-emulator", &["-e"]),
        ("gnome-terminal", &["--"]),
        ("konsole", &["-e"]),
        ("xterm", &["-e"]),
    ];

    for (term, args) in candidates {
        if Command::new(term).args(args).arg(&exe).spawn().is_ok() {
            return;
        }
    }

    eprintln!("No terminal emulator found.");
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;

    let mut stdout = stdout();

    execute!(
        stdout,
        EnterAlternateScreen,
        SetSize(124, 30), // request size
    )?;

    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen,)?;
    Ok(())
}
