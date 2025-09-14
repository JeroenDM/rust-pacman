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
    let mut reader = BufReader::new(file);
    return parse_file(&mut reader);
}

fn parse_file<T: BufRead>(reader: &mut T) -> io::Result<Recording> {
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
    return Ok(recording);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // Helper function to simplify testing
    fn test_parse_file(name: &str, input: &str, expected: Vec<(u64, char)>) {
        let mut cursor = Cursor::new(input);
        let result = parse_file(&mut cursor).unwrap();
        assert_eq!(result, expected, "Test failed: {}", name);
    }

    #[test]
    #[rustfmt::skip]
     fn test_parse_file_different_inputs() {
        test_parse_file("valid input",         "123,a\n456,b\n789,c\n",                       vec![(123, 'a'), (456, 'b'), (789, 'c')]);
        test_parse_file("empty input",         "",                                            vec![]);
        test_parse_file("malformed lines",     "123,a\ninvalid_line\n456,b\nno_comma\n789\n", vec![(123, 'a'), (456, 'b')]);
        test_parse_file("invalid timestamp",   "abc,a\n123,b\nnot_a_number,c\n456,d\n",       vec![(123, 'b'), (456, 'd')]);
        test_parse_file("empty character",     "123,a\n456,\n789,c\n",                        vec![(123, 'a'), (789, 'c')]);
        test_parse_file("unicode characters",  "100,🎵\n200,ñ\n300,中\n",                     vec![(100, '🎵'), (200, 'ñ'), (300, '中')]);
        test_parse_file("multiple characters", "123,abc\n456,xyz\n",                          vec![(123, 'a'), (456, 'x')]);
    }
}
