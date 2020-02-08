use std::fmt;
use crate::cube::Edge;
use crate::cube::EdgePiece;
use crate::cube::Color;
use crate::cube::Color::*;
use crate::cube::FaceOrientation;
use crate::cube::FaceOrientation::*;
use crate::cube::RotationDirection;

const UP_FACE : usize = 0;
const LEFT_FACE : usize = 1;
const FRONT_FACE : usize = 2;
const RIGHT_FACE : usize = 3;
const BACK_FACE : usize = 4;
const DOWN_FACE : usize = 5;

#[derive(Clone, Copy, Debug, PartialEq)]
struct ArrayFace {
    squares: [[Color; 3]; 3 ]
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cube {
    faces : [ArrayFace; 6]
}

impl ArrayFace {
    fn new(color : Color) -> ArrayFace {
        ArrayFace {
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
                ArrayFace::new(faces[0]),
                ArrayFace::new(faces[1]),
                ArrayFace::new(faces[2]),
                ArrayFace::new(faces[3]),
                ArrayFace::new(faces[4]),
                ArrayFace::new(faces[5])
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

    fn find_face(self, face_orientation : FaceOrientation) -> ArrayFace {
        match face_orientation {
            RIGHT => self.faces[3],
            LEFT => self.faces[1],
            BACK => self.faces[4],
            DOWN => self.faces[5],
            FRONT => self.faces[2],
            UP => self.faces[0],
        }
    }

    fn find_adjacent_edge_piece(self, face_orientation : FaceOrientation, x : usize, y : usize) -> EdgePiece {
        let (adjacent_face_orientation, adjacent_x, adjacent_y) = match (face_orientation, x, y) {
            (UP, 0, 1) => (BACK, 0, 1),
            (UP, 1, 0) => (LEFT, 0, 1),
            (UP, 1, 2) => (RIGHT, 0, 1),
            (UP, 2, 1) => (FRONT, 0, 1),
            (LEFT, 0, 1) => (UP, 1, 0),
            (LEFT, 1, 0) => (BACK, 1, 2),
            (LEFT, 1, 2) => (FRONT, 1, 0),
            (LEFT, 2, 1) => (DOWN, 0, 1),
            _ => (UP, 0, 1)
        };
        EdgePiece {
            face_orientation: adjacent_face_orientation,
            color: self.find_face(adjacent_face_orientation).squares[adjacent_x][adjacent_y],
        }
    }

    pub fn find_edge(self, color1 : Color, color2 : Color) -> Edge {
        for face_index in 0..5 {
            let face = self.faces[face_index];
            let face_orientation = FaceOrientation::fromIndex(face_index);
            for square in vec![[0, 1], [1, 0], [1, 2], [2, 1]] {
                let square_color = face.squares[square[0]][square[1]];
                let adjacent_piece = self.find_adjacent_edge_piece(face_orientation, square[0], square[1]);

                if square_color == color1 && adjacent_piece.color  == color2 {
                    return Edge {
                        side1: EdgePiece {
                            color: square_color,
                            face_orientation: face_orientation
                        },
                        side2: adjacent_piece
                    }
                } else if square_color == color2 && adjacent_piece.color == color1 {
                    return Edge {
                        side2: EdgePiece {
                            color: square_color,
                            face_orientation: face_orientation
                        },
                        side1: adjacent_piece
                    }
                }

            }
        }
        panic!("could not find edge");
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
        //  2345   ---> 2641
        //   6           5
        let face1 = self.faces[0];
        let face3 = self.faces[2];
        let face5 = self.faces[4];
        let face6 = self.faces[5];
        self.faces[0] = face3;
        self.faces[2] = face6;
        self.faces[4] = face1;
        self.faces[5] = face5;

        self.faces[1].rotate_right();
        self.faces[3].rotate_right();
    }

    pub fn orientation_rotate_down(&mut self) {
        self.orientation_rotate_up();
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

        let down1 = self.faces[5].squares[0][0];
        let down2 = self.faces[5].squares[0][1];
        let down3 = self.faces[5].squares[0][2];

        self.faces[1].squares[0][2] = down1;
        self.faces[1].squares[1][2] = down2;
        self.faces[1].squares[2][2] = down3;

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

    pub fn move_ui(&mut self) {
        self.move_u();
        self.move_u();
        self.move_u();
    }

    pub fn move_u(&mut self) {
        //    |RRR|                 |RRR|
        //    |RRR|                 |RRR|
        //    |RRR|                 |RRR|
        //|BBB|WWW|GGG|YYY|     |WWW|GGG|YYY|BBB|
        //|BBB|WWW|GGG|YYY| --> |BBB|WWW|GGG|YYY|
        //|BBB|WWW|GGG|YYY|     |BBB|WWW|GGG|YYY|
        //    |OOO|                 |OOO|
        //    |OOO|                 |OOO|
        //    |OOO|                 |OOO|
        let front0 = self.faces[FRONT_FACE].squares[0][0];
        let front1 = self.faces[FRONT_FACE].squares[0][1];
        let front2 = self.faces[FRONT_FACE].squares[0][2];

        let left0 = self.faces[LEFT_FACE].squares[0][0];
        let left1 = self.faces[LEFT_FACE].squares[0][1];
        let left2 = self.faces[LEFT_FACE].squares[0][2];

        let back0 = self.faces[BACK_FACE].squares[0][0];
        let back1 = self.faces[BACK_FACE].squares[0][1];
        let back2 = self.faces[BACK_FACE].squares[0][2];

        let right0 = self.faces[RIGHT_FACE].squares[0][0];
        let right1 = self.faces[RIGHT_FACE].squares[0][1];
        let right2 = self.faces[RIGHT_FACE].squares[0][2];

        self.faces[LEFT_FACE].squares[0][0] = front0;
        self.faces[LEFT_FACE].squares[0][1] = front1;
        self.faces[LEFT_FACE].squares[0][2] = front2;

        self.faces[FRONT_FACE].squares[0][0] = right0;
        self.faces[FRONT_FACE].squares[0][1] = right1;
        self.faces[FRONT_FACE].squares[0][2] = right2;

        self.faces[RIGHT_FACE].squares[0][0] = back0;
        self.faces[RIGHT_FACE].squares[0][1] = back1;
        self.faces[RIGHT_FACE].squares[0][2] = back2;

        self.faces[BACK_FACE].squares[0][0] = left0;
        self.faces[BACK_FACE].squares[0][1] = left1;
        self.faces[BACK_FACE].squares[0][2] = left2;

        self.faces[UP_FACE].rotate_left();
    }
    pub fn move_di(&mut self) {
        self.move_d();
        self.move_d();
        self.move_d();
    }
    pub fn move_d(&mut self) {
        //    |RRR|                 |RRR|
        //    |RRR|                 |RRR|
        //    |RRR|                 |RRR|
        //|BBB|WWW|GGG|YYY|     |BBB|WWW|GGG|YYY|
        //|BBB|WWW|GGG|YYY| --> |BBB|WWW|GGG|YYY|
        //|BBB|WWW|GGG|YYY|     |WWW|GGG|YYY|BBB|
        //    |OOO|                 |OOO|
        //    |OOO|                 |OOO|
        //    |OOO|                 |OOO|
        let front0 = self.faces[FRONT_FACE].squares[2][0];
        let front1 = self.faces[FRONT_FACE].squares[2][1];
        let front2 = self.faces[FRONT_FACE].squares[2][2];

        let left0 = self.faces[LEFT_FACE].squares[2][0];
        let left1 = self.faces[LEFT_FACE].squares[2][1];
        let left2 = self.faces[LEFT_FACE].squares[2][2];

        let back0 = self.faces[BACK_FACE].squares[2][0];
        let back1 = self.faces[BACK_FACE].squares[2][1];
        let back2 = self.faces[BACK_FACE].squares[2][2];

        let right0 = self.faces[RIGHT_FACE].squares[2][0];
        let right1 = self.faces[RIGHT_FACE].squares[2][1];
        let right2 = self.faces[RIGHT_FACE].squares[2][2];

        self.faces[LEFT_FACE].squares[2][0] = front0;
        self.faces[LEFT_FACE].squares[2][1] = front1;
        self.faces[LEFT_FACE].squares[2][2] = front2;

        self.faces[FRONT_FACE].squares[2][0] = right0;
        self.faces[FRONT_FACE].squares[2][1] = right1;
        self.faces[FRONT_FACE].squares[2][2] = right2;

        self.faces[RIGHT_FACE].squares[2][0] = back0;
        self.faces[RIGHT_FACE].squares[2][1] = back1;
        self.faces[RIGHT_FACE].squares[2][2] = back2;

        self.faces[BACK_FACE].squares[2][0] = left0;
        self.faces[BACK_FACE].squares[2][1] = left1;
        self.faces[BACK_FACE].squares[2][2] = left2;

        self.faces[DOWN_FACE].rotate_left();
    }
    pub fn move_bi(&mut self) {
        self.move_b();
        self.move_b();
        self.move_b();
    }
    pub fn move_b(&mut self) {
        //    |RRR|                 |GGG|
        //    |RRR|                 |RRR|
        //    |RRR|                 |RRR|
        //|BBB|WWW|GGG|YYY|     |RBB|WWW|GGG|YYO|
        //|BBB|WWW|GGG|YYY| --> |RBB|WWW|GGG|YYO|
        //|BBB|WWW|GGG|YYY|     |RBB|WWW|GGG|YYO|
        //    |OOO|                 |OOO|
        //    |OOO|                 |OOO|
        //    |OOO|                 |BBB|
        let up0 = self.faces[UP_FACE].squares[0][0];
        let up1 = self.faces[UP_FACE].squares[0][1];
        let up2 = self.faces[UP_FACE].squares[0][2];

        let left0 = self.faces[LEFT_FACE].squares[0][0];
        let left1 = self.faces[LEFT_FACE].squares[1][0];
        let left2 = self.faces[LEFT_FACE].squares[2][0];

        let down0 = self.faces[DOWN_FACE].squares[2][0];
        let down1 = self.faces[DOWN_FACE].squares[2][1];
        let down2 = self.faces[DOWN_FACE].squares[2][2];

        let right0 = self.faces[RIGHT_FACE].squares[0][2];
        let right1 = self.faces[RIGHT_FACE].squares[1][2];
        let right2 = self.faces[RIGHT_FACE].squares[2][2];

        self.faces[LEFT_FACE].squares[0][0] = up0;
        self.faces[LEFT_FACE].squares[1][0] = up1;
        self.faces[LEFT_FACE].squares[2][0] = up2;

        self.faces[UP_FACE].squares[0][0] = right0;
        self.faces[UP_FACE].squares[0][1] = right1;
        self.faces[UP_FACE].squares[0][2] = right2;

        self.faces[RIGHT_FACE].squares[0][2] = down0;
        self.faces[RIGHT_FACE].squares[1][2] = down1;
        self.faces[RIGHT_FACE].squares[2][2] = down2;

        self.faces[DOWN_FACE].squares[2][0] = left0;
        self.faces[DOWN_FACE].squares[2][1] = left1;
        self.faces[DOWN_FACE].squares[2][2] = left2;

        self.faces[BACK_FACE].rotate_left();
    }

    pub fn move_li(&mut self) {
        self.move_l();
        self.move_l();
        self.move_l();
    }

    pub fn move_l(&mut self) {
        //    |RRR|                 |YRR|
        //    |RRR|                 |YRR|
        //    |RRR|                 |YRR|
        //|BBB|WWW|GGG|YYY|     |BBB|RWW|GGG|YYO|
        //|BBB|WWW|GGG|YYY| --> |BBB|RWW|GGG|YYO|
        //|BBB|WWW|GGG|YYY|     |BBB|RWW|GGG|YYO|
        //    |OOO|                 |WOO|
        //    |OOO|                 |WOO|
        //    |OOO|                 |WOO|
        let up0 = self.faces[UP_FACE].squares[0][0];
        let up1 = self.faces[UP_FACE].squares[1][0];
        let up2 = self.faces[UP_FACE].squares[2][0];

        let back0 = self.faces[BACK_FACE].squares[0][0];
        let back1 = self.faces[BACK_FACE].squares[1][0];
        let back2 = self.faces[BACK_FACE].squares[2][0];

        let down0 = self.faces[DOWN_FACE].squares[0][0];
        let down1 = self.faces[DOWN_FACE].squares[1][0];
        let down2 = self.faces[DOWN_FACE].squares[2][0];

        let front0 = self.faces[FRONT_FACE].squares[0][0];
        let front1 = self.faces[FRONT_FACE].squares[1][0];
        let front2 = self.faces[FRONT_FACE].squares[2][0];

        self.faces[FRONT_FACE].squares[0][0] = up0;
        self.faces[FRONT_FACE].squares[1][0] = up1;
        self.faces[FRONT_FACE].squares[2][0] = up2;

        self.faces[UP_FACE].squares[0][0] = back2;
        self.faces[UP_FACE].squares[1][0] = back1;
        self.faces[UP_FACE].squares[2][0] = back0;

        self.faces[DOWN_FACE].squares[0][0] = front0;
        self.faces[DOWN_FACE].squares[1][0] = front1;
        self.faces[DOWN_FACE].squares[2][0] = front2;

        self.faces[BACK_FACE].squares[0][0] = down0;
        self.faces[BACK_FACE].squares[1][0] = down1;
        self.faces[BACK_FACE].squares[2][0] = down2;

        self.faces[LEFT_FACE].rotate_right();
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

        let down1 = self.faces[5].squares[0][2];
        let down2 = self.faces[5].squares[1][2];
        let down3 = self.faces[5].squares[2][2];

        let center02 = self.faces[2].squares[0][2];
        let center12 = self.faces[2].squares[1][2];
        let center22 = self.faces[2].squares[2][2];

        self.faces[2].squares[0][2] = down1;
        self.faces[2].squares[1][2] = down2;
        self.faces[2].squares[2][2] = down3;

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

    pub fn twist(&mut self, face_orientation : FaceOrientation) {
        match face_orientation {
            RIGHT => self.move_r(),
            LEFT => self.move_l(),
            BACK => self.move_b(),
            DOWN => self.move_d(),
            FRONT => self.move_f(),
            UP => self.move_u(),
        }
    }

    pub fn rotate(&mut self, rotation_direction : RotationDirection) {
        match rotation_direction {
            RotationDirection::RIGHT => self.orientation_rotate_right(),
            RotationDirection::LEFT => self.orientation_rotate_left(),
            RotationDirection::DOWN => self.orientation_rotate_down(),
            RotationDirection::UP => self.orientation_rotate_up()
        }
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
        for i in 0..5 {
            if self.faces[i].squares[1][1] == color {
                return FaceOrientation::fromIndex(i)
            }
        }
        println!("could not find color {}", color);
        return FaceOrientation::DOWN;
    }

    pub fn print(&self) {
        print!("{}", self)
    }

    fn ansi_format_convert(color : ansi_term::Colour) -> ansi_term::ANSIString<'static> {
            ansi_term::Style::new().on(color).fg(ansi_term::Colour::Black).paint("  ")
    }

    fn find_ansi_string(&self, face : usize, x : usize, y : usize) -> ansi_term::ANSIString<'static> {
        Cube::ansi_format_convert(match self.faces[face].squares[x][y] {
            Color::RED => ansi_term::Colour::RGB(255, 0, 0),
            Color::YELLOW => ansi_term::Colour::RGB(255, 255, 0),
            Color::WHITE => ansi_term::Colour::White,
            Color::GREEN => ansi_term::Colour::RGB(0, 255, 0),
            Color::BLUE => ansi_term::Colour::RGB(0, 0, 255),
            Color::ORANGE => ansi_term::Colour::RGB(255, 100, 0),
        })
    }

    pub fn ansi_print(&self) {
        //print!("{}", self.ansi_format())
        self.ansi_format();
        //self.print();
    }

    fn ansi_prefix<'a>() -> ansi_term::ANSIString<'a> {
        ansi_term::Style::new().paint("      ")
    }
    fn ansi_newline<'a>() -> ansi_term::ANSIString<'a> {
        ansi_term::Style::new().paint("\n")
    }
    fn ansi_format(&self) {
        let strings : &[ansi_term::ANSIString<'static>] = &[
        Cube::ansi_prefix(), self.find_ansi_string(0, 0, 0), self.find_ansi_string(0, 0, 1), self.find_ansi_string(0, 0, 2), Cube::ansi_newline(),
        Cube::ansi_prefix(), self.find_ansi_string(0, 1, 0), self.find_ansi_string(0, 1, 1), self.find_ansi_string(0, 1, 2), Cube::ansi_newline(),
        Cube::ansi_prefix(), self.find_ansi_string(0, 2, 0), self.find_ansi_string(0, 2, 1), self.find_ansi_string(0, 2, 2), Cube::ansi_newline(),
        //first long row
        self.find_ansi_string(1, 0, 0), self.find_ansi_string(1, 0, 1), self.find_ansi_string(1, 0, 2),
        self.find_ansi_string(2, 0, 0), self.find_ansi_string(2, 0, 1), self.find_ansi_string(2, 0, 2),
        self.find_ansi_string(3, 0, 0), self.find_ansi_string(3, 0, 1), self.find_ansi_string(3, 0, 2),
        self.find_ansi_string(4, 0, 0), self.find_ansi_string(4, 0, 1), self.find_ansi_string(4, 0, 2),
        Cube::ansi_newline(),
        //second long row
        self.find_ansi_string(1, 1, 0), self.find_ansi_string(1, 1, 1), self.find_ansi_string(1, 1, 2),
        self.find_ansi_string(2, 1, 0), self.find_ansi_string(2, 1, 1), self.find_ansi_string(2, 1, 2),
        self.find_ansi_string(3, 1, 0), self.find_ansi_string(3, 1, 1), self.find_ansi_string(3, 1, 2),
        self.find_ansi_string(4, 1, 0), self.find_ansi_string(4, 1, 1), self.find_ansi_string(4, 1, 2),
        Cube::ansi_newline(),
        //third long row
        self.find_ansi_string(1, 2, 0), self.find_ansi_string(1, 2, 1), self.find_ansi_string(1, 2, 2),
        self.find_ansi_string(2, 2, 0), self.find_ansi_string(2, 2, 1), self.find_ansi_string(2, 2, 2),
        self.find_ansi_string(3, 2, 0), self.find_ansi_string(3, 2, 1), self.find_ansi_string(3, 2, 2),
        self.find_ansi_string(4, 2, 0), self.find_ansi_string(4, 2, 1), self.find_ansi_string(4, 2, 2),
        Cube::ansi_newline(),
        //last row
        Cube::ansi_prefix(), self.find_ansi_string(5, 0, 0), self.find_ansi_string(5, 0, 1), self.find_ansi_string(5, 0, 2), Cube::ansi_newline(),
        Cube::ansi_prefix(), self.find_ansi_string(5, 1, 0), self.find_ansi_string(5, 1, 1), self.find_ansi_string(5, 1, 2), Cube::ansi_newline(),
        Cube::ansi_prefix(), self.find_ansi_string(5, 2, 0), self.find_ansi_string(5, 2, 1), self.find_ansi_string(5, 2, 2)
            ];
        println!("{}", ansi_term::ANSIStrings(strings));
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

    mod edges {
        use super::super::*;
        macro_rules! find_edge_tests {
            ($($name:ident: $value:expr,)*) => {
                $(
                    #[test]
                    fn $name() {
                        let cube = Cube::new([BLUE, ORANGE, WHITE, RED, YELLOW, GREEN]);
                        let expected = $value;
                        assert_eq!(expected, cube.find_edge(expected.side1.color, expected.side2.color));
                    }
                )*
            }
        }

        find_edge_tests! {
                edge_blue_orange: Edge {
                        side1: EdgePiece {
                                color: BLUE,
                                face_orientation: UP,
                        },
                        side2: EdgePiece {
                                color: ORANGE,
                                face_orientation: LEFT,
                        },
                },
                edge_blue_white: Edge {
                        side1: EdgePiece {
                                color: BLUE,
                                face_orientation: UP,
                        },
                        side2: EdgePiece {
                                color: WHITE,
                                face_orientation: FRONT,
                        },
                },
                edge_blue_red: Edge {
                        side1: EdgePiece {
                                color: BLUE,
                                face_orientation: UP,
                        },
                        side2: EdgePiece {
                                color: RED,
                                face_orientation: RIGHT,
                        },
                },
                edge_blue_yellow: Edge {
                        side1: EdgePiece {
                                color: BLUE,
                                face_orientation: UP,
                        },
                        side2: EdgePiece {
                                color: YELLOW,
                                face_orientation: BACK,
                        },
                },
        }
        /*
        #[test]
        fn find_edge() {
            test_edges(cube, (ORANGE, LEFT, BLUE, UP));
            test_edges(cube, (BLUE, UP, ORANGE, LEFT));
            test_edges(cube, (BLUE, UP, WHITE, FRONT));
            test_edges(cube, (BLUE, UP, RED, RIGHT));
            test_edges(cube, (BLUE, UP, YELLOW, BACK));
        }
        */
    }

    #[test]
    fn rotate_down() {
        //down should undo up
        let cube = Cube::new([BLUE, ORANGE, WHITE, RED, YELLOW, GREEN]);
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
        let mut cube = Cube::new([RED, BLUE, WHITE, GREEN, YELLOW, ORANGE]);
        cube.orientation_rotate_up();
        assert_eq!(
            format!("{}", cube),
            format!("{}", Cube::new([WHITE, BLUE, ORANGE, GREEN, RED, YELLOW])),
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
    fn mix_and_fix_r() {
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

    #[test]
    fn mix_and_fix_l() {
        let cube = Cube::new([RED, YELLOW, BLUE, WHITE, GREEN, ORANGE]);
        let mut mixed_cube = cube.clone();
        let count = 15;

        for _ in 0..count {
            mixed_cube.orientation_rotate_left();
            mixed_cube.move_l();
        }
        for _ in 0..count {
            mixed_cube.move_li();
            mixed_cube.orientation_rotate_right();
        }
        assert_eq!(format!("{}", cube), format!("{}", mixed_cube));
    }
    #[test]
    fn mix_and_fix_f() {
        let cube = Cube::new([RED, YELLOW, BLUE, WHITE, GREEN, ORANGE]);
        let mut mixed_cube = cube.clone();
        let count = 15;

        for _ in 0..count {
            mixed_cube.orientation_rotate_left();
            mixed_cube.move_f();
        }
        for _ in 0..count {
            mixed_cube.move_fi();
            mixed_cube.orientation_rotate_right();
        }
        assert_eq!(format!("{}", cube), format!("{}", mixed_cube));
    }

    #[test]
    fn mix_and_fix_b() {
        let cube = Cube::new([RED, YELLOW, BLUE, WHITE, GREEN, ORANGE]);
        let mut mixed_cube = cube.clone();
        let count = 15;

        for _ in 0..count {
            mixed_cube.orientation_rotate_left();
            mixed_cube.move_b();
        }
        for _ in 0..count {
            mixed_cube.move_bi();
            mixed_cube.orientation_rotate_right();
        }
        assert_eq!(format!("{}", cube), format!("{}", mixed_cube));
    }

    #[test]
    fn mix_and_fix_u() {
        let cube = Cube::new([RED, YELLOW, BLUE, WHITE, GREEN, ORANGE]);
        let mut mixed_cube = cube.clone();
        let count = 15;

        for _ in 0..count {
            mixed_cube.orientation_rotate_left();
            mixed_cube.move_u();
        }
        for _ in 0..count {
            mixed_cube.move_ui();
            mixed_cube.orientation_rotate_right();
        }
        assert_eq!(format!("{}", cube), format!("{}", mixed_cube));
    }

    #[test]
    fn mix_and_fix_d() {
        let cube = Cube::new([RED, YELLOW, BLUE, WHITE, GREEN, ORANGE]);
        let mut mixed_cube = cube.clone();
        let count = 15;

        for _ in 0..count {
            mixed_cube.orientation_rotate_left();
            mixed_cube.move_d();
        }
        for _ in 0..count {
            mixed_cube.move_di();
            mixed_cube.orientation_rotate_right();
        }
        assert_eq!(format!("{}", cube), format!("{}", mixed_cube));
    }
}

