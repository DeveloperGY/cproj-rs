use std::{collections::HashMap, hash::Hash};

pub struct ArgRetriever {
    tagged: HashMap<ArgRule, Option<Vec<String>>>,
    untagged: Vec<String>,
}

impl ArgRetriever {
    pub fn new(rules: &[ArgRule]) -> Self {
        let mut tagged = HashMap::new();

        for rule in rules {
            tagged.insert(rule.clone(), None);
        }

        Self {
            tagged,
            untagged: vec![],
        }
    }

    pub fn load(&mut self, args: &[&str]) {
        // clear currently loaded argset
        self.tagged.iter_mut().for_each(|(_, value)| *value = None);

        self.untagged.clear();

        // load in args
        let mut i = 0;
        while i < args.len() {
            // this will only ever contain 0 or 1 key
            let possible_keys: Vec<ArgRule> = self
                .tagged
                .keys()
                .filter(|k| **k == ArgRule::new(args[i], 0))
                .cloned()
                .collect();

            // tagged value
            if !possible_keys.is_empty() {
                let key = possible_keys[0].clone();
                let arg_count = key.get_arg_count();

                let mut sub_args = vec![None; arg_count];

                for j in 1..=arg_count {
                    sub_args[j - 1] = args.get(i + j).map(|val| val.to_string());
                }

                let sub_args: Option<Vec<String>> = sub_args.into_iter().collect();
                self.tagged.insert(key, sub_args);
                i += arg_count;
            }
            // untagged value
            else {
                self.untagged.push(args[i].to_string());
            }
            i += 1;
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.get_tag_args(tag).is_some()
    }

    pub fn get_tag_args(&self, tag: &str) -> Option<Vec<String>> {
        let possible_sub_args = self.tagged.get(&ArgRule::new(tag, 0))?;

        possible_sub_args.clone()
    }

    pub fn get_untagged(&self) -> Vec<String> {
        self.untagged.clone()
    }
}

/// Weve implemented Hash and PartialEq the way we have since
/// we can only have 1 arg rule per tag, regardless of if they have different
/// argument counts or not
///
#[derive(Clone, Eq)]
pub struct ArgRule {
    tag: String,
    arg_count: usize,
}

impl ArgRule {
    pub fn new(tag: &str, arg_count: usize) -> Self {
        Self {
            tag: tag.to_string(),
            arg_count,
        }
    }

    pub fn get_arg_count(&self) -> usize {
        self.arg_count
    }
}

impl PartialEq for ArgRule {
    fn eq(&self, other: &Self) -> bool {
        self.tag.as_str() == other.tag.as_str()
    }
}

impl Hash for ArgRule {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
    }
}
