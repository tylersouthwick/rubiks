use crate::array_cube::ArrayCube;
use crate::cube::FaceOrientation::*;
use crate::cube::Color;
use crate::cube::Color::*;
use crate::mover;

fn move_top_cross_edge(cube : &mut ArrayCube, color1 : Color, color2 : Color) {
    let edge = cube.find_edge(color1, color2);
    println!("edge: {:?}", edge)
}

fn top_cross(cube : &mut ArrayCube) {
    mover::orient_cube(cube, UP, WHITE);
    //let center_color = cube.findFace(UP);
    //println!("cetner_color: {}", center_color)
    //move top edge
    move_top_cross_edge(cube, WHITE, ORANGE)
}

pub fn solve(cube : &mut ArrayCube) {
    println!("solve!");
    cube.print();
    top_cross(cube);
    println!("top cross");
    cube.print();
}
