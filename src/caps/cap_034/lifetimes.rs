// la referencia devuelta por la función
// sólo puede estar asociada a la referencia 's'
// el compilador infiere que ambas
// tienen el mismo lifetime
fn primer_caracter(s: &str) -> &str {
    if s.is_empty() { &s[..1] } else { s }
}

// devuelve un slice al primer caracter de s1 o s2
// (según su posición en el alfabeto)
fn ordenar<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // para simplificar la explicación suponemos
    // que s1.len() > 1 y s2.len() > 1

    if s1[..1] < s2[..1] {
        &s1[..1]
    } else {
        &s2[..1]
    }
}

// ahora el lifetime de la referencia
// de salida sólo depende del lifetime de s1
fn ordenar2<'a>(s1: &'a str, s2: &str) -> &'a str {
    println!("s2: {}", s2);
    &s1[..1]
}

fn funcion<'a, 'b>(r1: &'a str, r2: &'b [i32]) -> &'a str {
    println!("{} : {:?}", r1, r2);
    r1
}

struct Alien<'a, 'b> {
    r1: &'a str,
    r2: &'b [i32],
}

impl<'a, 'b> Alien<'a, 'b> {
    fn new(r1: &'a str, r2: &'b [i32]) -> Self {
        Alien { r1, r2 }
    }
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Lifetimes");
    println!("--------------------");
    println!();

    // texto1 es propietaria del contenido "Trantor I"
    let texto1 = "Trantor I".to_string();

    // pasamos una referencia (&str)
    // devuelve un slice &str que apunta a "Trantor I"
    let s1 = primer_caracter(texto1.as_str());

    println!("El primer caracter de '{}' es: {}", texto1, s1);

    /*
        fn ordenar(s1: &str, s2: &str) -> &str {
            // para simplificar la explicación suponemos
            // que s1.len() > 1 y s2.len() > 1

            if s1[..1] < s2[..1] {
                &s1[..1]
            } else {
                &s2[..1]
            }
        }
    */

    let texto1 = "Trantor I".to_string();
    let texto2 = "Saturno V".to_string();

    let ref1 = texto1.as_str();
    let ref2 = texto2.as_str();

    // 'resultado' está vinculada al lifetime
    // de ref1 y ref2
    let resultado = ordenar(ref1, ref2);

    println!("Aquí está activo el lifetime: {:?}", resultado);

    drop(texto2);

    // texto2 deja de estar activo
    // ref2 deja de estar activa
    // el lifetime común ('a) deja de estar activo
    // resultado deja de estar activo

    // la siquiente línea dará error
    // println!("{:?}", resultado);

    // la siquiente línea está bien
    // porque el lifetime de ref1
    // depende de su variable propietaria
    println!("texto1 a través de ref1: {:?}", ref1);

    let texto1 = "Trantor I".to_string();
    let texto2 = "Saturno V".to_string();

    let ref1 = texto1.as_str();
    let ref2 = texto2.as_str();

    let resultado = ordenar2(ref1, ref2);

    drop(texto2);

    // 'resultado' sólo está vinculada a texto1,
    // por lo tanto esta referencia
    // sigue activa aunque desaparezca texto2
    println!("Primer caracter de {} es: {:?}", ref1, resultado);

    // declaración de lifetimes

    let saludo = String::from("Terrícolas!!");
    let ref_saludo = saludo.as_str();

    let lista = [1, 2, 3, 4];

    let ref2 = funcion(ref_saludo, &lista[..2]);
    println!("{}", ref2);

    // estructura con lifetimes

    let alien = Alien::new(ref_saludo, &lista[..2]);
    println!("{} | {}", alien.r1, alien.r2[0]);

    let Alien {
        r1: _texto,
        r2: listado,
    } = alien;

    drop(saludo);

    // println!("{}", texto); // ERROR
    println!("{}", listado[0]);
}
