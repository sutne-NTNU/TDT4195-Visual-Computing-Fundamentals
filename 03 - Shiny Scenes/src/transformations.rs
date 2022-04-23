extern crate nalgebra_glm as glm;
use glm::{perspective, rotation, scale, translation, vec3, Mat4};

use crate::camera::Camera;
use crate::scene_graph::SceneNode;
use crate::window;

/// Use the cameras position and rotation to create a single transformation matrix
/// for the scene.
pub unsafe fn view_projection_matrix(camera: &Camera) -> Mat4 {
    let identity: Mat4 = glm::identity();

    // Translation of scene based on camera position
    let camera_translation: Mat4 = translation(&(-camera.position));

    // Rotation based on cameras angle
    let camera_pitch_rotation: Mat4 = rotation(camera.pitch() as f32, &vec3(1.0, 0.0, 0.0));
    let camera_yaw_rotation: Mat4 = rotation(camera.yaw() as f32, &vec3(0.0, 1.0, 0.0));
    let camera_rotation: Mat4 = camera_pitch_rotation * camera_yaw_rotation;

    // Translation to enable perspective
    let ratio = window::WINDOW_WIDTH as f32 / window::WINDOW_HEIGHT as f32;
    let perspective: Mat4 = perspective(ratio, 1.0, 0.1, 1500.0);
    let perspective_translation: Mat4 = translation(&vec3(0.0, 0.0, -2.0));

    // Merge and return resulting matrix
    let mut view_projection_matrix = identity;
    view_projection_matrix = perspective_translation * view_projection_matrix;
    view_projection_matrix = camera_translation * view_projection_matrix;
    view_projection_matrix = camera_rotation * view_projection_matrix;
    view_projection_matrix = perspective * view_projection_matrix;

    // Return finished matrix
    return view_projection_matrix;
}

/// Recursivvely apply transformations to node and its children
pub unsafe fn update_node_transformations(node: &mut SceneNode, parent_transformation: &glm::Mat4) {
    // Scaling
    let scale: Mat4 = scale(&glm::identity(), &node.scale);
    // Rotation
    let pitch: Mat4 = rotation(node.rotation.x, &vec3(1.0, 0.0, 0.0));
    let yaw: Mat4 = rotation(node.rotation.y, &vec3(0.0, 1.0, 0.0));
    let roll: Mat4 = rotation(node.rotation.z, &vec3(0.0, 0.0, 1.0));
    let rotation = yaw * roll * pitch;
    // Translation
    let reference_translation = translation(&node.reference_point);
    let negative_reference_translation = translation(&-node.reference_point);
    let translation: Mat4 = translation(&node.position);

    // Construct the new matrix
    let mut my_matrix: Mat4 = glm::identity();
    my_matrix = negative_reference_translation * my_matrix;
    my_matrix = scale * my_matrix;
    my_matrix = rotation * my_matrix;
    my_matrix = reference_translation * my_matrix;
    my_matrix = translation * my_matrix;

    //Update the current transformation
    node.current_transformation_matrix = parent_transformation * my_matrix;

    for &child in &node.children {
        update_node_transformations(&mut *child, &node.current_transformation_matrix);
    }
}
