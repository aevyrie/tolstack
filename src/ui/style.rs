use iced::{button, container, Background, Color, Vector, Subscription, futures};
use std::time::Instant;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde_derive::*;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use colored::*;
use chrono;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializableColor { r: f32, g: f32, b: f32 }

impl SerializableColor {
    fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        SerializableColor{r, b, g}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSheet { map: HashMap<String, SerializableColor> }

impl ColorSheet {
    fn new() -> Self {
        ColorSheet{ map: HashMap::new() }
    }

    fn get(&self, lookup: &ValidatedNamedColor) -> Color {
        match self.map.get(&lookup.0) {
            Some(color) => Color::from_rgb(color.r, color.g, color.b),
            None => Color::from_rgb(1.0,0.0,1.0),
        }
    }

    fn is_valid_color_name(&self, lookup: ValidatedNamedColor) -> bool {
        self.map.contains_key(&lookup.0)
    }

    fn add(&mut self, name: &str, r: f32, g: f32, b: f32) -> Self {
        self.map.insert(name.to_string(), SerializableColor::from_rgb(r, g, b));
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedNamedColor(String);
impl ValidatedNamedColor {
    pub fn new(name: &str, color_sheet: &ColorSheet) -> Self {
        let unvalidated_color = ValidatedNamedColor(name.to_string());
        if color_sheet.is_valid_color_name(unvalidated_color) {
            return ValidatedNamedColor(name.to_string())
        } else {
            panic!()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleSheet {
    colors: ColorSheet,
    pub color_primary: ValidatedNamedColor,
    pub padding_large: u16,
    pub radius_item: u16,
    pub radius_panel: u16,
    pub spacing_outer: u16,
    pub text_size_h1: u16,
    pub text_color_h1: ValidatedNamedColor,
    pub text_size_h2: u16,
}

impl Default for StyleSheet {
    fn default() -> Self {
        let colors = ColorSheet::new()
            .add("primary", 0.95, 0.95, 0.95)
            .add("secondary", 0.95, 0.95, 0.95)
            .add("background", 0.95, 0.95, 0.95)
            .add("text", 0.95, 0.95, 0.95);

        StyleSheet{
            colors: colors.clone(),
            color_primary: ValidatedNamedColor::new("primary", &colors),
            padding_large: 10,
            radius_item: 5,
            radius_panel: 0,
            spacing_outer: 20,
            text_size_h1: 32,
            text_color_h1: ValidatedNamedColor::new("text", &colors),
            text_size_h2: 24,
        }
    }
}


#[derive(Debug, Clone)]
pub enum LoadError {
    FileError,
    FormatError,
}

#[derive(Debug, Clone)]
pub enum SaveError {
    DirectoryError,
    FileError,
    WriteError,
    FormatError,
}

impl StyleSheet {
    pub fn color(&self, name: &ValidatedNamedColor) -> iced_native::Color {
        self.colors.get(name)
    }

    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories::ProjectDirs::from("rs", "", "TolStack")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or(std::path::PathBuf::new())
        };

        path.push("style.json");

        path
    }

    pub async fn load() -> Result<StyleSheet, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::FileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)
    }

    pub async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;
        let json = serde_json::to_string_pretty(&self)
            .map_err(|_| SaveError::FormatError)?;
        let path = Self::path();
        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::DirectoryError)?;
        }
        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::FileError)?;
            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::WriteError)?;
        }
        Ok(())
    }

    pub fn check_style_file(&self) -> iced::Subscription<bool> {
        iced::Subscription::from_recipe(self.clone())
    }
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

//fn readfile() -> Result<StyleSheet, LoadError> {
    //placeholder
//    println!("{}: {}", chrono::offset::Local::now(), "Style file update detected".green());
//}

struct StyleFile {
    pub file: PathBuf,
}

impl<H, I> iced_native::subscription::Recipe<H, I> for StyleSheet
where
    H: std::hash::Hasher,
{
    type Output = bool;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(self: Box<Self>,_input: futures::stream::BoxStream<'static, I>,) -> futures::stream::BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;

        async_std::stream::repeat_with(move || {
            let mut edit_found = false;
            loop {
                match watch(StyleSheet::path()) {
                    Ok(_) =>  return true,
                    Err(_) => {},
                }
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