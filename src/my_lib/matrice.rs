use std::f32::consts::PI;

use libm::{cosf, sinf};

#[derive(Debug)]
pub struct Matrice4<T> {
    pub value: [T; 16],
}

fn deg_to_rad(value: f32) -> f32 {
    value * (PI / 180.0)
}

impl<T: Default> Default for Matrice4<T> {
    fn default() -> Self {
        Self {
            value: std::array::from_fn(|_| T::default()),
        }
    }
}

impl<T: Default> Matrice4<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Default> Matrice4<T>
where
    T: From<f32> + std::ops::Mul<Output = T> + Copy + std::ops::Add<Output = T>,
{
    pub fn identity() -> Self {
        let mut value = [T::default(); 16];
        value[0] = T::from(1f32);
        value[5] = T::from(1f32);
        value[10] = T::from(1f32);
        value[15] = T::from(1f32);
        Self { value }
    }

    pub fn scale(&mut self, scale: &[T; 3]) -> Matrice4<T> {
        let mut res: Matrice4<T> = Matrice4::<T>::identity();
        res.value[0] = scale[0];
        res.value[5] = scale[1];
        res.value[10] = scale[2];
        self.multiply(res)
    }

    pub fn multiply(&mut self, m: Matrice4<T>) -> Matrice4<T> {
        let mut res: Matrice4<T> = Matrice4::default();
        for y in 0..4 {
            for i in 0..4 {
                res.value[i + (y * 4)] = sub_mult_proc(
                    [
                        self.value[0 + (y % 4 * 4)],
                        self.value[1 + (y % 4 * 4)],
                        self.value[2 + (y % 4) * 4],
                        self.value[3 + (y % 4) * 4],
                    ],
                    [
                        m.value[0 + (i % 4 * 4)],
                        m.value[1 + (i % 4 * 4)],
                        m.value[2 + (i % 4 * 4)],
                        m.value[3 + (i % 4 * 4)],
                    ],
                );
            }
        }
        res
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
        let cos_z = cosf(z);
        let sin_z = sinf(z);
        let cos_x = cosf(x);
        let sin_x = sinf(x);
        let cos_y = cosf(y);
        let sin_y = sinf(z);

        let mut rot_mat: Matrice4<T> = Matrice4::identity();
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
}

fn sub_mult_proc<T>(a: [T; 4], b: [T; 4]) -> T
where
    T: std::ops::Mul<Output = T> + Copy + std::ops::Add<Output = T>,
{
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}
