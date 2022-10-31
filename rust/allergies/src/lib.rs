use enum_iterator::IntoEnumIterator;

pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq, IntoEnumIterator, Clone, Copy)]
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
        Allergen::into_enum_iter()
            .filter(|v| self.0 & (*v as u32) != 0)
            .collect::<Vec<Allergen>>()
    }
}
