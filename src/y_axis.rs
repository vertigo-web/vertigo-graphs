use crate::types::{GRID_DIVISIONS, GraphData};
use vertigo::{component, dom};

/// Renders the vertical Y axis with evenly spaced tick marks and optional value labels.
///
/// - `stroke` — line and label color; defaults to `"black"` when empty.
/// - `values` — when `true`, prints numeric value labels to the left of each tick.
/// - `decimals` — number of decimal places to use when formatting `values` labels.
/// - `value_scale` — if positive, overrides `data.scale_y` for label *values*
///   only (tick positions still use `data.scale_y`). Lets callers scale the
///   plot to SVG pixels while displaying labels in domain units.
/// - `labels_only` — when `true`, omits the axis line and tick marks and only
///   renders the value labels (requires `values = true` to have any effect).
#[component]
pub fn YAxis(
    data: GraphData,
    stroke: String,
    values: bool,
    decimals: u32,
    value_scale: f64,
    labels_only: bool,
) {
    let stroke = if stroke.is_empty() {
        "black".to_string()
    } else {
        stroke
    };
    if values {
        let fill = stroke.clone();
        let precision = decimals as usize;
        let labels = data.scale_y.render_value(move |h| {
            let label_scale = if value_scale > 0.0 { value_scale } else { h };
            let nodes = (0..=GRID_DIVISIONS).map(|i| {
                let ty = h * (i as f64 / GRID_DIVISIONS as f64) + 4.0;
                let val = label_scale * ((GRID_DIVISIONS - i) as f64 / GRID_DIVISIONS as f64);
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
            let d = data.scale_y.map(|h| {
                let mut s = format!("M0,0 L0,{h} ");
                for i in 0..=GRID_DIVISIONS {
                    let y = h * (i as f64 / GRID_DIVISIONS as f64);
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
        let d = data.scale_y.map(|h| {
            let mut s = format!("M0,0 L0,{h} ");
            for i in 0..=GRID_DIVISIONS {
                let y = h * (i as f64 / GRID_DIVISIONS as f64);
                s.push_str(&format!("M0,{y} L6,{y} "));
            }
            s
        });
        dom! {
            <path d={d} fill="none" stroke-width="1" {stroke} />
        }
    }
}
