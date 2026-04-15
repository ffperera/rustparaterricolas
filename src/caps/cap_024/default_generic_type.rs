#[derive(Debug)]
struct Carguero<T = f64> {
    nombre: String,
    carga: T,
}

trait Transportar {
    fn carga(&mut self, cantidad: i32);
}

trait Aterrizar {}

#[derive(Debug)]
struct GrupoDeSuperficie<T>
where
    T: Transportar + Aterrizar,
{
    nombre: String,
    naves: Vec<T>,
}

#[derive(Debug)]
struct Transbordador {
    nombre: String,
    carga: i32,
}

impl Transportar for Transbordador {
    fn carga(&mut self, cantidad: i32) {
        self.carga += cantidad;
    }
}

impl Aterrizar for Transbordador {}

// implementación genérica de GrupoDeSuperficie
impl<T> GrupoDeSuperficie<T>
where
    T: Transportar + Aterrizar,
{
    fn new(nombre: String) -> Self {
        GrupoDeSuperficie {
            nombre,
            naves: Vec::<T>::new(),
        }
    }

    fn agregar_nave(&mut self, nave: T) {
        self.naves.push(nave);
    }
}

#[derive(Debug)]
struct CargueroLigero {
    nombre: String,
    carga: i32,
}

impl Transportar for CargueroLigero {
    fn carga(&mut self, cantidad: i32) {
        self.carga += cantidad;
    }
}

impl Aterrizar for CargueroLigero {}

// implementación concreta de GrupoDeSuperficie
impl GrupoDeSuperficie<CargueroLigero> {
    //  ^--- no genérica
    //                       ^-- tipo de dato concreto

    // no podemos redefinir el método new()
    // ya está implementado para todos los tipos
    // que cumplen Transportar + Aterrizar

    fn cargueros_ligeros(nombre: String) -> Self {
        GrupoDeSuperficie {
            nombre,
            naves: Vec::<CargueroLigero>::new(),
        }
    }

    fn cargueros(&self) -> &[CargueroLigero] {
        // esto dará error si el vector está vacío
        // en un programa real comprobaríamos, etc.
        &self.naves[..]
    }
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Default generic type");
    println!("--------------------");
    println!();

    // declaración como Carguero<f64>
    let c1: Carguero;

    // inicialización: carga es f64
    // a partir del genérico por defecto
    c1 = Carguero {
        nombre: "Carguero 1".to_string(),
        // carga: 10, // <- ERROR
        carga: 10.0,
    };

    // inicialización como Carguero<i32>
    // se infiere ya que carga es i32
    let c2 = Carguero {
        nombre: "Carguero 2".to_string(),
        carga: 20,
    };

    println!("c1: {:?} {} {}", c1, c1.nombre, c1.carga);
    println!("c2: {:?} {} {}", c2, c2.nombre, c2.carga);

    println!();
    println!("GrupoDeSuperficie - implementación genérica y concreta");

    let t1 = Transbordador {
        nombre: "T1".to_string(),
        carga: 0,
    };

    let t2 = Transbordador {
        nombre: "T2".to_string(),
        carga: 0,
    };

    println!("Transbordador {} {}", t1.nombre, t1.carga);
    println!("Transbordador {} {}", t2.nombre, t2.carga);

    let g1 = GrupoDeSuperficie {
        nombre: "G1".to_string(),
        naves: vec![t1, t2],
    };

    println!("{:?}", g1);

    println!("GrupoDeSuperficie - new() ");

    let mut cargo_ligero = CargueroLigero {
        nombre: "CL1".to_string(),
        carga: 0,
    };

    cargo_ligero.carga(800);

    println!(
        "Carguero ligero {} {}",
        cargo_ligero.nombre, cargo_ligero.carga
    );

    let g2: GrupoDeSuperficie<Transbordador> = GrupoDeSuperficie::new("G2".to_string());

    println!("{:?}", g2);

    println!("GrupoDeSuperficie - implementación para CargueroLigero");

    let mut g5 = GrupoDeSuperficie::cargueros_ligeros("Ligeros".to_string());
    g5.agregar_nave(cargo_ligero);

    g5.cargueros()
        .iter()
        .for_each(|c| println!("Carguero ligero: {} {}", c.nombre, c.carga));

    println!("Grupo: {}", g5.nombre);

    println!("{:?}", g5);
}
