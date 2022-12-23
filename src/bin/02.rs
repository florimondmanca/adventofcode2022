// Rock Paper Scissors
// https://adventofcode.com/2022/day/2

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<usize> {
    let mut score = 0;

    let score_matrix = [
        //       X, Y, Z (player)
        /* A */ [3, 6, 0],
        /* B */ [0, 3, 6],
        /* C */ [6, 0, 3],
        /* (opponent) */
    ];

    for line in input.lines() {
        // A, B, C -> 0, 1, 2 (rock, paper, scissors)
        let opponent = line.chars().nth(0).unwrap() as usize - ('A' as usize);

        // X, Y, Z -> 0, 1, 2 (rock, paper, scissors)
        let player = line.chars().nth(2).unwrap() as usize - ('X' as usize);

        let outcome_score = score_matrix[opponent][player];
        let player_score = player + 1;

        score += outcome_score + player_score;
    }

    Some(score)
}

fn part2(input: &str) -> Option<usize> {
    let mut score = 0;

    for line in input.lines() {
        // A, B, C -> 0, 1, 2 (rock, paper, scissors)
        let opponent = line.chars().nth(0).unwrap() as usize - ('A' as usize);

        // X, Y, Z -> 0, 1, 2 (lose, draw, win)
        let outcome = line.chars().nth(2).unwrap() as usize - ('X' as usize);

        /*
        If the opponent's move is arranged as this 3-cycle...

              <--
          +-- Rock --+
          |          |
          |          |
        Paper --- Scissors
              -->

        Then:

        * To lose (outcome 0), choose the previous move (shift by -1, aka 0+2 mod 3).
        * To draw (outcome 1), choose the same move     (shift by 0,  aka 1+2 mod 3).
        * To win  (outcome 2), choose the next move     (shift by +1, aka 2+2 mod3).
        */

        let shift = (outcome + 2) % 3;
        let player = (opponent + shift) % 3;

        let outcome_score = outcome * 3;
        let player_score = player + 1;

        score += outcome_score + player_score;
    }

    Some(score)
}

#[test]
fn test_part1() {
    let input = &advent_of_code::read_file("inputs", 2);
    assert_eq!(part1(input), Some(11841));
}

#[test]
fn test_part2() {
    let input = &advent_of_code::read_file("inputs", 2);
    assert_eq!(part2(input), Some(13022));
}
