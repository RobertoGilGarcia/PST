struct PoligonoRegular {
    numero_lados : u32,
    longitud_lado : u32,
}

impl PoligonoRegular {
    fn new(numero_lados: u32, longitud_lado: u32) -> PoligonoRegular {
        PoligonoRegular { numero_lados, longitud_lado}
    }
    fn perimetro(&self) -> u32 {
        self.longitud_lado * self.numero_lados
    }
}

#[test]
fn perimetro_triangulo() {
    let mut t = PoligonoRegular {
        numero_lados: 3,
        longitud_lado: 10,
    };
    assert_eq!(t.perimetro(), 30);

    t.longitud_lado = 20;
    assert_eq!(t.perimetro(), 60);
}

#[test]
fn perimetro_cuadrado() {
    let c = PoligonoRegular::new(10, 4);

    assert_eq!(c.perimetro(), 40);
}
 fn main() {
    todo!()
 }