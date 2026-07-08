use crate::math::math_utils::radians;
use crate::math::matrix::Matrix4;
use crate::math::vec3::Vec3;

pub struct Camera {
    pub pos: Vec3,
    front: Vec3,
    up: Vec3,
    pub view: Matrix4<f32>,
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
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
            roll: 0.0,
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    /// Set Camera pos in <pos>
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.view = Matrix4::<f32>::look_at(self.pos.clone(), self.front.clone(), self.up.clone());
    }

    /// change Camera orientation
    pub fn rotate_cam(&mut self, angles: Vec3) {
        self.pitch = angles.x;
        self.yaw = angles.y;
        // let roll = angles.z;

        let p = Vec3 {
            x: f32::cos(radians(self.yaw)) * f32::cos(radians(self.pitch)),
            y: f32::sin(radians(self.pitch)),
            z: f32::sin(radians(self.yaw)) * f32::cos(radians(self.pitch)),
        };
        self.front = Vec3::normalized(p);

        self.view = Matrix4::<f32>::look_at(
            self.pos.clone(),
            self.pos.clone() + self.front.clone(),
            self.up.clone(),
        );
    }

    pub fn update_camera_pos(&mut self) {
        let p = Vec3 {
            x: f32::cos(radians(self.yaw)) * f32::cos(radians(self.pitch)),
            y: f32::sin(radians(self.pitch)),
            z: f32::sin(radians(self.yaw)) * f32::cos(radians(self.pitch)),
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

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
