use colortree;
use imagemap;
use colortree::Point;
use rayon::prelude::*;

#[derive(Debug)]
pub struct AllColors {
    aspect: f64,
    pub width: usize,
    pub height: usize,
    bits: u32,
    pub ctree: colortree::ColorTree,
    pub image: imagemap::Image,
}

impl AllColors {
    pub fn new(bits: u32, aspect: f64) -> AllColors {
        let colors = 2u32.pow(3*bits) as f64;
        let lx = ((colors * aspect).sqrt() * 0.95) as usize;
        let ly = ((colors / aspect).sqrt() * 0.95) as usize;
        AllColors {
            aspect: aspect,
            width: lx, height: ly,
            bits: bits,
            ctree: colortree::ColorTree::new(bits),
            image: imagemap::Image::new(lx, ly)
        }
    }

    pub fn seed(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        self.image.place(x, y, color);
        self.ctree.remove(colortree::Point{r: color.0 as u32, g: color.1 as u32, b: color.2 as u32});
    }

    pub fn iterate(&mut self) {
        if let Some((x, y)) = self.image.get_open() {
            let (ar, ag, ab) = self.average(x, y);
            if let Some((Point{r, g, b}, ..)) = self.ctree.nearest(Point{r: ar as u32, g: ag as u32, b: ab as u32}) {
                self.image.place(x, y, (r as u8, g as u8, b as u8));
                self.ctree.remove(Point{r, g, b});
            } else {
                println!("Warning: colortree is out of colors, check done");
            }
        }
    }

    pub fn done(&self) -> bool {
        self.image.done()
    }

    fn average(&self, x: usize, y: usize) -> (u8, u8, u8) {
        let (n, (r, g, b)) =
            (-1..2).into_par_iter().flat_map(|i| {
                (-1..2).into_par_iter().filter_map(move |j| {
                    let cx = x as i32 + i;
                    let cy = y as i32 + j;
                    if let Some((nr, ng, nb)) = self.image.get_color(cx, cy) {
                        let r = (nr as f64).powf(2.);
                        let g = (ng as f64).powf(2.);
                        let b = (nb as f64).powf(2.);
                        Some((1usize, (r, g, b)))
                    } else {
                        None
                    }
                })
            })
            .reduce(|| (0usize, (0f64, 0f64, 0f64)),
                  |(n, (r, g, b)), (n1, (r1, g1, b1))| (n + n1, (r + r1, g + g1, b + b1))
            );

        let num = n as f64;
        ((r/num).sqrt() as u8, (g/num).sqrt() as u8, (b/num).sqrt() as u8)
    }

    pub fn to_image(&self) -> Vec<Vec<(u8, u8, u8)>> {
        let fac: f64 = 256. / 2f64.powf(self.bits as f64);
        self.image.to_image((0, 0, 0))
            .par_iter()
            .map(|vs| {
                vs.into_iter()
                    .map(|v| {
                        ((v.0 as f64 * fac) as u8,
                         (v.1 as f64 * fac) as u8,
                         (v.2 as f64 * fac) as u8)
                    }).collect()
            }).collect()
    }
}