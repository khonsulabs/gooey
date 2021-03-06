use std::ops::{Deref, DerefMut};

use figures::Figure;
use stylecs::StyleComponent;

use super::Surround;
use crate::Scaled;

/// Adds padding (spacing) around a widget.
#[derive(Debug, Clone, Default)]
#[must_use]
pub struct Padding(pub Surround<Figure<f32, Scaled>>);

impl StyleComponent for Padding {
    fn should_be_inherited(&self) -> bool {
        false
    }
}

impl Padding {
    /// Returns a padding builder.
    pub fn build() -> Builder {
        Builder::default()
    }

    /// Returns an instance with uniform padding of `points` on all sides.
    pub fn uniform(points: f32) -> Self {
        Self::from(Figure::new(points))
    }
}

impl From<Figure<f32, Scaled>> for Padding {
    fn from(length: Figure<f32, Scaled>) -> Self {
        Self(Surround::from(Some(length)))
    }
}

impl Deref for Padding {
    type Target = Surround<Figure<f32, Scaled>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Padding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Builds [`Padding`].
#[derive(Default)]
#[must_use]
pub struct Builder {
    padding: Padding,
}

impl Builder {
    /// Sets the left padding to `points`.
    pub fn left(mut self, points: f32) -> Self {
        self.padding.left = Some(Figure::new(points));
        self
    }

    /// Sets the right padding to `points`.
    pub fn right(mut self, points: f32) -> Self {
        self.padding.right = Some(Figure::new(points));
        self
    }

    /// Sets the top padding to `points`.
    pub fn top(mut self, points: f32) -> Self {
        self.padding.top = Some(Figure::new(points));
        self
    }

    /// Sets the bottom padding to `points`.
    pub fn bottom(mut self, points: f32) -> Self {
        self.padding.bottom = Some(Figure::new(points));
        self
    }

    /// Returns the built padding.
    pub fn finish(self) -> Padding {
        self.padding
    }
}
