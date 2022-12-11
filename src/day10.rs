pub fn run() {
    let content = include_str!("inputs/10.txt");

    let mut strength = 0;

    let mut on_cycle1 = |x: i32, cycle: i32| {
        if cycle % 40 == 20 {
            strength += x * cycle;
        }
    };

    simulate(content, &mut on_cycle1);
    println!("Answer (part 1): {strength}");

    let mut screen = String::new();

    let mut on_cycle2 = |x: i32, cycle: i32| {
        let pixel_pos = (cycle - 1) % 40;

        if x.abs_diff(pixel_pos) <= 1 {
            screen.push('#');
        } else {
            screen.push('.');
        }

        if pixel_pos == 39 {
            screen.push('\n');
        }
    };

    simulate(content, &mut on_cycle2);
    println!("Answer (part 2):\n{}", screen);
}

fn simulate<F>(content: &str, mut on_cycle: F)
where
    F: FnMut(i32, i32),
{
    let mut cycle = 0;
    let mut x = 1;

    let mut run_cycle = |x: i32| {
        cycle += 1;
        on_cycle(x, cycle);
    };

    for line in content.lines() {
        if line == "noop" {
            run_cycle(x);
        } else {
            run_cycle(x);
            run_cycle(x);
            let dx = line.split_at(5).1.parse::<i32>().unwrap();
            x += dx;
        }
    }
}
