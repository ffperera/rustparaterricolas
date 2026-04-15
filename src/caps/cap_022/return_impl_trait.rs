trait Transportar {
    fn carga(&mut self, cantidad: i32);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Carguero {
    nombre: String,
    carga: f64,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Transbordador {
    nombre: String,
    carga: i32,
}

impl Transportar for Carguero {
    fn carga(&mut self, cantidad: i32) {
        self.carga += cantidad as f64;
    }
}

impl Transportar for Transbordador {
    fn carga(&mut self, cantidad: i32) {
        self.carga += cantidad;
    }
}

enum _ModeloTransporte {
    Carguero,
    Transbordador,
}

enum Nave {
    Carguero(Carguero),
    Transbordador(Transbordador),
}

impl Transportar for Nave {
    fn carga(&mut self, cantidad: i32) {
        match self {
            Nave::Carguero(c) => c.carga(cantidad),
            Nave::Transbordador(t) => t.carga(cantidad),
        }
    }
}

fn fabrica(modelo: i32, nombre: &str) -> impl Transportar {
    match modelo {
        1 => Nave::Carguero(Carguero {
            nombre: nombre.to_string(),
            carga: 0.0,
        }),
        _ => Nave::Transbordador(Transbordador {
            nombre: "Transbordador".to_string(),
            carga: 0,
        }),
    }
}

// fn fabrica2(modelo: i32, nombre: &str) -> impl Transportar {
//     if modelo == 1 {
//         return Carguero {
//             nombre: nombre.to_string(),
//             carga: 0.0,
//         };
//     }
//
//     if modelo == 2 {
//         return Transbordador {
//             nombre: "Transbordador".to_string(),
//             carga: 0,
//         };
//     }
// }

pub fn run() {
    println!();
    println!("--------------------");
    println!("Return impl Trait");
    println!("--------------------");
    println!();

    let mut t1 = fabrica(2, "T1");

    let mut c1 = fabrica(1, "C1");

    t1.carga(19);

    c1.carga(85);

    // como t1 y c1 son de tipo impl Trait,
    // sólo podemos utilizar métodos que
    // correspondan a ese trait
}
