fn main() {
    let data = include_str!("../input.txt").trim();

    let rounds: Vec<(char, char)> = data.split('\n')
        .map(|round| (round.chars().nth(0).unwrap(), round.chars().nth(2).unwrap()))
        .collect();
        

    let total_point1: usize = rounds.iter().map(judge1).sum();
    let total_point2: usize = rounds.iter().map(judge2).sum();

    println!(
        "Part 1: {}",
        total_point1
    );

    println!(
        "Part 2: {}",
        total_point2
    );
}

fn judge1(round: &(char, char)) -> usize {
    let point_of_mine = match round.1 {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Wrong character.")
    };

    let winning_point = match round {
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        _ => 0
    };

    point_of_mine + winning_point
}

fn judge2(round: &(char, char)) -> usize {
    let point_of_mine = match round {
        ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1,
        ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 2,
        ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 3,
        _ => panic!("Wrong character.")
    };

    let winning_point = match round.1 {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!("Wrong character.")
    };

    point_of_mine + winning_point
}