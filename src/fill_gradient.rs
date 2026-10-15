use std::sync::atomic::{AtomicUsize, Ordering};

use crate::line::{LineKind, build_line_path};
use crate::types::GraphData;
use vertigo::{component, computed_tuple, dom};

/// Gradient style for [`FillGradient`].
#[derive(Clone)]
pub enum GradientKind {
    /// Vertical linear gradient from the first color (top) to the second color (bottom).
    Linear(&'static str, &'static str),
}

/// Source of document-unique gradient ids, as `url(#id)` references are page-wide.
static NEXT_GRADIENT_ID: AtomicUsize = AtomicUsize::new(0);

/// Renders a filled area beneath the data series with a vertical gradient fade.
///
/// The top edge follows the same curve as [`Line`](crate::Line) when the same `line_kind` is used,
/// ensuring the fill aligns visually with the stroke.
#[component]
pub fn FillGradient(data: GraphData, kind: GradientKind, line_kind: LineKind) {
    let GradientKind::Linear(color_top, color_bottom) = kind;
    let id = format!(
        "vertigo-graphs-fill-{}",
        NEXT_GRADIENT_ID.fetch_add(1, Ordering::Relaxed)
    );
    let fill = format!("url(#{id})");
    let d = computed_tuple!(pts => &data.points, h => &data.scale_y)
        .map(move |(pts, h)| build_fill_path(&pts, h, &line_kind));
    dom! {
        <g>
            <defs>
                <linearGradient id={id} x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0" stop-color={color_top} />
                    <stop offset="1" stop-color={color_bottom} />
                </linearGradient>
            </defs>
            <path d={d} fill={fill} stroke="none" />
        </g>
    }
}

fn build_fill_path(pts: &[(f64, f64)], h: f64, line_kind: &LineKind) -> String {
    if pts.is_empty() {
        return String::new();
    }
    let last_x = pts.last().unwrap().0;
    let first_x = pts[0].0;
    let line = build_line_path(pts, h, line_kind);
    format!("{line} L{last_x},{h} L{first_x},{h} Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_points_give_empty_path() {
        assert_eq!(build_fill_path(&[], 10.0, &LineKind::Straight), "");
    }

    #[test]
    fn fill_closes_along_the_baseline() {
        let pts = [(0.0, 2.0), (10.0, 4.0)];
        assert_eq!(
            build_fill_path(&pts, 10.0, &LineKind::Straight),
            "M0,8 L10,6 L10,10 L0,10 Z"
        );
    }

    #[test]
    fn top_edge_matches_line() {
        let pts = [(0.0, 2.0), (5.0, 9.0), (10.0, 4.0)];
        let line = build_line_path(&pts, 10.0, &LineKind::Rounded);
        assert!(build_fill_path(&pts, 10.0, &LineKind::Rounded).starts_with(&line));
    }

    #[test]
    fn each_instance_references_its_own_gradient() {
        use vertigo::Computed;
        use vertigo::dev::inspect::{DomDebugFragment, log_start};

        let data = GraphData {
            label: Computed::from(|_| "Chart".to_string()),
            scale_x: Computed::from(|_| 10.0),
            scale_y: Computed::from(|_| 10.0),
            points: Computed::from(|_| vec![(0.0, 2.0), (10.0, 4.0)]),
        };

        log_start();
        let _el = dom! {
            <g>
                <FillGradient {&data} kind={GradientKind::Linear("red", "blue")} line_kind={} />
                <FillGradient {&data} kind={GradientKind::Linear("red", "blue")} line_kind={} />
            </g>
        };

        let html = DomDebugFragment::from_log().to_pseudo_html();
        let ids: Vec<&str> = html
            .split("id='")
            .skip(1)
            .map(|rest| rest.split('\'').next().unwrap())
            .collect();
        assert_eq!(ids.len(), 2, "{html}");
        assert_ne!(ids[0], ids[1], "{html}");
        for id in ids {
            assert!(html.contains(&format!("fill='url(#{id})'")), "{html}");
        }
        assert!(html.contains("stop-color='red'"), "{html}");
        assert!(html.contains("stop-color='blue'"), "{html}");
    }
}
