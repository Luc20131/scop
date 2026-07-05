use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3 {
            x: (self.x),
            y: (self.y),
            z: (self.z),
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        if self.x.eq(&other.x) && self.y.eq(&other.y) && self.z.eq(&other.z) {
            return true;
        }
        false
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// return the Vec3 argmument normalized without change it
    pub fn normalized(v: Vec3) -> Vec3 {
        let v_norm = f32::sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
        Vec3 {
            x: v.x / v_norm,
            y: v.y / v_norm,
            z: v.z / v_norm,
        }
    }

    /// Normalize the concerned Vec3
    pub fn normalize(&mut self) {
        let v_norm = f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        self.x /= v_norm;
        self.y /= v_norm;
        self.z /= v_norm;
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            x: v1.y * v2.z - v2.y * v1.z,
            y: v1.z * v2.x - v2.z * v1.x,
            z: v1.x * v2.y - v2.x * v1.y,
        }
    }

    pub fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
