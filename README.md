## Conway's Game of Life in Rust

### Build 

```bash
cargo build --release
```

### Run

```
cd target/release
./cgol
```

### Fullscreen

```
./cgol -c $COLUMNS -r $[ LINES-3 ] -i 30
```

### High Resolution Mode

```
./cgol -c $[ 2*COLUMNS ] -r $[ 2*LINES - 6 ] -i 30 -x
```
