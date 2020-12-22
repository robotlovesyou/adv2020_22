use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

fn read_decks<'a>(lines: impl Iterator<Item = &'a str>) -> (VecDeque<i32>, VecDeque<i32>) {
    let mut player_1 = VecDeque::new();
    let mut player_2 = VecDeque::new();

    let mut current_player = &mut player_1;

    for line in lines {
        if line.starts_with("Player") {
            continue;
        }
        if line.is_empty() {
            current_player = &mut player_2;
            continue;
        }
        if let Ok(n) = line.parse::<i32>() {
            current_player.push_back(n);
        }
    }

    (player_1, player_2)
}

fn round(player_1: &mut VecDeque<i32>, player_2: &mut VecDeque<i32>) {
    let card_1 = player_1.pop_front().expect("player 1 has no cards");
    let card_2 = player_2.pop_front().expect("player 2 has no cards");

    if card_1 > card_2 {
        player_1.push_back(card_1);
        player_1.push_back(card_2);
    } else {
        player_2.push_back(card_2);
        player_2.push_back(card_1);
    }
}

fn game(player_1: &mut VecDeque<i32>, player_2: &mut VecDeque<i32>) {
    while !player_1.is_empty() && !player_2.is_empty() {
        round(player_1, player_2);
    }
}

fn score(player: &VecDeque<i32>) -> usize {
    player
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * *card as usize)
        .sum()
}

fn part_1(player_1: &mut VecDeque<i32>, player_2: &mut VecDeque<i32>) -> usize {
    game(player_1, player_2);
    if player_1.is_empty() {
        score(player_2)
    } else {
        score(player_1)
    }
}

enum RoundResult {
    Player1Win(VecDeque<i32>),
    Player2Win(VecDeque<i32>),
    Continue(VecDeque<i32>, VecDeque<i32>, HashSet<u64>),
}

use self::RoundResult::*;

fn round_2(
    mut player_1: VecDeque<i32>,
    mut player_2: VecDeque<i32>,
    mut previous: HashSet<u64>,
) -> RoundResult {
    let mut hasher = DefaultHasher::new();
    player_1.hash(&mut hasher);
    player_2.hash(&mut hasher);
    let hash = hasher.finish();
    if previous.contains(&hash) {
        return Player1Win(player_1);
    }
    previous.insert(hash);
    let card_1 = player_1.pop_front().expect("player 1 has no cards");
    let card_2 = player_2.pop_front().expect("player 2 has no cards");

    let player_1_won_round =
        if player_1.len() >= card_1 as usize && player_2.len() >= card_2 as usize {
            let new_player_1 = player_1
                .iter()
                .take(card_1 as usize)
                .cloned()
                .collect::<VecDeque<i32>>();
            let new_player_2 = player_2
                .iter()
                .take(card_2 as usize)
                .cloned()
                .collect::<VecDeque<i32>>();
            match game_2(new_player_1, new_player_2) {
                Player1Win(_) => true,
                Player2Win(_) => false,
                _ => panic!("bad result"),
            }
        } else {
            card_1 > card_2
        };

    if player_1_won_round {
        player_1.push_back(card_1);
        player_1.push_back(card_2);
        if player_2.is_empty() {
            return Player1Win(player_1);
        }
    } else {
        player_2.push_back(card_2);
        player_2.push_back(card_1);
        if player_1.is_empty() {
            return Player2Win(player_2);
        }
    }

    Continue(player_1, player_2, previous)
}

fn game_2(player_1: VecDeque<i32>, player_2: VecDeque<i32>) -> RoundResult {
    let mut state = Continue(player_1, player_2, HashSet::new());
    while let Continue(player_1, player_2, previous) = state {
        let new_state = round_2(player_1, player_2, previous);
        state = new_state;
    }
    state
}

fn part_2(player_1: VecDeque<i32>, player_2: VecDeque<i32>) -> usize {
    let winning_deck = match game_2(player_1, player_2) {
        Player1Win(cards) | Player2Win(cards) => cards,
        _ => panic!("bad final game state"),
    };
    score(&winning_deck)
}

fn main() {
    let (mut player_1, mut player_2) = read_decks(include_str!("../input.txt").lines());
    println!("part 1: {}", part_1(&mut player_1, &mut player_2));

    let (player_1, player_2) = read_decks(include_str!("../input.txt").lines());
    println!("part 2: {}", part_2(player_1, player_2));
}
