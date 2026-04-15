// use std::f64::consts;

#[derive(Debug)]
struct PlanetaAlienigena {
    nombre: String,
    distancia: i32,
}

fn new_planeta(nombre: String, distancia: i32) -> PlanetaAlienigena {
    PlanetaAlienigena { nombre, distancia }
}

// estructuras de tupla

struct Velocidad(f64, f64, f64);
struct Posicion(f64, f64, f64);

#[allow(dead_code)]
struct Nave {
    nombre: String,
    posicion: Posicion,
    velocidad: Velocidad,
}

// usando tuplas normales
fn destino_tupla(posicion: (f64, f64, f64)) {
    // realizar cálculos con la posición
    println!("Rumbo a: {}, {}, {}", posicion.0, posicion.1, posicion.2);
}

// usando un tipo de dato como parámetro
// (estructura de tupla)
fn destino_struct(posicion: Posicion) {
    // realizar cálculos con la posición
    println!("Rumbo a: {}, {}, {}", posicion.0, posicion.1, posicion.2);
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Structs");
    println!("--------------------");
    println!();

    // la declaración es como
    // la de cualquier otro tipo

    // declaración sin inicialización
    let p0: PlanetaAlienigena;

    // inicialización posterior
    p0 = PlanetaAlienigena {
        nombre: "Prust I".to_string(),
        distancia: 54,
    };

    println!("{} {}", p0.nombre, p0.distancia);

    // declaración con inicialización
    let p1 = PlanetaAlienigena {
        nombre: "Prust I".to_string(),
        distancia: 54,
    };

    println!("{} {}", p1.nombre, p1.distancia);

    let p2 = PlanetaAlienigena {
        nombre: "Drust III".to_string(),
        distancia: 76,
    };

    println!("{} {}", p2.nombre, p2.distancia);

    // inicialización simplificada
    let nombre = "Trust I".to_string();
    let distancia = 227;

    let p3 = PlanetaAlienigena { nombre, distancia };

    println!("{} {}", p3.nombre, p3.distancia);

    // constructor básico
    // a través de función estándar
    let prust_uno = new_planeta("Prust I".to_string(), 54);

    println!("{} {}", prust_uno.nombre, prust_uno.distancia);

    // estructuras de tupla
    let pos = Posicion(0.0, 0.0, 0.0);
    println!("Posición: {}, {}, {}", pos.0, pos.1, pos.2);

    let v = Velocidad(100.0, 0.0, 0.0);
    println!("Velocidad: {}, {}, {}", v.0, v.1, v.2);

    // posición y velocidad con tuplas normales
    //
    let pos2 = (0.0, 0.0, 0.0);
    let v2 = (100.0, 0.0, 0.0);

    // no puede diferenciar posición y velocidad
    destino_tupla(pos2);
    destino_tupla(v2);

    destino_struct(pos);

    // destino_struct(v);
    // ERROR: expected `Posicion`, found `Velocidad`

    //
    // match con estructuras
    //

    let p1 = PlanetaAlienigena {
        nombre: "Prust I".to_string(),
        distancia: 44,
    };

    match p1 {
        PlanetaAlienigena {
            nombre,
            distancia: ..50,
        } => println!("{} está muy cerca.", nombre),
        PlanetaAlienigena { nombre, distancia } => println!("{}, {}", nombre, distancia),
    }

    let p1 = PlanetaAlienigena {
        nombre: "Prust I".to_string(),
        distancia: 44,
    };

    match p1 {
        // no nos interesa el valor de nombre
        // capturamos la distancia exacta en la variable d
        PlanetaAlienigena {
            nombre: _,
            distancia: d @ ..50,
        } => println!("{} está muy cerca.", d),
        PlanetaAlienigena { nombre, distancia } => println!("{}, {}", nombre, distancia),
    }

    // comodín .. para evitar escribir todos los campos
    // siempre va en la última posición de la lista de campos
    let p1 = PlanetaAlienigena {
        nombre: "Prust I".to_string(),
        distancia: 44,
    };

    match p1 {
        PlanetaAlienigena {
            distancia: ..50, ..
        } => println!("Está muy cerca."),
        PlanetaAlienigena { nombre, .. } => println!("Planeta {}", nombre),
    }
}

#[cfg(test)]
mod testing {

    use super::*;

    #[test]
    fn test_creacion_estructura() {
        let p0 = PlanetaAlienigena {
            nombre: "Prust I".to_string(),
            distancia: 54,
        };

        assert!(p0.nombre.contains("Prust I"));
        assert_eq!(p0.distancia, 54);
    }

    #[test]
    fn test_constructor_funcion_externa() {
        let prust_uno = new_planeta("Prust I".to_string(), 54);

        assert_eq!(prust_uno.nombre, "Prust I");
    }
}
