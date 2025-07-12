use raylib::prelude::*;
mod framebuffers;
mod line;
mod fill;

fn main() {
    

    let mut framebuffer = framebuffers::FrameBuffer::new(800, 600, Color::BLACK);
    

    // (165, 380) (185, 360) (180, 330) (207, 345) (233, 330) (230, 360) (250, 380) (220, 385) (205, 410) (193, 383)
    let polygon_points = [
        Vector2::new(165.0, 380.0),
        Vector2::new(185.0, 360.0),
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];

    for i in 0..polygon_points.len() {
        let start = polygon_points[i];
        let end = polygon_points[(i + 1) % polygon_points.len()];
        line::line(&mut framebuffer, start, end, Color::WHITE);
    }

    fill::fill(&mut framebuffer, &polygon_points, Color::WHITE);


    framebuffer.draw_image("output.png");
    println!("Drawing complete!");
    

    
}


