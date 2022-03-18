use std::io;
use crate::core::saisie_service::SaisieService;

pub struct SaisieClavierService {}

impl SaisieService for SaisieClavierService {
    fn saisie(&self) -> String {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("erreur lors de la saisie utilisateur");

        // on enleve le \n ajouter au moment de la saisie utilisateur
        input.replace("\n", "")
    }
}

pub static SAISIE_CLAVIER_SERVICE: SaisieClavierService = SaisieClavierService {};