use super::generate;
use super::count;

pub fn run(
    stage: &Vec<Vec<bool>>
) -> Vec<Vec<bool>> {
    let mut new_stage = generate::new(stage[0].len(), stage.len());
    let mut y = 0;
    for parent in stage {
        let mut x = 0;
        for child in parent {
            let cells = count::run(stage, x, y);
            new_stage[y][x] = if child == &true {
                if cells <= 1 || 4 <= cells { false } else { true }
            } else {
                if cells == 3 { true } else { false }
            };
            x += 1;
        }
        y += 1;
    }
    new_stage
}
