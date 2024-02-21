# master_mind_rust
A Rust-implemented solver for the master mind game.

You can play the master mind game online here → https://webgamesonline.com/mastermind/ .

## build

```shell
cargo build -r
```

## Usage 

```
A master mind solver implemented by Rust.

Usage: master_mind_rust [OPTIONS] [COLOR_NUM] [PIN_NUM]

Arguments:
  [COLOR_NUM]  number of colors [default: 6]
  [PIN_NUM]    number of pins [default: 4]

Options:
  -n, --non-duplicate    codes do not have duplicate colors
  -p, --policy <POLICY>  policy for choosing guess [default: minmax] [possible values: firstpick, minmax]
  -m, --mode <MODE>      mode of execution [default: guess] [possible values: guess, mktree, benchmark]
  -h, --help             Print help
```

### Guess mode

In this mode, the solver attempts to deduce the user's secret code. The user must provide feedback for the solver's guesses. Feedback consists of a pair of the number of hits and blows.

- **hit**: The number of pins that are correctly matched in both color and position to the secret code.
- **blow**: The number of pins that match the secret code in color but are positioned differently.

Example: If secret code is `[1, 2, 3, 4]` and guess code is `[0, 1, 2, 3]`, then the feedback: (hit, blow) is `(0, 3)`.

Example: If secret code is `[1, 2, 3, 4]` and guess code is `[0, 0, 3, 3]`, then the feedback: (hit, blow) is `(1, 0)`.


### Mktree mode

Generates trial paths for all possible secret codes. This mode is useful for evaluating the efficiency of policies. It is also used in benchmark mode.

### Benchmark mode

Measures the time required to make trial paths for various configurations. It is usefull for profiling the performance of this solver.
