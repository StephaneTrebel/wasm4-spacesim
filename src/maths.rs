extern crate alloc;

#[derive(Clone, Copy, PartialEq)]
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub fn transform(vertice: Coordinates, matrix: [f32; 16]) -> Coordinates {
    let w = vertice.x * matrix[3]
        + vertice.y * matrix[7]
        + vertice.z * matrix[11]
        + vertice.w * matrix[15];
    Coordinates {
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

pub fn rotate_xz(vertice: Coordinates, theta: f32) -> Coordinates {
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

pub fn rotate_yz(vertice: Coordinates, theta: f32) -> Coordinates {
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

pub fn project(vertice: Coordinates) -> Coordinates {
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
