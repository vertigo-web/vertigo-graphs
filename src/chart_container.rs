use crate::types::GraphData;
use vertigo::{Computed, DomNode, component, computed_tuple, dom};

/// Options for [`ChartContainer`].
#[derive(Clone, Default)]
pub struct ChartContainerParams {
    /// Extra horizontal space added on each side of the SVG viewBox so edge elements
    /// (e.g. wide bars, Y axis labels) are not clipped. The SVG element widens by the
    /// same amount, so the chart keeps its 1:1 scale. Default: `0.0`.
    pub padding_x: f64,
    /// Extra vertical space added above and below the SVG viewBox so edge elements
    /// (e.g. top and bottom Y axis labels) are not clipped. The SVG element grows by the
    /// same amount. Default: `0.0`.
    pub padding_y: f64,
    /// CSS background value. Default: empty, for no background.
    pub background: String,
}

/// Root wrapper that renders the chart title and SVG viewport.
///
/// `children` are the chart layer components (Grid, Axes, Line, Bar, …).
/// See [`ChartContainerParams`] for the options.
#[component]
pub fn ChartContainer(data: GraphData, children: Vec<DomNode>, params: ChartContainerParams) {
    let ChartContainerParams {
        padding_x,
        padding_y,
        background,
    } = params;
    let width = data.scale_x.map(move |w| w + 2.0 * padding_x);
    let height = data.scale_y.map(move |h| h + 2.0 * padding_y);
    let view_box = computed_tuple!(w => &data.scale_x, h => &data.scale_y).map(move |(w, h)| {
        format!(
            "{} {} {} {}",
            -padding_x,
            -padding_y,
            w + 2.0 * padding_x,
            h + 2.0 * padding_y
        )
    });
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
            <svg width={width} height={height} viewBox={view_box} style={style}>
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
    fn padding_grows_svg_to_match_view_box() {
        let data = GraphData {
            label: Computed::from(|_| "Chart".to_string()),
            scale_x: Computed::from(|_| 500.0),
            scale_y: Computed::from(|_| 400.0),
            points: Computed::from(|_| vec![]),
        };
        let params = ChartContainerParams {
            padding_x: 50.0,
            padding_y: 10.0,
            ..Default::default()
        };

        log_start();
        let _el = dom! {
            <ChartContainer {&data} {params}>
                <g />
            </ChartContainer>
        };

        let html = DomDebugFragment::from_log().to_pseudo_html();
        assert!(html.contains("viewBox='-50 -10 600 420'"), "{html}");
        assert!(html.contains("width='600'"), "{html}");
        assert!(html.contains("height='420'"), "{html}");
    }
}
