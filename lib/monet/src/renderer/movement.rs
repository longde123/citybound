
pub use descartes::{N, P3, P2, V3, V4, M4, Iso3, Persp3, ToHomogeneous, Norm, Into2d, Into3d,
                    WithUniqueOrthogonal, Inverse, Rotate};
use kay::{Recipient, Fate};

use ::{Renderer, Eye};

#[derive(Copy, Clone)]
pub enum Movement {
    Shift(V3),
    Zoom(N),
    Rotate(N),
}

#[derive(Copy, Clone)]
pub struct MoveEye {
    pub scene_id: usize,
    pub movement: Movement,
}
#[derive(Copy, Clone)]
pub struct EyeMoved {
    pub eye: Eye,
    pub movement: Movement,
}

impl Recipient<MoveEye> for Renderer {
    fn receive(&mut self, msg: &MoveEye) -> Fate {
        match *msg {
            MoveEye { scene_id, movement } => {
                match movement {
                    Movement::Shift(delta) => self.movement_shift(scene_id, delta),
                    Movement::Zoom(delta) => self.movement_zoom(scene_id, delta),
                    Movement::Rotate(delta) => self.movement_rotate(scene_id, delta),
                }

                for &id in &self.scenes[scene_id].eye_listeners {
                    id <<
                    EyeMoved {
                        eye: self.scenes[scene_id].eye,
                        movement: movement,
                    };
                }
                Fate::Live
            }
        }
    }
}

impl Renderer {
    fn movement_shift(&mut self, scene_id: usize, delta: V3) {
        let eye = &mut self.scenes[scene_id].eye;
        let eye_direction_2d = (eye.target - eye.position).into_2d().normalize();
        let absolute_delta = delta.x * eye_direction_2d.into_3d() +
                             delta.y * eye_direction_2d.orthogonal().into_3d() +
                             V3::new(0.0, 0.0, delta.z);

        eye.position += absolute_delta * (eye.position.z / 100.0);
        eye.target += absolute_delta * (eye.position.z / 100.0);
    }

    fn movement_zoom(&mut self, scene_id: usize, delta: N) {
        let eye = &mut self.scenes[scene_id].eye;
        let eye_direction = (eye.target - eye.position).normalize();
        if (eye.target - eye.position).norm() > 30.0 || delta < 0.0 {
            eye.position += eye_direction * delta * (eye.position.z / 100.0);
        }
    }

    fn movement_rotate(&mut self, scene_id: usize, delta: N) {
        let eye = &mut self.scenes[scene_id].eye;
        let relative_eye_position = eye.position - eye.target;
        let iso = Iso3::new(V3::new(0.0, 0.0, 0.0), V3::new(0.0, 0.0, delta));
        let rotated_relative_eye_position = iso.rotate(&relative_eye_position);

        eye.position = eye.target + rotated_relative_eye_position;
    }
}