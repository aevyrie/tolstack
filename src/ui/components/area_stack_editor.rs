use crate::analysis::structures::*;
use crate::ui::components::*;
use crate::ui::{icons, style};
use iced::{
    scrollable, Align, Column, Container, Element, HorizontalAlignment, Image, Length, Row,
    Scrollable, Text,
};

#[derive(Debug, Clone)]
pub enum StackEditorAreaMessage {
    EntryMessage(usize, entry_tolerance::Message),
    FilterMessage(filter_tolerance::Message),
    NewEntryMessage((String, Tolerance)),
    LabelMessage(editable_label::Message),
}

#[derive(Debug, Default, Clone)]
pub struct StackEditor {
    filter: ToleranceFilter,
    pub tolerances: Vec<ToleranceEntry>,
    scroll_state: scrollable::State,
    pub title: EditableLabel,
}
impl StackEditor {
    pub fn new() -> Self {
        StackEditor {
            title: EditableLabel::new("New Stack", "Add a name..."),
            ..Default::default()
        }
    }
    pub fn title(&mut self, title: String) -> Self {
        self.title.text = title;
        self.clone()
    }
    pub fn update(&mut self, message: StackEditorAreaMessage) {
        let StackEditor {
            filter,
            tolerances,
            scroll_state: _,
            title,
        } = self;
        match message {
            StackEditorAreaMessage::NewEntryMessage(tolerance) => {
                let (name, tol) = tolerance;
                tolerances.push(ToleranceEntry::new(name, tol).with_editing());
            }

            StackEditorAreaMessage::FilterMessage(message) => {
                match &message {
                    filter_tolerance::Message::FilterChanged(_) => {}
                };
                // Once we've processed the filter message in the parent component, pass the
                //  message into the filter to be processed.
                filter.update(message);
            }

            StackEditorAreaMessage::EntryMessage(i, message) => {
                // Some message `tol_message`  from a tolerance entry at index `i`
                match &message {
                    entry_tolerance::Message::EntryDelete => {
                        tolerances.remove(i);
                    }
                    entry_tolerance::Message::EntryMoveUp => {
                        if i > 0 {
                            tolerances.swap(i, i - 1)
                        }
                    }
                    entry_tolerance::Message::EntryMoveDown => {
                        if i < tolerances.len() - 1 {
                            tolerances.swap(i, i + 1)
                        }
                    }
                    entry_tolerance::Message::EntryFinishEditing => match tolerances.get_mut(i) {
                        Some(entry) => match &entry.input {
                            FormValues::Linear {
                                description: _,
                                dimension,
                                tolerance_pos,
                                tolerance_neg,
                                sigma,
                            } => {
                                let mut sanitized_dimension = 0.0;
                                let mut sanitized_tolerance_pos = 0.0;
                                let mut sanitized_tolerance_neg = 0.0;
                                let mut sanitized_sigma = 0.0;

                                entry.valid = true;

                                match dimension.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_dimension = value;
                                    }
                                    Err(_) => {
                                        entry.valid = false;
                                    }
                                }
                                match tolerance_pos.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_tolerance_pos = value;
                                    }
                                    Err(_) => {
                                        entry.valid = false;
                                    }
                                }
                                match tolerance_neg.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_tolerance_neg = value;
                                    }
                                    Err(_) => {
                                        entry.valid = false;
                                    }
                                }
                                match sigma.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_sigma = value;
                                    }
                                    Err(_) => {
                                        entry.valid = false;
                                    }
                                }
                                if entry.valid {
                                    entry.active = true;
                                    let linear = DimTol::new(
                                        sanitized_dimension,
                                        sanitized_tolerance_pos,
                                        sanitized_tolerance_neg,
                                        sanitized_sigma,
                                    );
                                    let linear = Tolerance::Linear(LinearTL::new(linear));
                                    entry.analysis_model = linear;
                                } else {
                                    entry.active = false;
                                }
                            }
                            FormValues::Float {
                                description: _,
                                diameter_hole,
                                diameter_pin,
                                tolerance_hole_pos,
                                tolerance_hole_neg,
                                tolerance_pin_pos,
                                tolerance_pin_neg,
                                sigma,
                            } => {
                                let mut sanitized_diameter_hole = 0.0;
                                let mut sanitized_diameter_pin = 0.0;
                                let mut sanitized_tolerance_hole_pos = 0.0;
                                let mut sanitized_tolerance_hole_neg = 0.0;
                                let mut sanitized_tolerance_pin_pos = 0.0;
                                let mut sanitized_tolerance_pin_neg = 0.0;
                                let mut sanitized_sigma = 0.0;

                                entry.valid = true;
                                match diameter_hole.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_diameter_hole = value;
                                    }
                                    Err(_) => {
                                        entry.valid = false;
                                    }
                                }
                                match diameter_pin.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_diameter_pin = value;
                                    }
                                    Err(_) => {
                                        entry.valid = false;
                                    }
                                }
                                match tolerance_hole_pos.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_tolerance_hole_pos = value;
                                    }
                                    Err(_) => {
                                        entry.valid = false;
                                    }
                                }
                                match tolerance_hole_neg.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_tolerance_hole_neg = value;
                                    }
                                    Err(_) => {
                                        entry.valid = false;
                                    }
                                }
                                match tolerance_pin_pos.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_tolerance_pin_pos = value;
                                    }
                                    Err(_) => {
                                        entry.valid = false;
                                    }
                                }
                                match tolerance_pin_neg.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_tolerance_pin_neg = value;
                                    }
                                    Err(_) => {
                                        entry.valid = false;
                                    }
                                }
                                match sigma.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_sigma = value;
                                    }
                                    Err(_) => {
                                        entry.valid = false;
                                    }
                                }
                                if entry.valid {
                                    entry.active = true;
                                    let hole = DimTol::new(
                                        sanitized_diameter_hole,
                                        sanitized_tolerance_hole_pos,
                                        sanitized_tolerance_hole_neg,
                                        sanitized_sigma,
                                    );
                                    let pin = DimTol::new(
                                        sanitized_diameter_pin,
                                        sanitized_tolerance_pin_pos,
                                        sanitized_tolerance_pin_neg,
                                        sanitized_sigma,
                                    );
                                    let data = Tolerance::Float(FloatTL::new(hole, pin, 3.0));
                                    //println!("{:#?}",data);
                                    entry.analysis_model = data;
                                }
                            }
                        },
                        None => {}
                    },
                    _ => {}
                };
                if let Some(tol) = tolerances.get_mut(i) {
                    // Once we've processed the entry message in the parent component, pass the
                    //  message into the entry it came from to be processed, after checking that
                    //  the entry exists.
                    tol.update(message);
                }
            }

            StackEditorAreaMessage::LabelMessage(label_message) => {
                // Pass the message into the title
                title.update(label_message);
            }
        }
    }
    pub fn view(&mut self, iss: &style::IcedStyleSheet) -> Element<StackEditorAreaMessage> {
        let StackEditor {
            filter,
            tolerances,
            scroll_state: _,
            title,
        } = self;

        let filtered_tols = tolerances
            .iter()
            .filter(|tol| filter.filter_value.matches(tol.analysis_model));

        // for each tolerance
        // add the dimension of each, and record the min and max across all iterations
        // this will set the width of the visualization
        // take the most negative value, and save the magnitude of this
        // for each tolerance, add this magnitude to the dimension, to get the ending location
        // save this for the next iteration, where this is the starting point of the next visuaal
        // the start and end coordinates of each bar are now available
        // push a spacer with a width = fillproportion(startpos)
        // push the visual with a width = fill proposrtion(endpos-startpos)

        let mut max = 0.0;
        let mut min = 0.0;
        let mut stack_total = 0.0;
        for tol in tolerances.iter() {
            stack_total += match tol.analysis_model {
                Tolerance::Linear(linear) => linear.distance.dim as f32,
                Tolerance::Float(_) => 0.0,
            };
            min = f32::min(min, stack_total);
            max = f32::max(max, stack_total);
        }

        let visualization_width = max - min;
        let mut start = min.abs();
        let mut visualize_positions: Vec<(f32, f32)> = Vec::new();

        for tol in tolerances.iter() {
            // could apply a log scale to the length here.
            let mut length = match tol.analysis_model {
                Tolerance::Linear(linear) => linear.distance.dim as f32,
                Tolerance::Float(_) => 0.0,
            };
            if length < 0.0 {
                start += length;
                length = length.abs();
            }
            visualize_positions.push((start, length));
            start += length;
        }

        // Iterate over all tols, calling their .view() function and adding them to a column
        let tolerances: Element<_> = if filtered_tols.count() > 0 {
            self.tolerances
                .iter_mut()
                .enumerate()
                .filter(|(_, tol)| filter.filter_value.matches(tol.analysis_model))
                .fold(
                    Column::new().spacing(iss.spacing(&iss.editor_tol_spacing)),
                    |column, (i, tol)| {
                        let spacer_1_len = visualize_positions[i].0.round() as u16 * 100;
                        let dim_len = visualize_positions[i].1.round() as u16 * 100;
                        let spacer_2_len = (visualization_width
                            - visualize_positions[i].0
                            - visualize_positions[i].1)
                            .round() as u16 * 100;
                        column.push(
                            //TODO add visualization here by creating a row, pushing the tol, then pushing the visualization for that row
                            Row::new()
                                .push(
                                    Container::new(tol.view(&iss).map(move |message| {
                                        // Take the message from the tolerance .view() and map it
                                        // to an `area_stack_editor` Message as an `EntryMessage`
                                        StackEditorAreaMessage::EntryMessage(i, message)
                                    }))
                                    .width(Length::FillPortion(2)),
                                )
                                .push(
                                    Container::new(
                                        Row::new()
                                            .push(if spacer_1_len > 0 {
                                                Container::new(Row::new())
                                                    .width(Length::FillPortion(spacer_1_len))
                                            } else {
                                                Container::new(Row::new())
                                            })
                                            .push(
                                                Container::new(Text::new("."))
                                                    .width(Length::FillPortion(dim_len))
                                                    .height(Length::Units(2))
                                                    .style(iss.container(&iss.visualization_contianer)),
                                            )
                                            .push(if spacer_2_len > 0 {
                                                Container::new(Row::new())
                                                    .width(Length::FillPortion(spacer_2_len))
                                            } else {
                                                Container::new(Row::new())
                                            }),
                                    )
                                    .width(Length::FillPortion(1)),
                                )
                                .spacing(iss.spacing(&iss.editor_content_spacing))
                                .align_items(Align::Center),
                        )
                    },
                )
                .into()
        } else {
            empty_message(match filter.filter_value {
                Filter::All => "There are no tolerances in the stack yet.",
                Filter::Some(tol) => match tol {
                    Tolerance::Linear(_) => "No linear tolerances in the stack.",
                    Tolerance::Float(_) => "No float tolerances in the stack.",
                },
            })
        };
        let content = Column::new()
            .spacing(iss.spacing(&iss.editor_tol_spacing))
            .push(tolerances);

        /*
        let stack_title = Text::new("Tolerance Stack")
            .width(Length::Fill)
            .size(iss.text_size(&iss.editor_title_text_size))
            .horizontal_alignment(HorizontalAlignment::Left);
        */

        let stack_title = title
            .view(&iss)
            .map(move |message| StackEditorAreaMessage::LabelMessage(message));

        let scrollable_content = Container::new(
            Scrollable::new(&mut self.scroll_state)
                .height(Length::Fill)
                .width(Length::Fill)
                .push(
                    Container::new(content).padding(iss.padding(&iss.editor_scroll_area_padding)),
                ),
        )
        .padding(iss.padding(&iss.editor_scroll_area_padding_correction))
        .style(iss.container(&iss.editor_scroll_container))
        .height(Length::Fill);

        let filter_controls = filter
            .view(&iss)
            .map(move |message| StackEditorAreaMessage::FilterMessage(message));

        let tol_stack_area = Container::new(
            Column::new()
                .push(
                    Row::new()
                        .push(stack_title)
                        .push(filter_controls)
                        .align_items(Align::Center),
                )
                .push(scrollable_content)
                .spacing(iss.spacing(&iss.editor_content_spacing))
                .max_width(1500),
        )
        .width(Length::Fill)
        .center_x();

        /*
        let new_tol_area = Container::new(
            Container::new(
                self.entry_form
                    .view(&iss)
                    .map(move |message| Message::NewEntryMessage(message)),
            )
            .padding(iss.padding(&iss.newtol_container_inner_padding))
            .style(iss.container(&iss.panel_container)),
        )
        .padding(iss.padding(&iss.newtol_container_outer_padding))
        .width(Length::Fill)
        .center_x();
        */

        tol_stack_area.into()
    }
    pub fn tolerances(&mut self, tolerances: Vec<ToleranceEntry>) -> Self {
        self.tolerances = tolerances;
        self.clone()
    }
}

fn empty_message(message: &str) -> Element<'static, StackEditorAreaMessage> {
    Container::new(
        Text::new(message)
            //.width(Length::Fill)
            .size(25)
            .horizontal_alignment(HorizontalAlignment::Center)
            .color([0.7, 0.7, 0.7]),
    )
    .width(Length::Fill)
    .height(Length::Units(200))
    .center_y()
    .center_x()
    .into()
}
