use gooey_browser::{
    utils::{create_element, CssBlockBuilder, CssManager, CssRules},
    WebSys, WebSysTransmogrifier,
};
use gooey_core::{Transmogrifier, TransmogrifierContext};
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;

use crate::container::{Container, ContainerTransmogrifier};

impl Transmogrifier<WebSys> for ContainerTransmogrifier {
    type State = Vec<CssRules>;
    type Widget = Container;
}

impl WebSysTransmogrifier for ContainerTransmogrifier {
    fn transmogrify(
        &self,
        context: TransmogrifierContext<'_, Self, WebSys>,
    ) -> Option<web_sys::HtmlElement> {
        let container = create_element::<HtmlDivElement>("div");
        let mut css_rules = Vec::new();
        if let Some(rule) = self.initialize_widget_element(&container, &context) {
            css_rules.push(rule);
        }

        let container_css = CssBlockBuilder::for_id(context.registration.id().id)
            .with_css_statement("display: flex")
            .with_css_statement("flex: 1")
            .with_css_statement("align-items: center")
            .with_css_statement("justify-content: center");
        css_rules.push(CssManager::shared().register_rule(&container_css.to_string()));
        *context.state = css_rules;

        context.frontend.with_transmogrifier(
            context.widget.child.id(),
            |child_transmogrifier, mut child_context| {
                if let Some(child) = child_transmogrifier.transmogrify(&mut child_context) {
                    container
                        .append_child(&child)
                        .expect("error appending child");
                }
                container.unchecked_into()
            },
        )
    }
}
