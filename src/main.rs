mod reportes; // Importamos nuestro módulo personalizado
mod calculos; // Importamos nuestro módulo personalizado
mod ui; // Importamos nuestro módulo personalizado

fn main() {
    let mi_reporte = ui::pedir_entrada_de_usuario();
    reportes::guardar_en_historial(&mi_reporte);
}
