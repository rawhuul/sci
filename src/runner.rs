use regex::Regex;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::utils::Utils;

const PROGRAMMING_STACK_PATTERNS: [(&'static str, &'static str); 5] = [
    ("Rust", r"\.rs$"),
    ("Python", r"\.py$"),
    ("Node.js", r"package\.json$"),
    ("C++", r"\.cpp$"),
    ("C", r"\.c$"),
];

const TEST_TOOL_PATTERNS: [(&'static str, &'static str); 5] = [
    ("Unit tests", r"(^test_|^test-|_test\.|_test$)"),
    ("Pytest", r"^pytest\.ini$"),
    ("Jest", r"jest.config.js$"),
    ("Catch2", r"#include <catch2/catch.hpp>"),
    ("CUnit", r"#include <CUnit/CUnit.h>"),
];

#[derive(Debug, Clone, Copy)]
pub struct StackInfo {
    stack: Option<&'static str>,
    test_tool: Option<&'static str>,
}

impl StackInfo {
    fn new() -> Self {
        Self {
            stack: None,
            test_tool: None,
        }
    }
}

pub struct TestRunner {
    dir: PathBuf,
    info: StackInfo,
}

impl TestRunner {
    pub fn new(dir: &PathBuf) -> Self {
        Self {
            dir: dir.into(),
            info: StackInfo::new(),
        }
    }

    pub fn identify(&mut self) -> StackInfo {
        let mut stack_info = StackInfo {
            stack: None,
            test_tool: None,
        };

        let dir = self.dir.clone();
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if let Some(file_name) = entry.file_name().to_str() {
                for (stack_name, pattern) in PROGRAMMING_STACK_PATTERNS.iter() {
                    let regex = Regex::new(pattern).unwrap();
                    if regex.is_match(file_name) {
                        stack_info.stack = Some(*stack_name);
                        break;
                    }
                }

                for (tool_name, pattern) in TEST_TOOL_PATTERNS.iter() {
                    let regex = Regex::new(pattern).unwrap();
                    if regex.is_match(file_name) {
                        stack_info.test_tool = Some(*tool_name);
                        break;
                    }
                }
            }
        }

        self.info = stack_info;
        stack_info
    }

    pub fn run(&self) {
        let dir = Utils::get_full_path(&self.dir);

        if self.info.stack.is_none() && self.info.test_tool.is_none() {
            eprintln!("Unable to identify stack in '{}'", dir);
        } else if self.info.test_tool.is_none() {
            eprintln!("Unable to identify stack in '{}'", dir);
        } else {
            if self.info.stack == Some("Rust") && self.info.test_tool == Some("Unit tests") {}
        }
    }
}
