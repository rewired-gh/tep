/// Overview: Tree to match substring with rule.
use std::collections::HashMap;

use super::node::Node;
use super::rule::Rule;

pub struct Tree {
    rules: Vec<Rule>,
    rules_from_chars: Vec<Vec<char>>,
    nodes: Vec<Node>,
}

impl Tree {
    pub fn from(rules: Vec<Rule>) -> Tree {
        // Improve performance for custom string matching.
        let rules_from_chars = rules
            .iter()
            .map(|rule| rule.from.chars().collect())
            .collect();

        let mut tree = Tree {
            rules,
            rules_from_chars,
            nodes: Vec::new(),
        };

        tree.nodes.push(Node {
            rule_index: None,
            children_map: HashMap::new(),
        });

        for index in 0..tree.rules.len() {
            tree.add(index);
        }

        tree
    }

    fn add(&mut self, rule_index: usize) {
        let from = &self.rules_from_chars[rule_index];
        let mut current_index = 0usize;

        for ch in from {
            current_index = match self.nodes[current_index].children_map.get(ch) {
                Some(&index) => index,
                None => {
                    // "index" will be the index of following node.
                    let index = self.nodes.len();
                    self.nodes[current_index].children_map.insert(*ch, index);
                    self.nodes.push(Node {
                        rule_index: None,
                        children_map: HashMap::new(),
                    });
                    index
                }
            };
        }

        self.nodes[current_index].rule_index = Some(rule_index);
    }

    pub fn query(&self, chars: &[char], start: usize, chars_len: usize) -> Option<(&str, usize)> {
        let mut pivot = start;
        let mut depth = 0usize;
        let mut current_index = 0usize;
        let mut rule_index: Option<usize> = None;

        while pivot < chars_len {
            match self.nodes[current_index].children_map.get(&chars[pivot]) {
                Some(&index) => {
                    current_index = index;
                    if let Some(i) = self.nodes[current_index].rule_index {
                        rule_index = Some(i);
                    }
                }
                None => break,
            }

            pivot += 1;
            depth += 1;
        }

        rule_index.map(|index| (self.rules[index].to, depth))
    }
}
