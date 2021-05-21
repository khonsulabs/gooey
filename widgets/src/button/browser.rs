use gooey_browser::{WebSys, WebSysTransmogrifier};
use gooey_core::Channels;
use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;

use crate::{
    button::{Button, ButtonTransmogrifier},
    window_document,
};

impl gooey_core::Transmogrifier<WebSys> for ButtonTransmogrifier {
    type State = u32;
    type Widget = Button;
}

impl WebSysTransmogrifier for ButtonTransmogrifier {
    fn transmogrify(
        &self,
        _state: &Self::State,
        _channels: &Channels<Button>,
        parent: &web_sys::Node,
        widget: &Button,
        _frontend: &WebSys,
    ) -> Option<web_sys::HtmlElement> {
        let document = window_document();
        let element = document
            .create_element("button")
            .expect("couldn't create button")
            .unchecked_into::<HtmlButtonElement>();
        element.set_inner_text(&widget.label);
        parent.append_child(&element).unwrap();
        Some(element.unchecked_into())
    }
}
