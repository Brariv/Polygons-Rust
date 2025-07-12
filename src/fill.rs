use raylib::prelude::*;
use crate::framebuffers;

pub fn fill(
    framebuffer: &mut framebuffers::FrameBuffer,
    polygon_points: &[Vector2],
    fill_color: Color,
) {
    
    // 1. Calcular los límites del polígono
    let min_y = polygon_points.iter().map(|p| p.y as i32).min().unwrap_or(0);
    let max_y = polygon_points.iter().map(|p| p.y as i32).max().unwrap_or(0);

    // 2. Para cada scanline (línea horizontal)
    for y in min_y..=max_y {
        let mut intersections: Vec<f32> = Vec::new();

        // 3. Recorremos cada par de puntos (borde del polígono)
        for i in 0..polygon_points.len() {
            let p1 = polygon_points[i];
            let p2 = polygon_points[(i + 1) % polygon_points.len()]; // siguiente punto, con cierre

            // Revisar si la línea cruza el scanline actual
            if (p1.y as i32 <= y && p2.y as i32 > y) || (p2.y as i32 <= y && p1.y as i32 > y) {
                let dy = p2.y - p1.y;
                if dy != 0.0 {
                    let t = (y as f32 - p1.y) / dy;
                    let x = p1.x + t * (p2.x - p1.x);
                    intersections.push(x);
                }
            }
        }

        // 4. Ordenar intersecciones de izquierda a derecha
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // 5. Rellenar entre pares de intersecciones
        let mut i = 0;
        while i + 1 < intersections.len() {
            let start_x = intersections[i].ceil() as i32;
            let end_x = intersections[i + 1].floor() as i32;

            for x in start_x..=end_x {
                framebuffer.set_pixel(x, y, fill_color);
            }

            i += 2; // avanzar al siguiente par
        }
    }
}

