# Vertigo Graphs

Blocks for building graphs in [vertigo](https://crates.io/crates/vertigo).

[![crates.io](https://img.shields.io/crates/v/vertigo-graphs)](https://crates.io/crates/vertigo-graphs)
[![Documentation](https://docs.rs/vertigo-graphs/badge.svg)](https://docs.rs/vertigo-graphs)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/vertigo-graphs.svg)
[![Dependency Status](https://deps.rs/crate/vertigo-graphs/0.1.3/status.svg)](https://deps.rs/crate/vertigo-graphs/0.1.3)
[![CI](https://github.com/vertigo-web/vertigo-graphs/actions/workflows/pipeline.yaml/badge.svg)](https://github.com/vertigo-web/vertigo-graphs/actions/workflows/pipeline.yaml)
[![downloads](https://img.shields.io/crates/d/vertigo-graphs.svg)](https://crates.io/crates/vertigo-graphs)

See [Changelog](https://github.com/vertigo-web/vertigo-graphs/blob/master/CHANGES.md) for recent features.

## Example

Dependencies:

```toml
vertigo = "0.11"
vertigo-graphs = "0.1"
```

Example:

```rust
use vertigo::{Computed, DomNode, dom, main};
use vertigo_graphs::{Graph, GraphData};

#[main]
fn app() -> DomNode {
    let data = GraphData {
        label: Computed::from(|_| "Graph".to_string()),
        scale_x: Computed::from(|_| 100.0),
        scale_y: Computed::from(|_| 100.0),
        points: Computed::from(|_| {
            vec![
                (0.0, 0.0),
                (20.0, 30.0),
                (40.0, 10.0),
                (60.0, 50.0),
                (80.0, 30.0),
                (100.0, 70.0),
            ]
        }),
    };

    dom! {
        <html>
            <head />
            <body>
                <Graph data={data} />
            </body>
        </html>
    }
}
```

## Storybook App

### Prepare

Install vertigo-cli:

* `cargo install vertigo-cli`

### Run

Build and run storybook in watch mode:

* `vertigo watch vertigo-graphs-storybook`

Eventually terminal will let you know that app is available under `http://localhost:4444/`

If you want to play around with the code, the browser will automatically refresh after the project has been recompiled.
