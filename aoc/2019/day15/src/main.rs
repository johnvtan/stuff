use intcode::*;

#[derive(PartialEq, Debug, Clone, Copy)]
enum MoveCmd {
    North,
    South,
    East,
    West,
}

const ALL_CMDS: [MoveCmd; 4] = [MoveCmd::North, MoveCmd::South, MoveCmd::East, MoveCmd::West];

impl MoveCmd {
    fn to(&self) -> i64 {
        match self {
            Self::North => 1,
            Self::South => 2,
            Self::East => 3,
            Self::West => 4,
        }
    }

    fn undo(&self) -> MoveCmd {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Status {
    HitWall,
    Moved,
    FoundOxygen,
}

impl Status {
    fn from(n: i64) -> Status {
        match n {
            0 => Status::HitWall,
            1 => Status::Moved,
            2 => Status::FoundOxygen,
            _ => unreachable!(),
        }
    }
}

enum DfsStatus {
    ExploreOnwards,
    ExploreFromCurrent,
    StopImmediately,
}

struct Droid {
    cpu: Intcode,
}

impl Droid {
    fn new() -> Self {
        let prog = std::fs::read_to_string("program.txt").expect("couldn't read program");
        let cpu = Intcode::new(csv_to_vec(prog));
        Self { cpu }
    }

    fn try_move(&mut self, cmd: MoveCmd) -> Status {
        assert!(!self.cpu.is_halted());
        self.cpu.input.push_back(cmd.to());
        self.cpu.run();
        Status::from(self.cpu.output.pop_front().unwrap())
    }

    // Returns true if the dfs should complete immediately, false if dfs can continue.
    fn dfs_inner<F>(&mut self, path_len: i64, from: Option<MoveCmd>, on_status: &mut F) -> bool
    where
        F: FnMut(&Status, i64) -> DfsStatus,
    {
        let to_explore = if let Some(from) = from {
            // Filter out the direction we came from to avoid infinite loop.
            ALL_CMDS
                .to_vec()
                .into_iter()
                .filter(|cmd| *cmd != from)
                .collect()
        } else {
            ALL_CMDS.to_vec()
        };

        for cmd in to_explore {
            let status = self.try_move(cmd);

            // Path length doesn't increase if we hit a wall. Also, no need to undo command if we
            // hit a wall since the droid didn't move.
            let (curr_path_len, should_undo) = if let Status::HitWall = status {
                (path_len, false)
            } else {
                (path_len + 1, true)
            };

            match on_status(&status, curr_path_len) {
                DfsStatus::ExploreOnwards => {
                    // Continue search recurisvely from the spot we moved to.
                    // Exit immediately if recursive search returns true.
                    if self.dfs_inner(path_len + 1, Some(cmd.undo()), on_status) {
                        return true;
                    }
                }
                DfsStatus::ExploreFromCurrent => {}
                DfsStatus::StopImmediately => return true,
            }

            // Undo last command if necessary
            if should_undo {
                // ensure that we actually moved.
                assert_ne!(self.try_move(cmd.undo()), Status::HitWall);
            }
        }
        false
    }

    fn dfs<F>(&mut self, on_status: &mut F)
    where
        F: FnMut(&Status, i64) -> DfsStatus,
    {
        self.dfs_inner(0, None, on_status);
    }

    fn find_shortest_path_to_oxygen(&mut self) -> i64 {
        let mut shortest = i64::max_value();
        self.dfs(&mut |status, path_len| match status {
            Status::HitWall => DfsStatus::ExploreFromCurrent,
            Status::Moved => DfsStatus::ExploreOnwards,
            Status::FoundOxygen => {
                shortest = std::cmp::min(shortest, path_len);
                DfsStatus::ExploreFromCurrent
            }
        });
        shortest
    }

    fn navigate_to_oxygen(&mut self) {
        self.dfs(&mut |status, _| match status {
            Status::HitWall => DfsStatus::ExploreFromCurrent,
            Status::Moved => DfsStatus::ExploreOnwards,
            Status::FoundOxygen => DfsStatus::StopImmediately,
        });
    }

    fn longest_path_to_wall(&mut self) -> i64 {
        let mut longest: i64 = 0;
        self.dfs(&mut |status, path_len| {
            match status {
                Status::HitWall => {
                    longest = std::cmp::max(longest, path_len);
                    DfsStatus::ExploreFromCurrent
                }
                Status::Moved | Status::FoundOxygen => DfsStatus::ExploreOnwards,
            }
        });
        longest
    }
}

fn main() {
    {
        let mut droid = Droid::new();
        let path_len = droid.find_shortest_path_to_oxygen();
        println!("part 1: {path_len}");
    }
    {
        let mut droid = Droid::new();
        droid.navigate_to_oxygen();
        let path_len = droid.longest_path_to_wall();
        println!("part 2: {path_len}");
    }
}
