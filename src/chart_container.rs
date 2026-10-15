use crate::types::GraphData;
use vertigo::{Computed, DomNode, component, computed_tuple, dom};

/// Root wrapper that renders the chart title and SVG viewport.
///
/// - `children` — chart layer components (Grid, Axes, Line, Bar, …).
/// - `padding_x` — extra horizontal space added on each side of the SVG viewBox
///   so edge elements (e.g. wide bars) are not clipped by the Y axis.
///   The SVG element widens by the same amount, so the chart keeps its 1:1 scale.
/// - `background` — optional CSS background value; omit or pass empty string for none.
#[component]
pub fn ChartContainer(data: GraphData, children: Vec<DomNode>, padding_x: f64, background: String) {
    let width = data.scale_x.map(move |w| w + 2.0 * padding_x);
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
            <svg width={width} height={data.scale_y} viewBox={view_box} style={style}>
                {..children}
            </svg>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vertigo::dev::inspect::{DomDebugFragment, log_start};

    #[test]
    fn padding_widens_svg_to_match_view_box() {
        let data = GraphData {
            label: Computed::from(|_| "Chart".to_string()),
            scale_x: Computed::from(|_| 500.0),
            scale_y: Computed::from(|_| 400.0),
            points: Computed::from(|_| vec![]),
        };

        log_start();
        let _el = dom! {
            <ChartContainer {&data} padding_x={50.0} background={}>
                <g />
            </ChartContainer>
        };

        let html = DomDebugFragment::from_log().to_pseudo_html();
        assert!(html.contains("viewBox='-50 0 600 400'"), "{html}");
        assert!(html.contains("width='600'"), "{html}");
    }
}
