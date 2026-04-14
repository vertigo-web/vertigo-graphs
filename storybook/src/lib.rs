use std::rc::Rc;
use vertigo::{Computed, DomNode, Value, bind, bind_spawn, dev::ValueMut, dom, get_driver, main};
use vertigo_graphs::{Graph, GraphData};

#[main]
fn app() -> DomNode {
    let state = State::default();

    let data = GraphData {
        label: Computed::from(|_| "Graph".to_string()),
        scale_x: Computed::from(|_| 100.0),
        scale_y: Computed::from(|_| 100.0),
        points: bind!(
            state,
            Computed::from(move |ctx| {
                let progress = (state.progress.get(ctx) as f64) / 100.0;
                let sine = progress.sin() * 100.0;
                let cosine = progress.cos() * 60.0;
                let tangent = progress.tan() * 100.0;
                let arc_sine = progress.asin() * 60.0;
                let arc_cosine = progress.acos() * 60.0;
                let arc_tangent = progress.atan() * 100.0;

                vec![
                    (0.0, sine),
                    (20.0, cosine),
                    (40.0, tangent),
                    (60.0, arc_sine),
                    (80.0, arc_cosine),
                    (100.0, arc_tangent),
                ]
            })
        ),
    };

    let on_click = bind_spawn!(state, |_| async move {
        state.start_animation().await;
    });

    dom! {
        <html>
            <head />
            <body>
                <Graph data={data} />
                <button {on_click}>"Start"</button>
                <p>{state.progress}</p>
            </body>
        </html>
    }
}

#[derive(Clone, Default)]
pub struct State {
    pub progress: Value<u32>,
    in_progress: Rc<ValueMut<bool>>,
}

impl State {
    pub async fn start_animation(self) {
        if self.in_progress.get() {
            return;
        }

        self.in_progress.set(true);

        for i in 0..100 {
            self.progress.set(i as u32);
            get_driver().sleep(20).await;
        }

        for i in (0..100).rev() {
            self.progress.set(i as u32);
            get_driver().sleep(10).await;
        }

        self.in_progress.set(false);
    }
}
