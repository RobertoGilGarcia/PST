fn sum<const N: usize>(a: &[u8], b: &[u8]) -> [u8; N] {
    assert!(N == a.len() + 1 || N == b.len() + 1);
    let mut result = [0; N]; // Array para almacenar el resultado

    // Punteros que apuntan al último elemento de los arrays
    let mut derecha_a = a.len();
    let mut derecha_b = b.len();
    let mut derecha_result = result.len();

    let mut acarreo = 0; // Acarreo inicializado a 0

    // Mientras haya espacio en el resultado
    while derecha_result > 0 {
        // Decrementamos el puntero del resultado
        derecha_result -= 1;

        // Obtenemos los valores de los arrays a y b si no hemos llegado al final, de lo contrario usamos 0
            let valor_a = if derecha_a > 0 {
            derecha_a -= 1;
            a[derecha_a]
        } else {
            0
        };

        let valor_b = if derecha_b > 0 {
            derecha_b -= 1;
            b[derecha_b]
        } else {
            0
        };

        // Sumamos los valores y el acarreo
        let suma = valor_a + valor_b + acarreo;

        // Si la suma es mayor que 9, establecemos el acarreo para la siguiente posición
        if suma > 9 {
            result[derecha_result] = suma % 10; // Guardamos el dígito menos significativo, solo ponemos el digito de la derecha en este caso.
            acarreo = 1; // Establecemos el acarreo
        } else {
            result[derecha_result] = suma; // Guardamos el resultado sin acarreo
            acarreo = 0; // No hay acarreo
        }
    }

    result
}



#[test]
fn no_carry_1() {
    assert_eq!([0, 5, 7, 9], sum(&[1, 2, 3], &[4, 5, 6]));
}

#[test]
fn carry_1() {
    assert_eq!([0, 5, 8, 8], sum(&[1, 2, 9], &[4, 5, 9]));
}

#[test]
fn carry_2() {
    assert_eq!([1, 4, 9, 8], sum(&[9, 9, 9], &[4, 9, 9]));
}

#[test]
fn carry_3() {
    assert_eq!([0, 5, 9, 8], sum(&[9, 9], &[4, 9, 9]));
}

#[test]
fn carry_4() {
    assert_eq!([1, 0, 4, 9, 8], sum(&[9, 9, 9, 9], &[4, 9, 9]));
}

#[test]
fn carry_5() {
    assert_eq!([1, 0, 0, 9], sum(&[9, 9, 9], &[1, 0]));
}

fn main() {
    let a = [1,2,4];
    let b = [4,5,6];
    let result : [u8;4] = sum(&a, &b);
    println!("resultado : {:?}", result);
}