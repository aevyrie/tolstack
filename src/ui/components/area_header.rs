use iced::{
    Align, Container, Element, HorizontalAlignment, Length, Row, Text, Column,
};
use crate::ui::{ components::* };

#[derive(Debug, Clone)]
pub enum Message {
    LabelMessage(editable_label::Message),
}

#[derive(Debug, Default, Clone)]
pub struct Header {
    pub title: EditableLabel,
}
impl Header {
    pub fn new() -> Self {
        Header {
            title: EditableLabel::new("New Project", "Add a project name..." ), 
        }
    }
    pub fn update(&mut self, message: Message) {
        let Header {
            title,
        } = self;
        match message {
            Message::LabelMessage(label_message) => {
                // Pass the message into the title
                title.update(label_message);
            }
        }
    }
    pub fn  view(&mut self) -> Element<Message> {
        let Header {
            title,
        } = self;
        let project_label = Text::new("Project: ")
            .width(Length::Shrink)
            .size(32)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Left);

        let project_title: Row<_> = Row::new()
            .push(project_label)
            .push(title.view().map( move |message| {
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