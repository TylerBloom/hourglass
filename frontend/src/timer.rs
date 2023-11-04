use std::{collections::VecDeque, future::Future};

use instant::{Duration, Instant};
use yew::{html, Html, Context};

use crate::Viewer;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Timer {
    name: String,
    start: Instant,
    length: Duration,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TimerStack {
    timers: VecDeque<Timer>,
    rotation_started: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimerMessage {
    NewTimer(String, Duration),
    Rotate,
}

impl Timer {
    /// Constructor
    fn new(name: String, length: Duration) -> Self {
        Self {
            name,
            length,
            start: Instant::now(),
        }
    }

    /// The timer is the main focus
    fn major_view(&self) -> Html {
        html! { <h1>{format!("{}: {}", self.name, self.display_time_left())}</h1> }
    }

    /// The timer isn't the focus and is part of the larger stack
    fn minor_view(&self) -> Html {
        html! { <p>{format!("{}: {}", self.name, self.display_time_left())}</p> }
    }

    fn display_time_left(&self) -> String {
        let elapsed = self.start.elapsed();
        let (overtime, dur) = match self.length.checked_sub(elapsed) {
            Some(dur) => (false, dur),
            None => (true, elapsed - self.length),
        };
        let secs = dur.as_secs();
        let mins = secs / 60;
        let hours = secs / (60 * 60);
        if overtime {
            format!("-{:0>2}:{:0>2}:{:0>2}", hours, mins % 60, secs % 60)
        } else {
            format!("{:0>2}:{:0>2}:{:0>2}", hours, mins % 60, secs % 60)
        }
    }
}

impl TimerStack {
    pub fn new() -> Self {
        Self::default()
    }

    fn tick_dur(&self) -> Duration {
        Duration::from_secs(2)
    }

    fn rotate_tick(&self) -> impl 'static + Future<Output = TimerMessage> {
        let dur = self.tick_dur();
        async move {
            gloo_timers::future::sleep(dur).await;
            TimerMessage::Rotate
        }
    }

    pub fn update(&mut self, ctx: &Context<Viewer>, msg: TimerMessage) -> bool {
        match msg {
            TimerMessage::NewTimer(name, dur) => {
                if !self.rotation_started {
                    ctx.link().send_future(self.rotate_tick());
                    self.rotation_started = true;
                }
                let timer = Timer::new(name, dur);
                self.timers.push_back(timer);
            }
            TimerMessage::Rotate => {
                ctx.link().send_future(self.rotate_tick());
                self.timers.rotate_left(1);
            }
        }
        true
    }

    pub fn view(&self) -> Html {
        let mut iter = self.timers.iter();
        let Some(timer) = iter.next() else {
            return html!{ <h1> { "Add a timer above!!" } </h1> };
        };
        html! {
            <>
            { timer.major_view() }
            { for iter.map(|timer| timer.minor_view()) }
            </>
        }
    }
}
