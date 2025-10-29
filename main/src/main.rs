use std::fs;

fn main() {
    day01::solve();
    //day02::solve();
    //day03::solve();
    //day04::solve();
    //day05::solve();
    //day06::solve();
    //day07::solve();
    //day08::solve();
    //day09::solve();
    //day10::solve();
    //day11::solve();
    //day12::solve();

    overwrite_days("day01/src/lib.rs", 2..=12).unwrap();
}

fn overwrite_days(template: &str, range: impl Iterator<Item = usize>) -> std::io::Result<()> {
    for i in range {
        let dst = format!("day{i:0>2}/src/lib.rs");
        let result = fs::copy(template, &dst)?;
        println!("wrote {result} bytes to {dst}");
    }
    Ok(())
}
