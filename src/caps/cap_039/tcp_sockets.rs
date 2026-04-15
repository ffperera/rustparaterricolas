use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::str;
use std::thread;
use std::time::Duration;
// use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::time::{sleep};
use std::sync::Arc;

#[allow(unused_variables)]
pub fn run() {
    println!();
    println!("--------------------");
    println!("Redes: TCP UDP sockets");
    println!("--------------------");
    println!();

    /*
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        let addrs = [
            SocketAddr::from(([127, 0, 0, 1], 9090)),
            SocketAddr::from(([127, 0, 0, 1], 4444)),
        ];

        let listener = TcpListener::bind(&addrs[..]).unwrap();

        let mut addrs_iter = "localhost:443".to_socket_addrs().unwrap();

        let addr1 = SocketAddr::from(([0, 0, 0, 0], 9191));
        let addr2 = SocketAddr::from(([127, 0, 0, 1], 4545));
        let addrs = vec![addr1, addr2];

        // let mut addrs_iter = (&addrs[..]).to_socket_addrs().unwrap();
        let listener = TcpListener::bind(&addrs[..]).unwrap();

    */

    // bloquea a la espera de una conexión externa
    // por ejemplo
    // nc 127.0.0.1 8080
    //
    // accept_test();

    // bloquea a la espera de conexiones externas
    // por ejemplo:
    // nc 127.0.0.1 8080
    //
    // incoming_test();

    number(72);

    // quote of the day
    qotd();

    let rt = tokio::runtime::Runtime::new().unwrap();

    // concurrencia
    let concurrencia = false;

    if concurrencia {
        // server utiliza native threads
        // para gestionar conexiones entrantes
        let _server_thread = thread::spawn(|| {
            // server_start();
        });
    } else {
        // server utiliza tokio threads
        // para gestionar conexiones entrantes
        rt.block_on(async {
            let _ = tokio::spawn(async { async_server_start().await });
        });
    }

    // esperar para que el servidor inicie
    thread::sleep(Duration::from_millis(100));

    let num_clientes = 5u8;
    let mut hilos_clientes = vec![];

    for client_id in 1..=num_clientes {
        hilos_clientes.push(thread::spawn(move || {
            let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

            for num in 1..=5 {
                thread::sleep(Duration::from_millis((client_id * num).into()));

                stream
                    .write_all(format!("Hola desde cliente #{} ({})\n", client_id, num).as_bytes())
                    .unwrap();

                // respuesta del servidor
                let mut buffer = [0; 1024];
                let num_bytes = stream.read(&mut buffer).unwrap();

                println!(
                    "Cliente #{} | respuesta: {}",
                    client_id,
                    String::from_utf8_lossy(&buffer[..num_bytes])
                );
            }
        }));
    }

    for handle in hilos_clientes {
        handle.join().unwrap();
    }

    // esperar en este hilo para
    // que el servidor siga activo durante
    // todo el proceso
    // thread::sleep(Duration::from_secs(5));
}

#[allow(dead_code)]
fn accept_test() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Esperando conexión en puerto 8080...");

    // aceptar una sola conexión
    let (mut stream, addr) = listener.accept().unwrap();
    println!("Cliente conectado desde IP: {}", addr);

    // preparamos un buffer para recoger la información
    let mut buffer = [0; 1024];
    let num_bytes = stream.read(&mut buffer).unwrap();
    println!(
        "Mensaje recibido: {}, bytes: {}",
        str::from_utf8(&buffer).unwrap(),
        num_bytes
    );

    // Responder
    stream
        .write_all("Saludos, terrícolas\n".as_bytes())
        .unwrap();
    println!("Respuesta enviada");
}

#[allow(dead_code)]
fn incoming_test() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Esperando conexiones en puerto 8080...");

    let mut buffer = [0; 1024];
    let mut contador = 1;

    for conexion in listener.incoming() {
        match conexion {
            Ok(mut stream) => {
                let _num_bytes = stream.read(&mut buffer).unwrap();
                println!(
                    "Mensaje recibido de conexión {}: {}",
                    contador,
                    str::from_utf8(&buffer).unwrap()
                );

                // Responder
                stream
                    .write_all("Saludos, terrícolas\n".as_bytes())
                    .unwrap();

                println!("Respuesta enviada\n");
            }
            Err(e) => eprintln!("Error aceptando conexión: {}", e),
        }

        contador += 1;
    }
}

fn number(number: i32) {
    // conectar al servidor web
    // (puerto 80 para HTTP)
    let mut stream = TcpStream::connect("numbersapi.com:80").unwrap();

    // construir solicitud HTTP GET
    let request = format!(
        "GET /{} HTTP/1.1\r\n\
                   Host: www.example.com\r\n\
                   Connection: close\r\n\
                   \r\n",
        number
    );

    // enviar solicitud
    // equivale a escribir sobre el stream
    stream.write_all(request.as_bytes()).unwrap();

    // leer respuesta del servidor
    // preparamos un buffer intermedio (4KB)
    let mut buffer = [0; 4096];

    // buffer final para almacenar la respuesta completa
    // utilizamos un vector de bytes (u8)
    let mut respuesta_bytes = Vec::new();

    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break; // Fin de la transmisión
        }

        // acumulamos los bytes leídos
        respuesta_bytes.extend_from_slice(&buffer[..bytes_read]);
    }

    // convertimos el vector de bytes a una cadena
    let respuesta = String::from_utf8_lossy(&respuesta_bytes);
    println!(
        "Respuesta recibida ({} bytes):\n{}",
        respuesta_bytes.len(),
        respuesta
    );
}

fn qotd() {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();

    socket.send_to(&[], "djxmmx.net:17").unwrap();
    let mut buf = [0; 256];

    println!();

    match socket.recv_from(&mut buf) {
        Ok(len) => println!("{}", String::from_utf8_lossy(&buf[..len.0])),
        Err(e) => println!("No se recibió información de QOTD, {}", e),
    }

    println!();
}

#[allow(dead_code)]
fn server_start() {
    let direccion = "127.0.0.1:8080";

    let listener = TcpListener::bind(&direccion).unwrap();
    println!("[SERVER] Escuchando en {}", &direccion);

    // conexiones
    let mut identificador = 0;

    for conn in listener.incoming() {
        let mut stream = conn.unwrap();

        identificador += 1;
        let conn_id = identificador;

        //cada cliente en un hilo separado
        thread::spawn(move || {
            // buffer temporal
            let mut buffer = [0; 1024];

            // sólo vamos a atender 5 solicitudes
            // por cada cliente
            for req_num in 1..=5 {
                // leer mensaje entrante (solicitud)
                let num_bytes = stream.read(&mut buffer).unwrap();

                let mensaje = String::from_utf8_lossy(&buffer[..num_bytes]);

                println!(
                    "[SERVER] Conn #{} | (req {}): {}",
                    conn_id, req_num, mensaje
                );

                // respuesta (eco con prefijo)
                let response = format!(
                    "[Conn {}] Leído. Solicitud num: {} ({})\n",
                    conn_id, req_num, mensaje
                );

                stream.write_all(response.as_bytes()).unwrap();
            }

            println!("[SERVER] ID #{}", conn_id);
        });
    }
}

// fn clientes_start(num_clientes: u8) {
//
//     let mut hilos_clientes = vec![];
//
//     for client_id in 1..=num_clientes {
//         hilos_clientes.push(thread::spawn(move || {
//
//             let mut stream = TcpStream::connect("127.0.0.1:8080")
//                             .unwrap();
//
//             for num in 1..=5 {
//                 thread::sleep(Duration::from_millis((client_id*num).into()));
//
//                 let _ = stream.write_all(
//                         format!(
//                             "Cliente #{} | (req {})\n",
//                             client_id,
//                             num
//                         ).as_bytes()
//                 ).unwrap();
//
//                 // respuesta del servidor
//                 let mut buffer = [0; 1024];
//                 let num_bytes = stream.read(&mut buffer).unwrap();
//
//                 println!(
//                     "Cliente #{} | respuesta: {}",
//                     client_id,
//                     String::from_utf8_lossy(&buffer[..num_bytes])
//                 );
//             }
//         }));
//     }
//
//     for handle in hilos_clientes {
//         handle.join().unwrap();
//     }
//
// }

async fn async_server_start() {
    let direccion = "127.0.0.1:8080";

    let listener = tokio::net::TcpListener::bind(&direccion).await;

    println!("[SERVER] Escuchando en {}", &direccion);

    // contador de conexiones
    let identificador = Arc::new(tokio::sync::Mutex::new(0));

    loop {
        let (mut stream, _) = (listener.as_ref().unwrap().accept().await).unwrap();

        // Incrementar contador de conexiones
        let conn_count = identificador.clone();
        let mut count_guard = conn_count.lock().await;
        *count_guard += 1;
        let conn_id = *count_guard;

        // liberar lock
        drop(count_guard);

        //cada cliente en un hilo separado
        tokio::spawn(async move {
            // buffer temporal
            let mut buffer = [0; 1024];

            // sólo vamos a atender 5 solicitudes
            // por cada cliente
            for req_num in 1..=5 {
                // leer mensaje entrante (solicitud)
                let num_bytes = stream.read(&mut buffer).await;
                let num_bytes = num_bytes.unwrap();

                let mensaje = String::from_utf8_lossy(&buffer[..num_bytes]);

                println!("[Async] Conn #{} | (req {}): {}", conn_id, req_num, mensaje);

                // respuesta (eco con prefijo)
                let response = format!(
                    "[Async Conn {}] Leído. Solicitud num: {} ({})\n",
                    conn_id, req_num, mensaje
                );

                _ = stream.write_all(response.as_bytes()).await;
                // .unwrap();
            }

            println!("[Async] ID #{}", conn_id);
        });
    }
}
