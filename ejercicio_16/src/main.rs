fn rec_bin_search(t: i32, a: &[i32]) -> isize {
    if a.len() == 0 { // si el array está vacio devuelve -1 directamente
        return -1
    }

    let  medio: isize = (a.len() / 2)as isize; //el medio del array
    if a[medio as usize] == t { // si en el medio está justo el valor pues lo devolvemos
        return medio;
    } else if a[medio as usize] > t { // si el medio es mayor que el valor
        return rec_bin_search(t, &a[0..medio as usize]); //seguimos buscando del inicio al medio
    } else {// si el medio es menor, buscamos del medio al final
        let resultado = rec_bin_search(t, &a[medio as usize + 1..]); //seguimos buscando y almacenamos respuesta en resultado
        if resultado == -1 { // si el resultado es -1 no esta, devolvemos -1
            return -1;
        } else {
            return medio + 1 + resultado; // si no, ajustamos los indices relativos a los absolutos, le sumamos al inicio de la parte de arriba el resultado mas 1
        }
    }

}

#[test]
fn middle_odd() {
    assert_eq!(2, rec_bin_search(3, &[-5, -2, 3, 14, 25]));
}

#[test]
fn middle_even_1() {
    assert_eq!(2, rec_bin_search(3, &[-5, -2, 3, 7, 14, 25]));
}

#[test]
fn middle_even_2() {
    assert_eq!(3, rec_bin_search(7, &[-5, -2, 3, 7, 14, 25]));
}

#[test]
fn first_element() {
    assert_eq!(0, rec_bin_search(1, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn last_element() {
    assert_eq!(5, rec_bin_search(25, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn not_found_left() {
    assert_eq!(-1, rec_bin_search(1, &[2, 3, 7, 14, 17, 25]));
}

#[test]
fn not_found_right() {
    assert_eq!(-1, rec_bin_search(42, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn found_single_element() {
    assert_eq!(0, rec_bin_search(3, &[3]));
}

#[test]
fn not_found_single_element() {
    assert_eq!(-1, rec_bin_search(42, &[3]));
}

#[test]
fn not_found_empty_array() {
    assert_eq!(-1, rec_bin_search(42, &[]));
}


fn main() {

}
