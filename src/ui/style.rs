use iced::{button, container, Background, Color, Vector, Subscription, futures};
use std::time::Instant;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde_derive::*;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use colored::*;
use chrono;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializableColor { r: u8, g: u8, b: u8, a: f32 }

impl SerializableColor {
    fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        SerializableColor{r, b, g, a}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorList { map: HashMap<String, SerializableColor> }
impl ColorList {
    fn new() -> Self {
        ColorList{ map: HashMap::new() }
    }

    fn get(&self, lookup: &NamedColor) -> Color {
        match self.map.get(&lookup.0) {
            Some(color) => Color::from_rgba8(color.r, color.g, color.b, color.a),
            None => Color::from_rgb(1.0,0.0,1.0),
        }
    }

    fn is_valid_name(&self, lookup: NamedColor) -> bool {
        self.map.contains_key(&lookup.0)
    }

    fn add(&mut self, name: &str, r: u8, g: u8, b: u8, a: f32) -> Self {
        self.map.insert(name.to_string(), SerializableColor::from_rgba(r, g, b, a));
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiusList { map: HashMap<String, u16> }
impl RadiusList {
    fn new() -> Self {
        RadiusList{ map: HashMap::new() }
    }

    fn get(&self, lookup: &NamedRadius) -> u16 {
        match self.map.get(&lookup.0) {
            Some(radius) => *radius,
            None => 0,
        }
    }

    fn is_valid_name(&self, lookup: NamedRadius) -> bool {
        self.map.contains_key(&lookup.0)
    }

    fn add(&mut self, name: &str, radius: u16) -> Self {
        self.map.insert(name.to_string(), radius);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidthList { map: HashMap<String, u16> }

impl WidthList {
    fn new() -> Self {
        WidthList{ map: HashMap::new() }
    }

    fn get(&self, lookup: &NamedWidth) -> u16 {
        match self.map.get(&lookup.0) {
            Some(width) => *width,
            None => 0,
        }
    }

    fn is_valid_name(&self, lookup: NamedWidth) -> bool {
        self.map.contains_key(&lookup.0)
    }

    fn add(&mut self, name: &str, width: u16) -> Self {
        self.map.insert(name.to_string(), width);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSizeList { map: HashMap<String, u16> }

impl TextSizeList {
    fn new() -> Self {
        TextSizeList{ map: HashMap::new() }
    }

    fn get(&self, lookup: &NamedTextSize) -> u16 {
        match self.map.get(&lookup.0) {
            Some(text_size) => *text_size,
            None => 0,
        }
    }

    fn is_valid_name(&self, lookup: NamedTextSize) -> bool {
        self.map.contains_key(&lookup.0)
    }

    fn add(&mut self, name: &str, text_size: u16) -> Self {
        self.map.insert(name.to_string(), text_size);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedColor(String);

impl NamedColor {
    pub fn new(name: &str, colors: &ColorList) -> Self {
        let unvalidated = NamedColor(name.to_string());
        if colors.is_valid_name(unvalidated) {
            return NamedColor(name.to_string())
        } else {
            panic!("NamedColor {} failed validation", name);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedRadius(String);

impl NamedRadius {
    pub fn new(name: &str, radii: &RadiusList) -> Self {
        let unvalidated = NamedRadius(name.to_string());
        if radii.is_valid_name(unvalidated) {
            return NamedRadius(name.to_string())
        } else {
            panic!("NamedRadius {} failed validation", name);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedWidth(String);

impl NamedWidth {
    pub fn new(name: &str, widths: &WidthList) -> Self {
        let unvalidated = NamedWidth(name.to_string());
        if widths.is_valid_name(unvalidated) {
            return NamedWidth(name.to_string())
        } else {
            panic!("NamedWidth {} failed validation", name);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedTextSize(String);

impl NamedTextSize {
    pub fn new(name: &str, text_sizes: &TextSizeList) -> Self {
        let unvalidated = NamedTextSize(name.to_string());
        if text_sizes.is_valid_name(unvalidated) {
            return NamedTextSize(name.to_string())
        } else {
            panic!("NamedTextSize {} failed validation", name);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicContainer {
    text_color: NamedColor,
    background: NamedColor,
    border_color: NamedColor,
    border_radius: NamedRadius,
    border_width: NamedWidth
}

pub struct ContainerStyle {
    text_color: Option<Color>,
    background: Option<Background>,
    border_color: Color,
    border_radius: u16,
    border_width: u16,
}

impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> container::Style {
        iced::container::Style{
            text_color: self.text_color,
            background: self.background,
            border_color: self.border_color,
            border_radius: self.border_radius,
            border_width: self.border_width,
        }
    }
}

impl ContainerStyle {
    pub fn new(container: &DynamicContainer, stylesheet: &StyleSheet) -> Self {
        ContainerStyle{
            text_color: Some(stylesheet.color_classes.get(&container.text_color)),
            background: Some(Background::Color(stylesheet.color_classes.get(&container.background))),
            border_color: stylesheet.color_classes.get(&container.border_color),
            border_radius: stylesheet.radius_classes.get(&container.border_radius),
            border_width: stylesheet.width_classes.get(&container.border_width),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleSheet {
    
    //Project Label
    pub color_project_label: NamedColor,
    pub text_size_project_label: NamedTextSize,

    //Editable Label
    pub color_editable_label_label: NamedColor,
    pub text_size_editable_label_label: NamedTextSize,

    //Background Container
    pub container_background: DynamicContainer,

    color_classes: ColorList,
    radius_classes: RadiusList,
    width_classes: WidthList,
    text_size_classes: TextSizeList,
}

impl Default for StyleSheet {
    fn default() -> Self {
        // Define classes first so they can be referenced in the StyleSheet construction
        let color_classes = ColorList::new()
            .add("primary", 245, 245, 245, 1.0)
            .add("secondary", 245, 245, 245, 1.0)
            .add("background", 245, 245, 245, 1.0)
            .add("text_h1", 245, 245, 245, 1.0)
            .add("text_h2", 245, 245, 245, 1.0)
            .add("text_p", 245, 245, 245, 1.0)
        ;
        let radius_classes = RadiusList::new()
            .add("none", 0)
            .add("small", 2)
            .add("large", 5)
        ;
        let width_classes = WidthList::new()
            .add("none", 0)
            .add("thin", 1)
            .add("bold", 3)
        ;
        let text_size_classes = TextSizeList::new()
            .add("h1", 32)
            .add("h2", 24)
            .add("p", 12)
        ;
        // Construct a stylesheet, note that `Named___` objects use a class list for validatation
        StyleSheet{
            //Colors
            color_project_label: NamedColor::new("text_h1", &color_classes),
            color_editable_label_label: NamedColor::new("text_h1", &color_classes),
            //Text Sizes
            text_size_project_label: NamedTextSize::new("h1", &text_size_classes),
            text_size_editable_label_label: NamedTextSize::new("h1", &text_size_classes),
            // Containers
            container_background: DynamicContainer {
                text_color: NamedColor::new("text", &color_classes),
                background: NamedColor::new("background", &color_classes),
                border_color: NamedColor::new("none", &color_classes),
                border_radius: NamedRadius::new("none", &radius_classes),
                border_width: NamedWidth::new("none", &width_classes),
            },
            // Classes placed at end to avoid needing a .clone()
            color_classes,
            radius_classes,
            width_classes,
            text_size_classes,
        }
    }
}

impl StyleSheet {
    pub fn color(&self, name: &NamedColor) -> iced_native::Color {
        self.color_classes.get(name)
    }

    pub fn text_size(&self, name: &NamedTextSize) -> u16 {
        self.text_size_classes.get(name)
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