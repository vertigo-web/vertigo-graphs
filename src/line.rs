use crate::types::GraphData;
use vertigo::{component, computed_tuple, dom};

/// Interpolation style for the [`Line`] and [`FillGradient`](crate::FillGradient) components.
#[derive(Clone, Default)]
pub enum LineKind {
    /// Connect points with straight segments.
    #[default]
    Straight,
    /// Smooth curve through points using a Catmull-Rom spline (converted to cubic Bézier).
    Rounded,
}

/// Options for [`Line`].
#[derive(Clone)]
pub struct LineParams {
    /// Straight segments or a smooth curve. Default: [`LineKind::Straight`].
    pub kind: LineKind,
    /// Stroke color. Default: `"steelblue"`.
    pub color: String,
}

impl Default for LineParams {
    fn default() -> Self {
        Self {
            kind: LineKind::default(),
            color: "steelblue".to_string(),
        }
    }
}

/// Renders the data series as a stroked SVG path.
///
/// See [`LineParams`] for the options.
#[component]
pub fn Line(data: GraphData, params: LineParams) {
    let LineParams { kind, color } = params;
    let d = computed_tuple!(pts => &data.points, h => &data.scale_y)
        .map(move |(pts, h)| build_line_path(&pts, h, &kind));
    dom! {
        <path d={d} fill="none" stroke={color} stroke-width="2" />
    }
}

pub(crate) fn build_line_path(pts: &[(f64, f64)], h: f64, kind: &LineKind) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Splits a rounded path into `[start, cp1, cp2, end]` cubic segments.
    fn cubic_segments(path: &str) -> Vec<[(f64, f64); 4]> {
        fn point(s: &str) -> (f64, f64) {
            let (x, y) = s.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        }
        let mut parts = path.strip_prefix('M').unwrap().split(" C");
        let mut start = point(parts.next().unwrap());
        parts
            .map(|part| {
                let pts: Vec<_> = part.split(' ').map(point).collect();
                let segment = [start, pts[0], pts[1], pts[2]];
                start = pts[2];
                segment
            })
            .collect()
    }

    #[test]
    fn empty_points_give_empty_path() {
        assert_eq!(build_line_path(&[], 10.0, &LineKind::Straight), "");
        assert_eq!(build_line_path(&[], 10.0, &LineKind::Rounded), "");
    }

    #[test]
    fn straight_flips_y_axis() {
        let pts = [(0.0, 0.0), (5.0, 10.0), (10.0, 4.0)];
        assert_eq!(
            build_line_path(&pts, 10.0, &LineKind::Straight),
            "M0,10 L5,0 L10,6"
        );
    }

    #[test]
    fn rounded_single_point_is_just_a_move() {
        assert_eq!(
            build_line_path(&[(3.0, 2.0)], 10.0, &LineKind::Rounded),
            "M3,8"
        );
    }

    #[test]
    fn rounded_passes_through_every_point() {
        let pts = [(0.0, 0.0), (5.0, 10.0), (10.0, 4.0), (20.0, 7.0)];
        let segments = cubic_segments(&build_line_path(&pts, 10.0, &LineKind::Rounded));
        assert_eq!(segments.len(), pts.len() - 1);
        for (segment, pair) in segments.iter().zip(pts.windows(2)) {
            assert_eq!(segment[0], (pair[0].0, 10.0 - pair[0].1));
            assert_eq!(segment[3], (pair[1].0, 10.0 - pair[1].1));
        }
    }

    #[test]
    fn rounded_control_points_stay_within_segment_x_range() {
        // Uneven spacing makes raw Catmull-Rom tangents overshoot along X.
        let pts = [(0.0, 0.0), (1.0, 10.0), (100.0, 0.0), (101.0, 10.0)];
        let path = build_line_path(&pts, 10.0, &LineKind::Rounded);
        for [start, cp1, cp2, end] in cubic_segments(&path) {
            let range = start.0..=end.0;
            assert!(
                range.contains(&cp1.0) && range.contains(&cp2.0),
                "control points {cp1:?} {cp2:?} outside {range:?}"
            );
        }
    }
}
