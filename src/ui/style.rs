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
pub struct SerializableColor { r: u8, g: u8, b: u8, a: f32 }

impl SerializableColor {
    fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        SerializableColor{r, b, g, a}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedColor(String);
impl Named for NamedColor {
    type List = ColorList;
    type NamedItem = NamedColor;
    fn new_unvalidated(name: &str) -> Self { NamedColor(name.to_string()) }
    fn name(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedRadius(String);
impl Named for NamedRadius {
    type List = RadiusList;
    type NamedItem = NamedRadius;
    fn new_unvalidated(name: &str) -> Self { NamedRadius(name.to_string()) }
    fn name(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedWidth(String);
impl Named for NamedWidth {
    type List = WidthList;
    type NamedItem = NamedWidth;
    fn new_unvalidated(name: &str) -> Self { NamedWidth(name.to_string()) }
    fn name(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedTextSize(String);
impl Named for NamedTextSize {
    type List = TextSizeList;
    type NamedItem = NamedTextSize;
    fn new_unvalidated(name: &str) -> Self { NamedTextSize(name.to_string()) }
    fn name(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedPadding(String);
impl Named for NamedPadding {
    type List = PaddingList;
    type NamedItem = NamedPadding;
    fn new_unvalidated(name: &str) -> Self { NamedPadding(name.to_string()) }
    fn name(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedSpacing(String);
impl Named for NamedSpacing {
    type List = SpacingList;
    type NamedItem = NamedSpacing;
    fn new_unvalidated(name: &str) -> Self { NamedSpacing(name.to_string()) }
    fn name(&self) -> &str { &self.0 }
}

pub trait Named {
    type List: NamedList;
    type NamedItem: Named;

    fn new(name: &str, list: &Self::List) -> Self::NamedItem {
        let unvalidated = Self::NamedItem::new_unvalidated(name);
        if list.is_valid_name(&unvalidated) {
            return unvalidated;
        } else {
            panic!("{:?} '{}' failed validation", std::any::type_name::<Self>(), name);
        }
    }
    fn new_unvalidated(name: &str) -> Self;
    fn name(&self) -> &str;
}

pub trait NamedList {
    type NamedItem: Named;
    type Value;
    type Stored;
    fn new() -> Self;
    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value;
    fn is_valid_name<T: Named>(&self, lookup: &T) -> bool;
    fn add(&mut self, name: &str, value: Self::Stored) -> Self;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorList { map: HashMap<String, SerializableColor> }
impl NamedList for ColorList {
    type NamedItem = NamedColor;
    type Value = Color;
    type Stored = SerializableColor;

    fn new() -> Self {
        ColorList{ map: HashMap::new() }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(color) => Color::from_rgba8(color.r, color.g, color.b, color.a),
            None => Color::from_rgb(1.0,0.0,1.0),
        }
    }

    fn is_valid_name<T:Named>(&self, lookup: &T) -> bool {
        self.map.contains_key(lookup.name())
    }

    fn add(&mut self, name: &str, value: Self::Stored) -> Self {
        self.map.insert(name.to_string(), value);
        self.clone()

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiusList { map: HashMap<String, u16> }
impl NamedList for RadiusList {
    type NamedItem = NamedRadius;
    type Value = u16;
    type Stored = u16;

    fn new() -> Self {
        RadiusList{ map: HashMap::new() }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(radius) => *radius,
            None => 0,
        }
    }

    fn is_valid_name<T:Named>(&self, lookup: &T) -> bool {
        self.map.contains_key(lookup.name())
    }

    fn add(&mut self, name: &str, value: Self::Stored) -> Self {
        self.map.insert(name.to_string(), value);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidthList { map: HashMap<String, u16> }
impl NamedList for WidthList {
    type NamedItem = NamedWidth;
    type Value = u16;
    type Stored = u16;

    fn new() -> Self {
        WidthList{ map: HashMap::new() }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(width) => *width,
            None => 0,
        }
    }

    fn is_valid_name<T:Named>(&self, lookup: &T) -> bool {
        self.map.contains_key(lookup.name())
    }

    fn add(&mut self, name: &str, value: Self::Stored) -> Self {
        self.map.insert(name.to_string(), value);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSizeList { map: HashMap<String, u16> }
impl NamedList for TextSizeList {
    type NamedItem = NamedTextSize;
    type Value = u16;
    type Stored = u16;

    fn new() -> Self {
        TextSizeList{ map: HashMap::new() }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(text_size) => *text_size,
            None => 0,
        }
    }

    fn is_valid_name<T:Named>(&self, lookup: &T) -> bool {
        self.map.contains_key(lookup.name())
    }

    fn add(&mut self, name: &str, value: Self::Stored) -> Self {
        self.map.insert(name.to_string(), value);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaddingList { map: HashMap<String, u16> }
impl NamedList for PaddingList {
    type NamedItem = NamedPadding;
    type Value = u16;
    type Stored = u16;

    fn new() -> Self {
        Self{ map: HashMap::new() }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(lookup) => *lookup,
            None => 0,
        }
    }

    fn is_valid_name<T:Named>(&self, lookup: &T) -> bool {
        self.map.contains_key(lookup.name())
    }

    fn add(&mut self, name: &str, value: Self::Stored) -> Self {
        self.map.insert(name.to_string(), value);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingList { map: HashMap<String, u16> }
impl NamedList for SpacingList {
    type NamedItem = NamedSpacing;
    type Value = u16;
    type Stored = u16;

    fn new() -> Self {
        Self{ map: HashMap::new() }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(lookup) => *lookup,
            None => 0,
        }
    }

    fn is_valid_name<T:Named>(&self, lookup: &T) -> bool {
        self.map.contains_key(lookup.name())
    }

    fn add(&mut self, name: &str, value: Self::Stored) -> Self {
        self.map.insert(name.to_string(), value);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyledContainer {
    text_color: NamedColor,
    background: NamedColor,
    border_color: NamedColor,
    border_radius: NamedRadius,
    border_width: NamedWidth
}

pub struct IcedContainerStyle {
    text_color: Option<Color>,
    background: Option<Background>,
    border_color: Color,
    border_radius: u16,
    border_width: u16,
}
impl container::StyleSheet for IcedContainerStyle {
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
impl IcedContainerStyle {
    pub fn new(container: &StyledContainer, iss: &IcedStyleSheet) -> Self {
        IcedContainerStyle{
            text_color: Some(iss.color.resolve(&container.text_color)),
            background: Some(Background::Color(iss.color.resolve(&container.background))),
            border_color: iss.color.resolve(&container.border_color),
            border_radius: iss.radius.resolve(&container.border_radius),
            border_width: iss.width.resolve(&container.border_width),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcedStyleSheet {
    
    //Project Label
    pub project_label_color: NamedColor,
    pub project_label_text_size: NamedTextSize,
    pub project_label_spacing: NamedSpacing,

    //Editable Label
    pub editablelabel_label_color: NamedColor,
    pub editablelabel_label_text_size: NamedTextSize,

    //Header
    pub header_spacing: NamedSpacing,
    pub header_button_spacing: NamedSpacing,

    //Background Container
    pub home_container: StyledContainer,
    pub home_padding: NamedPadding,   

    //area_mc_analysis
    pub mc_results_row_spacing: NamedSpacing,
    pub mc_results_col_spacing: NamedSpacing,

    //area_stack_editor
    pub editor_tol_spacing: NamedSpacing,
    pub editor_content_spacing: NamedSpacing,
    pub editor_title_text_size: NamedTextSize,
    pub editor_scroll_area_padding: NamedPadding,
    pub editor_header_padding: NamedPadding,
    pub editor_container: StyledContainer,
    pub editor_container_inner_padding: NamedPadding,
    pub editor_container_outer_padding: NamedPadding,
    pub newtol_container: StyledContainer,
    pub newtol_container_inner_padding: NamedPadding,
    pub newtol_container_outer_padding: NamedPadding,

    pub color: ColorList,
    pub radius: RadiusList,
    pub width: WidthList,
    pub text_size: TextSizeList,
    pub padding: PaddingList,
    pub spacing: SpacingList
}

impl Default for IcedStyleSheet {
    fn default() -> Self {
        // Define classes first so they can be referenced in the IcedStyleSheet construction
        let color = ColorList::new()
            .add("primary", SerializableColor{r: 245, g: 245, b: 245, a: 1.0})
            .add("secondary", SerializableColor{r: 245, g: 245, b: 245, a: 1.0})
            .add("background", SerializableColor{r: 245, g: 245, b: 245, a: 1.0})
            .add("text_h1", SerializableColor{r: 100, g: 100, b: 100, a: 1.0})
            .add("text_h2", SerializableColor{r: 100, g: 100, b: 100, a: 1.0})
            .add("text_p", SerializableColor{r: 100, g: 100, b: 100, a: 1.0})
        ;
        let radius = RadiusList::new()
            .add("none", 0)
            .add("small", 2)
            .add("large", 5)
        ;
        let width = WidthList::new()
            .add("none", 0)
            .add("thin", 1)
            .add("bold", 3)
        ;
        let text_size = TextSizeList::new()
            .add("h1", 32)
            .add("h2", 24)
            .add("p", 12)
        ;
        let padding = PaddingList::new()
            .add("narrow", 10)
            .add("wide", 20)
        ;
        let spacing = SpacingList::new()
            .add("near", 10)
            .add("far", 20)
            .add("huge", 40)
        ;
        // Construct a iss, note that `Named___` objects use a class list for validatation
        IcedStyleSheet{
            //Project Label
            project_label_color: NamedColor::new("text_h1", &color),
            project_label_text_size: NamedTextSize::new("h1", &text_size),
            project_label_spacing: NamedSpacing::new("near", &spacing),

            //Editable Label
            editablelabel_label_color: NamedColor::new("text_h1", &color),
            editablelabel_label_text_size: NamedTextSize::new("h1", &text_size),

            //Header
            header_spacing: NamedSpacing::new("far", &spacing),
            header_button_spacing: NamedSpacing::new("near", &spacing),

            //Background Container
            home_container: StyledContainer {
                text_color: NamedColor::new("text_p", &color),
                background: NamedColor::new("background", &color),
                border_color: NamedColor::new("background", &color),
                border_radius: NamedRadius::new("none", &radius),
                border_width: NamedWidth::new("none", &width),
            },
            home_padding: NamedPadding::new("narrow", &padding),
            
            //area_mc_analysis
            mc_results_row_spacing: NamedSpacing::new("far", &spacing),
            mc_results_col_spacing: NamedSpacing::new("far", &spacing),

            //area_stack_editor
            editor_tol_spacing: NamedSpacing::new("far", &spacing),
            editor_content_spacing: NamedSpacing::new("far", &spacing),
            editor_title_text_size: NamedTextSize::new("h2", &text_size),
            editor_scroll_area_padding: NamedPadding::new("narrow", &padding),
            editor_header_padding: NamedPadding::new("narrow", &padding),
            editor_container: StyledContainer {
                text_color: NamedColor::new("text_p", &color),
                background: NamedColor::new("background", &color),
                border_color: NamedColor::new("background", &color),
                border_radius: NamedRadius::new("none", &radius),
                border_width: NamedWidth::new("none", &width),
            },
            editor_container_inner_padding: NamedPadding::new("narrow", &padding),
            editor_container_outer_padding: NamedPadding::new("wide", &padding),
            newtol_container: StyledContainer {
                text_color: NamedColor::new("text_p", &color),
                background: NamedColor::new("background", &color),
                border_color: NamedColor::new("background", &color),
                border_radius: NamedRadius::new("none", &radius),
                border_width: NamedWidth::new("none", &width),
            },
            newtol_container_inner_padding: NamedPadding::new("wide", &padding),
            newtol_container_outer_padding: NamedPadding::new("wide", &padding),

            // Classes placed at end to avoid needing a .clone()
            color,
            radius,
            width,
            text_size,
            padding,
            spacing,
        }
    }
}

impl IcedStyleSheet {
    pub fn color(&self, name: &NamedColor) -> iced_native::Color {
        self.color.resolve(name)
    }
    
    pub fn radius(&self, name: &NamedRadius) -> u16 {
        self.radius.resolve(name)
    }

    pub fn width(&self, name: &NamedWidth) -> u16 {
        self.width.resolve(name)
    }

    pub fn text_size(&self, name: &NamedTextSize) -> u16 {
        self.text_size.resolve(name)
    }

    pub fn padding(&self, name: &NamedPadding) -> u16 {
        self.padding.resolve(name)
    }

    pub fn spacing(&self, name: &NamedSpacing) -> u16 {
        self.spacing.resolve(name)
    }

    pub fn container(&self, container: &StyledContainer) -> IcedContainerStyle {
        IcedContainerStyle::new(container, self)
    }

    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories::ProjectDirs::from("rs", "", "TolStack")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or(std::path::PathBuf::new())
        };

        path.push("style.toml");

        path
    }

    pub async fn load() -> Result<IcedStyleSheet, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::FileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;

        toml::from_str(&contents).map_err(|_| LoadError::FormatError)
    }

    pub async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;
        let toml = toml::to_string(&self)
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
            file.write_all(toml.as_bytes())
                .await
                .map_err(|_| SaveError::WriteError)?;
        }
        Ok(())
    }

    /*pub async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;
        let json = json::to_string_pretty(&self)
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
    }*/

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

impl<H, I> iced_native::subscription::Recipe<H, I> for IcedStyleSheet
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
                match watch(IcedStyleSheet::path()) {
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