extern crate serde;
extern crate serde_json;

use std::fs::{remove_file, File};
use std::path::Path;

extern crate rand;
use rand::distributions::{IndependentSample, Range};

use utils::system::pause;
use tree::red_black_tree::RBTree;

pub struct RBTreeHelper {}

impl RBTreeHelper {
    pub fn hit_branch(index: Branch) {
        // TODO: just for debuging
        RBTreeDebug::new().hint_branch(index)
    }

    pub fn init_hints() {
        let path = Path::new(RESULT_FILE);
        if !path.exists() {
            return;
        }

        let _r = remove_file(path);
    }

    pub fn is_all_hitted() -> bool {
        RBTreeDebug::new().is_all_hitted()
    }

    pub fn match_coverage() {
        Self::rand_tree(|nodes: &Vec<char>, tree: &mut RBTree<char>| {
            let target = Self::random_letter(nodes);
            println!("target: {:?}", target);
            tree.remove(&target);
            println!("removed: {:?}", tree.pre_order_with_color().unwrap());
            if Self::is_all_hitted() {
                pause();
            }
        });
    }

    pub fn rand_tree<F>(mut opr: F)
    where
        F: FnMut(&Vec<char>, &mut RBTree<char>),
    {
        loop {
            println!("============================================");
            // clear hints
            Self::init_hints();

            let mut tree = RBTree::new();

            let letters = Self::random_unique_letters();
            if letters.len() == 1 {
                continue;
            }
            for l in &letters {
                tree.insert(l.clone());
            }
            println!("letters: {:?}", letters);
            opr(&letters, &mut tree);
        }
    }

    fn random_unique_letters() -> Vec<char> {
        let alphabet = vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        let mut rng = rand::thread_rng();
        let size_range = Range::new(1, 27);
        let size = size_range.ind_sample(&mut rng);

        let mut letters = Vec::new();
        for _i in 0..size {
            let mut letter_index: usize;
            loop {
                let letter_range = Range::new(0, 26);
                letter_index = letter_range.ind_sample(&mut rng);
                if !letters.contains(&alphabet[letter_index]) {
                    break;
                }
            }

            letters.push(alphabet[letter_index]);
        }

        letters
    }

    pub fn random_letter(letters: &Vec<char>) -> char {
        let letters_num = letters.len();
        let mut rng = rand::thread_rng();
        let size_range = Range::new(0, letters_num);
        let index = size_range.ind_sample(&mut rng);
        letters[index]
    }
}

pub enum Branch {
    One = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
}

// TODO: act as a static global variable
#[derive(Serialize, Deserialize)]
pub struct RBTreeDebug {
    hints: Vec<bool>,
}

static RESULT_FILE: &str =
    // "/Users/iPersona/Dropbox/private/Projects/github/algorithm/target/debug/branch_hint.json";
    "target/debug/branch_hint.json";

impl RBTreeDebug {
    pub fn new() -> Self {
        RBTreeDebug { hints: vec![] }
    }

    pub fn hint_branch(&mut self, branch: Branch) {
        let path = Path::new(RESULT_FILE);
        if !path.exists() {
            let f = File::create(path).unwrap();
            serde_json::to_writer_pretty(f, &self).unwrap();
        }
        let f = File::open(path).unwrap();
        let mut v: RBTreeDebug = serde_json::from_reader(f).unwrap();
        if v.hints.len() == 0 {
            for _i in 0..5 {
                v.hints.push(false);
            }
        }
        v.hints[branch as usize] = true;
        let f2 = File::create(path).unwrap();
        let json_str = serde_json::to_string(&v).unwrap();
        println!("JSON: {}", json_str);
        let _ret = serde_json::to_writer_pretty(f2, &v);
    }

    pub fn is_all_hitted(&self) -> bool {
        let path = Path::new(RESULT_FILE);
        let f = File::open(path).unwrap();
        let v: RBTreeDebug = serde_json::from_reader(f).unwrap();
        v.hints.iter().all(|&x| x == true)
    }
}
