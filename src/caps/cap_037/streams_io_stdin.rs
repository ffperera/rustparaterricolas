use std::io::{self, Write};

pub fn run() {
    println!();
    println!("--------------------");
    println!("Entrada estándar: stdin");
    println!("--------------------");
    println!();

    // stdout - salida estándar
    let mut stdout = io::stdout();
    stdout.write_all("Bienvenido Sistema\n".as_bytes()).unwrap();
    stdout.flush().unwrap(); // forzar escritura inmediata

    let mut nombre = String::new();
    stdout
        .write_all("¿Cuál es tu nombre terrestre?".as_bytes())
        .unwrap();

    stdout.flush().unwrap();

    // stdin - entrada estándar
    // let stdin = io::stdin();
    // let mut reader = io::BufReader::new(stdin.lock());
    // reader.read_line(&mut nombre).unwrap();
    // let nombre = nombre.trim();
    //

    // let stdin = io::stdin();
    // let mut handle = stdin.lock();
    // // let mut reader = io::BufReader::new(stdin.lock());
    // handle.read_line(&mut nombre).unwrap();
    // let nombre = nombre.trim();
    //

    let stdin = io::stdin();
    // let mut handle = stdin.lock();
    // let mut reader = io::BufReader::new(stdin.lock());
    stdin.read_line(&mut nombre).unwrap();
    let nombre = nombre.trim();

    // stderr - salida de errores
    let mut stderr = io::stderr();

    if nombre.is_empty() {
        stderr
            .write_all("Error: Nombre vacío\n".as_bytes())
            .unwrap();

        stderr.flush().unwrap();
    } else {
        println!();
        println!("Bienvenido, {}!", nombre);
    }
}
