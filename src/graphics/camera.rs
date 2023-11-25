use std::io;

use wgpu::{Buffer, BindGroup, Device, Queue, BindGroupLayout};
use serde::{Serialize, Deserialize};
use cgmath::{Point3, Vector3, Matrix4};

use super::uniform_buffer::CameraUBuffer;

const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[derive(Serialize, Deserialize)]
pub struct Camera {
    projection: Matrix4<f32>,
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    width: f32,
    height: f32,
    znear: f32,
    zfar: f32,

    #[serde(skip)]
    pub camera_buffer: Option<CameraUBuffer>,

    #[serde(skip)]
    pub u_buffer: Option<Buffer>,

    #[serde(skip)]
    pub bind_group_layout: Option<BindGroupLayout>,

    #[serde(skip)]
    pub bind_group: Option<BindGroup>,
}

impl Camera {
    pub fn new(device: &Device, width: f32, height: f32) -> Self {
        
        let (camera_buffer, u_buffer) = CameraUBuffer::new(device);
        let bind_group_layout = CameraUBuffer::layout(device);
        let bind_group = CameraUBuffer::bind_group(device, &bind_group_layout, &u_buffer);
        let znear = -1000.0;
        let zfar = 1000.0;
        
        Self {
            projection: Camera::projection(width, height, znear, zfar),
            eye: Point3::new(0.0, 0.0, -1.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::unit_y(),
            width,
            height,
            znear,
            zfar,
            camera_buffer: Some(camera_buffer),
            u_buffer: Some(u_buffer),
            bind_group_layout: Some(bind_group_layout),
            bind_group: Some(bind_group),
        }
    }

    pub fn vp(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);

        return OPENGL_TO_WGPU_MATRIX * self.projection * view;
    }

    pub fn modify_buffer(&mut self, queue: &Queue) -> Result<(), io::Error> {
        let vp = self.vp();

        match &mut self.camera_buffer {
            Some(c_buffer) => {
                match &mut self.u_buffer {
                    Some(u_buffer) => {
                        c_buffer.view_projection = vp.into();
                        queue.write_buffer(&u_buffer, 0, bytemuck::cast_slice(&[*c_buffer]));
                        Ok(())
                    },
                    None => {
                        Err(io::Error::new(io::ErrorKind::NotFound, "TODO"))
                    }
                }
            },
            None => {
                Err(io::Error::new(io::ErrorKind::NotFound, "TODO"))
            }
        }
    }

    pub fn handle_resize(&mut self, w: f32, h: f32) {
        self.projection = Camera::projection(w, h, self.znear, self.zfar);
    }

    fn projection(w: f32, h: f32, near: f32, far: f32) -> Matrix4<f32> {
        cgmath::ortho(
            w / 2.0,
            -w / 2.0,
            -h / 2.0,
            h / 2.0,
            near,
            far
        )
    }
}