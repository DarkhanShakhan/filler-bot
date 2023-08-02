use crate::util::{self, *};
use std::{ops::Add, vec};

pub fn get_best_option(
    board: &[u128],
    opposite: &[u128],
    options: &[(usize, usize)],
    size: (usize, usize),
) -> (usize, usize) {
    let length = size.0;
    let board_coords = get_coords(board, length);
    let opp_coords = get_coords(opposite, length);
    let borders = find_territory_borders(
        (size.1, size.0),
        get_filled_board(size, &board_coords, &opp_coords),
        opp_coords.as_ref(),
    );
    let nearest = get_nearest_opposite_point(board_coords.as_ref(), borders.as_ref());
    get_nearest_option(options, nearest)
}

fn get_coords(board: &[u128], length: usize) -> Vec<(usize, usize)> {
    let mut coords = vec![];
    for (ix, nbr) in board.iter().enumerate() {
        for jx in 0..length {
            if nbr >> jx & 1 == 1 {
                coords.push((ix, length - jx - 1))
            }
        }
    }
    coords
}

pub fn get_filled_board(
    size: (usize, usize),
    points: &[(usize, usize)],
    opp_points: &[(usize, usize)],
) -> Vec<Vec<i32>> {
    let mut board = vec![vec![0; size.0]; size.1];
    for p in points.iter() {
        board[p.0][p.1] = 1;
    }
    for p in opp_points.iter() {
        board[p.0][p.1] = -1;
    }
    board
}

fn get_nearest_opposite_point(
    board_coords: &[(usize, usize)],
    opp_coords: &[(usize, usize)],
) -> (usize, usize) {
    let mut res = (0, 0);
    let mut min_distance = usize::MAX;
    let mut distance;
    for b in board_coords {
        for o in opp_coords {
            distance = manhatan_distance(*b, *o);
            if distance < min_distance {
                res = *o;
                min_distance = distance;
            }
        }
    }
    res
}

fn get_nearest_option(options: &[(usize, usize)], nearest: (usize, usize)) -> (usize, usize) {
    let mut res = (0, 0);
    let mut min_distance = usize::MAX;
    for opt in options {
        let dist = manhatan_distance(*opt, nearest);
        if dist < min_distance {
            res = *opt;
            min_distance = dist;
        }
    }
    res
}

fn manhatan_distance(point_1: (usize, usize), point_2: (usize, usize)) -> usize {
    point_1.0.abs_diff(point_2.0) + point_1.1.abs_diff(point_2.1)
}

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

fn find_territory_borders(
    size: (usize, usize),
    board: Vec<Vec<i32>>,
    points: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let k = 8;
    let mut blocked: bool = false;
    let mut count_free: u32 = 0;
    let mut value: i32;
    for p in points {
        //check each side
        //left side
        if p.0 >= k {
            let mut free = true;
            for ix in 1..k {
                value = board[p.0 - ix][p.1];
                if value != 0 {
                    free = false;
                }
                if value == 1 {
                    blocked = true;
                }
            }
            if free {
                count_free += 1;
            }
        }
        if p.0 + k < size.0 {
            let mut free = true;
            for ix in 1..k {
                value = board[p.0 + ix][p.1];
                if value != 0 {
                    free = false;
                }
                if value == 1 {
                    blocked = true;
                }
            }
            if free {
                count_free += 1;
            }
        }
        if p.1 >= k {
            let mut free = true;
            for ix in 1..k {
                value = board[p.0][p.1 - ix];
                if value != 0 {
                    free = false;
                }
                if value == 1 {
                    blocked = true;
                }
            }
            if free {
                count_free += 1;
            }
        }
        if p.1 + k < size.1 {
            let mut free = true;
            for ix in 1..k {
                value = board[p.0][p.1 + ix];
                if value != 0 {
                    free = false;
                }
                if value == 1 {
                    blocked = true;
                }
            }
            if free {
                count_free += 1;
            }
        }
        match count_free {
            0 => {}
            1 => {
                if !blocked {
                    res.push(*p);
                }
            }
            _ => res.push(*p),
        }
        blocked = false;
        count_free = 0;
    }
    res
}

fn best_option(board_bits: &[u128], opposite: &[u128], options: &[Point], size: Size) -> Point {
    let width = size.width;
    let mine = points(board_bits, width, CellOwnership::Mine);
    let opponent = points(opposite, width, CellOwnership::Opponent);
    //FIXME: get borders
    let nearest = nearest_opposite_point(&mine, &opponent);
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

fn fill_board(size: Size, mine: &[(usize, usize)], opposite: &[(usize, usize)]) -> util::Board {
    let mut board = Board::new(size);
    mine.iter().for_each(|(x, y)| {
        board.set_point(Point::new(*x, *y, CellOwnership::Mine));
    });
    opposite.iter().for_each(|(x, y)| {
        board.set_point(Point::new(*x, *y, CellOwnership::Opponent));
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
