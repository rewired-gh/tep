use std::collections::HashMap;
use crate::node::Node;
use crate::rule::Rule;

pub struct Tree {
    rules: Vec<Rule>,
    rules_from_chars: Vec<Vec<char>>,
    nodes: Vec<Node>,
    children_map: Vec<HashMap<char, usize>>,
}

impl Tree {
    pub fn from(rules: Vec<Rule>) -> Tree {
        let rules_from_chars = rules.iter().map(|rule| rule.from.chars().collect()).collect();

        let mut tree = Tree {
            rules,
            rules_from_chars,
            nodes: Vec::new(),
            children_map: Vec::new(),
        };

        tree.children_map.push(HashMap::new());
        tree.nodes.push(Node {
            rule_index: None,
            children_map_index: 0,
        });

        for index in 0..tree.rules.len() {
            tree.add(index);
        }

        tree
    }

    fn add(&mut self, rule_index: usize) {
        let from = &self.rules_from_chars[rule_index];
        let mut current_index = 0usize;
        let mut current_children_map_index = self.nodes[current_index].children_map_index;

        for ch in from {
            current_index = match self.children_map[current_children_map_index].get(ch) {
                Some(&index) => index,
                None => {
                    self.children_map.push(HashMap::new());
                    self.nodes.push(Node {
                        rule_index: None,
                        children_map_index: self.children_map.len() - 1,
                    });
                    let index = self.nodes.len() - 1;
                    self.children_map[current_children_map_index].insert(ch.clone(), index);
                    index
                }
            };
            current_children_map_index = self.nodes[current_index].children_map_index;
        }

        self.nodes[current_index].rule_index = Some(rule_index);
    }

    pub fn query(&self, chars: &Vec<char>, start: usize, chars_len: usize) -> Option<(&str, usize)> {
        let mut pivot = start;
        let mut depth = 0usize;
        let mut current_index = 0usize;
        let mut current_children_map_index = self.nodes[current_index].children_map_index;
        let mut rule_index: Option<usize> = None;

        loop {
            if pivot >= chars_len {
                break;
            }
            match self.children_map[current_children_map_index].get(&chars[pivot]) {
                Some(&index) => {
                    current_index = index;
                    if let Some(i) = self.nodes[current_index].rule_index {
                        rule_index = Some(i);
                    }
                    current_children_map_index = self.nodes[current_index].children_map_index;
                }
                None => break
            }
            pivot += 1;
            depth += 1;
        }

        match rule_index {
            Some(index) => Some((&self.rules[index].to, depth)),
            None => None
        }
    }
}