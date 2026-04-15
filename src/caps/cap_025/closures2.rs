pub fn run() {
    println!();
    println!("--------------------");
    println!("Fn, FnMut, FnOnce");
    println!("--------------------");
    println!();

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

    let planeta = "Júpiter".to_string();

    let closure = || {
        println!("Saludos desde Calisto");
        println!("Satélite de {}", planeta);
    };
    /*
        // esto es pseudocódigo para visualizar
        // qué ocurre internamente

        struct Closure {
            r1: &String,
        }

        impl FnOnce for Closure {
            type Output = ();
            fn call_once(self) {
                println!("Saludos desde Calisto");
                println!("Satélite de {}", self.r1);
            }
        }

        impl Fn for Closure {
            type Output = ();

            fn call(&self) {
                println!("Saludos desde Calisto");
                println!("Satélite de {}", self.r1);
            }
        }
    */
    saluda(closure);

    let mut a = 1;
    let mut b = 1;

    let mut fibonacci = || {
        let sum = a + b;
        a = b;
        b = sum;
        println!("{} : {}", a, b);
    };

    fibonacci();
    fibonacci();
    fibonacci();
    fibonacci();
    fibonacci();

    // Fn
    let planeta = "Júpiter".to_string();

    let closure = || {
        println!("Saludos desde Calisto");
        println!("Satélite de {}", planeta);
    };

    closure();

    // FnMut
    let mut planeta = "Júpiter".to_string();

    let mut closure = || {
        planeta.push_str(" es un planeta grandote.");
        println!("{}", planeta);
    };

    closure();

    // FnOnce
    let planeta = "Júpiter".to_string();

    let closure = move || {
        println!("{}", planeta);
        planeta
    };

    let destino = closure();
    println!("{}", destino);

    // error - closure no se puede utilizar
    // sólo se puede llamar una vez
    // let destino = closure();

    // FnOnce
    // transferir ownership de una única variable
    let planeta = "Júpiter".to_string();
    let luna = "Europa".to_string();
    let luna_ref = luna.as_str();

    let closure = move || {
        println!("{} es una luna de {}", luna_ref, planeta);
        planeta
    };

    // 'planeta' se consumió
    // su contenido está ahora en 'destino'
    // 'luna' no se consumió porque la closure
    // utiliza una referencia a su contenido
    let destino = closure();
    println!("{}", destino);
    println!("{}", luna);
}
