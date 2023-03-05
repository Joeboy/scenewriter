use crate::constants::DEFAULT_NUM_SPEAKER_STATS;
use crate::{
    document::{FarceDocument, FarceElement},
    utils::print_underlined,
};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct CharacterStats {
    pub num_speeches: usize,
    pub num_words: usize,
}

pub fn get_character_stats(document: &FarceDocument) -> HashMap<String, CharacterStats> {
    let mut character_stats: HashMap<String, CharacterStats> = HashMap::new();
    for element in &document.elements {
        match element {
            FarceElement::FDialogue(d) => match character_stats.get_mut(&d.character_name) {
                Some(cs) => {
                    cs.num_speeches += 1;
                    cs.num_words += d.get_num_words();
                }
                None => {
                    character_stats.insert(
                        d.character_name.clone(),
                        CharacterStats {
                            num_speeches: 1,
                            num_words: d.get_num_words(),
                        },
                    );
                }
            },
            _ => {}
        }
    }
    character_stats
}

pub fn print_stats(document: &FarceDocument) {
    let character_stats = get_character_stats(document);
    let mut num_actions: usize = 0;
    let mut num_action_words: usize = 0;
    let mut num_dialogues: usize = 0;
    let mut num_dialogue_words: usize = 0;
    let mut num_scenes: usize = 0;
    let mut num_int_scenes: usize = 0;
    let mut num_ext_scenes: usize = 0;
    for element in &document.elements {
        match element {
            FarceElement::FDialogue(d) => {
                num_dialogues += 1;
                num_dialogue_words += d.get_num_words();
            }
            FarceElement::FAction(a) => {
                num_actions += 1;
                num_action_words += a.get_num_words();
            }
            FarceElement::FSceneHeading(sh) => {
                num_scenes += 1;
                if sh.int_or_ext == "INT" {
                    num_int_scenes += 1;
                }
                if sh.int_or_ext == "EXT" {
                    num_ext_scenes += 1;
                }
            }
            FarceElement::FPageBreak => {}
        }
    }

    let mut sorted_speakers: Vec<_> = character_stats.iter().map(|(k, v)| (k, v)).collect();
    sorted_speakers.sort_by_key(|(_, v)| -(v.num_words as i32));

    println!();
    println!("=== STATS (not 100% trustworthy) ===");
    println!("");
    println!("{} distinct character names", character_stats.len());
    println!("{} dialogue sections", num_dialogues);
    println!("{} words of dialogue", num_dialogue_words);
    println!("{} actions", num_actions);
    println!("{} words of action", num_action_words);
    println!("{} Scenes", num_scenes);
    println!("{} interior scenes", num_int_scenes);
    println!("{} exterior scenes", num_ext_scenes);

    println!();
    if sorted_speakers.len() > DEFAULT_NUM_SPEAKER_STATS {
        println!("Top {} characters", DEFAULT_NUM_SPEAKER_STATS);
        println!("================");
    } else {
        println!("Characters");
        println!("==========");
    }
    for (character_name, cs) in sorted_speakers.iter().take(DEFAULT_NUM_SPEAKER_STATS) {
        print_underlined(character_name);
        println!("Total words: {}", cs.num_words);
        println!("Dialogue sections: {}", cs.num_speeches);
        println!();
    }
    println!();
}
