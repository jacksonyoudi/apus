#[derive(Clone)]
pub struct Letter {
    text: String,
}

pub struct PickupLorryHandle {
    done: bool,
}

impl Letter {
    pub fn new(text: String) -> Self {
        Letter {
            text,
        }
    }
}

pub struct Envelope {
    letter: Option<Letter>,
}

impl Envelope {
    pub fn wrap(&mut self, letter: &Letter) {
        self.letter = Some(letter.clone());
    }
}

pub fn buy_prestamped_envelope() -> Envelope {
    Envelope { letter: None }
}


impl PickupLorryHandle {
    pub fn pickup(&mut self, envelope: &Envelope) {
        
    }
}
