use crate::cube::Cube;
use crate::cube::Color::*;
use crate::cube::FaceOrientation::*;

fn topCross(cube : &mut Cube) {
    let centerColor = cube.findFace(UP);
    //move top edge

}

pub fn solve(cube : &mut Cube) {
    println!("solve!");
    cube.print();
    topCross(cube);
    println!("top cross");
    cube.print();
}
