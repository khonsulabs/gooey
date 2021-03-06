use gooey_browser::{
    utils::{create_element, CssRules},
    WebSys, WebSysTransmogrifier,
};
use gooey_core::{Transmogrifier, TransmogrifierContext, Widget, WidgetRef};
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;

use crate::component::{Behavior, Component, ComponentTransmogrifier};

impl<B: Behavior> WebSysTransmogrifier for ComponentTransmogrifier<B> {
    fn transmogrify(
        &self,
        context: TransmogrifierContext<'_, Self, WebSys>,
    ) -> Option<web_sys::HtmlElement> {
        let container = create_element::<HtmlDivElement>("div");
        *context.state = self.initialize_widget_element(&container, &context);
        if let Some(child) = context
            .frontend
            .with_transmogrifier(
                context.widget.content.id(),
                |child_transmogrifier, mut child_context| {
                    child_transmogrifier.transmogrify(&mut child_context)
                },
            )
            .unwrap_or_default()
        {
            container.append_child(&child).unwrap();
        }
        Some(container.unchecked_into())
    }
}

impl<B: Behavior> From<ComponentTransmogrifier<B>> for gooey_browser::RegisteredTransmogrifier {
    fn from(transmogrifier: ComponentTransmogrifier<B>) -> Self {
        Self(std::boxed::Box::new(transmogrifier))
    }
}

impl<B: Behavior> Transmogrifier<WebSys> for ComponentTransmogrifier<B> {
    type State = Option<CssRules>;
    type Widget = Component<B>;

    fn initialize(
        &self,
        component: &mut Self::Widget,
        widget: &WidgetRef<Self::Widget>,
        frontend: &WebSys,
    ) -> Self::State {
        Self::initialize_component(component, widget, frontend);
        None
    }

    fn receive_command(
        &self,
        command: <Self::Widget as Widget>::Command,
        context: &mut TransmogrifierContext<'_, Self, WebSys>,
    ) {
        Self::forward_command_to_content(command, context);
    }
}
