use vertigo::{Computed, DomNode, dom};
use vertigo_graphs::{ChartContainer, GraphData, Grid, Legend, Line, Tooltip, XAxis, YAxis};

pub fn render() -> DomNode {
    let data = GraphData {
        label: Computed::from(|_| "Built from blocks".to_string()),
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
            <Line {&data} params={} />
            <Legend {&data} />
            <Tooltip {&data} />
        </ChartContainer>
    }
}
