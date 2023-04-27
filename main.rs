use std::{
    fs::{
        File,
    },
    io::{
        prelude::*,
        BufReader,
    },
    env,
    fmt,
    iter::Peekable,
};

const RAW_DATA_MAX: f64 = 256.0;
const BIAS: f64 = 0.7;

type FileData = Vec<Vec<Value>>;

fn group<T: std::cmp::PartialEq>(vec: Vec<T>) -> Vec<(T, i32)> {
    let mut result: Vec<(T, i32)> = Vec::new();
    for x in vec {
        if result.len() == 0 {
            result.push((x, 1));
            continue;
        }

        let mut last = result.last_mut().unwrap();

        if last.0 == x {
            last.1 += 1;
        } else {
            result.push((x, 1));
        }
    }
    result
}

#[derive(Debug, Clone, Copy)]
enum Value {
    One,
    Zero,
}

impl Value {
    pub fn to_char(&self) -> char {
        match self {
            Value::One   => '1',
            Value::Zero  => '0',
        }
    }

    pub fn to_char_invert(&self) -> char {
        match self {
            Value::Zero  => '1',
            Value::One   => '0',
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Value::One   => "1",
            Value::Zero  => "0",
        })
    }
}

fn parse_data<'a, I>(data: &mut Peekable<I>) -> Vec<Value> 
where
    I: Iterator<Item = &'a (i32, i32)>,
{
    use Value::*;

    let mut result = Vec::new();
    let d = data;

    {
        let b1 = d.next().unwrap();
        assert!(b1.0 == 1 && b1.1 >= 27 && b1.1 <= 30);
        let b0 = d.next().unwrap();
        assert!(b0.0 == 0 && b0.1 >= 78 && b0.1 <= 80);
    }

    while d.peek().is_some() {
        let high = d.next().unwrap();
        assert!(high.0 == 1);
        let low = d.next().unwrap();
        assert!(low.0 == 0);
        if low.1 > 200 {
            return result;
        }
        if low.1 > 7 {
            result.push(One);
        } else {
            result.push(Zero);
        }
    }

    result
}

fn parse_many_datas(data: Vec<(i32, i32)>) -> Vec<Vec<Value>> {
    let mut data = data.iter().peekable();

    let mut result = vec![];

    while data.peek().is_some() { 
        result.push(parse_data(&mut data));
    }

    result
}

fn values_to_str(vs: &Vec<Value>, invert: bool) -> String {
    let mut s = String::with_capacity(vs.len());

    for (i, val) in vs.iter().enumerate() {
        s.push(if !invert { val.to_char() } else { val.to_char_invert() });
        // because i is zero-indexed
        if (i+1) % 8 == 0 {
            s.push(' ');
        }
    }

    s
}

fn dump_file_data(path: &String, data: &FileData, invert: bool, reverse: bool) {
    println!("{path}:");
    for d in data {
        if reverse {
            println!("{}:\t{}", d.len(), values_to_str(
                    &(d.into_iter().rev().map(|x| *x).collect()), invert));
        } else {
            println!("{}:\t{}", d.len(), values_to_str(d, invert));
        }
    }
    println!();
}

fn main() -> Result<(), ()> {
    let mut args = env::args().peekable();

    let _program = args.next();

    let mut debug: bool = false;
    let mut invert: bool = false;
    let mut reverse: bool = false;

    while let Some(arg) = args.peek() {
        match &arg[..] {
            "--debug" => {
                debug = true;
                args.next();
            }
            "--invert" => {
                invert = true;
                args.next();
            }
            "-r" | "--reverse" => {
                reverse = true;
                args.next();
            }
            _ => {
                break;
            }
        }  
    }

    let mut datas: Vec<(String, _)> = vec![];

    for path in args {
        if debug {
            println!("==================================================");
            println!("{}", path);
        }

        let file = File::open(&path).map_err(|e| {
            eprintln!("ERROR: Could not open file `{path}`: {e}");
        })?;
        let buf = BufReader::new(file);

        let data: Vec<_> = buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .map(|l| l.parse::<i32>())
            .filter(|l| l.is_ok())
            .map(|l| l.unwrap())
            .map(|l| l as f64 / RAW_DATA_MAX)
            .map(|l| if l > BIAS { 1 } else { 0 })
            .collect();

        if debug {
            for x in &data {
                print!("{}", x);
            }
            println!();
        }

        let data = group(data);

        if debug {
            println!("{}: {:?}\n", path, data);
        }

        let data: Vec<Vec<Value>> = parse_many_datas(data);

        datas.push((path, data));
    }

    for (path, data) in &datas {
        dump_file_data(&path, &data, invert, reverse);
    }
    Ok(())
}
