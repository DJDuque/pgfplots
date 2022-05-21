use super::*;

#[test]
fn coordinate_2d_from_short_tuple() {
    let coord: Coordinate2D = (1.0, -1.0).into();
    assert_eq!(coord.x, 1.0);
    assert_eq!(coord.y, -1.0);
    assert!(coord.error_x.is_none());
    assert!(coord.error_y.is_none());
}

#[test]
fn coordinate_2d_from_long_tuple() {
    let coord: Coordinate2D = (1.0, -1.0, None, None).into();
    assert_eq!(coord.x, 1.0);
    assert_eq!(coord.y, -1.0);
    assert!(coord.error_x.is_none());
    assert!(coord.error_y.is_none());

    let coord: Coordinate2D = (1.0, -1.0, Some(3.0), None).into();
    assert_eq!(coord.x, 1.0);
    assert_eq!(coord.y, -1.0);
    assert_eq!(coord.error_x.unwrap(), 3.0);
    assert!(coord.error_y.is_none());

    let coord: Coordinate2D = (1.0, -1.0, None, Some(3.0)).into();
    assert_eq!(coord.x, 1.0);
    assert_eq!(coord.y, -1.0);
    assert!(coord.error_x.is_none());
    assert_eq!(coord.error_y.unwrap(), 3.0);

    let coord: Coordinate2D = (1.0, -1.0, Some(4.0), Some(3.0)).into();
    assert_eq!(coord.x, 1.0);
    assert_eq!(coord.y, -1.0);
    assert_eq!(coord.error_x.unwrap(), 4.0);
    assert_eq!(coord.error_y.unwrap(), 3.0);
}

#[test]
fn coordinate_2d_to_string() {
    let coord: Coordinate2D = (1.0, -1.0, None, None).into();
    assert_eq!(coord.to_string(), "(1,-1)");

    let coord: Coordinate2D = (1.0, -1.0, Some(3.0), None).into();
    assert_eq!(coord.to_string(), "(1,-1)\t+- (3,0)");

    let coord: Coordinate2D = (1.0, -1.0, None, Some(3.0)).into();
    assert_eq!(coord.to_string(), "(1,-1)\t+- (0,3)");

    let coord: Coordinate2D = (1.0, -1.0, Some(4.0), Some(3.0)).into();
    assert_eq!(coord.to_string(), "(1,-1)\t+- (4,3)");
}
