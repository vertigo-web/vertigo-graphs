use crate::types::GraphData;
use vertigo::{Computed, DomNode, component, computed_tuple, dom};

/// Root wrapper that renders the chart title and SVG viewport.
///
/// - `children` — chart layer components (Grid, Axes, Line, Bar, …).
/// - `padding_x` — extra horizontal space added on each side of the SVG viewBox
///   so edge elements (e.g. wide bars) are not clipped by the Y axis.
/// - `background` — optional CSS background value; omit or pass empty string for none.
#[component]
pub fn ChartContainer(data: GraphData, children: Vec<DomNode>, padding_x: f64, background: String) {
    let view_box = computed_tuple!(w => &data.scale_x, h => &data.scale_y)
        .map(move |(w, h)| format!("{} 0 {} {}", -padding_x, w + 2.0 * padding_x, h));
    let style = Computed::from(move |_| {
        if background.is_empty() {
            "border: 1px solid #ccc".to_string()
        } else {
            format!("border: 1px solid #ccc; background: {background}")
        }
    });
    dom! {
        <div>
            <p>{data.label}</p>
            <svg width={data.scale_x} height={data.scale_y} viewBox={view_box} style={style}>
                {..children}
            </svg>
        </div>
    }
}
