# Rewrites: [3D Version](https://github.com/lucascompython/particle-simulation-3d/) - [2D Version](https://github.com/lucascompython/particle-simulation-2d/)



# Particle Attraction/Repulsion

This is a satisfying "simulation" of particles that attract and repel to the mouse. It's not really a simulation, but it looks cool.  

Keybindings:

- `R`: reset the particles
- `SPACE/M1 Click`: toggle the mouse attraction/repulsion
- `B`: toggle the screen border collision
- `Arrow Keys (Up/Down/Left/Right)`: Add/Remove 1/1/100/100 particles
- `J/K`: decrease/increase the multipler
- `+/-`: increase/decrease ball size
- `ESC`: quit  

More options with ./particles --help


## Demo

https://github.com/lucascompython/particles/assets/77930083/ee3eb461-5a3a-4dbc-801a-0812c62102f9

## Compilation

Install raylib on your system, then adjust the [build.rs](/build.rs) file to point to the raylib library.  
Then, just run `cargo build --release` to compile.  
I will maybe add Android and Web support in the future.  

## TODO

- [ ] Add Android support
- [ ] Add Web support
- [ ] Add a GUI  
