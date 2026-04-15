use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    println!();
    println!("--------------------");
    println!("Smart pointers");
    println!("--------------------");
    println!();

    println!();
    println!("Box");
    println!();

    struct Planeta {
        nombre: String,
        // datos grandes: mapa topográfico
        // 1000 x 1000
        // (evitar sobrecarga en stack)
        topografia: Box<[[f64; 1000]; 1000]>,
    }

    let tierra = Planeta {
        nombre: "Tierra".to_string(),
        topografia: Box::new([[0.0; 1000]; 1000]),
    };

    println!("Planeta: {}", tierra.nombre);

    println!(
        "Datos: {}",
        tierra.topografia.len() * tierra.topografia[0].len()
    );

    println!("Datos: {}", tierra.topografia[1][1]);

    trait SistemaNave {
        fn activar(&self);
    }

    // estructura unidad para usar como marcador
    struct MotorIonico;
    impl SistemaNave for MotorIonico {
        fn activar(&self) {
            println!("Propulsión iónica al 100%!");
        }
    }

    // estructura unidad para usar como marcador
    struct EscudoDeflector;
    impl SistemaNave for EscudoDeflector {
        fn activar(&self) {
            println!("Escudos activados!");
        }
    }

    // los sistemas de la nave
    // están almacenados en un vector
    // Box almacena cualquier estructura
    // que implemente el trait SistemaNave
    struct Nave {
        sistemas: Vec<Box<dyn SistemaNave>>,
    }

    // dos motores y un escudo
    let caza_espacial = Nave {
        sistemas: vec![
            Box::new(MotorIonico),
            Box::new(MotorIonico),
            Box::new(EscudoDeflector),
        ],
    };

    for sistema in caza_espacial.sistemas {
        // dynamic dispatch
        sistema.activar();
    }

    // Box<[T]>  vs Vec<T>

    // Box<[f64]>
    let topografia = Box::new([1.0; 1000]);
    println!("Mem var: {}", std::mem::size_of_val(&topografia));
    println!("Items: {}", topografia.len());
    println!("Mem cont: {}", std::mem::size_of::<[f64; 1000]>());

    // Mem var: 8
    // Items: 1000
    // Mem cont: 8000

    // Vec<f64>
    let mut topografia = vec![1.0; 999];
    topografia.push(2.0);

    println!("Mem var: {}", std::mem::size_of_val(&topografia));
    println!("Items: {}", topografia.len());
    println!(
        "Mem cont: {}",
        topografia.capacity() * std::mem::size_of::<f64>()
    );

    // Mem var: 24
    // Items: 1000
    // Mem cont: 15984

    // Rc

    let s = Rc::new(String::from("Hola"));

    {
        // ahora s2 es propietaria de "Hola"
        let s2 = s.clone();
        println!("s2: {}", s2);

        // s2 se destruye aquí
        // pero su contenido sigue existiendo (contador > 0)
    }

    // sintaxis preferida en muchos casos
    // porque muestra explícitamente la idea de clonar un Rc
    let s3 = Rc::clone(&s);

    // s3 se destruye, pero la estructura Rc sigue existiendo
    // por lo tanto el contenido sigue teniendo propietario
    drop(s3);

    println!("s: {}", s);

    // RefCell

    // x : es propietaria del valor
    // y es inmutable
    let x = RefCell::new(5);

    // pero nos permite recuperar una
    // referencia mutable mediante borrow_mut()
    // esta línea compila y se ejecuta
    *(x.borrow_mut()) = 12;

    println!("{}", x.borrow());

    // esta referencia implícita se destruye
    // en el momento de usarla
    // (ya no queda ningún préstamo mutable activo)

    // podríamos intentar hacer más
    // de un préstamo mutable:
    // estas líneas compilan, pero
    // generarían panic en ejecución
    let borrow1 = x.borrow_mut();

    // let borrow2 = x.borrow_mut();
    // already borrowed: BorrowMutError

    println!("{}", borrow1);

    struct NaveAtaque {
        nombre: String,
        // escudo
        // mutabilidad interior
        escudo: RefCell<u32>,
    }

    impl NaveAtaque {
        fn new(nombre: &str, escudo_inicial: u32) -> Self {
            NaveAtaque {
                nombre: nombre.to_string(),
                escudo: RefCell::new(escudo_inicial),
            }
        }

        fn recibir_ataque(&self, danio: u32) {
            // acceso mutable
            // a través de referencia inmutable
            let mut escudo = self.escudo.borrow_mut();

            // *escudo = *escudo - danio;
            // para evitar desbordamiento, mejor:
            *escudo = escudo.saturating_sub(danio);

            println!("{} recibe {} de daño!", self.nombre, danio);
            println!("Escudo restante: {}", *escudo);
        }

        fn estado(&self) {
            // Acceso inmutable
            let escudo = self.escudo.borrow();
            println!("{} - Escudo: {}/100", self.nombre, *escudo);
        }
    }

    // enterprise es inmutable
    let enterprise = NaveAtaque::new("Enterprise", 100);

    enterprise.estado();

    // modifica el estado interno
    enterprise.recibir_ataque(30);
    enterprise.recibir_ataque(25);

    enterprise.estado();

    // intento de modificación simultánea
    // (provocará panic)
    // let escudo1 = enterprise.escudo.borrow_mut();

    // Rc<RefCell<T>>

    // s es inmutable
    let s = String::from("Saludos");

    // encapsulamos el contenido dentro de RefCell y Rc
    // s se consume, ya no está disponible
    // x el la nueva propietaria, a través de Rc
    let x = Rc::new(RefCell::new(s));

    // creamos otra propietaria a través de Rc
    let y = Rc::clone(&x);

    // vamos a intentar modificar el contenido
    // mediante borrow_mut()
    let mut borrow_mut = x.borrow_mut();
    borrow_mut.push_str(" terrícolas");

    // el contenido de RefCell está 'secuestrado'
    // por borrow_mut
    println!("Soy borrow_mut {:?}", borrow_mut);

    // pero tanto x como y mostrarán algo como
    // RefCell { value: <borrowed> }
    println!("Soy y: {:?}", y);

    // ese contenido estará disponible cuando
    // desaparezca el préstamo mutable
    // (p.e. si sale fuera de scope o se hace drop explícito)
    drop(borrow_mut);

    // Ahora tanto x como y permiten el acceso a su contenido
    println!("Soy x: {}", x.borrow());
    println!("Soy y: {}", y.borrow());

    // Cow
    //
    fn comprobar_planeta<'a>(datos: &'a str) -> Cow<'a, str> {
        if datos.contains("[x]") {
            Cow::Borrowed(datos)
        } else {
            Cow::Owned(format!("{} [x]", datos))
        }
    }

    // listado de planetas
    // [x] <- verificado
    let planetas = [
        "Mercurio [x]",
        "Venus",
        "Tierra [x]",
        "Marte [x]",
        "Júpiter",
        "Saturno [x]",
    ];

    for planeta in &planetas {
        match comprobar_planeta(planeta) {
            Cow::Borrowed(p) => {
                // &str
                println!("{:?}", p);
            }
            Cow::Owned(p) => {
                // String
                let mut s = p;
                s.push_str(" - Verificado");
                println!("{:?}", s);
            }
        }
    }

    // "Mercurio [x]"
    // "Venus [x] - Verificado"
    // "Tierra [x]"
    // "Marte [x]"
    // "Júpiter [x] - Verificado"
    // "Saturno [x]"
}
