use vertigo::{Computed, DomNode, dom};
use vertigo_graphs::{ChartContainer, GraphData, Grid, Line, LineKind, LineParams, XAxis, YAxis};

pub fn render() -> DomNode {
    let data = GraphData {
        label: Computed::from(|_| "Rounded Graph".to_string()),
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
        <ChartContainer {&data} params={}>
            <Grid {&data} params={} />
            <XAxis {&data} params={} />
            <YAxis {&data} params={} />
            <Line {&data} params={LineParams { kind: LineKind::Rounded, ..Default::default() }} />
        </ChartContainer>
    }
}
