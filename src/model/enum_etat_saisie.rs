pub enum EtatSaisie {
    PLUS,
    MOINS,
    TROUVE
}

impl EtatSaisie {
    fn as_str(&self) -> &str {
        match self {
            EtatSaisie::PLUS => "plus",
            EtatSaisie::MOINS => "moins",
            EtatSaisie::TROUVE => "trouve"
        }
    }

    pub fn to_string(&self) -> String {
        String::from(self.as_str())
    }
}