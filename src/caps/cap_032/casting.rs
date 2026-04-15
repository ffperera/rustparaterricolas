use std::f64::consts;
use std::str::FromStr;

#[derive(Debug)]
struct Planeta {
    nombre: String,
    radio: f64,
    volumen: f64,
}

impl From<f64> for Planeta {
    fn from(radio: f64) -> Self {
        Planeta {
            nombre: String::from("Sphere1"),
            radio: radio,
            volumen: 4.0 / 3.0 * consts::PI * radio.powf(3.0),
        }
    }
}

impl From<String> for Planeta {
    fn from(nombre: String) -> Self {
        let radio = 100.0;
        Planeta {
            nombre: nombre,
            radio: radio,
            volumen: 4.0 / 3.0 * consts::PI * radio.powf(3.0),
        }
    }
}

impl Planeta {
    fn new(nombre: &str, radio: f64) -> Self {
        Self {
            nombre: String::from(nombre),
            radio,
            volumen: 4.0 / 3.0 * consts::PI * radio.powf(3.0),
        }
    }
}

fn simular_planeta<T: Into<Planeta>>(dato: T) {
    let planeta: Planeta = dato.into();
    println!(
        "Planeta simulado '{}' - Radio: {:.2} km, Volumen: {:.2} km³",
        planeta.nombre, planeta.radio, planeta.volumen
    );
}

impl FromStr for Planeta {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f64>() {
            Ok(radio) => {
                return Ok(
                    Planeta::from(radio), //      ^-- aprovechamos que ya está
                                          //          implementado From<f64>
                );
            }
            Err(_) => return Err("Radio no válido"),
        }
    }
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Casting de datos complejos: From Into");
    println!("--------------------");
    println!();

    let radio = 1200.0; // km
    let sphere1 = Planeta::from(radio);

    println!(
        "{} : radio: {}km , volumen: {:.2}km3",
        sphere1.nombre, sphere1.radio, sphere1.volumen
    );

    let sphere1: Planeta = 1200.0.into();

    println!(
        "{} : radio: {}km , volumen: {:.2}km3",
        sphere1.nombre, sphere1.radio, sphere1.volumen
    );

    let sphere1 = Planeta::new("Sphere", 1200.0);
    println!(
        "{} : radio: {}km , volumen: {:.2}km3",
        sphere1.nombre, sphere1.radio, sphere1.volumen
    );

    // error --  no implementado para i32
    // simular_planeta(20);

    // ok -- f64 sí es un dato válido
    simular_planeta(120.0);

    // ok -- String sí es un dato valido
    simular_planeta(String::from("Planetoide"));

    // Ok(Planeta { nombre: "Sphere1",
    //              radio: 127.9,
    //              volumen: 8763957.094699219 })

    let _planeta: Planeta = 1200.0.into();

    // casting - FromStr / parse()
    let planeta = "hola".parse::<Planeta>();
    println!("{:?}", planeta);
    // Err("Radio no válido")

    let planeta = "127.9".parse::<Planeta>();
    println!("{:?}", planeta);

    let planeta: Planeta = "127.9".parse().unwrap();
    println!("{:?}", planeta);
}
