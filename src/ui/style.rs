use iced::{button, container, Background, Color, Vector, Subscription, futures};
use std::time::Instant;
use std::path::{Path, PathBuf};
use serde_derive::*;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

#[derive(Debug, Clone)]
pub struct StyleSheet {
    colors: ColorSheet,
    last_update: Instant,
    color_primary: Color,
    padding_large: u16,
    radius_item: u16,
    radius_panel: u16,
    spacing_outer: u16,
    text_size_h1: u16,
    text_color_h1: Color,
    text_size_h2: u16,
}
#[derive(Debug, Clone, Copy)]
struct ColorSheet {
    primary: Color,
    secondary: Color,
    text: Color,
    background: Color,
}

impl Default for StyleSheet {
    fn default() -> Self {
        let colors = ColorSheet{
            primary: Color::from_rgb(0.95, 0.95, 0.95),
            secondary: Color::from_rgb(0.95, 0.95, 0.95),
            text: Color::from_rgb(0.95, 0.95, 0.95),
            background: Color::from_rgb(0.95, 0.95, 0.95),
        };

        StyleSheet{
            colors,
            last_update: Instant::now(),
            color_primary: colors.primary,
            padding_large: 10,
            radius_item: 5,
            radius_panel: 0,
            spacing_outer: 20,
            text_size_h1: 32,
            text_color_h1: colors.text,
            text_size_h2: 24,
        }
    }
}

struct StyleSheetSerialize {

}

enum FileState {
    Updated(StyleSheet),
    Stale,
}


fn watch(path: PathBuf) -> Result<notify::event::Event, Box<dyn std::error::Error>> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| tx.send(res).unwrap())?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    for result in rx {
        match result {
            Ok(result) => return Ok(result),
            Err(e) => return Err(Box::from(e)),
        }
    }

    Err(Box::from(std::io::Error::new(std::io::ErrorKind::Other, "No event returned from fn watch")))
}

pub fn check_style_file(path: PathBuf) -> iced::Subscription<Option<StyleSheet>> {
    iced::Subscription::from_recipe(StyleFile{file: path})
}

fn readfile() -> Option<StyleSheet> {
    //placeholder
    Some(StyleSheet::default())
}

struct StyleFile {
    pub file: PathBuf,
}

impl<H, I> iced_native::subscription::Recipe<H, I> for StyleFile
where
    H: std::hash::Hasher,
{
    type Output = Option<StyleSheet>;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.file.hash(state);
    }

    fn stream(self: Box<Self>,_input: futures::stream::BoxStream<'static, I>,) -> futures::stream::BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;

        async_std::stream::repeat_with(move || {
            match watch(self.file.clone()) {
                Ok(_) => readfile(),
                Err(_) => None,
            }
        }).boxed()
    }
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

