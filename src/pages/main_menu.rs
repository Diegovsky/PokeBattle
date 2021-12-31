use iced::{Button, Column, Text, button, };

use crate::{data::{self, PokemonData, deserialize_pokemon}, widget::centered_column};

use super::{Action, Dialog, Page, SelectPokemon, Either};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    StartGame,
    Exit,
}

pub struct Option {
    pub text: &'static str,
    state: button::State,
    msg: Message,
}

impl Option {
    pub fn new(text: &'static str, msg: Message) -> Self {
        Self {
            text,
            state: button::State::new(),
            msg,
        }
    }

    pub fn as_label(&mut self) -> Button<Message> {
        Button::new(&mut self.state, Text::new(self.text)).on_press(self.msg)
    }
}

pub struct Menu {
    options: [Option; 2],
}

impl Menu {
    pub fn new() -> Self {
        Self {
            options: [
                Option::new("Play game", Message::StartGame),
                Option::new("Exit", Message::Exit),
            ],
        }
    }
    pub fn view(&mut self) -> Column<Message> {
        let mut col = centered_column()
            .push(Text::new("Welcome to USUC!"));
        for ele in self.options.iter_mut() {
            col = col.push(ele.as_label())
        }
        col.width(iced::Length::Fill)
    }
    pub fn update(&mut self, msg: Message) -> Action<Dialog<Vec<PokemonData>, anyhow::Error>> {
        match msg {
            Message::StartGame => {
                Action::Push(Dialog::new("Where is your pokemon.yaml path?", "pokemon.yaml", |path| deserialize_pokemon(path)))
            }
            Message::Exit => Action::Pop,
        }
    }
}

crate::page! {
    impl Page for Menu where Message = Message {
        @update msg => { self.update(msg) };
        @view => { self.view() };
    }
}
