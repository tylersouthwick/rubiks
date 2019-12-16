
mod cube;

fn main() {
    let mut cube = cube::Cube::default();
    cube.print();
    println!("Hello, world!");

    cube.orientation_rotate_right();
    cube.print();

    cube.orientation_rotate_up();
    cube.print();
}
