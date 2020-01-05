static dist: [[i32; 2]; 8] = [
    [-1, 0], [-1, -1], [0, -1], [1, -1],
    [1, 0], [1, 1], [0, 1], [-1, 1]
];

pub fn run(
    stage: &Vec<Vec<bool>>,
    x: usize,
    y: usize
) -> usize {
    let mut count = 0;
    for pos in dist.iter() {
        let x_target = x as i32 + pos[0];
        let y_target = y as i32 + pos[1];
        if -1 < x_target && x_target < (stage[0].len() - 1) as i32 && -1 < y_target && y_target < (stage.len() - 1) as i32 {
            count += if stage[y_target as usize][x_target as usize] { 1 } else { 0 };
        }
    }
    count
}

