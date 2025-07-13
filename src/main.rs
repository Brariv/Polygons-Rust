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

    fill::fill(&mut framebuffer, &polygon_points, Color::YELLOW);

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


    // (413, 177) (448, 159) (502, 88) (553, 53) (535, 36) (676, 37) (660, 52) (750, 145) (761, 179) (672, 192) (659, 214) (615, 214) (632, 230) (580, 230) (597, 215) (552, 214) (517, 144) (466, 180)
    let polygon_points = [
        Vector2::new(413.0, 177.0),
        Vector2::new(448.0, 159.0),
        Vector2::new(502.0, 88.0),
        Vector2::new(553.0, 53.0),
        Vector2::new(535.0, 36.0),
        Vector2::new(676.0, 37.0),
        Vector2::new(660.0, 52.0),
        Vector2::new(750.0, 145.0),
        Vector2::new(761.0, 179.0),
        Vector2::new(672.0, 192.0),
        Vector2::new(659.0, 214.0),
        Vector2::new(615.0, 214.0),
        Vector2::new(632.0, 230.0),
        Vector2::new(580.0, 230.0),
        Vector2::new(597.0, 215.0),
        Vector2::new(552.0, 214.0),
        Vector2::new(517.0, 144.0),
        Vector2::new(466.0, 180.0)
    ];


    //(682, 175) (708, 120) (735, 148) (739, 170)
    let polygon_points_hole = [
        Vector2::new(682.0, 175.0),
        Vector2::new(708.0, 120.0),
        Vector2::new(735.0, 148.0),
        Vector2::new(739.0, 170.0)
    ];

    for i in 0..polygon_points.len() {
        let start = polygon_points[i];
        let end = polygon_points[(i + 1) % polygon_points.len()];
        line::line(&mut framebuffer, start, end, Color::WHITE);
    }

    for i in 0..polygon_points_hole.len() {
        let start = polygon_points_hole[i];
        let end = polygon_points_hole[(i + 1) % polygon_points_hole.len()];
        line::line(&mut framebuffer, start, end, Color::WHITE);
    }

    fill::fill(&mut framebuffer, &polygon_points, Color::GREEN);
    fill::fill(&mut framebuffer, &polygon_points_hole, Color::BLACK);


    framebuffer.draw_image("out.bmp");
    println!("Drawing complete!");
    

    
}


