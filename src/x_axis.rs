use crate::types::{GRID_DIVISIONS, GraphData};
use vertigo::{component, computed_tuple, dom};

/// Options for [`XAxis`].
#[derive(Clone)]
pub struct XAxisParams {
    /// Line color. Default: `"black"`.
    pub stroke: String,
}

impl Default for XAxisParams {
    fn default() -> Self {
        Self {
            stroke: "black".to_string(),
        }
    }
}

/// Renders the horizontal X axis with evenly spaced tick marks.
///
/// See [`XAxisParams`] for the options.
#[component]
pub fn XAxis(data: GraphData, params: XAxisParams) {
    let XAxisParams { stroke } = params;
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
