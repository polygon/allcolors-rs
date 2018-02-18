#[macro_use]
extern crate glium;
extern crate image;
extern crate rand;

mod display;
mod colortree;
mod imagemap;
mod allcolors;

use allcolors::AllColors;
use std::fs::File;

struct Program {
    pub allcolors: AllColors,
    saved: bool
}

impl Program {
    fn new(bits: u32, aspect: f64) -> Program {
        Program{allcolors: AllColors::new(bits, aspect), saved: false}
    }
}

impl display::DisplayProgram for Program {
    fn update(&mut self) -> Vec<Vec<(u8, u8, u8)>> {
        if !self.allcolors.done() {
            for _ in 0..self.allcolors.image.num_open() {
                self.allcolors.iterate();
            }
        } else {
            if !self.saved {
                self.saved = true;
                let mut im = image::ImageBuffer::new(self.allcolors.width as u32, self.allcolors.height as u32);
                let v = self.allcolors.to_image();
                for i in 0..self.allcolors.width {
                    for j in 0..self.allcolors.height {
                        let &p = &v[i][j];
                        im.put_pixel(i as u32, j as u32, image::Rgb([p.0, p.1, p.2]));
                    }
                }
                im.save("finished.png");
            }
        }
        self.allcolors.to_image()
    }
}

fn main() {
    let mut program = Program::new(8, 1.5);
    let sx = (program.allcolors.width / 3) as usize;
    let sy = (program.allcolors.height / 3) as usize;
    program.allcolors.seed(0, 0, (0, 0, 0));
    display::run(program);
}