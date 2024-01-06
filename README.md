# Particle Attraction/Repulsion

This is a satisfying "simulation" of particles that attract and repel to the mouse. It's not really a simulation, but it looks cool.  

Keybindings:

- `R`: reset the particles
- `SPACE/M1 Click`: toggle the mouse attraction/repulsion
- `B`: toggle the screen border collision
- `Arrow Keys (Up/Down/Left/Right)`: Add/Remove 1/1/100/100 particles
- `ESC`: quit  

More options with ./particles --help



![Showcase](/showcase.mov)


<video width="320" height="240" controls>
  <source src="/showcase.mov" type="video/mp4">
</video>
[Showcase With 10x multiplier](https://raw.githubusercontent.com/lucascompython/particles/main/showcase_multiplier10.mp4)



## Compilation

Install raylib on your system, then adjust the [build.rs](/build.rs) file to point to the raylib library.  
Then, just run `cargo build --release` to compile.  
I will maybe add Android and Web support in the future.  

## TODO

- [ ] Add Android support
- [ ] Add Web support
- [ ] Add a GUI  
