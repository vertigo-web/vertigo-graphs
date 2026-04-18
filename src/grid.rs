use crate::types::{GRID_DIVISIONS, GraphData};
use vertigo::{component, computed_tuple, dom};

/// Which grid lines to draw.
#[derive(Clone, Default)]
pub enum GridKind {
    /// Draw both horizontal (Y) and vertical (X) grid lines.
    #[default]
    Both,
    /// Draw horizontal grid lines only (useful when X positions are meaningful labels).
    YOnly,
}

/// Renders background grid lines at evenly spaced tick positions.
///
/// - `stroke` — line color; defaults to `"#e0e0e0"` (light gray) when empty.
#[component]
pub fn Grid(data: GraphData, kind: GridKind, stroke: String) {
    let stroke = if stroke.is_empty() {
        "#e0e0e0".to_string()
    } else {
        stroke
    };
    let d = computed_tuple!(w => &data.scale_x, h => &data.scale_y).map(move |(w, h)| {
        let mut s = String::new();
        // Iterate through every tick position so the top and bottom rows
        // also get a grid line aligned with their Y-axis label.
        for i in 0..=GRID_DIVISIONS {
            let y = h * (i as f64 / GRID_DIVISIONS as f64);
            s.push_str(&format!("M0,{y} L{w},{y} "));
            if matches!(kind, GridKind::Both) {
                let x = w * (i as f64 / GRID_DIVISIONS as f64);
                s.push_str(&format!("M{x},0 L{x},{h} "));
            }
        }
        s
    });
    dom! {
        <path d={d} fill="none" stroke={stroke} stroke-width="1" />
    }
}
