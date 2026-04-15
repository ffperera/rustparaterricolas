use std::time::Instant;
use tokio::sync::oneshot;
use tokio::time::Duration;

async fn operacion(n: i32, cancel_rx: oneshot::Receiver<()>) -> i32 {
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

        _ = cancel_rx => {
                println!("Operación {} cancelada", n);
                println!("Se procede a cierre controlado");
        }


    }

    0
}

async fn vigilante(t: u64, cancel_tx: oneshot::Sender<()>) {
    tokio::time::sleep(Duration::from_millis(t)).await;
    cancel_tx.send(()).unwrap();
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Async/Await - cancel mediante canal oneshot ");
    println!("--------------------");
    println!();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let inicio = Instant::now();

        let (tx, rx) = oneshot::channel();

        let op_1 = tokio::spawn(operacion(1, rx));
        let cancela = tokio::spawn(vigilante(150, tx));

        let resultado = op_1.await.unwrap();
        cancela.await.unwrap();

        println!("Resultado: {:?}", resultado);

        println!("Duración: {}", inicio.elapsed().as_millis());
    });
}
