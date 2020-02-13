use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdgePiece {
    pub color : Color,
    pub face_orientation : FaceOrientation,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub side1 : EdgePiece,
    pub side2 : EdgePiece,
}

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
    pub fn from_index(i : usize) -> FaceOrientation {
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
pub enum RotationDirection {
    RIGHT,
    LEFT,
    DOWN,
    UP,
}

