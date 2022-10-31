use enum_iterator::{all, Sequence};

pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq, Sequence)]
#[repr(u32)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & (*allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        all::<Allergen>()
            .filter(|v| self.0 & (*v as u32) != 0)
            .collect::<Vec<Allergen>>()
    }
}
