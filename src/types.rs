use vertigo::Computed;

/// Number of divisions used for axis ticks and grid lines.
pub const GRID_DIVISIONS: usize = 5;

/// Shared data driving all chart components.
///
/// - `label` — chart title shown above the SVG.
/// - `scale_x` / `scale_y` — total width and height of the data coordinate space.
/// - `points` — data series as `(x, y)` pairs in data coordinates (y=0 at bottom).
#[derive(Clone)]
pub struct GraphData {
    pub label: Computed<String>,
    pub scale_x: Computed<f64>,
    pub scale_y: Computed<f64>,
    pub points: Computed<Vec<(f64, f64)>>,
}
