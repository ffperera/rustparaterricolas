// mpsc = Multiple Producer, Single Consumer
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Tipos de mensajes interestelares
#[derive(Debug)]
enum Mensaje {
    Aviso(String),
    Alerta(String),
    Desconexion,
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Threads channels");
    println!("--------------------");
    println!();

    // channel
    let (emisor, receptor) = mpsc::channel();

    // naves exploradoras

    let naves = vec!["Tar", "Andrómeda", "Perseo", "Orión"];
    let num_naves = naves.len();
    let mut handles = vec![];

    for nave in naves {
        // clonar emisor para cada nave
        let emisor_nave = emisor.clone();

        let handle = thread::spawn(move || {
            // simular exploración espacial
            for i in 1..=3 {
                // enviar mensaje
                let mensaje = match i {
                    3 => Mensaje::Alerta(format!("{}: ¡Energía crítica!", nave)),
                    _ => Mensaje::Aviso(format!("{}: Explorando sector {}", nave, i)),
                };

                // mensaje a la estación central
                emisor_nave.send(mensaje).unwrap();

                // simular tiempo entre transmisiones
                thread::sleep(Duration::from_millis(nave.len() as u64 * 3));
            }

            // desconexión
            emisor_nave.send(Mensaje::Desconexion).unwrap();
            println!("{}: Transmisiones finalizadas", nave);
        });

        handles.push(handle);
    }

    // liberar emisor original (no usado)
    drop(emisor);

    // Estación central procesando mensajes
    println!("Estación Central");
    let mut naves_desconectadas = 0;

    for mensaje in receptor {
        match mensaje {
            Mensaje::Aviso(texto) => {
                println!("[RECIBIDO] {}", texto)
            }
            Mensaje::Alerta(texto) => {
                println!("[ALERTA!] {}", texto)
            }
            Mensaje::Desconexion => {
                naves_desconectadas += 1;
                println!(
                    "Nave desconectada (total: {}/{})",
                    naves_desconectadas, num_naves
                );

                // todas las naves han desconectado
                if naves_desconectadas == num_naves {
                    println!("Transmisiones finalizadas");
                    break;
                }
            }
        }
    }

    // Esperar a que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }

    // ----------------------
    // sync_channel
    println!();
    println!();

    // crear canal síncrono
    // con tamaño de buffer para 3 mensajes
    let (emisor, receptor) = mpsc::sync_channel(3);

    // hilo 1 - envía órdenes
    let tx = emisor.clone();
    thread::spawn(move || {
        let ordenes = vec![
            "Activar escudos".to_string(),
            "Cargar propulsores".to_string(),
            "Iniciar secuencia de despegue".to_string(),
            "Desplegar satélites".to_string(),
            "Establecer órbita".to_string(),
        ];

        for (i, orden) in ordenes.into_iter().enumerate() {
            println!("Comandante {}: '{}'", i + 1, orden);
            tx.send(orden).unwrap();

            //  pausa entre órdenes
            thread::sleep(Duration::from_millis(100));
        }
    });

    // hilo 2 - recibe órdenes
    thread::spawn(move || {
        for msg in receptor {
            println!("Nave recibió: '{}'", msg);

            // simular procesamiento
            thread::sleep(Duration::from_millis(300));
        }
    });

    // Esperar para que los hilos terminen su trabajo
    thread::sleep(Duration::from_secs(2));
    println!("Canal cerrado");
}
