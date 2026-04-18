use crate::types::GraphData;
use vertigo::{component, dom};

/// Renders the chart label as an in-SVG text element (top-left corner).
#[component]
pub fn Legend(data: GraphData) {
    dom! {
        <text x="10" y="20" font-size="14" fill="black">{data.label}</text>
    }
}
