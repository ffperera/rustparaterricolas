use std::f64::consts;

fn volumen_planeta(radio: f64) -> Option<f64> {
    if radio < 10.0 {
        // no válido
        return None;
    }

    // resultado válido envuelto en Some()
    let volumen = 4.0 / 3.0 * consts::PI * radio.powf(3.0);
    Some(volumen)
}

fn superficie_planeta(radio: f64) -> Result<f64, String> {
    if radio < 10.0 {
        // no válido
        return Err(String::from("radio muy pequeño"));
    }

    // resultado válido envuelto en Some()
    let superficie = 4.0 * consts::PI * radio.powf(2.0);
    Ok(superficie)
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Option / Result");
    println!("--------------------");
    println!();

    // ------------- Option

    // match

    // rama None
    match volumen_planeta(8.0) {
        Some(v) => println!("Volumen: {}", v),
        None => println!("Planeta no válido"),
    }

    // rama Some()
    match volumen_planeta(100.0) {
        Some(v) => println!("Volumen: {}", v),
        None => println!("Planeta no válido"),
    }

    // if let
    if let Some(v) = volumen_planeta(100.0) {
        println!("Volumen: {}", v);
    };

    // unwrap()
    let v = volumen_planeta(100.0).unwrap();
    println!("Volumen: {}", v);

    // expect()
    let v = volumen_planeta(100.0).expect("radio muy pequeño");

    println!("Volumen: {}", v);

    // unwrap_or()
    let v = volumen_planeta(100.0).unwrap_or(0.0);
    println!("Volumen: {}", v);

    // v = 0.0
    let v = volumen_planeta(1.0).unwrap_or(0.0);
    println!("Volumen: {}", v);

    // unwrap_or_else()
    let v = volumen_planeta(100.0).unwrap_or_else(|| {
        let volumen_minimo = 12.8;
        volumen_minimo
    });

    println!("Volumen: {}", v);

    // ------------- Result

    // match

    // rama Err
    match superficie_planeta(8.0) {
        Ok(v) => println!("superficie: {}", v),
        Err(e) => println!("Error {}", e),
    }

    // rama Ok()
    match superficie_planeta(100.0) {
        Ok(v) => println!("superficie: {}", v),
        Err(e) => println!("Error {}", e),
    }

    // if let
    if let Ok(v) = superficie_planeta(100.0) {
        println!("superficie: {}", v);
    };

    // unwrap()
    let v = superficie_planeta(100.0).unwrap();
    println!("superficie: {}", v);

    // expect()
    let v = superficie_planeta(100.0).expect("radio muy pequeño");

    println!("superficie: {}", v);

    // unwrap_or()
    let v = superficie_planeta(100.0).unwrap_or(0.0);
    println!("superficie: {}", v);

    // v = 0.0
    let v = superficie_planeta(1.0).unwrap_or(0.0);
    println!("superficie: {}", v);

    // unwrap_or_else()
    let v = superficie_planeta(1.0).unwrap_or_else(|error| {
        println!("Error: {}", error);

        println!("Se usará valor por defecto");
        let superficie_minima = 40.2;
        superficie_minima
    });

    println!("superficie: {}", v);
}
