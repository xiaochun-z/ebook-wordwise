use csv::{Reader, ReaderBuilder};
use regex::{self, Replacer};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::result::Result;

const WORDWISE_DICTIONARY_PATH: &str = "resources/wordwise-dict.";
const LEMMA_DICTIONARY_PATH: &str = "resources/lemmatization-en.csv";

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
    fn get_meaning(&self, def_length: i32, including_phoneme: bool) -> String {
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

pub fn load_dict(lang: &str) -> Result<HashMap<String, DictRecord>, Error> {
    let wordwise_dict_path = format!("{}{}.csv", WORDWISE_DICTIONARY_PATH, lang);
    let file = match File::open(&wordwise_dict_path) {
        Ok(file) => file,
        Err(e) => {
            println!("{:?}", e);
            if e.kind() == ErrorKind::NotFound {
                println!("{} not found", wordwise_dict_path);
            }
            return Err(e);
        }
    };

    let mut reader = Reader::from_reader(file);

    let mut wordwise_dict: HashMap<String, DictRecord> = HashMap::new();

    for result in reader.records() {
        let record = result?;
        let (word, phoneme, full_def, short_def, example_sentences, hint_lvl) = (
            record.get(1).unwrap().to_string(),
            record.get(2).unwrap().to_string(),
            record.get(3).unwrap().to_string(),
            record.get(4).unwrap().to_string(),
            record.get(5).unwrap().to_string(),
            record.get(6).unwrap().parse::<i32>().unwrap(),
        );
        wordwise_dict.insert(
            word.clone(),
            DictRecord {
                word: word.clone(),
                phoneme,
                full_def,
                short_def,
                example_sentences,
                hint_lvl,
            },
        );
    }
    //println!("{:?}", wordwise_dict.get("amperage"));

    Ok(wordwise_dict)
}

pub fn load_lemma() -> Result<HashMap<String, String>, Error> {
    let file = File::open(LEMMA_DICTIONARY_PATH)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    let lemma_dict: HashMap<_, _> = reader
        .records()
        .map(|result| {
            result.map(|record| {
                (
                    record.get(1).unwrap().to_string(),
                    record.get(0).unwrap().to_string(),
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .collect();

    Ok(lemma_dict)
}

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

fn annotate_phrase(
    sentence: &str,
    dict: &HashMap<String, DictRecord>,
    lemma_dict: &HashMap<String, String>,
    including_phoneme: bool,
    hint_level: i32,
) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut result = String::new();
    let mut i = 0;
    let max_phrase_length = 5;
    let def_length = 1;

    while i < words.len() {
        let mut longest_match = None;
        let mut longest_length = 0;

        // Try to find the longest phrase in the dictionary, up to max_phrase_length words
        for j in (i + 1)..=words.len() {
            if j - i > max_phrase_length {
                break;
            }
            let phrase: String = words[i..j].join(" ");
            let (cleaned_phrase, _, _) = clean_word(&phrase, false);

            if dict.contains_key(&cleaned_phrase) {
                let length = j - i;
                if length > longest_length {
                    longest_length = length;
                    longest_match = Some(phrase);
                }
            }
        }

        if let Some(phrase) = longest_match {
            //println!("{} -> {}", phrase, longest_length);
            let dict_record = get_dict_record(phrase.as_str(), dict, lemma_dict, hint_level);
            match dict_record {
                Some(dr) => {
                    result.push_str(&format!(
                        "{} ",
                        wrap_with_ruby_tag(dr, phrase.as_str(), def_length, including_phoneme)
                    ));
                }
                None => {
                    result.push_str(&format!("{} ", words[i]));
                }
            }
            i += longest_length;
        } else {
            // If no phrase matches, check for individual word match

            let dict_record = get_dict_record(words[i], dict, lemma_dict, hint_level);

            match dict_record {
                Some(dr) => {
                    result.push_str(&format!(
                        "{} ",
                        wrap_with_ruby_tag(dr, words[i], def_length, including_phoneme)
                    ));
                }
                None => {
                    result.push_str(&format!("{} ", words[i]));
                }
            }
            i += 1;
        }
    }

    result.trim_end().to_string()
}

fn get_dict_record<'a>(
    word: &str,
    wordwise_dict: &'a HashMap<String, DictRecord>,
    lemma_dict: &HashMap<String, String>,
    hint_level: i32,
) -> Option<&'a DictRecord> {
    let (clean_word, _, _) = clean_word(word, true);
    // First, find the word in the wordwise dictionary
    if let Some(dict_record) = wordwise_dict.get(clean_word.as_str()) {
        // Skip the word if hint level is not met
        if dict_record.hint_lvl > hint_level {
            return None;
        }
        return Some(dict_record);
    }

    // Not found, and it's not a phrase, find its normal form
    if !word.contains(' ') {
        if let Some(normal_form) = lemma_dict.get(clean_word.as_str()) {
            // Then, find the normal form word in the wordwise dictionary
            if let Some(dict_record) = wordwise_dict.get(normal_form) {
                // Skip the word if hint level is not met
                if dict_record.hint_lvl > hint_level {
                    return None;
                }
                return Some(dict_record);
            }
        }
    }

    None
}

fn wrap_with_ruby_tag(
    ws: &DictRecord,
    target: &str,
    def_length: i32,
    including_phoneme: bool,
) -> String {
    let (clean_word, prefix, suffix) = clean_word(target, false);
    let update = format!(
        "{}<ruby>{}<rt>{}</rt></ruby>{}",
        prefix,
        clean_word,
        ws.get_meaning(def_length, including_phoneme),
        suffix
    );
    target.replace(target, &update)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dict() {
        let lang = "en";
        let result = load_dict(lang);

        match result {
            Ok(dict) => {
                assert!(!dict.is_empty());
                assert!(dict.get("amperage").is_some());
            }
            Err(_) => {
                // Add your assertions for error cases
                panic!("Failed to load dictionary");
            }
        }
    }

    #[test]
    fn test_load_lemma() {
        let result = load_lemma();
        match result {
            Ok(dict) => {
                assert!(!dict.is_empty());
                //println!("{:?}", dict);
                assert_eq!(dict.get("zips").unwrap(), "zip");
            }
            Err(_) => {
                panic!("Failed to load lemmatization dictionary");
            }
        }
    }

    #[test]
    fn test_clean_word() {
        let test_cases = vec![
            (
                ", Hello, World，大家！!*•-&",
                true,
                "hello, world，大家！",
                ", ",
                "!*•-&",
            ),
            (
                "Hello, World，大家！!*•-&",
                false,
                "Hello, World，大家！",
                "",
                "!*•-&",
            ),
        ];

        for (word, lowercase, expected_cleaned_word, expected_prefix, expected_suffix) in test_cases
        {
            let (cleaned_word, prefix, suffix) = clean_word(word, lowercase);
            assert_eq!(cleaned_word, expected_cleaned_word);
            assert_eq!(prefix, expected_prefix);
            assert_eq!(suffix, expected_suffix);
        }
    }

    #[test]
    fn test_get_dict_record() {
        let word = "riboses";
        let wordwise_dict = load_dict("en").unwrap();
        let lemma_dict = load_lemma().unwrap();
        let result = get_dict_record(word, &wordwise_dict, &lemma_dict, 1);
        assert!(result.is_some());
        match result {
            Some(dict_record) => {
                //println!("{:?}", dict_record);
                assert_eq!(dict_record.word, "ribose");
            }
            None => {
                panic!("Failed to find word in dictionary");
            }
        }
    }

    #[test]
    fn test_get_meaning() {
        let word = "pictorial";
        let dict = load_dict("en").unwrap();
        let dict_record = dict.get(word).unwrap();
        assert_eq!(dict_record.phoneme, "/pɪkˈtɔriəl/");
        assert_eq!(
            dict_record.full_def,
            "of or relating to painting or drawing"
        );
        let res = dict_record.get_meaning(2, true);
        assert_eq!(res, "/pɪkˈtɔriəl/ of or relating to painting or drawing");
    }

    #[test]
    fn test_wrap_with_ruby_tag() {
        let word = "pictorials.";
        let dict = load_dict("en").unwrap();
        let dict_record = dict.get("pictorial").unwrap();
        let res = wrap_with_ruby_tag(&dict_record, word, 2, true);
        assert_eq!(
            res,
            "<ruby>pictorials<rt>/pɪkˈtɔriəl/ of or relating to painting or drawing</rt></ruby>."
        );
        let res = wrap_with_ruby_tag(&dict_record, word, 1, false);
        assert_eq!(
            res,
            "<ruby>pictorials<rt>relating to a drawing</rt></ruby>."
        );
    }

    #[test]
    fn test_annotate_phrase() {
        let data = vec![("I think this is in someone's pocket, but I'm not ascertained.","I think this is <ruby>in someone's pocket<rt>under someone's control</rt></ruby>, but I'm not <ruby>ascertained<rt>discovered by a method</rt></ruby>."),
        ("I think this is in someone's pocket but I'm not ascertained","I think this is <ruby>in someone's pocket<rt>under someone's control</rt></ruby> but I'm not <ruby>ascertained<rt>discovered by a method</rt></ruby>")];

        let hashes = load_dict("en").unwrap();
        let lemma = load_lemma().unwrap();

        for (input, output) in data {
            let result = annotate_phrase(input, &hashes, &lemma, false, 1);
            assert_eq!(result, output);
        }
    }
}
