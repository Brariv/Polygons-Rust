use raylib::prelude::*;
mod framebuffers;
mod line;
mod fill;

fn main() {
    

    let mut framebuffer = framebuffers::FrameBuffer::new(800, 600, Color::BLACK);
    

    // (377, 249) (411, 197) (436, 249)
    let polygon_points = [
        Vector2::new(377.0, 249.0),
        Vector2::new(411.0, 197.0),
        Vector2::new(436.0, 249.0)
    ];

    for i in 0..polygon_points.len() {
        let start = polygon_points[i];
        let end = polygon_points[(i + 1) % polygon_points.len()];
        line::line(&mut framebuffer, start, end, Color::WHITE);
    }

    fill::fill(&mut framebuffer, &polygon_points, Color::RED);


    framebuffer.draw_image("out.bmp");
    println!("Drawing complete!");
    

    
}


