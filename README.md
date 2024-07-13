# poker-eval
A fast command-line equity evaluator for Texas Hold'em poker. Based on the excellent [aya_poker](https://docs.rs/aya_poker/latest/aya_poker/) crate.

## Installation
### Download
1. Download the [latest release](https://github.com/mortenscheel/poker-eval/releases/latest) for your platform.
2. Extract the `poker-eval` binary

### Git + Cargo
1. `git clone https://github.com/mortenscheel/poker-eval`
2. `cd poker-eval`
3. `cargo install --path .`

## Usage
```bash
$ poker-eval --help
Command-line poker equity evaluator

Usage: poker-eval [OPTIONS] [COMMAND]

Commands:
  completions  Generate shell completions
  help         Print this message or the help of the given subcommand(s)

Options:
  -p, --player <CARDS>
          Player hand [aliases: hero]
  -o, --opponent <CARDS>
          Opponent hand [aliases: villain]
  -u, --unknown-opponents <UNKNOWN_OPPONENTS>
          Unknown (random) opponents [default: 0]
  -b, --board <CARDS>
          Board cards
      --samples <SAMPLES>
          Number of iterations [default: 100000]
      --seed <SEED>
          RNG seed [default: 42]
      --output <OUTPUT>
          Output style [default: pretty] [possible values: pretty, numeric]
      --performance
          Show performance stats
  -h, --help
          Print help
  -V, --version
          Print version

<CARDS> examples:
"As 3c": Ace of spaces and three of clubs.
"Qd Th": Queen of diamonds and 10 of hearts.
```

### Examples
```bash
# Preflop comparison
$ poker-eval --player "Ah Kc" --opponent "Qs Qd"
K♣ A♥ has 43.0% equity on preflop against Q♦ Q♠.

# Preflop against unknown
$ poker-eval --player "8c 8s"
8♣ 8♠ has 69.0% equity on preflop against random hand.

# Postflop against unknown
$ poker-eval --player "7s Ad" --board "Tc 7c 8h"
7♠ A♦ has 57.3% equity on 7♣ 8♥ T♣ against random hand.

# Postflop comparison
$ poker-eval --player "7s Ad" --board "Tc 7c 8h" --opponent "8d Ks"
7♠ A♦ has 21.9% equity on 7♣ 8♥ T♣ against 8♦ K♠.

# Comparison against single known card
$ poker-eval --player "9h 9c" --opponent "Ad"
9♣ 9♥ has 61.8% equity on preflop against A♦.

# Increased sample size and performance info
$ poker-eval --hero "Ad 9c" --villain "Kh Qh" --board "9d Qs 3c" --samples 25000000 --performance
25000000 samples in 2518 ms - 9928 samples/ms.
9♣ A♦ has 20.2% equity on 3♣ 9♦ Q♠ against Q♥ K♥.

# Multiple opponents
$ poker-eval --player "6d 6c" --opponent "Th Jh" --opponent "8c Qs"
6♣ 6♦ has 31.6% equity on preflop against [T♥ J♥, 8♣ Q♠].

# Extra unknown opponents (random hands)
poker-eval --player "6d 6c" --opponent "Th Jh" --opponent "8c Qs" --unknown-opponents 2
6♣ 6♦ has 21.8% equity on preflop against [T♥ J♥, 8♣ Q♠, random hand, random hand].
```