use crate::submarine::Submarine;
use anyhow::anyhow;
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

pub fn cmd<S: AsRef<str>>(str: S) -> Result<Command> {
    match str.as_ref().split_ascii_whitespace().collect::<Vec<&str>>()[..] {
        [cmd, dist] => {
            let d = dist.parse::<usize>()?;
            match cmd.to_ascii_lowercase().as_ref() {
                "forward" => Ok(Command::Forward(d)),
                "up" => Ok(Command::Up(d)),
                "down" => Ok(Command::Down(d)),
                c => Err(anyhow!("Unknown command, {}", c)),
            }
        }
        _ => Err(anyhow!("Illegal input")),
    }
}

pub trait Dive {
    fn dive(&mut self, cmd: Command) -> &Self;
    fn dives<I: IntoIterator<Item = Command>>(&mut self, cmds: I) -> &Self;
}

impl Dive for Submarine {
    fn dive(&mut self, cmd: Command) -> &Self {
        match cmd {
            Command::Forward(d) => self.position.horizontal += d,
            Command::Up(d) => self.position.depth -= d,
            Command::Down(d) => self.position.depth += d,
        }
        self
    }

    fn dives<I: IntoIterator<Item = Command>>(&mut self, cmds: I) -> &Self {
        cmds.into_iter().for_each(|cmd| {
            self.dive(cmd);
        });
        self
    }
}

pub trait AimDive {
    fn aim_dive(&mut self, cmd: Command) -> &Self;
    fn aim_dives<I: IntoIterator<Item = Command>>(&mut self, cmds: I) -> &Self;
}

impl AimDive for Submarine {
    fn aim_dive(&mut self, cmd: Command) -> &Self {
        match cmd {
            Command::Forward(d) => {
                self.position.horizontal += d;
                self.position.depth += self.aim * d;
            }
            Command::Up(d) => self.aim -= d,
            Command::Down(d) => self.aim += d,
        }
        self
    }

    fn aim_dives<I: IntoIterator<Item = Command>>(&mut self, cmds: I) -> &Self {
        cmds.into_iter().for_each(|cmd| {
            self.aim_dive(cmd);
        });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Command::{Down, Forward, Up};

    const CMDS: [Command; 6] = [Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];

    #[test]
    fn it_can_parse_cmds() -> Result<()> {
        assert_eq!(cmd("forward 5")?, Forward(5));
        assert_eq!(cmd("down 8")?, Down(8));
        assert_eq!(cmd("up 3")?, Up(3));
        Ok(())
    }

    #[test]
    fn it_can_dive_to_new_position() {
        let mut sub = Submarine::new();
        sub.dives(CMDS);
        assert_eq!(sub.position.horizontal, 15);
        assert_eq!(sub.position.depth, 10);
    }

    #[test]
    fn it_can_aim_dive_to_new_position() {
        let mut sub = Submarine::new();
        sub.aim_dives(CMDS);
        assert_eq!(sub.position.horizontal, 15);
        assert_eq!(sub.position.depth, 60);
    }
}
