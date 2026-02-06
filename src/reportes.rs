use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
pub enum Clasificacion {
    Excelente,
    Aceptable,
    Lento,
}

#[derive(Debug)] // Esto permite imprimir en pantalla usando {:?}
pub struct Reporte {
    pub tiempos: Vec<f32>,
    pub total_errores: i32,
    pub promedio: f32,
    pub categoria: Clasificacion,
}

pub fn guardar_en_historial(datos: &Reporte) {
    let mut archivo = OpenOptions::new()
        .append(true)
        .create(true)
        .open("salida/historial_rendimiento.txt")
        .expect("No se pudo abrir el archivo de historial");

    // Añadimos una marca de separación para que el historial sea legible
    write!(archivo, "\n--- NUEVA SESIÓN ---\n").unwrap();
    write!(archivo, "{:#?}\n", datos).unwrap();
    write!(archivo, "---------------------\n").unwrap();
}

