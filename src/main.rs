use aya_poker::{base::*, deck::Deck, poker_rank};
use clap::{CommandFactory, Parser, Subcommand};
use colored::Colorize;
use std::cmp::Ordering;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    after_help = "\
<CARDS> examples:
\"As 3c\": ace of spaces and three of clubs.
\"Qd Th\": queen of diamonds and 10 of hearts)
"
)]
struct Args {
    /// Player hand
    #[arg(short, long, value_name = "CARDS", visible_alias = "hero", value_parser = player_parser)]
    player: Option<Hand>,

    /// Opponent hand
    #[arg(short, long, value_name = "CARDS", visible_alias = "villain", value_parser = player_parser)]
    opponent: Option<Hand>,

    /// Board cards
    #[arg(short, long, value_name = "CARDS", value_parser = board_parser)]
    board: Option<Hand>,

    /// Number of iterations
    #[arg(long, default_value = "100000")]
    samples: usize,

    /// RNG seed
    #[arg(long, default_value = "42")]
    seed: u64,

    /// Output style
    #[arg(long, default_value = "pretty")]
    output: Output,

    /// Show performance stats
    #[arg(long)]
    performance: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}

#[derive(clap::ValueEnum, Default, Debug, Clone)]
enum Output {
    #[default]
    Pretty,
    Numeric,
}
fn main() -> Result<(), String> {
    let args = Args::parse();
    if let Some(command) = args.command {
        return match command {
            Command::Completions { shell } => {
                shell.generate(&mut Args::command(), &mut std::io::stdout());
                Ok(())
            }
        };
    }
    let samples = args.samples;
    let seed = args.seed;
    let output = args.output;
    let player = args.player.unwrap_or(Hand::new());
    let opponent = args.opponent.unwrap_or(Hand::new());
    let board = args.board.unwrap_or(Hand::new());

    let start = Instant::now();
    let equity = equity_calculator(&player, &opponent, &board, &samples, &seed);
    if args.performance {
        let duration = start.elapsed().as_millis() as usize;
        let samples_per_milli = samples / duration;
        let performance = format!(
            "{} samples in {:.0} ms - {:.0} samples/ms.",
            samples, duration, samples_per_milli
        );

        eprintln!("{}", performance.cyan());
    }
    match output {
        Output::Numeric => {
            println!("{}", equity);
        }
        Output::Pretty => {
            println!(
                "{} has {:.1}% equity on {} against {}.",
                {
                    if player.is_empty() {
                        "Random hand".to_string()
                    } else {
                        player.to_string()
                    }
                },
                100.0 * equity,
                {
                    if board.is_empty() {
                        "preflop".to_string()
                    } else {
                        board.to_string()
                    }
                },
                {
                    if opponent.is_empty() {
                        "random hand".to_string()
                    } else {
                        opponent.to_string()
                    }
                },
            );
        }
    }
    Ok(())
}

fn player_parser(val: &str) -> Result<Hand, String> {
    hand_parser(val, 2)
}
fn board_parser(val: &str) -> Result<Hand, String> {
    hand_parser(val, 5)
}

fn hand_parser(val: &str, max: usize) -> Result<Hand, String> {
    let hand: Hand = match val.parse::<Hand>() {
        Ok(hand) => {
            if hand.len() > max {
                return Err(format!("Maximum {} cards allowed", max));
            }

            hand
        }
        Err(_) => {
            return Err(format!("Unable to parse {}", val));
        }
    };

    Ok(hand)
}

fn equity_calculator(
    player: &Hand,
    opponent: &Hand,
    board: &Hand,
    samples: &usize,
    seed: &u64,
) -> f64 {
    // To simulate board run-outs, we begin by preparing a deck
    // that doesn't contain the already dealt-out cards
    let available_cards = CARDS
        .iter()
        .filter(|c| !player.contains(c))
        .filter(|c| !opponent.contains(c))
        .filter(|c| !board.contains(c));
    let mut deck = Deck::with_seed(available_cards, *seed);

    let mut pots_won = 0.0;
    for _ in 0..*samples {
        // Then, for each run we draw cards to complete the board
        deck.reset();
        let mut player_hand = *player;
        let player_missing = 2 - player_hand.len();
        if player_missing > 0 {
            player_hand = player_hand
                .iter()
                .chain(deck.deal(player_missing).unwrap().iter())
                .collect::<Hand>();
        }
        let mut opponent_hand = *opponent;
        let opponent_missing = 2 - opponent_hand.len();
        if opponent_missing > 0 {
            opponent_hand = opponent_hand
                .iter()
                .chain(deck.deal(opponent_missing).unwrap().iter())
                .collect::<Hand>();
        }
        let missing = 5 - board.len();
        let complete_board = board
            .iter()
            .chain(deck.deal(missing).unwrap().iter())
            .collect::<Hand>();

        // Evaluate the player's hand given the completed board
        player_hand.extend(complete_board.iter());
        let player_rank = poker_rank(&player_hand);

        // Evaluate the opponent's hand
        opponent_hand.extend(complete_board.iter());
        let opponent_rank = poker_rank(&opponent_hand);

        // And record the player's share of the pot for the run
        match player_rank.cmp(&opponent_rank) {
            Ordering::Greater => pots_won += 1.0,
            Ordering::Less => {}
            Ordering::Equal => pots_won += 0.5,
        };
    }

    pots_won / *samples as f64
}
