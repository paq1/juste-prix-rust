use std::io;

pub trait SaisieService {
    fn saisie() -> String;
}

pub fn saisie_clavier() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("erreur lors de la saisie utilisateur");

    // on enleve le \n ajouter au moment de la saisie utilisateur
    input.replace("\n", "")
}