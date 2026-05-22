use crate::my_lib::matrice::Matrix4;
use crate::my_lib::vec3::Vec3;

pub struct Camera {
    pos: Vec3,
    target: Vec3,
    up: Vec3,
}

impl Camera {
    /// init Camera at (0.0, 0.0, 0.0)
    pub fn new() -> Self {
        Camera {
            pos: Vec3::new(0.0, 0.0, 0.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
        }

        // Matrix4::<f32>::look_at(cam_pos, cam_target, up)
    }

    /// Set Camera pos in <pos>
    pub fn set_pos(&mut self, pos: Vec3) -> Matrix4<f32> {
        self.pos.x = f32::sin(pos.x * 10.0);
        self.pos.y = 0.0;
        self.pos.z = f32::cos(pos.z * 10.0);

        Matrix4::<f32>::look_at(self.pos.clone(), self.target.clone(), self.up.clone())
    }

    /// Add pos vector to cam pos
    pub fn move_cam(&mut self, pos: Vec3) -> Matrix4<f32> {
        self.pos.x += pos.x;
        self.pos.z += pos.z;
        let p = Vec3 {
            x: f32::sin(self.pos.x) * f32::cos(self.pos.y) * 2.0,
            y: 0.0,
            z: f32::cos(self.pos.x) * 2.0,
        };

        Matrix4::<f32>::look_at(p, self.target.clone(), self.up.clone())
    }
}
