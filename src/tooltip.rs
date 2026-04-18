use crate::types::GraphData;
use vertigo::{component, computed_tuple, dom};

/// Renders a simple stats overlay inside the SVG (bottom-left corner).
///
/// Currently displays point count and maximum Y value.
#[component]
pub fn Tooltip(data: GraphData) {
    let text = computed_tuple!(pts => &data.points, w => &data.scale_x, h => &data.scale_y).map(
        |(pts, _w, _h)| {
            if pts.is_empty() {
                return "No data".to_string();
            }
            let max_y = pts
                .iter()
                .map(|(_, y)| *y)
                .fold(f64::NEG_INFINITY, f64::max);
            format!("n={} max_y={:.0}", pts.len(), max_y)
        },
    );
    let y = data.scale_y.map(|h| h - 10.0);
    dom! {
        <text x="10" y={y} font-size="11" fill="#555">{text}</text>
    }
}
