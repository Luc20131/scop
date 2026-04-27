use libm::{cos, sin};
use std::num;

#[derive(Debug)]
pub struct Matrice4<T> {
    pub value: [T; 16],
}

// impl get for MyMatrx {}
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

struct CosSin {
    cos: f64,
    sin: f64,
}

impl CosSin {
    pub fn new(value: f64) -> Self {
        Self {
            cos: cos(value),
            sin: sin(value),
        }
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

    pub fn translate(&mut self, translation: &[T; 3]) {
        self.value[12] = self.value[12] + translation[0];
        self.value[13] = self.value[13] + translation[1];
        self.value[14] = self.value[14] + translation[2];
    }

    pub fn rotate(&mut self, x: f64, y: f64, z: f64)
    where
        T: From<f64> + std::ops::Mul<Output = T> + Copy + std::ops::Add<Output = T>,
    {
        if x != 0.0 {
            let mut rot_x: Matrice4<T> = Matrice4::identity();
            let cos_sin_x = CosSin::new(x);
            rot_x.value[0] = T::from(cos_sin_x.cos);
            rot_x.value[1] = T::from(-cos_sin_x.sin);
            rot_x.value[4] = T::from(cos_sin_x.sin);
            rot_x.value[5] = T::from(cos_sin_x.cos);
            self.value = self.multiply(rot_x).value;
        }
        if y != 0.0 {
            let mut rot_y: Matrice4<T> = Matrice4::identity();
            let cos_sin_y = CosSin::new(y);
            rot_y.value[0] = T::from(cos_sin_y.cos);
            rot_y.value[2] = T::from(cos_sin_y.sin);
            rot_y.value[8] = T::from(-cos_sin_y.sin);
            rot_y.value[10] = T::from(cos_sin_y.cos);
            self.value = self.multiply(rot_y).value;
        }
        if z != 0.0 {
            let mut rot_z: Matrice4<T> = Matrice4::identity();
            let cos_sin_z = CosSin::new(z);
            rot_z.value[5] = T::from(cos_sin_z.cos);
            rot_z.value[6] = T::from(-cos_sin_z.sin);
            rot_z.value[9] = T::from(cos_sin_z.sin);
            rot_z.value[10] = T::from(cos_sin_z.cos);
            self.value = self.multiply(rot_z).value;
        }
    }
}

fn sub_mult_proc<T>(a: [T; 4], b: [T; 4]) -> T
where
    T: std::ops::Mul<Output = T> + Copy + std::ops::Add<Output = T>,
{
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}
