use iced::{
    Align, Container, Element, HorizontalAlignment, Length, Row, Text, Column, Button, button
};
use crate::ui::{components::*, style};

#[derive(Debug, Clone)]
pub enum Message {
    LabelMessage(editable_label::Message),
    OpenFile
}

#[derive(Debug, Default, Clone)]
pub struct Header {
    pub title: EditableLabel,
    button_open: button::State,
}
impl Header {
    pub fn new() -> Self {
        Header {
            title: EditableLabel::new("New Project", "Add a project name..." ),
            button_open: button::State::new(), 
        }
    }
    pub fn update(&mut self, message: Message) {
        let Header {
            title,
            button_open,
        } = self;
        match message {
            Message::LabelMessage(label_message) => {
                // Pass the message into the title
                title.update(label_message);
            }
            Message::OpenFile => {}
        }
    }
    pub fn  view(&mut self, stylesheet: &style::StyleSheet) -> Element<Message> {
        let Header {
            title,
            button_open,
        } = self;
        let project_label = Text::new("Project: ")
            .width(Length::Shrink)
            .size(32)
            .color(stylesheet.color(&stylesheet.text_color_h1))
            .horizontal_alignment(HorizontalAlignment::Left);

        let project_title: Row<_> = Row::new()
            .push(project_label)
            .push(title.view(&stylesheet).map( move |message| {
                Message::LabelMessage(message)
            }))
            .align_items(Align::Center)
            .spacing(10)
            .into();
                        
        let project_title_container = 
            Container::new(
                Row::new()
                    .push(project_title)
                    .width(Length::Shrink)
            )
            .width(Length::Fill)
            .center_x()
            .center_y();

        let header = Container::new(
            Column::new()
                .max_width(800)
                .spacing(20)
                .push(project_title_container)
                .push(Button::new(
                    button_open, 
                    Row::new()
                        .push(Text::new("Open"))
                        //.push(icons::edit())
                        .spacing(10)
                    )
                    .on_press(Message::OpenFile)
                    .style(style::Button::Neutral)
                )
            )
            .width(Length::Fill)
            .padding(10)
            .center_x();

        header.into()
    }
    pub fn title(&mut self, title: String) -> Self {
        self.title.text = title;
        self.clone()
    }
}