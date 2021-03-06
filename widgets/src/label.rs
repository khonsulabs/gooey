use gooey_core::{
    styles::{ColorPair, FallbackComponent, StyleComponent, TextColor},
    Context, StyledWidget, Widget,
};
use gooey_text::Text;

#[cfg(feature = "gooey-rasterizer")]
mod rasterizer;

#[cfg(feature = "frontend-browser")]
mod browser;

#[derive(Debug)]
pub struct Label {
    label: Text,
}

impl Label {
    pub fn new<S: Into<Text>>(label: S) -> StyledWidget<Self> {
        StyledWidget::from(Self {
            label: label.into(),
        })
    }

    pub fn set_label(&mut self, label: impl Into<Text>, context: &Context<Self>) {
        self.label = label.into();
        context.send_command(Command::LabelChanged);
    }
}

#[derive(Debug)]
pub enum Command {
    LabelChanged,
}

impl Widget for Label {
    type Command = Command;
    type Event = ();

    const CLASS: &'static str = "gooey-label";
    const FOCUSABLE: bool = false;
}

#[derive(Debug)]
pub struct LabelTransmogrifier;

/// The label's text color.
#[derive(Debug, Clone)]
pub struct LabelColor(pub ColorPair);
impl StyleComponent for LabelColor {}

impl From<LabelColor> for ColorPair {
    fn from(color: LabelColor) -> Self {
        color.0
    }
}

impl FallbackComponent for LabelColor {
    type Fallback = TextColor;
    type Value = ColorPair;

    fn value(&self) -> Option<&ColorPair> {
        Some(&self.0)
    }
}
