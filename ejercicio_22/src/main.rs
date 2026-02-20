

fn main() {
    todo!()
}

#[derive(Debug,PartialEq)]

struct Pila { //en este caso es una pila de u32, tamaño 5, asi se declara, teniendo en cuenta la cima.
    cima : usize,
    a : [u32;5], // es el almacen de elementos
}

impl Pila { // implementamos caracteristicas, crear pila, añadir a la pila y quitar de la pila.
    fn new() -> Pila { //no hace falta pasarle nada porque siempre que me cree una pila va a ser de la misma manera, con estos valores.
        Pila { a : [0;5], cima : 0 }
    }

    fn push(&mut self, dato : u32,) -> Result<(),String>{ // &mut porque cambia, () si todo ok, String si hay que imprimir el error.
        if self.cima == self.a.len() { //capacidad maxima, siempre va a ser 5.
            return Err(String::from("Pila llena")); //si la pila esta llena, error
        }
        self.a[self.cima] = dato; // la nueva cima es el dato
        self.cima += 1; // aumentamos una unidad a la pila.
        Ok(()) // todo esta OK
    }

    fn pop(&mut self) -> Result<u32, String>{ //&mut porque cambia, u32 porque sera la nueva cima, String para imprimir el error.
        if self.cima == 0{
            return Err(String::from("Pila vacía"));
        }
        self.cima -= 1; // le bajamos una unidad a la cima
        Ok(self.a[self.cima]) // devolvemos la nueva cima con todo OK
    }
}


#[test]
fn test_new() {
    let p = Pila::new();

    assert_eq!(p, Pila {a:[0,0,0,0,0], cima:0});
}

#[test]
fn test_push_if_not_full() {
    let mut p = Pila::new();
    let _ = p.push(3);

    assert_eq!(p, Pila {a:[3,0,0,0,0], cima:1});
}

#[test]
fn test_push_until_full() {
    let mut p = Pila::new();
    let _ = p.push(3);
    let _ = p.push(4);
    let _ = p.push(5);
    let _ = p.push(6);
    let _ = p.push(7);

    assert_eq!(p.push(8), Err(String::from("Pila llena")));  // returns Err
    assert_eq!(p, Pila {a:[3,4,5,6,7], cima:5});
}


#[test]
fn test_pop_if_not_empty() {
    let mut p = Pila::new();
    let _ = p.push(3);

    let e = p.pop();

    assert_eq!(Ok(3), e);
    assert_eq!(p, Pila {a:[3,0,0,0,0], cima:0});
}

#[test]
fn test_pop_if_empty() {
    let mut p = Pila::new();

    let e = p.pop();

    assert_eq!(Err(String::from("Pila vacía")), e);
    assert_eq!(p, Pila {a:[0,0,0,0,0], cima:0});
}

#[test]
fn test_pop_until_empty() {
    let mut p = Pila::new();
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