
extern crate ansi_term;

mod cube;
mod array_cube;
mod solver;
mod mover;
mod terminal_render;

fn main() {
    let mut cube = array_cube::Cube::default();//random();
    cube.move_r();
    cube.ansi_print();
    solver::solve(&mut cube);
    cube.ansi_print();

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
