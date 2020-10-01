use crate::ui::{components::*, icons, style};
use iced::{
    button, Align, Button, Column, Container, Element, HorizontalAlignment, Length, Row, Text,
    VerticalAlignment,
};

#[derive(Debug, Clone)]
pub enum Message {
    NewFile,
    OpenFile,
    SaveFile,
    SaveAsFile,
    ExportCSV,
    AddTolLinear,
    AddTolFloat,
    Help,
}

#[derive(Debug, Default, Clone)]
pub struct Header {
    button_new: button::State,
    button_open: button::State,
    button_save: button::State,
    button_export: button::State,
    button_save_as: button::State,
    button_add_tol_linear: button::State,
    button_add_tol_float: button::State,
    button_help: button::State,
}
impl Header {
    pub fn new() -> Self {
        Header {
            button_new: button::State::new(),
            button_open: button::State::new(),
            button_save: button::State::new(),
            button_export: button::State::new(),
            button_save_as: button::State::new(),
            button_add_tol_linear: button::State::new(),
            button_add_tol_float: button::State::new(),
            button_help: button::State::new(),
        }
    }
    pub fn update(&mut self, message: Message) {
        let Header {
            button_new: _,
            button_open: _,
            button_save: _,
            button_export: _,
            button_save_as: _,
            button_add_tol_linear: _,
            button_add_tol_float: _,
            button_help: _,
        } = self;
        match message {
            _ => (), // This message is captured in main.rs
        }
    }
    pub fn view(&mut self, iss: &style::IcedStyleSheet) -> Element<Message> {
        let Header {
            button_new,
            button_open,
            button_save,
            button_export,
            button_save_as,
            button_add_tol_linear,
            button_add_tol_float,
            button_help,
        } = self;
        let project_label = Text::new("Project: ")
            .width(Length::Shrink)
            .size(iss.text_size(&iss.project_label_text_size))
            .color(iss.color(&iss.project_label_color))
            .horizontal_alignment(HorizontalAlignment::Left);

        let button_new =
            header_button(button_new, "New\n", icons::new(), iss).on_press(Message::NewFile);

        let button_open =
            header_button(button_open, "Open\n", icons::load(), iss).on_press(Message::OpenFile);

        let button_save =
            header_button(button_save, "Save\n", icons::save(), iss).on_press(Message::SaveFile);

        let button_save_as = header_button(button_save_as, "Save As\n", icons::duplicate(), iss)
            .on_press(Message::SaveAsFile);

        let button_export = header_button(button_export, "Export CSV", icons::export(), iss)
            .on_press(Message::ExportCSV);

        let button_add_tol_linear =
            header_button(button_add_tol_linear, "Add Linear\n", icons::add(), iss)
                .on_press(Message::AddTolLinear);

        let button_add_tol_float =
            header_button(button_add_tol_float, "Add Float\n", icons::add(), iss)
                .on_press(Message::AddTolFloat);

        let button_help =
            header_button(button_help, "Help\n", icons::help(), iss).on_press(Message::Help);

        let ribbon = Container::new(
            Row::new()
                .push(button_new)
                .push(button_open)
                .push(button_save)
                .push(button_save_as)
                .push(button_export)
                .push(button_add_tol_linear)
                .push(button_add_tol_float)
                .push(button_help)
                .width(Length::Fill)
                .spacing(iss.spacing(&iss.header_button_external_spacing)),
        )
        .width(Length::Fill)
        .padding(iss.padding(&iss.header_button_padding))
        .style(iss.container(&iss.header_menu_container));

        let header = Column::new()
            .push(ribbon)
            //.push(
            //    Container::new(Column::new().push(project_title_container).max_width(800))
            //        .width(Length::Fill)
            //        .padding(10)
            //        .center_x(),
            //)
            .spacing(iss.spacing(&iss.header_spacing));

        header.into()
    }
}

fn header_button<'a>(
    state: &'a mut button::State,
    text: &str,
    icon: Text,
    iss: &style::IcedStyleSheet,
) -> Button<'a, Message> {
    Button::new(
        state,
        Column::new()
            .spacing(iss.spacing(&iss.header_button_internal_spacing))
            .push(
                Container::new(icon.size(iss.text_size(&iss.header_button_icon_size)))
                    .center_x()
                    .center_y()
                    //.height(Length::Fill)
                    .width(Length::Fill),
            )
            .push(
                Container::new(
                    Text::new(text)
                        .width(Length::Fill)
                        .size(iss.text_size(&iss.header_button_text_size))
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .center_x()
                .center_y()
                .width(Length::Fill),
            ),
    )
    .style(iss.button(&iss.header_button_style))
    .height(iss.dimension(&iss.header_button_height))
    .width(iss.dimension(&iss.header_button_width))
}
