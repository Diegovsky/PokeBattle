use iced::{Column, Container, Element, Length, Row};

use crate::pages::IMessage;

/* pub struct Center<'a, T> {
    inner: Row<'a, Column<'a, T>>
}

impl<'a, T: 'a> Center<'a, T> {
    pub fn new(val: T) -> Self {
        Self {
            inner: Row::new().align_items(iced::Align::Center).push(Column::new().align_items(iced::Align::Center))
                .push(val)
        }
    }
} */

pub trait ElementExt<'a, Msg> {
    fn padding(self, padding: u16) -> Element<'a, Msg>;
}

impl<'a, Msg> ElementExt<'a, Msg> for Element<'a, Msg> where Msg: IMessage {
    fn padding(self, padding: u16) -> Element<'a, Msg> {
        Container::new(self)
            .padding(padding)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}


pub fn centered_column<'a, Message>() -> Column<'a, Message> {
    Column::new()
        .align_items(iced::Align::Center)
        .spacing(10)
}

pub fn centered_row<'a, Message>() -> Row<'a, Message> {
    Row::new()
        .align_items(iced::Align::Center)
        .spacing(10)
}
