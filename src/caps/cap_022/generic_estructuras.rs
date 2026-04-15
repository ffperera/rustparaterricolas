// use std::fmt::Display;

trait Transportar {
    fn carga(&mut self, cantidad: i32) -> i32;
    fn descarga(&mut self, cantidad: i32) -> i32;
}

trait Aterrizar {
    fn desciende(&self) -> bool;
    fn despega(&self) -> bool;
    fn ir_a_orbita(&self) -> bool;
}

// trait OtroTrait {}

#[derive(Debug)]
struct Transbordador {
    nombre: String,
    carga: i32,
}

impl Transbordador {
    fn new(nombre: String) -> Self {
        Transbordador { nombre, carga: 0 }
    }
}

impl Transportar for Transbordador {
    fn carga(&mut self, cantidad: i32) -> i32 {
        self.carga += cantidad;
        self.carga
    }
    fn descarga(&mut self, cantidad: i32) -> i32 {
        self.carga -= cantidad;
        self.carga
    }
}

impl Aterrizar for Transbordador {
    fn desciende(&self) -> bool {
        true
    }
    fn despega(&self) -> bool {
        true
    }
    fn ir_a_orbita(&self) -> bool {
        true
    }
}

// definición con restriccion
/*
struct  GrupoDeSuperficie <T>
where T: Transportar + Aterrizar
{
    nombre: String,
    naves: Vec<T>,
}
*/

// definición sin restriccion

#[derive(Debug)]
struct GrupoDeSuperficie<T> {
    nombre: String,
    naves: Vec<T>,
}

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

    fn con_naves(nombre: String, naves: Vec<T>) -> Self {
        GrupoDeSuperficie { nombre, naves }
    }
}

// conflicto con new()
// sólo puede haber un método new() para la implementación
// de GrupoDeSuperficie, aunque se apliquen diferentes restricciones

// impl <T> GrupoDeSuperficie<T>
// where T: OtroTrait
// {
//     fn new(nombre: String) -> Self {
//         GrupoDeSuperficie {
//             nombre,
//             naves: Vec::<T>::new(),
//         }
//     }
// }

// implementación concreta

#[derive(Debug)]
struct CargueroLigero {
    nombre: String,
    carga: i32,
}

impl CargueroLigero {
    fn new(nombre: String) -> Self {
        CargueroLigero { nombre, carga: 0 }
    }
}

impl Transportar for CargueroLigero {
    fn carga(&mut self, cantidad: i32) -> i32 {
        self.carga += cantidad;
        self.carga
    }
    fn descarga(&mut self, cantidad: i32) -> i32 {
        self.carga -= cantidad;
        self.carga
    }
}

impl Aterrizar for CargueroLigero {
    fn desciende(&self) -> bool {
        true
    }
    fn despega(&self) -> bool {
        true
    }
    fn ir_a_orbita(&self) -> bool {
        true
    }
}

impl GrupoDeSuperficie<CargueroLigero> {
    // fn new(nombre: String) -> Self {
    //     GrupoDeSuperficie {
    //         nombre,
    //         naves: Vec::<CargueroLigero>::new(),
    //     }
    // }

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

#[derive(Debug)]
struct OtroTransporte {
    nombre: String,
    carga: i32,
}

impl GrupoDeSuperficie<OtroTransporte> {
    fn info(&self) {
        println!("Soy OtroTransporte");
    }
}

trait Saludar {
    fn hola(&self);
    fn adios(&self);
}

impl<T> Saludar for GrupoDeSuperficie<T> {
    fn hola(&self) {
        println!("Hola, somos el grupo {}", self.nombre);
    }

    fn adios(&self) {
        println!("Vuelve cuando quieras al grupo {}", self.nombre);
    }
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Estructuras genéricas");
    println!("--------------------");
    println!();

    let mut t1 = Transbordador {
        nombre: "T1".to_string(),
        carga: 0,
    };

    t1.carga(100);
    t1.descarga(50);

    t1.desciende();
    t1.despega();
    t1.ir_a_orbita();

    println!("Transbordador {} en órbita.", t1.nombre);

    let t2 = Transbordador {
        nombre: "T2".to_string(),
        carga: 0,
    };

    // declaración implícita a partir de inicialización
    let g1 = GrupoDeSuperficie {
        nombre: "G1".to_string(),
        naves: vec![t1, t2],
    };

    _ = g1;

    let t1 = Transbordador::new("T1".to_string());
    let t2 = Transbordador::new("T2".to_string());

    let ligero = CargueroLigero::new("L1".to_string());
    println!("Carguero Ligero {}", ligero.nombre);

    let otro = OtroTransporte {
        nombre: "O1".to_string(),
        carga: 100,
    };
    println!("Otro Transporte:  {}, carga: {}", otro.nombre, otro.carga);

    let otro_grupo: GrupoDeSuperficie<OtroTransporte> = GrupoDeSuperficie {
        nombre: "OtroGrupo".to_string(),
        naves: vec![otro],
    };

    otro_grupo.info();

    // declaración explícita + inicialización
    let g2: GrupoDeSuperficie<Transbordador> = GrupoDeSuperficie {
        nombre: "G2".to_string(),
        naves: vec![t1, t2],
    };

    _ = g2;

    // declaración sin inicialización
    let g3: GrupoDeSuperficie<Transbordador>;
    _ = g3;
    // println!("Grupo {:?}", g3);

    // declaración implícita a través de new()
    // let mut g1 = GrupoDeSuperficie::new("G2".to_string());
    //              ^--- error, el compilador no puede determinar <T>

    // declaración explícita + inicialización
    // mediante el método new()
    let g3: GrupoDeSuperficie<Transbordador> = GrupoDeSuperficie::new("G2".to_string());
    println!("Grupo {:?}", g3);

    // declaración implícita
    // pero la inicialización incluye el tipo
    // de forma explicita
    let g3 = GrupoDeSuperficie::<Transbordador>::new("G3".to_string());
    println!("Grupo {:?}", g3);

    // declaración, tiene que ser explícita
    let g4: GrupoDeSuperficie<Transbordador>;
    _ = g4;

    // inicialización implícita
    // mediante el método con_naves()

    let t1 = Transbordador::new("T1".to_string());
    let t2 = Transbordador::new("T2".to_string());

    let g4 = GrupoDeSuperficie::con_naves("GR1".to_string(), vec![t1, t2]);
    _ = g4;

    //
    let g5 = GrupoDeSuperficie::cargueros_ligeros("Ligeros".to_string());
    _ = g5;

    let mut g6 = GrupoDeSuperficie::cargueros_ligeros("Ligeros".to_string());

    g6.naves.push(CargueroLigero::new("Ligero1".to_string()));

    g6.naves.push(CargueroLigero::new("Ligero2".to_string()));

    println!("Cargueros ligeros: {:?}", g6.cargueros());

    // println!("Transbordadores: {:?}", g4.cargueros());
    // ERROR
    // no method named `cargueros` found for struct
    // `GrupoDeSuperficie<Transbordador>` in the current scope

    // implementación de un trait sobre una
    // estructura genérica

    let g5 = GrupoDeSuperficie::cargueros_ligeros("Ligeros".to_string());

    g5.hola();
    g5.adios();

    let t1 = Transbordador::new("T1".to_string());
    let t2 = Transbordador::new("T2".to_string());

    let g4 = GrupoDeSuperficie::con_naves("GR1".to_string(), vec![t1, t2]);

    g4.hola();
    g4.adios();
}
