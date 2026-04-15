use std::env;

mod caps;

fn main() {
    let comandos = vec![
        "castingas",
        "declaracion",
        "ownership",
        "funciones",
        "expresiones",
        "if",
        "tuplas",
        "enums",
        "match",
        "option",
        "structs",
        "arrays",
        "slices",
        "vectores",
        "strings",
        "traits",
        "generic",
        "closures",
        "colecciones",
        "iteradores",
        "bucles",
        "smartpointers",
        "casting",
        "errores",
        "lifetimes",
        "threads",
        "entradasalida",
        "async",
        "datetime",
        "redes",
        "arquitectura",
    ];

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: {} <comando>", args[0]);
        eprint!("Comandos disponibles: ");

        for comando in &comandos {
            eprint!("{} ", comando);
        }

        std::process::exit(1);
    }

    match args[1].as_str() {
        "castingas" => caps::cap_004::casting_as::run(),
        "declaracion" => caps::cap_004::declaracion::run(),
        "ownership" => caps::cap_005::ownership::run(),
        "funciones" => caps::cap_007::funciones::run(),
        "expresiones" => caps::cap_008::expresiones::run(),
        "if" => caps::cap_009::if_else::run(),
        "tuplas" => caps::cap_010::tuplas::run(),
        "enums" => caps::cap_011::enums::run(),
        "match" => caps::cap_012::match_chapter::run(),
        "option" => caps::cap_013::option_result::run(),
        "structs" => caps::cap_014::structs::run(),
        "arrays" => caps::cap_015::arrays::run(),
        "slices" => caps::cap_016::slices::run(),
        "vectores" => caps::cap_017::vectores::run(),
        "strings" => caps::cap_018::strings::run(),
        "traits" => caps::cap_020::traits::run(),
        "implementar" => caps::cap_021::implementar::run(),
        "generic" => {
            caps::cap_022::generic_estructuras::run();
            caps::cap_022::return_impl_trait::run();
            caps::cap_024::generic_trait_01::run();
            caps::cap_024::generic_trait_02::run();
            caps::cap_024::sobrecarga_operadores::run();
            caps::cap_024::default_generic_type::run();
            caps::cap_024::constante_generica::run();
        }

        "closures" => {
            caps::cap_025::closures::run();
            caps::cap_025::closures2::run();
        }

        "colecciones" => caps::cap_026::colecciones::run(),
        "iteradores" => caps::cap_027::iteradores::run(),
        "bucles" => caps::cap_028::bucles::run(),
        "smartpointers" => caps::cap_029::smart_pointers::run(),

        "casting" => {
            caps::cap_032::casting::run();
            caps::cap_032::casting_try_from::run();
        }

        "errores" => {
            caps::cap_033::errores::run();
            caps::cap_033::errores_dispatch::run();
        }

        "lifetimes" => caps::cap_034::lifetimes::run(),
        "threads" => {
            caps::cap_036::threads::run();
            caps::cap_036::threads_channels::run();
        }

        "entradasalida" => {
            caps::cap_037::streams_io_open_create::run();
            caps::cap_037::streams_io::run();
            caps::cap_037::streams_io_stdin::run();
            caps::cap_037::streams_io_seek::run();
        }

        "async" => {
            caps::cap_038::async_await_join::run();
            caps::cap_038::async_await_select::run();
            caps::cap_038::async_await_joinset::run();
            caps::cap_038::async_await_runtime::run();
            caps::cap_038::async_await_abort::run();
            caps::cap_038::async_await_canales::run();
            caps::cap_038::async_await_cancellation_token::run();
            caps::cap_038::async_await_println::run();
        }
        "redes" => caps::cap_039::tcp_sockets::run(),
        "datetime" => caps::cap_035::datetime::run(),
        "testing" => caps::cap_040::testing::run(),
        "arquitectura" => {
            caps::cap_041::arquitectura::run();
            caps::cap_041::arquitectura_2::run();
            caps::cap_041::arquitectura_3::run();
        }

        _ => std::process::exit(1),
    }
}