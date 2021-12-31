use iced::Element;

mod main_menu;
pub use main_menu::Menu;

mod battle;
pub use battle::Battle;

mod result_page;
pub use result_page::ResultPage;

mod select_pokemon;
pub use select_pokemon::SelectPokemon;

mod dialog;
pub use dialog::Dialog;

mod edit_pokemon;

pub enum Action<P> {
    Pop,
    Push(P),
    Replace(P),
    Continue,
}

impl<P> Action<P> {
    fn map<F, B>(self, f: F) -> Action<B>
    where
        F: FnOnce(P) -> B,
    {
        match self {
            Self::Push(val) => Action::Push(f(val)),
            Self::Replace(val) => Action::Replace(f(val)),
            Self::Pop => Action::Pop,
            Self::Continue => Action::Continue,
        }
    }
}

pub struct AnyPage(pub Box<dyn Page>);

struct _AssertPageBoxable(dyn Page);

pub trait IMessage: std::any::Any + Send + std::fmt::Debug {}

pub trait Page {
    fn update(&mut self, message: Box<dyn crate::pages::IMessage>) -> Action<AnyPage>;
    fn view<'a>(&'a mut self) -> Element<'a, crate::Message>;
}

impl Page for () {
    fn update(&mut self, _: Box<dyn crate::pages::IMessage>) -> Action<AnyPage> {
        unreachable!()
    }

    fn view<'a>(&'a mut self) -> Element<'a, crate::Message> {
        unreachable!()
    }
}

pub enum Either<A,B> where A: Page, B: Page {
    A(A),
    B(B)
}

impl<A, B> Page for Either<A, B> where A: Page, B: Page {
    fn update(&mut self, message: Box<dyn crate::pages::IMessage>) -> Action<AnyPage> {
        match self {
            Either::A(page) => page.update(message),
            Either::B(page) => page.update(message),
        }
    }

    fn view<'a>(&'a mut self) -> Element<'a, crate::Message> {
        match self {
            Either::A(page) => page.view(),
            Either::B(page) => page.view(),
        }
    }
}

#[macro_export]
macro_rules! page {
    // Main uses
    //// Most General use
    { impl Page for $type:ty where Message = $msg:ty {
        @update $msg_name:ident => $update:expr;
        @view => $view:expr;
    } } => {
        crate::page!(@impl $type, $msg, $msg_name, $update, $view);
    };

    { impl Page for $type:ty where Message = $msg:ty {
        @update $msg_name:ident => $update:expr;
        @view => $view:expr;
    } } => {
        crate::page!(@impl $type, $msg, $msg_name, $update, $view);
    };

    //// Method use
    { Message = $msg:ty; 
        @update $msg_name:ident => $update:expr;
        @view => $view:expr;
        
    } => {
        
    };

    // Internal Uses
    (@impl $type:ty, $msg:ty, $msg_name:ident, $update:expr, $view:expr) => {
        impl crate::pages::IMessage for $msg {}
        impl crate::pages::Page for $type {
            page!(@impl_methods $msg, $msg_name, $update, $view)
        }
    };

    (@impl_methods $msg:ty, $msg_name:ident, $update:expr, $view:expr) => {
        page!(@update $msg, $msg_name, $update);
        page!(@view $msg, $view);
    };

    (@update $msg:ty, $msg_name:ident, $blo:expr) => {
        fn update(&mut self, $msg_name: Box<dyn crate::pages::IMessage>) -> crate::pages::Action<crate::pages::AnyPage> {
            let $msg_name =  *Box::<dyn std::any::Any>::downcast::<$msg>($msg_name).unwrap();
            let _action: crate::pages::Action<_> = $blo;
           _action.map(|p| crate::pages::AnyPage(Box::new(p)))
        }
    };

    (@view $msg:ty, $blo:expr) => {
        fn view<'a>(&'a mut self) -> ::iced::Element<'a, crate::Message> {
           let el: iced::Element<$msg> = $blo;
           use crate::widget::ElementExt;
           iced::Element::from(el).padding(10).map(|msg| crate::Message::Custom(Box::new(msg)))
        }
    };

}
