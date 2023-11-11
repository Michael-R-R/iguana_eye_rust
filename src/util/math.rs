use cgmath::{Vector2, Vector3, Vector4, Matrix4, SquareMatrix, InnerSpace};

pub fn to_world_pos(
    w: f32, h: f32,
    cursor_pos: &Vector2<f32>,
    camera_pos: &Vector3<f32>,
    projection: &Matrix4<f32>) 
    -> Option<Vector3<f32>> {

    // Normal device coordinates
    let x = (2.0 * cursor_pos.x) / w - 1.0;
    let y = 1.0 - (2.0 * cursor_pos.y) / h;
    let ray_ndc = cgmath::Vector2::new(x, y);

    // 4D homogenous clip coordinates
    let ray_clip = cgmath::Vector4::new(ray_ndc.x, ray_ndc.y, -1.0, 1.0);

    // World position
    let prj_inverse = match projection.invert() {
        Some(val) => val,
        None => return None
    };
    let mut ray_eye = prj_inverse * ray_clip;
    ray_eye = Vector4::new(ray_eye.x, ray_eye.y, -1.0, 1.0);

    return Some(Vector3::new(ray_eye.x + camera_pos.x, 
        ray_eye.y + camera_pos.y, 
        ray_eye.z + camera_pos.z))
}

pub fn to_world_ray(
    w: f32, h: f32,
    cursor_pos: &Vector2<f32>,
    view: &Matrix4<f32>,
    projection: &Matrix4<f32>) 
    -> Option<Vector3<f32>> {

    // Normal device coordinates
    let x = (2.0 * cursor_pos.x) / w - 1.0;
    let y = 1.0 - (2.0 * cursor_pos.y) / h;
    let ray_ndc = cgmath::Vector2::new(x, y);

    // 4D homogenous clip coordinates
    let ray_clip = cgmath::Vector4::new(ray_ndc.x, ray_ndc.y, -1.0, 1.0);

    // World position
    let prj_inverse = match projection.invert() {
        Some(val) => val,
        None => return None
    };
    let mut ray_eye = prj_inverse * ray_clip;
    ray_eye = Vector4::new(ray_eye.x, ray_eye.y, -1.0, 1.0);

    // World ray
    let view_inverse = match view.invert() {
        Some(val) => val,
        None => return None
    };
    let inverse_matrix = view_inverse * ray_eye;
    let mut ray_world = Vector3::new(inverse_matrix.x, inverse_matrix.y, inverse_matrix.z);
    ray_world = ray_world.normalize();

    return Some(ray_world);
}