use crate::view::item_view::ItemView;
use crate::view::text_view::{TextStyle, TextView};

const ANSI_RESET: &str = "\x1b[0m";

pub trait TextRenderer {
    fn render(&self, text: &TextView);
}

impl TextRenderer for Box<dyn TextRenderer> {
    fn render(&self, text: &TextView) {
        // Delegate the call to the inner renderer
        self.as_ref().render(text);
    }
}

pub struct ConsoleTextRenderer {}

impl TextRenderer for ConsoleTextRenderer {
    fn render(&self, text: &TextView) {
        print!("{}", text.value)
    }
}

pub struct ColorDecorator<T: TextRenderer> {
    pub decoratee: T,
}

impl<T: TextRenderer> TextRenderer for ColorDecorator<T> {
    fn render(&self, text: &TextView) {
        let color = match text.style {
            TextStyle::Normal => "",
            TextStyle::Magic => "\x1b[38;5;39m",
            TextStyle::Rare => "\x1b[38;5;220m",
            TextStyle::Unique => "\x1b[38;5;208m",
            TextStyle::UnfulfilledRequirement => "\x1b[31m",
            _ => { "\x1b[38m"}
        };

        print!("{}", color);
        self.decoratee.render(text);
        print!("{}", ANSI_RESET);
    }
}

pub struct BoldDecorator<T: TextRenderer> {
    pub decoratee: T,
}

impl<T: TextRenderer> TextRenderer for BoldDecorator<T> {
    fn render(&self, text: &TextView) {
        print!("\x1b[1m");
        self.decoratee.render(text);
        print!("{}", ANSI_RESET);
    }
}

pub struct ItalicDecorator<T: TextRenderer> {
    pub decoratee: T,
}

impl<T: TextRenderer> TextRenderer for ItalicDecorator<T> {
    fn render(&self, text: &TextView) {
        print!("\x1b[3m");
        self.decoratee.render(text);
        print!("{}", ANSI_RESET);
    }
}

pub struct UnderlineDecorator<T: TextRenderer> {
    pub decoratee: T,
}

impl<T: TextRenderer> TextRenderer for UnderlineDecorator<T> {
    fn render(&self, text: &TextView) {
        print!("\x1b[4m");
        self.decoratee.render(text);
        print!("{}", ANSI_RESET);
    }
}

pub struct HighlightDecorator<T: TextRenderer> {
    pub decoratee: T,
}

impl<T: TextRenderer> TextRenderer for HighlightDecorator<T> {
    fn render(&self, text: &TextView) {
        let color = match text.style {
            TextStyle::Magic => "\x1b[1m\x1b[48;5;60m",
            TextStyle::Rare => "\x1b[1m\x1b[48;5;59m",
            TextStyle::Unique => "\x1b[1m\x1b[48;5;95m",// orange
            _ => ""
        };

        print!("{}", color);
        self.decoratee.render(text);
        print!("{}", ANSI_RESET);
    }
}

pub struct NewLineDecorator<T: TextRenderer> {
    pub decoratee: T,
}

impl<T: TextRenderer> TextRenderer for NewLineDecorator<T> {
    fn render(&self, text: &TextView) {
        self.decoratee.render(text);
        println!();
    }
}

pub struct HighlightModificationDecorator<T: TextRenderer> {
    pub decoratee: T,
}

impl <T: TextRenderer> TextRenderer for HighlightModificationDecorator<T> {
    fn render(&self, text: &TextView) {
        let mut new_text = text.value.clone();
        if text.value.contains("%mod_start") && text.value.contains("%mod_end") {
            new_text = new_text.replace("%mod_start", "\x1b[38;5;39m");
            new_text = new_text.replace("%mod_end", ANSI_RESET);
        }

        self.decoratee.render(&TextView {
            value: new_text,
            style: text.style.clone()
        });
    }
}