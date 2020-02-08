use crate::array_cube::Cube;
use crate::cube::FaceOrientation;
use crate::cube::FaceOrientation::*;
use crate::cube::Color;

pub fn orient_cube(cube : &mut Cube, face_orientation : FaceOrientation, color : Color) {
    let current_face = cube.findCenter(color);
    println!("move {} from {:?} to {:?}", color, current_face, face_orientation);
    match (current_face, face_orientation) {
        (FRONT, UP) => cube.orientation_rotate_up(),
        (FRONT, LEFT) => cube.orientation_rotate_right(),
        (FRONT, RIGHT) => cube.orientation_rotate_left(),
        (FRONT, BACK) => {
            cube.orientation_rotate_left();
            cube.orientation_rotate_left();
        },
        (FRONT, DOWN) => cube.orientation_rotate_down(),
        (BACK, RIGHT) => cube.orientation_rotate_right(),
        (BACK, LEFT) => cube.orientation_rotate_left(),
        (BACK, UP) => cube.orientation_rotate_down(),
        (BACK, DOWN) => cube.orientation_rotate_up(),
        (BACK, FRONT) => {
            cube.orientation_rotate_left();
            cube.orientation_rotate_left();
        },
        (UP, DOWN) => {
            cube.orientation_rotate_up();
            cube.orientation_rotate_up();
        },
        (UP, BACK) => cube.orientation_rotate_up(),
        (UP, LEFT) => {
            cube.orientation_rotate_up();
            cube.orientation_rotate_left();
        },
        (UP, RIGHT) => {
            cube.orientation_rotate_up();
            cube.orientation_rotate_right();
        },
        (UP, FRONT) => cube.orientation_rotate_down(),
        (LEFT, FRONT) => cube.orientation_rotate_left(),
        (LEFT, BACK) => cube.orientation_rotate_right(),
        (LEFT, UP) => {
            cube.orientation_rotate_left();
            cube.orientation_rotate_up();
        },
        (LEFT, DOWN) => {
            cube.orientation_rotate_left();
            cube.orientation_rotate_down();
        },
        (LEFT, RIGHT) => {
            cube.orientation_rotate_left();
            cube.orientation_rotate_left();
        },
        (RIGHT, LEFT) => {
            cube.orientation_rotate_left();
            cube.orientation_rotate_left();
        },
        (RIGHT, BACK) => {
            cube.orientation_rotate_left();
        },
        (RIGHT, FRONT) => {
            cube.orientation_rotate_right();
        },
        (RIGHT, UP) => {
            cube.orientation_rotate_right();
            cube.orientation_rotate_up();
        },
        (RIGHT, DOWN) => {
            cube.orientation_rotate_right();
            cube.orientation_rotate_down();
        },
        (DOWN, UP) => {
            cube.orientation_rotate_up();
            cube.orientation_rotate_up();
        },
        (DOWN, BACK) => cube.orientation_rotate_down(),
        (DOWN, FRONT) => cube.orientation_rotate_up(),
        (DOWN, LEFT) => {
            cube.orientation_rotate_up();
            cube.orientation_rotate_right();
        },
        (DOWN, RIGHT) => {
            cube.orientation_rotate_up();
            cube.orientation_rotate_left();
        },
        _ => println!("no-op")
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Color::*;

    use super::orient_cube;

    fn do_test(face_orientation : FaceOrientation, color : Color) {
        let mut cube = Cube::new([RED, YELLOW, BLUE, WHITE, GREEN, ORANGE]);
        orient_cube(&mut cube, face_orientation, color);
        println!("color on {:?}: {}", face_orientation, cube.findFace(face_orientation));
        assert_eq!(cube.findFace(face_orientation), color);
    }

    mod front_to {
        use super::*;
        //font is blue
        fn check_side(face_orientation : FaceOrientation) {
            do_test(face_orientation, BLUE);
        }
        #[test]
        fn up() {
            check_side(UP);
        }

        #[test]
        fn left() {
            check_side(LEFT);
        }

        #[test]
        fn front() {
            check_side(FRONT);
        }
        #[test]
        fn right() {
            check_side(RIGHT);
        }
        #[test]
        fn back() {
            check_side(BACK);
        }
        #[test]
        fn down() {
            check_side(DOWN);
        }
    }
    mod back_to {
        use super::*;
        //back is green 
        fn check_side(face_orientation : FaceOrientation) {
            do_test(face_orientation, GREEN);
        }
        #[test]
        fn up() {
            check_side(UP);
        }

        #[test]
        fn left() {
            check_side(LEFT);
        }

        #[test]
        fn front() {
            check_side(FRONT);
        }
        #[test]
        fn right() {
            check_side(RIGHT);
        }
        #[test]
        fn back() {
            check_side(BACK);
        }
        #[test]
        fn down() {
            check_side(DOWN);
        }
    }
    mod up_to {
        use super::*;
        //up is red
        fn check_side(face_orientation : FaceOrientation) {
            do_test(face_orientation, RED);
        }
        #[test]
        fn up() {
            check_side(UP);
        }

        #[test]
        fn left() {
            check_side(LEFT);
        }

        #[test]
        fn front() {
            check_side(FRONT);
        }
        #[test]
        fn right() {
            check_side(RIGHT);
        }
        #[test]
        fn back() {
            check_side(BACK);
        }
        #[test]
        fn down() {
            check_side(DOWN);
        }
    }
    mod left_to {
        use super::*;
        //left is yellow
        fn check_side(face_orientation : FaceOrientation) {
            do_test(face_orientation, YELLOW);
        }
        #[test]
        fn up() {
            check_side(UP);
        }

        #[test]
        fn left() {
            check_side(LEFT);
        }

        #[test]
        fn front() {
            check_side(FRONT);
        }
        #[test]
        fn right() {
            check_side(RIGHT);
        }
        #[test]
        fn back() {
            check_side(BACK);
        }
        #[test]
        fn down() {
            check_side(DOWN);
        }
    }
    mod right_to {
        use super::*;
        //right is white
        fn check_side(face_orientation : FaceOrientation) {
            do_test(face_orientation, WHITE);
        }
        #[test]
        fn up() {
            check_side(UP);
        }

        #[test]
        fn left() {
            check_side(LEFT);
        }

        #[test]
        fn front() {
            check_side(FRONT);
        }
        #[test]
        fn right() {
            check_side(RIGHT);
        }
        #[test]
        fn back() {
            check_side(BACK);
        }
        #[test]
        fn down() {
            check_side(DOWN);
        }
    }
    mod bottom_to {
        use super::*;
        //bottom is ORANGE
        fn check_side(face_orientation : FaceOrientation) {
            do_test(face_orientation, ORANGE);
        }
        #[test]
        fn up() {
            check_side(UP);
        }

        #[test]
        fn left() {
            check_side(LEFT);
        }

        #[test]
        fn front() {
            check_side(FRONT);
        }
        #[test]
        fn right() {
            check_side(RIGHT);
        }
        #[test]
        fn back() {
            check_side(BACK);
        }
        #[test]
        fn down() {
            check_side(DOWN);
        }
    }
}
