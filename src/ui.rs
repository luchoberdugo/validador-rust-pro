// Importamos la librería que usamos para capturar los datos del usuario
use std::io;
use crate::reportes::Reporte;


pub fn pedir_entrada_de_usuario() -> Reporte {
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

        // --- GESTIÓN DE SALIDA ---
        // Si el usuario escribe "salir" rompemos el bucle
        if entrada.trim().to_lowercase() == "salir" {
            println!("Cerrando el validador... ¡Hasta pronto!");
            break; // Rompe el loop
        }

        // --- GESTIÓN DE ERRORES ---
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
        }

        let promedio = if historial_tiempos.is_empty() {
            0.0
        } else {
            historial_tiempos.iter().sum::<f32>() / historial_tiempos.len() as f32
        };

        Reporte {
            tiempos: historial_tiempos,
            total_errores: errores,
            promedio,
        }
    }
