use std::usize;


fn main() {
    println!("Hello, world!");
}



fn next_token(s: &str, start: usize) -> Option<(usize,usize)>{
    let string = &s[start..];
    let mut in_word = false;
    let mut word_start = 0;

    let mut i = 0;//indices de los chars.

    for c in string.chars() {
        if c.is_whitespace() { // si c es whitespace
            if in_word { // estoy en una palabra, la devuelvo
                return Some((word_start + start, i - 1 + start)); // indices relativos ajustados sumandole posicon de inicio(start)
            } 
        } else { // si no es un whitespace
            if !in_word { // y no estoy en una palabra, es porque he encontrado una
                in_word = true; //en palabra = true
                word_start = i; //inicio de palabra para devolverlo despues ajustado al inicio de una palabra (i)
            }
        }
        i += 1; // avanzo una posicion para ver si es whitespace o no
    }
    if in_word {Some((word_start + start, i - 1 + start))} else { return None;}//si estoy en una palabra, devuelvo sus posiciones y si no None
}

#[test]
fn t1(){
    assert_eq!(None, next_token(&"   ",0));
}

#[test]
fn t2(){
    assert_eq!((1,4), next_token(&" hola hola", 0).unwrap());
    assert_eq!((6,9), next_token(&" hola hola", 5).unwrap());
    assert_eq!(None , next_token(&" hola hola", 10));
}

#[test]
fn t3(){
    assert_eq!((0,3), next_token("hola ",0).unwrap());
    assert_eq!((2,5), next_token("  hola ", 0).unwrap());
    assert_eq!((1,4), next_token(" hola ", 0).unwrap());
    assert_eq!((1,4), next_token(" hola", 0).unwrap());
}
