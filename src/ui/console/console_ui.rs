use crate::ui::console::decorator_builder::TextRendererBuilder;
use crate::ui::console::text_decorator::{
    BoldDecorator, ColorDecorator, ConsoleTextRenderer, HighlightDecorator, ItalicDecorator,
    NewLineDecorator, TextRenderer, UnderlineDecorator,
};
use crate::ui::ui::UI;
use crate::view::item_view::ItemView;

pub struct ConsoleUI {
    item_title_text_renderer: Box<dyn TextRenderer>,
    item_class_type_text_renderer: Box<dyn TextRenderer>,
    item_requirement_text_renderer: Box<dyn TextRenderer>,
    item_normal_text_renderer: Box<dyn TextRenderer>,
    item_modifiers_text_render: Box<dyn TextRenderer>,
    item_modified_values_text_render: Box<dyn TextRenderer>
}

impl Default for ConsoleUI {
    fn default() -> Self {
        Self {
            item_title_text_renderer: TextRendererBuilder::new()
                .with_color()
                .with_bold()
                .with_italic()
                .with_highlight()
                .build(),
            item_class_type_text_renderer: TextRendererBuilder::new()
                .with_italic()
                .build(),
            item_requirement_text_renderer: TextRendererBuilder::new().with_color().build(),
            item_normal_text_renderer: TextRendererBuilder::new().build(),
            item_modifiers_text_render: TextRendererBuilder::new()
                .with_color()
                .build(),
            item_modified_values_text_render: TextRendererBuilder::new()
                .with_highlight_modifications()
                .build()
        }
    }
}

impl UI for ConsoleUI {
    fn display_item_view(&self, item_view: &ItemView) {
        if item_view.item_name.is_some() {
            self.item_title_text_renderer
                .render(&item_view.item_name.as_ref().unwrap());
            self.item_title_text_renderer
                .render(&item_view.item_base);
        } else {
            self.item_title_text_renderer
                .render(&item_view.item_base);
        }

        if item_view.damage.is_some() {
            self.item_modified_values_text_render
                .render(&item_view.damage.as_ref().unwrap());
        }

        for requirement in item_view.requirements.iter() {
            self.item_requirement_text_renderer.render(&requirement);
        }

        self.item_class_type_text_renderer.render(&item_view.item_class);

        for line in item_view.description.iter() {
            self.item_modifiers_text_render.render(line);
        }
    }
}
