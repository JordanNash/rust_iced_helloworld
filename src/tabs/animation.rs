//! "Animation" tab: a canvas driven by a timer subscription.

use std::time::{Duration, Instant};

use iced::keyboard;
use iced::mouse;
use iced::widget::canvas::{self, Canvas, Geometry, Path};
use iced::widget::{button, column, row, slider, text};
use iced::{Alignment, Element, Length, Point, Rectangle, Renderer, Theme};

pub struct Animation {
    running: bool,
    angle: f32,
    speed: f32,
    last_tick: Instant,
    last_key: String,
}

impl Default for Animation {
    fn default() -> Self {
        Animation {
            running: true,
            angle: 0.0,
            speed: 1.5,
            last_tick: Instant::now(),
            last_key: String::from("(none yet)"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Toggle,
    SpeedChanged(f32),
}

impl Animation {
    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Toggle => {
                self.running = !self.running;
                self.last_tick = Instant::now();
            }
            Message::SpeedChanged(v) => self.speed = v,
        }
    }

    /// Called by the parent on every timer tick.
    pub fn tick(&mut self, now: Instant) {
        // Clamp so a long gap between subscriptions can't cause a jump.
        let dt = now
            .saturating_duration_since(self.last_tick)
            .min(Duration::from_millis(100));
        self.last_tick = now;
        if self.running {
            self.angle += self.speed * dt.as_secs_f32();
        }
    }

    /// Called by the parent for every key press (just records it for display).
    pub fn key_pressed(&mut self, key: &keyboard::Key, modifiers: keyboard::Modifiers) {
        self.last_key = if modifiers.is_empty() {
            format!("{key:?}")
        } else {
            format!("{modifiers:?} + {key:?}")
        };
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            text("Canvas + subscriptions").size(24), // was 28
            Canvas::new(Orbit { angle: self.angle })
                .width(Length::Fill)
                .height(Length::Fixed(300.0)),
            row![
                button(if self.running { " Pause " } else { "Resume" }).on_press(Message::Toggle),
                text(format!("Speed: {:.1}", self.speed)),
                slider(0.0..=6.0, self.speed, Message::SpeedChanged).step(0.1_f32),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            text("Tip: press Space to pause/resume. Any key press is captured by a subscription."),
            text("Jordan wrote this."), // Jordan added this
            text(format!("Last key: {}", self.last_key)),
        ]
        .spacing(14)
        .into()
    }
}

// --------------------------------------------------------------- canvas ---

struct Orbit {
    angle: f32,
}

impl canvas::Program<Message> for Orbit {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let palette = theme.extended_palette();

        let center = frame.center();
        let radius = bounds.width.min(bounds.height) / 2.0 - 40.0;

        // Orbit path
        frame.stroke(
            &Path::circle(center, radius),
            canvas::Stroke::default()
                .with_color(palette.background.strong.color)
                .with_width(2.0),
        );
        // Sun
        frame.fill(&Path::circle(center, 18.0), palette.primary.strong.color);

        // Planet
        let planet = Point::new(
            center.x + radius * self.angle.cos(),
            center.y + radius * self.angle.sin(),
        );
        frame.fill(&Path::circle(planet, 14.0), palette.success.strong.color);

        // Moon
        let moon = Point::new(
            planet.x + 30.0 * (self.angle * 4.0).cos(),
            planet.y + 30.0 * (self.angle * 4.0).sin(),
        );
        frame.fill(&Path::circle(moon, 6.0), palette.danger.strong.color);

        vec![frame.into_geometry()]
    }
}
