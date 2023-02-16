use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use hash_table::HashTable;

const ENTRIES: usize = 503;

fn main() -> io::Result<()> {
    let nomes_p = Path::new(".\\nomes_10000.txt");
    let nomes_buf = BufReader::new(File::open(nomes_p)?);
    let mut nomes_ht = HashTable::new(ENTRIES);

    for s in nomes_buf.lines() {
        nomes_ht.insert(s?);
    }

    let consultas = Path::new(".\\consultas.txt");
    let mut consultas = parse_file(consultas)?;

    println!("{}", nomes_ht.get_list_statistics());

    Ok(())
}

fn parse_file<P>(path: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let mut res: Vec<String> = Vec::new();
    let file = File::open(path)?;
    let buffer = BufReader::new(file);

    for l in buffer.lines() {
        match l {
            Ok(l) => res.push(l),
            Err(e) => return Err(e),
        }
    }

    Ok(res)
}
