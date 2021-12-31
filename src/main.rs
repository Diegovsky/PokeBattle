#![feature(trait_upcasting)]
#![windows_subsystem = "windows"]

use iced::{Element, Sandbox, Settings};


mod data;
mod pages;
mod widget;

use pages::{Action, Page};

pub struct Usuc {
    pages: Vec<Box<dyn Page>>,
}

#[derive(Debug)]
pub enum Message {
    Custom(Box<dyn pages::IMessage>),
}

impl Sandbox for Usuc {
    type Message = Message;
    fn new() -> Self {
        Self {
            pages: vec![Box::new(pages::Menu::new())],
        }
    }
    fn title(&self) -> String {
        "Pokemonas".into()
    }
    fn update(&mut self, msg: Message) {
        match msg {
            Message::Custom(msg) => {
                if let Some(page) = self.pages.last_mut() {
                    match page.update(msg) {
                        Action::Pop => {
                            self.pages.pop();
                        }
                        Action::Push(page) => self.pages.push(page.0),
                        Action::Replace(page) => *(self.pages.last_mut().unwrap()) = page.0,
                        Action::Continue => (),
                    };
                }
            }
        }
    }
    fn view(&mut self) -> Element<Message> {
        if let Some(page) = self.pages.last_mut() {
            page.view()
        } else {
            panic!("Ended")
        }
    }
}

fn main() {
    <Usuc as Sandbox>::run(Settings::default()).unwrap();
}
