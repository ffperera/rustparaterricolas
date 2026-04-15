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
    println!("Async/Await - macro select!");
    println!("--------------------");
    println!();

    // con join!()
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let inicio = Instant::now();

        tokio::select! {

            resultado = operacion(1) => {
                println!("Termina op 1: {} ", resultado);
            }

            resultado = operacion(2) => {
                println!("Termina op 2: {} ", resultado);
            }

        }

        println!("Duración: {}", inicio.elapsed().as_millis());
    });
}
