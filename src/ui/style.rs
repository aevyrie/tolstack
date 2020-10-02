use iced::{button, container, futures, Background, Color, Vector};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde_derive::*;
use std::collections::HashMap;
use std::path::PathBuf;

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
pub struct SerializableColor {
    r: u8,
    g: u8,
    b: u8,
    a: f32,
}

#[allow(dead_code)]
impl SerializableColor {
    fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        SerializableColor { r, b, g, a }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedColor(String);
impl Named for NamedColor {
    type List = ColorList;
    type NamedItem = NamedColor;
    fn new_unvalidated(name: &str) -> Self {
        NamedColor(name.to_string())
    }
    fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedRadius(String);
impl Named for NamedRadius {
    type List = RadiusList;
    type NamedItem = NamedRadius;
    fn new_unvalidated(name: &str) -> Self {
        NamedRadius(name.to_string())
    }
    fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedWidth(String);
impl Named for NamedWidth {
    type List = WidthList;
    type NamedItem = NamedWidth;
    fn new_unvalidated(name: &str) -> Self {
        NamedWidth(name.to_string())
    }
    fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedDimension(String);
impl Named for NamedDimension {
    type List = DimensionList;
    type NamedItem = NamedDimension;
    fn new_unvalidated(name: &str) -> Self {
        NamedDimension(name.to_string())
    }
    fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedTextSize(String);
impl Named for NamedTextSize {
    type List = TextSizeList;
    type NamedItem = NamedTextSize;
    fn new_unvalidated(name: &str) -> Self {
        NamedTextSize(name.to_string())
    }
    fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedPadding(String);
impl Named for NamedPadding {
    type List = PaddingList;
    type NamedItem = NamedPadding;
    fn new_unvalidated(name: &str) -> Self {
        NamedPadding(name.to_string())
    }
    fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedSpacing(String);
impl Named for NamedSpacing {
    type List = SpacingList;
    type NamedItem = NamedSpacing;
    fn new_unvalidated(name: &str) -> Self {
        NamedSpacing(name.to_string())
    }
    fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedVector(String);
impl Named for NamedVector {
    type List = VectorList;
    type NamedItem = NamedVector;
    fn new_unvalidated(name: &str) -> Self {
        NamedVector(name.to_string())
    }
    fn name(&self) -> &str {
        &self.0
    }
}

pub trait Named {
    type List: NamedList;
    type NamedItem: Named;

    fn new(name: &str, list: &Self::List) -> Self::NamedItem {
        let unvalidated = Self::NamedItem::new_unvalidated(name);
        if list.is_valid_name(&unvalidated) {
            unvalidated
        } else {
            panic!(
                "{:?} '{}' failed validation",
                std::any::type_name::<Self>(),
                name
            );
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
pub struct ColorList {
    map: HashMap<String, SerializableColor>,
}
impl NamedList for ColorList {
    type NamedItem = NamedColor;
    type Value = Color;
    type Stored = SerializableColor;

    fn new() -> Self {
        ColorList {
            map: HashMap::new(),
        }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(color) => Color::from_rgba8(color.r, color.g, color.b, color.a),
            None => Color::from_rgb(1.0, 0.0, 1.0),
        }
    }

    fn is_valid_name<T: Named>(&self, lookup: &T) -> bool {
        self.map.contains_key(lookup.name())
    }

    fn add(&mut self, name: &str, value: Self::Stored) -> Self {
        self.map.insert(name.to_string(), value);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiusList {
    map: HashMap<String, u16>,
}
impl NamedList for RadiusList {
    type NamedItem = NamedRadius;
    type Value = u16;
    type Stored = u16;

    fn new() -> Self {
        RadiusList {
            map: HashMap::new(),
        }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(radius) => *radius,
            None => 0,
        }
    }

    fn is_valid_name<T: Named>(&self, lookup: &T) -> bool {
        self.map.contains_key(lookup.name())
    }

    fn add(&mut self, name: &str, value: Self::Stored) -> Self {
        self.map.insert(name.to_string(), value);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidthList {
    map: HashMap<String, u16>,
}
impl NamedList for WidthList {
    type NamedItem = NamedWidth;
    type Value = u16;
    type Stored = u16;

    fn new() -> Self {
        WidthList {
            map: HashMap::new(),
        }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(width) => *width,
            None => 0,
        }
    }

    fn is_valid_name<T: Named>(&self, lookup: &T) -> bool {
        self.map.contains_key(lookup.name())
    }

    fn add(&mut self, name: &str, value: Self::Stored) -> Self {
        self.map.insert(name.to_string(), value);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionList {
    map: HashMap<String, u16>,
}
impl NamedList for DimensionList {
    type NamedItem = NamedDimension;
    type Value = u16;
    type Stored = u16;

    fn new() -> Self {
        DimensionList {
            map: HashMap::new(),
        }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(width) => *width,
            None => 0,
        }
    }

    fn is_valid_name<T: Named>(&self, lookup: &T) -> bool {
        self.map.contains_key(lookup.name())
    }

    fn add(&mut self, name: &str, value: Self::Stored) -> Self {
        self.map.insert(name.to_string(), value);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSizeList {
    map: HashMap<String, u16>,
}
impl NamedList for TextSizeList {
    type NamedItem = NamedTextSize;
    type Value = u16;
    type Stored = u16;

    fn new() -> Self {
        TextSizeList {
            map: HashMap::new(),
        }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(text_size) => *text_size,
            None => 0,
        }
    }

    fn is_valid_name<T: Named>(&self, lookup: &T) -> bool {
        self.map.contains_key(lookup.name())
    }

    fn add(&mut self, name: &str, value: Self::Stored) -> Self {
        self.map.insert(name.to_string(), value);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaddingList {
    map: HashMap<String, u16>,
}
impl NamedList for PaddingList {
    type NamedItem = NamedPadding;
    type Value = u16;
    type Stored = u16;

    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(lookup) => *lookup,
            None => 0,
        }
    }

    fn is_valid_name<T: Named>(&self, lookup: &T) -> bool {
        self.map.contains_key(lookup.name())
    }

    fn add(&mut self, name: &str, value: Self::Stored) -> Self {
        self.map.insert(name.to_string(), value);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingList {
    map: HashMap<String, u16>,
}
impl NamedList for SpacingList {
    type NamedItem = NamedSpacing;
    type Value = u16;
    type Stored = u16;

    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(lookup) => *lookup,
            None => 0,
        }
    }

    fn is_valid_name<T: Named>(&self, lookup: &T) -> bool {
        self.map.contains_key(lookup.name())
    }

    fn add(&mut self, name: &str, value: Self::Stored) -> Self {
        self.map.insert(name.to_string(), value);
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorList {
    map: HashMap<String, (f32, f32)>,
}
impl NamedList for VectorList {
    type NamedItem = NamedVector;
    type Value = (f32, f32);
    type Stored = (f32, f32);

    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn resolve(&self, lookup: &Self::NamedItem) -> Self::Value {
        match self.map.get(&lookup.0) {
            Some(lookup) => *lookup,
            None => (1.0, 1.0),
        }
    }

    fn is_valid_name<T: Named>(&self, lookup: &T) -> bool {
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
    border_width: NamedWidth,
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
        iced::container::Style {
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
        IcedContainerStyle {
            text_color: Some(iss.color.resolve(&container.text_color)),
            background: Some(Background::Color(iss.color.resolve(&container.background))),
            border_color: iss.color.resolve(&container.border_color),
            border_radius: iss.radius.resolve(&container.border_radius),
            border_width: iss.width.resolve(&container.border_width),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyledButton {
    active_shadow_offset: NamedVector,
    active_background: NamedColor,
    active_border_radius: NamedRadius,
    active_border_width: NamedWidth,
    active_border_color: NamedColor,
    active_text_color: NamedColor,
    hover_shadow_offset: NamedVector,
    hover_background: NamedColor,
    hover_border_radius: NamedRadius,
    hover_border_width: NamedWidth,
    hover_border_color: NamedColor,
    hover_text_color: NamedColor,
}

pub struct IcedButtonStyle {
    active_shadow_offset: Vector,
    active_background: Option<Background>,
    active_border_radius: u16,
    active_border_width: u16,
    active_border_color: Color,
    active_text_color: Color,
    hover_shadow_offset: Vector,
    hover_background: Option<Background>,
    hover_border_radius: u16,
    hover_border_width: u16,
    hover_border_color: Color,
    hover_text_color: Color,
}
impl button::StyleSheet for IcedButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: self.active_shadow_offset,
            background: self.active_background,
            border_radius: self.active_border_radius,
            border_width: self.active_border_width,
            border_color: self.active_border_color,
            text_color: self.active_text_color,
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            shadow_offset: self.hover_shadow_offset,
            background: self.hover_background,
            border_radius: self.hover_border_radius,
            border_width: self.hover_border_width,
            border_color: self.hover_border_color,
            text_color: self.hover_text_color,
        }
    }
}
impl IcedButtonStyle {
    pub fn new(button: &StyledButton, iss: &IcedStyleSheet) -> Self {
        let x_a = iss.vector.resolve(&button.active_shadow_offset).0;
        let y_a = iss.vector.resolve(&button.active_shadow_offset).1;
        let x_h = iss.vector.resolve(&button.hover_shadow_offset).0;
        let y_h = iss.vector.resolve(&button.hover_shadow_offset).1;
        IcedButtonStyle {
            // Styling for active button
            active_shadow_offset: Vector::new(x_a, y_a),
            active_background: Some(Background::Color(
                iss.color.resolve(&button.active_background),
            )),
            active_border_radius: iss.radius.resolve(&button.active_border_radius),
            active_border_width: iss.width.resolve(&button.active_border_width),
            active_border_color: iss.color.resolve(&button.active_border_color),
            active_text_color: iss.color.resolve(&button.active_text_color),
            // Styling for hovered button
            hover_shadow_offset: Vector::new(x_h, y_h),
            hover_background: Some(Background::Color(
                iss.color.resolve(&button.hover_background),
            )),
            hover_border_radius: iss.radius.resolve(&button.hover_border_radius),
            hover_border_width: iss.width.resolve(&button.hover_border_width),
            hover_border_color: iss.color.resolve(&button.hover_border_color),
            hover_text_color: iss.color.resolve(&button.hover_text_color),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcedStyleSheet {
    // Project Label
    pub project_label_color: NamedColor,
    pub project_label_text_size: NamedTextSize,
    pub project_label_spacing: NamedSpacing,

    // Editable Label
    pub editablelabel_label_color: NamedColor,
    pub editablelabel_label_text_size: NamedTextSize,

    // Header
    pub header_spacing: NamedSpacing,
    pub header_button_internal_spacing: NamedSpacing,
    pub header_button_external_spacing: NamedSpacing,
    pub header_button_text_size: NamedTextSize,
    pub header_button_icon_size: NamedTextSize,
    pub header_button_padding: NamedPadding,
    pub header_button_height: NamedDimension,
    pub header_button_width: NamedDimension,
    pub header_button_style: StyledButton,
    pub header_menu_container: StyledContainer,

    // Background Container
    pub home_container: StyledContainer,
    pub home_padding: NamedPadding,

    // area_mc_analysis
    pub mc_results_row_spacing: NamedSpacing,
    pub mc_results_col_spacing: NamedSpacing,
    pub mc_results_container_inner_padding: NamedPadding,
    pub mc_results_container_outer_padding: NamedPadding,
    pub results: NamedTextSize,

    // area_stack_editor
    pub editor_tol_spacing: NamedSpacing,
    pub editor_content_spacing: NamedSpacing,
    pub editor_title_text_size: NamedTextSize,
    pub editor_scroll_area_padding: NamedPadding,
    pub editor_scroll_area_padding_correction: NamedPadding,
    pub editor_scroll_container: StyledContainer,
    pub editor_header_padding: NamedPadding,
    pub editor_container_inner_padding: NamedPadding,
    pub editor_container_outer_padding: NamedPadding,
    pub newtol_container_inner_padding: NamedPadding,
    pub newtol_container_outer_padding: NamedPadding,

    // entry_tolerance
    pub tol_entry_summary_text_size: NamedTextSize,
    pub tol_entry_padding: NamedPadding,
    pub tol_entry_spacing: NamedSpacing,
    pub tol_entry_button_text_size: NamedTextSize,
    pub tol_entry_button_spacing: NamedSpacing,
    pub tol_entry_button_padding: NamedPadding,
    pub tol_entry_container: StyledContainer,
    pub tol_edit_button_text_size: NamedTextSize,
    pub tol_edit_field_padding: NamedPadding,
    pub tol_edit_field_text_size: NamedTextSize,
    pub tol_edit_heading_text_size: NamedTextSize,
    pub tol_edit_label_text_size: NamedTextSize,
    pub tol_edit_label_spacing: NamedSpacing,
    pub tol_edit_vspacing: NamedSpacing,
    pub tol_edit_padding: NamedPadding,

    // General Coontainers
    pub panel_container: StyledContainer,

    // General Buttons
    pub button_action: StyledButton,
    pub button_active: StyledButton,
    pub button_inactive: StyledButton,
    pub button_constructive: StyledButton,
    pub button_destructive: StyledButton,
    pub button_icon: StyledButton,

    // Named propery lists
    pub color: ColorList,
    pub radius: RadiusList,
    pub width: WidthList,
    pub dimension: DimensionList,
    pub text_size: TextSizeList,
    pub padding: PaddingList,
    pub spacing: SpacingList,
    pub vector: VectorList,
}

impl Default for IcedStyleSheet {
    fn default() -> Self {
        // Define classes first so they can be referenced in the IcedStyleSheet construction
        let color = ColorList::new()
            .add(
                "none",
                SerializableColor {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0.0,
                },
            )
            .add(
                "primary",
                SerializableColor {
                    r: 0,
                    g: 126,
                    b: 167,
                    a: 1.0,
                },
            )
            .add(
                "constructive",
                SerializableColor {
                    r: 107,
                    g: 212,
                    b: 37,
                    a: 1.0,
                },
            )
            .add(
                "destructive",
                SerializableColor {
                    r: 239,
                    g: 62,
                    b: 54,
                    a: 1.0,
                },
            )
            .add(
                "background",
                SerializableColor {
                    r: 230,
                    g: 230,
                    b: 230,
                    a: 1.0,
                },
            )
            .add(
                "active",
                SerializableColor {
                    r: 240,
                    g: 240,
                    b: 240,
                    a: 1.0,
                },
            )
            .add(
                "inactive",
                SerializableColor {
                    r: 245,
                    g: 245,
                    b: 245,
                    a: 1.0,
                },
            )
            .add(
                "highlight",
                SerializableColor {
                    r: 250,
                    g: 250,
                    b: 250,
                    a: 1.0,
                },
            )
            .add(
                "panel",
                SerializableColor {
                    r: 243,
                    g: 242,
                    b: 241,
                    a: 1.0,
                },
            )
            .add(
                "panel_border",
                SerializableColor {
                    r: 200,
                    g: 200,
                    b: 200,
                    a: 1.0,
                },
            )
            .add(
                "text_light",
                SerializableColor {
                    r: 250,
                    g: 250,
                    b: 250,
                    a: 1.0,
                },
            )
            .add(
                "text",
                SerializableColor {
                    r: 20,
                    g: 20,
                    b: 20,
                    a: 1.0,
                },
            )
            .add(
                "scroll_area",
                SerializableColor {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 1.0,
                },
            )
            .add(
                "entry",
                SerializableColor {
                    r: 248,
                    g: 248,
                    b: 248,
                    a: 1.0,
                },
            )
            .add(
                "entry_border",
                SerializableColor {
                    r: 230,
                    g: 230,
                    b: 230,
                    a: 1.0,
                },
            );
        let radius = RadiusList::new()
            .add("none", 0)
            .add("small", 4)
            .add("large", 6)
            .add("extra_large", 12);
        let width = WidthList::new()
            .add("none", 0)
            .add("thin", 1)
            .add("bold", 3);
        let dimension = DimensionList::new()
            .add("height_ribbon_tall", 70)
            .add("height_ribbon_short", 40)
            .add("width_button_ribbon", 50);
        let text_size = TextSizeList::new()
            .add("h1", 32)
            .add("h2", 24)
            .add("h3", 20)
            .add("p", 14)
            .add("icon_huge", 25)
            .add("icon_medium", 18);
        let padding = PaddingList::new()
            .add("tiny", 2)
            .add("narrow", 10)
            .add("wide", 20)
            .add("extra_wide", 50)
            .add("panel_outer", 10)
            .add("panel_inner", 30);
        let spacing = SpacingList::new()
            .add("near", 10)
            .add("far", 20)
            .add("huge", 40);
        let vector = VectorList::new()
            .add("none", (0.0, 0.0))
            .add("bottom", (0.0, 1.0))
            .add("top", (0.0, -1.0));

        // Construct an IcedStyleSheet, note that `Named` objects use a class list for validatation
        IcedStyleSheet {
            //Project Label
            project_label_color: NamedColor::new("text", &color),
            project_label_text_size: NamedTextSize::new("h2", &text_size),
            project_label_spacing: NamedSpacing::new("near", &spacing),

            //Editable Label
            editablelabel_label_color: NamedColor::new("text", &color),
            editablelabel_label_text_size: NamedTextSize::new("h2", &text_size),

            //Header
            header_spacing: NamedSpacing::new("near", &spacing),
            header_button_external_spacing: NamedSpacing::new("near", &spacing),
            header_button_internal_spacing: NamedSpacing::new("near", &spacing),
            header_button_text_size: NamedTextSize::new("p", &text_size),
            header_button_icon_size: NamedTextSize::new("icon_medium", &text_size),
            header_button_padding: NamedPadding::new("narrow", &padding),
            header_button_height: NamedDimension::new("height_ribbon_tall", &dimension),
            header_button_width: NamedDimension::new("width_button_ribbon", &dimension),
            header_button_style: StyledButton {
                active_shadow_offset: NamedVector::new("none", &vector),
                active_background: NamedColor::new("panel", &color),
                active_border_radius: NamedRadius::new("none", &radius),
                active_border_width: NamedWidth::new("none", &width),
                active_border_color: NamedColor::new("panel_border", &color),
                active_text_color: NamedColor::new("text", &color),
                hover_shadow_offset: NamedVector::new("none", &vector),
                hover_background: NamedColor::new("panel_border", &color),
                hover_border_radius: NamedRadius::new("small", &radius),
                hover_border_width: NamedWidth::new("none", &width),
                hover_border_color: NamedColor::new("text", &color),
                hover_text_color: NamedColor::new("text", &color),
            },
            header_menu_container: StyledContainer {
                text_color: NamedColor::new("text", &color),
                background: NamedColor::new("panel", &color),
                border_color: NamedColor::new("panel_border", &color),
                border_radius: NamedRadius::new("none", &radius),
                border_width: NamedWidth::new("thin", &width),
            },

            //Home Container
            home_container: StyledContainer {
                text_color: NamedColor::new("text", &color),
                background: NamedColor::new("background", &color),
                border_color: NamedColor::new("background", &color),
                border_radius: NamedRadius::new("none", &radius),
                border_width: NamedWidth::new("none", &width),
            },
            home_padding: NamedPadding::new("extra_wide", &padding),

            //area_mc_analysis
            mc_results_row_spacing: NamedSpacing::new("far", &spacing),
            mc_results_col_spacing: NamedSpacing::new("far", &spacing),
            mc_results_container_inner_padding: NamedPadding::new("panel_inner", &padding),
            mc_results_container_outer_padding: NamedPadding::new("panel_outer", &padding),
            results: NamedTextSize::new("p", &text_size),

            //area_stack_editor
            editor_tol_spacing: NamedSpacing::new("near", &spacing),
            editor_content_spacing: NamedSpacing::new("far", &spacing),
            editor_title_text_size: NamedTextSize::new("h2", &text_size),
            editor_scroll_area_padding: NamedPadding::new("wide", &padding),
            editor_scroll_area_padding_correction: NamedPadding::new("tiny", &padding),
            editor_scroll_container: StyledContainer {
                text_color: NamedColor::new("text", &color),
                background: NamedColor::new("scroll_area", &color),
                border_color: NamedColor::new("panel_border", &color),
                border_radius: NamedRadius::new("large", &radius),
                border_width: NamedWidth::new("thin", &width),
            },
            editor_header_padding: NamedPadding::new("narrow", &padding),
            editor_container_inner_padding: NamedPadding::new("panel_inner", &padding),
            editor_container_outer_padding: NamedPadding::new("panel_outer", &padding),
            newtol_container_inner_padding: NamedPadding::new("panel_inner", &padding),
            newtol_container_outer_padding: NamedPadding::new("panel_outer", &padding),

            // entry_tolerance
            tol_entry_summary_text_size: NamedTextSize::new("p", &text_size),
            tol_entry_padding: NamedPadding::new("narrow", &padding),
            tol_entry_spacing: NamedSpacing::new("near", &spacing),
            tol_entry_button_text_size: NamedTextSize::new("p", &text_size),
            tol_entry_button_spacing: NamedSpacing::new("near", &spacing),
            tol_entry_button_padding: NamedPadding::new("narrow", &padding),
            tol_entry_container: StyledContainer {
                text_color: NamedColor::new("text", &color),
                background: NamedColor::new("entry", &color),
                border_color: NamedColor::new("entry_border", &color),
                border_radius: NamedRadius::new("small", &radius),
                border_width: NamedWidth::new("thin", &width),
            },
            tol_edit_button_text_size: NamedTextSize::new("p", &text_size),
            tol_edit_field_padding: NamedPadding::new("narrow", &padding),
            tol_edit_field_text_size: NamedTextSize::new("p", &text_size),
            tol_edit_heading_text_size: NamedTextSize::new("h3", &text_size),
            tol_edit_label_text_size: NamedTextSize::new("p", &text_size),
            tol_edit_label_spacing: NamedSpacing::new("far", &spacing),
            tol_edit_vspacing: NamedSpacing::new("near", &spacing),
            tol_edit_padding: NamedPadding::new("wide", &padding),

            // General Containers
            panel_container: StyledContainer {
                text_color: NamedColor::new("text", &color),
                background: NamedColor::new("panel", &color),
                border_color: NamedColor::new("panel_border", &color),
                border_radius: NamedRadius::new("none", &radius),
                border_width: NamedWidth::new("none", &width),
            },

            // General Buttons
            button_action: StyledButton {
                active_shadow_offset: NamedVector::new("bottom", &vector),
                active_background: NamedColor::new("primary", &color),
                active_border_radius: NamedRadius::new("small", &radius),
                active_border_width: NamedWidth::new("none", &width),
                active_border_color: NamedColor::new("primary", &color),
                active_text_color: NamedColor::new("text_light", &color),
                hover_shadow_offset: NamedVector::new("bottom", &vector),
                hover_background: NamedColor::new("primary", &color),
                hover_border_radius: NamedRadius::new("small", &radius),
                hover_border_width: NamedWidth::new("none", &width),
                hover_border_color: NamedColor::new("primary", &color),
                hover_text_color: NamedColor::new("text", &color),
            },
            button_active: StyledButton {
                active_shadow_offset: NamedVector::new("top", &vector),
                active_background: NamedColor::new("highlight", &color),
                active_border_radius: NamedRadius::new("small", &radius),
                active_border_width: NamedWidth::new("none", &width),
                active_border_color: NamedColor::new("primary", &color),
                active_text_color: NamedColor::new("text", &color),
                hover_shadow_offset: NamedVector::new("top", &vector),
                hover_background: NamedColor::new("highlight", &color),
                hover_border_radius: NamedRadius::new("small", &radius),
                hover_border_width: NamedWidth::new("none", &width),
                hover_border_color: NamedColor::new("primary", &color),
                hover_text_color: NamedColor::new("text", &color),
            },
            button_inactive: StyledButton {
                active_shadow_offset: NamedVector::new("none", &vector),
                active_background: NamedColor::new("entry", &color),
                active_border_radius: NamedRadius::new("small", &radius),
                active_border_width: NamedWidth::new("thin", &width),
                active_border_color: NamedColor::new("entry_border", &color),
                active_text_color: NamedColor::new("text", &color),
                hover_shadow_offset: NamedVector::new("bottom", &vector),
                hover_background: NamedColor::new("primary", &color),
                hover_border_radius: NamedRadius::new("small", &radius),
                hover_border_width: NamedWidth::new("thin", &width),
                hover_border_color: NamedColor::new("primary", &color),
                hover_text_color: NamedColor::new("text_light", &color),
            },
            button_constructive: StyledButton {
                active_shadow_offset: NamedVector::new("bottom", &vector),
                active_background: NamedColor::new("constructive", &color),
                active_border_radius: NamedRadius::new("small", &radius),
                active_border_width: NamedWidth::new("none", &width),
                active_border_color: NamedColor::new("primary", &color),
                active_text_color: NamedColor::new("text_light", &color),
                hover_shadow_offset: NamedVector::new("bottom", &vector),
                hover_background: NamedColor::new("constructive", &color),
                hover_border_radius: NamedRadius::new("small", &radius),
                hover_border_width: NamedWidth::new("none", &width),
                hover_border_color: NamedColor::new("primary", &color),
                hover_text_color: NamedColor::new("text", &color),
            },
            button_destructive: StyledButton {
                active_shadow_offset: NamedVector::new("bottom", &vector),
                active_background: NamedColor::new("destructive", &color),
                active_border_radius: NamedRadius::new("small", &radius),
                active_border_width: NamedWidth::new("none", &width),
                active_border_color: NamedColor::new("primary", &color),
                active_text_color: NamedColor::new("text_light", &color),
                hover_shadow_offset: NamedVector::new("bottom", &vector),
                hover_background: NamedColor::new("destructive", &color),
                hover_border_radius: NamedRadius::new("small", &radius),
                hover_border_width: NamedWidth::new("none", &width),
                hover_border_color: NamedColor::new("primary", &color),
                hover_text_color: NamedColor::new("text", &color),
            },
            button_icon: StyledButton {
                active_shadow_offset: NamedVector::new("none", &vector),
                active_background: NamedColor::new("none", &color),
                active_border_radius: NamedRadius::new("none", &radius),
                active_border_width: NamedWidth::new("none", &width),
                active_border_color: NamedColor::new("none", &color),
                active_text_color: NamedColor::new("text", &color),
                hover_shadow_offset: NamedVector::new("none", &vector),
                hover_background: NamedColor::new("none", &color),
                hover_border_radius: NamedRadius::new("none", &radius),
                hover_border_width: NamedWidth::new("none", &width),
                hover_border_color: NamedColor::new("none", &color),
                hover_text_color: NamedColor::new("primary", &color),
            },

            // Classes placed at end to avoid needing a .clone()
            color,
            radius,
            width,
            dimension,
            text_size,
            padding,
            spacing,
            vector,
        }
    }
}

#[allow(dead_code)]
impl IcedStyleSheet {
    pub fn color(&self, name: &NamedColor) -> iced::Color {
        self.color.resolve(name)
    }

    pub fn radius(&self, name: &NamedRadius) -> u16 {
        self.radius.resolve(name)
    }

    pub fn width(&self, name: &NamedWidth) -> u16 {
        self.width.resolve(name)
    }

    pub fn dimension(&self, name: &NamedDimension) -> iced::Length {
        iced::Length::Units(self.dimension.resolve(name))
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

    pub fn vector(&self, name: &NamedVector) -> (f32, f32) {
        self.vector.resolve(name)
    }

    pub fn container(&self, container: &StyledContainer) -> IcedContainerStyle {
        IcedContainerStyle::new(container, self)
    }

    pub fn button(&self, button: &StyledButton) -> IcedButtonStyle {
        IcedButtonStyle::new(button, self)
    }

    pub fn toggle_button(
        &self,
        active: bool,
        button_active: &StyledButton,
        button_inactive: &StyledButton,
    ) -> IcedButtonStyle {
        if active {
            IcedButtonStyle::new(button_active, self)
        } else {
            IcedButtonStyle::new(button_inactive, self)
        }
    }

    fn path() -> std::path::PathBuf {
        let mut path =
            if let Some(project_dirs) = directories::ProjectDirs::from("rs", "", "TolStack") {
                project_dirs.data_dir().into()
            } else {
                std::env::current_dir().unwrap_or(std::path::PathBuf::new())
            };

        path.push("style.json");

        path
    }

    /*pub async fn load() -> Result<IcedStyleSheet, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::FileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;

        toml::from_str(&contents).map_err(|_| LoadError::FormatError)
    }*/

    pub async fn load() -> Result<IcedStyleSheet, LoadError> {
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

    /*
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
    }*/

    pub async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;
        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::FormatError)?;
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

    Err(Box::from(std::io::Error::new(
        std::io::ErrorKind::Other,
        "No event returned from fn watch",
    )))
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

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;

        async_std::stream::repeat_with(move || loop {
            match watch(IcedStyleSheet::path()) {
                Ok(_) => return true,
                Err(_) => {}
            }
        })
        .boxed()
    }
}

#[allow(dead_code)]
pub enum Button {
    Filter { selected: bool },
    Choice { selected: bool },
    Icon,
    Destructive,
    Constructive,
    Neutral,
}

impl Button {}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::Filter { selected } => {
                if *selected {
                    button::Style {
                        background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
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
                        background: Some(Background::Color(Color::from_rgb(0.2, 0.4, 0.7))),
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
                background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                border_radius: 5,
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 1.0),
                ..button::Style::default()
            },
            Button::Constructive => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.2, 0.8, 0.2))),
                border_radius: 5,
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 1.0),
                ..button::Style::default()
            },
            Button::Neutral => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.8, 0.8, 0.8))),
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
                Button::Filter { selected } if !selected => Color::from_rgb(0.5, 0.5, 0.5),
                Button::Filter { selected } if !selected => Color::from_rgb(0.3, 0.5, 0.8),
                _ => active.text_color,
            },
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            ..active
        }
    }
}

#[allow(dead_code)]
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
