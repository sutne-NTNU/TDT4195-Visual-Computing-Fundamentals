#![allow(dead_code)]

/// Fill vector of indices from 0 to vertices.len().
///
/// * `indices` - Vector to place the indices in
/// * `vertices` - Vector with vertices to count
pub fn enumerate_indices(
    indices: &mut Vec<u32>,
    vertices: &Vec<f32>,
) {
    for i in 0..(*vertices).len() {
        indices.push(i as u32);
    }
}

/// Simple triangle with coordintaes give in the first task.
///
/// * `vertices` - Vector to place vertices in
/// * `indices` - Vector to place indices in
pub fn first_triangle(
    vertices: &mut Vec<f32>,
    indices: &mut Vec<u32>,
) {
    *vertices = vec![
        -0.6, -0.6, 0.0, //
        0.6, -0.6, 0.0, //
        0.0, 0.6, 0.0, //
    ];
    enumerate_indices(indices, vertices);
}

/// Five triangles in an envelope pattern.
///
/// * `vertices` - Vector to place vertices in
/// * `indices` - Vector to place indices in
pub fn five_triangles(
    vertices: &mut Vec<f32>,
    indices: &mut Vec<u32>,
) {
    *vertices = vec![
        // triangle 1: Left
        -0.01, 0.00, 0.00, //
        -0.31, 0.30, 0.00, //
        -0.31, -0.30, 0.00, //
        // Triangle 2: Middle (under roof)
        0.00, 0.01, 0.00, //
        0.30, 0.31, 0.00, //
        -0.30, 0.31, 0.00, //
        // Triangle 3: Right
        0.01, 0.00, 0.00, //
        0.31, -0.30, 0.00, //
        0.31, 0.30, 0.00, //
        // Triangle 4: Bottom
        0.00, -0.01, 0.00, //
        -0.30, -0.31, 0.00, //
        0.30, -0.31, 0.00, //
        // Triangle 5: Top (roof)
        -0.30, 0.324, 0.00, //
        0.30, 0.324, 0.00, //
        0.00, 0.624, 0.00, //
    ];
    enumerate_indices(indices, vertices);
}

/// Triangle that is clipped
///
/// * `vertices` - Vector to place vertices in
/// * `indices` - Vector to place indices in
pub fn clipping_triangle(
    vertices: &mut Vec<f32>,
    indices: &mut Vec<u32>,
) {
    *vertices = vec![
        0.6, -0.8, -1.2, //
        0.0, 0.4, 0.0, //
        -0.8, 0.2, 1.2, //
    ];
    enumerate_indices(indices, vertices);
}

/// Triangle for demonstrating culling and mirroring/colouring with the sahders
///
/// * `vertices` - Vector to place vertices in
/// * `indices` - Vector to place indices in
pub fn culling_triangle(
    vertices: &mut Vec<f32>,
    indices: &mut Vec<u32>,
    visible: bool,
) {
    *vertices = vec![
        -0.5, -0.5, 0.0, //
        0.5, 0.0, 0.0, //
        0.0, 0.5, 0.0, //
    ];

    let mut counter_clockwise: Vec<u32> = vec![0, 1, 2];
    let mut clockwise: Vec<u32> = vec![2, 1, 0];

    if visible {
        indices.append(&mut counter_clockwise);
    } else {
        indices.append(&mut clockwise);
    }
}

/// Fill with vertices and indices for drawing a circle using **gl::TRIANGLE_FAN**.
///
/// * `vertices` - Vector to place vertices in
/// * `indices` - Vector to place indices in
pub fn circle(
    vertices: &mut Vec<f32>,
    indices: &mut Vec<u32>,
) {
    let center = (0.0, 0.0, 0.0);
    let radius = 0.7;
    let number_of_vertices = 250;
    create_circle_vertices(vertices, center, radius, number_of_vertices);
    enumerate_indices(indices, vertices);
}

/// Create vertices for a circle.
///
/// * `vertices` - Vector to append the vertices to
/// * `center` - Center of the circle (x, y, z)
/// * `radius` - Radius of the circle
/// * `number_of_vertices` - How many vertices to generate (higher means smoother circle)
fn create_circle_vertices(
    vertices: &mut Vec<f32>,
    center: (f32, f32, f32),
    radius: f32,
    number_of_vertices: u32,
) {
    let mut center_vec: Vec<f32> = vec![center.0, center.1, center.2];
    vertices.append(&mut center_vec);
    for i in 0..number_of_vertices + 1 {
        let val = (i as f32 * 2.0 * std::f32::consts::PI) / (number_of_vertices as f32);
        vertices.push(center.0 + (radius * f32::cos(val))); // X
        vertices.push(center.1 + (radius * f32::sin(val))); // Y
        vertices.push(center.2); // Z
    }
}
