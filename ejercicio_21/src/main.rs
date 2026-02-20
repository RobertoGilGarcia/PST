#[derive(Copy, Clone)] // esta linea es obligatoria, para poder copiar las propiedades y que no haya errores de compilacion.

struct Square {
    width : i32
}

impl Square {
    fn area(self: Self) -> i32 {
        self.width * self.width
    }
}

#[test]
fn t() {
    let s = Square { width: 2 };
    let s2 = s; // se transfiere la propiedad a s2, ahora la pertenencia de esa variable es s2, para quitarlo hay que poner el derive,

    assert_eq!(4, s.area());
    assert_eq!(4, s2.area());


    assert_eq!(4, s.area());
    assert_eq!(4, s2.area());
}

fn main() {
    todo!()
}