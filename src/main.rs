use clap::{command, Parser};

mod screen;
mod snake;

use snake::SnakeGame;

#[derive(Parser, Debug)]
#[command(name = "snake", version = "0.1.0", about = "Classic snake game for your terminal")]
struct Cli {
    /// Sets difficulty to easy
    #[arg(short = 'e', long, conflicts_with = "hard")]
    easy: bool,

    /// Sets difficulty to hard
    #[arg(short = 'h', long, conflicts_with = "easy")]
    hard: bool,

    /// Enables multiplayer mode
    #[arg(short = 'm', long)]
    multiplayer: bool,

    /// Steer the snakes using two keys only (increased difficulty)
    #[arg(short = 't', long = "two_key_steering")]
    two_key_steering: bool,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let mut target_fps = 8.0;

    if args.hard {
        target_fps *= 1.5;
    } else if args.easy {
        target_fps *= 0.7;
    }
    let target_fps = target_fps;

    let num_players = if args.multiplayer { 2 } else { 1 };
    let is_four_key_steering = !args.two_key_steering;

    SnakeGame::new(num_players, target_fps, is_four_key_steering).run()
}
