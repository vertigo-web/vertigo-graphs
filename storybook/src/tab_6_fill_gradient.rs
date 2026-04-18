use vertigo::{Computed, DomNode, dom};
use vertigo_graphs::{
    ChartContainer, FillGradient, GradientKind, GraphData, Grid, GridKind, Line, LineKind, XAxis,
    YAxis,
};

pub fn render() -> DomNode {
    let data = GraphData {
        label: Computed::from(|_| "Built from blocks".to_string()),
        scale_x: Computed::from(|_| 1000.0),
        scale_y: Computed::from(|_| 500.0),
        points: Computed::from(move |_| {
            vec![
                (0.0, 0.0),
                (25.0, 70.0),
                (50.0, 200.0),
                (75.0, 150.0),
                (100.0, 100.0),
                (125.0, 250.0),
                (150.0, 380.0),
                (175.0, 220.0),
                (200.0, 200.0),
                (225.0, 350.0),
                (250.0, 400.0),
                (275.0, 300.0),
                (300.0, 50.0),
                (325.0, 150.0),
                (350.0, 200.0),
                (375.0, 180.0),
                (400.0, 100.0),
                (425.0, 150.0),
                (450.0, 300.0),
                (475.0, 220.0),
                (500.0, 200.0),
                (525.0, 350.0),
                (550.0, 300.0),
                (575.0, 200.0),
                (600.0, 50.0),
                (625.0, 150.0),
                (650.0, 200.0),
                (675.0, 280.0),
                (700.0, 200.0),
                (725.0, 350.0),
                (750.0, 400.0),
                (775.0, 420.0),
                (800.0, 300.0),
                (825.0, 250.0),
                (850.0, 200.0),
                (875.0, 200.0),
                (900.0, 50.0),
                (925.0, 150.0),
                (950.0, 100.0),
                (975.0, 180.0),
                (1000.0, 100.0),
            ]
        }),
    };

    dom! {
        <ChartContainer {&data} padding_x={50} background="black">
            <Grid {&data} kind={GridKind::YOnly} stroke={} />
            <XAxis {&data} stroke="white" />
            <YAxis {&data} stroke="white" values={true} decimals={} value_scale={} labels_only={} />
            <Line {&data} color="rgba(24,243,243,1)" kind={LineKind::Rounded} />
            <FillGradient {&data}
                line_kind={LineKind::Rounded}
                kind={GradientKind::Linear("rgba(24,243,243,0.3);", "rgba(24,243,243,0.0);")}
            />
        </ChartContainer>
    }
}
