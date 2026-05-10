use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Matrice4<T> {
    pub value: [T; 16],
}

// impl<T: Default> std::ops::Index<usize> for Matrice4<T>
// where
//     T: ?Sized + Clone,
// {
//     type Output = T;
//     fn index(&self, i: usize) -> T {
//         self.value[i]
//     }
// }

pub fn deg_to_rad(value: f32) -> f32 {
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
    T: From<f32>
        + std::ops::Mul<Output = T>
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::MulAssign,
{
    pub fn identity() -> Self {
        let mut value = [T::default(); 16];
        value[0] = T::from(1f32);
        value[5] = T::from(1f32);
        value[10] = T::from(1f32);
        value[15] = T::from(1f32);
        Self { value }
    }

    pub fn scale(&mut self, scale: T) -> Matrice4<T> {
        let mut res: Matrice4<T> = Matrice4::<T>::identity();
        res.value[0] *= scale;
        res.value[5] *= scale;
        res.value[10] *= scale;
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

        // let mut rot_x: Matrice4<T> = Matrice4::identity();

        // let mut rot_y: Matrice4<T> = Matrice4::identity();

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
        // let mut res = Matrice4::identity();
        // if x != 0.0 {
        //     rot_x.value[5] = T::from(cos_x);
        //     rot_x.value[6] = T::from(-sin_x);
        //     rot_x.value[9] = T::from(sin_x);
        //     rot_x.value[10] = T::from(cos_x);
        // }
        // if y != 0.0 {
        //     rot_y.value[0] = T::from(cos_y);
        //     rot_y.value[2] = T::from(-sin_y);
        //     rot_y.value[8] = T::from(sin_y);
        //     rot_y.value[10] = T::from(cos_y);
        // }
        // if z != 0.0 {
        //     rot_z.value[0] = T::from(cos_z);
        //     rot_z.value[1] = T::from(-sin_z);
        //     rot_z.value[4] = T::from(sin_z);
        //     rot_z.value[5] = T::from(cos_z);
        // }
        // res.value = res.multiply(rot_x).value;
        // res.value = rot_y.multiply(res.clone()).value;
        // res.value = res.multiply(rot_z).value;
        self.value = self.multiply(rot_mat).value;
    }

    pub fn multiply(&mut self, m: Matrice4<T>) -> Matrice4<T> {
        let mut res: Matrice4<T> = Matrice4::default();
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
