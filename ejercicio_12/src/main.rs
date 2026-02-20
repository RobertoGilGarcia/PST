fn partition(a: &mut[i8]) {
    for i in 0..a.len() {
        for j in (i+1)..a.len() {
            if a[i] >= 0 && a[j] < 0 {
                let aux = a[i];
                a[i] = a[j];
                a[j] = aux;
            }
        }
    }
}


fn main() {
    let mut array = [1,2,3,45,-7,-9];
    partition( &mut array);
}
 
#[test]
fn all_tests() {
    let test_cases = vec![
        vec![1, 2, 0, -1, -2, 1, -4, 2],
        vec![-5, 1, 2, 0, -1, -2, 1, -4, 2],
        vec![-5, 1, 2, 0, -1, -2, 1, -4, 2, -8],
        vec![-1, 2, 0, -1, -2, 1, -4],
        vec![1, 2, 0],
        vec![-1, -2, -3],
    ];
 
    for mut orig in test_cases {
        let mut array = orig.clone();
 
        partition(&mut array);
        eprintln!("test case: {orig:?}\nYour result:    {array:?}\n");
 
    // check array is partitioned: negatives on the left
    for i in 0..array.len() {
            if i < array.len() - 1 {
                assert!(!(array[i] >= 0 && array[i + 1] < 0))
            };
        }
 
    // check the partitioned array has the same elements than orig
    array.sort();
    orig.sort();
    assert_eq!(array, orig);
    }
}