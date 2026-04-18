use crate::types::GraphData;
use vertigo::{component, computed_tuple, dom};

/// Simple all-in-one line chart (single straight-line series, no axis components).
///
/// For composable charts with axes, grid, and multiple layers use [`ChartContainer`] instead.
#[component]
pub fn Graph(data: GraphData) {
    let path = computed_tuple!(points => &data.points, scale_y => &data.scale_y).map(
        |(points, height)| {
            let mut path = String::new();
            for (i, (x, y)) in points.iter().enumerate() {
                let cmd = if i == 0 { 'M' } else { 'L' };
                path.push_str(&format!("{}{},{} ", cmd, x, height - y));
            }
            path
        },
    );
    dom! {
        <div>
            <p>{data.label}</p>
            <svg width={data.scale_x} height={data.scale_y} style="border: 1px solid #ccc">
                <path d={path} fill="none" stroke="black" stroke-width="2" />
            </svg>
        </div>
    }
}
