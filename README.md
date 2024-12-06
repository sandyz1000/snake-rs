# Snake


### Building from Source

```bash
cargo run --release
```

## Controls

At the moment, up to two players are supported. The controls for making the snake turn left or right are:

* Player 1: arrow keys
* Player 2: WASD keys

Pressing Esc or q will terminate the game.

Here's the complete and detailed README for the Snake game:

---

# Snake

A classic snake game implemented in Rust that runs directly in your terminal. Enjoy retro gaming fun with up to two players, adjustable difficulty, and optional two-key steering for an extra challenge.

---

## Features

- **Single-player and Multiplayer Modes**: Play solo or compete against a friend.
- **Adjustable Difficulty**: Choose between easy or hard modes to match your skill level.
- **Two-Key Steering**: Simplify or increase the challenge by toggling between four-key and two-key steering.
- **Keyboard Controls**: Supports intuitive controls for both players.

---

## Prerequisites

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

---

## Building from Source

To build and run the game, clone the repository and execute the following command:

```bash
cargo run --release
```

This builds the game in optimized mode for the best performance.

---

## CLI Usage

### Syntax:
```bash
snake [OPTIONS]
```

### Options:
| Flag                     | Description                                          |
|--------------------------|------------------------------------------------------|
| `-e`, `--easy`           | Sets difficulty to easy.                             |
| `-h`, `--hard`           | Sets difficulty to hard.                             |
| `-m`, `--multi`          | Enables multiplayer mode.                            |
| `-t`, `--two_key_steering` | Enables two-key steering mode for increased difficulty. |

#### Examples:
1. **Single-player Easy Mode**:
   ```bash
   snake --easy
   ```

2. **Multiplayer with Hard Difficulty**:
   ```bash
   snake --multi --hard
   ```

3. **Single-player Hard Mode with Two-Key Steering**:
   ```bash
   snake --hard --two_key_steering
   ```

---

## Controls

### Player Controls:
- **Player 1**: Use the arrow keys to steer the snake.
- **Player 2**: Use the `W`, `A`, `S`, `D` keys to control your snake.

### Universal Controls:
- **Pause**: Press `Esc` or `q` to terminate the game.

---

## Gameplay Mechanics

- The objective is to guide your snake to consume food and grow in length while avoiding collisions with walls, yourself, or other snakes.
- The speed of the game varies based on the selected difficulty level:
  - **Easy Mode**: Slower snake speed for casual gameplay.
  - **Hard Mode**: Faster snake speed for an added challenge.
- The game supports both **four-key steering** (default) and **two-key steering** (enabled via `--two_key_steering`), which requires you to navigate using only left and right turns.

---

## Known Issues and Limitations

- Terminal window size may affect the display. Ensure your terminal is large enough for smooth gameplay.
- Sound effects are not currently implemented.

---

## Contributing

Contributions are welcome! Feel free to fork the repository and submit pull requests. For major changes, please open an issue first to discuss your proposal.

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

--- 

Enjoy the game and challenge your friends! 🎮
