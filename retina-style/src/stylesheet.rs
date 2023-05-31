// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{Rule, CascadeOrigin};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stylesheet {
    rules: Vec<Rule>,
}

impl Stylesheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(cascade_origin: CascadeOrigin, input: &str) -> Self {
        crate::parse::parse_stylesheet(cascade_origin, input)
    }

    pub fn push(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }
}
