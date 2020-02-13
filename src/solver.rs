use crate::array_cube::Cube;
use crate::cube::FaceOrientation::*;
use crate::cube::Color;
use crate::cube::Color::*;
use crate::mover;

fn move_top_cross_edge(cube : &mut Cube, color : Color) {
    let edge = cube.find_edge(WHITE, color);

    match (edge.side1.face_orientation, edge.side2.face_orientation) {
        (LEFT, BACK) => cube.move_bi(),
        (DOWN, BACK) => {
            cube.move_bi();
            cube.move_bi();
        },
        (BACK, LEFT) => cube.move_l(),
        _ => println!("no-op: {:?}", edge),
    }
}

fn top_cross(cube : &mut Cube) {
    mover::orient_cube(cube, UP, WHITE);
    //let center_color = cube.findFace(UP);
    //println!("cetner_color: {}", center_color)
    //move top edge
    for edge in &[ORANGE, RED, BLUE, GREEN] {
        move_top_cross_edge(cube, *edge)
    }
}

pub fn solve(cube : &mut Cube) {
    println!("solve!");
    cube.ansi_print();
    top_cross(cube);
    println!("top cross");
    cube.ansi_print();
}

#[cfg(test)]
mod tests {

    #[test]
    fn move_top_center() {
        /*
        let mut cube = Cube::new([WHITE, RED, BLUE, ORANGE, GREEN, YELLOW]);
        cube.move_b();
        cube.move_b();
        cube.ansi_print();
        //solve(&mut cube);
        move_top_cross_edge(&mut cube, GREEN);
        cube.ansi_print();
        assert_eq!(0, 1);
        */
    }
}
