//! "Media" tab: a generated raster image, an inline SVG, and a QR code.
//! Features used: `image-without-codecs`, `svg`, `qr_code`.

use iced::widget::{column, image, qr_code, row, slider, svg, text, text_input};
use iced::{Alignment, Element, Length};

const IMG_SIZE: u32 = 96;

const LOGO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120">
  <rect x="4" y="4" width="112" height="112" rx="20" fill="#1f2335"/>
  <circle cx="60" cy="60" r="38" fill="none" stroke="#7aa2f7" stroke-width="8"/>
  <polygon points="60,32 84,76 36,76" fill="#9ece6a"/>
  <circle cx="60" cy="60" r="7" fill="#f7768e"/>
</svg>"##;

pub struct Media {
    hue: f32,
    image: image::Handle,
    svg: svg::Handle,
    qr_text: String,
    qr: Option<qr_code::Data>,
}

impl Default for Media {
    fn default() -> Self {
        let qr_text = String::from("https://www.rust-lang.org");
        Media {
            hue: 210.0,
            image: render_image(210.0),
            svg: svg::Handle::from_memory(LOGO_SVG.as_bytes()),
            qr: qr_code::Data::new(&qr_text).ok(),
            qr_text,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    HueChanged(f32),
    QrTextChanged(String),
}

impl Media {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::HueChanged(hue) => {
                self.hue = hue;
                // A new Handle means a new GPU texture; only do it on change,
                // never inside `view`.
                self.image = render_image(hue);
            }
            Message::QrTextChanged(value) => {
                self.qr = qr_code::Data::new(&value).ok();
                self.qr_text = value;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let qr: Element<'_, Message> = match &self.qr {
            Some(data) => qr_code(data).cell_size(8_u16).into(),
            None => text("That text can't be encoded as a QR code.")
                .style(text::danger)
                .into(),
        };

        column![
            text("Media").size(28),
            text("Image: pixels generated in code, so no image file or decoder is needed"),
            row![
                image(self.image.clone())
                    .width(Length::Fixed(192.0))
                    .height(Length::Fixed(192.0)),
                column![
                    text(format!("Hue: {:.0}", self.hue)),
                    slider(0.0..=300.0, self.hue, Message::HueChanged),
                ]
                .spacing(8)
                .width(Length::Fill),
            ]
            .spacing(16)
            .align_y(Alignment::Center),
            text("SVG: inline vector art, sharp at any size"),
            svg(self.svg.clone())
                .width(Length::Fixed(120.0))
                .height(Length::Fixed(120.0)),
            text("QR code: type to re-encode"),
            text_input("Text to encode", &self.qr_text)
                .on_input(Message::QrTextChanged)
                .padding(10),
            qr,
        ]
        .spacing(14)
        .into()
    }
}

/// A small smooth-gradient bitmap whose colors shift with `hue` (degrees).
fn render_image(hue: f32) -> image::Handle {
    let mut pixels = Vec::with_capacity((IMG_SIZE * IMG_SIZE * 4) as usize);
    let last = (IMG_SIZE - 1) as f32;

    for y in 0..IMG_SIZE {
        for x in 0..IMG_SIZE {
            let fx = x as f32 / last;
            let fy = y as f32 / last;
            let (r, g, b) = hsv_to_rgb(
                (hue + fx * 60.0) % 360.0,
                0.35 + 0.65 * fy,
                1.0 - 0.5 * fx * fy,
            );
            pixels.extend_from_slice(&[r, g, b, 255]);
        }
    }

    image::Handle::from_rgba(IMG_SIZE, IMG_SIZE, pixels)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let hp = h / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r, g, b) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    let to_byte = |f: f32| ((f + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    (to_byte(r), to_byte(g), to_byte(b))
}
