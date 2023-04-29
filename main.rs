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
    process::ExitCode,
};

const RAW_DATA_MAX: f64 = 256.0;
const BIAS: f64 = 0.7;

#[derive(Debug, Clone, Copy)]
enum Value {
    One,
    Zero,
}

impl Value {
    fn to_char(&self) -> char {
        match self {
            Value::One   => '1',
            Value::Zero  => '0',
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

struct MessageRaw {
    data: Vec<Value>,
}

impl MessageRaw {
    fn parse_message<'a, I>(data: &mut Peekable<I>) -> Self
        where
        I: Iterator<Item = &'a (i32, i32)>,
        {
            use Value::*;

            let mut result = Self {
                data: Vec::new(),
            };
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
                    result.data.push(One);
                } else {
                    result.data.push(Zero);
                }
            }

            result
        }

    fn reversed(&self) -> Self {
        Self {
            data: self.data.iter()
                .rev()
                .map(|x| *x)
                .collect(),
        }
    }
}

impl fmt::Display for MessageRaw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(self.data.len() + self.data.len()/8);

        for (i, val) in self.data.iter().enumerate() {
            s.push(val.to_char());
            // because i is zero-indexed
            if (i+1) % 8 == 0 {
                s.push(' ');
            }
        }

        write!(f, "{}", s)
    }
}

struct FileData {
    data: Vec<MessageRaw>,
    path: String,
}

impl FileData {
    fn parse_file(data: Vec<(i32, i32)>) -> Vec<MessageRaw> {
        let mut data = data.iter().peekable();

        let mut result = vec![];

        while data.peek().is_some() { 
            result.push(MessageRaw::parse_message(&mut data));
        }

        result
    }

    fn new(path: String) -> Option<Self> {
        let file = File::open(&path).map_err(|e| {
            eprintln!("ERROR: Could not open file `{path}`: {e}");
        }).ok()?;
        let buf = BufReader::new(file);

        let data: Vec<_> = buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .map(|l| l.parse::<i32>())
            .filter(|l| l.is_ok())
            .map(|l| l.unwrap())
            .map(|l| l as f64 / RAW_DATA_MAX)
            .map(|l| if l > BIAS { 1 } else { 0 })
            .collect();

        let data = group(data);

        let ret = Self {
            data: Self::parse_file(data),
            path
        };

        Some(ret)
    }

    fn dump(&self, reverse: bool) {
        println!("{}:", self.path);
        for d in &self.data {
            if reverse {
                println!("{}:\t{}", d.data.len(), d.reversed());
            } else {
                println!("{}:\t{}", d.data.len(), d);
            }
        }
        println!();
    }
}

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

fn entry() -> Result<(), ()> {
    let mut args = env::args().peekable();

    let _program = args.next();

    let mut reverse: bool = false;

    while let Some(arg) = args.peek() {
        match &arg[..] {
            "-r" | "--reverse" => {
                reverse = true;
                args.next();
            }
            _ => {
                break;
            }
        }  
    }

    if !args.peek().is_some() {
        eprintln!("ERROR: Not enough arguments. Provide file to parse.");
        return Err(());
    }

    let mut datas = vec![];

    for path in args {
        let data = FileData::new(path).ok_or(())?;
        datas.push(data);
    }

    for data in &datas {
        data.dump(reverse);
    }
    Ok(())
}

fn main() -> ExitCode {
    match entry() {
        Err(()) => ExitCode::FAILURE,
        Ok(())  => ExitCode::SUCCESS,
    }
}
