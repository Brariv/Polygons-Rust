use raylib::prelude::*;
mod framebuffers;
mod line;
mod fill;

fn main() {
    

    let mut framebuffer = framebuffers::FrameBuffer::new(800, 600, Color::BLACK);
    

    // (321, 335) (288, 286) (339, 251) (374, 302)
    let polygon_points = [
        Vector2::new(321.0, 335.0),
        Vector2::new(288.0, 286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0)
    ];

    for i in 0..polygon_points.len() {
        let start = polygon_points[i];
        let end = polygon_points[(i + 1) % polygon_points.len()];
        line::line(&mut framebuffer, start, end, Color::WHITE);
    }

    fill::fill(&mut framebuffer, &polygon_points, Color::BLUE);


    framebuffer.draw_image("out.bmp");
    println!("Drawing complete!");
    

    
}


