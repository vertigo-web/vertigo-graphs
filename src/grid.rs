use crate::types::{GRID_DIVISIONS, GraphData};
use vertigo::{component, computed_tuple, dom};

/// Which grid lines to draw.
#[derive(Clone, Default)]
pub enum GridKind {
    /// Draw both horizontal (Y) and vertical (X) grid lines.
    #[default]
    Both,
    /// Draw horizontal grid lines only (useful when X positions are meaningful labels).
    YOnly,
}

/// Options for [`Grid`].
#[derive(Clone)]
pub struct GridParams {
    /// Which grid lines to draw. Default: [`GridKind::Both`].
    pub kind: GridKind,
    /// Line color. Default: `"#e0e0e0"` (light gray).
    pub stroke: String,
    /// How many steps the horizontal lines divide the height into, to keep them on the ticks of
    /// a [`YAxis`](crate::YAxis) with the same `divisions`. 0 counts as 1. Default:
    /// [`GRID_DIVISIONS`], as are the vertical lines always, like the [`XAxis`](crate::XAxis) ticks.
    pub y_divisions: usize,
}

impl Default for GridParams {
    fn default() -> Self {
        Self {
            kind: GridKind::default(),
            stroke: "#e0e0e0".to_string(),
            y_divisions: GRID_DIVISIONS,
        }
    }
}

/// Renders background grid lines at evenly spaced tick positions.
///
/// See [`GridParams`] for the options.
#[component]
pub fn Grid(data: GraphData, params: GridParams) {
    let GridParams {
        kind,
        stroke,
        y_divisions,
    } = params;
    let y_divisions = y_divisions.max(1);
    let d = computed_tuple!(w => &data.scale_x, h => &data.scale_y).map(move |(w, h)| {
        let mut s = String::new();
        // Iterate through every tick position so the top and bottom rows
        // also get a grid line aligned with their Y-axis label.
        for i in 0..=y_divisions {
            let y = h * (i as f64 / y_divisions as f64);
            s.push_str(&format!("M0,{y} L{w},{y} "));
        }
        if matches!(kind, GridKind::Both) {
            for i in 0..=GRID_DIVISIONS {
                let x = w * (i as f64 / GRID_DIVISIONS as f64);
                s.push_str(&format!("M{x},0 L{x},{h} "));
            }
        }
        s
    });
    dom! {
        <path d={d} fill="none" stroke={stroke} stroke-width="1" />
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vertigo::Computed;
    use vertigo::dev::inspect::{DomDebugFragment, log_start};

    fn render(params: GridParams) -> String {
        let data = GraphData {
            label: Computed::from(|_| "Chart".to_string()),
            scale_x: Computed::from(|_| 500.0),
            scale_y: Computed::from(|_| 600.0),
            points: Computed::from(|_| vec![]),
        };

        log_start();
        let _el = dom! { <Grid {&data} {params} /> };
        DomDebugFragment::from_log().to_pseudo_html()
    }

    #[test]
    fn y_divisions_set_the_horizontal_lines() {
        let html = render(GridParams {
            kind: GridKind::YOnly,
            y_divisions: 6,
            ..Default::default()
        });
        assert_eq!(html.matches('M').count(), 7, "{html}");
        assert!(html.contains("M0,100 L500,100"), "{html}");
    }

    #[test]
    fn vertical_lines_keep_the_default_divisions() {
        let html = render(GridParams {
            y_divisions: 6,
            ..Default::default()
        });
        assert_eq!(html.matches('M').count(), 7 + GRID_DIVISIONS + 1, "{html}");
        assert!(html.contains("M100,0 L100,600"), "{html}");
    }
}
