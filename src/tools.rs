use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

pub type Recording = Vec<(u64, char)>;

pub fn write_recording_to_file(recording: &Recording, filename: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for (timestamp, character) in recording {
        writeln!(writer, "{},{}", timestamp, character)?;
    }

    writer.flush()?;
    Ok(())
}

pub fn read_recording_from_file(filename: &str) -> io::Result<Recording> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut recording = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((timestamp_str, char_str)) = line.split_once(',') {
            if let (Ok(timestamp), Some(character)) =
                (timestamp_str.parse::<u64>(), char_str.chars().next())
            {
                recording.push((timestamp, character));
            }
        }
    }

    Ok(recording)
}
