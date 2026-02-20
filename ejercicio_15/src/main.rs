
fn main() {
println!("dddd")
}
fn bin_search(t: i32, a: &[i32]) -> isize {
    if a == [] {
        return -1;
    }

    let mut izquierda: isize = 0;
    let mut derecha: isize = a.len() as isize -1;
    let  medio: isize = izquierda + derecha / 2;
    {
        if a[medio as usize] == t  {
            return medio as isize;
        }
        while t != a[medio as usize] && a.len() > 0  && izquierda < derecha{
            if a[izquierda as usize] == t {
                return izquierda as isize;
            }
            izquierda += 1;
            if a[derecha as usize] == t {
                return derecha as isize;
            }
            derecha -= 1;
        }
        -1
    }
}


#[test]
fn middle_odd() {
    assert_eq!(2, bin_search(3, &[-5, -2, 3, 14, 25]));
}

#[test]
fn middle_even_1() {
    assert_eq!(2, bin_search(3, &[-5, -2, 3, 7, 14, 25]));
}

#[test]
fn middle_even_2() {
    assert_eq!(3, bin_search(7, &[-5, -2, 3, 7, 14, 25]));
}

#[test]
fn first_element() {
    assert_eq!(0, bin_search(1, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn last_element() {
    assert_eq!(5, bin_search(25, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn not_found_left() {
    assert_eq!(-1, bin_search(1, &[2, 3, 7, 14, 17, 25]));
}

#[test]
fn not_found_right() {
    assert_eq!(-1, bin_search(42, &[1, 2, 3, 7, 14, 25]));
}

#[test]
fn found_single_element() {
    assert_eq!(0, bin_search(3, &[3]));
}

#[test]
fn not_found_single_element() {
    assert_eq!(-1, bin_search(42, &[3]));
}

#[test]
fn not_found_empty_array() {
    assert_eq!(-1, bin_search(42, &[]));
}

