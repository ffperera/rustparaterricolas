// use core::num;
// use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

pub fn run() {
    println!();
    println!("--------------------");
    println!("Entrada / Salida: seek | acceso aleatorio");
    println!("--------------------");
    println!();

    let path = "./src/seek.txt";
    match seek_test(path) {
        Err(e) => println!("Error {}", e),
        Ok(()) => println!("ok"),
    }
}

fn seek_test(path: &str) -> std::io::Result<()> {
    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true) // crea si no existe
        .truncate(true) // añadir / no truncar
        .open(path)?;

    // escribir algunos datos
    file.write_all(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ")?;

    // mover el puntero al byte 10 (carácter K)
    file.seek(SeekFrom::Start(10))?;

    // Verificar la posición actual
    let pos = file.stream_position()?;
    println!("Posición actual: {}", pos);
    // 10

    // leer desde la posición actual

    let mut buffer = [0; 5];
    file.read_exact(&mut buffer)?;
    println!("Leído: {}", String::from_utf8_lossy(&buffer));

    // KLMNO

    // moverse desde la posición actual
    file.seek(SeekFrom::Current(-3))?; // Retrocede 3 bytes
    println!(
        "Posición después de retroceder: {}",
        file.stream_position()? // 12
    );

    // moverse desde el final
    file.seek(SeekFrom::End(-5))?;
    // 5 bytes antes del final

    println!("Posición desde el final: {}", file.stream_position()?);
    // 21 (V)

    Ok(())
}
