use std::time::Instant;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::time::Duration;

async fn operacion(n: i32) -> i32 {
    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut stdout = io::stdout();
    let msg = format!("(op{}) Tarea 1 terminada\n", n);
    _ = stdout.write_all(msg.as_bytes()).await;

    tokio::time::sleep(Duration::from_millis(100)).await;
    let msg = format!("(op{}) Tarea 2 terminada \n", n);
    _ = stdout.write_all(msg.as_bytes()).await;

    n
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Async/Await - salida estándar println ");
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

        let mut join = tokio::task::JoinSet::new();

        for a in 1..=6 {
            // lanzamos todas las operaciones
            join.spawn(operacion(a));
        }

        while let Some(res) = join.join_next().await {
            match res {
                Ok(resultado) => println!("Resultado: {:?}", resultado),
                Err(e) => println!("Error en operación: {}", e),
            }
        }

        println!("Duración: {}", inicio.elapsed().as_millis());
    });
}
