use std::time::Instant;
use tokio::time::Duration;

async fn operacion(n: i32) -> i32 {
    tokio::time::sleep(Duration::from_millis(100)).await;
    println!("(op{}) Tarea 1 terminada", n);

    tokio::time::sleep(Duration::from_millis(100)).await;
    println!("(op{}) Tarea 2 terminada", n);

    n
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Async/Await - abort() / timeout ");
    println!("--------------------");
    println!();

    // timeout usando un temporizador fijo
    // el temporizador se completa siempre
    //
    println!();
    println!("Timeout usando abort() ");
    println!();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let inicio = Instant::now();

        let op_1 = tokio::spawn(operacion(1));

        // este temporizador se completa siempre
        tokio::time::sleep(Duration::from_millis(150)).await;

        // si la operación no ha terminado, la abortamos.
        if !op_1.is_finished() {
            println!("No se ha podido completar la operacion.");
            op_1.abort();
        }

        // intentamos recuperar el resultado
        // por si la operación se ha completado
        let resultado = op_1.await;

        println!("Duración: {}", inicio.elapsed().as_millis());
        println!("Resultado: {:?}", resultado);
    });

    // timeout usando select!
    //
    println!();
    println!("Timeout usando select!");
    println!();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let inicio = Instant::now();

        tokio::select! {

            resultado = tokio::spawn(operacion(1)) => {
                println!("Resultado: {:?}", resultado);
            }

            _ = tokio::time::sleep(Duration::from_millis(250)) => {
                println!("No se ha podido completar la operacion.");
            }
        }

        println!("Duración: {}", inicio.elapsed().as_millis());
    });

    // timeout usando timeout()
    println!();
    println!("Timeout usando timeout()");
    println!();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let inicio = Instant::now();

        let resultado = tokio::time::timeout(Duration::from_millis(250), operacion(1)).await;

        match resultado {
            Ok(n) => println!("Operación completada: {}", n),
            Err(_) => println!("No se pudo completar"),
        }

        println!("Duración: {}", inicio.elapsed().as_millis());
    });
}
