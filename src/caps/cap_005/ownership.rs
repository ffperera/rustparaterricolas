struct Asteroide {
    _roca: i32,
}

struct Planetoide {
    _roca1: i128,
    _roca2: i128,
    _roca3: i128,
}

fn memoria_stack(a: i32, b: Asteroide, c: Planetoide) {
    println!("------- inside");
    println!("Dirección stack: {:p}", &a);
    println!("Asteroide:       {:p}", &b);

    // se ha pasado por referencia internamente
    println!("Planetoide:      {:p}", &c);
}

// declaración de la función
fn funcion(p1: i32, p2: String) {
    // p1 propietaria del valor (copia)
    // p2 propietaria del contenido (move)
    println!("{} {}", p1, p2);
}

fn retorno(alien_interno: String) -> String {
    // alien_interno es propietaria del contenido
    println!("alien_interno posee {}", alien_interno);

    // en Rust no hace falta escribir return
    // al final de una función
    // return alien_interno;

    // devolver una variable equivale a
    // una asignación hacia el exterior
    alien_interno
}

pub fn run() {
    let en_stack = 1;

    // 4 bytes
    let asteroide = Asteroide { _roca: 1 };

    // 16 * 3  bytes
    let planetoide = Planetoide {
        _roca1: 1,
        _roca2: 1,
        _roca3: 1,
    };

    println!("Dirección stack: {:p}", &en_stack);
    println!("Asteroide:       {:p}", &asteroide);
    println!("Planetoide:      {:p}", &planetoide);

    let asteroide_b = asteroide;
    println!("Asteroide:       {:p}", &asteroide_b);

    let planetoide_b = planetoide;
    println!("Planetoide:      {:p}", &planetoide_b);

    // el compilador optimiza el paso de variables
    // que no implementan Copy ni Drop
    memoria_stack(en_stack, asteroide_b, planetoide_b);

    // -------------------

    // a es una variable propietaria del valor 10
    let a = 10;

    // alien_x es propietaria de "Planeta" (heap)
    let alien_x = String::from("Planeta");

    // llamada a la función
    // la variable 'a' no se consume
    // la variable 'alien_x' se consume
    funcion(a, alien_x);

    println!("{}", a); // ok

    // alien_x se consumió, ya no está activa
    // println!("{}", alien_x); // ERROR

    // -------------------

    // alien_x es propietaria de "Planeta"
    let alien_x = String::from("Planeta");

    println!("alien_x posee {}", alien_x);

    // alien_x mueve el conteido a la función y
    // se consume
    let alien_y = retorno(alien_x);

    //          ^-- la función mueve
    //              el contenido a alien_y

    //    ^-- alien_y es ahora la única
    //        propietaria de "Planeta"

    println!("alien_y posee {}", alien_y);

    // -------------------

    let alien_x = String::from("Urano");

    let alien_y = alien_x.clone();

    // ahora hay 2 "Urano", independientes
    // cada uno de ellos en alguna posición del heap

    // la variable original no se consume
    println!("{} = {}", alien_x, alien_y);

    // ahora forzamos un drop de alien_x
    drop(alien_x);

    // el "Urano" que dependía de alien_x
    // desaparece de memoria

    // pero sigue estando el "Urano" de alien_y
    println!("{}", alien_y);

    // -------------------

    let texto = String::from("Aliens vs Humanos");
    // texto.push_str("!"); // <- ERROR

    // la nueva variable texto es ahora
    // una variable propietaria mutable
    let mut texto = texto;
    texto.push_str("!");

    println!("{}", texto);

    // movemos de nuevo a una variable propietaria inmutable
    let texto = texto;
    // texto.push_str("!"); // <- ERROR

    println!("{}", texto);
}

#[cfg(test)]
mod testing {

    #[test]
    fn test_reescribir_test_no_valido() {
        assert!(false);
    }
}
