use crate::ui::{components::*, style};
use iced::{
    button, Align, Button, Column, Container, Element, HorizontalAlignment, Length, Row, Text,
};

#[derive(Debug, Clone)]
pub enum Message {
    LabelMessage(editable_label::Message),
    OpenFile,
}

#[derive(Debug, Default, Clone)]
pub struct Header {
    pub title: EditableLabel,
    button_open: button::State,
}
impl Header {
    pub fn new() -> Self {
        Header {
            title: EditableLabel::new("New Project", "Add a project name..."),
            button_open: button::State::new(),
        }
    }
    pub fn update(&mut self, message: Message) {
        let Header { title, button_open } = self;
        match message {
            Message::LabelMessage(label_message) => {
                // Pass the message into the title
                title.update(label_message);
            }
            Message::OpenFile => {}
        }
    }
    pub fn view(&mut self, iss: &style::IcedStyleSheet) -> Element<Message> {
        let Header { title, button_open } = self;
        let project_label = Text::new("Project: ")
            .width(Length::Shrink)
            .size(iss.text_size(&iss.project_label_text_size))
            .color(iss.color(&iss.project_label_color))
            .horizontal_alignment(HorizontalAlignment::Left);

        let project_title: Row<_> = Row::new()
            .push(project_label)
            .push(
                title
                    .view(&iss)
                    .map(move |message| Message::LabelMessage(message)),
            )
            .align_items(Align::Center)
            .spacing(iss.spacing(&iss.project_label_spacing))
            .into();

        let project_title_container =
            Container::new(Row::new().push(project_title).width(Length::Shrink))
                .width(Length::Fill)
                .center_x()
                .center_y();

        let header = Container::new(
            Column::new()
                .max_width(800)
                .spacing(iss.spacing(&iss.header_spacing))
                .push(project_title_container)
                .push(
                    Button::new(
                        button_open,
                        Row::new()
                            .push(Text::new("Open"))
                            //.push(icons::edit())
                            .spacing(iss.spacing(&iss.header_button_spacing)),
                    )
                    .on_press(Message::OpenFile)
                    .style(style::Button::Neutral),
                ),
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
