use raylib::prelude::*;

pub struct FrameBuffer{
    new_image: Image,
    image_width: i32, 
    image_height: i32,
    _color: Color
}

impl FrameBuffer {

    pub fn new(image_width: i32, image_height: i32, color: Color) -> Self {
        let new_image = Image::gen_image_color(image_width, image_height, color);
        Self {
            new_image,
            image_width,
            image_height,
            _color: color
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.image_width && y >= 0 && y < self.image_height {
            self.new_image.draw_pixel(
                x as i32, 
                y as i32, 
                color
            )
        }
    }

    // pub fn get_pixel(&self, x: i32, y: i32) -> Color {
    //     if x >= 0 && x < self.image_width && y >= 0 && y < self.image_height {
    //         let image_data = self.new_image.get_image_data();
    //         let idx = (y * self.image_width + x) as usize;
    //         if idx < image_data.len() {
    //             image_data[idx]
    //         } else {
    //             Color::BLANK
    //         }
    //     }
    //     else {
    //         Color::BLANK
    //     }
    // }

    pub fn draw_image(&self, output_file_name: &str) {
        self.new_image.export_image(output_file_name);
        println!("Image created and saved as '{}'!", output_file_name);
    }
}

