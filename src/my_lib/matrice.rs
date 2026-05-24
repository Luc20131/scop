use cgmath::num_traits::ops::mul_add;

use crate::my_lib::vec3::Vec3;
use std::{
    f32::{self, consts::PI},
    ops::Mul,
    process::Output,
};

#[derive(Debug, Clone)]
pub struct Matrix4<T> {
    pub value: [T; 16],
}

impl<T: Default> Default for Matrix4<T> {
    fn default() -> Self {
        Self {
            value: std::array::from_fn(|_| T::default()),
        }
    }
}

// impl<T> Mul<Output = Vec3> for Matrix4<f32>
// where T: std::ops::Mul<Output = Matrix4<f32>, {
//     fn mul(self, rhs: Vec3) -> Matrix4<f32> {
//         Matrix4
//     }
// }

// Matrices Preset
impl<T: Default> Matrix4<T>
where
    T: From<f32> + Copy,
{
    pub fn identity() -> Self {
        let mut value = [T::default(); 16];
        value[0] = T::from(1f32);
        value[5] = T::from(1f32);
        value[10] = T::from(1f32);
        value[15] = T::from(1f32);
        Self { value }
    }

    pub fn look_at(cam_pos: Vec3, target_pos: Vec3, up: Vec3) -> Matrix4<f32> {
        let cam_dir = Vec3::normalize(cam_pos.clone() - target_pos);
        let cam_right = Vec3::normalize(Vec3::cross(up, cam_dir.clone()));
        let cam_up = Vec3::cross(cam_dir.clone(), cam_right.clone());
        let mut cam = Matrix4 {
            value: [
                cam_right.x,
                cam_right.y,
                cam_right.z,
                0.0f32,
                cam_up.x,
                cam_up.y,
                cam_up.z,
                0.0f32,
                cam_dir.x,
                cam_dir.y,
                cam_dir.z,
                0.0f32,
                0.0f32,
                0.0f32,
                0.0f32,
                1.0f32,
            ],
        };
        let mut pos_matrix = Matrix4::identity();
        pos_matrix.value[12] = -cam_pos.x;
        pos_matrix.value[13] = -cam_pos.y;
        pos_matrix.value[14] = -cam_pos.z;

        cam.value = cam.multiply(pos_matrix).value;
        cam
    }

    /// Create perspective matrix,
    /// fov need to be in radians
    pub fn perspective(aspect: f32, fov: f32, near: f32, far: f32) -> Matrix4<f32> {
        Matrix4 {
            value: [
                (1.0 / (aspect * f32::tan(fov / 2.0))),
                0.0f32,
                0.0f32,
                0.0f32,
                0.0f32,
                (1.0 / (f32::tan(fov / 2.0))),
                0.0f32,
                0.0f32,
                0.0f32,
                0.0f32,
                ((far + near) / (near - far)),
                ((2.0 * far * near) / (near - far)),
                0.0f32,
                0.0f32,
                1.0f32,
                0.0f32,
            ],
        }
    }
}

// Matrices Methods
impl<T: Default> Matrix4<T>
where
    T: From<f32>
        + std::ops::Mul<Output = T>
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::MulAssign,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scale(&mut self, scale: T) -> Matrix4<T> {
        let mut res: Matrix4<T> = Matrix4::<T>::identity();
        res.value[0] = scale;
        res.value[5] = scale;
        res.value[10] = scale;
        self.multiply(res)
    }

    pub fn translate(&mut self, x: T, y: T, z: T) {
        self.value[3] = self.value[3] + x;
        self.value[7] = self.value[7] + y;
        self.value[11] = self.value[11] + z;
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32)
    where
        T: From<f32> + std::ops::Mul<Output = T> + Copy + std::ops::Add<Output = T>,
    {
        let cos_z = f32::cos(z);
        let sin_z = f32::sin(z);
        let cos_x = f32::cos(x);
        let sin_x = f32::sin(x);
        let cos_y = f32::cos(y);
        let sin_y = f32::sin(y);
        let mut rot_mat: Matrix4<T> = Matrix4::identity();
        rot_mat.value[0] = T::from(cos_z * cos_y);
        rot_mat.value[1] = T::from((cos_z * sin_y * sin_x) - (sin_z * cos_x));
        rot_mat.value[2] = T::from((cos_z * sin_y * cos_x) + (sin_z * sin_x));
        rot_mat.value[4] = T::from(sin_z * cos_y);
        rot_mat.value[5] = T::from((sin_z * sin_y * sin_x) + (cos_z * cos_x));
        rot_mat.value[6] = T::from((sin_z * sin_y * cos_x) - (cos_z * sin_x));
        rot_mat.value[8] = T::from(-sin_y);
        rot_mat.value[9] = T::from(cos_y * sin_x);
        rot_mat.value[10] = T::from(cos_y * cos_x);
        self.value = self.multiply(rot_mat).value;
    }

    pub fn multiply(&mut self, m: Matrix4<T>) -> Matrix4<T> {
        let mut res: Matrix4<T> = Matrix4::default();
        for y in 0..4 {
            for i in 0..4 {
                res.value[i + (y * 4)] = sub_mult_proc(
                    [
                        self.value[0 + i],
                        self.value[4 + i],
                        self.value[8 + i],
                        self.value[12 + i],
                    ],
                    [
                        m.value[0 + y * 4],
                        m.value[1 + y * 4],
                        m.value[2 + y * 4],
                        m.value[3 + y * 4],
                    ],
                );
            }
        }
        res
    }
}

fn sub_mult_proc<T>(a: [T; 4], b: [T; 4]) -> T
where
    T: std::ops::Mul<Output = T> + Copy + std::ops::Add<Output = T>,
{
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}

pub fn deg_to_rad(value: f32) -> f32 {
    value * (PI / 180.0)
}
