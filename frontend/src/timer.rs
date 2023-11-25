use std::{collections::VecDeque, future::Future};

use instant::{Duration, Instant};
use yew::{html, Context, Html};

use crate::Viewer;

pub static RECORD_MSG: &str = "Please report your match BEFORE leaving your table";

#[derive(Debug, Clone, PartialEq, Eq)]
struct Timer {
    name: String,
    start: Instant,
    length: Duration,
    id: usize,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TimerStack {
    timers: VecDeque<Timer>,
    rotation_started: bool,
    /// A counter used to assign unique ids to each timer
    counter: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimerMessage {
    NewTimer(String, Duration),
    DeleteTimer(usize),
    Rotate,
}

impl Timer {
    /// Constructor
    fn new(name: String, length: Duration, id: usize) -> Self {
        Self {
            name,
            length,
            id,
            start: Instant::now(),
        }
    }

    /// The timer is the main focus
    fn major_view(&self, ctx: &Context<Viewer>) -> Html {
        let id = self.id;
        let cb = ctx.link().callback(move |_| TimerMessage::DeleteTimer(id));
        html! {
            <tr>
                <td class = "major-inner">
                    <h1 style={ self.text_color() }> { format!("{}: {}", self.name, self.display_time_left()) } </h1>
                    <p> { RECORD_MSG } </p>
                </td>
                <td class = "closer"> <button onclick = {cb}> { "X" } </button> </td>
            </tr>
        }
    }

    /// The timer isn't the focus and is part of the larger stack
    fn minor_view(&self, ctx: &Context<Viewer>) -> Html {
        let id = self.id;
        let cb = ctx.link().callback(move |_| TimerMessage::DeleteTimer(id));
        html! {
            <tr>
                <td class = "minor-inner">
                <h3 style={ self.text_color() }> { format!("{}: {}", self.name, self.display_time_left()) } </h3>
                <p> { RECORD_MSG } </p>
                </td>
                <td class = "closer"> <button onclick = {cb}> { "X" } </button> </td>
            </tr>
        }
    }

    fn text_color(&self) -> &'static str {
        match self.seconds_left() {
            ..=60 => "color:red;",
            61..=300 => "color:orange;",
            _ => "color:black;",
        }
    }

    fn seconds_left(&self) -> i64 {
        let elapsed = self.start.elapsed();
        let (overtime, dur) = match self.length.checked_sub(elapsed) {
            Some(dur) => (false, dur),
            None => (true, elapsed - self.length),
        };
        let secs = dur.as_secs() as i64;
        if overtime {
            -secs
        } else {
            secs
        }
    }

    fn display_time_left(&self) -> String {
        let secs = self.seconds_left();
        let overtime = secs < 0;
        let secs = secs.abs();
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
        match self.timers.front().map(|t| t.seconds_left()) {
            Some(..=60) => Duration::from_secs(20),
            Some(61..=300) => Duration::from_secs(15),
            None | Some(_) => Duration::from_secs(10),
        }
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
                let timer = Timer::new(name, dur, self.counter);
                self.counter += 1;
                self.timers.push_back(timer);
            }
            TimerMessage::Rotate => {
                ctx.link().send_future(self.rotate_tick());
                if !self.timers.is_empty() {
                    self.timers.rotate_right(1);
                }
            }
            TimerMessage::DeleteTimer(id) => {
                self.timers.retain(|t| t.id != id);
            }
        }
        true
    }

    pub fn view(&self, ctx: &Context<Viewer>) -> Html {
        let mut iter = self.timers.iter();
        let Some(timer) = iter.next() else {
            return html! { <h1> { "Add a timer above!!" } </h1> };
        };
        html! {
            <>
            <table>
            <tr>
            <td class = "major">
                { timer.major_view(ctx) }
            </td>
            </tr>
            </table>
            <p> { "" } </p>
            <table>
            {
                for iter.map(|timer| wrapped_minor_row(timer.minor_view(ctx)))
            }
            </table>
            </>
        }
    }
}

fn wrapped_minor_row(inner: Html) -> Html {
    html! {
        <tr>
            <td class = "spacer"> { "" } </td>
            <td class = "minor"> { inner } </td>
            <td class = "spacer"> { "" } </td>
        </tr>
    }
}
