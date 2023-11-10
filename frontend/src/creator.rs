use std::borrow::Cow;

use derive_more::From;
use instant::Duration;
use yew::{html, Context, Html};

use crate::{input::TextInput, timer::TimerMessage, ViewMessage, Viewer};

pub struct TimerCreator {
    name: String,
    duration: Option<Duration>,
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum CreatorMessage {
    NameInput(String),
    DurationInput(Option<Duration>),
    Submit,
}

impl TimerCreator {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            duration: None,
        }
    }

    pub fn update(&mut self, ctx: &Context<Viewer>, msg: CreatorMessage) -> bool {
        match msg {
            CreatorMessage::NameInput(name) => {
                self.name = name;
            }
            CreatorMessage::DurationInput(dur) => {
                self.duration = dur;
            }
            CreatorMessage::Submit => {
                if let Some(dur) = self.duration {
                    ctx.link()
                        .send_message(TimerMessage::NewTimer(self.name.clone(), dur));
                }
            }
        }
        false
    }

    pub fn view(&self, ctx: &Context<Viewer>) -> Html {
        let name = ctx
            .link()
            .callback(move |s: String| ViewMessage::Creator(s.into()));
        let dur = ctx.link().callback(move |s: String| {
            ViewMessage::Creator(
                s.parse()
                    .ok()
                    .map(|num: u64| Duration::from_secs(num * 60))
                    .into(),
            )
        });
        let submit = ctx.link().callback(|_| CreatorMessage::Submit);
        html! {
            <>
                <TextInput label = { Cow::from("Timer name:") } process = { name }/>
                <TextInput label = { Cow::from("Length (in minutes):") } process = { dur }/>
                <button onclick = { submit }> { "Create timer" } </button>
            </>
        }
    }
}
