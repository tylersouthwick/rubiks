//implementing https://cube20.org/src/cubepos.pdf
use lazy_static;

const NMOVES : usize = 18;
const TWISTS : usize = 3;
const FACES : usize = 6;
const M : u8= 48;
const CUBIES : usize = 24;

type Twist = u8;
const NO_TWIST : Twist = 0;
const CLOCKWISE : Twist = 1;
const COUNTER_CLOCKWISE : Twist = 2;

const faces : [char ; FACES] = ['U','F','R','D','B','L'];

/*
#[derive(PartialEq, PartialOrd)]
struct Cubie {
    slot : u8,
    twist : Twist,
}
*/
type Cubie = usize;

/*
#[derive(PartialEq, PartialOrd)]
struct Edge {
    slot : u8,
    flipped : bool,
}
*/
type Edge = usize;

#[derive(Debug, PartialEq)]
struct CubieCube {
    cubies : [Cubie; 8],
    edges : [Edge; 12],
}
impl CubieCube {
    fn new() -> Self {
        let mut cubies : [Cubie; 8] = [0; 8];
        let mut edges: [Edge; 12] = [0; 12];
        for i in 0..8 {
            cubies[i] = corner_val(i, 0);
        }
        for i in 0..12 {
            edges[i] = edge_val(i, 0);
        }
        CubieCube {
            cubies,
            edges,
        }
    }
    fn identity() -> Self {
        CubieCube::new()
    }

    fn invert_into(self, dst : &mut CubieCube) {
        for i in 0..8 {
            let cval = self.cubies[i];
            dst.cubies[corner_perm(cval)] = corner_ori_sub(i, cval);
        }
        for i in 0..12 {
            let eval = self.edges[i];
            dst.edges[edge_perm(eval)] = edge_val(i, edge_ori(eval));
        }
    }

    fn edge4flip(&mut self, a : usize, b : usize, c : usize, d : usize) {
        let t = self.edges[d];
        self.edges[d] = edge_flip(self.edges[c]);
        self.edges[c] = edge_flip(self.edges[b]);
        self.edges[b] = edge_flip(self.edges[a]);
        self.edges[a] = edge_flip(self.edges[d]);
    }
    fn corner4flip(&mut self, a : usize, b : usize, c : usize, d : usize) {
        let t = self.cubies[d];
        self.cubies[d] = cubie_data.corner_ori_inc[self.cubies[c]];
        self.cubies[c] = cubie_data.corner_ori_dec[self.cubies[b]];
        self.cubies[b] = cubie_data.corner_ori_inc[self.cubies[a]];
        self.cubies[a] = cubie_data.corner_ori_dec[self.cubies[d]];
    }

    fn _move(&mut self, mov : usize) {
        let p = cubie_data.corner_trans[mov] ;
        self.cubies[0] = p[self.cubies[0]] ;
        self.cubies[1] = p[self.cubies[1]] ;
        self.cubies[2] = p[self.cubies[2]] ;
        self.cubies[3] = p[self.cubies[3]] ;
        self.cubies[4] = p[self.cubies[4]] ;
        self.cubies[5] = p[self.cubies[5]] ;
        self.cubies[6] = p[self.cubies[6]] ;
        self.cubies[7] = p[self.cubies[7]] ;

        let edge_p = cubie_data.edge_trans[mov] ;
        self.edges[0] = edge_p[self.edges[0]] ;
        self.edges[1] = edge_p[self.edges[1]] ;
        self.edges[2] = edge_p[self.edges[2]] ;
        self.edges[3] = edge_p[self.edges[3]] ;
        self.edges[4] = edge_p[self.edges[4]] ;
        self.edges[5] = edge_p[self.edges[5]] ;
        self.edges[6] = edge_p[self.edges[6]] ;
        self.edges[7] = edge_p[self.edges[7]] ;
        self.edges[8] = edge_p[self.edges[8]] ;
        self.edges[9] = edge_p[self.edges[9]] ;
        self.edges[10] = edge_p[self.edges[10]] ;
        self.edges[11] = edge_p[self.edges[11]] ;
    }

    fn movepc(&mut self, mov : usize) {
        match mov {
            0 => {
                rot4(&mut self.edges, 0, 2, 3, 1);
                rot4(&mut self.cubies, 0, 1, 3, 2);
            },
            1=> {
                rot22(&mut self.edges,0,2,3,1);
                rot22(&mut self.cubies,0,1,3,2);
            },
            2=> {
                rot4(&mut self.edges,1,3,2,0); 
                rot4(&mut self.cubies,2,3,1,0);
            },
            3=> { 
                rot4(&mut self.edges,3,7,11,6); 
                self.corner4flip(3,7,6,2); 
            },
            4=> { 
                rot22(&mut self.edges,3,7,11,6); 
                rot22(&mut self.cubies,2,3,7,6);
            },
            5 => { 
                rot4(&mut self.edges,6,11,7,3); 
                self.corner4flip(3,2,6,7); 
            },
            6 => { 
                self.edge4flip(2,5,10,7); 
                self.corner4flip(1,5,7,3); 
            },
            7 => { 
                rot22(&mut self.edges,2,5,10,7); 
                rot22(&mut self.cubies,3,1,5,7); 
            },
            8 => { 
                self.edge4flip(7,10,5,2); 
                self.corner4flip(1,3,7,5); 
            },
            9 => { 
                rot4(&mut self.edges,9,11,10,8); 
                rot4(&mut self.cubies,4,6,7,5); 
            },
            10 => { 
                rot22(&mut self.edges,9,11,10,8); 
                rot22(&mut self.cubies,4,6,7,5); 
            },
            11 => { 
                rot4(&mut self.edges,8,10,11,9); 
                rot4(&mut self.cubies,5,7,6,4); 
            },
            12 => { 
                rot4(&mut self.edges,0,4,8,5); 
                self.corner4flip(0,4,5,1); 
            },
            13 => { 
                rot22(&mut self.edges,0,4,8,5); 
                rot22(&mut self.cubies,1,0,4,5); 
            },
            14 => { 
                rot4(&mut self.edges,5,8,4,0); 
                self.corner4flip(0,1,5,4); 
            },
            15 => { 
                self.edge4flip(1,6,9,4); 
                self.corner4flip(2,6,4,0); 
            },
            16 => { 
                rot22(&mut self.edges,1,6,9,4); 
                rot22(&mut self.cubies,0,2,6,4); 
            },
            17 => { 
                self.edge4flip(4,9,6,1); 
                self.corner4flip(2,0,4,6); 
            },
            _ => panic!("invalid mov: {}", mov)
        }
    }

}
fn mul(a : &CubieCube, b : &CubieCube, r : &mut CubieCube) {
    for i in 0..8 {
        let cc = a.cubies[i];
        r.cubies[i] = corner_ori_add(b.cubies[corner_perm(cc)], cc) ;
    }
    for i in 0..12 {
        let cc = a.edges[i] ;
        r.edges[i] = edge_ori_add(b.edges[edge_perm(cc)], cc) ;
    }
}

lazy_static! {
    static ref cubie_data : CubieData = CubieData::new();
}

fn rot2(cc : &mut [usize], a : usize, b : usize) {
    let t = cc[a];
    cc[a] = cc[b];
    cc[b] = t;
}

fn rot4(cc : &mut [usize], a : usize, b : usize, c : usize, d : usize) {
    let t = cc[d];
    cc[d] = cc[c];
    cc[c] = cc[b];
    cc[b] = cc[a];
    cc[a] = t;
}
fn rot22(cc : &mut [usize], a : usize, b : usize, c : usize, d : usize) {
    rot2(cc, a, c);
    rot2(cc, b, d);
}

fn edge_perm(cubieval : usize) -> usize {
    cubieval >> 1
}

fn edge_ori(cubieval : usize) -> usize {
    cubieval & 1
}
fn corner_perm(cubieval : usize) -> usize {
    cubieval & 7
}
fn corner_ori(cubieval : usize) -> usize {
    cubieval >> 3
}
fn edge_flip(cubieval : usize) -> usize {
    cubieval ^ 1
}
fn edge_val(perm : usize, ori : usize) -> usize {
    perm * 2 + ori
}
fn corner_val(perm : usize, ori: usize) -> usize {
    ori * 8 + perm
}
fn edge_ori_add(cv1 : usize, cv2 : usize) -> usize {
    (cv1 + (cv2 & 18)) % 24
}
fn corner_ori_sub(cv1 : usize, cv2 : usize) -> usize {
    cv1 + cubie_data.corner_ori_neg_strip[cv2]
}
fn corner_ori_add(cv1 : usize, cv2 : usize) -> usize {
    cubie_data.mod24[cv1 + (cv2 & 0x18)]
}

struct CubieData {
    corner_ori_inc : [usize ; CUBIES],
    corner_ori_dec : [usize ; CUBIES],
    corner_ori_neg_strip : [usize ; CUBIES],
    mod24 : [usize ; 2 * CUBIES],
    //edge_twist_perm : [ [usize; 4]; FACES],
    //corner_twist_perm : [ [usize; 4]; FACES],
    //edge_change : [usize; FACES],
    edge_trans : [[usize ; CUBIES] ; NMOVES],
    corner_trans : [[usize ; CUBIES] ; NMOVES],
    inv_move : [usize; NMOVES],
}

type moveseq = Vec<usize>;

impl CubieData {
    fn new() -> Self {
        let mut corner_ori_inc : [usize ; CUBIES] = [0; CUBIES];
        let mut corner_ori_dec : [usize ; CUBIES] = [0; CUBIES];
        let mut corner_ori_neg_strip : [usize ; CUBIES] = [0; CUBIES];
        let mut mod24 : [usize ; 2 * CUBIES] = [0; 2 * CUBIES];
        for i in 0..CUBIES {
            let perm = corner_perm(i);
            let ori = corner_ori(i);
            corner_ori_inc[i] = corner_val(perm, (ori + 1) % 3);
            corner_ori_dec[i] = corner_val(perm, (ori + 2) % 3);
            corner_ori_neg_strip[i] = corner_val(0, (3 - ori) % 3);
            mod24[i] = i;
            mod24[i + CUBIES] = i;
        }
        let edge_twist_perm : [ [usize; 4]; FACES] = [
            [0, 2, 3, 1],
            [3, 7, 11, 6],
            [2, 5, 10, 7],
            [9, 11, 10, 8],
            [0, 4, 8, 5],
            [1, 6, 9, 4],
        ];
        let corner_twist_perm : [ [usize; 4]; FACES] = [
            [0, 1, 3, 2],
            [2, 3, 7, 6],
            [3, 1, 5, 7],
            [4, 6, 7, 5],
            [1, 0, 4, 5],
            [0, 2, 6, 4]
        ];
        let edge_change : [ usize; FACES ] = [0, 0, 1, 0, 0, 1];
        let corner_change : [[ usize; 4 ] ; FACES] = [
            [0, 0, 0, 0],
            [1, 2, 1, 2],
            [1, 2, 1, 2],
            [0, 0, 0, 0],
            [1, 2, 1, 2],
            [1, 2, 1, 2],
        ];
        let mut edge_trans : [[usize ; CUBIES] ; NMOVES] = [ [0; CUBIES] ; NMOVES];
        let mut corner_trans : [[usize ; CUBIES] ; NMOVES] = [ [0; CUBIES] ; NMOVES];
        for m in 0..NMOVES {
            for c in 0..CUBIES {
                edge_trans[m][c] = c;
                corner_trans[m][c] = c;
            }
        }
        for f in 0..FACES {
            for t in 0..3 {
                let m = f * TWISTS + t;
                let isquarter = t == 0 || t == 2;
                let perminc = t + 1;
                if m < 0 {
                    continue;
                }
                for i in 0..4 {
                    let ii = (i + perminc) % 4;
                    for o in 0..2 {
                        let mut oo = o; /*new orientation*/
                        if isquarter {
                            oo = oo ^ edge_change[f];
                        }
                        edge_trans[m][edge_val(edge_twist_perm[f][i], o)] = edge_val(edge_twist_perm[f][ii], oo);
                    }
                    for o in 0..3 {
                        let mut oo = o;
                        if isquarter {
                            oo = (corner_change[f][i] + oo) % 3;
                        }
                        corner_trans[m][corner_val(corner_twist_perm[f][i], o)] = corner_val(corner_twist_perm[f][ii], oo);
                    }
                }
            }
        }
        let mut inv_move : [ usize ; NMOVES ] = [ 0 ; NMOVES];
        for i in 0..NMOVES {
            inv_move[i] = TWISTS * (i/TWISTS) + (NMOVES - i - 1) % TWISTS;
        }
        CubieData {
            corner_ori_inc, corner_ori_dec, corner_ori_neg_strip, mod24,
            //edge_twist_perm,
            //corner_twist_perm,
            //edge_change,
            edge_trans,
            corner_trans,
            inv_move,
        }
    }
}

impl CubieData {
    fn invert_sequence(self, seq : moveseq) -> moveseq {
        let len = seq.len();
        let mut r = Vec::with_capacity(len);
        for i in 0..seq.len() {
            r[len - i - 1] = self.inv_move[seq[i]];
        }
        r
    }
} 

#[cfg(test)]
mod tests {

    mod move_tests {
    use super::super::*;

    #[test]
    #[ignore]
    fn verify_b_f() {
        let mut cube = CubieCube::new();
        for i in 0..NMOVES {
            cube._move(i);
            cube.movepc(i);
        }
        assert_eq!(CubieCube::identity(), cube);
    }
    #[test]
    fn verify_forward_move() {
        for i in 0..FACES {
            let mut cube = CubieCube::identity();
            for j in 0..4 {
                cube._move(i * TWISTS);
            }
            assert_eq!(CubieCube::identity(), cube);
        }
    }
    #[test]
    #[ignore]
    fn verify_backward_move() {
        for i in 0..FACES {
            let mut cube = CubieCube::identity();
            for j in 0..4 {
                cube.movepc(i * TWISTS);
            }
            assert_eq!(CubieCube::identity(), cube);
        }
    }
    }
}
