//! Core traits and types used to create Graphical User Interfaces (GUIs -
//! `gooey`s).

#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms
)]
#![allow(clippy::if_not_else, clippy::module_name_repetitions)]
#![cfg_attr(doc, warn(rustdoc::all))]

pub mod assets;
mod frontend;
mod gooey;
/// Types used for styling.
pub mod styles;
mod widget;

pub use euclid;
pub use palette;

pub use self::{frontend::*, gooey::*, widget::*};

/// A unit representing physical pixels on a display.
#[derive(Debug, Clone, Copy, Default)]
pub struct Pixels;

/// A unit aiming to represent the scaled resolution of the display the
/// interface is being displayed on. The ratio between [`Pixels`] and `Points`
/// can vary based on many things, including the display configuration, the
/// system user interface settings, and the browser's zoom level. Each
/// [`Frontend`] will use its best available methods for translating `Points` to
/// [`Pixels`] in a way that is consistent with other applications.
#[derive(Debug, Clone, Copy, Default)]
pub struct Points;

/// The name of the class assigned to the root widget of a window.
pub const ROOT_CLASS: &str = "gooey-root";

/// The name of the class assigned to widgets that have a solid background
pub const SOLID_WIDGET_CLASS: &str = "gooey-solid";
