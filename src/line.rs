use crate::types::GraphData;
use vertigo::{component, computed_tuple, dom};

/// Interpolation style for the [`Line`] and [`FillGradient`] components.
#[derive(Clone, Default)]
pub enum LineKind {
    /// Connect points with straight segments.
    #[default]
    Straight,
    /// Smooth curve through points using a Catmull-Rom spline (converted to cubic Bézier).
    Rounded,
}

/// Renders the data series as a stroked SVG path.
///
/// - `kind` — straight segments or smooth curve (default: `LineKind::Straight`).
/// - `color` — stroke color; defaults to `"steelblue"` when empty.
#[component]
pub fn Line(data: GraphData, kind: LineKind, color: String) {
    let stroke = if color.is_empty() {
        "steelblue".to_string()
    } else {
        color
    };
    let d = computed_tuple!(pts => &data.points, h => &data.scale_y)
        .map(move |(pts, h)| build_line_path(&pts, h, &kind));
    dom! {
        <path d={d} fill="none" {stroke} stroke-width="2" />
    }
}

pub fn build_line_path(pts: &[(f64, f64)], h: f64, kind: &LineKind) -> String {
    if pts.is_empty() {
        return String::new();
    }
    let svg: Vec<(f64, f64)> = pts.iter().map(|&(x, y)| (x, h - y)).collect();
    let mut s = format!("M{},{}", svg[0].0, svg[0].1);
    match kind {
        LineKind::Straight => {
            for &(x, y) in &svg[1..] {
                s.push_str(&format!(" L{x},{y}"));
            }
        }
        LineKind::Rounded => {
            // Catmull-Rom spline converted to cubic Bezier.
            // X components of control points are clamped to the [p1.x, p2.x]
            // segment so the curve always advances monotonically along X
            // (the raw tangent can overshoot when neighbour spacing is uneven,
            // making the spline loop back on itself).
            for i in 0..svg.len() - 1 {
                let p0 = if i == 0 { svg[0] } else { svg[i - 1] };
                let p1 = svg[i];
                let p2 = svg[i + 1];
                let p3 = if i + 2 < svg.len() {
                    svg[i + 2]
                } else {
                    svg[svg.len() - 1]
                };
                let (x_lo, x_hi) = if p1.0 <= p2.0 {
                    (p1.0, p2.0)
                } else {
                    (p2.0, p1.0)
                };
                let cp1_x = (p1.0 + (p2.0 - p0.0) / 6.0).clamp(x_lo, x_hi);
                let cp1_y = p1.1 + (p2.1 - p0.1) / 6.0;
                let cp2_x = (p2.0 - (p3.0 - p1.0) / 6.0).clamp(x_lo, x_hi);
                let cp2_y = p2.1 - (p3.1 - p1.1) / 6.0;
                s.push_str(&format!(
                    " C{cp1_x},{cp1_y} {cp2_x},{cp2_y} {},{}",
                    p2.0, p2.1
                ));
            }
        }
    }
    s
}
