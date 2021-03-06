use figures::Figure;
use stylecs::StyleComponent;

use crate::Scaled;

/// The width of lines stroked/drawn. Default is `1.` [`Scaled`].
#[derive(Debug, Copy)]
pub struct LineWidth<Unit>(pub Figure<f32, Unit>);

impl<Unit> Clone for LineWidth<Unit> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Default for LineWidth<Scaled> {
    fn default() -> Self {
        Self::new(1.)
    }
}

impl<Unit> LineWidth<Unit> {
    /// Creates a new `LineWidth` using `value` in `Unit`.
    #[must_use]
    pub const fn new(value: f32) -> Self {
        Self(Figure::new(value))
    }

    /// Returns the line width's raw value.
    #[must_use]
    pub fn get(&self) -> f32 {
        self.0.get()
    }

    /// Returns the line width as a type-safe measurement.
    #[must_use]
    pub const fn length(&self) -> Figure<f32, Unit> {
        self.0
    }
}

impl StyleComponent for LineWidth<Scaled> {}
