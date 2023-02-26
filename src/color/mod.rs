use std::io::{Error, Write};

//mod vec3;
pub mod color { 
        use crate::Vec3;
        pub type Color = Vec3;

        pub mod utils {
            use crate::color::color::Color;
            //,use color;
        // this is simplified 
        pub fn write_color(pixel_color: Color) {
            println!(
                "{} {} {}", 
                (255.999 * pixel_color.x()) as i32,
                (255.999 * pixel_color.y()) as i32,
                (255.999 * pixel_color.z()) as i32
            );
        }
        }

}
