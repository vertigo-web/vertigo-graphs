use crate::line::{LineKind, build_line_path};
use crate::types::GraphData;
use vertigo::{Computed, component, computed_tuple, dom};

/// Gradient style for [`FillGradient`].
#[derive(Clone)]
pub enum GradientKind {
    /// Vertical linear gradient from `top_color` (opaque) to `bottom_color` (transparent).
    ///
    /// Because Vertigo's `dom!` macro lowercases SVG element names (breaking `<linearGradient>`),
    /// the gradient is implemented via a CSS `mask-image`; `bottom_color` is therefore ignored
    /// and the fade is always to fully transparent.
    Linear(&'static str, &'static str),
}

/// Renders a filled area beneath the data series with a vertical gradient fade.
///
/// The top edge follows the same curve as [`Line`] when the same `line_kind` is used,
/// ensuring the fill aligns visually with the stroke.
#[component]
pub fn FillGradient(data: GraphData, kind: GradientKind, line_kind: LineKind) {
    let (color_top, color_bottom) = match kind {
        GradientKind::Linear(top, bottom) => (top, bottom),
    };
    let fill = Computed::from(move |_| color_top.trim_end_matches(';').to_string());
    let _ = color_bottom;
    let d = computed_tuple!(pts => &data.points, h => &data.scale_y)
        .map(move |(pts, h)| build_fill_path(&pts, h, &line_kind));
    dom! {
        <path d={d} fill={fill} stroke="none" style="mask-image: linear-gradient(to bottom, black, transparent); -webkit-mask-image: linear-gradient(to bottom, black, transparent)" />
    }
}

fn build_fill_path(pts: &[(f64, f64)], h: f64, line_kind: &LineKind) -> String {
    if pts.is_empty() {
        return String::new();
    }
    let last_x = pts.last().unwrap().0;
    let first_x = pts[0].0;
    let line = build_line_path(pts, h, line_kind);
    format!("{line} L{last_x},{h} L{first_x},{h} Z")
}
