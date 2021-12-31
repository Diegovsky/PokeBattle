use std::{
    borrow::Cow,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(serde::Deserialize, Clone, PartialEq, Debug)]
pub struct Attack {
    name: String,
    damage: f64,
}

impl Attack {
    pub fn new<S: Into<String>>(name: S, damage: f64) -> Self {
        Self {
            name: name.into(),
            damage,
        }
    }
    pub fn label(&self) -> String {
        format!("{} ({} damage)", self.name, self.damage)
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct PokemonData {
    pub name: String,
    pub health: f64,
    pub attacks: [Attack; 4],
}

#[derive(Clone)]
pub struct Pokemon {
    data: PokemonData,
    pub current_health: f64,
}

impl std::ops::Deref for Pokemon {
    type Target = PokemonData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl std::ops::DerefMut for Pokemon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Pokemon {
    pub fn from_data(data: PokemonData) -> Self {
        Self {
            current_health: data.health,
            data,
        }
    }
    pub fn percentage(&self) -> f64 {
        self.current_health as f64 / self.health as f64
    }
    pub fn receive(&mut self, attack: &Attack) -> bool {
        if self.current_health <= attack.damage {
            return true;
        }
        self.current_health -= attack.damage.clamp(-self.health, self.health);
        false
    }
}

impl ToString for Pokemon {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

pub fn deserialize_pokemon<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<PokemonData>> {
    let reader = std::fs::File::open(path)?;
    Ok(serde_yaml::from_reader(reader)?)
}
