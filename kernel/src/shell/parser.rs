// kernel/src/shell/parser.rs
//
// Krisha Shell — command line parser 🦚
// SCAFFOLDING: Placeholder for Lesson 6 (heap-based parsing).

#![allow(dead_code)]

pub struct ParsedCommand<'a> {
    pub command: &'a str,
}

pub fn parse(_line: &str) -> Option<ParsedCommand<'_>> {
    None
}