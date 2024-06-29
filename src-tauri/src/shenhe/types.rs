use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tauri::Runtime;

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
    pub fn get_meaning(
        &self,
        def_length: i32,
        user_hint_lvl: i32,
        user_show_phoneme: bool,
    ) -> String {
        let mut definition = String::new();
        if (user_hint_lvl == 0 || user_hint_lvl >= self.hint_lvl)
            && user_show_phoneme
            && !self.phoneme.is_empty()
        {
            definition += &self.phoneme;
        }

        if user_hint_lvl >= self.hint_lvl {
            if def_length == 1 {
                definition += &format!(" {}", self.short_def);
            } else if def_length == 2 {
                definition += &format!(" {}", self.full_def);
            }
        }

        definition.trim().to_string()
    }
}

pub trait Clean {
    fn clean_word(word: &str, lowercase: bool) -> (String, String, String);
}
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

pub enum Annotator<'a> {
    RubyAnnotator(i32, bool),
    ColorAnnotator(&'a str, i32, bool),
    InlineAnnotator(i32, bool),
}

pub fn annotate_text(
    annotator: &Annotator,
    dr: &DictRecord,
    target: &str,
    def_length: i32,
) -> String {
    match annotator {
        Annotator::RubyAnnotator(hint_lvl, phoneme) => {
            let (clean_word, prefix, suffix) = Cleaner::clean_word(target, false);
            let meaning = dr.get_meaning(def_length, *hint_lvl, *phoneme);
            if meaning.len() > 0 {
                let update = format!(
                    "{}<ruby>{}<rt>{}</rt></ruby>{}",
                    prefix, clean_word, meaning, suffix
                );
                return target.replace(target, &update);
            }

            target.to_string()
        }
        Annotator::ColorAnnotator(color, _hint_lvl, _phoneme) => {
            let (clean_word, prefix, suffix) = Cleaner::clean_word(target, false);
            if clean_word.len() > 0 {
                let update = format!(
                    "{}<span style='color:{color}'>{}</span>{}",
                    prefix, clean_word, suffix
                );
                return target.replace(target, &update);
            }
            target.to_string()
        }
        Annotator::InlineAnnotator(hint_lvl, phoneme) => {
            let (clean_word, prefix, suffix) = Cleaner::clean_word(target, false);
            let meaning = dr.get_meaning(def_length, *hint_lvl, *phoneme);
            if meaning.len() > 0 {
                let update = format!(
                    "{}{}<span style='font-size:smaller;color:gray'> [{}]</span>{}",
                    prefix, clean_word, meaning, suffix
                );
                return target.replace(target, &update);
            }
            target.to_string()
        }
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
    pub wordwise_style: i32,
}

pub struct ProgressReporter<'a, R: Runtime> {
    progress_fn: fn(f32, &tauri::Window<R>),
    pub tauri_window: &'a tauri::Window<R>,
}

impl<'a, R: Runtime> ProgressReporter<'a, R> {
    pub fn new(
        tauri_window: &'a tauri::Window<R>,
        progress_fn: fn(f32, &tauri::Window<R>),
    ) -> Self {
        Self {
            progress_fn,
            tauri_window,
        }
    }

    pub fn report(&self, progress: f32) {
        (self.progress_fn)(progress, &self.tauri_window);
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkMesg<'a> {
    pub class_name: &'a str,
    pub text: &'a str,
}

impl<'a> WorkMesg<'a> {
    pub fn new(class_name: &'a str, text: &'a str) -> Self {
        Self { class_name, text }
    }
}

pub struct ChunkParameter<'a> {
    pub format: &'a str,
    pub dict: &'a HashMap<String, DictRecord>,
    pub lemma: &'a HashMap<String, String>,
    pub def_length: i32,
    pub including_phoneme: bool,
    pub hint_level: i32,
    pub annotator: &'a Annotator<'a>,
}
pub type ProcessChunkFn = fn(input: &str, param: &ChunkParameter) -> String;

pub static APP_DATA_DIR: OnceCell<String> = OnceCell::new();
