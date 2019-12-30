
mod cube;
mod array_cube;
mod solver;
mod mover;

fn main() {
    let mut cube = array_cube::ArrayCube::random();
    cube.print();
    solver::solve(&mut cube);
    println!("Hello, world!");

    /*
    let mut mixed_cube = cube.clone();
    let mut i = 0;
    loop {
        println!("------ step {} move --------", i);
        mixed_cube.move_r();
        mixed_cube.print();
        println!("------ step {} rotate --------", i);
        mixed_cube.rotate_right();
        mixed_cube.print();
        i = i + 1;

        if mixed_cube == cube {
            println!("solved: {}!", i);
            break;
        }
    }
    */
}
