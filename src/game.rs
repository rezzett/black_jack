use crate::player::Player;
use std::io::stdin;
use rand::{thread_rng, Rng};

pub fn run() {
    println!("\n\n{0} {3} {2} {1} {3} {0}    Black Jack!    {1} {2} {0} {3} {1} {0}\n",
             '\u{2663}', '\u{2664}', '\u{2665}', '\u{2666}'
    );
    let player = input("Enter your name:");
    let mut player = Player::new(&player);
    let mut dealer = Player::new("Dealer");

    loop {
        let mut deck = make_deck();
        let bot_decision = bot_decide_stop();
        loop {
            if dealer.score > bot_decision {
                dealer.saved = true;
                println!("The dealer saved");
            }
            if !dealer.saved {
                dealer.take_card(&mut deck);
                println!("The dealer takes a card");
                if dealer.is_overflow() {
                    print_result(&dealer, &player, "\u{2665} Overkill! You win! \u{2663}");
                    break;
                }
            }

            if !player.saved {
                let decision = input("Do you want take one more card?(y/n)");
                if decision.as_str() == "y" {
                    let card = player.take_card(&mut deck);
                    println!("You got a card: [{}]. Your score: {}", pretty_card(card), player.score);

                    if player.is_overflow() {
                        print_result(&dealer, &player, "\u{2666} Overkill! You lose! \u{2664}");
                        break;
                    }
                } else {
                    player.saved = true;
                    println!("{} saved.", player.name);
                }
            }

            if dealer.saved && player.saved {
                if dealer.score > player.score {
                    print_result(&dealer, &player, "\u{2666} You lose! \u{2665}");
                } else if dealer.score < player.score {
                    print_result(&dealer, &player, "\u{2663} You win! \u{2664}");
                } else {
                    print_result(&dealer, &player, "\u{2665} Draw! \u{2665}");
                }
                break;
            }
        }

        if do_play_again() {
            player.reset();
            dealer.reset();
            continue;
        } else {
            println!("Thank you! Good bye!");
            break;
        }
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("[READ ERROR]: Cannot read input");
    buf.trim().to_string()
}

fn print_result(player: &Player, dealer: &Player, message: &str) {
    println!("{}\n {}-{}\n {}-{}", message, dealer.name, dealer.score, player.name, player.score);
}

fn do_play_again() -> bool {
    let decision = input("Do you want play again?(y/n)");
    if &decision == "y" {
        true
    } else {
        false
    }
}

fn make_deck() -> Vec<usize> {
    let mut deck = Vec::new();
    for i in 2..15 {
        deck.push(i);
        deck.push(i);
        deck.push(i);
        deck.push(i);
    }
    deck
}

fn bot_decide_stop() -> usize {
    thread_rng().gen_range(14, 19)
}

fn pretty_card(n: usize) -> String {
    match n {
        2 => String::from("Two"),
        3 => String::from("Three"),
        4 => String::from("Four"),
        5 => String::from("Five"),
        6 => String::from("Six"),
        7 => String::from("Seven"),
        8 => String::from("Eight"),
        9 => String::from("Nine"),
        10 => String::from("Ten"),
        11 => String::from("Jack"),
        12 => String::from("Queen"),
        13 => String::from("King"),
        14 => String::from("Ace"),
        _ => String::new(),
    }
}

