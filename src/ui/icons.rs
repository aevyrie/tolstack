use iced::{Font, HorizontalAlignment, Length, Text};

const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("fonts/icons.ttf"),
};

fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}

pub fn edit() -> Text {
    icon('\u{e803}')
}

pub fn delete() -> Text {
    icon('\u{F1F8}')
}

pub fn check() -> Text {
    icon('\u{e806}')
}

pub fn save() -> Text {
    icon('\u{e800}')
}

pub fn load() -> Text {
    icon('\u{f115}')
}