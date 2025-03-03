// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use font::*;

mod font;
mod markdown;

pub struct Markdown {}
// add Block

pub trait Format {
    fn format(&self) -> String;
}

// block
//  rows: [ Columns ]

pub struct Block {
    pub columns: Vec<Column>,
}

pub struct Column {
    pub rows: Vec<Row>,
}

pub struct Row {
    pub value: String,
    pub padding: usize,
}

// create test
// for each row -> find largest row
//  add padding to every row - so that all have the same length
//
