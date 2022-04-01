use std::fs::File;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

const MAX_SINGLE: usize = 12;
const ITEM_PP: usize = 4;
const OUT_HEADER: &[&str] = &[
    "person", "sweet1", "sweet2", "sweet3", "sweet4", "savory1", "savory2", "savory3", "savory4",
];

#[derive(Debug, Serialize, Deserialize)]
struct Named {
    name: String,
}

fn make_random_sample(vec: &[Named]) -> Vec<&Named> {
    let mut rng = rand::thread_rng();

    let mut buf: Vec<_> = (0..vec.len())
        .cycle()
        .take(vec.len() * MAX_SINGLE)
        .map(|i| &vec[i])
        .collect();

    buf.shuffle(&mut rng);
    buf
}

fn main() {
    let read_csv = |path| {
        let file = File::open(path).unwrap();
        let mut reader = csv::Reader::from_reader(file);
        reader.deserialize().map(Result::unwrap).collect()
    };

    let people_input: Vec<Named> = read_csv("people.csv");
    let sweet_input: Vec<Named> = read_csv("sweet.csv");
    let savory_input: Vec<Named> = read_csv("savory.csv");

    let sweet_sample = make_random_sample(&sweet_input);
    let savory_sample = make_random_sample(&savory_input);

    let output_file = File::create("output.csv").unwrap();
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(output_file);

    let mut sweet_iter = sweet_sample.iter();
    let mut savory_iter = savory_sample.iter();

    wtr.write_record(OUT_HEADER).unwrap();

    for person in people_input {
        wtr.write_field(&person.name).unwrap();

        for &sweet in sweet_iter.by_ref().take(ITEM_PP) {
            wtr.write_field(&sweet.name).unwrap();
        }

        for &savory in savory_iter.by_ref().take(ITEM_PP) {
            wtr.write_field(&savory.name).unwrap();
        }

        wtr.write_record(None::<&[u8]>).unwrap();
    }

    wtr.flush().unwrap();
}
