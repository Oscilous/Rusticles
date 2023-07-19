# Rusticles
Simple sand/water particle cellular automata implemented in Rust

## Running the project
Before you start, make sure you have rustc and cargo installed.
You can check with
```sh
rustc --version
```
```sh
cargo --version
```
1. Clone the repo

```sh
git clone https://github.com/Oscilous/Rusticles.git
```

2. Change directory to the Rusticles project
   
```sh
cd Rusticles
```

3. Build the executable
```sh
cargo build
```
4. Run the executable
```sh
cargo run
```
## Interacting with the simulation
* Left click on a empty pixel, to place a particle.
  * Sand is selected by default.
* To change selected particle, left click on the wanted particle.
* Additionally, selected particle can be selected with the keyboard.
  * Pressing "w" will select water as the selected particle.
  * Pressing "s" will select sand as the selected particle.
