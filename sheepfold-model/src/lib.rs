use serde::{Deserialize, Serialize};

use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
    random, rng,
};

use random_names::RandomName;
use strum::EnumString;
use uuid::Uuid;

pub type SheepId = Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString)]
pub enum SheepColor {
    Black,
    White,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sheep {
    id: SheepId,
    name: String,
    weight: u64,
    color: SheepColor,
}

pub struct SheepBuilder {
    name: Option<String>,
    weight: Option<u64>,
    color: Option<SheepColor>,
}

impl SheepBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            weight: None,
            color: None,
        }
    }

    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn with_weight(&mut self, value: u64) -> &mut Self {
        self.weight = Some(value);
        self
    }

    pub fn with_color(&mut self, value: SheepColor) -> &mut Self {
        self.color = Some(value);
        self
    }

    pub fn build(self) -> Sheep {
        Sheep {
            id: Uuid::new_v4(),
            name: self.name.or_else(|| Some(RandomName::new().name)).unwrap(),
            weight: self
                .weight
                .or_else(|| Some(rng().random_range(20..100)))
                .unwrap(),
            color: self.color.or_else(|| Some(random())).unwrap(),
        }
    }
}

impl Sheep {
    pub fn is_danny(&self) -> bool {
        self.color == SheepColor::Black && self.name == String::from("Danny")
    }

    pub fn is_heavy(&self) -> bool {
        self.weight > 40
    }

    pub fn id(&self) -> &SheepId {
        &self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Distribution<SheepColor> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SheepColor {
        match rng.random_range(0..2) {
            0 => SheepColor::Black,
            _ => SheepColor::White,
        }
    }
}
