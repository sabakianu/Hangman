use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
struct Date {
    incepator: Vec<String>,
    intermediar: Vec<String>,
    avansat: Vec<String>,
    expert: Vec<String>,
}

impl Date {
    fn new() -> Self {
        Self {
            incepator: vec![
                "casa".to_string(),
                "masa".to_string(),
                "luna".to_string(),
                "soare".to_string(),
                "copac".to_string(),
                "apa".to_string(),
                "foc".to_string(),
                "nor".to_string(),
                "vânt".to_string(),
                "cer".to_string(),
            ],
            intermediar: vec![
                "floare".to_string(),
                "frunze".to_string(),
                "umbrela".to_string(),
                "carte".to_string(),
                "lumina".to_string(),
                "mesteacan".to_string(),
                "fereastra".to_string(),
                "zapada".to_string(),
                "pian".to_string(),
                "cheie".to_string(),
            ],
            avansat: vec![
                "televizor".to_string(),
                "calculator".to_string(),
                "fericire".to_string(),
                "melancolie".to_string(),
                "programare".to_string(),
                "universitate".to_string(),
                "cameleon".to_string(),
                "paralelipiped".to_string(),
                "electricitate".to_string(),
                "fotografie".to_string(),
            ],
            expert: vec![
                "subterfugiu".to_string(),
                "hipopotomonstrosesquipedaliofobie".to_string(),
                "electromagnetism".to_string(),
                "deoxiribonucleic".to_string(),
                "anticonstitutional".to_string(),
                "transcendental".to_string(),
                "paleoastronautica".to_string(),
                "extraterestru".to_string(),
                "neurostiinta".to_string(),
                "filozofie".to_string(),
            ],
        }
    }
}
fn random(lista: &Vec<String>) -> &str {
    let mut generator = thread_rng();
    let cuvant_ales = lista.choose(&mut generator);
    match cuvant_ales {
        Some(cuvant) => cuvant,
        None => panic!("Lista este goală!"),
    }
}
fn choose_dificultate() -> String {
    let date = Date::new();
    println!("Salut, alege o dificultate pt joc:");
    println!("A: expert");
    println!("B: avansat");
    println!("C: intermediar");
    println!("D: incepator");
    let dificultate = citeste();
    let mut cuvant = String::new();
    match dificultate {
        'A' => cuvant.push_str(random(&date.expert)),
        'B' => cuvant.push_str(random(&date.avansat)),
        'C' => cuvant.push_str(random(&date.intermediar)),
        'D' => cuvant.push_str(random(&date.incepator)),
        _ => panic!("Nu ai introdus o dificultate buna!"),
    }
    cuvant
}
fn citeste() -> char {
    let mut input = String::new();
    let character;
    println!("Introdu un caracter: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Eroare la citirea inputului");
    let caracter = input.trim().chars().next();
    match caracter {
        Some(c) => character = c,
        None => character = '_',
    }
    character
}

fn modifica(modificat: &mut String, cuvant: &String, ch: char) {
    for (index, character) in cuvant.chars().enumerate() {
        if ch == character {
            modificat.replace_range(index..index + 1, &ch.to_string());
        }
    }
}

fn joc(cuvant: &String) {
    let mut litere_folosite = String::new();
    let mut de_ghicit = String::new();
    let mut gresit: u32 = 0;
    let lungime = cuvant.len();
    let prima = cuvant.chars().next();
    match prima {
        Some(litera) => {
            litere_folosite.push(litera);
        }
        None => panic!("EROARE"),
    }
    let ultima = cuvant.chars().last();
    match ultima {
        Some(litera) => {
            litere_folosite.push(litera);
        }
        None => panic!("EROARE"),
    }
    for i in 0..lungime {
        if i == 0 {
            let prima = cuvant.chars().next();
            match prima {
                Some(litera) => {
                    litere_folosite.push(litera);
                }
                None => panic!("EROARE"),
            }
        } else if i == lungime - 1 {
            let ultima = cuvant.chars().last();
            match ultima {
                Some(litera) => {
                    litere_folosite.push(litera);
                }
                None => panic!("EROARE"),
            }
        }
        de_ghicit.push('_');
    }
    for i in litere_folosite.chars() {
        modifica(&mut de_ghicit, cuvant, i);
    }
    while de_ghicit.contains('_') {
        println!("Cuvant: {de_ghicit}");
        println!("Greseli: {gresit}");
        let litera = citeste();
        if !litere_folosite.contains(litera) {
            litere_folosite.push(litera);
            if cuvant.contains(litera) {
                modifica(&mut de_ghicit, cuvant, litera);
            } else {
                gresit += 1;
                if gresit == 3 {
                    println!("Ai pierdut!");
                    println!("Cuvantul era {cuvant}");
                    return;
                }
            }
        }
    }
    println!("Cuvant: {de_ghicit}");
    println!("Ai castigat!");
}
fn main() {
    let cuvant = choose_dificultate();
    joc(&cuvant);
}
