use crate::player::Player;
use std::io::stdin;

pub fn run() {
    println!("\n\n{0} {3} {2} {1} {3} {0}    Black Jack!    {1} {2} {0} {3} {1} {0}\n",
             '\u{2663}', '\u{2664}', '\u{2665}', '\u{2666}'
    );
    let player = input("Enter your name:");
    let mut player = Player::new(&player);
    let mut dealer = Player::new("Dealer");
    let mut deck = Vec::new();
    let mut player_pass = false;
    let mut dealer_pass = false;
    for i in 2..15 {
        deck.push(i);
        deck.push(i);
        deck.push(i);
        deck.push(i);
    }

    loop {
        if dealer.score > 17 {
            dealer_pass = true;
        }
        if !dealer_pass {
            dealer.take_card(&mut deck);
            println!("Диллер берет карту.");
            if dealer.is_overflow() {
                print_result(&dealer, &player, "\u{2665} У диллера преребор! Вы виграли! \u{2663}");
                break;
            }
        }

        if !player_pass {
            let decision = input("Вы хотите взять еще одну карту?(y/n)");
            if decision.as_str() == "y" {
                let card = player.take_card(&mut deck);
                println!("Вам пришла карта {}. всего очков: {}", pretty_card(card), player.score);

                if player.is_overflow() {
                    print_result(&dealer, &player, "\u{2666} У Вас преребор! Вы проиграли! \u{2664}");
                    break;
                }
            } else {
                player_pass = true;
                println!("{} спасовал.", player.name);
            }
        }

        if dealer_pass && player_pass {
            if dealer.score > player.score {
                print_result(&dealer, &player, "\u{2666} Вы проиграли! \u{2665}");
            } else if dealer.score < player.score {
                print_result(&dealer, &player, "\u{2663} Вы Выиграли! \u{2664}");
            } else {
                print_result(&dealer, &player, "\u{2665} Ничья! \u{2665}");
            }
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

fn pretty_card(n: usize) -> String {
    match n {
        2 => String::from("Двойка"),
        3 => String::from("Тройка"),
        4 => String::from("Четверка"),
        5 => String::from("Пятерка"),
        6 => String::from("Шестерка"),
        7 => String::from("Семерка"),
        8 => String::from("Восьмерка"),
        9 => String::from("Девятка"),
        10 => String::from("Десятка"),
        11 => String::from("Валет"),
        12 => String::from("Дама"),
        13 => String::from("Король"),
        14 => String::from("Туз"),
        _ => String::new(),
    }
}

