/// Módulo que agrupa todos los capítulos del libro de Rust.
///
/// En este módulo se incluyen ejemplos de código organizados por capítulos,
/// cubriendo desde los conceptos básicos hasta temas más avanzados como
/// programación asíncrona, testing y patrones de arquitectura.
pub mod cap_004 {
    /// Capítulo 4: Declaraciones, tipos de datos y casting.
    ///
    /// En este capítulo vemos ejemplos relacionados con declaración de variables,
    /// shadowing, tipos de datos primitivos y casting entre tipos.
    pub mod casting_as;
    /// Capítulo 4: Declaraciones y tipos de datos.
    ///
    /// En este capítulo vemos ejemplos relacionados con declaración de variables,
    /// tipos de datos, constantes y shadowing.
    pub mod declaracion;
}

pub mod cap_005 {
    /// Capítulo 5: Ownership y Borrowing.
    ///
    /// En este capítulo vemos ejemplos relacionados con ownership, reglas de ownership,
    /// referencias, borrowing y conceptos básicos de lifetimes.
    pub mod ownership;
}

pub mod cap_007 {
    /// Capítulo 7: Funciones.
    ///
    /// En este capítulo vemos ejemplos relacionados con definición de funciones,
    /// parámetros, valores de retorno, funciones pura y closure basics.
    pub mod funciones;
}

pub mod cap_008 {
    /// Capítulo 8: Expresiones.
    ///
    /// En este capítulo vemos ejemplos relacionados con expresiones en Rust,
    /// evaluación de expresiones, expresiones de asignación y evaluación diferida.
    pub mod expresiones;
}

pub mod cap_009 {
    /// Capítulo 9: Control de flujo: if-else.
    ///
    /// En este capítulo vemos ejemplos relacionados con estructuras condicionales,
    /// expresiones if-else, if-let y evaluación de expresiones booleanas.
    pub mod if_else;
}

pub mod cap_010 {
    /// Capítulo 10: Tuplas.
    ///
    /// En este capítulo vemos ejemplos relacionados con tuplas, acceso a elementos,
    /// tuplas como valores de retorno y destructuración de tuplas.
    pub mod tuplas;
}

pub mod cap_011 {
    /// Capítulo 11: Enumerados.
    ///
    /// En este capítulo vemos ejemplos relacionados con enumerados, variantes de enum,
    /// enums con datos, métodos en enums y el trait Option.
    pub mod enums;
}

pub mod cap_012 {
    /// Capítulo 12: Match y Pattern Matching.
    ///
    /// En este capítulo vemos ejemplos relacionados con la expresión match,
    /// pattern matching, destructuración, guards y wildcards.
    pub mod match_chapter;
}

pub mod cap_013 {
    /// Capítulo 13: Option y Result.
    ///
    /// En este capítulo vemos ejemplos relacionados con el enum Option,
    /// el enum Result, manejo de errores y el operador ?.
    pub mod option_result;
}

pub mod cap_014 {
    /// Capítulo 14: Estructuras.
    ///
    /// En este capítulo vemos ejemplos relacionados con structs, estructuras de tupla,
    /// métodos en structs, herencia de structs y associated functions.
    pub mod structs;
}

pub mod cap_015 {
    /// Capítulo 15: Arrays.
    ///
    /// En este capítulo vemos ejemplos relacionados con arrays, inicialización de arrays,
    /// acceso a elementos, arrays en la pila y límites de Arrays.
    pub mod arrays;
}

pub mod cap_016 {
    /// Capítulo 16: Slices.
    ///
    /// En este capítulo vemos ejemplos relacionados con slices, &str,
    /// slices de arrays, reference sharing y slice patterns.
    pub mod slices;
}

pub mod cap_017 {
    /// Capítulo 17: Vectores.
    ///
    /// En este capítulo vemos ejemplos relacionados con vectores, creación de vectores,
    /// métodos de vectores, iteración y capacidad de vectores.
    pub mod vectores;
}

pub mod cap_018 {
    /// Capítulo 18: Strings.
    ///
    /// En este capítulo vemos ejemplos relacionados con String, &str,
    /// operaciones con strings, UTF-8 y manipulación de texto.
    pub mod strings;
}

pub mod cap_020 {
    /// Capítulo 20: Traits.
    ///
    /// En este capítulo vemos ejemplos relacionados con traits, definición de traits,
    /// implementación de traits, trait bounds y traits como parámetros.
    pub mod traits;
}

pub mod cap_021 {
    /// Capítulo 21: Implementación de Traits.
    ///
    /// En este capítulo vemos ejemplos relacionados con implementación de traits,
    /// trait para tipos existentes, traits con parámetros genéricos y default implementations.
    pub mod implementar;
}

pub mod cap_022 {
    /// Capítulo 22: Tipos Genéricos y impl Trait.
    ///
    /// En este capítulo vemos ejemplos relacionados con genéricos, tipos genéricos en structs,
    /// impl Trait como tipo de retorno y abstracción sobre tipos concretos.
    pub mod generic_estructuras;
    /// Capítulo 22: impl Trait.
    ///
    /// En este capítulo vemos ejemplos relacionados con impl Trait como tipo de retorno,
    /// ocultación de tipos concretos y diferencias con generics.
    pub mod return_impl_trait;
}

pub mod cap_024 {
    /// Capítulo 24: Traits Genéricos.
    ///
    /// En este capítulo vemos ejemplos relacionados con traits genéricos,
    /// trait bounds múltiples, where clauses y generic trait implementation.
    pub mod generic_trait_01;
    /// Capítulo 24: Traits Genéricos Avanzado.
    ///
    /// En este capítulo vemos ejemplos relacionados con traits genéricos avanzados,
    ///trait bounds con generics, Default trait y constantes genéricas.
    pub mod generic_trait_02;
    /// Capítulo 24: Sobrecarga de Operadores.
    ///
    /// En este capítulo vemos ejemplos relacionados con sobrecarga de operadores,
    /// implementación de traits de operadores y operadores personalizados.
    pub mod sobrecarga_operadores;
    /// Capítulo 24: Tipos Genéricos por Defecto.
    ///
    /// En este capítulo vemos ejemplos relacionados con tipos genéricos por defecto,
    /// aplicación de valores por defecto y combinación con otros generics.
    pub mod default_generic_type;
    /// Capítulo 24: Constantes Genéricas.
    ///
    /// En este capítulo vemos ejemplos relacionados con constantes genéricas,
    /// const generics, arrays de tamaño genérico y evaluación en tiempo de compilación.
    pub mod constante_generica;
}

pub mod cap_025 {
    /// Capítulo 25: Closures.
    ///
    /// En este capítulo vemos ejemplos relacionados con closures, funciones anónimas,
    /// capturing environment, Fn, FnMut y FnOnce traits.
    pub mod closures;
    /// Capítulo 25: Closures Avanzado.
    ///
    /// En este capítulo vemos ejemplos relacionados con closures avanzados,
    /// Type annotations, closures como parámetros y retorno de closures.
    pub mod closures2;
}

pub mod cap_026 {
    /// Capítulo 26: Colecciones.
    ///
    /// En este capítulo vemos ejemplos relacionados con colecciones de la librería estándar,
    /// HashMap, HashSet, BTreeMap y operaciones comunes con colecciones.
    pub mod colecciones;
}

pub mod cap_027 {
    /// Capítulo 27: Iteradores.
    ///
    /// En este capítulo vemos ejemplos relacionados con el trait Iterator,
    /// iteradores, adaptadores de iteradores, consumidores y evaluación "lazy".
    pub mod iteradores;
}

pub mod cap_028 {
    /// Capítulo 28: Bucles.
    ///
    /// En este capítulo vemos ejemplos relacionados con bucles, loop, while,
    /// for loops, iteración sobre ranges y control de flujo en bucles.
    pub mod bucles;
}

pub mod cap_029 {
    /// Capítulo 29: Smart Pointers.
    ///
    /// En este capítulo vemos ejemplos relacionados con Box, Rc, RefCell,
    /// smart pointers, interior mutability y patrones de ownership complejos.
    pub mod smart_pointers;
}

pub mod cap_032 {
    /// Capítulo 32: From e Into.
    ///
    /// En este capítulo vemos ejemplos relacionados con los traits From e Into,
    /// conversión de tipos, type casting personalizado y el trait FromStr.
    pub mod casting;
    /// Capítulo 32: TryFrom y TryInto.
    ///
    /// En este capítulo vemos ejemplos relacionados con TryFrom y TryInto,
    /// conversión de tipos que puede fallar, manejo de errores en conversiones.
    pub mod casting_try_from;
}

pub mod cap_033 {
    /// Capítulo 33: Manejo de Errores.
    ///
    /// En este capítulo vemos ejemplos relacionados con manejo de errores personalizado,
    /// creación de tipos de error, Trait objects para errores y el patrón Result.
    pub mod errores;
    /// Capítulo 33: Dispatch de Errores.
    ///
    /// En este capítulo vemos ejemplos relacionados con dispatch de errores,
    /// Box<dyn Error>, enums de error y diferentes estrategias de manejo.
    pub mod errores_dispatch;
}

pub mod cap_034 {
    /// Capítulo 34: Lifetimes.
    ///
    /// En este capítulo vemos ejemplos relacionados con lifetimes, anotaciones de lifetime,
    /// referencias válidas, lifetime elisión y reglas de lifetimes.
    pub mod lifetimes;
}

pub mod cap_035 {
    /// Capítulo 35: DateTime.
    ///
    /// En este capítulo vemos ejemplos relacionados con fechas y tiempos,
    /// librería chrono, duración, timestamp y operaciones con fechas.
    pub mod datetime;
}

pub mod cap_036 {
    /// Capítulo 36: Hilos.
    ///
    /// En este capítulo vemos ejemplos relacionados con threads, creación de hilos,
    /// spawn y join, Arc y Mutex para compartir estado entre hilos.
    pub mod threads;
    /// Capítulo 36: Canales.
    ///
    /// En este capítulo vemos ejemplos relacionados con canales de comunicación,
    /// mpsc channels, envío de mensajes entre hilos y diseño concurrent.
    pub mod threads_channels;
}

pub mod cap_037 {
    /// Capítulo 37: Streams y Archivos.
    ///
    /// En este capítulo vemos ejemplos relacionados con streams, lectura y escritura,
    /// archivos, buffered I/O y operaciones de I/O asíncrono.
    pub mod streams_io;
    /// Capítulo 37: Apertura y Creación de Archivos.
    ///
    /// En este capítulo vemos ejemplos relacionados con abrir y crear archivos,
    /// File::open, File::create, opciones de apertura y manejo de rutas.
    pub mod streams_io_open_create;
    /// Capítulo 37: Seek y Posicionamiento.
    ///
    /// En este capítulo vemos ejemplos relacionados con posicionamiento en archivos,
    /// seek, tell, lectura en posiciones específicas y archivos de tamaño variable.
    pub mod streams_io_seek;
    /// Capítulo 37: Entrada Estándar.
    ///
    /// En este capítulo vemos ejemplos relacionados con stdin, lectura interactiva,
    /// buffered reader y procesamiento de entrada del usuario.
    pub mod streams_io_stdin;
}

pub mod cap_038 {
    /// Capítulo 38: Async/Await - Join.
    ///
    /// En este capítulo vemos ejemplos relacionados con async/await,
    /// ejecución concurrente, join de futures y manejo de múltiples tareas.
    pub mod async_await_join;
    /// Capítulo 38: Async/Await - Select.
    ///
    /// En este capítulo vemos ejemplos relacionados con select,
    /// selección entre múltiples futures, race conditions y timeouts.
    pub mod async_await_select;
    /// Capítulo 38: Async/Await - JoinSet.
    ///
    /// En este capítulo vemos ejemplos relacionados con JoinSet,
    /// gestión de múltiples tareas, spawn dinámicos y cancelación.
    pub mod async_await_joinset;
    /// Capítulo 38: Async/Await - Runtime.
    ///
    /// En este capítulo vemos ejemplos relacionados con runtime de Tokio,
    /// setup del runtime, spawn de tareas y #[tokio::main].
    pub mod async_await_runtime;
    /// Capítulo 38: Async/Await - Abort.
    ///
    /// En este capítulo vemos ejemplos relacionados con abort de tareas,
    /// cancelación de futures, graceful shutdown y manejo de tareas huérfanas.
    pub mod async_await_abort;
    /// Capítulo 38: Async/Await - Canales.
    ///
    /// En este capítulo vemos ejemplos relacionados con canales asíncronos,
    /// tokio channels, mpsc y comunicación entre tareas async.
    pub mod async_await_canales;
    /// Capítulo 38: Async/Await - Cancellation Token.
    ///
    /// En este capítulo vemos ejemplos relacionados con cancellation tokens,
    /// señales de cancelación, cooperative cancellation y shutdown coordinated.
    pub mod async_await_cancellation_token;
    /// Capítulo 38: Async/Await - Println.
    ///
    /// En este capítulo vemos ejemplos relacionados con println en contexto async,
    /// macros de debug tokio, logging y output en aplicaciones async.
    pub mod async_await_println;
}

pub mod cap_039 {
    /// Capítulo 39: TCP Sockets.
    ///
    /// En este capítulo vemos ejemplos relacionados con networking,
    /// TCP sockets, clientes y servidores, y comunicación por red.
    pub mod tcp_sockets;
}

pub mod cap_040 {
    /// Capítulo 40: Testing.
    ///
    /// En este capítulo vemos ejemplos relacionados con testing,
    /// unit tests, integración, assertions y marcos de testing.
    pub mod testing;
}

pub mod cap_041 {
    /// Capítulo 41: Arquitectura.
    ///
    /// En este capítulo vemos ejemplos relacionados con patrones de arquitectura,
    /// ports and adapters, Hexagonal architecture y functional core, imperative shell.
    pub mod arquitectura;
    /// Capítulo 41: Arquitectura Avanzada.
    ///
    /// En este capítulo vemos ejemplos relacionados con arquitectura avanzada,
    /// servicios, repositorios, casos de uso y separación de responsabilidades.
    pub mod arquitectura_2;
    /// Capítulo 41: Arquitectura - Functional Core.
    ///
    /// En este capítulo vemos ejemplos relacionados con functional core imperative shell,
    /// domínio puro, capa de aplicación, adapters de infraestructura y patrones hex.
    pub mod arquitectura_3;
}