use crate::util::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::vec;

pub fn get_best_option(
    board: &[u128],
    opposite: &[u128],
    options: &[(usize, usize)],
    size: (usize, usize),
) -> (usize, usize) {
    let length = size.0;
    let board_coords = get_coords(board, length);
    let opp_coords = get_coords(opposite, length);
    // let filled_board = get_filled_board(size, &board_coords, &opp_coords);
    // println!("{:?}", filled_board);
    // let mut file = OpenOptions::new().append(true).open("nearest.txt").unwrap();
    // let nearest = get_target(size, &board_coords, &opp_coords);
    let borders = find_territory_borders(
        (size.1, size.0),
        get_filled_board(size, &board_coords, &opp_coords),
        opp_coords.as_ref(),
    );
    let nearest = get_nearest_opposite_point(board_coords.as_ref(), borders.as_ref());
    // let res = format!("Nearest: {:?}\n", nearest,);
    // file.write_all(res.as_bytes()).unwrap();
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

fn get_target(
    size: (usize, usize),
    my_points: &[(usize, usize)],
    opp_points: &[(usize, usize)],
) -> (usize, usize) {
    let mut res = (0, 0);
    // get all points nearest to each angle
    let mut left_top = (0, 0);
    let mut left_top_min_distance = usize::MAX;
    let mut right_top = (0, 0);
    let mut right_top_min_distance = usize::MAX;
    let mut left_down = (0, 0);
    let mut left_down_min_distance = usize::MAX;
    let mut right_down = (0, 0);
    let mut right_down_min_distance = usize::MAX;
    for opp in opp_points {
        let mut dist = manhatan_distance((size.1, 0), *opp);
        if dist < left_down_min_distance {
            left_down = *opp;
            left_down_min_distance = dist;
        }
        dist = manhatan_distance((0, 0), *opp);
        if dist < left_top_min_distance {
            left_top = *opp;
            left_top_min_distance = dist;
        }
        dist = manhatan_distance((0, size.0), *opp);
        if dist < right_top_min_distance {
            right_top = *opp;
            right_top_min_distance = dist;
        }
        dist = manhatan_distance((size.0, size.1), *opp);
        if dist < right_down_min_distance {
            right_down = *opp;
            right_down_min_distance = dist;
        }
    }
    let distances = vec![
        left_down_min_distance,
        left_top_min_distance,
        right_down_min_distance,
        right_top_min_distance,
    ];
    let points = vec![left_down, left_top, right_down, right_top];
    // let distances = vec![left_down_min_distance, left_top_min_distance, r]
    // let mut max_distance = 0;
    // for (ix, dist) in distances.iter().enumerate() {
    //     if max_distance < *dist {
    //         res = angles[ix];
    //         max_distance = *dist;
    //     }
    // }
    let board = get_filled_board(size, my_points, opp_points);
    let angles = vec![(size.1, 0), (0, 0), (size.1, size.0), (0, size.0)];
    // let mut min = i32::MAX;
    // for (ix, dist) in distances.iter().enumerate() {
    //     if *dist < min as usize {
    //         res = angles[ix];
    //         min = *dist as i32;
    //     }
    // }
    let mut count = 0;
    let mut max = 0;
    for ix in 0..4 {
        let (slope, intercept) = find_line_formula(
            points[ix].0 as f64,
            points[ix].1 as f64,
            angles[ix].0 as f64,
            angles[ix].1 as f64,
        );
        for jx in points[ix].0..angles[ix].0 {
            let y = find_y(slope, intercept, jx as f64);
            if board[jx][y] == 1 {
                break;
            }
            count += 1;
        }
        if count >= max {
            res = points[ix];
            max = count;
        }
        count = 0;
    }
    res
}

fn find_line_formula(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64) {
    let slope = (y2 - y1) / (x2 - x1);
    let y_intercept = y1 - slope * x1;
    (slope, y_intercept)
}

fn find_y(slope: f64, y_intercept: f64, x: f64) -> usize {
    (x * slope + y_intercept) as usize
}

fn get_furthest_opposite_point(
    opp_coords: &[(usize, usize)],
    size: (usize, usize),
) -> (usize, usize) {
    let mut res = (0, 0);
    let mut min_distance = usize::MAX;
    for opp in opp_coords {
        let mut dist = manhatan_distance((0, size.1), *opp);
        if dist < min_distance {
            res = *opp;
            min_distance = dist;
        }
        dist = manhatan_distance((0, 0), *opp);
        if dist < min_distance {
            res = *opp;
            min_distance = dist;
        }
        dist = manhatan_distance((size.0, 0), *opp);
        if dist < min_distance {
            res = *opp;
            min_distance = dist;
        }
        dist = manhatan_distance(size, *opp);
        if dist < min_distance {
            res = *opp;
            min_distance = dist;
        }
    }
    // println!("{:?}", res);
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

#[cfg(test)]
mod tests {
    #[test]
    fn find_available_opts() {
        use super::*;
        let res = find_available_options(&[8, 8, 0, 0], &[0, 0, 0, 2], &[2, 2], 2);
        println!("{:?}", res);
    }
    #[test]
    fn get_target_opt() {
        use super::*;
        let size = (8, 10);
        let my_points = vec![(5, 3), (5, 4), (5, 5), (4, 5), (3, 5)];
        let opp_points = vec![(8, 6), (8, 7), (7, 7), (7, 6)];
        let target = get_target(size, &[], &opp_points);
        println!("{:?}", target);
        // assert!(target == (6, 7));
    }
}

fn find_territory_borders(
    size: (usize, usize),
    board: Vec<Vec<i32>>,
    points: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let k = 4;
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

fn is_free_and_blocked(lane: &[i32]) -> (bool, bool) {
    let mut blocked = false;
    let mut free = true;
    for l in lane {
        if *l != 0 {
            free = false;
        }
        if *l == 1 {
            blocked = true;
        }
    }
    return (free, blocked);
}
//TODO:
// 1. find all opponent's open borders: open borders are points with at least one empty slot from each angle
// 2. choose the nearest one
// 3. define point's empty routes and choose the nearest one e.g. Point(4,4) has empty routes Point(2,4) or Point(6, 4).
// need to choose between these two points
// N.B. the point which is surrounded from each route except one and at least one route is occupied by me should n't be included
// 4. turn the territory to that point
