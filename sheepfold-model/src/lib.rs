use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
    random, rng,
};

use random_names::RandomName;
use serde::{Deserialize, Serialize};
use strum::EnumString;
use toasty::stmt::Uuid;

#[derive(Debug, PartialEq, toasty::Embed, EnumString, Serialize, Deserialize)]
pub enum SheepColor {
    Black,
    White,
}

#[derive(Debug, toasty::Model, Serialize, Deserialize)]
#[table = "sheepfold"]
pub struct Sheep {
    #[key]
    #[auto]
    id: Uuid,

    #[default(RandomName::new().name)]
    name: String,

    #[default(rng().random_range(20..100))]
    weight: u64,

    #[default(random::<SheepColor>())]
    color: SheepColor,
}

impl Distribution<SheepColor> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SheepColor {
        match rng.random_range(0..2) {
            0 => SheepColor::Black,
            _ => SheepColor::White,
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

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}
