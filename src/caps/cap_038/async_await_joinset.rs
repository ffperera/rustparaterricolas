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
    println!("Async/Await - JoinSet");
    println!("--------------------");
    println!();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let inicio = Instant::now();

        let mut join = tokio::task::JoinSet::new();

        for a in 0..3 {
            join.spawn(operacion(a));
        }

        // let resultados = join.join_all().await;

        while let Some(tarea) = join.join_next().await {
            println!("Resultado tarea: {}", tarea.unwrap());
        }

        println!("Duración: {}", inicio.elapsed().as_millis());
    });
}
