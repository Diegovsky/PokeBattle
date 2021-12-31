use iced::{button, Button, Element, Text};

use crate::widget::centered_column;

use super::Action;

pub struct ResultPage {
    text: &'static str,

    btn_state: button::State,
}

#[derive(Debug, Copy, Clone)]
struct Message {}

impl ResultPage {
    pub fn new(text: &'static str) -> Self {
        Self {
            text,
            btn_state: Default::default(),
        }
    }
    pub fn update(&mut self, _: Message) -> Action<()> {
        Action::Pop
    }
    pub fn view<'a>(&'a mut self) -> Element<'a, Message> {
        centered_column()
            .push(Text::new(self.text))
            .push(Button::new(&mut self.btn_state, Text::new("Great!")).on_press(Message {}))
            .into()
    }
}

crate::page! {
    impl Page for ResultPage where Message = Message {
        @update msg => self.update(msg);
        @view => self.view();
    }
}
