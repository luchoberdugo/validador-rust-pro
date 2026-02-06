use crate::reportes::Clasificacion;

// 'fn' para declarar función
// 'tiempo: f32' es lo que recibe
// '-> String' es lo que devuelve al final
pub fn clasificar_velocidad(tiempo: f32) -> Clasificacion {
        if tiempo < 1.0 {
            Clasificacion::Excelente
        } else if tiempo <= 3.0 {
            Clasificacion::Aceptable
        } else {
            Clasificacion::Lento
        }
    }
