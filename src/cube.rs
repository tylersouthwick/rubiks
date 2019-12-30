use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    RED, YELLOW, WHITE, GREEN, BLUE, ORANGE,
}

use Color::*;


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Color::RED => "R",
            Color::YELLOW => "Y",
            Color::WHITE => "W",
            Color::GREEN => "G",
            Color::BLUE => "B",
            Color::ORANGE => "O"
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FaceOrientation {
    RIGHT,
    LEFT,
    BACK,
    DOWN,
    FRONT,
    UP,
}

impl FaceOrientation {
    fn fromIndex(i : usize) -> FaceOrientation {
        match i {
            0 => FaceOrientation::UP,
            1 => FaceOrientation::LEFT,
            2 => FaceOrientation::FRONT,
            3 => FaceOrientation::RIGHT,
            4 => FaceOrientation::BACK,
            _ => FaceOrientation::DOWN,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Face {
    squares: [[Color; 3]; 3 ]
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cube {
    faces : [Face; 6]
}

impl Face {
    fn new(color : Color) -> Face {
        Face {
            squares: [[color, color, color],
                [color, color, color],
                [color, color, color]]
        }
    }

    fn rotate_left(&mut self) {
        self.rotate_right();
        self.rotate_right();
        self.rotate_right();
    }

    fn rotate_right(&mut self) {
        /*
           0 1 2    6 3 0
           3 4 5 -> 7 4 1
           6 7 8    8 5 2
        */
        let square0 = self.squares[0][0];
        let square1 = self.squares[0][1];
        let square2 = self.squares[0][2];
        let square3 = self.squares[1][0];
        let square5 = self.squares[1][2];
        let square6 = self.squares[2][0];
        let square7 = self.squares[2][1];
        let square8 = self.squares[2][2];

/*
        println!("square0: {}", square0);
        println!("square1: {}", square1);
        println!("square2: {}", square2);
        println!("square3: {}", square3);
        println!("square4: {}", self.squares[1][1]);
        println!("square5: {}", square5);
        println!("square6: {}", square6);
        println!("square7: {}", square7);
        println!("square8: {}", square8);
        */

        self.squares[0][0] = square6;
        self.squares[0][1] = square3;
        self.squares[0][2] = square0;
        self.squares[1][0] = square7;
        self.squares[1][2] = square1;
        self.squares[2][0] = square8;
        self.squares[2][1] = square5;
        self.squares[2][2] = square2;

/*
        println!("square00: {}", self.squares[0][0]);
        println!("square01: {}", self.squares[0][1]);
        println!("square02: {}", self.squares[0][2]);
        println!("square10: {}", self.squares[1][0]);
        println!("square11: {}", self.squares[1][1]);
        println!("square12: {}", self.squares[1][2]);
        println!("square20: {}", self.squares[2][0]);
        println!("square21: {}", self.squares[2][1]);
        println!("square22: {}", self.squares[2][2]);
        */
    }
}

impl Cube {

    pub fn new(faces : [Color; 6]) -> Self {
        Cube {
            faces: [
                Face::new(faces[0]),
                Face::new(faces[1]),
                Face::new(faces[2]),
                Face::new(faces[3]),
                Face::new(faces[4]),
                Face::new(faces[5])
            ]
        }
    }

    pub fn random() -> Self {
        let mut cube = Cube::default();
        for _ in 0..500 {
            cube.rotate_right();
            cube.move_r();
            cube.orientation_rotate_left();
        }
        cube
    }

    pub fn default() -> Self {
        Cube::new([BLUE, RED, YELLOW, ORANGE, WHITE, GREEN])
    }

    pub fn findFace(self, faceOrientation : FaceOrientation) -> Color {
        match faceOrientation {
            RIGHT => self.faces[3].squares[1][1],
            LEFT => self.faces[1].squares[1][1],
            BACK => self.faces[4].squares[1][1],
            DOWN => self.faces[5].squares[1][1],
            FRONT => self.faces[2].squares[1][1],
            UP => self.faces[0].squares[1][1],
        }
    }

    pub fn findCenter(self, color : Color) -> FaceOrientation {
        for i in 0..4 {
            if self.faces[i].squares[1][1] == color {
                return FaceOrientation::fromIndex(i)
            }
        }
        return FaceOrientation::DOWN;
    }

    pub fn orientation_rotate_right(&mut self) {
        //rotate the cube so the face to the right is now the center cube
        //   1           1
        //  2345   ---> 3452
        //   6           6
        let face2 = self.faces[1];
        let face3 = self.faces[2];
        let face4 = self.faces[3];
        let face5 = self.faces[4];
        self.faces[1] = face3;
        self.faces[2] = face4;
        self.faces[3] = face5;
        self.faces[4] = face2;
    }

    pub fn orientation_rotate_left(&mut self) {
        self.orientation_rotate_right();
        self.orientation_rotate_right();
        self.orientation_rotate_right();
    }

    pub fn orientation_rotate_up(&mut self) {
        //rotate the cube so the face to the right is now the center cube
        //   1           3
        //  2345   ---> 2645
        //   6           1
        let face1 = self.faces[0];
        let face3 = self.faces[2];
        let face6 = self.faces[5];
        self.faces[0] = face3;
        self.faces[2] = face6;
        self.faces[5] = face1;
    }

    pub fn orientation_rotate_down(&mut self) {
        self.orientation_rotate_up();
        self.orientation_rotate_up();
    }

    pub fn rotate_left(&mut self) {
        self.rotate_right();
        self.rotate_right();
        self.rotate_right();
    }

    pub fn rotate_right(&mut self) {
        //   0           1
        //  1234   ---> 5204
        //   5           3
        //    |BBB|                 |RRR|
        //    |BBB|                 |RRR|
        //    |BBB|                 |RRR|
        //|RRR|YYY|OOO|WWW|     |GGG|YYY|BBB|WWW|
        //|RRR|YYY|OOO|WWW| --> |GGG|YYY|BBB|WWW|
        //|RRR|YYY|OOO|WWW|     |GGG|YYY|BBB|WWW|
        //    |GGG|                 |OOO|
        //    |GGG|                 |OOO|
        //    |GGG|                 |OOO|
        //faces 2 & 4 do not change
        //1 -> 0
        //5 -> 1
        //0 -> 3
        //3 -> 5
        let face0 = self.faces[0];
        let face1 = self.faces[1];
        let face3 = self.faces[3];
        let face5 = self.faces[5];

        self.faces[0] = face1;
        self.faces[1] = face5;
        self.faces[3] = face0;
        self.faces[5] = face3;

/*
        println!("----------- before rotating");
        self.print();
        println!("----------- after rotating");
        */
        //the center has to rotate
        self.faces[2].rotate_right();
        self.faces[1].rotate_right();
        self.faces[3].rotate_right();
        self.faces[4].rotate_left();
        self.faces[0].rotate_right();
        self.faces[5].rotate_right();
    }

    pub fn move_f(&mut self) {
        //    |RRR|                 |RRR|
        //    |RRR|                 |RRR|
        //    |RRR|                 |YYY|
        //|YYY|WWW|GGG|BBB|     |YYO|WWW|RGG|BBB|
        //|YYY|WWW|GGG|BBB| --> |YYO|WWW|RGG|BBB|
        //|YYY|WWW|GGG|BBB|     |YYO|WWW|RGG|BBB|
        //    |OOO|                 |GGG|
        //    |OOO|                 |OOO|
        //    |OOO|                 |OOO|
        let left1 = self.faces[1].squares[0][2];
        let left2 = self.faces[1].squares[1][2];
        let left3 = self.faces[1].squares[2][2];

        let up1 = self.faces[0].squares[2][0];
        let up2 = self.faces[0].squares[2][1];
        let up3 = self.faces[0].squares[2][2];

        let right1 = self.faces[3].squares[0][0];
        let right2 = self.faces[3].squares[1][0];
        let right3 = self.faces[3].squares[2][0];

        let bottom1 = self.faces[5].squares[0][0];
        let bottom2 = self.faces[5].squares[0][1];
        let bottom3 = self.faces[5].squares[0][2];

        self.faces[1].squares[0][2] = bottom1;
        self.faces[1].squares[1][2] = bottom2;
        self.faces[1].squares[2][2] = bottom3;

        self.faces[0].squares[2][0] = left3;
        self.faces[0].squares[2][1] = left2;
        self.faces[0].squares[2][2] = left1;

        self.faces[3].squares[0][0] = up1;
        self.faces[3].squares[1][0] = up2;
        self.faces[3].squares[2][0] = up3;

        self.faces[5].squares[0][0] = right3;
        self.faces[5].squares[0][1] = right2;
        self.faces[5].squares[0][2] = right1;

        self.faces[2].rotate_right();
    }

    pub fn move_fi(&mut self) {
        self.move_f();
        self.move_f();
        self.move_f();
    }

    pub fn move_r(&mut self) {
        //    |RRR|                 |RRW|
        //    |RRR|                 |RRW|
        //    |RRR|                 |RRW|
        //|YYY|WWW|GGG|BBB|     |YYY|WWO|GGG|RBB|
        //|YYY|WWW|GGG|BBB| --> |YYY|WWO|GGG|RBB|
        //|YYY|WWW|GGG|BBB|     |YYY|WWO|GGG|RBB|
        //    |OOO|                 |OOB|
        //    |OOO|                 |OOB|
        //    |OOO|                 |OOB|
        let up1 = self.faces[0].squares[0][2];
        let up2 = self.faces[0].squares[1][2];
        let up3 = self.faces[0].squares[2][2];

        let right1 = self.faces[4].squares[0][0];
        let right2 = self.faces[4].squares[1][0];
        let right3 = self.faces[4].squares[2][0];

        let bottom1 = self.faces[5].squares[0][2];
        let bottom2 = self.faces[5].squares[1][2];
        let bottom3 = self.faces[5].squares[2][2];

        let center02 = self.faces[2].squares[0][2];
        let center12 = self.faces[2].squares[1][2];
        let center22 = self.faces[2].squares[2][2];

        self.faces[2].squares[0][2] = bottom1;
        self.faces[2].squares[1][2] = bottom2;
        self.faces[2].squares[2][2] = bottom3;

        self.faces[0].squares[0][2] = center02;
        self.faces[0].squares[1][2] = center12;
        self.faces[0].squares[2][2] = center22;

        self.faces[4].squares[0][0] = up3;
        self.faces[4].squares[1][0] = up2;
        self.faces[4].squares[2][0] = up1;

        self.faces[5].squares[0][2] = right3;
        self.faces[5].squares[1][2] = right2;
        self.faces[5].squares[2][2] = right1;

        self.faces[3].rotate_right();
    }

    pub fn move_ri(&mut self) {
        self.move_r();
        self.move_r();
        self.move_r();
    }

    pub fn print(&self) {
        print!("{}", self)
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = "    ";
        write!(f,
"{}|{}{}{}|\n\
{}|{}{}{}|\n\
{}|{}{}{}|\n\
|{}{}{}|{}{}{}|{}{}{}|{}{}{}|\n\
|{}{}{}|{}{}{}|{}{}{}|{}{}{}|\n\
|{}{}{}|{}{}{}|{}{}{}|{}{}{}|\n\
{}|{}{}{}|\n\
{}|{}{}{}|\n\
{}|{}{}{}|\n\
",
            prefix, self.faces[0].squares[0][0], self.faces[0].squares[0][1], self.faces[0].squares[0][2],
            prefix, self.faces[0].squares[1][0], self.faces[0].squares[1][1], self.faces[0].squares[1][2],
            prefix, self.faces[0].squares[2][0], self.faces[0].squares[2][1], self.faces[0].squares[2][2],
            //first long row
            self.faces[1].squares[0][0], self.faces[1].squares[0][1], self.faces[1].squares[0][2],
            self.faces[2].squares[0][0], self.faces[2].squares[0][1], self.faces[2].squares[0][2],
            self.faces[3].squares[0][0], self.faces[3].squares[0][1], self.faces[3].squares[0][2],
            self.faces[4].squares[0][0], self.faces[4].squares[0][1], self.faces[4].squares[0][2],
            //second long row
            self.faces[1].squares[1][0], self.faces[1].squares[1][1], self.faces[1].squares[1][2],
            self.faces[2].squares[1][0], self.faces[2].squares[1][1], self.faces[2].squares[1][2],
            self.faces[3].squares[1][0], self.faces[3].squares[1][1], self.faces[3].squares[1][2],
            self.faces[4].squares[1][0], self.faces[4].squares[1][1], self.faces[4].squares[1][2],
            //third long row
            self.faces[1].squares[2][0], self.faces[1].squares[2][1], self.faces[1].squares[2][2],
            self.faces[2].squares[2][0], self.faces[2].squares[2][1], self.faces[2].squares[2][2],
            self.faces[3].squares[2][0], self.faces[3].squares[2][1], self.faces[3].squares[2][2],
            self.faces[4].squares[2][0], self.faces[4].squares[2][1], self.faces[4].squares[2][2],
            //last row
            prefix, self.faces[5].squares[0][0], self.faces[5].squares[0][1], self.faces[5].squares[0][2],
            prefix, self.faces[5].squares[1][0], self.faces[5].squares[1][1], self.faces[5].squares[1][2],
            prefix, self.faces[5].squares[2][0], self.faces[5].squares[2][1], self.faces[5].squares[2][2]
                )
    }
}

#[cfg(test)]
mod tests {
    use super::Color::*;
    use super::Cube;

    #[test]
    fn rotate_down() {
        //down should undo up
        let cube = Cube::new([RED, YELLOW, WHITE, GREEN, BLUE, ORANGE]);
        let mut cube_to_rotate = cube.clone();
        cube_to_rotate.orientation_rotate_up();
        cube_to_rotate.orientation_rotate_down();
        assert_eq!(
            format!("{}", cube),
            format!("{}", cube_to_rotate)
        );
    }

    #[test]
    fn rotate_up() {
        let mut cube = Cube::new([RED, YELLOW, WHITE, GREEN, BLUE, ORANGE]);
        cube.orientation_rotate_up();
        assert_eq!(
            format!("{}", cube),
            format!("{}", Cube::new([WHITE, YELLOW, ORANGE, GREEN, BLUE, RED])),
        );
    }

    #[test]
    fn orientation_rotate_right() {
        let mut cube = Cube::new([RED, GREEN, YELLOW, BLUE, WHITE, ORANGE]);
        cube.orientation_rotate_right();
        //   1           1
        //  2345   ---> 3452
        //   6           6
        assert_eq!(
            format!("{}", cube),
            format!("{}", Cube::new([RED, YELLOW, BLUE, WHITE, GREEN, ORANGE])),
        );
    }

    #[test]
    fn rotate_right() {
        let mut cube = Cube::new([BLUE, YELLOW, ORANGE, WHITE, RED, GREEN]);
        cube.rotate_right();
        assert_eq!(
            format!("{}", cube),
            format!("{}", Cube::new([YELLOW, GREEN, ORANGE, BLUE, RED, WHITE])),
        );
    }

    #[test]
    fn rotate_left() {
        //left should undo right
        let cube = Cube::new([RED, YELLOW, WHITE, GREEN, BLUE, ORANGE]);
        let mut cube_to_rotate = cube.clone();
        cube_to_rotate.orientation_rotate_right();
        cube_to_rotate.orientation_rotate_left();
        assert_eq!(
            format!("{}", cube),
            format!("{}", cube_to_rotate)
        );
    }

    #[test]
    fn move_f() {
        let mut cube = Cube::new([RED, YELLOW, WHITE, GREEN, BLUE, ORANGE]);
        cube.print();
        cube.move_f();
        let prefix = "    ";
        let expected= format!("{}|RRR|\n\
            {}|RRR|\n\
            {}|YYY|\n\
        |YYO|WWW|RGG|BBB|\n\
        |YYO|WWW|RGG|BBB|\n\
        |YYO|WWW|RGG|BBB|\n\
        {}|GGG|\n\
        {}|OOO|\n\
        {}|OOO|\n", prefix, prefix, prefix, prefix, prefix, prefix);
println!("{}", expected);
        assert_eq!(format!("{}", cube), expected);
    }

    #[test]
    fn move_fi() {
        //ui should undo u
        let cube = Cube::new([RED, YELLOW, WHITE, GREEN, BLUE, ORANGE]);
        let mut cube_to_move = cube.clone();
        cube_to_move.move_f();
        cube_to_move.move_fi();
        assert_eq!(
            format!("{}", cube),
            format!("{}", cube_to_move)
        );
    }

    #[test]
    fn move_r() {
        //    |WWW|                 |WWB|
        //    |WWW|                 |WWB|
        //    |WWW|                 |WWB|
        //|RRR|BBB|OOO|GGG|     |RRR|BBY|OOO|WGG|
        //|RRR|BBB|OOO|GGG| --> |RRR|BBY|OOO|WGG|
        //|RRR|BBB|OOO|GGG|     |RRR|BBY|OOO|WGG|
        //    |YYY|                 |YYG|
        //    |YYY|                 |YYG|
        //    |YYY|                 |YYG|
        let mut cube = Cube::new([WHITE, RED, BLUE, ORANGE, GREEN, YELLOW]);
        cube.print();
        cube.move_r();
        println!("--------");
        cube.print();
        println!("--------");
        let prefix = "    ";
        let expected= format!("{}|WWB|\n\
            {}|WWB|\n\
            {}|WWB|\n\
        |RRR|BBY|OOO|WGG|\n\
        |RRR|BBY|OOO|WGG|\n\
        |RRR|BBY|OOO|WGG|\n\
        {}|YYG|\n\
        {}|YYG|\n\
        {}|YYG|\n", prefix, prefix, prefix, prefix, prefix, prefix);
println!("{}", expected);
        assert_eq!(format!("{}", cube), expected);
    }

    #[test]
    fn move_ri_rotate_right() {
    /*
        |BBB|
        |BBB|
        |BBB|
    |RRR|YYY|OOO|WWW|
    |RRR|YYY|OOO|WWW|
    |RRR|YYY|OOO|WWW|
        |GGG|
        |GGG|
        |GGG|
    ------ step 0 move ri --------
        |BBW|
        |BBW|
        |BBW|
    |RRR|YYB|OOO|GWW|
    |RRR|YYB|OOO|GWW|
    |RRR|YYB|OOO|GWW|
        |GGY|
        |GGY|
        |GGY|
    ------ step 0 rotate right --------
        |RRR|
        |RRR|
        |RRR|
    |GGG|YYY|BBB|WWW|
    |GGG|YYY|BBB|WWW|
    |YYY|BBB|WWW|GGG|
        |OOO|
        |OOO|
        |OOO|
    */
        let cube = Cube::new([BLUE, RED, YELLOW, ORANGE, WHITE, GREEN]);
        let mut scrambled_cube = cube.clone();
        scrambled_cube.move_ri();
        scrambled_cube.rotate_right();
        println!("scrambled!");
        scrambled_cube.print();
        let prefix = "    ";
        let expected= format!("{}|RRR|\n\
            {}|RRR|\n\
            {}|RRR|\n\
        |GGG|YYY|BBB|WWW|\n\
        |GGG|YYY|BBB|WWW|\n\
        |YYY|BBB|WWW|GGG|\n\
        {}|OOO|\n\
        {}|OOO|\n\
        {}|OOO|\n", prefix, prefix, prefix, prefix, prefix, prefix);

        println!("expected {}", expected);
        assert_eq!(format!("{}", scrambled_cube), expected);
    }

    #[test]
    fn move_ri_rotate_right_multiple_times() {
        let mut cube = Cube::new([BLUE, RED, YELLOW, ORANGE, WHITE, GREEN]);
        for _ in 0..6 {
            cube.move_r();
            cube.rotate_right();
        }
        println!("scrambled!");
        cube.print();
        let prefix = "    ";
        let expected= format!("{}|RWW|\n\
            {}|BGG|\n\
            {}|OYY|\n\
        |WOG|WRR|GWB|OOB|\n\
        |WOY|GYB|YRR|GWB|\n\
        |GRY|BOY|OOB|YRR|\n\
        {}|OGG|\n\
        {}|BBY|\n\
        {}|WWR|\n", prefix, prefix, prefix, prefix, prefix, prefix);

        println!("expected {}", expected);
        assert_eq!(format!("{}", cube), expected);
    }

    #[test]
    fn mix_and_fix() {
        let cube = Cube::new([RED, YELLOW, WHITE, GREEN, BLUE, ORANGE]);
        let mut mixed_cube = cube.clone();
        let count = 15;

        for _ in 0..count {
            mixed_cube.orientation_rotate_left();
            mixed_cube.move_r();
        }
        for _ in 0..count {
            mixed_cube.move_ri();
            mixed_cube.orientation_rotate_right();
        }
        assert_eq!(format!("{}", cube), format!("{}", mixed_cube));
    }
}
