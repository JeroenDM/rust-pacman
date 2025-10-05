# Rust testing: beyond the basics

## The idea

In this repository I experiment with the concepts explained in
[Build bigger in less time: code testing beyond the basics - Predrag Gruevski | EuroRust 2024](https://youtu.be/3EFue8PDyic).

In describes 3 approaches for testing:

- Invariant testing
- Snapshot testing
- Deterministic simulation

I decided a small game would be a good toy project to try out these techniques. So forked the first [pacman implementation](https://github.com/mendess/rust-pacman.git) I could fined and got to work. The game was implemented by [Pedro Mendes](https://github.com/mendess) using using [Piston-rs](https://www.piston.rs/) and OpenGl.

## The twist

After working on it for a couple of hours it became clear that this turned more into an exercise of incrementally refactoring existing code to enable things like deterministic simulation testing. Some examples:

- Remove any use of random numbers without seed. (Not that hard.)
- Add tools to record and replay games. (Generic tooling, maybe reusable?)
- Start different versions of the game (maps, monster locations) from a single input data structure. (It turned out the original code base did not really lend itself for this purpose.)


## The future

Specifically for deterministic simulation testing, it is useful to take this into account from the start. This avoids a bunch of refactoring to get it to work nicely. Maybe I can build a simple game from scratch, or look at [another pacman implementation](https://github.com/Warhorst/pacman) and try the same.


---

*from the original readme:*

## Running
```Bash
cargo run --release --bin pac
```

## Key bindings
Vim keys or arrow keys for movement

`P` to pause

`Q` to quit
