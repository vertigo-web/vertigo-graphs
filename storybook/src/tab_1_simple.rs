use vertigo::{Computed, DomNode, dom};
use vertigo_graphs::{Graph, GraphData};

pub fn render() -> DomNode {
    let data = GraphData {
        label: Computed::from(|_| "Simple Graph".to_string()),
        scale_x: Computed::from(|_| 500.0),
        scale_y: Computed::from(|_| 500.0),
        points: Computed::from(move |_| {
            vec![
                (0.0, 0.0),
                (100.0, 200.0),
                (200.0, 100.0),
                (300.0, 300.0),
                (400.0, 200.0),
                (500.0, 400.0),
            ]
        }),
    };

    dom! {
        <Graph data={data} />
    }
}
