# Rust PID Controller Simulation

This is a demo project of mine based on a previous Python project.

As of writing, it is just a translation of that Python code to Rust, so I can get a feel for the language.
  
- Features:
    - Uses SDL2
      - otherwise written from scratch!
      - I could have used an existing engine, but this was mostly for experiencing the writing and debugging process
    - Currently a bit disorganized and not documented well for others
    - "TODO" comments are being used, still in progress.
    - Functional PID controlled "cube" that follows your cursor. 🙂



Run `cargo run -r` (executable is huge without release mode) to compile and run the program.
- You'll have to have SDL2 installed on your system beforehand - follow the [rust-sdl2 documentation](https://github.com/Rust-SDL2/rust-sdl2/tree/master#requirements) for detailed instructions.