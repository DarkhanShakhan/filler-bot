mod app;
mod bot;
mod util;
fn main() {
    let mut app = app::App::default();
    app.start();
    // let stdin = io::stdin();
    // let mut anfield = anfield::Anfield::new();
    // let mut piece = piece::Piece::new();
    // for line in stdin.lock().lines() {
    //     let mut input: String = String::new();
    //     match line {
    //         Ok(l) => input = l,
    //         _ => {
    //             return;
    //         }
    //     };

    //     if input.is_empty() {
    //         continue;
    //     }
    //     if anfield.is_collect() {
    //         if let Some(inp) = parse_line(&input) {
    //             anfield.collect(inp);
    //         }
    //         continue;
    //     }
    //     if piece.is_collect() {
    //         piece.collect(input);
    //         if !piece.is_collect() {
    //             //TODO: run the algorithm
    //             let opts =
    //                 algorithm::find_available_options(anfield.get_board(), piece.get_piece());
    //             let nearest_opp = algorithm::find_nearest_opposite(anfield.get_board());
    //             let res = algorithm::get_best_option(opts, nearest_opp);

    //             println!("{} {}", res.0, res.1);
    //             piece.clear();
    //             anfield.clear();
    //         }
    //         continue;
    //     }
    //     if is_anfield_info(&input) {
    //         if let Some((w, h)) = parse_size(&input) {
    //             anfield.set_size((w, h));
    //             anfield.set_is_collect(true);
    //         }
    //         continue;
    //     }
    //     // println!("{}", input);
    //     if is_piece_info(&input) {
    //         if let Some((w, h)) = parse_size(&input) {
    //             piece.set_size((w, h));
    //             piece.set_is_collect(true);
    //         }
    //     }
    // }
}
