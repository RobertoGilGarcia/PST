fn main() {
  todo!()
}


enum Figuras { // en este caso es un enum, que dentro tiene structs, que dentro tienen sus propios parametros
    Cuadrado {lado: f64},
    Rectangulo{
        base : f64,
        altura : f64
    }
}

impl Figuras { //implementamos las caracteristicas que queramos que tengan nuestros enums.
    fn perimetro(&self) -> f64{
        match self {
            Figuras::Cuadrado {lado} => {lado * 4.0}, //usamos match para cada fiigura con esta sintaxis,
            Figuras::Rectangulo { base, altura } => {(base * 2.0) + (altura * 2.0)},
        }
    }
    fn area(&self) -> f64 {
        match self {
            Figuras::Cuadrado {lado} => {lado*lado},
            Figuras::Rectangulo { base, altura } => {base*altura}
        }
    }
    fn zoom(&mut self, parametro: f64) { // en la caracteristica zoom, no devolvemos nada, solo cambiamos las medidas
        match self {
            Figuras::Cuadrado { lado } => {*lado *= parametro;}, // lado es un puntero , pero *lado es lo que esta dentro del puntero
            Figuras::Rectangulo { base, altura } => {
                *base = (*base) * parametro; // es importante atravesar el puntero
                *altura = (*altura) * parametro;
            }

        }
    }
}



#[test]
fn test_perimetros() {
    let figuras = [
        Figuras::Cuadrado { lado: 5.0 },
        Figuras::Rectangulo {
            base: 10.0,
            altura: 2.0,
        },
        Figuras::Cuadrado { lado: 3.0 },
    ];

    let mut perimetros = [0.0; 3];
    for i in 0..figuras.len() {
        perimetros[i] = figuras[i].perimetro();
    }

    assert_eq!(perimetros, [20.0, 24.0, 12.0]);
}

#[test]
fn test_areas() {
    let figuras = [
        Figuras::Cuadrado { lado: 5.0 },
        Figuras::Rectangulo {
            base: 10.0,
            altura: 2.0,
        },
        Figuras::Cuadrado { lado: 3.0 },
    ];

    let mut areas = [0.0; 3];
    for i in 0..figuras.len() {
        areas[i] = figuras[i].area();
    }

    assert_eq!(areas, [25.0, 20.0, 9.0]);
}


#[test]
fn test_zoom_perimetros() {
    let mut figuras = [
        Figuras::Cuadrado { lado: 5.0 },
        Figuras::Rectangulo {
            base: 10.0,
            altura: 2.0,
        },
        Figuras::Cuadrado { lado: 3.0 },
    ];

    let mut perimetros = [0.0; 3];
    for i in 0..figuras.len() {
        perimetros[i] = figuras[i].perimetro();
    }
    assert_eq!(perimetros, [20.0, 24.0, 12.0]);

    for i in 0..figuras.len() {
        figuras[i].zoom(2.0); // El método zoom multiplica por el parámetro cada lado de la figura
    }
    for i in 0..figuras.len() {
        perimetros[i] = figuras[i].perimetro();
    }
    assert_eq!(perimetros, [40.0, 48.0, 24.0]);


    for i in 0..figuras.len() {
        figuras[i].zoom(0.5); // El método zoom multiplica por el parámetro cada lado de la figura
    }
    for i in 0..figuras.len() {
        perimetros[i] = figuras[i].perimetro();
    }

    assert_eq!(perimetros, [20.0, 24.0, 12.0]);

}