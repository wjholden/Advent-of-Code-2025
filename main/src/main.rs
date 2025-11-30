use std::fs;

fn main() {
    day01::solve();
    // day02::solve();
    // day03::solve();
    // day04::solve();
    // day05::solve();
    // day06::solve();
    // day07::solve();
    // day08::solve();
    // day09::solve();
    // day10::solve();
    // day11::solve();
    // day12::solve();

    //overwrite_days("day01/src/lib.rs", 2..=12).unwrap();
}

#[allow(dead_code)]
fn overwrite_days(template: &str, range: impl Iterator<Item = usize>) -> std::io::Result<()> {
    let template = fs::read_to_string(template)?;

    for i in range {
        let dst = format!("day{i:0>2}/src/lib.rs");
        //let result = fs::copy(template, &dst)?;
        fs::write(&dst, template.replace("day01", &format!("day{i:0>2}")))?;
        println!("wrote to {dst}");
    }
    Ok(())
}
