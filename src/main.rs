pub mod matrix;

use matrix::Matrix;
use std::fmt::Display;

struct HorseRide {
    n: u8,
    board: Matrix,
    moves_x: [i8; 8],
    moves_y: [i8; 8],
    node_counter: u32,
    x0: i8,
    y0: i8,
}

impl HorseRide {
    fn new(n: u8, x0: i8, y0: i8) -> HorseRide {
        let mut hr = HorseRide {
            n,
            x0,
            y0,
            board: Matrix::new_with_initial(n.into(), n.into(), -1),
            moves_x: [2, 1, -1, -2, -2, -1, 1, 2],
            moves_y: [1, 2, 2, 1, -1, -2, -2, -1],
            node_counter: 0,
        };
        hr.board.set(
            x0.try_into().expect("Initial x must be positive"),
            y0.try_into().expect("initial y must be positive"),
            0,
        );
        hr
    }

    fn try_move(&mut self, steps: i8, x: i8, y: i8) -> bool {
        self.node_counter += 1;

        for i in 0..=7 {
            let (nx, ny) = (x + self.moves_x[i], y + self.moves_y[i]);
            let limit: i8 = self.n.try_into().expect("Could not translate bounds");

            if nx >= 0 && nx < limit && ny >= 0 && ny < limit {
                let (nxu, nyu) = (nx.try_into().unwrap(), ny.try_into().unwrap());

                if self.board.get(nxu, nyu) > -1 {
                    continue;
                }

                self.board.set(nxu, nyu, steps);

                if steps == (limit * limit - 1) {
                    return true;
                }

                if self.try_move(steps + 1, nx, ny) {
                    return true;
                } else {
                    self.board.set(nxu, nyu, steps - 1);
                }
            }
        }

        return false;
    }

    fn run(&mut self) -> bool {
        self.try_move(1, self.x0.into(), self.y0.into())
    }
}

impl Display for HorseRide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}\nMovimentos realizados: {}",
            self.board, self.node_counter
        ))
    }
}

fn main() {
    let mut hr = HorseRide::new(8, 0, 0);

    if hr.run() {
        println!("{}", hr);
    } else {
        println!("{}\nNenhuma solução encontrada", hr);
    }
}
