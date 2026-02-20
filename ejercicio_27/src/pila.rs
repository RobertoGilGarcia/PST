#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pila<Tipo, const TAMANIO: usize> {
    cima: usize,
    a: [Tipo; TAMANIO],
}

impl<Tipo: Copy + Default, const TAMANIO: usize> Pila<Tipo, TAMANIO> {
    pub fn new() -> Pila<Tipo, TAMANIO> {
        Pila {
            cima: 0,
            a: [Tipo::default(); TAMANIO],
        }
    }

    pub fn push(&mut self, parametro: Tipo) -> Result<(), String> {
        if self.cima < TAMANIO {
            self.a[self.cima] = parametro;
            self.cima += 1;
            Ok(())
        } else {
            Err(String::from("Pila llena"))
        }
    }

    pub fn pop(&mut self) -> Result<Tipo, String> {
        if self.cima == 0 {
            Err(String::from("Pila vacía"))
        } else {
            self.cima -= 1;
            Ok(self.a[self.cima])
        }
    }
    pub fn is_empty(&self) -> bool {
        if self.cima == 0 {
            true
        } else {
            false
        }
    }
}
