use std::borrow::Cow;

use iced::widget::{button, pick_list};
use iced::{Button, Column, Element, Text};

use crate::data::{Pokemon, PokemonData};

use super::{Action, Battle, IMessage, Page};

pub struct SelectPokemon<Next, CB> where CB: Fn(PokemonData)->Next {
    pokemon_list: Vec<PokemonData>,
    btn_state: button::State,
    list_state: pick_list::State<IndexedPokemon>,
    text: &'static str,

    on_select: CB,
}

#[derive(Clone, Debug)]
pub enum Message {
    Select(usize),
    Random,
}

#[derive(Debug, Clone)]
struct IndexedPokemon(String, usize);

impl ToString for IndexedPokemon {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl std::cmp::PartialEq for IndexedPokemon {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl std::cmp::Eq for IndexedPokemon {}

impl<Next, CB> SelectPokemon<Next, CB> where Next: Page , CB: Fn(PokemonData)->Next {
    pub fn new(pokemon_list: Vec<PokemonData>, on_select: CB ) -> Self {
        Self {
            pokemon_list,
            btn_state: Default::default(),
            list_state: Default::default(),
            text: "Select your pokemon!",
            on_select: Box::new(on_select),
        }
    }
    pub fn view(&mut self) -> iced::Element<'_, Message> {
        let indexedlist: Vec<IndexedPokemon> = self
            .pokemon_list
            .iter()
            .enumerate()
            .map(|(i, p)| IndexedPokemon(p.name.clone(), i))
            .collect();
        let cb = 
            |indexed: IndexedPokemon| Message::Select(indexed.1);
        let mut list = Column::new().spacing(10).align_items(iced::Align::Center).width(iced::Length::Fill)
            .push(Text::new(self.text))
            .push(pick_list::PickList::new(
            &mut self.list_state,
            Cow::Owned(indexedlist),
            None,
            cb,
        ));
        list = list.push(
            Button::new(&mut self.btn_state, Text::new("Random!"))
                .on_press(Message::SelectRandom(None)),
        );
        list.into()
    }
    pub fn update(&mut self, msg: Message) -> super::Action<Next> {
        let pokemon = match msg {
            Message::Select(index) => {
                self.pokemon_list[index].clone()
            }
            Message::Random => {
                rand::random::<usize>() % self.pokemon_list.len()
            }
        };
        Action::Replace(self.on_select(pokemon))
    }
}

impl IMessage for Message {}

impl<Next, CB> Page for SelectPokemon<Next, CB> where Next: Page {
    crate::page! {
        Message = Message;
        @update msg => self.update(msg);
        @view => self.view();
    }
}
