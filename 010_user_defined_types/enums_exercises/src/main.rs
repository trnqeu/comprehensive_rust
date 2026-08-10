// Esercizi sul capitolo "Enums" di Comprehensive Rust.
// https://google.github.io/comprehensive-rust/user-defined-types/enums.html
//
// Completa ogni funzione al posto di `todo!()`, poi esegui:
//   cargo test
// I test sono già scritti: non modificarli, servono da specifica.
// Nota: questa pagina non parla ancora di `match` — non ti serve per
// completare questi esercizi (arriva nel prossimo capitolo).

/// Esercizio 1 — Enum con sole varianti unit
///
/// La forma più semplice di enum: ogni variante non porta con sé nessun
/// dato, un po' come un "elenco di opzioni" (simile a un enum del C).
#[derive(Debug, PartialEq)]
enum Direzione {
    Sinistra,
    Destra,
}

/// Restituisci la variante `Direzione::Destra`.
fn direzione_destra() -> Direzione {
    Direzione::Destra
}

/// Esercizio 2 — Un enum può mescolare tipi di variante diversi
///
/// Uno stesso enum può avere varianti unit (senza dati), varianti "tuple"
/// (con dati posizionali, come una tupla) e varianti "struct" (con campi con
/// nome) — tutte insieme. Restano comunque valori dello STESSO tipo
/// `MossaGiocatore`, cosa che con tre struct separate non potresti ottenere.
#[derive(Debug, PartialEq)]
enum MossaGiocatore {
    Passa,                             // variante unit: nessun dato
    Corri(Direzione),                  // variante tuple: un dato posizionale
    Teletrasporto { x: u32, y: u32 },  // variante struct: campi con nome
}

/// Costruisci la variante unit `MossaGiocatore::Passa`.
fn passa() -> MossaGiocatore {
    MossaGiocatore::Passa
}

/// Costruisci la variante tuple `MossaGiocatore::Corri`, che porta con sé
/// una `Direzione`.
fn corri(direzione: Direzione) -> MossaGiocatore {
    MossaGiocatore::Corri(direzione)
}

/// Costruisci la variante struct `MossaGiocatore::Teletrasporto`, coi campi
/// `x` e `y`.
fn teletrasporto(x: u32, y: u32) -> MossaGiocatore {
    MossaGiocatore::Teletrasporto {x, y}
}

/// Esercizio 3 — Un solo tipo per forme diverse
///
/// Proprio perché tutte le varianti sono dello stesso tipo, una funzione può
/// riceverne una qualsiasi come parametro. Scrivi una funzione che riceva
/// una `MossaGiocatore` e restituisca `true` se è ESATTAMENTE
/// `MossaGiocatore::Passa` (usa `==`, reso possibile da `#[derive(PartialEq)]`
/// — niente `match` necessario qui).
fn e_una_pausa(mossa: &MossaGiocatore) -> bool {
    mossa == &MossaGiocatore::Passa 
}

/// Esercizio 4 — Leggere il discriminante
///
/// Ogni variante di un enum ha un "discriminante": un numero intero che
/// identifica quale variante è attiva. Con `#[repr(u32)]` puoi leggerlo con
/// un cast `as u32`. Senza valori espliciti, il discriminante parte da 0 e
/// aumenta di 1 per ogni variante.
#[derive(Debug, PartialEq)]
#[repr(u32)]
enum CodiceEsito {
    Ok,      // 0
    Avviso,  // 1
    Errore,  // 2
}

/// Restituisci il discriminante di `esito` come `u32` (usa `esito as u32`).
fn discriminante(esito: CodiceEsito) -> u32 {
    esito as u32
}

/// Esercizio 5 — Discriminanti espliciti
///
/// Puoi assegnare esplicitamente il valore di una variante con `= valore`.
/// Le varianti successive senza valore esplicito continuano a contare da lì
/// (es. `B = 10000` seguito da `C` fa sì che `C` valga `10001`, non un
/// numero "intuitivo" a sé stante).
#[derive(Debug, PartialEq)]
#[repr(u32)]
enum CodiceHttp {
    Ok = 200,
    NonTrovato = 404,
    ErroreServer = 405, // <- quale valore avrà? non è scritto esplicitamente
}

/// Restituisci il discriminante di `CodiceHttp::ErroreServer` come `u32`.
/// (Suggerimento: qual è il discriminante della variante subito precedente,
/// più 1?)
fn codice_errore_server() -> u32 {
    CodiceHttp::ErroreServer as u32
}

/// Esercizio 6 (bonus) — Ottimizzazione dello spazio (niche optimization)
///
/// La pagina nota che `Option<&T>` non occupa più spazio di `&T` da solo:
/// Rust riusa il bit pattern "nullo" — altrimenti impossibile per un
/// riferimento non-null — per rappresentare `None`, senza bisogno di un
/// discriminante separato. Verifica questo fatto con `std::mem::size_of`.
fn dimensione_option_riferimento() -> usize {
    std::mem::size_of::<Option<&i32>>()
}

fn main() {
    println!("Completa gli esercizi in questo file, poi lancia `cargo test`.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direzione_destra() {
        assert_eq!(direzione_destra(), Direzione::Destra);
    }

    #[test]
    fn test_passa() {
        assert_eq!(passa(), MossaGiocatore::Passa);
    }

    #[test]
    fn test_corri() {
        assert_eq!(corri(Direzione::Sinistra), MossaGiocatore::Corri(Direzione::Sinistra));
    }

    #[test]
    fn test_teletrasporto() {
        assert_eq!(teletrasporto(3, 7), MossaGiocatore::Teletrasporto { x: 3, y: 7 });
    }

    #[test]
    fn test_e_una_pausa() {
        assert!(e_una_pausa(&MossaGiocatore::Passa));
        assert!(!e_una_pausa(&MossaGiocatore::Corri(Direzione::Destra)));
        assert!(!e_una_pausa(&MossaGiocatore::Teletrasporto { x: 0, y: 0 }));
    }

    #[test]
    fn test_discriminante() {
        assert_eq!(discriminante(CodiceEsito::Ok), 0);
        assert_eq!(discriminante(CodiceEsito::Avviso), 1);
        assert_eq!(discriminante(CodiceEsito::Errore), 2);
    }

    #[test]
    fn test_codice_errore_server() {
        assert_eq!(codice_errore_server(), 405);
    }

    #[test]
    fn test_dimensione_option_riferimento() {
        assert_eq!(dimensione_option_riferimento(), std::mem::size_of::<&i32>());
    }
}
