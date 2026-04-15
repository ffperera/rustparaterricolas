// los parámetros pueden ser mutables
// se comportan como variables locales
fn f(mut a: i32) -> i32 {
    a += 1;
    a
}

fn alien_sum(a: i32, b: i32) -> i32 {
    if b > a {
        return b + a;
    }

    a + b
}

fn slice_param(nombre: &str) -> String {
    String::from(nombre)
}

// fn nombre_planeta_ref() -> &String {
//     let nombre = String::from("Plutonia");
//
//     // intentamos devolver una referencia
//     // a la variable propietaria nombre
//     &nombre
// }

fn otra_funcion() {
    let a = alien_sum(2, 2);
    println!("Alien sum función: {a}");

    // a partir de aquí, alien_sum es
    // una variable i32
    let alien_sum = 47;
    println!("Alien sum shadowing: {alien_sum}");

    // let resultado = 4 + alien_sum(2, 2) * 5;
    // println!("resultado: {resultado}");
}

#[allow(unused_variables)]
fn funcion_externa() {
    let a = 1;

    fn funcion_interna() {
        // println!("a: {a}");
    }

    funcion_interna();
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Funciones en Rust");
    println!("--------------------");
    println!();

    funcion_externa();

    // ejemplo con parámetro mutable
    println!("Ejemplo con parámetro mutable: {}", f(10));

    // ejemplo scope
    // el scope depende de la declaración,
    // no de la inicialización.
    let a: i32;

    {
        a = 10;
    }

    println!("{a}");

    // let a = alien_sum(a:2, b:2);
    // let a = alien_sum(a=2, b=2);
    let a = alien_sum(2, 2);
    println!("Alien sum: {a}");

    let resultado = 4 + alien_sum(2, 2) * 5;
    println!("resultado: {resultado}");

    let nombre1 = "Sphere A";
    let slice1 = &nombre1[..];

    // los slices son referencias
    // no son propietarias, no se consumen
    println!("slice1: {}", slice_param(slice1));
    println!("slice1: {}", slice1);

    fn nombre_planeta() -> String {
        String::from("Plutonia")
    }

    // planeta es ahora propietaria
    // del String devuelto por la función
    let planeta = nombre_planeta();

    println!("Nombre del planeta: {}", planeta);

    // nadie captura el String devuelto
    // por la función
    nombre_planeta();

    // el ownership es capturado
    // por una variable temporal
    println!("Planeta: {}", nombre_planeta());

    // funciones como variables
    //
    // shadowing
    otra_funcion();

    // alien_sum sigue siendo función en este scope
    let resultado = 4 + alien_sum(2, 2) * 5;
    println!("resultado: {resultado}");

    // asignar una función a una variable
    let f = alien_sum;

    let resultado = f(2, 3);
    println!("Resultado f(2,3) : {}", resultado);
    println!("Resultado alien_sum : {}", alien_sum(2, 2));

    fn genera(cuerpo_celeste: fn()) {
        println!("Vamos a generar un... ");
        cuerpo_celeste();
    }

    fn genera_planeta() {
        println!("Planeta");
    }

    fn genera_asteroide() {
        println!("Asteroide");
    }

    // llamadas a genera con diferentes callbacks
    genera(genera_planeta);
    genera(genera_asteroide);

    // planeta es un function pointer
    let planeta = genera_planeta;
    genera(planeta);
}
