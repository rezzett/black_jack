use rand::{Rng, thread_rng};

pub struct Player {
    pub name: String,
    pub score: usize,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player { name: name.to_string(), score: 0 }
    }

    pub fn take_card(&mut self, deck: &mut Vec<usize>) -> usize {
        let rand_card = thread_rng().gen_range(0, deck.len());
        let rand_card = deck.remove(rand_card);
        self.score += rand_card;
        rand_card
    }

    pub fn is_overflow(&self) -> bool {
        self.score > 21
    }
}