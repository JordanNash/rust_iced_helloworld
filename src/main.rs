//! Iced 0.13 feature tour, split into modules.
//!
//! Pattern: every tab is a self-contained module with its own state,
//! `Message` enum, `update` and `view`. This file wraps each tab's messages
//! in a parent `Message` variant and routes them with `Element::map` /
//! `Task::map`.

mod tabs;

use std::time::{Duration, Instant};

use iced::keyboard::{self, Key};
use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, pick_list, row, scrollable, text,
};
use iced::{time, Alignment, Element, Length, Size, Subscription, Task, Theme};

use tabs::animation::{self, Animation};
use tabs::async_task::{self, AsyncTab};
use tabs::editing::{self, Editing};
use tabs::media::{self, Media};
use tabs::sysinfo::{self, SysInfo};
use tabs::widgets::{self, Widgets};

fn main() -> iced::Result {
    iced::application("Iced Feature Tour", App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .window_size(Size::new(1000.0, 780.0))
        .centered()
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Widgets,
    Async,
    Animation,
    Media,
    Text,
    System,
}

impl Tab {
    const ALL: [Tab; 6] = [
        Tab::Widgets,
        Tab::Async,
        Tab::Animation,
        Tab::Media,
        Tab::Text,
        Tab::System,
    ];

    fn label(self) -> &'static str {
        match self {
            Tab::Widgets => "Widgets",
            Tab::Async => "Async",
            Tab::Animation => "Animation",
            Tab::Media => "Media",
            Tab::Text => "Text",
            Tab::System => "System",
        }
    }
}

struct App {
    tab: Tab,
    theme: Theme,
    widgets: Widgets,
    async_tab: AsyncTab,
    animation: Animation,
    media: Media,
    editing: Editing,
    sysinfo: SysInfo,
}

impl Default for App {
    fn default() -> Self {
        App {
            tab: Tab::Widgets,
            theme: Theme::TokyoNight,
            widgets: Widgets::default(),
            async_tab: AsyncTab::default(),
            animation: Animation::default(),
            media: Media::default(),
            editing: Editing::default(),
            sysinfo: SysInfo::default(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TabSelected(Tab),
    ThemeChanged(Theme),
    /// Uses `Theme::default()`, which follows macOS light/dark mode
    /// (feature `auto-detect-theme`).
    MatchSystemTheme,

    // Each tab's messages are wrapped so the parent can route them.
    Widgets(widgets::Message),
    Async(async_task::Message),
    Animation(animation::Message),
    Media(media::Message),
    Editing(editing::Message),
    SysInfo(sysinfo::Message),

    // Global events fanned out to the tabs that care.
    Tick(Instant),
    KeyPressed(Key, keyboard::Modifiers),
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabSelected(tab) => {
                self.tab = tab;
                if tab == Tab::System {
                    return self.sysinfo.refresh().map(Message::SysInfo);
                }
            }
            Message::ThemeChanged(theme) => self.theme = theme,
            Message::MatchSystemTheme => self.theme = Theme::default(),

            Message::Widgets(m) => self.widgets.update(m),
            Message::Async(m) => {
                // Task::map wraps the child's follow-up messages in ours.
                return self
                    .async_tab
                    .update(m, self.widgets.name())
                    .map(Message::Async);
            }
            Message::Animation(m) => self.animation.update(m),
            Message::Media(m) => self.media.update(m),
            Message::Editing(m) => self.editing.update(m),
            Message::SysInfo(m) => return self.sysinfo.update(m).map(Message::SysInfo),

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
            button("Match macOS").on_press(Message::MatchSystemTheme),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        // Element::map turns the child's Message into ours.
        let body: Element<'_, Message> = match self.tab {
            Tab::Widgets => self.widgets.view().map(Message::Widgets),
            Tab::Async => self.async_tab.view().map(Message::Async),
            Tab::Animation => self.animation.view().map(Message::Animation),
            Tab::Media => self.media.view().map(Message::Media),
            Tab::Text => self.editing.view(&self.theme).map(Message::Editing),
            Tab::System => self.sysinfo.view().map(Message::SysInfo),
        };

        container(
            column![header, horizontal_rule(1), scrollable(body).height(Length::Fill)].spacing(14),
        )
        .padding(20)
        .into()
    }
}
