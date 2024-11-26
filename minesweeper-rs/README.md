# Minesweeper-rs

Minesweeper-rs is a command-line implementation of the classic Minesweeper game written in Rust.

## Features

- Command-line interface
- Customizable grid size and number of mines
- Safe and efficient memory usage


## Inside the game

```
   0 1 2 3 4 5 6 7 8 91011121314
 0 . . . . . . . . . . . . . . .
 1 . . . . . . . . . . . . . . .
 2 . . . . . . . . . . . . . . .
 3 . . . . . . . . . . . . . . .
 4 . . . . . . . . . . . . . . .
 5 . . . . . . . . . . . . . . .
 6 . . . . . . . . . . . . . . .
 7 . . . . . . . . . . . . . . .
 8 . . . . . . . . . . . . . . .
 9 . . . . . . . . . . . . . . .
10 . . . . . . . . . . . . . . .
11 . . . . . . . . . . . . . . .
12 . . . . . . . . . . . . . . .
13 . . . . . . . . . . . . . . .
14 . . . . . . . . . . . . . . .

Open Cell -> 1 [x] [y]
Flag Cell -> 2 [x] [y]
```

## How to play

- Open a cell: `1 [x] [y]`
- Flag a cell: `2 [x] [y]`