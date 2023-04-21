pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Self> {
        if self.health > 0 {
            return None;
        }

        let mana = if self.level < 10 { None } else { Some(100) };

        Some(Self {
            health: 100,
            mana,
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana < mana_cost {
                0
            } else {
                self.mana = Some(mana - mana_cost);
                mana_cost * 2
            }
        } else {
            self.health -= if self.health < mana_cost {
                self.health
            } else {
                mana_cost
            };
            0
        }
    }
}
