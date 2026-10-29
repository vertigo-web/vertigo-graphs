use crate::types::{GRID_DIVISIONS, GraphData};
use vertigo::{component, dom};

/// Options for [`YAxis`].
#[derive(Clone)]
pub struct YAxisParams {
    /// Line and label color. Default: `"black"`.
    pub stroke: String,
    /// When `true`, prints numeric value labels to the left of each tick. Default: `false`.
    ///
    /// The labels stick out of the plot to the left, and the top and bottom ones above
    /// and below it, so give the chart room for them with
    /// [`ChartContainerParams`](crate::ChartContainerParams)'s `padding_x` and `padding_y`.
    pub values: bool,
    /// Number of decimal places to use when formatting `values` labels. Default: `0`.
    pub decimals: u32,
    /// If positive, overrides `data.scale_y` for label *values* only (tick positions
    /// still use `data.scale_y`). Lets callers scale the plot to SVG pixels while
    /// displaying labels in domain units. Default: `0.0`, which uses `data.scale_y`.
    pub value_scale: f64,
    /// When `true`, omits the axis line and tick marks and only renders the value labels
    /// (requires `values` to have any effect). Default: `false`.
    pub labels_only: bool,
    /// How many steps the axis is divided into, with a tick and a label at each step's ends.
    /// 0 counts as 1. Default: [`GRID_DIVISIONS`]. [`nice_ticks`](crate::nice_ticks) picks it
    /// with `value_scale` and `decimals` for round labels, and [`GridParams`](crate::GridParams)'s
    /// `y_divisions` keeps the grid lines on the ticks.
    pub divisions: usize,
}

impl Default for YAxisParams {
    fn default() -> Self {
        Self {
            stroke: "black".to_string(),
            values: false,
            decimals: 0,
            value_scale: 0.0,
            labels_only: false,
            divisions: GRID_DIVISIONS,
        }
    }
}

/// Renders the vertical Y axis with evenly spaced tick marks and optional value labels.
///
/// See [`YAxisParams`] for the options.
#[component]
pub fn YAxis(data: GraphData, params: YAxisParams) {
    let YAxisParams {
        stroke,
        values,
        decimals,
        value_scale,
        labels_only,
        divisions,
    } = params;
    let divisions = divisions.max(1);
    if values {
        let fill = stroke.clone();
        let precision = decimals as usize;
        let labels = data.scale_y.render_value(move |h| {
            let label_scale = if value_scale > 0.0 { value_scale } else { h };
            let nodes = (0..=divisions).map(|i| {
                let ty = h * (i as f64 / divisions as f64) + 4.0;
                let val = label_scale * ((divisions - i) as f64 / divisions as f64);
                let text = format!("{:.*}", precision, val);
                let fc = fill.clone();
                dom! {
                    <text x="-8" y={ty} font-size="11" text-anchor="end" fill={fc}>{text}</text>
                }
            });
            dom! { <g>{..nodes}</g> }
        });
        if labels_only {
            dom! { <g>{labels}</g> }
        } else {
            let d = data.scale_y.map(move |h| {
                let mut s = format!("M0,0 L0,{h} ");
                for i in 0..=divisions {
                    let y = h * (i as f64 / divisions as f64);
                    s.push_str(&format!("M0,{y} L6,{y} "));
                }
                s
            });
            dom! {
                <g>
                    <path d={d} fill="none" stroke-width="1" stroke={stroke} />
                    {labels}
                </g>
            }
        }
    } else if labels_only {
        dom! { <g /> }
    } else {
        let d = data.scale_y.map(move |h| {
            let mut s = format!("M0,0 L0,{h} ");
            for i in 0..=divisions {
                let y = h * (i as f64 / divisions as f64);
                s.push_str(&format!("M0,{y} L6,{y} "));
            }
            s
        });
        dom! {
            <path d={d} fill="none" stroke-width="1" {stroke} />
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vertigo::Computed;
    use vertigo::dev::inspect::{DomDebugFragment, log_start};

    fn render(params: YAxisParams) -> String {
        let data = GraphData {
            label: Computed::from(|_| "Chart".to_string()),
            scale_x: Computed::from(|_| 500.0),
            scale_y: Computed::from(|_| 500.0),
            points: Computed::from(|_| vec![]),
        };

        log_start();
        let _el = dom! { <YAxis {&data} {params} /> };
        DomDebugFragment::from_log().to_pseudo_html()
    }

    #[test]
    fn defaults_draw_black_axis_without_labels() {
        let html = render(YAxisParams::default());
        assert!(html.contains("<path"), "{html}");
        assert!(html.contains("stroke='black'"), "{html}");
        assert!(!html.contains("<text"), "{html}");
    }

    #[test]
    fn labels_use_decimals_and_value_scale() {
        let html = render(YAxisParams {
            values: true,
            decimals: 1,
            value_scale: 10.0,
            ..Default::default()
        });
        assert!(html.contains("<path"), "{html}");
        for label in ["10.0", "8.0", "2.0", "0.0"] {
            assert!(
                html.contains(&format!(">{label}</text>")),
                "{label}: {html}"
            );
        }
    }

    #[test]
    fn divisions_set_the_labels() {
        let html = render(YAxisParams {
            values: true,
            value_scale: 60.0,
            divisions: 6,
            ..Default::default()
        });
        let labels = html.matches("<text").count();
        assert_eq!(labels, 7, "{html}");
        for label in ["60", "50", "10", "0"] {
            assert!(
                html.contains(&format!(">{label}</text>")),
                "{label}: {html}"
            );
        }
        // The axis line plus a tick at each label
        assert_eq!(html.matches(" L6,").count(), 7, "{html}");
    }

    #[test]
    fn labels_only_drops_axis_line() {
        let html = render(YAxisParams {
            values: true,
            labels_only: true,
            ..Default::default()
        });
        assert!(!html.contains("<path"), "{html}");
        assert!(html.contains(">500</text>"), "{html}");
    }
}
