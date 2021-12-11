use std::{
    fs::File,
    io::{self, BufRead, BufReader, Result},
    path::Path,
    str::FromStr,
};

pub fn open_file_read(path: &Path) -> io::Result<impl BufRead> {
    Ok(BufReader::new(File::open(path)?))
}

pub fn get_lines(path: &Path) -> Result<BufReader<File>> {
    let f = File::open(path)?;
    Ok(BufReader::new(f))
}

fn match_line_to_type<T: FromStr>(bytes_read: Result<usize>, mut line: String) -> Option<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    match bytes_read {
        Ok(bytes) => {
            if bytes == 0 {
                return None;
            }
            Some(line.split(" ").map(|c| c.trim().parse().unwrap()).collect())
        }
        Err(error) => {
            return None;
        }
    }
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

pub fn read_different_data<T: FromStr, U: FromStr>(path: &Path) -> Result<(Vec<T>, Vec<U>)>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    <U as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buf_reader = get_lines(path)?;
    let mut line = String::new();
    let bytes_read = buf_reader.read_line(&mut line);
    let data = match_line_to_type(bytes_read, line);
    Ok((data.unwrap(), buffer_to_vec::<U>(buf_reader).unwrap()))
}
