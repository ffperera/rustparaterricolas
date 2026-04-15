use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

pub fn run() {
    println!();
    println!("--------------------");
    println!("Threads");
    println!("--------------------");
    println!();

    println!("Identificador proceso: {:?}", thread::current().id());

    fn bucle() {
        for i in 0..50 {
            print!("{i}.");
        }
    }

    // lanza hilo ejecutando la función bucle()
    let tarea1 = thread::spawn(bucle);

    // lanza segundo hilo
    let tarea2 = thread::spawn(bucle);

    // join() le dice al hilo principal que espere
    // hasta que termine la ejecución del hilo
    tarea1.join().unwrap();
    tarea2.join().unwrap();

    // cuando termina la ejecución de los dos hilos
    // termina el hilo principal y el programa

    println!();

    let builder = thread::Builder::new()
        .name(String::from("Soy un hilo"))
        .stack_size(16 * 1024);

    let tarea3 = builder.spawn(bucle);

    tarea3.unwrap().join().unwrap();

    println!();

    // ejemplo devolver valores

    fn contador() -> i32 {
        let mut contador = 0;
        for _i in 0..50 {
            contador += 1;

            // detener la ejecución durante 1ms
            thread::sleep(Duration::from_millis(1));
        }

        // devuelve contador
        contador
    }

    let tarea1 = thread::spawn(contador);
    let tarea2 = thread::spawn(contador);

    let mut total = 0;

    // si todo ha ido bien, el resultado vendrá en Ok()
    if let Ok(contador) = tarea1.join() {
        total = total + contador;
    }

    if let Ok(contador) = tarea2.join() {
        total = total + contador;
    }

    println!("Contador total: {}", total);

    let mensaje = "Para los humanos... \
    Rust es indistinguible de la magia"
        .to_string();

    // creamos un scope común a todos los hilos
    // no hace falta hacer join() explícito,
    // el hilo principal esperará hasta la finalización
    // de los hilos internos
    thread::scope(|actual| {
        // 'mensaje' es capturado mediante
        // una referencia inmutable

        actual.spawn(|| {
            println!("{}", mensaje);
            println!("{:?}", thread::current().id());
        });

        // también podemos utilizar join()
        // para capturar posibles errores
        // o valores devueltos por el hilo
        let hilo2 = actual.spawn(|| {
            println!("{}", mensaje);
            println!("{:?}", thread::current().id());
        });

        if let Err(error) = hilo2.join() {
            println!("Error {:?}", error);
        }
    });

    // la variable mensaje no se ha consumido,
    // sigue estando activa en este scope
    println!("{}", mensaje);

    // ejemplo con atomics

    // contador para la cuenta atrás
    static CUENTA_ATRAS: AtomicUsize = AtomicUsize::new(10);

    println!("Secuencia de lanzamiento...");

    // creamos 3 hilos
    // que actúan como sistemas de la nave
    let mut sistemas = vec![];

    for id in 1..=3 {
        // referencia a la variable atómica
        // para pasarla al hilo correspondiente
        let contador = &CUENTA_ATRAS;

        sistemas.push(thread::spawn(move || {
            // cada sistema disminuye el contador 3 veces
            for _ in 0..3 {
                // decrementar el contador
                let valor_actual = contador.fetch_sub(1, Ordering::Relaxed);

                println!("Sistema {}: T-{}", id, valor_actual);

                // pausa para simular
                // la ejecución de código adicional
                thread::sleep(Duration::from_millis(100));
            }
        }));
    }

    //  esperar para que todos los hilos terminen
    for hilo in sistemas {
        hilo.join().unwrap();
    }

    // Leemos el valor final
    let valor_final = &CUENTA_ATRAS.load(Ordering::Relaxed);
    println!("Despegue en T-{}", valor_final);

    // version con scope threads

    // contador para la cuenta atrás
    let cuenta_atras_b: AtomicUsize = AtomicUsize::new(10);

    println!("Secuencia de lanzamiento...");

    // con scoped garantizamos que las referencias
    // a cuenta_atras_b sean válidas durante
    // la ejecución de todos los hilos
    thread::scope(|actual| {
        // que actúan como sistemas de la nave
        let mut sistemas = vec![];

        for id in 1..=3 {
            // referencia a la variable atómica
            // para pasarla al hilo correspondiente
            let contador = &cuenta_atras_b;

            sistemas.push(actual.spawn(move || {
                // cada sistema disminuye el contador 3 veces
                for _ in 0..3 {
                    // decrementar el contador
                    let valor_actual = contador.fetch_sub(1, Ordering::Relaxed);

                    println!("Sistema {}: T-{}", id, valor_actual);

                    // pausa para simular
                    // la ejecución de código adicional
                    thread::sleep(Duration::from_millis(100));
                }
            }));
        }

        //  esperar para que todos los hilos terminen
        for hilo in sistemas {
            hilo.join().unwrap();
        }
    });

    // Leemos el valor final
    let valor_final = &cuenta_atras_b.load(Ordering::Relaxed);
    println!("Despegue en T-{}", valor_final);

    // Arc
    println!();

    // envolvemos el contenido en Arc
    let mensaje = Arc::new(
        "Para los humanos... \
        Rust es indistinguible de la magia"
            .to_string(),
    );

    let mut tareas = Vec::new();

    // tareas es un vector de handles
    // que gestionan threads
    for _i in 0..5 {
        // obtenemos una nueva referencia
        // (Arc incrementa su contador)
        let msg = mensaje.clone();

        // lanzamos el hilo y guardamos su handler
        // en el vector de tareas
        tareas.push(thread::spawn(move || {
            // la closure pasa a ser propietaria
            // de la referencia Arc
            println!("{}", msg);
            println!("{:?}", thread::current().id());
        }));
    }

    for tarea in tareas {
        tarea.join().unwrap();
    }

    // la variable mensaje no se ha consumido,
    // sigue estando activa, encapsulada en Arc
    println!("{}", mensaje);

    // -------------------
    // Mutex
    println!();

    let mensaje = "Terrícolas".to_string();

    // cuando envolvemos 'mensaje' en un Mutex,
    // mensaje se consume
    // ahora su contenido puede ser modificado a
    // través del Mutex a pesar de que 'modificable'
    // está declarada como inmutable
    // (Mutex gestiona el acceso exclusivo al contenido)
    let modificable = Mutex::new(mensaje);

    thread::scope(|actual| {
        actual.spawn(|| {
            // con lock() conseguimos acceso exclusivo
            // al contenido
            // ningún otro hilo puede acceder ahora
            let mut lock = modificable.lock().unwrap();
            lock.push_str(" vs Aliens");

            // no es necesario desbloquear, la exclusividad
            // termina cuando la referencia queda fuera de scope

            // pero también podemos desbloquear de forma explícita:
            drop(lock);
        });

        actual.spawn(|| {
            let mut lock = modificable.lock().unwrap();
            lock.push_str("!");
        });
    });

    // para acceder al contenido tenemos que usar lock()
    // en este caso se genera una referencia temporal y
    // vuelve a desbloquearse tras la llamada a println!()
    println!("{:?}", modificable.lock().unwrap());

    // -------------------
    // RwLock
    println!();

    // planetas con vida inteligente
    struct Planeta {
        nombre: String,
        inteligencia: RwLock<bool>,
    }

    let tierra = Planeta {
        nombre: "Tierra".to_string(),
        inteligencia: RwLock::new(false),
    };

    thread::scope(|actual| {
        // sondas para detectar inteligencia
        let mut sondas = vec![];
        for id in 1..=3 {
            let planeta_ref = &tierra;
            sondas.push(actual.spawn(move || {
                // detectar inteligencia
                for _ in 0..10 {
                    let estado = planeta_ref.inteligencia.read().unwrap();
                    if *estado {
                        println!("Sonda {}", id);
                        println!("Detectó indicios de inteligencia");
                        println!("Planeta {}", planeta_ref.nombre);
                    };

                    drop(estado);
                    thread::sleep(Duration::from_millis(10));
                }
            }));
        }

        //
        let senales = actual.spawn(|| {
            for i in 0..30 {
                if i == 17 {
                    let mut estado = tierra.inteligencia.write().unwrap();
                    *estado = true;

                    drop(estado);
                }

                thread::sleep(Duration::from_millis(5));
            }
        });

        for sonda in sondas {
            sonda.join().unwrap();
        }
        senales.join().unwrap();
    });

    // Arc + RwLock
    println!("");

    let tierra = Planeta {
        nombre: "Tierra".to_string(),
        inteligencia: RwLock::new(false),
    };

    // envolver con Arc
    let arc = Arc::new(tierra);

    // sondas para detectar inteligencia
    let mut sondas = vec![];
    for id in 1..=3 {
        let planeta_ref = arc.clone();
        sondas.push(thread::spawn(move || {
            // detectar inteligencia
            for _ in 0..10 {
                let estado = planeta_ref.inteligencia.read().unwrap();

                if *estado {
                    println!("Sonda(ARC) {}", id);
                    println!("Detectó indicios de inteligencia");
                    println!("Planeta {}", planeta_ref.nombre);
                };

                drop(estado);
                thread::sleep(Duration::from_millis(10));
            }
        }));
    }

    let planeta_ref = arc.clone();

    let senales = thread::spawn(move || {
        for i in 0..30 {
            if i == 17 {
                let mut estado = planeta_ref.inteligencia.write().unwrap();

                *estado = true;

                // no sería necesario el drop explícito
                // porque la variable local termina igualmente
                // al finalizar el bloque
                drop(estado);
            }

            thread::sleep(Duration::from_millis(5));
        }
    });

    for sonda in sondas {
        sonda.join().unwrap();
    }
    senales.join().unwrap();

    // Arc + Mutex
    println!();

    // base estelar es un vector
    // encapsulado en Mutex
    // encapsulado en Arc

    let base_estelar = Arc::new(Mutex::new(Vec::<String>::new()));

    // naves que intenan acceder a la base
    let nombres_naves = vec![
        "Enterprise".to_string(),
        "Millennium Falcon".to_string(),
        "Nostromo".to_string(),
        "Serenity".to_string(),
        "Galactica".to_string(),
    ];

    let mut handles = vec![];

    for nombre in nombres_naves {
        // clonamos referencia Arc
        let base_ref = base_estelar.clone();

        let handle = thread::spawn(move || {
            // simular tiempo de viaje
            let tiempo_viaje = Duration::from_millis(nombre.len() as u64);

            thread::sleep(tiempo_viaje);

            // bloquear el mutex para acceder al vector
            let mut registro = base_ref.lock().unwrap();

            // entrada de la nave
            registro.push(nombre.clone());
            println!("{} ha llegado a base", nombre);

            // liberar bloqueo lo antes posible
            drop(registro);

            // operaciones después de desbloquear
            println!("   >>> {} descargando...", nombre);
        });

        handles.push(handle);
    }

    // esperar a todos los hilos
    for handle in handles {
        handle.join().unwrap();
    }

    // revisar llegadas a la base
    println!();
    println!("Naves que han llegado a base:");
    println!();

    let registro = base_estelar.lock().unwrap();
    for (i, nave) in registro.iter().enumerate() {
        println!("{}. {}", i + 1, nave);
    }
}
