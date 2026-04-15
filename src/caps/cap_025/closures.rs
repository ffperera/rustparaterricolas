// esta función es de tipo {alien_sum}
fn alien_sum(a: i32, b: i32) -> i32 {
    a + b
}

// esta función es de tipo {human_mul}
fn human_mul(a: i32, b: i32) -> i32 {
    a * b
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Closures y function pointers");
    println!("--------------------");
    println!();

    let sum = alien_sum;
    let mul = human_mul;

    sum(2, 2);
    mul(2, 3);

    // sum y mul son de tipos diferentes

    // no podríamos hacer esto:
    /*
    let mut sum = alien_sum;
    sum = human_mul; // ERROR
    sum(2,2);
    */

    let pointer1: fn(i32, i32) -> i32 = alien_sum;
    let pointer2: fn(i32, i32) -> i32 = human_mul;

    println!("Function pointer 1 : {}", pointer1(2, 2));
    println!("Function pointer 2 : {}", pointer2(2, 2));

    // estas dos variables sí son del mismo tipo
    // podríamos hacer:
    let mut pointer1: fn(i32, i32) -> i32 = alien_sum;
    println!("Function pointer (sum) : {}", pointer1(2, 2));

    pointer1 = human_mul;
    println!("Function pointer (mul) : {}", pointer1(2, 2));

    // punteros a funciones
    // parámetros de funciones

    fn f(callback: fn(i32) -> i32, n: i32) {
        let resultado = callback(n);

        println!("Resultado para {}: {}", n, resultado);
    }

    fn duplicar(n: i32) -> i32 {
        n * 2
    }

    fn triplicar(n: i32) -> i32 {
        n * 3
    }

    // duplica es de tipo {duplicar}
    let duplica = duplicar;

    // triplica es de tipo fn(i32)->i32
    let triplica: fn(i32) -> i32 = triplicar;

    // llamada con nombre de función
    f(duplicar, 10);

    // llamada con variable de tipo {duplicar}
    f(duplica, 10);

    // llamada con puntero a función
    f(triplica, 10);

    // closure
    // similar a un bloque de Rust

    let a = 10;

    {
        // el interior del bloque captura
        // (tiene acceso a...)
        // las variables del ámbito superior
        let b = a + 2;
        println!("{} : {}", a, b);
    }

    // closure
    // asignada a un puntero a función
    let f = |x| x + 22;

    // el tipo de dato lo infiere en la primera llamada
    println!("LLamada a closure: {}", f(10));

    // ejemplo con función normal

    // aquí los tipos se tienen que declarar obligatoriamente
    fn plus_22(x: i32) -> i32 {
        x + 22
    }

    // g es un puntero a función
    // (sólo existe en tiempo de compilación)
    let g = plus_22;

    println!("Función normal: {}", g(10));

    // variable en el ámbito actual
    // (fuera del cuerpo de la closure)
    let incremento = 22;

    // puede usar la variable externa sin ningún problema
    let f = |x| x + incremento;

    println!("{}", f(10));
    println!("{}", g(10));

    // función que recibe una closure
    fn funcion_externa(h: impl Fn(i32) -> i32) {
        println!("{}", h(20));
    }

    funcion_externa(f);

    // Por lo tanto, una función que espere un argumento de tipo puntero a función,
    // podrá recibir como argumento una función estándar y una función anónima
    // (*closure* sin captura), pero no podrá recibir una *closure* con captura.

    // esta función recibe un puntero a funcion
    fn h(f: fn()) {
        f();
    }

    // puede recibir perfectamente una
    // closure sin captura
    h(|| {
        println!("Closure sin captura como argumento");
    });

    // let a = 1;
    // h(|| {
    //     println!("Closure con captura {}", a);
    // });
    //
    // ^- ERROR : una closure con captura no puede
    //            ser convertida a puntero a función

    // función genérica que recibe closures
    // fn saluda<F: Fn() -> ()>(f: F) {
    //     f();
    // }

    fn saluda<F>(f: F)
    where
        F: Fn() -> (),
    {
        f();
    }

    let desde_calisto = || {
        println!("Saludos desde Calisto");
    };

    saluda(desde_calisto);

    let planeta = "Júpiter".to_string();
    let desde_calisto_jupiter = || {
        println!("Saludos desde Calisto");
        println!("Satélite de {}", planeta);
    };

    saluda(desde_calisto_jupiter);

    // función normal
    fn saludo_desde_pluton() {
        println!("Saludos desde Plutón");
    }

    saluda(saludo_desde_pluton);

    // syntactic sugar usando impl
    fn saludar(f: impl Fn()) {
        f();
    }

    saludar(desde_calisto);
    saludar(desde_calisto_jupiter);
    saludar(saludo_desde_pluton);
}
