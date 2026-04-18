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

/// Renders the data series as a vertical bar chart.
///
/// Bar width is 80 % of the minimum x-spacing between adjacent points.
/// Bars with `y ≤ 0` are skipped.
#[component]
pub fn Bar(data: GraphData, kind: BarKind) {
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
    let bar_width = if pts.len() < 2 {
        w * 0.8
    } else {
        let min_spacing = pts
            .windows(2)
            .map(|p| (p[1].0 - p[0].0).abs())
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
