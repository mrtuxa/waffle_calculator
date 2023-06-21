use std::io;

fn berechne_waffel_zutaten(anzahl_waffeln: i32) {
    // Portionsgröße für 10 Waffeln
    let faktor = anzahl_waffeln as f64 / 10.0;

    // Zutatenmengen berechnen
    let eier = (4.0 * faktor) as i32;
    let zucker = (200.0 * faktor) as i32;
    let butter = (250.0 * faktor) as i32;
    let vanillezucker = (2.0 * faktor) as i32;
    let backpulver = (1.0 * faktor) as i32;
    let milch = (200.0 * faktor) as i32;
    let mehl = (400.0 * faktor) as i32;
    let club_mate = (100.0 * faktor) as i32;
    let rum = 0; // optional, nach Belieben hinzufügen
    let salz = (1.0 * faktor) as i32;

    // Ausgabe der Zutatenmengen
    println!("Für {} Waffeln werden folgende Zutaten benötigt:", anzahl_waffeln);
    println!("Eier: {} Stück", eier);
    println!("Zucker: {} g", zucker);
    println!("Butter: {} g (weich oder zerlassen)", butter);
    println!("Vanillezucker: {} EL", vanillezucker);
    println!("Backpulver: {} EL", backpulver);
    println!("Milch: {} ml", milch);
    println!("Mehl: {} g", mehl);
    println!("Club-Mate: {} ml (alternativ 100 Sprudel)", club_mate);
    println!("Rum: nach Belieben");
    println!("Salz: {} TL", salz);
}

fn main() {
    println!("Geben Sie die Anzahl der gewünschten Waffeln ein:");

    let mut anzahl = String::new();
    io::stdin()
        .read_line(&mut anzahl)
        .expect("Fehler beim Lesen der Eingabe.");

    let anzahl: i32 = anzahl.trim().parse().expect("Ungültige Eingabe.");

    berechne_waffel_zutaten(anzahl);
}

