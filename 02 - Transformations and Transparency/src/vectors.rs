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

/// Three overlapping triangles
///
/// * `vertices` - Vector to place vertices in
/// * `indices` - Vector to place indices in
/// * `colors` - Vector to place colors for each vertex
pub fn four_triangles(
    vertices: &mut Vec<f32>,
    indices: &mut Vec<u32>,
    colors: &mut Vec<f32>,
) {
    let size = 0.8;
    *vertices = vec![
        // triangle 1:
        0.0, 0.0, -0.1, //
        -size, size, -0.1, //
        -size, -size, -0.1, //
        // Triangle 2:
        0.0, 0.0, 0.0, //
        size, size, 0.0, //
        -size, size, 0.0, //
        // Triangle 3:
        0.0, 0.0, 0.1, //
        size, -size, 0.1, //
        size, size, 0.1, //
        // Triangle 4:
        0.0, 0.0, 0.1, //
        -size, -size, 0.1, //
        size, -size, 0.1, //
    ];

    enumerate_indices(indices, vertices);

    let alpha: f32 = 0.9;
    let blend = 0.95;
    *colors = vec![
        // Triangle 1:
        1.0, 0.0, 0.0, alpha, //
        1.0, 0.0, blend, alpha, //
        1.0, blend, 0.0, alpha, //
        // Triangle 2:
        0.0, 1.0, 0.0, alpha, //
        0.0, 1.0, blend, alpha, //
        blend, 1.0, 0.0, alpha, //
        // Triangle 3:
        0.0, 0.0, 1.0, alpha, //
        0.0, blend, 1.0, alpha, //
        blend, 0.0, 1.0, alpha, //
        // Triangle 4:
        1.0, 0.0, 1.0, alpha, //
        0.0, 1.0, 1.0, alpha, //
        1.0, 1.0, 0.0, alpha, //
    ];
}

/// Three overlapping triangles
///
/// * `vertices` - Vector to place vertices in
/// * `indices` - Vector to place indices in
/// * `colors` - Vector to place colors for each vertex
pub fn three_overlapping_triangles(
    vertices: &mut Vec<f32>,
    indices: &mut Vec<u32>,
    colors: &mut Vec<f32>,
) {
    let spacing = 0.5;
    *vertices = vec![
        // Triangle 1:
        -0.5, -0.2, spacing, //
        0.8, 0.0, spacing, //
        0.55, 0.6, spacing, //
        // triangle 2:
        -0.5, -0.8, 0.0, //
        0.5, -0.8, 0.0, //
        0.0, 0.8, 0.0, //
        // Triangle 3:
        -0.8, 0.0, -spacing, //
        0.5, -0.2, -spacing, //
        -0.55, 0.6, -spacing, //
    ];

    enumerate_indices(indices, vertices);

    let alpha: f32 = 0.4;
    let blend = 0.0;
    *colors = vec![
        // Triangle 1:
        0.0, 0.0, 1.0, alpha, //
        0.0, blend, 1.0, alpha, //
        blend, 0.0, 1.0, alpha, //
        // Triangle 2:
        0.0, 1.0, 0.0, alpha, //
        0.0, 1.0, blend, alpha, //
        blend, 1.0, 0.0, alpha, //
        // Triangle 3:
        1.0, 0.0, 0.0, alpha, //
        1.0, 0.0, blend, alpha, //
        1.0, blend, 0.0, alpha, //
    ];
}

/// Exactly the same as Three overlapping triangles except here
/// i make sure to draw the triangles in reverse (because z-axis is flipped
/// when performing multiplication with persepctiev matrix).graphics
///
/// I also change the order when drawing the back of the triangles to
/// make the blending function properly from both sides (as you move around it).
pub fn three_triangles_in_3d(
    vertices: &mut Vec<f32>,
    indices: &mut Vec<u32>,
    colors: &mut Vec<f32>,
) {
    let spacing = 0.3;
    *vertices = vec![
        // Triangle 1:
        -0.5, -0.2, spacing, //
        0.8, 0.0, spacing, //
        0.55, 0.6, spacing, //
        // triangle 2:
        -0.5, -0.8, 0.0, //
        0.5, -0.8, 0.0, //
        0.0, 0.8, 0.0, //
        // Triangle 3:
        -0.8, 0.0, -spacing, //
        0.5, -0.2, -spacing, //
        -0.55, 0.6, -spacing, //
    ];

    // With perspective we need to reorder the triangles again
    *indices = vec![
        //
        0, 2, 1, //
        3, 5, 4, //
        6, 8, 7, //
        //
        6, 7, 8, //
        3, 4, 5, //
        0, 1, 2, //
    ];

    let alpha: f32 = 0.4;
    let blend = 0.8;
    *colors = vec![
        // Triangle 1:
        0.0, 0.0, 1.0, alpha, //
        0.0, blend, 1.0, alpha, //
        blend, 0.0, 1.0, alpha, //
        // Triangle 2:
        0.0, 1.0, 0.0, alpha, //
        0.0, 1.0, blend, alpha, //
        blend, 1.0, 0.0, alpha, //
        // Triangle 3:
        1.0, 0.0, 0.0, alpha, //
        1.0, 0.0, blend, alpha, //
        1.0, blend, 0.0, alpha, //
    ];
}
