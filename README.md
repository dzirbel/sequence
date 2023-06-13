A simulation for the board game [Sequence](https://en.wikipedia.org/wiki/Sequence_(game)).

Written to learn and explore [Rust](https://www.rust-lang.org/) and to experiment with simple
strategies for non-human players. The game engine can be found in [src/core](src/core) and provides
a complete runner of the game. Player implementations can be found in [src/players](src/players); so
far, only a simple heuristic-based player exists (which beats a purely random player about 92-8).
