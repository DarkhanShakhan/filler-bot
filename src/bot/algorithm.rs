use crate::util::{self, *};
use std::vec;

fn find_available_options(
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

pub fn available_options(
    board: &[u128],
    opposite: &[u128],
    piece: &[u128],
    max: usize,
) -> Vec<Point> {
    find_available_options(board, opposite, piece, max)
        .iter()
        .map(|p| Point::from(*p))
        .collect()
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
    let mut count_opp = 0;
    for i in 0..=max {
        for j in 0..piece.len() {
            count += count_overlaps(board_snap[j] >> i, piece[j]);
            count_opp += count_overlaps(opp_snap[j] >> i, piece[j]);
        }
        if count == 1 && count_opp == 0 {
            res.push((ix, max - i))
        }
        count = 0;
        count_opp = 0;
    }
    res
}

pub fn best_option(board_bits: &[u128], opposite: &[u128], options: &[Point], size: Size) -> Point {
    let width = size.width;
    let mine = points(board_bits, width, CellOwnership::Mine);
    let opponent = points(opposite, width, CellOwnership::Opponent);
    let borders = fill_board(size, &mine, &opponent).borders(&opponent, 8);
    let nearest = nearest_opposite_point(&mine, &borders);
    nearest_option(options, nearest)
}

fn points(board_bits: &[u128], width: usize, cell: util::CellOwnership) -> Vec<util::Point> {
    let mut res = vec![];
    for (ix, nbr) in board_bits.iter().enumerate() {
        for jx in 0..width {
            if nbr >> jx & 1 == 1 {
                res.push(util::Point::new(ix, width - jx - 1, cell))
            }
        }
    }
    res
}

fn fill_board(size: Size, mine: &[Point], opposite: &[Point]) -> util::Board {
    let mut board = Board::new(size);
    mine.iter().for_each(|p| {
        board.set_point(*p);
    });
    opposite.iter().for_each(|p| {
        board.set_point(*p);
    });
    board
}

fn nearest_opposite_point(mine: &[Point], opposite: &[Point]) -> Point {
    let mut res: Point = Point::default();
    let mut min_distance = usize::MAX;
    let mut distance: usize;
    for m in mine {
        for o in opposite {
            distance = *m + *o;
            if distance < min_distance {
                res = *o;
                min_distance = distance;
            }
        }
    }
    res
}

fn nearest_option(options: &[Point], nearest_opp: Point) -> Point {
    let mut res = Point::default();
    let mut min_distance = usize::MAX;
    let mut distance: usize;
    for option in options {
        distance = *option + nearest_opp;
        if distance < min_distance {
            res = *option;
            min_distance = distance;
        }
    }
    res
}
