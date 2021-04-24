use rand;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
enum Pixel {
    Free,
    Marked,
    Placed (u8, u8, u8),
}

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<Vec<Pixel>>,
    open: Vec<(usize, usize)>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {width, height, data: vec![vec![Pixel::Free; height]; width], open: Vec::new()}
    }

    pub fn place(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        if (x >= self.width) || (y >= self.height) {
            panic!("Tried to place out of bounds");
        }

        let (r, g, b) = color;
        self.data[x][y] = Pixel::Placed (r, g, b);

        for i in -1..2 {
            for j in -1..2 {
                match self.get(x as i32 + i, y as i32 + j) {
                    Some(&Pixel::Free) => {
                        let px = (x as i32 + i) as usize;
                        let py = (y as i32 + j) as usize;
                        self.open.push((px, py));
                        self.data[px][py] = Pixel::Marked;
                    },
                    _ => ()
                }
            }
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&Pixel> {
        if !((0 <= x) && (x < self.width as i32)) ||
           !((0 <= y) && (y < self.height as i32)) {
            return None
        }
        Some (&self.data[x as usize][y as usize])
    }

    pub fn get_color(&self, x: i32, y: i32) -> Option<(u8, u8, u8)> {
        match self.get(x, y) {
            Some(&Pixel::Placed(r, g, b)) => Some((r, g, b)),
            _ => None
        }
    }

    pub fn get_open(&mut self) -> Option<(usize, usize)> {
        if self.open.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let i:usize = rng.gen_range(0..self.open.len());
        let (x, y) = self.open.swap_remove(i);
        Some((x, y))
    }

    pub fn done(&self) -> bool {
        self.open.is_empty()
    }

    pub fn to_image(&self, default: (u8, u8, u8)) -> Vec<Vec<(u8, u8, u8)>> {
        let mut out = vec![vec![default; self.height]; self.width];
        for i in 0..self.width {
            for j in 0..self.height {
                match self.data[i][j] {
                    Pixel::Placed (r, g, b) => {
                        out[i][j] = (r, g, b);
                    }
                    _ => ()
                }
            }
        }
        out
    }

    pub fn num_open(&self) -> usize {
        self.open.len()
    }
}
