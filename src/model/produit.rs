trait Couteux {
    fn prix(&self) -> i32;
}

pub struct Produit {
    pub nom: String,
    pub cout: i32
}

impl Couteux for Produit {
    fn prix(&self) -> i32 {
        self.cout
    }
}