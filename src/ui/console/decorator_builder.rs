use std::mem;
use crate::ui::console::text_decorator::{BoldDecorator, ColorDecorator, HighlightDecorator, ConsoleTextRenderer, ItalicDecorator, TextRenderer, UnderlineDecorator, NewLineDecorator, HighlightModificationDecorator};

pub struct TextRendererBuilder {
    renderer: Box<dyn TextRenderer>
}

impl TextRendererBuilder {
    pub fn new() -> Self {
        TextRendererBuilder {
            renderer: Box::new(ConsoleTextRenderer {})
        }
    }

    fn wrap<F>(&mut self, wrapper: F) -> &mut Self
    where
        F: FnOnce(Box<dyn TextRenderer>) -> Box<dyn TextRenderer>
    {
        // Temporarily replace with a dummy or move out using mem::replace
        // We use a Box::new(ConsoleTextRenderer) as a temporary "seed"
        let old_renderer = mem::replace(&mut self.renderer, Box::new(ConsoleTextRenderer {}));
        self.renderer = wrapper(old_renderer);
        self
    }

    pub fn with_color(self) -> Self {
        let mut s = self;
    s.wrap(|inner| Box::new(ColorDecorator{decoratee: inner}));
        s
    }

    pub fn with_bold(self) -> Self {
        let mut s = self;
        s.wrap(|inner| Box::new(BoldDecorator{decoratee: inner}));
        s
    }

    pub fn with_italic(self) -> Self {
        let mut s = self;
        s.wrap(|inner| Box::new(ItalicDecorator{decoratee: inner}));
        s
    }

    pub fn with_underline(self) -> Self {
        let mut s = self;
        s.wrap(|inner| Box::new(UnderlineDecorator {decoratee: inner}));
        s
    }

    pub fn with_highlight(self) -> Self {
        let mut s = self;
        s.wrap(|inner| Box::new(HighlightDecorator {decoratee: inner}));
        s
    }

    pub fn with_highlight_modifications(self) -> Self {
        let mut s = self;
        s.wrap(|inner| Box::new(HighlightModificationDecorator{decoratee: inner}));
        s
    }

    pub fn build(mut self) -> Box<dyn TextRenderer> {
        self.wrap(|inner| Box::new(NewLineDecorator {decoratee: inner}));
        self.renderer
    }
}