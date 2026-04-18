use std::rc::Rc;
use vertigo::{Computed, DomNode, Value, bind, bind_spawn, dev::ValueMut, dom, get_driver};
use vertigo_graphs::{Graph, GraphData};

pub fn render() -> DomNode {
    let state = State::default();

    let data = GraphData {
        label: Computed::from(|_| "Animated Graph".to_string()),
        scale_x: Computed::from(|_| 500.0),
        scale_y: Computed::from(|_| 500.0),
        points: bind!(
            state,
            Computed::from(move |ctx| {
                let progress = (state.progress.get(ctx) as f64) / 500.0;
                let sine = progress.sin() * 100.0 * 5.0;
                let cosine = progress.cos() * 60.0 * 5.0;
                let tangent = progress.tan() * 80.0 * 5.0;
                let arc_sine = progress.asin() * 60.0 * 5.0;
                let arc_cosine = progress.acos() * 60.0 * 5.0;
                let arc_tangent = progress.atan() * 100.0 * 5.0;

                vec![
                    (0.0, sine),
                    (100.0, cosine),
                    (200.0, tangent),
                    (300.0, arc_sine),
                    (400.0, arc_cosine),
                    (500.0, arc_tangent),
                ]
            })
        ),
    };

    let on_click = bind_spawn!(state, |_| async move {
        state.start_animation().await;
    });

    dom! {
        <Graph data={data} />
        <button {on_click}>"Start"</button>
        <p>{state.progress}</p>
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

        for i in 0..500 {
            self.progress.set(i as u32);
            get_driver().sleep(2).await;
        }

        for i in (0..500).rev() {
            self.progress.set(i as u32);
            get_driver().sleep(1).await;
        }

        self.in_progress.set(false);
    }
}
