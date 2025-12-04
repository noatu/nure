/*
Скласти консольну програму для переведення чисел з комою з однієї системи числення в іншу.
Вхідні данні:
    Система числення: 16
    Діапазон чисел: від 0 до 255
    Точність: 2 знаків після коми
    Організація вводу: файл, значення розділені «;»

Вихідні дані:
    Система числення: 8
    Організація виводу: файл, значення розділені «;»
*/
use std::{env, error::Error, fs};

fn hex_to_bin(s: &str) -> Result<String, String> {
    let mut result = String::new();
    for c in s.chars() {
        result += match c {
            '.' | ',' => ".",
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' | 'a' => "1010",
            'B' | 'b' => "1011",
            'C' | 'c' => "1100",
            'D' | 'd' => "1101",
            'E' | 'e' => "1110",
            'F' | 'f' => "1111",
            _ => return Err(format!("Can't convert `{c}` to binary")),
        }
    }
    Ok(result)
}

fn bin_char_to_oct(s: &str) -> Result<&'static str, String> {
    Ok(match s.trim_end_matches('0') {
        "" => "0",    // 000
        "001" => "1", // 001
        "01" => "2",  // 010
        "011" => "3", // 011
        "1" => "4",   // 100
        "101" => "5", // 101
        "11" => "6",  // 110
        "111" => "7", // 111
        _ => return Err(format!("Can't convert `{s}` to octal")),
    })
}

fn bin_to_oct(s: &str) -> Result<String, String> {
    let (whole, fraction) = if s.contains('.') {
        let mut split = s.split('.');
        (split.next().unwrap(), Some(split.next().unwrap()))
    } else if s.contains(',') {
        let mut split = s.split(',');
        (split.next().unwrap(), Some(split.next().unwrap()))
    } else {
        (s, None)
    };

    let mut whole = String::from(whole);
    while whole.len() % 3 != 0 {
        whole = "0".to_string() + &whole;
    }

    let mut result = String::new();

    for chunk in whole.chars().collect::<Vec<char>>().chunks(3) {
        result += bin_char_to_oct(&chunk.iter().collect::<String>())?;
    }
    if let Some(fract) = fraction {
        result += ".";
        for chunk in fract.chars().collect::<Vec<char>>().chunks(3) {
            result += bin_char_to_oct(&chunk.iter().collect::<String>())?;
        }
    }

    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = env::args().nth(1).expect("Provide input filename");
    let output_file = env::args().nth(2).expect("Provide output filename");

    let contents: String = fs::read_to_string(input_file)?;
    let numbers: Vec<&str> = contents.trim().split(';').collect();

    let mut result: Vec<String> = Vec::new();
    for num in numbers {
        let bin = hex_to_bin(num)?;
        let oct = bin_to_oct(&bin)?;
        result.push(oct);
    }

    println!("{}", result.join(";"));
    fs::write(output_file, result.join(";"))?;
    Ok(())
}
