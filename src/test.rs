use crate::board::Board;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct CsvRecord {
    #[serde(rename = "coordinates")]
    coords: Vec<u32>,
}

pub fn launch() -> Result<(), Box<dyn std::error::Error>> {
    println!("yoloooo");

    let file_path = "./src/test.csv";

    if Path::new(file_path).exists() {
        println!("Le fichier {} existe.", file_path);
        let file = File::open("./src/test.csv")?;

        let mut boards = Vec::new();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(file);
        for result in rdr.deserialize() {
            let mut board = Board::new();
            let record: CsvRecord = result?;
            let mut coords = record.coords.into_iter();
            while let (Some(x), Some(y)) = (coords.next(), coords.next()) {
                _ = board.set_no_cell(x as usize, y as usize);
            }
            boards.push(board);
        }

        for board in boards {
            println!("{}", board);
        }
    } else {
        println!("Le fichier {} n'existe pas.", file_path);
    }
    Ok(())
}
