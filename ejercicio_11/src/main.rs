/// Devuelve la longitud de la secuencia comenzando por `n`.
fn seq_length(mut n: i32) -> u32 {
  let mut total: u32 = 1;
  loop {
    if n == 1 {
        break;
    } else if n % 2 == 0 {
        n = n / 2;
    } else {
        n = 3 * n + 1;
    }
    total += 1;
  }
  total //este es total, el valor TOTAL de la secuencia de esta funcion, que es lo que nos pide.
}
 
#[test]
fn test_seq_length() {
    assert_eq!(seq_length(11), 15);
}
 
#[test]
fn test_seq_length_2() {
    assert_eq!(seq_length(9), 20);
}

fn main(){
    todo!()
}