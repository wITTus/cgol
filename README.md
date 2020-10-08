## Conway's Game of Life in Rust

### Features

- Terminal graphics
- Specify custom amount of columns and rows (`-c`, `-r`)
- Color cells by age
- Configurable speed interval in milliseconds (`-t`)
- Random / empty canvas (`--mode`)
- High terminal resolution mode (`-x`)
- Mark specific patterns with red color (`-m`)
- Insert patterns into canvas (`-i`)
- Load .rle files (`-p`)
- Load .cells files (`-p`)

### Build 

```bash
cargo build --release
```

### Run

![Image](img/normal.png)

```
cd target/release
./cgol
```

### High Resolution Mode

![Image](img/highres.png "2474x450 cells")

Make the font size of your terminal very small (e.g. via `ctrl +/-`). Then:

```
./cgol -x
```

### Load Patterns

![Image](img/pattern.png)

```
./cgol --mode empty -t 10 -x -p ../../patterns/glidergun.cells -i
```

### Mark Patterns in Random Output

```
./cgol -t 50 -p ../../patterns/glider.cells -i -m
```
