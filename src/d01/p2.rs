use crate::input::grouped_input::GroupedInput;

pub fn run() -> String {
    let stdin = std::io::stdin();
    let input = GroupedInput::from(stdin.lock());
    let mut all_pack_values: Vec<usize> = input.map(|pack| pack.iter().sum()).collect();
    all_pack_values.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top_3: usize = all_pack_values.iter().take(3).sum();
    format!("{}", top_3)
}
