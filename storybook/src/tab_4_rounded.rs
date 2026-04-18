use vertigo::{Computed, DomNode, dom};
use vertigo_graphs::{ChartContainer, GraphData, Grid, Line, LineKind, XAxis, YAxis};

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
        <ChartContainer {&data} padding_x={} background={}>
            <Grid {&data} kind={} stroke={} />
            <XAxis {&data} stroke={} />
            <YAxis {&data} stroke={} values={} decimals={} value_scale={} labels_only={} />
            <Line {&data} color={} kind={LineKind::Rounded} />
        </ChartContainer>
    }
}
