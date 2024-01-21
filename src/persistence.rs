use std::{
    fs::File,
    io::{self, Write},
};

use serde::{de::DeserializeOwned, Serialize};

pub fn write_to_file<T: Serialize>(data: &T, file_path: &str) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = io::BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, data)?;
    writer.flush()?;
    Ok(())
}

pub fn read_from_file<T: DeserializeOwned + Default + Serialize>(file_path: &str) -> io::Result<T> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            let _ = write_to_file(&T::default(), file_path);
            File::open(file_path)?
        }
    };

    let reader = io::BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}
