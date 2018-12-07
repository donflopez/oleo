use amethyst::core::nalgebra::{Vector2, Vector3};
use amethyst::core::Transform;
use amethyst::renderer::ScreenDimensions;

pub struct Algebra {
    camera: Vector3<f32>,
    screen: Vector2<f32>,
    mouse: Vector2<f32>,
}

impl Default for Algebra {
    fn default() -> Self {
        Algebra {
            camera: Vector3::new(0., 0., 100.),
            screen: Vector2::new(1024., 900.),
            mouse: Vector2::new(0., 0.),
        }
    }
}

impl Algebra {
    pub fn get_tile_pos(
        &self,
        object_z: f32,
        tile_dimension: f32,
        grid_start_coord: f32,
    ) -> Vector2<f32> {
        let m_pos = self.get_mouse_position(object_z);
        let x = (m_pos[0] / tile_dimension) - grid_start_coord;
        let y = (m_pos[1] / tile_dimension) - grid_start_coord;

        Vector2::new(x, y)
    }

    pub fn get_mouse_position(&self, object_z: f32) -> Vector2<f32> {
        let s_w = self.screen[0];
        let s_h = self.screen[1];
        let x_cam = self.camera[0];
        let y_cam = self.camera[1];
        let z_cam = self.camera[2];
        let x_mouse = self.mouse[0];
        let y_mouse = self.mouse[1];
        let distance = z_cam - object_z;

        let x = translate_to_global_position(distance, x_mouse, s_w) + x_cam;
        let y = y_cam - translate_to_global_position(distance, y_mouse, s_h);

        Vector2::new(x, y)
    }

    pub fn set_camera(&mut self, camera: &Transform) {
        self.camera = camera.translation().clone();
    }

    pub fn set_screen(&mut self, screen: &ScreenDimensions) {
        self.screen = Vector2::new(screen.width(), screen.height());
    }

    pub fn set_mouse(&mut self, x: f32, y: f32) {
        self.mouse = Vector2::new(x, y);
    }
}

fn translate_to_global_position(distance: f32, mouse_coord: f32, screen_dimension: f32) -> f32 {
    ((2. * distance * mouse_coord) / screen_dimension) - distance
}
