/// Capítulo 4: Declaraciones y tipos de datos.
///
/// En este capítulo vemos ejemplos relacionados con declaración de variables,
/// tipos de datos, constantes y shadowing.
const PI_APROX: f64 = 3.1416;

const GIGA_BYTE: u64 = 1024 * 1024 * 1024;
const TERA_BYTE: u64 = 1024 * GIGA_BYTE;
const PETA_BYTE: u64 = 1024 * TERA_BYTE;

pub fn run() {
    println!();
    println!("--------------------");
    println!("Declaración y shadowing");
    println!("--------------------");
    println!();

    println!("PI aprox {}", PI_APROX);

    // declaración
    let a: i32;
    let b: bool;
    let c: String;

    // inicialización
    a = 2;
    b = true;
    c = String::from("Saludos, terrícolas.");

    println!("{} {} {}", a, b, c);

    // declaración explícita + inicialización
    let a: i32 = 27;
    let b: bool = false;
    let c: String = String::from("Saludos, terrícolas.");

    println!("{} {} {}", a, b, c);

    // declaración implícita + inicialización
    let a = 27; // infiere i32
    let b = false; // infiere bool
    let c = String::from("Saludos, terrícolas."); // String

    println!("{} {} {}", a, b, c);

    // a es inmutable
    let a: i32; // declaración
    a = 25; // inicialización

    // a = 27;      // < ERROR
    // cannot assign twice to immutable variable

    // a es mutable
    let mut b: i32; // declaración
    b = 25; // inicialización

    println!("{} {}", a, b);

    b = 27; // modificación
    println!("{} {}", a, b);

    // shadowing
    let a = 27;
    println!("shadowing {}", a); // 27

    {
        // nuevo scope
        let a = 3;
        println!("shadowing {}", a); // 3
                                     // termina scope
    }

    println!("shadowing {}", a); // 27

    // literales enteros
    let num = 1__0000_00;

    let dec = 1000i64;
    let dec1 = 1000_i64;
    let suma = 12i64 + 18i64;

    let hex = 0xABCDi32;
    let hex1 = 0xCCC;

    let oct = 0o35;
    let oct1 = 0o217;

    // byte = u8
    let byte = b'a'; // ascii
    let byte1 = b'c'; // ascii

    println!("{num} {suma} {dec} {hex} {oct} {byte}");
    println!("{dec1} {hex1} {oct1} {byte1}");

    let char = 'Ñ';
    println!("{} UTF: {}", char, char as u32);

    let char = 'á';
    println!("{} UTF: {}", char, char as u32);

    let char = 'ü';
    println!("{} UTF: {}", char, char as u32);

    let verdadero = true;
    let falso = !verdadero;

    let or = verdadero || falso;
    let and = verdadero && falso;

    println!("{} {} {} {}", verdadero, falso, or, and);

    let verdadero = 5 > 2;
    let falso = 3 == 7;

    println!("{} {} {} {}", verdadero, falso, 2 != -2, 2 == 2);

    let _vacio = ();
    let vacio: () = ();

    println!("Unidad: {:?}", vacio); // ()

    // comportamiento similar a void de C
    fn hacer_nada() -> () {
        println!("Deja tu mente en blanco, terrícola.");
    }

    let nada = hacer_nada();

    println!("Valor devuelto por función: {:?}", nada); // ()

    // valor devuelto por una expresión
    // que no devuelve 'algo'
    let nada = {
        let _ = 12 + 4;
    };

    println!("Bloque sin expresión: {:?}", nada); // ()

    type Metros = f64;
    type Litros = f64;

    let a: Metros = 12.1;
    let b: Litros = 13.4;

    println!("Uso de type: {}m {}l", a, b);

    // constantes
    {
        const PI_LOCAL: f64 = 3.14;
        println!("Constante local: {}", PI_LOCAL);
        println!("Constante global: {}", std::f64::consts::PI);
    }

    // ERROR
    // println!("Constante local: {}", PI_LOCAL);

    println!(
        "Constantes globales: {} | {} | {} ",
        GIGA_BYTE, TERA_BYTE, PETA_BYTE
    );

    // static
}
