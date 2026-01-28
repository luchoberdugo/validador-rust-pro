// Importamos la librería que usamos para capturar los datos del usuario
use std::io;
// use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)] // Esto permite imprimir en pantalla usando {:?}
struct Reporte {
    tiempos: Vec<f32>,
    total_errores: i32,
    promedio: f32,
}

fn main() {
    // Creamos un vector para almacenar los tiempos que vamos a medir
    let mut historial_tiempos: Vec<f32> = Vec::new();
    let mut errores: i32 = 0;

    // El bucle "loop" se repite para siempre hasta que lo detengas
    loop {
        println!("\n--- Validador de Carga (Escribe 'salir' para cerrar) ---");
        println!("¿Cuántos segundos tarda en cargar la web?");

        let mut entrada = String::new();

        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");

        // --- 1. GESTIÓN DE SALIDA ---
        // Si el usuario escribe "salir" rompemos el bucle
        if entrada.trim().to_lowercase() == "salir" {
            println!("Cerrando el validador... ¡Hasta pronto!");
            break; // Rompe el loop
        }

        // --- 2. GESTIÓN DE ERRORES ---
        // Usamos match para convertir el texto a número sin que el programa explote

        let tiempo: f32 = match entrada.trim().parse() {
            Ok(num) => num, // Si todo salió bien, entonces guardamos el número en 'tiempo'
            Err(_) => {
                errores += 1;
                println!("⚠️ ¡Error! Debes ingresar un número (ejemplo: 2.5) o 'salir'.");
                continue; // Vuleve al inicio del loop inmediatamente
            }
        };

        historial_tiempos.push(tiempo); // Agregamos el tiempo al historial
        println!("Dato guardado. Llevas {} análisis realizados.", historial_tiempos.len());

        // --- 3. LÓGICA DE DECISIÓN ---
        let mensaje: String = clasificar_velocidad(tiempo);
        println!("Resultado: {mensaje}");
    }

    println!("\n--- RESUMEN DE LA SESIÓN ---");
    println!("Has validado la cantidad de {} páginas.", historial_tiempos.len());

    let suma: f32 = historial_tiempos.iter().sum();
    let cantidad = historial_tiempos.len();

    // Calculamos el promedio. OJO! Hay que convertir 'cantidad' a decimal con 'as f32'
    let promedio = suma / cantidad as f32;

    println!("El promedio de carga de hoy fue: {:.2} segundos por página.", promedio);
    println!("Total errores de entrada: {}", errores);

    let mi_reporte = Reporte {
        tiempos: historial_tiempos.clone(),
        total_errores: errores,
        promedio: promedio,
    };

    let mut archivo = OpenOptions::new()
        .append(true)
        .create(true)
        .open("salida/historial_rendimiento.txt")
        .expect("No se pudo abrir el archivo de historial");

    // Añadimos una marca de separación para que el historial sea legible
    write!(archivo, "\n--- NUEVA SESIÓN ---\n").unwrap();
    write!(archivo, "{:#?}\n", mi_reporte).unwrap();
    write!(archivo, "---------------------\n").unwrap();

    println!("✅ Datos añadidos al historial.");
}

// 'fn' para declarar función
// 'tiempo: f32' es lo que recibe
// '-> String' es lo que devuelve al final
fn clasificar_velocidad(tiempo: f32) -> String {
    if tiempo <= 2.0 {
        String::from("🚀 Excelente")
    } else if tiempo <= 5.0 {
        String::from("⚠️ Aceptable")
    } else {
        String::from("🐢 Muy lento")
    }
}
