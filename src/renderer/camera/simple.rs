use gl_matrix::common::to_radian;
use gl_matrix::{mat4};

#[derive(Debug)]
struct CameraView {
    position: [f32; 3],
    target: [f32; 3],
    up: [f32; 3],
}

#[derive(Debug)]
pub(in super::super) struct SimpleCamera {
    view: CameraView,
    projection: [f32; 16],
}

impl SimpleCamera {
    pub(in super::super) fn new(
        position: [f32; 3],
        target: [f32; 3],
        up: [f32; 3],
        aspect: f32,
    ) -> SimpleCamera {
        let mut projection = [0.; 16];
        mat4::perspective(&mut projection, to_radian(60.), aspect, 0.1, Some(100.));

        let view = CameraView {
            position,
            target,
            up,
        };
        SimpleCamera { view, projection }
    }

    pub(in super::super) fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.view.position[0] = self.view.position[0] + x;
        self.view.position[1] = self.view.position[1] + y;
        self.view.position[2] = self.view.position[2] + z;
        self.view.target[0] = self.view.target[0] + x;
        self.view.target[1] = self.view.target[1] + y;
        self.view.target[2] = self.view.target[2] + z;
    }


    pub(in super::super) fn view(&self) -> [f32; 16] {
        let mut view = [0.; 16];
        mat4::look_at(
            &mut view,
            &self.view.position,
            &self.view.target,
            &self.view.up,
        );

        view
    }

    pub(in super::super) fn projection(&self) -> [f32; 16] {
        self.projection
    }
}
