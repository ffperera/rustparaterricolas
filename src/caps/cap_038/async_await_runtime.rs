// use tokio::select;
// use tokio::sync::oneshot;
use tokio::time::Duration;

pub fn run() {
    println!();
    println!("--------------------");
    println!("Async/Await - runtime básico Tokio");
    println!("--------------------");
    println!();

    async fn operacion(n: i32) {
        // tarea 1
        tokio::time::sleep(Duration::from_millis(100)).await;
        println!("(op{}) Tarea 1 terminada", n);

        // tarea 2
        tokio::time::sleep(Duration::from_millis(100)).await;
        println!("(op{}) Tarea 2 terminada", n);
    }

    // bloque asíncrono
    let b = async {
        let op = operacion(1);
        op.await;

        println!("Termina la operación asíncrona");
    };

    // creación del contexto asíncrono
    let rt = tokio::runtime::Runtime::new().unwrap();

    // rt.block_on(async {
    //     b.await;
    //     println!("Fin");
    // });
    //

    rt.block_on(b);
}
