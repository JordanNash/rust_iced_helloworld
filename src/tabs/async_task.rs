//! "Async" tab: run a future in the background with `Task::perform`.

use std::time::{Duration, Instant};

use iced::widget::{button, column, progress_bar, text};
use iced::{Element, Task};

const LOAD_TIME: Duration = Duration::from_secs(2);

enum Status {
    Idle,
    Loading { started: Instant },
    Done(Result<String, String>),
}

pub struct AsyncTab {
    status: Status,
    now: Instant,
}

impl Default for AsyncTab {
    fn default() -> Self {
        AsyncTab {
            status: Status::Idle,
            now: Instant::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Loaded(Result<String, String>),
}

impl AsyncTab {
    /// The parent uses this to decide whether the timer subscription is needed.
    pub fn is_loading(&self) -> bool {
        matches!(self.status, Status::Loading { .. })
    }

    /// Called by the parent on every timer tick.
    pub fn tick(&mut self, now: Instant) {
        self.now = now;
    }

    /// `name` comes from the Widgets tab; the parent passes it in so the
    /// tabs don't have to know about each other.
    pub fn update(&mut self, message: Message, name: &str) -> Task<Message> {
        match message {
            Message::Start => {
                self.now = Instant::now();
                self.status = Status::Loading { started: self.now };
                return Task::perform(fetch_greeting(name.to_string()), Message::Loaded);
            }
            Message::Loaded(result) => self.status = Status::Done(result),
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let loading = self.is_loading();

        let status: Element<'_, Message> = match &self.status {
            Status::Idle => text("Nothing requested yet.").into(),
            Status::Loading { started } => {
                let elapsed = self.now.saturating_duration_since(*started);
                let pct = (elapsed.as_secs_f32() / LOAD_TIME.as_secs_f32()).min(1.0);
                column![
                    text("Working in the background (the UI stays responsive)..."),
                    progress_bar(0.0..=1.0, pct),
                ]
                .spacing(8)
                .into()
            }
            Status::Done(Ok(msg)) => text(msg.clone()).style(text::success).into(),
            Status::Done(Err(err)) => text(err.clone()).style(text::danger).into(),
        };

        column![
            text("Async tasks").size(28),
            text(
                "Task::perform runs a future off the UI thread and turns its \
                 result into a Message. The greeting uses the name from the Widgets tab."
            ),
            button(if loading { "Loading..." } else { "Fetch greeting" })
                .on_press_maybe((!loading).then_some(Message::Start)),
            status,
        ]
        .spacing(14)
        .into()
    }
}

async fn fetch_greeting(name: String) -> Result<String, String> {
    tokio::time::sleep(LOAD_TIME).await;
    if name.trim().is_empty() {
        Err("Enter a name on the Widgets tab first!".to_string())
    } else {
        Ok(format!("Hello, {}! (delivered asynchronously)", name.trim()))
    }
}
