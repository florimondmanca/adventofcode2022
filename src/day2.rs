use std::error::Error;
use std::fs;
use std::path::Path;

fn part1(content: String) -> Result<(), Box<dyn Error>> {
    let mut score = 0;

    let score_matrix = vec![
        //           X, Y, Z (player)
        /* A */ vec![3, 6, 0],
        /* B */ vec![0, 3, 6],
        /* C */ vec![6, 0, 3],
        /* (opponent) */
    ];

    for line in content.lines() {
        // A, B, C -> 0, 1, 2 (rock, paper, scissors)
        let opponent = line.chars().nth(0).unwrap() as usize - ('A' as usize);

        // X, Y, Z -> 0, 1, 2 (rock, paper, scissors)
        let player = line.chars().nth(2).unwrap() as usize - ('X' as usize);

        let outcome_score = score_matrix[opponent][player];
        let player_score = player + 1;

        score += outcome_score + player_score;
    }

    println!("Answer (part 1): {}", score);

    Ok(())
}

fn part2(content: String) -> Result<(), Box<dyn Error>> {
    let mut score = 0;

    for line in content.lines() {
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

    println!("Answer (part 2): {}", score);

    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(Path::new("inputs/2.txt"))?;
    part1(content.clone())?;
    part2(content.clone())?;
    Ok(())
}
