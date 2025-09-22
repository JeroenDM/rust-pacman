

How to run it as a simulator?

- Make rendering optional.
- Tick as fast as possible (or slower for visual replays).
- Every tick has an option input event / action by agent.


Main Action types:
- None (just tick and other agents can still execute actions).
- arrow keys
- pause
- quit

Inputs for the simulator?
- Run
- Pause
- Step forward
- Step backwards

Resizing the window is not part of the simulator, but just part of the gui in which we can see the simulator / game in action.

Command line options:
- `--record` Save all input actions with timestamp to file.
- `--nogui` Run headless, without rendering to the screen. (This means you cannot play with keyboard input.)
- `--replay` Replay from recording.


Also implement ghost ai with bt?

How to abstract over the input source, aka easily switch between recording or keyboard input,
without duplicating the whole piston event loop?



## Injecting random numbers

I want to inject a random number generator into the main `Game` data structure without using global variables.
It quickly becomes unwieldy to manage however. For example, consider the folling game struct with nested substructers.


```rust
struct House {}

impl House {
    fn generate(&mut self) {
        for i in self.some_range {
            // use random number here.
            let rand_num = ??;
            self.add_random_wall(rand_num);
        }
    }
}

struct Map {
   houses : Vec<House>
   // other data.
}

struct Game {
    map : Map,
    // other data
}
```


Is there a specific workaround for the case where I want to access two different parts of a struct both as mutable reference?

```rust

struct Map {
    house : House,
    generator : Gen,
}

struct House {}

impl House {
    fn bar(&mut self, &mut Gen) {
       // do stuff
    }
}

impl Map {
    fn foo(&mut self) {
        // This works!
        self.house.bar(&mut self.generator)
    }
}
```

It does work!

https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=%0Astruct+Map+%7B%0A++++house+%3A+House%2C%0A++++rng+%3A+Gen%2C%0A%7D%0A%0Astruct+House+%7B+a+%3A+i32+%7D%0A%0Astruct+Gen+%7B+b+%3A+i32+%7D%0A%0Aimpl+Gen+%7B%0A++++fn+gen_num%28%26mut+self%29+-%3E+i32+%7B%0A++++++++self.b+%2B%3D+1%3B%0A++++++++self.b%0A++++%7D%0A%7D%0A%0Aimpl+House+%7B%0A++++fn+bar%28%26mut+self%2C+g+%3A+%26mut+Gen%29+%7B%0A++++++++self.a+%2B%3D+g.gen_num%28%29%3B%0A++++++++println%21%28%22house.a+%3D+%7B%7D%22%2C+self.a%29%3B%0A++++%7D%0A%7D%0A%0Aimpl+Map+%7B%0A++++fn+foo%28%26mut+self%29+%7B%0A++++++++%2F%2F+let+mut+house_ref+%3A+%26mut+House+%3D+self.house%3B%0A++++++++self.house.bar%28%26mut+self.rng%29%3B%0A++++%7D%0A%7D%0A%0Afn+main%28%29+%7B%0A++++let+mut+m+%3D+Map+%7B%0A++++++++house%3A+House+%7B+a%3A+1%7D%2C%0A++++++++rng%3A+Gen+%7Bb+%3A+2+%7D%2C%0A++++%7D%3B%0A++++%0A++++m.foo%28%29%3B%0A%7D
