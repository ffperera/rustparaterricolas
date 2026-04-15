use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};

fn operacion_costosa() {
    sleep(Duration::from_millis(258));
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Hora, Fecha, Duración");
    println!("--------------------");
    println!();

    // new()
    let segundos = 2;
    let nanosegundos = 0;
    let _dos_segundos = Duration::new(segundos, nanosegundos);

    // from_...
    let doce_segundos = Duration::from_secs(12);
    let cien_ms = Duration::from_millis(100);
    let doscientos_us = Duration::from_micros(200);
    let trescientos_ns = Duration::from_nanos(300);

    // operaciones simples
    let tiempo_total = doce_segundos + cien_ms + doscientos_us + trescientos_ns;

    println!("Tiempo total: {:?}", tiempo_total);

    let desde = Instant::now();

    operacion_costosa();

    // elapsed() devuelve Duration
    let duracion = desde.elapsed();

    println!("Tiempo transcurrido {:?}", duracion);

    // SystemTime  (timestamp)
    // el tiempo medido por SystemTime no es monotónico
    // ya que el tiempo de sistema puede cambiar de una
    // operación a la siguiente
    let sys_time = SystemTime::now();

    // por ese motivo hay que encapsular el cálculo
    // de invervalos en un Option
    let duracion = sys_time.duration_since(UNIX_EPOCH).unwrap();
    let segundos = duracion.as_secs();
    println!("Timestamp, segundos: {}", segundos);

    // Chrono timestamp
    let segundos = chrono::Utc::now().timestamp();
    println!("Timestamp, segundos: {}", segundos);

    // timestamp a DateTime
    let fecha_hora = DateTime::from_timestamp(1753089286, 0).unwrap();
    println!(
        "Fecha y hora UTC: {}",
        fecha_hora.format("%Y-%m-%d %H:%M:%S")
    );

    // timestamp a DateTime
    // para una zona horaria, por ejemplo la zona
    // horaria del sistema
    let fecha_hora = Local.timestamp_opt(1753089286, 0).unwrap();
    println!(
        "Fecha y hora UTC: {}",
        fecha_hora.format("%Y-%m-%d %H:%M:%S")
    );

    // chrono DateTime
    // fecha y hora actual (universal)
    let ahora: DateTime<Utc> = Utc::now();
    println!("UTC: {}", ahora.format("%Y-%m-%d %H:%M:%S"));

    // fecha y hora actual (local)
    let ahora: DateTime<Local> = Local::now();
    println!("Local: {}", ahora.format("%Y-%m-%d %H:%M:%S"));

    let ahora: DateTime<Utc> = Utc::now();
    println!("UTC: {}", ahora.format("%Y-%m-%d %H:%M:%S"));

    let future: DateTime<Utc> = ahora + chrono::Duration::days(5);
    println!("Dentro de 5 días: {}", future.format("%Y-%m-%d %H:%M:%S"));

    // obtener DateTime a partir de fechas
    // en cadenas de texto
    //
    println!("-------");
    println!("parse_from_rfc2822() / parse_from_rfc3339()");

    let rfc2822 = "Mon, 21 Jul 2025 21:00:09 +0200";
    println!(
        "Desde formato rfc2822 : {}",
        DateTime::parse_from_rfc2822(rfc2822).unwrap()
    );

    let rfc3339 = "2025-07-21T21:00:09+02:00";
    println!(
        "Desde formato rfc3339 : {}",
        DateTime::parse_from_rfc3339(rfc3339).unwrap()
    );

    // fecha en formato genérico
    let fecha = "2025-07-21 21:00:09 +0200";

    println!("En UTC: {:?}", fecha.parse::<DateTime<Utc>>());

    println!(
        "En zona horaria local: {:?}",
        fecha.parse::<DateTime<Local>>()
    );

    println!(
        "Zona horaria fija: {:?}",
        fecha.parse::<DateTime<FixedOffset>>()
    );

    // fecha en formato rfc3339
    let fecha = "2025-07-21T21:00:09Z";

    println!("En UTC: {:?}", fecha.parse::<DateTime<Utc>>());

    println!(
        "En zona horaria local: {:?}",
        fecha.parse::<DateTime<Local>>()
    );

    println!(
        "Zona horaria fija: {:?}",
        fecha.parse::<DateTime<FixedOffset>>()
    );

    let fecha = "2025 Jul 21 12:09:14.274 +0200";
    let date_time = DateTime::parse_from_str(&fecha, "%Y %b %d %H:%M:%S%.3f %z");

    println!("Fecha en zona horaria fija: {}", date_time.unwrap());

    println!("-------");
    println!("format");

    let fecha = "2025-07-21T21:00:09Z";
    let date_time = DateTime::parse_from_rfc3339(fecha);
    let date_time = date_time.unwrap();

    // generar un String con format() y to_string()
    let texto = date_time.format("%Y/%m/%d %H:%M:%S %z").to_string();

    println!("Formato a medida: {}", texto);

    let solo_fecha = date_time.format("%Y/%m/%d").to_string();

    println!("Sólo fecha: {}", solo_fecha);

    // format() se puede utilizar directamente
    // como argumento, sin convertir a String
    println!("Sólo hora: {}", date_time.format("%H:%M:%S"));

    // generar directamente formato rfc2822
    println!("rfc2822: {}", date_time.to_rfc2822());

    // generar directamente formato rfc3339
    println!("rfc3339: {}", date_time.to_rfc3339());

    println!("-------");
    println!("cálculos con fechas");

    // cálculos de fechas

    // let now = Utc::now();
    // let almost_three_weeks_from_now = now
    //         .checked_add_signed(Duration::weeks(2))
    //         .and_then(|in_2weeks|
    //             in_2weeks.checked_add_signed(Duration::weeks(1)))
    //         .and_then(day_earlier);
    //
    // match almost_three_weeks_from_now {
    //     Some(x) => println!("{}", x),
    //     None => eprintln!("Almost three weeks from now overflows!"),
    // }
    //

    // viaje a Marte
    let lanzamiento = Utc::now();

    // se utiliza chrono::Duration
    // que incluye más métodos útiles
    // ~7 meses
    let duracion_viaje = chrono::Duration::days(210);

    // DateTime + Duration
    let llegada_marte = lanzamiento + duracion_viaje;

    println!("Lanzamiento: {}", lanzamiento.format("%Y-%m-%d"));
    println!("Llegada estimada: {}", llegada_marte.format("%Y-%m-%d"));

    // diferencia (chrono::Duration)
    let diferencia = llegada_marte.signed_duration_since(lanzamiento);
    println!("Duración del viaje: {} días", diferencia.num_days());

    // comparar fechas
    // en este caso utilizando NaiveDateTime
    let hoy = NaiveDateTime::parse_from_str("2025-07-21T21:00:09", "%Y-%m-%dT%H:%M:%S").unwrap();

    let ventana_lanzamiento =
        NaiveDateTime::parse_from_str("2025-12-21T21:00:09", "%Y-%m-%dT%H:%M:%S").unwrap();

    match hoy.cmp(&ventana_lanzamiento) {
        std::cmp::Ordering::Less => {
            let dias_restantes = ventana_lanzamiento - hoy;
            println!("{} días para el lanzamiento.", dias_restantes.num_days());
        }
        std::cmp::Ordering::Equal => println!("Lanzamiento inminente."),
        std::cmp::Ordering::Greater => println!("Misión en progreso."),
    }

    // NaiveDate permite representar sólo fechas (sin hora)
    // podríamos haber hecho lo mismo con NaiveDateTime o DateTime
    let inicio = NaiveDate::from_ymd_opt(2024, 8, 1).unwrap();
    let fin = NaiveDate::from_ymd_opt(2024, 8, 5).unwrap();

    println!("Planificación diaria de misión:");
    let mut dia_actual = inicio;
    while dia_actual <= fin {
        println!("Día {}: Preparación sistemas", dia_actual.format("%d/%m"));

        dia_actual = dia_actual + chrono::Duration::days(1);
    }
}
