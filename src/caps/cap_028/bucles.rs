pub fn run() {
    println!();
    println!("--------------------");
    println!("Bucles");
    println!("--------------------");
    println!();

    // loop simple

    let mut contador = 0;
    loop {
        print!("{contador}.");
        // código
        if contador > 10 {
            break;
        }
        contador += 1;
    }

    println!();

    // loop con etiquetas

    let mut contador = 0;

    'exterior: loop {
        let mut contador_interior = 0;
        'interior: loop {
            contador_interior += 1;
            if contador_interior > 10 {
                // se detiene el bucle interior
                break 'interior;
            }

            // si se cumple la condición, romper el bucle exterior
            if contador_interior * contador > 90 {
                break 'exterior;
            }
        }

        print!("{} .", contador * contador_interior);

        // si se cumple la condición, romper bucle exterior
        if contador > 10 {
            // break sin etiqueta hace referencia al bucle de su nivel
            break;
        }

        contador += 1;
    }

    println!();

    // devolver valores desde loop

    let mut contador = 0;

    // el bucle exterior devuelve i32
    let total = 'exterior: loop {
        let mut contador_interior = 0;

        // el bucle interior devuelve String
        let _interior_aviso = 'interior: loop {
            contador_interior += 1;
            if contador_interior > 10 {
                break 'interior "Fuera del bucle interior".to_string();
            }

            if contador_interior * contador > 90 {
                break 'exterior contador_interior * contador;
            }
        };

        print!("{} .", contador * contador_interior);

        if contador > 10 {
            // por defecto hace referencia al bucle de su nivel
            break contador * contador_interior;
        }

        contador += 1;
    };

    println!();
    println!("Total: {:?}", total);

    // bucles while
    let mut contador = 0;

    'exterior: while contador < 10 {
        let mut contador_interior = 0;
        'interior: while contador_interior < 10 {
            contador_interior += 1;
            if contador_interior == 7 {
                // se detiene el bucle interior
                break 'interior;
            }

            // si se cumple la condición, romper el bucle exterior
            if contador_interior * contador > 90 {
                break 'exterior;
            }
        }

        print!("{} .", contador * contador_interior);

        contador += 1;
    }

    println!();

    // bucles for

    let vector = &vec![1, 2, 3, 4, 5, 6, 7, 8];

    'exterior: for a in 4..10 {
        for b in vector {
            if a < 5 || a % 2 == 1 {
                continue;
            }

            if a * b > 60 {
                break 'exterior;
            }

            print!("{a} * {b}={} ", a * b);
        }
    }

    println!();

    // while let
    let mut telemetria = vec![
        Some(85),
        Some(92),
        Some(78),
        None, // pérdida de señal
        Some(95),
        Some(130), // valor crítico
        None,
    ]
    .into_iter();

    let mut contador = 1;

    // procesar iterador
    while let Some(dato) = telemetria.next() {
        match dato {
            Some(valor) => {
                println!("Lectura {}: {}%", contador, valor);

                if valor > 100 {
                    println!("¡ALERTA!");
                    break;
                }
            }
            None => {
                println!("Lectura {}: sin señal", contador)
            }
        }

        contador += 1;
    }

    println!();
}
