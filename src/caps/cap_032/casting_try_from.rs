use std::f64::consts;

struct Planeta {
    nombre: String,
    radio: f64,
    volumen: f64,
}

#[derive(Debug)]
pub enum ErrorPlaneta {
    ErrorRadioPequeno,
    ErrorRadioNegativo,
}

impl TryFrom<f64> for Planeta {
    type Error = ErrorPlaneta;

    fn try_from(radio: f64) -> Result<Self, Self::Error> {
        if radio <= 0.0 {
            Err(ErrorPlaneta::ErrorRadioNegativo)
        } else if radio <= 10.0 {
            Err(ErrorPlaneta::ErrorRadioPequeno)
        } else {
            Ok(Planeta {
                nombre: String::from("Sphere1"),
                radio: radio,
                volumen: 4.0 / 3.0 * consts::PI * radio.powf(3.0),
            })
        }
    }
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("TryFrom - casting datos complejos");
    println!("--------------------");
    println!();

    let radio = 1200.0; // km    
    if let Ok(sphere1) = Planeta::try_from(radio) {
        println!(
            "{} : radio: {}km , volumen: {:.2}km3",
            sphere1.nombre, sphere1.radio, sphere1.volumen
        );
    };

    let radio = 8.0; // km
    match Planeta::try_from(radio) {
        Ok(sphere1) => {
            println!(
                "{} : radio: {}km , volumen: {:.2}km3",
                sphere1.nombre, sphere1.radio, sphere1.volumen
            );
        }
        Err(ErrorPlaneta::ErrorRadioNegativo) => println!("Error: Radio negativo"),
        Err(ErrorPlaneta::ErrorRadioPequeno) => println!("Error: Radio demasiado pequeño"),
    };

    // try_into  está disponible de forma automática
    let sphere1: Planeta = (100.0).try_into().unwrap();

    println!(
        "{} : radio: {}km , volumen: {:.2}km3",
        sphere1.nombre, sphere1.radio, sphere1.volumen
    );
}
