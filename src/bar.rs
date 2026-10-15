use crate::types::GraphData;
use vertigo::{component, computed_tuple, dom};

/// Corner style for the [`Bar`] component.
#[derive(Clone, Default)]
pub enum BarKind {
    /// Rectangular bars with sharp corners.
    #[default]
    Straight,
    /// Bars with rounded top corners; the `f64` is the corner radius in SVG units.
    /// Radius is automatically clamped so it never exceeds half the bar width or bar height.
    Rounded(f64),
}

/// Options for [`Bar`].
#[derive(Clone, Default)]
pub struct BarParams {
    /// Corner style. Default: [`BarKind::Straight`].
    pub kind: BarKind,
}

/// Renders the data series as a vertical bar chart.
///
/// Bar width is 80 % of the minimum spacing between distinct x positions
/// (points may come in any order), or 80 % of the chart width when all points
/// share one x position. Bars with `y ≤ 0` are skipped.
///
/// See [`BarParams`] for the options.
#[component]
pub fn Bar(data: GraphData, params: BarParams) {
    let BarParams { kind } = params;
    let d = computed_tuple!(pts => &data.points, w => &data.scale_x, h => &data.scale_y)
        .map(move |(pts, w, h)| build_bar_path(&pts, w, h, &kind));
    dom! {
        <path d={d} fill="steelblue" stroke="none" />
    }
}

fn build_bar_path(pts: &[(f64, f64)], w: f64, h: f64, kind: &BarKind) -> String {
    if pts.is_empty() {
        return String::new();
    }
    // Measure gaps between sorted, distinct x positions so unsorted input
    // or duplicate x values don't shrink bars to zero width.
    let mut xs: Vec<f64> = pts.iter().map(|&(x, _)| x).collect();
    xs.sort_by(f64::total_cmp);
    xs.dedup();
    let bar_width = if xs.len() < 2 {
        w * 0.8
    } else {
        let min_spacing = xs
            .windows(2)
            .map(|p| p[1] - p[0])
            .fold(f64::INFINITY, f64::min);
        min_spacing * 0.8
    };
    let mut s = String::new();
    for &(x, y) in pts {
        if y <= 0.0 {
            continue;
        }
        let xl = x - bar_width / 2.0;
        let xr = x + bar_width / 2.0;
        let yt = h - y;
        let yb = h;
        match kind {
            BarKind::Straight => {
                s.push_str(&format!("M{xl},{yb} V{yt} H{xr} V{yb} Z "));
            }
            BarKind::Rounded(r) => {
                let r = r.min(bar_width / 2.0).min(y / 2.0);
                s.push_str(&format!(
                    "M{xl},{yb} V{} Q{xl},{yt} {},{yt} H{} Q{xr},{yt} {xr},{} V{yb} Z ",
                    yt + r,
                    xl + r,
                    xr - r,
                    yt + r,
                ));
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_points_give_empty_path() {
        assert_eq!(build_bar_path(&[], 100.0, 10.0, &BarKind::Straight), "");
    }

    #[test]
    fn bar_width_is_80_percent_of_min_spacing() {
        // Min spacing 5 → width 4 → each bar spans x ± 2.
        let pts = [(0.0, 1.0), (10.0, 1.0), (15.0, 1.0)];
        assert_eq!(
            build_bar_path(&pts, 20.0, 10.0, &BarKind::Straight),
            "M-2,10 V9 H2 V10 Z M8,10 V9 H12 V10 Z M13,10 V9 H17 V10 Z "
        );
    }

    #[test]
    fn unsorted_points_use_min_spacing_of_sorted_positions() {
        // Sorted positions 0, 5, 10 → min spacing 5 → width 4.
        let pts = [(10.0, 1.0), (0.0, 1.0), (5.0, 1.0)];
        assert_eq!(
            build_bar_path(&pts, 20.0, 10.0, &BarKind::Straight),
            "M8,10 V9 H12 V10 Z M-2,10 V9 H2 V10 Z M3,10 V9 H7 V10 Z "
        );
    }

    #[test]
    fn duplicate_x_does_not_collapse_bar_width() {
        // Positions 0, 0, 10 → distinct spacing 10 → width 8.
        let pts = [(0.0, 1.0), (0.0, 2.0), (10.0, 1.0)];
        assert_eq!(
            build_bar_path(&pts, 20.0, 10.0, &BarKind::Straight),
            "M-4,10 V9 H4 V10 Z M-4,10 V8 H4 V10 Z M6,10 V9 H14 V10 Z "
        );
    }

    #[test]
    fn non_positive_values_are_skipped() {
        let pts = [(0.0, 0.0), (10.0, -3.0), (20.0, 5.0)];
        assert_eq!(
            build_bar_path(&pts, 30.0, 10.0, &BarKind::Straight),
            "M16,10 V5 H24 V10 Z "
        );
    }

    #[test]
    fn rounded_radius_is_clamped_to_half_bar_height() {
        // Width 8, height 2 → radius clamps to 1.
        let pts = [(10.0, 2.0), (20.0, 0.0)];
        assert_eq!(
            build_bar_path(&pts, 30.0, 10.0, &BarKind::Rounded(100.0)),
            "M6,10 V9 Q6,8 7,8 H13 Q14,8 14,9 V10 Z "
        );
    }

    #[test]
    fn rounded_radius_is_clamped_to_half_bar_width() {
        // Width 8, height 20 → radius clamps to 4.
        let pts = [(10.0, 20.0), (20.0, 0.0)];
        assert_eq!(
            build_bar_path(&pts, 30.0, 30.0, &BarKind::Rounded(100.0)),
            "M6,30 V14 Q6,10 10,10 H10 Q14,10 14,14 V30 Z "
        );
    }
}
