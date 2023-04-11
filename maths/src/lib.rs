#[derive(Default, Clone, Copy, PartialEq)]
pub struct Coordinates3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Coordinates2d {
    pub x: f32,
    pub y: f32,
}

pub fn transform(vertice: Coordinates3d, matrix: [f32; 16]) -> Coordinates3d {
    let w = vertice.x * matrix[3]
        + vertice.y * matrix[7]
        + vertice.z * matrix[11]
        + vertice.w * matrix[15];
    Coordinates3d {
        x: (vertice.x * matrix[0]
            + vertice.y * matrix[4]
            + vertice.z * matrix[8]
            + vertice.w * matrix[12])
            / w,
        y: (vertice.x * matrix[1]
            + vertice.y * matrix[5]
            + vertice.z * matrix[9]
            + vertice.w * matrix[13])
            / w,
        z: (vertice.x * matrix[2]
            + vertice.y * matrix[6]
            + vertice.z * matrix[10]
            + vertice.w * matrix[14])
            / w,
        w: 1.0,
    }
}

pub fn rotate_xz(vertice: Coordinates3d, theta: f32) -> Coordinates3d {
    transform(
        vertice,
        [
            theta.cos(),
            0.0,
            -theta.sin(),
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            theta.sin(),
            0.0,
            theta.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ],
    )
}

pub fn rotate_yz(vertice: Coordinates3d, theta: f32) -> Coordinates3d {
    transform(
        vertice,
        [
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            theta.cos(),
            theta.sin(),
            0.0,
            0.0,
            -theta.sin(),
            theta.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ],
    )
}

pub fn project(vertice: Coordinates3d) -> Coordinates3d {
    transform(
        vertice,
        [
            128.0,
            0.0,
            0.0,
            0.0,
            0.0,
            128.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            1.0 / 2.0,
            0.0,
            0.0,
            0.0,
            0.0,
        ],
    )
}

// compute distance between a
// player and anything
pub fn distance(coords: Coordinates3d) -> f32 {
    (coords.x.powi(2) + coords.y.powi(2) + coords.z.powi(2))
        .sqrt()
        .floor()
}
