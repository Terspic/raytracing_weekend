use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn squared_norm(self) -> f64 {
        self.dot(self)
    }

    pub fn norm(self) -> f64 {
        self.squared_norm().sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    pub fn normalized(&mut self) {
        *self = self.normalize();
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn clamp(self, min: f64, max: f64) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
        }
    }

    pub fn reflect(&self, n: Self) -> Self {
        *self - 2.0 * n * self.dot(n)
    }

    pub fn refract(self, n: Self, eta1: f64, eta2: f64) -> Self {
        let cos_theta = -self.dot(n).min(1.0);
        let perp = (eta1 / eta2) * (self + cos_theta * n);
        let parallel = -(1.0 - perp.squared_norm()).abs().sqrt() * n;
        perp + parallel
    }

    pub fn random(r: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(r.clone()),
            y: rng.gen_range(r.clone()),
            z: rng.gen_range(r.clone()),
        }
    }

    pub fn random_unit_sphere() -> Self {
        Self::random(-1.0..1.0).normalize()
    }

    pub fn random_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let v = vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if v.norm() <= 1.0 {
                return v
            }
        }
    }

    pub fn is_near(&self, v: Self) -> bool {
        (self.x - v.x).abs() < f64::EPSILON
            && (self.y - v.y).abs() < f64::EPSILON
            && (self.z - v.z).abs() < f64::EPSILON
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.mul(self)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(o: Point3, d: Vec3) -> Self {
        Self { origin: o, dir: d }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }
}

pub fn ray(origin: Point3, dir: Vec3) -> Ray {
    Ray { origin, dir }
}

pub fn is_campled(v: f64, min: f64, max: f64) -> bool {
    v >= min && v <= max
}
