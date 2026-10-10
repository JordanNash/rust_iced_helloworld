//! "System" tab: system information, lazy widgets, and the debug overlay.
//! Features used: `system`, `lazy`, `debug`.

use std::cell::Cell;

use iced::widget::{button, column, container, lazy, text, text_input};
use iced::{system, Element, Font, Task};

#[derive(Default)]
pub struct SysInfo {
    info: Option<system::Information>,
    note: String,
    // `Cell` lets `view(&self)` count calls without needing `&mut self`.
    view_calls: Cell<u32>,
    lazy_builds: Cell<u32>,
    lazy_key: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Fetch,
    Fetched(system::Information),
    NoteChanged(String),
    Invalidate,
}

impl SysInfo {
    /// Ask the runtime for system information; the answer arrives as `Fetched`.
    pub fn refresh(&self) -> Task<Message> {
        system::fetch_information().map(Message::Fetched)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Fetch => return self.refresh(),
            Message::Fetched(info) => self.info = Some(info),
            Message::NoteChanged(note) => self.note = note,
            Message::Invalidate => self.lazy_key += 1,
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        self.view_calls.set(self.view_calls.get() + 1);

        let info_text = match &self.info {
            // Dumping the whole struct shows the CPU, memory, graphics
            // backend (Metal on your Mac) and adapter (your GPU).
            Some(info) => format!("{info:#?}"),
            None => String::from("Querying..."),
        };

        column![
            text("System & performance").size(28),
            text("System information (feature `system`)"),
            button("Refresh").on_press(Message::Fetch),
            container(text(info_text).font(Font::MONOSPACE).size(14))
                .padding(12)
                .style(container::rounded_box),
            text("Lazy widgets (feature `lazy`)"),
            text(
                "Type below. view() runs on every keystroke, but the lazy block \
                 below only rebuilds when its key changes."
            ),
            text_input("Type here...", &self.note)
                .on_input(Message::NoteChanged)
                .padding(10),
            text(format!("view() has run {} times", self.view_calls.get())),
            lazy(self.lazy_key, |_| {
                let builds = self.lazy_builds.get() + 1;
                self.lazy_builds.set(builds);
                container(text(format!("Lazy block built {builds} time(s)")))
                    .padding(12)
                    .style(container::rounded_box)
            }),
            button("Invalidate lazy block").on_press(Message::Invalidate),
            text("Debug overlay (feature `debug`): press fn + F12 to toggle it."),
        ]
        .spacing(14)
        .into()
    }
}
