// use std::fs;
use std::fs::File;
use std::io::Read;

pub fn run() {
    println!();
    println!("--------------------");
    println!("Entrada / Salida: abrir y crear ficheros");
    println!("--------------------");
    println!();

    let path = "./src/texto_externo.txt";

    let mut file = File::open(path).unwrap();

    let mut contenido = String::new();
    file.read_to_string(&mut contenido).unwrap();

    println!("Fichero abierto con open()");

    println!();
    println!("Usando options()");

    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true) // crea si no existe
        .truncate(false) // añadir / no truncar
        .open(path)
        .unwrap();

    // Leer contenido existente
    let mut contenido = String::new();
    file.read_to_string(&mut contenido).unwrap();
}
