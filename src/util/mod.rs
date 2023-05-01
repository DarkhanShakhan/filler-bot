pub fn is_piece_info(input: &String) -> bool {
    return input.starts_with("Piece");
}

pub fn is_anfield_info(input: &String) -> bool {
    return input.starts_with("Anfield");
}

//FIXME: functional programming
pub fn get_size(input: &str) -> Option<(i32, i32)> {
    let mut iter = input.strip_suffix(":")?.split_whitespace();
    let first_num = iter.nth(1)?;
    let second_num = iter.next()?;
    let w: i32 = first_num.parse().ok()?;
    let h: i32 = second_num.parse().ok()?;
    Some((w, h))
}

pub fn get_line(input: &String) -> Option<&str> {
    input.split_whitespace().last()
}
