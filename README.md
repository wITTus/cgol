## Conway's Game of Life in Rust

### Features

- Terminal graphics
- Specify custom amount of columns and rows (`-c`, `-r`)
- Color cells by age
- Configurable speed interval in milliseconds (`-t`)
- Random / empty canvas (`--mode`)
- Terminal high resolution mode (`-x`)
- Mark specific patterns with red color (`-m`)
- Insert patterns into canvas (`-i`)
- Load .rle files (`-p`)
- Load .cells files (`-p`)
- Custom rules (`--rule`)

### Build 

```bash
cargo build --release
```

### Run

```
cd target/release
./cgol
```

![Image](img/normal.png)

### High Resolution Mode

Make the font size of your terminal very small (e.g. via `ctrl +/-`). Then:

```
./cgol -x
```

![Image](img/highres.png "2474x450 cells")

### Load Patterns

```
./cgol --mode empty -t 10 -x -p ../../patterns/glidergun.cells -i
```

![Image](img/pattern.png)

### Mark Patterns in Random Output

```
./cgol -t 50 -p ../../patterns/glider.cells -i -m
```

![Image](img/mark.png)

### Custom Rules

Try some [well-known life-like cellular automata](https://www.conwaylife.com/wiki/Cellular_automaton#Well-known_life-like_cellular_automata)!

```
./cgol --rule B0/S8 -x
```

![Image](img/customrule.png)


