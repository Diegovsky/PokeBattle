
use iced::{Button, Column, Element, Text, TextInput, text_input, widget::button};

use crate::{data::deserialize_pokemon, widget::centered_column};

use super::{Action, Battle, Page, SelectPokemon};

pub struct Dialog<T, E> {
    prompt: String,

    error: Option<anyhow::Error>,
    state: String,

    arg: String,
    validator: Box<dyn Fn(String)->Result<T, E>>,

    btn_state: button::State,
    input_state: text_input::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    TypedText(String),
    Try,
}

impl<T, E> Dialog<T, E> where E: std::fmt::Display {
    pub fn new<S: Into<String>, V: Fn(String)->Result<T, E>>(prompt: S, arg: S, validator: V) -> Self {
        Self {
            prompt: prompt.into(),

            arg: arg.into(),
            validator: Box::new(validator),

            error: None,
            state: String::with_capacity(56),

            btn_state: Default::default(),
            input_state:Default::default(),
        }
    }
    pub fn update(&mut self, msg: Message) -> Action<SelectPokemon<SelectPokemon<Battle>>> {
        match msg {
            Message::TypedText(txt) => { self.state = txt; Action::Continue },
            Message::Try => {
                match self.validator(std::mem::take(&mut self.state)) {
                    Ok(list) => Action::Replace(SelectPokemon::new("Select your pokemon!", list.clone(), |player| {
                        SelectPokemon::new("Select your oponent!", list, |other| {
                            Battle::new(player, other)
                        })
                    })),
                    Err(err) => { self.error = Some(err); Action::Continue },
                }
            }
        }
    }
    pub fn view(&mut self) -> Element<'_, Message> {
        let mut col = centered_column()
            .push(Text::new(self.prompt.clone()))
            .push(
            TextInput::new(&mut self.input_state, &self.placeholder, &self.state, |text| Message::TypedText(text))
                .on_submit(Message::Try));
        if let Some(err) = self.error.take() {
            col = col.push(Text::new(err.to_string()))
        }
        col.push(Button::new(&mut self.btn_state, Text::new("Ok"))
            .on_press(Message::Try))
            .into()
    }
}

impl<T, E> Page for Dialog<T, E> {
crate::page! {
        Message = Message;
        @update msg => self.update(msg);
        @view => self.view();
    }
}
