/*
 trait anterior, no genérico

 trait Transportar {
     fn carga(&mut self, cantidad: i32) -> i32;
 }
*/

#[derive(Debug)]
struct Transbordador {
    nombre: String,
    carga: i32,
}

#[derive(Debug)]
struct Carguero {
    nombre: String,
    carga: f64,
}


trait Transportar<T> {
    fn carga(&mut self, cantidad: T);
}

// implementación para Transbordador
impl Transportar<i32> for Transbordador {

    fn carga(&mut self, cantidad: i32) {
        self.carga += cantidad;
    }
}

// implementación para Carguero (f64)
impl Transportar<f64> for Carguero {

    fn carga(&mut self, cantidad: f64) {
        self.carga += cantidad;
    }
}

// <- ERROR
// conflicting implementations of trait 
// `Transportar<_>` for type `Carguero<_>
/*
impl<f32> Transportar<f32> for Carguero<f32> 
where f32: std::ops::AddAssign {

    fn carga(&mut self, cantidad: f32) {
        self.carga += cantidad;
    }
}
*/


pub fn run() {

    println!("");
    println!("--------------------");
    println!("Generic Traits");
    println!("--------------------");
    println!("");

    let mut t1 = Transbordador {
        nombre: "T1".to_string(),
        carga: 0,
    };

    let mut c2 = Carguero {
        nombre: "C1".to_string(),
        carga: 0.0,
    };

    t1.carga(10); // i32
    c2.carga(10.0); // f64

    println!("Transbordador: {:#?} {}", t1, t1.nombre);
    println!("Carguero: {:#?} {}", c2, c2.nombre);
}
