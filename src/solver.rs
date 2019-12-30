use crate::array_cube::ArrayCube;
use crate::cube::Color::*;
use crate::cube::FaceOrientation::*;

fn topCross(cube : &mut ArrayCube) {
    let centerColor = cube.findFace(UP);
    //move top edge

}

pub fn solve(cube : &mut ArrayCube) {
    println!("solve!");
    cube.print();
    topCross(cube);
    println!("top cross");
    cube.print();
}
