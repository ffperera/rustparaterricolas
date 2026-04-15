// use core::num;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

pub fn run() {
    println!();
    println!("--------------------");
    println!("Entrada / Salida: ficheros");
    println!("--------------------");
    println!();

    let path = "./src/texto_externo.txt";

    // fs::read_to_string()
    test_fs_read_to_string(path);

    // File::read_to_string()
    test_file_read_to_string(path);

    // fs::read()
    test_fs_read(path);

    // File::read_to_end()
    test_file_read_to_ennd(path);

    // fichero de texto
    // contenido dividido en líneas (trozos separados por \n)
    // BufReader::lines()
    test_leer_fichero_texto_linea_a_linea(path);

    // fichero de texto
    // linea a linea con read_line
    // BufReader::read_line()
    test_leer_fichero_texto_linea_a_linea_con_read_line(path);

    // fichero
    // byte a byte
    test_bytes(path);

    // fichero
    // byte a byte
    test_file_bytes(path);

    // fichero binario
    // leer y procesar trozos de N bytes
    test_fichero_binario_chunks(path);

    // listar contenido de un directorio
    //
    let dir = "./data";
    test_listar_contenido_directorio(dir);
    //
    // comprobar si existe un fichero

    test_file_existe("./data/saludo.txt");

    // o simplemente
    if fs::exists("./data/saludo.txt").unwrap() {
        println!("El fichero existe.");
    }

    // create_dir() + create_dir_all()
    test_create_dir();

    // abrir fichero para escritura
    test_abrir_fichero_escritura();

    // File::write()
    test_file_write(path);

    // File::write_all()
    test_file_write_all(path);

    // BufWriter
    test_bufwriter(path);

    // borramos el fichero que acabamos de crear
    fs::remove_file(path).unwrap();
}

fn test_fs_read_to_string(path: &str) {
    // fs::read_to_string()
    // let path = "./src/texto_externo.txt";
    let contenido = fs::read_to_string(path);
    println!("fs::read_to_string()");

    match contenido {
        Ok(texto) => println!("Contenido: {}", texto),
        Err(error) => println!("{:?}", error),
    }
}

fn test_file_read_to_string(path: &str) {
    println!("File::read_to_string()");

    let mut archivo = File::open(path).unwrap();
    let mut contenido = String::new();
    let num_bytes = archivo.read_to_string(&mut contenido).unwrap();

    println!("Contenido: {}, bytes {}", contenido, num_bytes);

    // podríamos cerrar el fichero de forma explícita
    // con drop() si fuera neceario
    drop(archivo);
}

fn test_fs_read(path: &str) {
    println!("fs::read()");

    let contenido = fs::read(path);

    match contenido {
        Ok(data) => println!("Contenido: {:?}", &data[..12]),
        Err(error) => println!("{:?}", error),
    }
}

fn test_file_read_to_ennd(path: &str) {
    // File::read_to_end()
    println!("File::read_to_end()");

    let mut archivo = File::open(path).unwrap();
    let mut contenido = Vec::new();
    let num_bytes = archivo.read_to_end(&mut contenido).unwrap();

    println!("Contenido: {:?}", &contenido[..12]);
    println!("bytes {}", num_bytes);
}

fn test_leer_fichero_texto_linea_a_linea(path: &str) {
    println!("BufReader::lines()");

    // abrir el archivo
    let archivo = File::open(path).unwrap();

    // crear BufReader para lectura eficiente
    let lector = BufReader::new(archivo);

    // leer línea por línea
    for (num_linea, linea) in lector.lines().enumerate() {
        // error de lectura?
        let contenido = linea.unwrap_or(String::new());

        if contenido.is_empty() {
            println!("L {}: <vacía>", num_linea + 1);
        } else {
            println!(
                "L {}: {} ({} caracteres)",
                num_linea + 1,
                contenido,
                contenido.len()
            );
        }
    }
}

fn test_leer_fichero_texto_linea_a_linea_con_read_line(path: &str) {
    println!("BufReader::read_line()");

    let archivo = File::open(path).unwrap();

    // lector BufReader
    let mut lector = BufReader::new(archivo);
    let mut num_linea = 0;
    let mut buffer = String::new();

    // buble para leer línea a línea
    loop {
        buffer.clear();

        match lector.read_line(&mut buffer) {
            Ok(0) => break, // Fin del archivo
            Ok(num_bytes) => {
                let contenido = buffer.trim_end().to_string();

                if contenido.is_empty() {
                    println!("L {}: <vacía>", num_linea + 1);
                } else {
                    println!(
                        "L {}: {} ({} caracteres, {} bytes)",
                        num_linea + 1,
                        contenido,
                        contenido.len(),
                        num_bytes
                    );
                }
                num_linea += 1;
            }
            Err(e) => {
                eprintln!("Error leyendo línea {}: {}", num_linea + 1, e);
                break;
            }
        }
    }
}

fn test_bytes(path: &str) {
    println!("BufReader::bytes()");

    let archivo = File::open(path).unwrap();
    let lector = BufReader::new(archivo);

    let mut contador = 0;

    // leer byte a byte
    for byte in lector.bytes() {
        match byte {
            Ok(b) => print!("{:?} ", b),
            Err(_) => println!("\nError"),
        }

        contador += 1;
        if contador > 15 {
            break;
        }
    }
}

fn test_file_bytes(path: &str) {
    println!("File::bytes() (sin BufReader)");

    let archivo = File::open(path).unwrap();
    // let lector = BufReader::new(archivo);

    let mut contador = 0;

    // leer byte a byte
    for byte in archivo.bytes() {
        match byte {
            Ok(b) => print!("{:?} ", b),
            Err(_) => println!("\nError"),
        }

        contador += 1;
        if contador > 15 {
            break;
        }
    }
}

fn test_fichero_binario_chunks(path: &str) {
    println!("BufReader: fill_buf() + consume() ");
    // abrir el archivo
    let archivo = File::open(path).unwrap();

    // crear BufReader para lectura eficiente
    // utilizar un buffer intermedio de 10 bytes
    // n = 10
    //
    // 'lector' tiene que ser mutable para gestionar
    // su estado interno
    let mut lector = BufReader::with_capacity(10, archivo);

    // leer trozos de n bytes
    loop {
        let buffer = lector.fill_buf().unwrap();
        let num_bytes = buffer.len();

        if num_bytes == 0 {
            break;
        }

        // for i in 0..buffer.len() {
        //     print!("{} ", buffer[i]);
        // }

        for chunk in buffer {
            print!("{} ", chunk);
        }

        lector.consume(num_bytes);
        println!();
    }

    drop(lector);

    println!();
}

fn test_listar_contenido_directorio(dir: &str) {
    println!("Listar contenido de {}", dir);

    // let dir = "./data";
    let entries = fs::read_dir(dir).unwrap();

    for entry in entries {
        let entry = entry.unwrap();

        // std::path
        let path = entry.path();

        // nombre del archivo
        let fichero = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("[nombre no válido]");

        // Determinar tipo de entrada
        let metadata = fs::metadata(&path).unwrap();
        let file_type = if metadata.is_dir() {
            "DIR"
        } else if metadata.is_file() {
            "ARCHIVO"
        } else {
            "OTRO"
        };

        println!("{:<30} {:<10} {:<20}", fichero, file_type, metadata.len());
    }

    println!();
}

fn test_file_existe(path: &str) {
    println!("Comprobar si fichero existe: fs::exists()");

    if let Ok(existe) = fs::exists(path) {
        if existe {
            println!("El fichero existe.");
        } else {
            println!("No está.");
        }
    }
}

fn test_file_write(path: &str) {
    println!("File::write()");

    // File::write()
    let texto = "Texto muy grande, mucha información";
    let mut archivo = File::create(path).unwrap();

    let texto = texto.as_bytes();
    let total_bytes = texto.len();
    let mut pos = 0;

    loop {
        let num_bytes = archivo.write(&texto[pos..]).unwrap();
        pos += num_bytes;

        if pos >= total_bytes {
            break;
        }
    }

    let contenido = fs::read_to_string(path).unwrap();
    println!("Contenido: {}", contenido);
}

fn test_file_write_all(path: &str) {
    println!("File::write_all()");

    // File::write_all()
    let texto = "Fichero de claves públicas";
    let mut archivo = File::create(path).unwrap();

    archivo.write_all(texto.as_bytes()).unwrap();

    let contenido = fs::read_to_string(path).unwrap();
    println!("Contenido: {}", contenido);
}

fn test_bufwriter(path: &str) {
    println!("BufWriter");

    // escritura con BufWriter
    let texto = ["Contenido ", "formado ", "por ", "pequeños ", "trozos."];

    let archivo = File::create(path).unwrap();
    let mut buffer = BufWriter::new(archivo);

    for txt in &texto {
        // escribimos al buffer intermedio
        buffer.write_all(txt.as_bytes()).unwrap();
    }

    // enviamos todo al fichero
    buffer.flush().unwrap();

    let contenido = fs::read_to_string(path).unwrap();
    println!("Contenido: {}", contenido);
}

fn test_create_dir() {
    // `create_dir()` y `create_dir_all()`
    println!("create_dir() + create_dir_all()");

    let path = "./data/nuevo";
    fs::create_dir(path).unwrap();

    if fs::exists(path).unwrap() {
        println!("Directorio creado.");
        fs::remove_dir(path).unwrap();
        println!("Directorio eliminado.");
    }

    let path = "./data/nuevo/otro";
    fs::create_dir_all(path).unwrap();

    if fs::exists(path).unwrap() {
        println!("Directorios creados.");
        fs::remove_dir_all("./data/nuevo").unwrap();
        println!("Directorios eliminados.");
    }
}

fn test_abrir_fichero_escritura() {
    println!("Abrir fichero en modo escritura, File::create()");
    // 1 - crear un fichero
    let path = "./data/nuevo_fichero.txt";

    let mut nuevo = File::create(path).unwrap();
    write!(nuevo, "Fichero de claves secretas.").unwrap();

    // 2 - abrir un fichero que ya existe
    println!("Abrir fichero que ya existe, File::create()");

    let mut existente = File::create(path).unwrap();

    // el contenido original del fichero se borra
    // (se trunca el contenido)
    write!(existente, "Fichero de claves secretas.").unwrap();

    println!("Abrir fichero para añadir contenido, options.append()");
    // 3 - fichero existente, con open()
    //   options()
    //          .read(true)
    //          .write(true)
    //          .create(true)
    //          .append(true)
    let mut fichero = File::options().append(true).open(path).unwrap();

    write!(fichero, "!!").unwrap();

    // escritura en ficheros

    // fs::write()
    println!("Escribir con fs::write()");

    let texto = "Fichero de claves públicas";
    fs::write(path, texto).unwrap();

    // equivalente a
    fs::write(path, texto.as_bytes()).unwrap();

    println!("Datos a {}", path);

    let contenido = fs::read_to_string(path).unwrap();
    println!("Contenido: {}", contenido);

    // macro write!()
    let texto = "Otra información importante.";
    let mut archivo = File::create(path).unwrap();
    write!(archivo, "Macro write!(): {}", texto).unwrap();

    let contenido = fs::read_to_string(path).unwrap();
    println!("Contenido: {}", contenido);
}
