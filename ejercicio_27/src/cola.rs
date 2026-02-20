use std::fmt::Debug;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Cola<Tipo, const TAMANIO: usize> {
    first: usize,
    last: usize,
    a: [Tipo; TAMANIO],
    len: usize,
}

impl<Tipo: Copy + Default + PartialEq, const TAMANIO: usize> Cola<Tipo, TAMANIO> {
    pub fn new() -> Cola<Tipo, TAMANIO> {
        Cola {
            first: 0,
            last: 0,
            a: [Tipo::default(); TAMANIO],
            len: 0,
        }
    }

    pub fn enqueue(&mut self, parametro: Tipo) -> Result<(), String> {
        if self.len == TAMANIO {
            Err(String::from("Cola llena"))
        } else {
            self.a[self.last] = parametro;
            self.last = (self.last + 1) % TAMANIO;
            self.len += 1;
            Ok(())
        }
    }

    pub fn dequeue(&mut self) -> Result<Tipo, String> {
        if self.len == 0 {
            Err(String::from("Cola vacía"))
        } else {
            let devolver = self.a[self.first];
            self.first = (self.first + 1) % TAMANIO;
            self.len -= 1;
            Ok(devolver)
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

