use vertigo::{Computed, DomNode, dom};
use vertigo_graphs::{Bar, BarKind, ChartContainer, GraphData, Grid, XAxis, YAxis};

pub fn render() -> DomNode {
    let data = GraphData {
        label: Computed::from(|_| "Bar Graph".to_string()),
        scale_x: Computed::from(|_| 600.0),
        scale_y: Computed::from(|_| 500.0),
        points: Computed::from(move |_| {
            vec![
                (50.0, 50.0),
                (150.0, 200.0),
                (250.0, 100.0),
                (350.0, 300.0),
                (450.0, 200.0),
                (550.0, 400.0),
            ]
        }),
    };

    dom! {
        <ChartContainer {&data} padding_x={} background={}>
            <Grid {&data} kind={} stroke={} />
            <XAxis {&data} stroke={} />
            <YAxis {&data} stroke={} values={} decimals={} value_scale={} labels_only={} />
            <Bar {&data} kind={BarKind::Rounded(15.0)} />
        </ChartContainer>
    }
}
