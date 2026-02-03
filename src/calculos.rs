// 'fn' para declarar función
// 'tiempo: f32' es lo que recibe
// '-> String' es lo que devuelve al final
pub fn clasificar_velocidad(tiempo: f32) -> String {
        if tiempo <= 2.0 {
        String::from("🚀 Excelente")
        } else if tiempo <= 5.0 {
            String::from("⚠️ Aceptable")
        } else {
            String::from("🐢 Muy lento")
        }
    }
