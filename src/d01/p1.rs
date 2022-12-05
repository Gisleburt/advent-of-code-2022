use crate::input::usize_grouped_input::UsizeGroupedInput;

pub fn d01p1() -> String {
    let stdin = std::io::stdin();
    let input = UsizeGroupedInput::from(stdin.lock());
    let max = input.map(|pack| pack.iter().sum())
        .reduce(usize::max)
        .expect("Something went wrong, there were no numbers");
    format!("{}", max)
}
