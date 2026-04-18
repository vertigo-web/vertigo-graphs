use std::rc::Rc;

use vertigo::{DomNode, css, dom, main, router::Router};
use vertigo_forms::{Tab, Tabs, TabsParams};

mod route;
mod tab_1_simple;
mod tab_2_animated;
mod tab_3_built;
mod tab_4_rounded;
mod tab_5_bar;
mod tab_6_fill_gradient;

use route::Route;

#[main]
fn app() -> DomNode {
    let current_tab = Router::<Route>::new_history_router();

    let tabs = vec![
        Tab {
            key: Route::Simple,
            name: "Simple".to_string(),
            render: Rc::new(|_: &Route| tab_1_simple::render()),
        },
        Tab {
            key: Route::Animated,
            name: "Animated".to_string(),
            render: Rc::new(|_: &Route| tab_2_animated::render()),
        },
        Tab {
            key: Route::Built,
            name: "Built".to_string(),
            render: Rc::new(|_: &Route| tab_3_built::render()),
        },
        Tab {
            key: Route::Rounded,
            name: "Rounded".to_string(),
            render: Rc::new(|_: &Route| tab_4_rounded::render()),
        },
        Tab {
            key: Route::Bar,
            name: "Bar".to_string(),
            render: Rc::new(|_: &Route| tab_5_bar::render()),
        },
        Tab {
            key: Route::FillGradient,
            name: "Fill Gradient".to_string(),
            render: Rc::new(|_: &Route| tab_6_fill_gradient::render()),
        },
    ];

    dom! {
        <html>
            <head />
            <body>
                <Tabs
                    {&current_tab}
                    {tabs}
                    params={bordered_tabs()}
                />
            </body>
        </html>
    }
}

pub fn bordered_tabs() -> TabsParams {
    TabsParams {
        header_item_add_css: css! {"
            border: 1px solid black;
            padding: 0px 10px;
        "},
        header_active_item_add_css: css! {"
            background-color: lightgray;
        "},
        content_css: css! {"
            border: solid 1px black;
            padding: 5px 10px;
        "},
        container_css: css! {"
            margin: 10px;
        "},
        ..Default::default()
    }
}
