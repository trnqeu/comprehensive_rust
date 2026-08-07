// Esercizi sul capitolo "Strings" (&str vs String) di Comprehensive Rust.
//
// Completa ogni funzione al posto di `todo!()`, poi esegui:
//   cargo test
// per verificare le tue soluzioni. I test sono già scritti: non modificarli,
// servono da specifica.

/// Esercizio 1 — &str -> String con format!
///
/// `&str` è un riferimento immutabile a dati UTF-8, `String` è un buffer
/// posseduto (owned). `format!` costruisce una `String` a partire da valori
/// dinamici, con la stessa sintassi di `println!`.
///
/// Scrivi una funzione che riceva un nome (`&str`) e restituisca un saluto
/// come `String`, es. saluta("Marco") -> "Ciao, Marco!"
fn saluta(nome: &str) -> String {
    todo!()
}

/// Esercizio 2 — Costruire una String pezzo per pezzo
///
/// `String::new()` crea una stringa vuota; `push_str(&str)` aggiunge una
/// slice, `push(char)` aggiunge un singolo carattere.
///
/// Scrivi una funzione che unisca le parole di `parole` separandole con uno
/// spazio, SENZA usare `.join(" ")` (usa invece push_str/push in un ciclo,
/// per fare pratica con i metodi del capitolo).
/// Nota: nessuno spazio finale, e "" se la slice è vuota.
fn unisci_parole(parole: &[&str]) -> String {
    todo!()
}

/// Esercizio 3 — Slicing sicuro sui confini UTF-8
///
/// Se selezioni un range di byte non allineato a un confine di carattere,
/// `&s[a..b]` va in panic. Caratteri come 'è' occupano 2 byte in UTF-8, quindi
/// indicizzare "a occhio" è pericoloso: meglio usare l'iteratore `chars()`
/// (o `char_indices()`) per trovare i confini corretti.
///
/// Scrivi una funzione che restituisca i primi `n` CARATTERI (non byte) di
/// `s` come slice `&str`. Se `n` è maggiore del numero di caratteri, restituisci
/// l'intera stringa.
fn prime_n_caratteri(s: &str, n: usize) -> &str {
    todo!()
}

/// Esercizio 4 — Caratteri vs byte
///
/// Per stringhe non-ASCII, il numero di caratteri e il numero di byte possono
/// differire (es. "caffè" ha 5 caratteri ma 6 byte, perché 'è' occupa 2 byte).
///
/// Scrivi una funzione che restituisca la tupla (numero_di_caratteri, numero_di_byte).
fn conta_caratteri_e_byte(s: &str) -> (usize, usize) {
    todo!()
}

/// Esercizio 5 — Iterare sui caratteri
///
/// Scrivi una funzione che dica se `s` è un palindromo, ignorando maiuscole/
/// minuscole e spazi (es. "I topi non avevano nipoti" è un palindromo).
fn is_palindromo(s: &str) -> bool {
    todo!()
}

/// Esercizio 6 (bonus) — Raw string literals
///
/// Le raw string (`r"..."`) disabilitano le sequenze di escape: `r"\n"` è
/// letteralmente backslash + n, non un a-capo. Sono comode per percorsi di
/// Windows o regex, dove altrimenti dovresti raddoppiare i backslash.
///
/// Scrivi una funzione che conti quante volte compare il carattere backslash
/// '\\' in una stringa.
fn conta_backslash(s: &str) -> usize {
    todo!()
}

fn main() {
    println!("Completa gli esercizi in questo file, poi lancia `cargo test`.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saluta() {
        assert_eq!(saluta("Marco"), "Ciao, Marco!");
        assert_eq!(saluta("Anna"), "Ciao, Anna!");
    }

    #[test]
    fn test_unisci_parole() {
        assert_eq!(unisci_parole(&["Rust", "è", "fantastico"]), "Rust è fantastico");
        assert_eq!(unisci_parole(&["ciao"]), "ciao");
        assert_eq!(unisci_parole(&[]), "");
    }

    #[test]
    fn test_prime_n_caratteri() {
        assert_eq!(prime_n_caratteri("caffè", 4), "caff");
        assert_eq!(prime_n_caratteri("caffè", 5), "caffè");
        assert_eq!(prime_n_caratteri("caffè", 100), "caffè");
        assert_eq!(prime_n_caratteri("héllo", 1), "h");
    }

    #[test]
    fn test_conta_caratteri_e_byte() {
        assert_eq!(conta_caratteri_e_byte("caffè"), (5, 6));
        assert_eq!(conta_caratteri_e_byte("Rust"), (4, 4));
        assert_eq!(conta_caratteri_e_byte(""), (0, 0));
    }

    #[test]
    fn test_is_palindromo() {
        assert!(is_palindromo("Anna"));
        assert!(is_palindromo("I topi non avevano nipoti"));
        assert!(!is_palindromo("Rust"));
    }

    #[test]
    fn test_conta_backslash() {
        let percorso = r"C:\Users\test\file.txt";
        assert_eq!(conta_backslash(percorso), 3);
        assert_eq!(conta_backslash("nessuno"), 0);
    }
}
