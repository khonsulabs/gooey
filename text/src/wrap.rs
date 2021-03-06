use approx::abs_diff_eq;
use gooey_core::{
    figures::{Figure, Size},
    styles::Style,
    Scaled,
};
use gooey_renderer::{Renderer, TextMetrics};

mod measured;
mod tokenizer;
pub(crate) use self::{measured::*, tokenizer::*};
use crate::{
    prepared::{PreparedLine, PreparedSpan, PreparedText},
    Text,
};

pub(crate) struct TextWrapper<'a, R: Renderer> {
    options: TextWrap,
    renderer: &'a R,
    context_style: Option<&'a Style>,
    prepared_text: PreparedText,
}

pub(crate) enum ParserStatus {
    LineStart,
    InWord,
    TrailingPunctuation,
    Whitespace,
}

struct TextWrapState {
    width: Option<Figure<f32, Scaled>>,
    current_vmetrics: Option<TextMetrics<Scaled>>,
    current_span_offset: Figure<f32, Scaled>,
    current_groups: Vec<SpanGroup>,
    lines: Vec<PreparedLine>,
}

impl TextWrapState {
    #[must_use]
    fn push_group(&mut self, group: SpanGroup, single_line: bool) -> bool {
        let mut new_line = false;
        if let SpanGroup::EndOfLine(metrics) = &group {
            self.update_vmetrics(*metrics);
            self.new_line();
            true
        } else {
            let spans = group.spans();
            let total_width = spans
                .iter()
                .map(|s| s.metrics().width)
                .fold(Figure::default(), |sum, width| sum + width);

            if let Some(width) = self.width {
                let new_width = total_width + self.current_span_offset;
                let remaining_width: Figure<f32, Scaled> = width - new_width;

                // If the value is negative and isn't zero (-0. is a valid float)
                if !abs_diff_eq!(remaining_width.get(), 0., epsilon = 0.1)
                    && remaining_width.get().is_sign_negative()
                {
                    if abs_diff_eq!(self.current_span_offset.get(), 0.) {
                        // TODO Split the group if it can't fit on a single line
                        // For now, just render it anyways.
                    } else {
                        self.new_line();
                        new_line = true;
                        if single_line {
                            return new_line;
                        }
                    }
                }
            }
            self.current_span_offset += total_width;
            self.current_groups.push(group);
            new_line
        }
    }

    fn update_vmetrics(&mut self, new_metrics: TextMetrics<Scaled>) {
        self.current_vmetrics = match self.current_vmetrics {
            Some(metrics) => Some(TextMetrics {
                ascent: metrics.ascent.max(new_metrics.ascent),
                descent: metrics.descent.min(new_metrics.descent),
                line_gap: metrics.line_gap.max(new_metrics.line_gap),
                width: Figure::default(),
            }),
            None => Some(new_metrics),
        }
    }

    fn position_span(&mut self, span: &mut PreparedSpan) {
        let width = span.metrics().width;
        span.set_location(self.current_span_offset);
        self.current_span_offset += width;
    }

    fn new_line(&mut self) {
        // Remove any whitespace from the end of the line
        while matches!(self.current_groups.last(), Some(SpanGroup::Whitespace(_))) {
            self.current_groups.pop();
        }

        let mut spans = Vec::new();
        for group in &self.current_groups {
            for span in group.spans() {
                spans.push(span);
            }
        }

        self.current_span_offset = Figure::default();
        for span in &mut spans {
            self.update_vmetrics(*span.metrics());
            self.position_span(span);
        }

        if let Some(metrics) = self.current_vmetrics.take() {
            self.lines.push(PreparedLine {
                spans,
                metrics,
                alignment_offset: Figure::default(),
            });
        }
        self.current_span_offset = Figure::default();
        self.current_groups.clear();
    }

    fn finish(mut self) -> Vec<PreparedLine> {
        if !self.current_groups.is_empty() || self.lines.is_empty() {
            self.new_line();
        }

        self.lines
    }
}

impl<'a, R: Renderer> TextWrapper<'a, R> {
    pub fn wrap(
        text: &Text,
        renderer: &'a R,
        options: TextWrap,
        context_style: Option<&'a Style>,
    ) -> PreparedText {
        TextWrapper {
            options,
            renderer,
            context_style,
            prepared_text: PreparedText::default(),
        }
        .wrap_text(text)
    }

    fn wrap_text(mut self, text: &Text) -> PreparedText {
        let width = self.options.width();

        let measured = MeasuredText::new(text, self.renderer, self.context_style);

        let mut state = TextWrapState {
            width,
            current_span_offset: Figure::default(),
            current_vmetrics: None,
            current_groups: Vec::new(),
            lines: Vec::new(),
        };

        match measured.info {
            MeasuredTextInfo::Groups(groups) => {
                let single_line = self.options.is_single_line();
                for group in groups {
                    if state.push_group(group, single_line) && single_line {
                        break;
                    }
                }

                self.prepared_text.lines = state.finish();
                if let Some(width) = width {
                    self.prepared_text.align(width);
                }
            }
            MeasuredTextInfo::NoText(metrics) => {
                self.prepared_text.lines.push(PreparedLine {
                    metrics,
                    alignment_offset: Figure::default(),
                    spans: Vec::default(),
                });
            }
        }

        self.prepared_text
    }
}

/// Text wrapping options.
#[derive(Debug, Clone, Copy)]
pub enum TextWrap {
    /// Do not wrap the text.
    NoWrap,
    /// Render the text in a single line. Do not render past `max_width`.
    SingleLine {
        /// The width of the line to render within.
        width: Figure<f32, Scaled>,
    },
    /// Render a multi-line text block.
    MultiLine {
        /// The size of the text area.
        size: Size<f32, Scaled>,
    },
}

impl TextWrap {
    /// Returns true if this is a multiline wrap.
    #[must_use]
    pub fn is_multiline(&self) -> bool {
        matches!(self, Self::MultiLine { .. })
    }

    /// Returns true if this is a single-line wrap.
    #[must_use]
    pub fn is_single_line(&self) -> bool {
        !self.is_multiline()
    }

    /// Returns the width of the rendered area, if one was provided.
    #[must_use]
    pub fn width(&self) -> Option<Figure<f32, Scaled>> {
        match self {
            Self::MultiLine { size, .. } => Some(Figure::new(size.width)),
            Self::SingleLine { width, .. } => Some(*width),
            Self::NoWrap => None,
        }
    }

    /// Returns the height of the rendendered area, if one was provided.
    #[must_use]
    pub fn height(&self) -> Option<Figure<f32, Scaled>> {
        match self {
            Self::MultiLine { size, .. } => Some(Figure::new(size.height)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use gooey_core::{
        figures::{DisplayScale, Displayable, Point, Rect},
        styles::{FontSize, Style, SystemTheme},
        Pixels,
    };
    use gooey_renderer::StrokeOptions;

    use super::*;
    use crate::Span;

    #[derive(Debug)]
    struct MockTextRenderer;

    impl Renderer for MockTextRenderer {
        fn theme(&self) -> SystemTheme {
            SystemTheme::default()
        }

        fn size(&self) -> gooey_core::figures::Size<f32, Scaled> {
            unimplemented!()
        }

        fn clip_to(&self, _bounds: gooey_core::figures::Rect<f32, Scaled>) -> Self {
            unimplemented!()
        }

        fn clip_bounds(&self) -> gooey_core::figures::Rect<f32, Scaled> {
            unimplemented!()
        }

        fn scale(&self) -> DisplayScale<f32> {
            unimplemented!()
        }

        fn stroke_rect(
            &self,
            _rect: &impl Displayable<f32, Pixels = Rect<f32, Pixels>>,
            _style: &StrokeOptions,
        ) {
            unimplemented!()
        }

        fn fill_rect(
            &self,
            _rect: &impl Displayable<f32, Pixels = Rect<f32, Pixels>>,
            _color: gooey_core::styles::Color,
        ) {
            unimplemented!()
        }

        fn stroke_line<P: Displayable<f32, Pixels = Point<f32, Pixels>>>(
            &self,
            _point_a: P,
            _point_b: P,
            _options: &StrokeOptions,
        ) {
            unimplemented!()
        }

        fn render_text(
            &self,
            _text: &str,
            _baseline_origin: impl Displayable<f32, Pixels = Point<f32, Pixels>>,
            _options: &gooey_renderer::TextOptions,
        ) {
            unimplemented!()
        }

        #[allow(clippy::cast_precision_loss)]
        fn measure_text(
            &self,
            text: &str,
            options: &gooey_renderer::TextOptions,
        ) -> TextMetrics<Scaled> {
            // Return a fixed width per character, based on the font size.;
            TextMetrics {
                width: options.text_size * text.len() as f32 * 0.6,
                ascent: options.text_size * 0.8,
                descent: -options.text_size * 0.2,
                line_gap: options.text_size * 0.1,
            }
        }

        fn draw_image(
            &self,
            _image: &gooey_core::assets::Image,
            _location: impl Displayable<f32, Pixels = Point<f32, Pixels>>,
        ) {
            unimplemented!()
        }
    }

    #[test]
    /// This test should have "This line should " be on the first line and "wrap" on the second
    fn wrap_one_word() {
        let renderer = MockTextRenderer;
        let wrap = Text::from(vec![Span::new(
            "This line should wrap",
            Style::new().with(FontSize::new(12.)),
        )])
        .wrap(
            &renderer,
            TextWrap::MultiLine {
                size: Size::new(80.0, f32::MAX),
            },
            None,
        );
        println!("Wrapped text: {:#?}", wrap);
        assert_eq!(wrap.lines.len(), 2);
        assert_eq!(wrap.lines[0].spans.len(), 3); // "this"," ","line"
        assert_eq!(wrap.lines[1].spans.len(), 3); // "should"," ","wrap"
        assert_eq!(wrap.lines[1].spans[0].text(), "should");
    }

    #[test]
    /// This test should have "This line should " be on the first line and "wrap" on the second
    fn wrap_one_word_different_span() {
        let renderer = MockTextRenderer;

        let first_style = Style::new().with(FontSize::new(12.));

        let second_style = Style::new().with(FontSize::new(10.));

        let wrap = Text::from(vec![
            Span::new("This line should ", first_style),
            Span::new("wrap", second_style),
        ])
        .wrap(
            &renderer,
            TextWrap::MultiLine {
                size: Size::new(130.0, f32::MAX),
            },
            None,
        );
        assert_eq!(wrap.lines.len(), 2);
        assert_eq!(wrap.lines[0].spans.len(), 5);
        assert_eq!(wrap.lines[1].spans.len(), 1);
        assert_eq!(wrap.lines[1].spans[0].text(), "wrap");
        assert_ne!(
            wrap.lines[0].spans[0].metrics().ascent,
            wrap.lines[1].spans[0].metrics().ascent
        );
    }
}
