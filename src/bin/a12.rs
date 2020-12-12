use adventofcode2020::prelude::*;
use std::str::FromStr;

enum Action {
    MoveShip(i32, i32),
    MoveVector(i32, i32),
    Turn(i32),
    Forward(i32),
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();
        let a = bytes[0];
        let num = std::str::from_utf8(&bytes[1..])?;
        let num = num.parse::<i32>()?;

        let action = match a {
            b'N' => Action::MoveShip(0, num),
            b'S' => Action::MoveShip(0, -num),
            b'E' => Action::MoveShip(num, 0),
            b'W' => Action::MoveShip(-num, 0),
            b'L' | b'R' => {
                if num < 0 {
                    return Err(Error::General("Turn must be positive".into()));
                }
                if num % 90 != 0 {
                    return Err(Error::General("Turn must be multiple of 90".into()));
                }

                let turns = (num % 360) / 90;

                if a == b'L' {
                    Action::Turn(4 - turns)
                } else {
                    Action::Turn(turns)
                }
            }
            b'F' => Action::Forward(num),
            _ => return Err(Error::General("Invalid action".into())),
        };

        Ok(action)
    }
}

fn run(actions: &[Action], vx: i64, vy: i64) -> Result<u64> {
    #[derive(Debug)]
    struct State {
        sx: i64,
        sy: i64,
        vx: i64,
        vy: i64,
    }

    impl State {
        fn new(ax: i64, ay: i64, vx: i64, vy: i64) -> Self {
            Self {
                sx: ax,
                sy: ay,
                vx,
                vy,
            }
        }
    }

    actions
        .iter()
        .try_fold(State::new(0, 0, vx, vy), |mut state, a| -> Result<State> {
            match a {
                Action::MoveShip(dx, dy) => {
                    state.sx += *dx as i64;
                    state.sy += *dy as i64;
                }
                Action::MoveVector(dx, dy) => {
                    state.vx += *dx as i64;
                    state.vy += *dy as i64;
                }
                Action::Turn(n) => {
                    let (mut vx, mut vy) = (state.vx, state.vy);
                    for _i in 0..*n {
                        let (prev_vx, prev_vy) = (vx, vy);
                        vy = -prev_vx;
                        vx = prev_vy;
                    }
                    state.vx = vx;
                    state.vy = vy;
                }
                Action::Forward(n) => {
                    state.sx += *n as i64 * state.vx;
                    state.sy += *n as i64 * state.vy;
                }
            }
            //dbg!(&state);
            Ok(state)
        })
        .map(|state| state.sx.abs() as u64 + state.sy.abs() as u64)
}

fn main() -> Result<()> {
    let actions: Vec<Action> = read_file("data/12.txt")?;

    let part1 = run(&actions, 1, 0)?;
    println!("{}", part1);

    let actions: Vec<Action> = actions
        .into_iter()
        .map(|a| match a {
            Action::MoveShip(dx, dy) => Action::MoveVector(dx, dy),
            _ => a,
        })
        .collect();
    let part2 = run(&actions, 10, 1)?;
    println!("{}", part2);

    Ok(())
}