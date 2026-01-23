// Importamos la librería que usamos para capturar los datos del usuario
use std::io;

fn main() {
    // Creamos un vector para almacenar los tiempos que vamos a medir
    let mut historial_tiempos: Vec<f32> = Vec::new();

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
                println!("⚠️ ¡Error! Debes ingresar un número (ejemplo: 2.5) o 'salir'.");
                continue; // Vuleve al inicio del loop inmediatamente
            }
        };

        historial_tiempos.push(tiempo); // Agregamos el tiempo al historial
        println!("Dato guardado. Llevas {} análisis realizados.", historial_tiempos.len());

        // --- 3. LÓGICA DE DECISIÓN ---
        if tiempo <= 2.0 {
            println!("¡Excelente! Tu sitio es muy accesible.");
        } else if tiempo <= 5.0 {
            println!("⚠️ Está bien, pero podría mejorar.");
        } else {
            println!("🐢 ¡Cuidado! Es demasiado lento.");
        }
    }

    println!("\n--- RESUMEN DE LA SESIÓN ---");
    println!("Has validado la cantidad de {} páginas.", historial_tiempos.len());

    let suma: f32 = historial_tiempos.iter().sum();
    let cantidad = historial_tiempos.len();

    // Calculamos el promedio. OJO! Hay que convertir 'cantidad' a decimal con 'as f32'
    let promedio = suma / cantidad as f32;

    println!("El promedio de carga de hoy fue: {:.2} segundos por página.", promedio);
}
