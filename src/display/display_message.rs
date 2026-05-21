use crate::utils::scale_to_resolution;
use macroquad::{miniquad::EventHandler, prelude::*};

#[derive(PartialEq, Clone, Copy)]
pub enum MessageType {
    Error,
    Info,
}

pub struct Message {
    pub text: String,
    pub message_type: MessageType,
    pub timer: f32,
}

impl Message {
    pub fn new(text: String, mess_type: MessageType) -> Self {
        Message {
            text,
            message_type: mess_type,
            timer: if mess_type == MessageType::Error {1.} else {2.},
        }
    }
    pub fn display_message(&self) {
        let rectangle_color = match self.message_type {
            MessageType::Error => Some(Color {
                r: (0.9),
                g: (0.16),
                b: (0.22),
                a: (0.8),
            }),
            MessageType::Info => Some(Color {
                r: (0.16),
                g: (0.9),
                b: (0.22),
                a: (0.8),
            }),
        };
        let font_size = scale_to_resolution(30.);
        let text_dimensions = measure_text(&self.text, None, font_size as u16, 0.8);
        let x = screen_width() * 0.99 - (text_dimensions.width * 1.5);
        let y = screen_height() * 0.05;
        draw_rectangle(
            x - text_dimensions.width * 0.05,
            y - text_dimensions.height * 1.5,
            text_dimensions.width * 1.4,
            text_dimensions.height * 2.,
            rectangle_color.unwrap_or(Color {
                r: (0.9),
                g: (0.16),
                b: (0.22),
                a: (0.8),
            }),
        );
        draw_text(&self.text, x, y, font_size as f32, BLACK);
    }
}
