fn main() {
    // Part 1
    let mut my_score: i32 = 0;
    let mut opponent_score: i32 = 0;
        if let Ok(input) = std::fs::read_to_string("./input") {
        let game = input.split("\n");
        for game_round in game {
            if game_round == "" {
                break;
            }
            let hands: Vec<&str> = game_round.split(" ").collect();
            let mut my_hand = hands[1];
            let mut opponent_hand = hands[0];
            match opponent_hand {
                "A" => {
                    opponent_hand = "Rock";
                    opponent_score += 1;
                },
                "B" => {
                    opponent_hand = "Paper";
                    opponent_score += 2;
                },
                "C" => {
                    opponent_hand = "Scissor";
                    opponent_score += 3;
                },
                _ => {},
            };
            match my_hand {
                "X" => {
                    my_hand = "Rock";
                    my_score += 1;
                    if opponent_hand == "Scissor" {
                        my_score += 6;
                    } else if opponent_hand == "Paper" {
                        opponent_score += 6;
                    }
                },
                "Y" => {
                    my_hand = "Paper";
                    my_score += 2;
                    if opponent_hand == "Rock" {
                        my_score += 6;
                    } else if opponent_hand == "Scissor" {
                        opponent_score += 6;
                    }
                },
                "Z" => {
                    my_hand = "Scissor";
                    my_score += 3;
                    if opponent_hand == "Paper" {
                        my_score += 6;
                    } else if opponent_hand == "Rock" {
                        opponent_score += 6;
                    }
                },
                _ => {},
            };
            if my_hand == opponent_hand {
                my_score += 3;
                opponent_score += 3;
            }
        }
        println!("Part1: {}", my_score);
    }
    // Part 2
    let mut my_score: i32 = 0;
    let mut _opponent_score: i32 = 0;
        if let Ok(input) = std::fs::read_to_string("./input") {
        let game = input.split("\n");
        for game_round in game {
            if game_round == "" {
                break;
            }
            let hands: Vec<&str> = game_round.split(" ").collect();
            let mut my_hand = hands[1];
            let mut opponent_hand = hands[0];
            match opponent_hand {
                "A" => {
                    opponent_hand = "Rock";
                    opponent_score += 1;
                },
                "B" => {
                    opponent_hand = "Paper";
                    opponent_score += 2;
                },
                "C" => {
                    opponent_hand = "Scissor";
                    opponent_score += 3;
                },
                _ => {},
            };
            match my_hand {
                "X" => { // lose
                    opponent_score += 6;
                    match opponent_hand {
                        "Rock" => {
                           my_hand = "Scissor";
                           my_score += 3; 
                        },
                        "Paper" => {
                            my_hand = "Rock";
                            my_score += 1;
                        },
                        "Scissor" => {
                            my_hand = "Paper";
                            my_score += 2;
                        },
                    _ => {},
                    };
                },
                "Y" => { // draw
                    my_score += 3;
                    opponent_score += 3;
                    match opponent_hand {
                        "Rock" => {
                            my_hand = "Rock";
                            my_score += 1
                        },
                        "Paper" => {
                            my_hand = "Paper";
                            my_score += 2;
                        },
                        "Scissor" => {
                            my_hand = "Scissor";
                            my_score += 3;
                        },
                        _ => {},
                    };
                },
                "Z" => { // win
                    my_score += 6;
                    match opponent_hand {
                        "Rock" => {
                            my_hand = "Paper";
                            my_score += 2;
                        },
                        "Paper" => {
                            my_hand = "Scissor";
                            my_score += 3;
                        },
                        "Scissor" => {
                            my_hand = "Rock";
                            my_score += 1;
                        },
                        _ => {},
                    };
                },
                _ => {},
            };
        }
        println!("Part2: {}", my_score);
    }
}