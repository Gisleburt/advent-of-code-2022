use crate::input::grouped_input::GroupedInput;

pub fn run() -> String {
    let stdin = std::io::stdin();
    let input = GroupedInput::from(stdin.lock());
    let max = input
        .map(|pack| pack.iter().sum())
        .reduce(usize::max)
        .expect("Something went wrong, there were no numbers");
    format!("{}", max)
}
