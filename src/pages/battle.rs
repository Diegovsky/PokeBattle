use iced::{Button, Column, Element, Length, ProgressBar, Row, Scrollable, Space, Text, button, scrollable};

use crate::{data::{Pokemon, PokemonData}, page, widget::{centered_column, centered_row}};

use super::{Action, IMessage, result_page::ResultPage};

pub struct Battle {
    selected_move: usize,
    last_move: Option<String>,
    player: PokemonShowcase,
    other: PokemonShowcase,

    scroll_state: scrollable::State,
}

struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn new(current: f64, max: f64) -> Self {
        Self {
            current: current as f32,
            max: max as f32,
        }
    }
    pub fn widget(&self) -> Row<'static, Message> {
        centered_row().push(ProgressBar::new(0.0..=self.max, self.current))
            .push(Text::new(format!("[{:.2}/{:.2}]", self.current, self.max)))
    }
}

impl Into<Element<'static, Message>> for Health {
    fn into(self) -> Element<'static, Message> {
        self.widget().into()
    }
}

#[derive(Debug, Clone, Copy)]
struct Message(usize);


struct PokemonShowcase {
    model: Pokemon,
    attack_state: [button::State; 4],
    buttons_enabled: bool,
}

impl std::ops::Deref for PokemonShowcase {
    type Target = Pokemon;
    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl std::ops::DerefMut for PokemonShowcase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.model
    }
}

impl PokemonShowcase {
    fn new(pokemon: Pokemon, buttons_enabled: bool) -> Self {
        Self {
            model: pokemon,
            attack_state: [Default::default(); 4],
            buttons_enabled
        }
    }
    fn push_attacks<'a>(&'a mut self, mut col: Column<'a, Message>) -> Column<'a, Message> {
        let separator = 2;
        let mut row = Row::new().spacing(10);
        for (i, (atk, state)) in self.model.attacks.iter().zip(self.attack_state.iter_mut()).enumerate() {
            let label = Text::new(atk.label());
            let el: Element<'a, Message> = if self.buttons_enabled {
                Button::new(state, label)
                    .on_press(Message(i))
                    .into()
            } else {
                label.into()
            };
            row = row.push(el);

            if (i+1) % separator == 0 {
                col = col.push(row);
                row = Row::new();
            }
        }
        col
    }
    pub fn widget<'a>(&'a mut self) -> Element<'a, Message> {
        let col = centered_column().align_items(iced::Align::Start)
            .push(Text::new(self.model.name.clone()))
            .push(Health::new(self.model.current_health, self.model.health).widget());
        self.push_attacks(col)
            .into()
    }
}

use crate::widget::ElementExt;

impl Battle {
    pub fn new(player: PokemonData, other: PokemonData) -> Self {
        Self {
            selected_move: 0,
            last_move: None,
            player: PokemonShowcase::new(Pokemon::from_data(player), true),
            other: PokemonShowcase::new(Pokemon::from_data(other), false),

            scroll_state: Default::default(),
        }
    }
    fn view<'a>(&'a mut self) -> Element<'a, Message> {
        println!("other: {:?}", self.other.name);
        let scrol = Scrollable::new(&mut self.scroll_state);
        let mut col = centered_column()
            .push(self.player.widget())
            .push(self.other.widget());
        if let Some(last_move) = self.last_move.take() {
            col = col.push(Text::new(format!("Oponent's last move: {}", last_move)));
        }
        scrol.push(col).into()
    }
    fn update(&mut self, msg: Message) -> Action<ResultPage> {
        self.selected_move = msg.0;
        let attack = &self.player.attacks[self.selected_move];
        if self.other.receive(attack) {
            Action::Replace(ResultPage::new("You win!"))
        } else {
            let i = rand::random::<usize>() % self.other.attacks.len();
            let attack = &self.other.attacks[i];
            self.last_move = Some(attack.label());
            if self.player.receive(attack) {
                Action::Replace(ResultPage::new("You lose!"))
            } else {
                Action::Continue
            }
        }
    }
}

page! {
    impl Page for Battle where Message = Message {
        @update msg => self.update(msg);
        @view => self.view();
    }
}
