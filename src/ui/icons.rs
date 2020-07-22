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
    icon('\u{F303}')
}

pub fn delete() -> Text {
    icon('\u{F1F8}')
}

pub fn check() -> Text {
    icon('\u{2713}')
}
