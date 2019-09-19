// use std::fs::File;
// use std::string::String;
use std::ops::Sub;
use std::ops::Add;
use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn origin() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn dot(self, other: Point) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    fn magnitude(self) -> f64 {
        return (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        // TODO Add small error correction value
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[derive(Copy, Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

struct Ray {
    e: Point,
    d: Point,
}

// struct Plane {
//     n: Point,
// }

struct Sphere {
    c: Point,
    r: f64,
}

fn print_row(row: &[Pixel]) {
    for (_x, pixel) in row.iter().enumerate() {
        print!("  {} {} {}", pixel.r, pixel.g, pixel.b);
    }
}

fn intersects(ray: &Ray, sphere: &Sphere, t: &mut f64) -> bool {
    let e = ray.e;
    let d = ray.d;
    let c = sphere.c;
    let r = sphere.r;

    let A = d.dot(d);
    let B = (d*2.0).dot(e-c);
    let C = (e-c).dot(e-c) - r.powf(2.0);

    let descriminant = B.powf(2.0) - 4.0 * A * C;

    let epsilon = 0.1;

    if descriminant < epsilon {
        return false;
    } else if descriminant > epsilon {
        // Find smallest t value -- that's the closest hit
        let t1 = (-B + (B.powf(2.0) - 4.0 * A * C).sqrt()) / (2.0 * A);
        let t2 = (-B - (B.powf(2.0) - 4.0 * A * C).sqrt()) / (2.0 * A);
        if t1.abs() < t2.abs() {
            *t = t1;
        } else {
            *t = t2;
        }
        return true;
    } else { // nearly equal -- one solution
        // NOTE But there's no guarantee of which side this ray is colliding with the sphere from
        // -- it could be from within the sphere
        *t = -B / (2.0 * A);
        return true;
    }

    // TODO make sure that t is a valid value (within the bounds)
}

fn main() {
    print!("P3");

    const HEIGHT: usize = 300; // == num rows
    const WIDTH: usize = 300;  // == num cols
    print!("  {} {}", WIDTH, HEIGHT);

    print!("  255");

    // Create the rows
    let mut rows = vec![vec![Pixel { r: 0, g: 0, b: 0 }; WIDTH]; HEIGHT];
    let sphere = Sphere { c: Point { x: 0.0, y: 0.0, z: -400.0 }, r: 100.0 };

    // Do something to the rows
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // rows[y][x] = Pixel { r: (255 * y/WIDTH) as u8, g: (255 * x/WIDTH) as u8, b: (255 * x/WIDTH) as u8 };
            let ray = Ray {
                e: Point {
                    x: ((x as f64) - (WIDTH as f64)/2.0),
                    y: ((y as f64) - (HEIGHT as f64)/2.0),
                    z: 0.0
                },
                d: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0
                },
            };
            let mut t: f64 = 0.0;
            if intersects(&ray, &sphere, &mut t) {
                rows[y][x] = Pixel {
                    r: (255.0 * t.sqrt()/sphere.c.z.abs().sqrt()) as u8,
                    g: (155.0 * t.sqrt()/sphere.c.z.abs().sqrt()) as u8,
                    b: (155.0 * t.sqrt()/sphere.c.z.abs().sqrt()) as u8,
                };
            } else {
                rows[y][x] = Pixel { r: 0, g: 0, b: 0 };
            }
        }
    }

    // Print out the rows
    for (_y, row) in rows.iter().enumerate() {
        print_row(&row);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from other (for mod tests) scope.
    use super::*;

    #[test]
    fn point_subtraction() {
        let a = Point {
            x: 1.0,
            y: 2.0,
            z: 4.0,
        };
        let b = Point {
            x: 2.0,
            y: 0.0,
            z: 2.0,
        };
        let actual = a - b;
        let correct = Point {
            x: -1.0,
            y:  2.0,
            z:  2.0,
        };
        assert_eq!(actual, correct);
    }

    #[test]
    fn point_equality() {
        let a = Point {
            x: 1.0,
            y: 2.0,
            z: 4.0,
        };
        let b = Point {
            x: 1.0,
            y: 2.0,
            z: 4.0,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn simple_intersection() {
        let sphere = Sphere {
            c: Point {
                x:  0.0,
                y:  0.0,
                z: -8.0,
            },
            r: 2.0,
        };
        let ray = Ray {
            e: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            d: Point {
                x:  0.0,
                y:  0.0,
                z: -1.0,
            },
        };
        assert_eq!(intersects(&ray, &sphere), true);
    }
}
