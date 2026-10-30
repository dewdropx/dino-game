# Dino Game — Rust

A 2D endless runner inspired by the Google's Dino game, built from scratch in **Rust** using the [Macroquad](https://macroquad.rs/) game development library.

The project started as a simple playable prototype and evolved into a sprite-based game featuring animated characters, scrolling backgrounds, randomized obstacles, progressive difficulty, and collision detection.

## Features

- **Dinosaur Animation** — Animated running sprites with frame switching and jump mechanics.
- **Jump Physics** — Gravity, jumping, and ground collision handling.
- **Mushroom Obstacles** — Red pixel-art mushrooms replace the traditional cactus obstacles.
- **Randomized Obstacle Patterns** — Mushrooms spawn individually and in groups with varying heights.
- **Collision Detection** — Custom rectangular hitboxes for more forgiving gameplay.
- **Dynamic Scoring** — Score increases as the game progresses.
- **Progressive Difficulty** — Obstacle speed increases with the score, up to a configured limit.
- **Parallax Backgrounds** — Scrolling clouds and mountains create a sense of depth.
- **Pixel-Art Environment** — Textured grass-and-dirt ground and sprite-based graphics.
- **Game Over and Restart** — Restart the game after a collision without relaunching the application.

## Built With

- **Rust** — Game logic, physics, and state management.
- **Macroquad** — 2D rendering, textures, input handling, and the game loop.
- **Cargo** — Dependency management and building the project.

## Prerequisites

Install the following before running the game:

- [Rust and Cargo](https://www.rust-lang.org/tools/install)
- A system supported by Macroquad.

Verify your installation:

```bash
rustc --version
cargo --version
```

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/dewdropx/dino-game.git
cd dino-game
```

### 2. Run the game

```bash
cargo run
```

Cargo will download the required dependencies, compile the project, and launch the game.

## Controls

| Key | Action |
|---|---|
| `Space` | Jump |
| `Up Arrow` | Jump |
| `R` | Restart |

## Project Structure

```text
dino-game/
├── assets/
│   ├── dino_sheet.png
│   ├── sky.png
│   ├── clouds.png
│   ├── mountains.png
│   ├── sprites_misc.png
│   └── mushroom.png
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```
