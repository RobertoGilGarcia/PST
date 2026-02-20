fn search(t: i32, a: &[i32]) -> isize {
    let mut izquierda_a: usize = 0;  // Usamos usize porque los índices en Rust son usize

    // Recorremos el array hasta que encontramos t o llegamos al final
    while izquierda_a < a.len() {
        if a[izquierda_a] == t {  // Comparamos el valor actual con t
            return izquierda_a as isize;  // Si lo encontramos, devolvemos el índice
        }
        izquierda_a += 1;  // Avanzamos al siguiente índice
    }

    -1  // Si no encontramos t, devolvemos -1
}

#[test]
fn middle_odd() {
    assert_eq!(2, search(3, &[14, 25, 3, -5, -2]));
}

#[test]
fn middle_even_1() {
    assert_eq!(3, search(3, &[14, -5, -2, 3, 7, 25]));
}

#[test]
fn middle_even_2() {
    assert_eq!(2, search(3, &[-5, -2, 3, 7, 14, 25]));
}

#[test]
fn first_element() {
    assert_eq!(0, search(1, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn last_element() {
    assert_eq!(5, search(25, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn not_found_left() {
    assert_eq!(-1, search(1, &[2, 3, 7, 14, 17, 25]));
}

#[test]
fn not_found_right() {
    assert_eq!(-1, search(42, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn found_single_element() {
    assert_eq!(0, search(3, &[3]));
}

#[test]
fn not_found_single_element() {
    assert_eq!(-1, search(42, &[3]));
}

#[test]
fn not_found_empty_array() {
    assert_eq!(-1, search(42, &[]));
}

fn main() {
    let a = [1, 2, 3];
    let t = 1;
    let result = search(t, &a);
    if result != -1 {
        println!("Elemento encontrado en la posición: {}", result);
    } else {
        println!("Elemento no encontrado");
    }
}