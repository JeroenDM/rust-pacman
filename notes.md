

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
