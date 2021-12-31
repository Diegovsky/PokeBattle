use crate::data::PokemonData;


#[derive(Debug, Clone)]
pub struct EditPokemon {
    list: Vec<PokemonData>
}

#[derive(Debug, Clone)]
enum Message {
    NoneSelected,
    Selected(PokemonData),
    Edited,
    Cancel
}

crate::page!{ 
    impl Page for EditPokemon where Message = Message {
        @update msg => {
            match msg {
                Message::NoneSelected
            }
        };
        @view => {

        };
    }
}
