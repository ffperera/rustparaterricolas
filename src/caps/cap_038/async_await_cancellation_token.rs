use std::time::Instant;
// use tokio::sync::oneshot;
use tokio::time::Duration;
use tokio_util::sync::CancellationToken;

async fn operacion(n: i32, cancel: CancellationToken) -> i32 {
    tokio::select! {
        _ = async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("(op{}) Tarea 1 terminada", n);

            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("(op{}) Tarea 2 terminada", n);

        } => {
            println!("Operación {} terminada", n);
            return n;
        }

        _ = cancel.cancelled() => {
                println!("Operación {} cancelada", n);
                println!("Se procede a cierre controlado");
        }

    }

    0
}

async fn vigilante(t: u64, token: CancellationToken) {
    tokio::time::sleep(Duration::from_millis(t)).await;
    token.cancel();
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Async/Await - CancellationToken");
    println!("--------------------");
    println!();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let inicio = Instant::now();

        let token = CancellationToken::new();

        let mut join = tokio::task::JoinSet::new();

        for a in 1..=3 {
            // lanzamos todas las operaciones
            join.spawn(operacion(a, token.clone()));
        }

        let cancela = tokio::spawn(vigilante(150, token.clone()));

        while let Some(res) = join.join_next().await {
            match res {
                Ok(resultado) => println!("Resultado: {:?}", resultado),
                Err(e) => println!("Error en operación: {}", e),
            }
        }

        let _ = cancela.await;

        println!("Duración: {}", inicio.elapsed().as_millis());
    });
}
