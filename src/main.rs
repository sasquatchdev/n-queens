const C: usize = 12;
const R: usize = 12;
const N: usize = 12;

type State = [[bool; R]; C];

fn main() {
    let start = std::time::Instant::now();

    let mut state = [[false; R]; C];
    let mut solutions = Vec::new();

    // start recursive backtracking
    solve(&mut state, 0, &mut solutions);

    let elapsed = start.elapsed();

    // log solution count
    println!("found {} solutions", solutions.len());

    // log metrics
    println!("elapsed time: {:?}", elapsed);
}

fn _prettify(state: &State) {
    for i in 0..C {
        for j in 0..R {
            if state[i][j] {
                print!("Q ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
    println!();
}

fn solve(state: &mut State, position: usize, solutions: &mut Vec<State>) {
    if position == N {
        solutions.push(*state);
        return;
    }

    for i in 0..C {
        let pos = position * C + i;

        if valid(state, pos) {
            state[i][position] = true; // place queen
            solve(state, position + 1, solutions);
            state[i][position] = false; // remove queen
        }
    }
}

fn valid(state: &State, position: usize) -> bool {
    let col = position % C;
    let row = position / C;

    // since a queen can attack in all 8 directions,
    // we need to check all of them for intersection
    // with another queen (i.e. true in the state array)

    let directions = [
        (0, 1), (0, -1),    // vertical
        (-1, 0), (1, 0),    // horizontal
        (-1, -1), (-1, 1),  // diagonal left
        (1, 1), (1, -1),    // diagonal right
    ];

    for (dc, dr) in directions.iter() {
        let mut c = col as isize;
        let mut r = row as isize;

        loop {
            c += dc;
            r += dr;

            if c < 0 || c >= C as isize || r < 0 || r >= R as isize {
                break;
            }

            if state[c as usize][r as usize] {
                return false;
            }
        }
    }

    true
}
