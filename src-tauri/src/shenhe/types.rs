use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DictRecord {
    pub word: String,
    pub phoneme: String,
    pub full_def: String,
    pub short_def: String,
    pub example_sentences: String,
    pub hint_lvl: i32,
}

impl DictRecord {
    pub fn get_meaning(&self, def_length: i32, including_phoneme: bool) -> String {
        let mut definition = String::new();

        if including_phoneme && !self.phoneme.is_empty() {
            definition += &self.phoneme;
        }

        if def_length == 1 {
            definition += &format!(" {}", self.short_def);
        } else if def_length == 2 {
            definition += &format!(" {}", self.full_def);
        }

        definition.trim().to_string()
    }
}

pub trait Annotator {
    fn annotate(
        &self,
        dr: &DictRecord,
        target: &str,
        def_length: i32,
        including_phoneme: bool,
    ) -> String;
}
pub trait Clean {
    fn clean_word(word: &str, lowercase: bool) -> (String, String, String);
}
pub struct RubyAnnotator {}
pub struct Cleaner {}
impl Clean for Cleaner {
    fn clean_word(word: &str, lowercase: bool) -> (String, String, String) {
        let punctuation: HashSet<char> = " `…*•.?!“”‘’\",:;()[]{}<>'-&#~".chars().collect();

        let cleaned_word = word.trim_matches(|c: char| punctuation.contains(&c));
        let prefix = word
            .chars()
            .take_while(|c| punctuation.contains(c))
            .collect::<String>();
        let suffix = word
            .chars()
            .rev()
            .take_while(|c| punctuation.contains(c))
            .collect::<String>();
        //println!("{} -> {}", word, cleaned_word);
        if lowercase {
            (
                cleaned_word.to_lowercase(),
                prefix.to_string(),
                suffix.chars().rev().collect::<String>(),
            )
        } else {
            (
                cleaned_word.to_string(),
                prefix.to_string(),
                suffix.chars().rev().collect::<String>(),
            )
        }
    }
}
impl Annotator for RubyAnnotator {
    fn annotate(
        &self,
        dr: &DictRecord,
        target: &str,
        def_length: i32,
        including_phoneme: bool,
    ) -> String {
        let (clean_word, prefix, suffix) = Cleaner::clean_word(target, false);
        let update = format!(
            "{}<ruby>{}<rt>{}</rt></ruby>{}",
            prefix,
            clean_word,
            dr.get_meaning(def_length, including_phoneme),
            suffix
        );
        target.replace(target, &update)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Payload {
    pub book: String,
    pub format: String,
    pub language: String,
    pub hint_level: i32,
    pub allow_long: bool,
    pub show_phoneme: bool,
}
