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

pub struct RedAnnotator {}

impl Annotator for RedAnnotator {
    fn annotate(
        &self,
        _dr: &DictRecord,
        target: &str,
        _def_length: i32,
        _including_phoneme: bool,
    ) -> String {
        let (clean_word, prefix, suffix) = Cleaner::clean_word(target, false);
        let update = format!(
            "{}<span style='color:red'>{}</span>{}",
            prefix,
            clean_word,
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

pub struct ChunkParameter<'a, 'b> {
    pub format: &'b str,
    pub dict: &'a HashMap<String, DictRecord>,
    pub lemma: &'a HashMap<String, String>,
    pub def_length: i32,
    pub including_phoneme: bool,
    pub hint_level: i32,
}
pub type ProcessChunkFn = fn(input: &str, param: &ChunkParameter) -> String;
