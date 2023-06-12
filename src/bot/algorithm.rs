use std::vec;

pub fn find_available_options(
    anfield: &Vec<Vec<char>>,
    piece: &Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    //TODO: choose the player
    let mut out = vec![];
    for i in 0..=anfield.len() - piece.len() {
        for j in 0..=anfield[0].len() - piece[0].len() {
            match is_available(anfield, piece, i, j) {
                true => out.push((j, i)),
                false => continue,
            }
        }
    }
    out
}

pub fn find_nearest_opposite(anfield: &Vec<Vec<char>>) -> (usize, usize) {
    let mut nearest = (anfield[0].len(), anfield.len());
    let mut curr: f32;
    let mut min = f32::MAX;
    for i in 0..anfield.len() {
        for j in 0..anfield[0].len() {
            if anfield[i][j] == '$' || anfield[i][j] == 's' {
                curr = ((i * i + j * j) as f32).sqrt();
                if curr < min {
                    min = curr;
                    nearest = (i, j);
                }
            }
        }
    }
    nearest
}

fn is_available(anfield: &[Vec<char>], piece: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let mut anf_char: char;
    let mut piece_char: char;
    let mut is_overlap = false;
    for ix in 0..piece.len() {
        for jx in 0..piece[0].len() {
            anf_char = anfield[i + ix][j + jx];
            piece_char = piece[ix][jx];
            if anf_char == '.' || piece_char == '.' {
                continue;
            }
            if !is_overlap && (anf_char == 'a' || anf_char == '@') && piece_char == 'O' {
                is_overlap = true;
                continue;
            }
            return false;
        }
    }
    is_overlap
}

pub fn get_best_option(opts: Vec<(usize, usize)>, nearest_opp: (usize, usize)) -> (usize, usize) {
    let mut max = 0.0;
    let mut curr: f32;
    let mut out = (0, 0);
    for (x, y) in opts {
        curr = (((nearest_opp.0 - x).pow(2) + (nearest_opp.1 - y).pow(2)) as f32).sqrt();
        if curr > max {
            out = (x, y);
            max = curr;
        }
    }
    out
}
