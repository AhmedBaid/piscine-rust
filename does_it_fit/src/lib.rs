mod areas_volumes;
pub use crate::areas_volumes::*;
pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let container_area = x * y;
    let shape_area = match kind {
        areas_volumes::GeometricalShapes::Square => {
            let area = areas_volumes::square_area(a);
            area * times
        }
        areas_volumes::GeometricalShapes::Circle => {
            let area = areas_volumes::circle_area(a);
            area as usize * times
        }
        areas_volumes::GeometricalShapes::Rectangle => {
            let area = areas_volumes::rectangle_area(a, b);
            area * times
        }
        areas_volumes::GeometricalShapes::Triangle => {
            let area = areas_volumes::triangle_area(a, b);
            area as usize * times
        }
    };
    shape_area <= container_area
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let container_area = x * y * z;
    let shape_area = match kind {
        areas_volumes::GeometricalVolumes::Cube => {
            let area = areas_volumes::cube_volume(a);
            area * times
        }
        areas_volumes::GeometricalVolumes::Sphere => {
            let area = areas_volumes::sphere_volume(a);
            area as usize * times
        }
        areas_volumes::GeometricalVolumes::Cone => {
            let area = areas_volumes::cone_volume(a, b);
            area as usize * times
        }
        areas_volumes::GeometricalVolumes::TriangularPyramid => {
            let area = areas_volumes::triangular_pyramid_volume(a as f64, b);
            area as usize * times
        }
        areas_volumes::GeometricalVolumes::Parallelepiped => {
            let area = areas_volumes::parallelepiped_volume(a, b, c);
            area as usize * times
        }
    };
    shape_area <= container_area
}
