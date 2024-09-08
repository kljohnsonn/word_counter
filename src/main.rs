use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::{usize, vec};


#[derive(Debug, Eq)]
struct Word {
    pub word: String,
    pub count: u32,
}


#[derive(Debug)]
struct WordList {
    pub words: HashMap<String, Word>,

}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.count.partial_cmp(&other.count)
    }
} 

impl Ord for Word {

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}


impl Word {

    pub fn new(word: &str) -> Self {
        Self {
            word: word.to_string(),
            count: 1,

        }
    } 
}


fn vec_to_string(vec: Vec<&str>) -> String {
    return vec.into_iter().map(|s| s.to_string()).collect();
}


impl WordList {
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
        }
    }

    pub fn sort(&mut self) -> Vec<&mut Word> {
        let list = &mut self.words;

        let mut words: Vec<&mut Word> = self.words.values_mut().collect();
        words.sort();

        

        words

    }
    

    pub  fn find(&mut self, word: String) -> Option<&mut Word> {

        let list = &mut self.words;

        if let Some(word) = list.get_mut(&word) {
            word.count += 1;
            Some(word)
        } else {
            None
        }
       
    }
}

// open file, read each word on each line, for each word, look in wordlist, if word exist, increment Word struct counter by 1, if not
// construct a new word and add it to the Wordlist.



fn main() -> std::io::Result<()> {
    let mut txt= File::open("test.txt")?;
    let mut contents = String::new();

    txt.read_to_string(&mut contents)?;

    println!("{:?}\n", contents);
    let contents_lower = contents.to_lowercase();
    let words: Vec<&str> = contents_lower.split(" ").collect();
    let mut word_list = WordList::new();


    for word in words {
        if word_list.find(word.to_string().to_lowercase()).is_some() {
            println!("Word found");

        } else {
            println!("Word not found!");

            let word_entry = Word::new(word);
            word_list.words.insert(word.to_string().to_lowercase(), word_entry);
        }
    }
    word_list.sort();
    println!("{:?}", word_list.sort());
    

    Ok(())

}
