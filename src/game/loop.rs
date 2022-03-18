use crate::core::saisie_service::SaisieService;
use crate::model::produit::Produit;
use crate::model::enum_etat_saisie::EtatSaisie;

use crate::model::hello::HELLO_WORLD;
use crate::game::services::saisie_clavier_service::SAISIE_CLAVIER_SERVICE;

pub fn gameloop() {
    let porsche911: Produit = Produit { nom: String::from("porsche911"), cout: 122000 };
    let mut run: bool = true;

    while run {

        // demande du prix a l'utilisateur
        println!("dite moi un prix (saisir nombre entre 10.000 et 200.000) : ");
        let prix_choisi: i32 = nombre_saisi();

        // analyse de prix
        let info: String = plus_ou_moins(prix_choisi, &porsche911).to_string();

        // feedback sur l'analyse
        println!("c'est {}", info);

        // détermination de la fin de partie
        if est_trouve(info) {
            run = false;
        }
    }

    println!("{}", HELLO_WORLD);
    println!("bravo");
}

fn plus_ou_moins(prix_choisi: i32, produit: &Produit) -> EtatSaisie {
    match prix_choisi {
        n if (n > produit.cout) => EtatSaisie::MOINS,
        n if (n < produit.cout) => EtatSaisie::PLUS,
        _ => EtatSaisie::TROUVE
    }
}

fn est_trouve(value: String) -> bool {
    value == EtatSaisie::TROUVE.to_string()
}

fn stof(chaine: String) -> i32 {
    println!("chaine a transformer : {}", chaine);
    chaine
        .to_string()
        .parse::<i32>()
        .unwrap()
}

fn nombre_saisi() -> i32 {
    stof(SAISIE_CLAVIER_SERVICE.saisie())
}