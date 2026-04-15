trait Transportar {
    type A;

    fn carga(&mut self, cantidad: Self::A);
}

#[derive(Debug)]
struct Transbordador {
    nombre: String,
    carga: i32,
}

#[derive(Debug)]
struct Carguero<T> {
    nombre: String,
    carga: T,
}

impl Transportar for Transbordador {
    type A = i32;

    fn carga(&mut self, cantidad: Self::A) {
        self.carga += cantidad;
    }
}

impl Transportar for Carguero<f64> {
    type A = f64;

    fn carga(&mut self, cantidad: Self::A) {
        self.carga += cantidad;
    }
}

impl Transportar for Carguero<f32> {
    type A = f32;

    fn carga(&mut self, cantidad: Self::A) {
        self.carga += cantidad;
    }
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Generic Traits - Associated types");
    println!("--------------------");
    println!();

    let mut t1 = Transbordador {
        nombre: "T1".to_string(),
        carga: 0,
    };

    let mut c2 = Carguero {
        nombre: "C2".to_string(),
        carga: 0.0, // f64
    };

    let mut mc3 = Carguero {
        nombre: "MC3".to_string(),
        carga: 0.0f32, // f32
    };

    t1.carga(10);
    c2.carga(10.0);
    mc3.carga(5.1);

    println!("Transbordador: {:#?} {}", t1, t1.nombre);
    println!("Carguero: {:#?} {}", c2, c2.nombre);
    println!("MiniCarguero: {:#?} {}", mc3, mc3.nombre);
}
