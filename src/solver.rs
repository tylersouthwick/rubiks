use crate::array_cube::ArrayCube;
use crate::cube::FaceOrientation::*;
use crate::cube::Color::*;
use crate::mover;

fn top_cross(cube : &mut ArrayCube) {
    mover::orient_cube(cube, UP, WHITE);
    //let center_color = cube.findFace(UP);
    //println!("cetner_color: {}", center_color)
    //move top edge

}

pub fn solve(cube : &mut ArrayCube) {
    println!("solve!");
    cube.print();
    top_cross(cube);
    println!("top cross");
    cube.print();
}
