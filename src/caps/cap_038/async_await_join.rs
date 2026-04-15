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
    println!("Async/Await - macro join!");
    println!("--------------------");
    println!();

    // con join!()
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let inicio = Instant::now();

        let (res1, res2, res3) = tokio::join!(operacion(1), operacion(2), operacion(3));

        println!("Resultado: {} | {} | {} ", res1, res2, res3);

        println!("Duración: {}", inicio.elapsed().as_millis());
    });

    // con spawn()
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let inicio = Instant::now();

        let op_1 = tokio::spawn(operacion(1));
        let op_2 = tokio::spawn(operacion(2));
        let op_3 = tokio::spawn(operacion(3));

        // tokio::time::sleep(Duration::from_millis(150)).await;

        println!("Resultado: ");

        print!("{}", op_1.await.unwrap());
        print!("{}", op_2.await.unwrap());
        print!("{}", op_3.await.unwrap());

        println!("Duración: {}", inicio.elapsed().as_millis());
    });
}
