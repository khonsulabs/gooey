use crate::{
    core::{Frontend, Gooey, StyledWidget, Transmogrifiers, Widget, WidgetStorage},
    frontends::browser::WebSys,
    style::default_stylesheet,
    widgets::browser::{default_transmogrifiers, register_transmogrifiers},
};

/// Runs a browser-based [`App`](crate::app::App) with `transmogrifiers` and the
/// root widget from `initializer`. Unless overriden by `transmogrifier`, all
/// widgets from [`gooey::widget`](crate::widgets) will use the built-in
/// transmogrifiers.
pub fn browser_main_with<W: Widget + Send + Sync, C: FnOnce(&WidgetStorage) -> StyledWidget<W>>(
    mut transmogrifiers: Transmogrifiers<WebSys>,
    initializer: C,
) {
    register_transmogrifiers(&mut transmogrifiers);
    let mut ui = WebSys::new(Gooey::with(
        transmogrifiers,
        default_stylesheet(),
        initializer,
    ));
    ui.gooey().process_widget_messages(&ui);
    ui.install_in_id("gooey")
}

/// Runs a browser-based [`App`](crate::app::App) with the root widget from
/// `initializer`. All widgets from [`gooey::widget`](crate::widgets) will be
/// usable. If you wish to use other widgets, use `browser_main_with` and
/// provide the transmogrifiers for the widgets you wish to use.
pub fn browser_main<W: Widget + Send + Sync, C: FnOnce(&WidgetStorage) -> StyledWidget<W>>(
    initializer: C,
) {
    browser_main_with(default_transmogrifiers(), initializer)
}