use crate::my_lib::math_utils::radians;
use crate::my_lib::matrice::Matrix4;
use crate::my_lib::vec3::Vec3;

pub struct Camera {
    pos: Vec3,
    front: Vec3,
    up: Vec3,
    pub view: Matrix4<f32>,
}

impl Camera {
    /// init Camera at (0.0, 0.0, 0.0)
    pub fn new() -> Self {
        Camera {
            pos: Vec3::new(0.0, 0.0, 1.0),
            front: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            view: Matrix4::<f32>::look_at(
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
        }
    }

    /// Set Camera pos in <pos>
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.view = Matrix4::<f32>::look_at(self.pos.clone(), self.front.clone(), self.up.clone());
    }

    /// change Camera orientation
    pub fn rotate_cam(&mut self, angles: Vec3) {
        let mut pitch = angles.x;
        let yaw = angles.y;
        let roll = angles.z;

        let p = Vec3 {
            x: f32::cos(radians(yaw)) * f32::cos(radians(pitch)),
            y: f32::sin(radians(pitch)),
            z: f32::sin(radians(yaw)) * f32::cos(radians(pitch)),
        };
        self.front = Vec3::normalized(p);

        self.view = Matrix4::<f32>::look_at(
            self.pos.clone(),
            self.pos.clone() + self.front.clone(),
            self.up.clone(),
        );
    }

    pub fn move_cam(&mut self, pos: Vec3, angles: Vec3) {
        self.pos = pos;
        self.rotate_cam(angles);
    }

    // TODO zoom and camera translation 
}
