# Rust PID Controller Simulation

This is similar to my [other Rust PID Controller simulator](https://github.com/joshuaPurushothaman/rust-pid-sim). The only difference is that the original used SDL2 and this uses [Bevy](https://bevyengine.org/) to run the simulator.

I'm using this to:
- learn Bevy with a familiar project
- expand on the PID Controller simulator
- eventually use it as a small testing ground for some more stuff I've been wanting to implement with:
  - "live plotting"
  - Rendering to a terminal with Unicode and ANSI coloring trickery (something I've done before with C++ and C#)