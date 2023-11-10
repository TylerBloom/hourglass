#![allow(unused, dead_code)]
use std::future::Future;

use creator::{CreatorMessage, TimerCreator};
use derive_more::From;
use instant::Duration;
use timer::{TimerMessage, TimerStack};
use yew::prelude::*;

mod creator;
mod input;
mod timer;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Viewer/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

pub struct Viewer {
    creator: TimerCreator,
    timers: TimerStack,
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum ViewMessage {
    Creator(CreatorMessage),
    Timer(TimerMessage),
    Tick,
}

impl Viewer {
    fn tick_msg(&self) -> impl 'static + Future<Output = ViewMessage> {
        async move {
            gloo_timers::future::sleep(Duration::from_millis(100)).await;
            ViewMessage::Tick
        }
    }
}

impl Component for Viewer {
    type Message = ViewMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let viewer = Viewer {
            creator: TimerCreator::new(),
            timers: TimerStack::new(),
        };
        ctx.link().send_future(viewer.tick_msg());
        viewer
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ViewMessage::Creator(msg) => self.creator.update(ctx, msg),
            ViewMessage::Timer(msg) => self.timers.update(ctx, msg),
            ViewMessage::Tick => {
                ctx.link().send_future(self.tick_msg());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { self.creator.view(ctx) }
                { self.timers.view(ctx) }
            </>
        }
    }
}
