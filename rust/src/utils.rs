use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
    str::FromStr,
};

fn get_lines(path: &Path) -> Result<BufReader<File>> {
    let f = File::open(path)?;
    Ok(BufReader::new(f))
}

fn buffer_to_vec<T: FromStr>(mut buffer: BufReader<File>) -> Result<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut ret = Vec::new();
    loop {
        let mut line = String::new();
        match buffer.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return Ok(ret);
                }
                ret.push(line.trim().parse().unwrap());
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
}

pub fn read_data<T: FromStr>(path: &Path) -> Result<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let buf_reader = get_lines(path)?;
    buffer_to_vec(buf_reader)
}
