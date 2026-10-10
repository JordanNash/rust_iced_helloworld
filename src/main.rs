//! Iced 0.13 feature tour, split into modules.
//!
//! Pattern: every tab is a self-contained module with its own state,
//! `Message` enum, `update` and `view`. This file wraps each tab's messages
//! in a parent `Message` variant and routes them with `Element::map` /
//! `Task::map`.
//! 

mod tabs;

use std::time::{Duration, Instant};

use iced::keyboard::{self, Key};
use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, pick_list, row, scrollable, text,
};
use iced::{time, Alignment, Element, Length, Size, Subscription, Task, Theme};

use tabs::animation::{self, Animation};
use tabs::async_task::{self, AsyncTab};
use tabs::widgets::{self, Widgets};

fn main() -> iced::Result {
    iced::application("Iced Feature Tour", App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .window_size(Size::new(760.0, 680.0))
        .centered()
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Widgets,
    Async,
    Animation,
}

impl Tab {
    const ALL: [Tab; 3] = [Tab::Widgets, Tab::Async, Tab::Animation];

    fn label(self) -> &'static str {
        match self {
            Tab::Widgets => "Widgets",
            Tab::Async => "Async",
            Tab::Animation => "Animation",
        }
    }
}

struct App {
    tab: Tab,
    theme: Theme,
    widgets: Widgets,
    async_tab: AsyncTab,
    animation: Animation,
}

impl Default for App {
    fn default() -> Self {
        App {
            tab: Tab::Widgets,
            theme: Theme::TokyoNight,
            widgets: Widgets::default(),
            async_tab: AsyncTab::default(),
            animation: Animation::default(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TabSelected(Tab),
    ThemeChanged(Theme),

    // Each tab's messages are wrapped so the parent can route them.
    Widgets(widgets::Message),
    Async(async_task::Message),
    Animation(animation::Message),

    // Global events fanned out to the tabs that care.
    Tick(Instant),
    KeyPressed(Key, keyboard::Modifiers),
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabSelected(tab) => self.tab = tab,
            Message::ThemeChanged(theme) => self.theme = theme,

            Message::Widgets(m) => self.widgets.update(m),
            Message::Async(m) => {
                // Task::map wraps the child's follow-up messages in ours.
                return self
                    .async_tab
                    .update(m, self.widgets.name())
                    .map(Message::Async);
            }
            Message::Animation(m) => self.animation.update(m),

            Message::Tick(now) => {
                self.async_tab.tick(now);
                self.animation.tick(now);
            }
            Message::KeyPressed(key, modifiers) => {
                self.animation.key_pressed(&key, modifiers);
                if self.tab == Tab::Animation && key == Key::Named(keyboard::key::Named::Space) {
                    self.animation.update(animation::Message::Toggle);
                }
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        // Only run the 60fps timer when something needs it.
        let animating =
            (self.tab == Tab::Animation && self.animation.is_running()) || self.async_tab.is_loading();

        let tick = if animating {
            time::every(Duration::from_millis(16)).map(Message::Tick)
        } else {
            Subscription::none()
        };

        let keys =
            keyboard::on_key_press(|key, modifiers| Some(Message::KeyPressed(key, modifiers)));

        Subscription::batch([tick, keys])
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn view(&self) -> Element<'_, Message> {
        let tabs = row(Tab::ALL.iter().map(|&tab| {
            button(text(tab.label()))
                .style(if tab == self.tab {
                    button::primary
                } else {
                    button::secondary
                })
                .on_press(Message::TabSelected(tab))
                .into()
        }))
        .spacing(8);

        let header = row![
            tabs,
            horizontal_space(),
            text("Theme:"),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        // Element::map turns the child's Message into ours.
        let body: Element<'_, Message> = match self.tab {
            Tab::Widgets => self.widgets.view().map(Message::Widgets),
            Tab::Async => self.async_tab.view().map(Message::Async),
            Tab::Animation => self.animation.view().map(Message::Animation),
        };

        container(
            column![header, horizontal_rule(1), scrollable(body).height(Length::Fill)].spacing(14),
        )
        .padding(20)
        .into()
    }
}
