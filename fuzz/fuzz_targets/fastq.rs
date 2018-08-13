#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate seq_io;
extern crate criterion;

use std::io::Cursor;
use seq_io::fastq::{Reader, Record};


fuzz_target!(|data: &[u8]| {
    let reader = Cursor::new(data);
    let mut reader = Reader::with_capacity(reader, 3);
    let mut count: usize = 0;

    while let Some(result) = reader.next() {
        if let Ok(record) = result {
            count += record.seq().len();
        }
    }
    criterion::black_box(count);
});
