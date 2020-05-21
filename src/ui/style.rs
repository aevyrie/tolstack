use iced::{button, container, Background, Color, Vector};
use std::collections::HashMap;
use serde_derive::*;
use NamedColors::*;
use StyleSheetColor::*;
use StyleClass::*;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
enum StyleClass {
    SubmitButton,
    CancelButton,
    CenterX,
    CenterY,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
enum NamedColors {
    BackgroundLight,
    BackgroundDark,
}

#[derive(Serialize, Deserialize)]
struct ApplicationStyleSheet {
    colors: HashMap<NamedColors, StyleSheetColor>
}
impl Default for ApplicationStyleSheet {
    fn default() -> Self {
        let colors: HashMap<NamedColors, StyleSheetColor> = [
            (BackgroundLight, Hex(String::from("FAFAFA"))),
        ].iter().cloned().collect();

        ApplicationStyleSheet {
            colors,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
enum StyleSheetColor {
    Hex(String),
    RGB(f32, f32, f32),
}

impl From<StyleSheetColor> for Color {
    fn from(item: StyleSheetColor) -> Self {
        match item {
            StyleSheetColor::Hex(hex) => {
                if hex.len() == 6 {
                    let mut rgb: [f32;3] = [1.0, 0.0, 1.0];
                    for i in rgb.iter_mut().enumerate() {
                        let hex_u8:u8 = match u8::from_str_radix(&hex[i.0*2..i.0*2+2], 16) {
                            Ok(result) => result,
                            Err(_) => {
                                println!("Unable to parse hex string as u8");
                                255
                            }
                        };
                        *i.1 = f32::from(hex_u8)/255.0;
                    }
                    Color::from_rgb(rgb[0], rgb[1], rgb[2])
                } else {
                    println!("Invalid hex color string length");
                    Color::from_rgb(1.0, 0.0, 1.0)
                }
            }
            StyleSheetColor::RGB(r, g, b) => {
                Color::from_rgb(r, g, b)
            }
        }
    }
}

trait Appear {
    fn appear(self, stylesheet: &ApplicationStyleSheet, classes: &[StyleClass]) -> Self;
}

pub enum Button {
    Filter { selected: bool },
    Choice { selected: bool },
    Icon,
    Destructive,
    Constructive,
    Neutral,
}
impl button::StyleSheet for Button {
fn active(&self) -> button::Style {
    match self {
        Button::Filter { selected } => {
            if *selected {
                button::Style {
                    background: Some(Background::Color(
                        Color::from_rgb(0.95, 0.95, 0.95),
                    )),
                    border_radius: 5,
                    text_color: Color::BLACK,
                    ..button::Style::default()
                }
            } else {
                button::Style::default()
            }
        }
        Button::Choice { selected } => {
            if *selected {
                button::Style {
                    background: Some(Background::Color(
                        Color::from_rgb(0.2, 0.4, 0.7),
                    )),
                    border_radius: 5,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                }
            } else {
                button::Style::default()
            }
        }
        Button::Icon => button::Style {
            text_color: Color::from_rgb(0.5, 0.5, 0.5),
            ..button::Style::default()
        },
        Button::Destructive => button::Style {
            background: Some(Background::Color(Color::from_rgb(
                0.8, 0.2, 0.2,
            ))),
            border_radius: 5,
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 1.0),
            ..button::Style::default()
        },
        Button::Constructive => button::Style {
            background: Some(Background::Color(Color::from_rgb(
                0.2, 0.8, 0.2,
            ))),
            border_radius: 5,
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 1.0),
            ..button::Style::default()
        },
        Button::Neutral => button::Style {
            background: Some(Background::Color(Color::from_rgb(
                0.8, 0.8, 0.8,
            ))),
            border_radius: 5,
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 1.0),
            ..button::Style::default()
        },
    }
}

fn hovered(&self) -> button::Style {
    let active = self.active();

    button::Style {
        text_color: match self {
            Button::Icon => Color::from_rgb(0.2, 0.2, 0.7),
            Button::Filter { selected } if !selected => {
                Color::from_rgb(0.5, 0.5, 0.5)
            }
            Button::Filter { selected } if !selected => {
                Color::from_rgb(0.3, 0.5, 0.8)
            }
            _ => active.text_color,
        },
        shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
        ..active
    }
}
}

pub enum Container {
Entry,
Background,
}
impl container::StyleSheet for Container {
fn style(&self) -> container::Style {
    match self {
        Container::Entry => container::Style {
            text_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
            background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
            border_radius: 5,
            border_width: 1,
            border_color: Color::from_rgb(0.9, 0.9, 0.9),
        },
        Container::Background => container::Style {
            text_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
            background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
            border_radius: 5,
            border_width: 1,
            border_color: Color::from_rgb(0.9, 0.9, 0.9),
        },
    }
}
}

