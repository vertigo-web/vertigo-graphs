use crate::types::{GRID_DIVISIONS, GraphData};
use vertigo::{component, computed_tuple, dom};

/// Renders the horizontal X axis with evenly spaced tick marks.
///
/// `stroke` sets the line color; defaults to `"black"` when empty.
#[component]
pub fn XAxis(data: GraphData, stroke: String) {
    let stroke = if stroke.is_empty() {
        "black".to_string()
    } else {
        stroke
    };
    let d = computed_tuple!(w => &data.scale_x, h => &data.scale_y).map(|(w, h)| {
        let mut s = format!("M0,{h} L{w},{h} ");
        for i in 0..=GRID_DIVISIONS {
            let x = w * (i as f64 / GRID_DIVISIONS as f64);
            s.push_str(&format!("M{x},{h} L{x},{} ", h - 6.0));
        }
        s
    });
    dom! {
        <path d={d} fill="none" {stroke} stroke-width="1" />
    }
}
