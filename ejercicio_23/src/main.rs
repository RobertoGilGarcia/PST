use std::env::consts;

fn main() {
        todo!()
}

#[derive(Debug,PartialEq,Copy, Clone)]

struct Pila <T, const TAMANYO:usize>{ //pila generica de tipo T generico con TAMANYO desconocido en un principio.
    a: [T; TAMANYO], // lo que es la propia pila de valores de tipo T y de tamaño TAMANYO
    cima : usize, // la cima de la pila como indice.
}

impl <T: Default + Copy,const  TAMANYO : usize> Pila<T, TAMANYO> { // implementamos el default para apilar cualquier tipo de datos
    fn new() -> Pila<T,TAMANYO> { // crear una pila [T;TAMANYO]
        Pila {a: [T::default(); TAMANYO], cima :0}
    }
    fn push(&mut self, dato: T) -> Result<(), String> { // () si todo OK, String si hay error.
        if self.cima == self.a.len() {
            return Err(String::from("Pila llena")); // error de la pila llena
        }
        self.a[self.cima] = dato; // si se puede meter, actualizamos la cima al dato y le añadimos una unidad a la cima
        self.cima += 1;
        Ok(()) // todo OK, devolvemos ()
    }
    fn pop(&mut self) -> Result<T, String> { //Si todo OK devolvemos la nueva cima de tipo T, si error String con el error
        if self.cima == 0 {
            return Err(String::from("Pila vacía")); // error de la pila vacia
        }
        self.cima -= 1; // primero le restamos uno a la cima
        Ok(self.a[self.cima]) // despues devolvemos el dato de la nueva cima de tipot T.
    }
}






#[test]
fn test_push_string() {
    let mut p : Pila<&str,4> = Pila::new();
    let _ = p.push("hola");

    assert_eq!(p, Pila {a:["hola", "", "", ""], cima:1});
}

#[test]
fn test_new() {
    let p : Pila<u32,5> = Pila::new();

    assert_eq!(p, Pila {a:[0,0,0,0,0], cima:0});
}

#[test]
fn test_push_if_not_full() {
    let mut p : Pila<f64,4> = Pila::new();
    let _ = p.push(3.0);

    assert_eq!(p, Pila {a:[3.0,0.0,0.0,0.0], cima:1});
}

#[test]
fn test_push_until_full() {
    let mut p : Pila<u32,4> = Pila::new();
    let _ = p.push(3);
    let _ = p.push(4);
    let _ = p.push(5);
    let _ = p.push(6);

    assert_eq!(p.push(7), Err(String::from("Pila llena")));  // returns Err
    assert_eq!(p, Pila {a:[3,4,5,6], cima:4});
}


#[test]
fn test_pop_if_not_empty() {
    let mut p: Pila<i32,5> = Pila::new();
    let _ = p.push(-3);

    let e = p.pop();

    assert_eq!(Ok(-3), e);
    assert_eq!(p, Pila {a:[-3,0,0,0,0], cima:0});
}

#[test]
fn test_pop_if_empty() {
    let mut p: Pila<u8,5> = Pila::new();

    let e = p.pop();

    assert_eq!(Err(String::from("Pila vacía")), e);
    assert_eq!(p, Pila {a:[0,0,0,0,0], cima:0});
}

#[test]
fn test_pop_until_empty() {
    let mut p: Pila<u64,5> = Pila::new();
    let _ = p.push(3);
    let _ = p.push(4);
    let _ = p.push(5);
    let _ = p.push(6);
    let _ = p.push(7);


    let _ = p.pop();
    let _ = p.pop();
    let _ = p.pop();
    let _ = p.pop();
    let _ = p.pop();

    assert_eq!(p.pop(), Err(String::from("Pila vacía")));  // returns Err
    assert_eq!(p, Pila {a:[3,4,5,6,7], cima:0});
}

