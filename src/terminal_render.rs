use crate::cube::Color;
use crate::array_cube::Cube;
use ansi_term;

#[derive(Copy, Clone)]
struct TerminalFormatter {
    cube : Cube
}

fn ansi_format_convert(color : ansi_term::Colour) -> ansi_term::ANSIString<'static> {
    ansi_term::Style::new().on(color).fg(ansi_term::Colour::Black).paint("  ")
}

fn ansi_prefix<'a>() -> ansi_term::ANSIString<'a> {
    ansi_term::Style::new().paint("      ")
}
fn ansi_newline<'a>() -> ansi_term::ANSIString<'a> {
    ansi_term::Style::new().paint("\n")
}
pub fn ansi_print(cube : &Cube) {
    //print!("{}", self.ansi_format())
    let formatter = TerminalFormatter {
        cube: *cube,
    };
    formatter.ansi_format();
    //self.print();
}

impl TerminalFormatter {


fn find_ansi_string(self , face : usize, x : usize, y : usize) -> ansi_term::ANSIString<'static> {
    ansi_format_convert(match self.cube.faces[face].squares[x][y] {
        Color::RED => ansi_term::Colour::RGB(255, 0, 0),
        Color::YELLOW => ansi_term::Colour::RGB(255, 255, 0),
        Color::WHITE => ansi_term::Colour::White,
        Color::GREEN => ansi_term::Colour::RGB(0, 255, 0),
        Color::BLUE => ansi_term::Colour::RGB(0, 0, 255),
        Color::ORANGE => ansi_term::Colour::RGB(255, 100, 0),
    })
}

fn ansi_format(&self) {
    let strings : &[ansi_term::ANSIString<'static>] = &[
        ansi_prefix(), self.find_ansi_string(0, 0, 0), self.find_ansi_string(0, 0, 1), self.find_ansi_string(0, 0, 2), ansi_newline(),
        ansi_prefix(), self.find_ansi_string(0, 1, 0), self.find_ansi_string(0, 1, 1), self.find_ansi_string(0, 1, 2), ansi_newline(),
        ansi_prefix(), self.find_ansi_string(0, 2, 0), self.find_ansi_string(0, 2, 1), self.find_ansi_string(0, 2, 2), ansi_newline(),
        //first long row
        self.find_ansi_string(1, 0, 0), self.find_ansi_string(1, 0, 1), self.find_ansi_string(1, 0, 2),
        self.find_ansi_string(2, 0, 0), self.find_ansi_string(2, 0, 1), self.find_ansi_string(2, 0, 2),
        self.find_ansi_string(3, 0, 0), self.find_ansi_string(3, 0, 1), self.find_ansi_string(3, 0, 2),
        self.find_ansi_string(4, 0, 0), self.find_ansi_string(4, 0, 1), self.find_ansi_string(4, 0, 2),
        ansi_newline(),
        //second long row
        self.find_ansi_string(1, 1, 0), self.find_ansi_string(1, 1, 1), self.find_ansi_string(1, 1, 2),
        self.find_ansi_string(2, 1, 0), self.find_ansi_string(2, 1, 1), self.find_ansi_string(2, 1, 2),
        self.find_ansi_string(3, 1, 0), self.find_ansi_string(3, 1, 1), self.find_ansi_string(3, 1, 2),
        self.find_ansi_string(4, 1, 0), self.find_ansi_string(4, 1, 1), self.find_ansi_string(4, 1, 2),
        ansi_newline(),
        //third long row
        self.find_ansi_string(1, 2, 0), self.find_ansi_string(1, 2, 1), self.find_ansi_string(1, 2, 2),
        self.find_ansi_string(2, 2, 0), self.find_ansi_string(2, 2, 1), self.find_ansi_string(2, 2, 2),
        self.find_ansi_string(3, 2, 0), self.find_ansi_string(3, 2, 1), self.find_ansi_string(3, 2, 2),
        self.find_ansi_string(4, 2, 0), self.find_ansi_string(4, 2, 1), self.find_ansi_string(4, 2, 2),
        ansi_newline(),
        //last row
        ansi_prefix(), self.find_ansi_string(5, 0, 0), self.find_ansi_string(5, 0, 1), self.find_ansi_string(5, 0, 2), ansi_newline(),
        ansi_prefix(), self.find_ansi_string(5, 1, 0), self.find_ansi_string(5, 1, 1), self.find_ansi_string(5, 1, 2), ansi_newline(),
        ansi_prefix(), self.find_ansi_string(5, 2, 0), self.find_ansi_string(5, 2, 1), self.find_ansi_string(5, 2, 2)
            ];
    println!("{}", ansi_term::ANSIStrings(strings));
}

}
