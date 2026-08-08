// Esercizi sul capitolo "Named Structs" di Comprehensive Rust.
// https://google.github.io/comprehensive-rust/user-defined-types/named-structs.html
//
// Completa ogni funzione al posto di `todo!()`, poi esegui:
//   cargo test
// per verificare le tue soluzioni. I test sono già scritti: non modificarli,
// servono da specifica.

/// Esercizio 1 — Definizione e istanziazione esplicita
///
/// Uno struct con campi con nome si definisce con
/// `struct Nome { campo: Tipo, ... }` e si istanzia con
/// `Nome { campo: valore, ... }`.
#[derive(Debug, PartialEq)]
struct Persona {
    nome: String,
    eta: u8,
}

/// Costruisci una `Persona` usando la sintassi esplicita `campo: valore`.
fn crea_persona(nome: &str, eta: u8) -> Persona {
    let persona = Persona {
        nome: nome.to_string(),
        eta: eta
    };
    persona
}

/// Esercizio 2 — Field init shorthand
///
/// Quando una variabile si chiama esattamente come il campo, puoi scrivere
/// solo il nome invece di `campo: campo`.
///
/// Costruisci una `Persona` usando la shorthand syntax (qui `nome` è già una
/// `String` e `eta` un `u8`, con i nomi giusti).
fn crea_persona_shorthand(nome: String, eta: u8) -> Persona {
    
    let persona = Persona {
        nome,
        eta
    };
    persona
}

/// Esercizio 3 — Accesso e mutazione dei campi
///
/// I campi si leggono/scrivono con `variabile.campo`. Per modificarli la
/// variabile (o il riferimento) deve essere `mut`.
///
/// Scrivi una funzione che faccia "festeggiare il compleanno" a una persona,
/// incrementando `eta` di 1, e restituisca il nuovo valore di `eta`.
fn festeggia_compleanno(persona: &mut Persona) -> u8 {
    persona.eta += 1;
    persona.eta
}

/// Esercizio 4 — Struct update syntax (`..`)
///
/// `Nome { campo: nuovo_valore, ..vecchia_istanza }` copia da
/// `vecchia_istanza` tutti i campi che non specifichi esplicitamente.
#[derive(Debug, PartialEq, Clone)]
struct Rettangolo {
    larghezza: f64,
    altezza: f64,
    colore: String,
}

/// Restituisci un nuovo `Rettangolo` con la stessa larghezza e altezza di
/// `rett`, ma con `colore` uguale a "rosso". Usa la struct update syntax.
fn colora_di_rosso(rett: &Rettangolo) -> Rettangolo {
    let nuovo_rettangolo = Rettangolo {
        colore: String::from("rosso"),
        ..*rett
    };
    nuovo_rettangolo
}

/// Esercizio 5 — Passare struct per riferimento
///
/// Passare uno struct per `&riferimento` a una funzione evita di spostarne
/// (move) la proprietà.
///
/// Scrivi una funzione che calcoli l'area di un `Rettangolo` senza prenderne
/// possesso.
fn area(rett: &Rettangolo) -> f64 {
    rett.larghezza * rett.altezza
}

/// Esercizio 6 (bonus) — Tuple struct
///
/// I tuple struct hanno campi senza nome, accessibili con `.0`, `.1`, ecc.
/// Sono utili quando il nome del campo non aggiungerebbe informazione (es.
/// un wrapper attorno a un singolo valore).
#[derive(Debug, PartialEq)]
struct Metri(f64);

/// Converti dei metri in centimetri (1 m = 100 cm).
fn metri_in_centimetri(m: Metri) -> f64 {
    m.0 *100.0
}

/// Esercizio 7 (bonus) — Struct a dimensione zero
///
/// Uno struct senza campi non occupa memoria a runtime; si usa spesso come
/// "marcatore" (marker), tipicamente per implementare un trait senza avere
/// bisogno di dati.
#[derive(Debug, PartialEq)]
struct Marcatore;

/// Crea un'istanza di `Marcatore`.
fn crea_marcatore() -> Marcatore {
    Marcatore
}

fn main() {
    println!("Completa gli esercizi in questo file, poi lancia `cargo test`.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crea_persona() {
        let p = crea_persona("Marco", 30);
        assert_eq!(p, Persona { nome: "Marco".to_string(), eta: 30 });
    }

    #[test]
    fn test_crea_persona_shorthand() {
        let p = crea_persona_shorthand("Anna".to_string(), 25);
        assert_eq!(p, Persona { nome: "Anna".to_string(), eta: 25 });
    }

    #[test]
    fn test_festeggia_compleanno() {
        let mut p = Persona { nome: "Luca".to_string(), eta: 40 };
        let nuova_eta = festeggia_compleanno(&mut p);
        assert_eq!(nuova_eta, 41);
        assert_eq!(p.eta, 41);
    }

    #[test]
    fn test_colora_di_rosso() {
        let r = Rettangolo { larghezza: 3.0, altezza: 4.0, colore: "blu".to_string() };
        let r2 = colora_di_rosso(&r);
        assert_eq!(r2.larghezza, 3.0);
        assert_eq!(r2.altezza, 4.0);
        assert_eq!(r2.colore, "rosso");
        // l'originale non deve essere modificato
        assert_eq!(r.colore, "blu");
    }

    #[test]
    fn test_area() {
        let r = Rettangolo { larghezza: 3.0, altezza: 4.0, colore: "verde".to_string() };
        assert_eq!(area(&r), 12.0);
    }

    #[test]
    fn test_metri_in_centimetri() {
        assert_eq!(metri_in_centimetri(Metri(2.5)), 250.0);
        assert_eq!(metri_in_centimetri(Metri(0.0)), 0.0);
    }

    #[test]
    fn test_crea_marcatore() {
        assert_eq!(crea_marcatore(), Marcatore);
    }
}
