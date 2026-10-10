//! "Widgets" tab: a form with common input widgets.

use iced::widget::{
    button, checkbox, column, progress_bar, radio, row, slider, text, text_input, toggler,
};
use iced::{Alignment, Element};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextSize {
    Small,
    Medium,
    Large,
}

impl TextSize {
    const ALL: [TextSize; 3] = [TextSize::Small, TextSize::Medium, TextSize::Large];

    fn label(self) -> &'static str {
        match self {
            TextSize::Small => "Small",
            TextSize::Medium => "Medium",
            TextSize::Large => "Large",
        }
    }

    fn points(self) -> f32 {
        match self {
            TextSize::Small => 8.0,
            TextSize::Medium => 16.0,
            TextSize::Large => 32.0,
        }
    }
}

pub struct Widgets {
    name: String,
    agree: bool,
    notifications: bool,
    volume: f32,
    size: TextSize,
}

impl Default for Widgets {
    fn default() -> Self {
        Widgets {
            name: String::new(),
            agree: false,
            notifications: true,
            volume: 40.0,
            size: TextSize::Medium,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    AgreeToggled(bool),
    NotificationsToggled(bool),
    VolumeChanged(f32),
    SizeSelected(TextSize),
}

impl Widgets {
    /// Other tabs can read (but not change) the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::NameChanged(name) => self.name = name,
            Message::AgreeToggled(v) => self.agree = v,
            Message::NotificationsToggled(v) => self.notifications = v,
            Message::VolumeChanged(v) => self.volume = v,
            Message::SizeSelected(s) => self.size = s,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let greeting = if self.name.trim().is_empty() {
            "Type your name above".to_string()
        } else {
            format!("Hello, {}!", self.name.trim())
        };

        let sizes = row(TextSize::ALL.iter().map(|&s| {
            radio(s.label(), s, Some(self.size), Message::SizeSelected).into()
        }))
        .spacing(16);

        column![
            text("Form widgets").size(28),
            text_input("Your name", &self.name)
                .on_input(Message::NameChanged)
                .padding(10),
            // on_press_maybe(None) renders the button disabled.
            button("Clear").on_press_maybe(if self.name.is_empty() {
                None
            } else {
                Some(Message::NameChanged(String::new()))
            }),
            row![
                checkbox("I agree to nothing in particular", self.agree)
                    .on_toggle(Message::AgreeToggled),
                toggler(self.notifications)
                    .label("Notifications")
                    .on_toggle(Message::NotificationsToggled),
            ]
            .spacing(24),
            text(if self.agree {
                "Thanks for agreeing."
            } else {
                "You have not agreed yet."
            })
            .style(if self.agree { text::success } else { text::danger }),
            row![
                text(format!("Volume: {:>3.0}", self.volume)),
                slider(0.0..=100.0, self.volume, Message::VolumeChanged),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            progress_bar(0.0..=100.0, self.volume),
            text("Text size:"),
            sizes,
            text(greeting).size(self.size.points()),
        ]
        .spacing(14)
        .into()
    }
}
