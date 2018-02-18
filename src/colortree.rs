use std;
use std::cmp::max;

#[derive(Debug)]
pub enum ColorTree {
    Octant {cube: Cube, num_leaf: u32, children: Vec<ColorTree>},
    Leaf {pos: Point, active: bool},
}

#[derive(Debug)]
pub struct Cube {
    tl: Point, 
    br: Point,
    size: u32
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub r: u32,
    pub g: u32,
    pub b: u32
}

impl ColorTree {
    pub fn new(bits: u32) -> ColorTree {
        ColorTree::mk_subtree(bits, Point {r:0,g:0,b:0})
    }

    fn mk_subtree(bits: u32, offset: Point) -> ColorTree {
        if bits > 0 {
            let size = 2u32.pow(bits as u32);
            let csize = size / 2;
            let mut children = Vec::new();
            children.push(ColorTree::mk_subtree(
                bits - 1, offset ));
            children.push(ColorTree::mk_subtree(
                bits - 1, Point {r: offset.r, g: offset.g, b: offset.b + csize } ));
            children.push(ColorTree::mk_subtree(
                bits - 1, Point {r: offset.r, g: offset.g + csize, b: offset.b } ));
            children.push(ColorTree::mk_subtree(
                bits - 1, Point {r: offset.r, g: offset.g + csize, b: offset.b + csize} ));
            children.push(ColorTree::mk_subtree(
                bits - 1, Point {r: offset.r + csize, g: offset.g, b: offset.b } ));
            children.push(ColorTree::mk_subtree(
                bits - 1, Point {r: offset.r + csize, g: offset.g, b: offset.b + csize } ));
            children.push(ColorTree::mk_subtree(
                bits - 1, Point {r: offset.r + csize, g: offset.g + csize, b: offset.b } ));
            children.push(ColorTree::mk_subtree(
                bits - 1, Point {r: offset.r + csize, g: offset.g + csize, b: offset.b + csize } ));
            ColorTree::Octant{
                cube: Cube { tl: offset, br: offset + (size - 1), size: size },
                num_leaf: 2u32.pow(3*bits),
                children,
            }            
        } else {
            ColorTree::Leaf {pos: offset, active: true}
        }
    }

    pub fn nearest(&self, from: Point) -> Option<(Point, u32)> {
        self.nearest_radius(from, std::u32::MAX)
    }

    fn nearest_radius(&self, from: Point, radius: u32) -> Option<(Point, u32)> {
        match *self {
            ColorTree::Leaf{ ref pos, active} => {
                match active {
                    true => {
                        let dist = from.dist(pos);
                        if radius > dist {
                            Some((*pos, dist))
                        } else { None }
                    }
                    false => None
                }
            },
            ColorTree::Octant { num_leaf, ref children, .. } => {
                if num_leaf == 0 { return None; }
                let mut best = radius;
                let mut bestpos = None;
                let mut distances: Vec<_> = children.into_iter()
                    .map(|c| (c.dist(from), c))
                    .filter_map(|(dc, c)| {
                        match dc {
                            Some(d) => Some((d, c)),
                            None => None
                        }
                    })
                    .collect();
                distances.sort_by(|&(di, ..), &(dj, ..)| di.cmp(&dj));
                for (dc, c) in distances {
                    if dc >= best { continue; }
                    if let Some((pos, dist)) = c.nearest_radius(from, best) {
                        best = dist;
                        bestpos = Some((pos, dist));
                    }
                }
                bestpos
            }
        }
    }

    pub fn remove(&mut self, point: Point) -> bool {
        match *self {
            ColorTree::Leaf { ref mut active, .. } => {
                if *active == true {
                    *active = false;
                    true
                } else {
                    false
                }
            },
            ColorTree::Octant { ref cube, ref mut num_leaf, ref mut children } => {
                if *num_leaf == 0 {
                    return false;
                }
                let mut idx = 0;
                if point.r >= cube.tl.r + (cube.size / 2) { idx += 4; }
                if point.g >= cube.tl.g + (cube.size / 2) { idx += 2; }
                if point.b >= cube.tl.b + (cube.size / 2) { idx += 1; }
                if children[idx].remove(point) == true {
                    *num_leaf -= 1;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn dist(&self, from: Point) -> Option<u32> {
        match *self {
            ColorTree::Leaf { ref pos, active } => {
                if active {
                    Some(pos.dist(&from))
                } else { None }
            },
            ColorTree::Octant { ref cube, num_leaf, .. } => {
                if num_leaf > 0 {
                    Some(cube.dist(&from))
                } else { None }
            }
        }
    }
}

impl Point {
    fn dist(&self, &other: &Point) -> u32 {
        let dr = self.r as i64 - other.r as i64;
        let dg = self.g as i64 - other.g as i64;
        let db = self.b as i64 - other.b as i64;
        (dr.pow(2) + dg.pow(2) + db.pow(2)) as u32
    }
}

impl Cube {
    fn dist(&self, &from: &Point) -> u32 {
        let dr = max(max(0, self.tl.r as i64 - from.r as i64), max(0, from.r as i64 - self.br.r as i64));
        let dg = max(max(0, self.tl.g as i64 - from.g as i64), max(0, from.g as i64 - self.br.g as i64));
        let db = max(max(0, self.tl.b as i64 - from.b as i64), max(0, from.b as i64 - self.br.b as i64));
        (dr.pow(2) + dg.pow(2) + db.pow(2)) as u32
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point{ r: self.r + rhs.r,
               g: self.g + rhs.g,
               b: self.b + rhs.b }
    }
}

impl std::ops::Add<u32> for Point {
    type Output = Point;

    fn add(self, rhs: u32) -> Point {
        Point{ r: self.r + rhs,
               g: self.g + rhs,
               b: self.b + rhs }
    }
}