use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    RED, YELLOW, WHITE, GREEN, BLUE, ORANGE,
}

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

/*
enum Movement {
    R, RI,
    L, LI,
    B, BI,
    D, DI,
    F, FI,
    U, UI,
}
*/

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Side {
    squares: [[Color; 3]; 3 ]
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cube {
    sides : [Side; 6]
}

impl Side {
    fn new(color : Color) -> Side {
        Side {
            squares: [[color, color, color],
                [color, color, color],
                [color, color, color]]
        }
    }
}

impl Cube {

    pub fn new(sides : [Color; 6]) -> Self {
        Cube {
            sides: [
                Side::new(sides[0]),
                Side::new(sides[1]),
                Side::new(sides[2]),
                Side::new(sides[3]),
                Side::new(sides[4]),
                Side::new(sides[5])
            ]
        }
    }

    pub fn default() -> Self {
        Cube {
            sides: [
                Side::new(Color::RED),
                Side::new(Color::YELLOW),
                Side::new(Color::WHITE),
                Side::new(Color::GREEN),
                Side::new(Color::BLUE),
                Side::new(Color::ORANGE)
            ]
        }
    }

    pub fn orientation_rotate_right(&mut self) {
        //rotate the cube so the side to the right is now the center cube
        //   1           1
        //  2345   ---> 3452
        //   6           6
        let side2 = self.sides[1];
        let side3 = self.sides[2];
        let side4 = self.sides[3];
        let side5 = self.sides[4];
        self.sides[1] = side3;
        self.sides[2] = side4;
        self.sides[3] = side5;
        self.sides[4] = side2;
    }

    pub fn orientation_rotate_left(&mut self) {
        self.orientation_rotate_right();
        self.orientation_rotate_right();
        self.orientation_rotate_right();
    }

    pub fn orientation_rotate_up(&mut self) {
        //rotate the cube so the side to the right is now the center cube
        //   1           3
        //  2345   ---> 2645
        //   6           1
        let side1 = self.sides[0];
        let side3 = self.sides[2];
        let side6 = self.sides[5];
        self.sides[0] = side3;
        self.sides[2] = side6;
        self.sides[5] = side1;
    }

    pub fn orientation_rotate_down(&mut self) {
        self.orientation_rotate_up();
        self.orientation_rotate_up();
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
        let left1 = self.sides[1].squares[0][2];
        let left2 = self.sides[1].squares[1][2];
        let left3 = self.sides[1].squares[2][2];

        let up1 = self.sides[0].squares[2][0];
        let up2 = self.sides[0].squares[2][1];
        let up3 = self.sides[0].squares[2][2];

        let right1 = self.sides[3].squares[0][0];
        let right2 = self.sides[3].squares[1][0];
        let right3 = self.sides[3].squares[2][0];

        let bottom1 = self.sides[5].squares[0][0];
        let bottom2 = self.sides[5].squares[0][1];
        let bottom3 = self.sides[5].squares[0][2];

        self.sides[1].squares[0][2] = bottom1;
        self.sides[1].squares[1][2] = bottom2;
        self.sides[1].squares[2][2] = bottom3;

        self.sides[0].squares[2][0] = left3;
        self.sides[0].squares[2][1] = left2;
        self.sides[0].squares[2][2] = left1;

        self.sides[3].squares[0][0] = up1;
        self.sides[3].squares[1][0] = up2;
        self.sides[3].squares[2][0] = up3;

        self.sides[5].squares[0][0] = right3;
        self.sides[5].squares[0][1] = right2;
        self.sides[5].squares[0][2] = right1;
    }

    pub fn move_fi(&mut self) {
        self.move_f();
        self.move_f();
        self.move_f();
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
            prefix, self.sides[0].squares[0][0], self.sides[0].squares[0][1], self.sides[0].squares[0][2],
            prefix, self.sides[0].squares[1][0], self.sides[0].squares[1][1], self.sides[0].squares[1][2],
            prefix, self.sides[0].squares[2][0], self.sides[0].squares[2][1], self.sides[0].squares[2][2],
            //first long row
            self.sides[1].squares[0][0], self.sides[1].squares[0][1], self.sides[1].squares[0][2],
            self.sides[2].squares[0][0], self.sides[2].squares[0][1], self.sides[2].squares[0][2],
            self.sides[3].squares[0][0], self.sides[3].squares[0][1], self.sides[3].squares[0][2],
            self.sides[4].squares[0][0], self.sides[4].squares[0][1], self.sides[4].squares[0][2],
            //second long row
            self.sides[1].squares[1][0], self.sides[1].squares[1][1], self.sides[1].squares[1][2],
            self.sides[2].squares[1][0], self.sides[2].squares[1][1], self.sides[2].squares[1][2],
            self.sides[3].squares[1][0], self.sides[3].squares[1][1], self.sides[3].squares[1][2],
            self.sides[4].squares[1][0], self.sides[4].squares[1][1], self.sides[4].squares[1][2],
            //third long row
            self.sides[1].squares[2][0], self.sides[1].squares[1][1], self.sides[1].squares[2][2],
            self.sides[2].squares[2][0], self.sides[2].squares[1][1], self.sides[2].squares[2][2],
            self.sides[3].squares[2][0], self.sides[3].squares[1][1], self.sides[3].squares[2][2],
            self.sides[4].squares[2][0], self.sides[4].squares[1][1], self.sides[4].squares[2][2],
            //last row
            prefix, self.sides[5].squares[0][0], self.sides[5].squares[0][1], self.sides[5].squares[0][2],
            prefix, self.sides[5].squares[1][0], self.sides[5].squares[1][1], self.sides[5].squares[1][2],
            prefix, self.sides[5].squares[2][0], self.sides[5].squares[2][1], self.sides[5].squares[2][2]
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
    fn rotate_right() {
        let mut cube = Cube::new([RED, YELLOW, WHITE, GREEN, BLUE, ORANGE]);
        cube.orientation_rotate_right();
        //   1           1
        //  2345   ---> 3452
        //   6           6
        assert_eq!(
            format!("{}", cube),
            format!("{}", Cube::new([RED, WHITE, GREEN, BLUE, YELLOW, ORANGE])),
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

}
