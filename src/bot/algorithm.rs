use crate::util::*;
use std::vec;

pub fn find_available_options(
    board: &[u128],
    opposite: &[u128],
    piece: &[u128],
    max: usize,
) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for ix in 0..=board.len() - piece.len() {
        res.append(
            get_overlaps(
                &board[ix..(ix + piece.len())],
                &opposite[ix..(ix + piece.len())],
                piece,
                max,
                ix,
            )
            .as_mut(),
        );
    }
    res
}

fn get_overlaps(
    board_snap: &[u128],
    opp_snap: &[u128],
    piece: &[u128],
    max: usize,
    ix: usize,
) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let mut count = 0;
    for i in 0..=max {
        for j in 0..piece.len() {
            if count_overlaps(board_snap[j] >> i, piece[j]) == 1
                && count_overlaps(opp_snap[j] >> i, piece[j]) == 0
            {
                count += 1;
            }
        }
        if count == 1 {
            res.push((ix, max - i))
        }
        count = 0;
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_available_opts() {
        use super::*;
        let res = find_available_options(&[8, 8, 0, 0], &[0, 0, 0, 2], &[2, 2], 2);
        println!("{:?}", res);
    }
}
