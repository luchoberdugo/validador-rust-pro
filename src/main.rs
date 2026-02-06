use crate::reportes::Clasificacion;

mod reportes; // Importamos nuestro módulo personalizado
mod calculos; // Importamos nuestro módulo personalizado
mod ui; // Importamos nuestro módulo personalizado

fn main() {
    let mi_reporte = ui::pedir_entrada_de_usuario();

    match mi_reporte.categoria {
        Clasificacion::Excelente => println!("🚀 ¡Tu web vuela!"),
        Clasificacion::Aceptable => println!("✅ Rendimiento dentro de lo normal."),
        Clasificacion::Lento => println!("⚠️ Ojo, la carga es muy pesada."),
    }
    
    reportes::guardar_en_historial(&mi_reporte);
}
