enum WineGrapes {
    CabernetFranc,
    Tannat,
    Merlot,
}

fn taste_wine(grapes: WineGrapes) {
    match grapes {
        WineGrapes::CabernetFranc => println!("This is a Cabertnet Franc wine."),
        // WineGrapes::Tannat => println!("This is a Tannat wine."),
        // WineGrapes::Merlot => println!("This is a Merlot wine."),
    }
}

fn main() {
    taste_wine(WineGrapes::CabernetFranc);
}

/*
Exhaustiveness refers to the requirement that match expressions must cover all possible variants of an enum, or all valid values of the matched type. This is a key feature that ensures safety and prevents bugs due to missing cases.

Why Does Rust Enforce Exhaustiveness?
      - Prevents logical errors: No missing cases.
      - Increases safety: Compiler forces you to handle all possible values.
      - Avoids runtime panics: Rust ensures code is correct at compile time.
*/