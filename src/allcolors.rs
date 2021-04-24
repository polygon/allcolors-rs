use colortree;
use imagemap;
use colortree::Point;

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
        } else { () }
    }

    pub fn done(&self) -> bool {
        self.image.done()
    }

    fn average(&self, x: usize, y: usize) -> (u8, u8, u8) {
        let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);
        let mut num:f64 = 0.;
        for i in -1..2 {
            for j in -1..2 {
                let cx = x as i32 + i;
                let cy = y as i32 + j;
                if let Some((nr, ng, nb)) = self.image.get_color(cx, cy) {
                    /*r += nr as f64;
                    g += ng as f64;
                    b += nb as f64;*/
                    r += (nr as f64).powf(2.);
                    g += (ng as f64).powf(2.);
                    b += (nb as f64).powf(2.);
                    num += 1.;
                }
            }
        }

        ((r/num).sqrt() as u8, (g/num).sqrt() as u8, (b/num).sqrt() as u8)
        //((r/num) as u8, (g/num) as u8, (b/num) as u8)
    }

    pub fn to_image(&self) -> Vec<Vec<(u8, u8, u8)>> {
        let mut v = self.image.to_image((0, 0, 0));
        let fac: f64 = 256. / 2f64.powf(self.bits as f64);
        for x in 0..self.width {
            for y in 0..self.height {
                v[x][y] = ((v[x][y].0 as f64 * fac) as u8,
                           (v[x][y].1 as f64 * fac) as u8,
                           (v[x][y].2 as f64 * fac) as u8);
            }
        }
        v
    }
}
