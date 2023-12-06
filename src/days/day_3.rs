
pub fn exec(input_data: String) {
    let lines = input_data.lines();

    let mut numbers:Vec<Vec<(usize, String)>> = vec![vec![]];
    let mut symbols: Vec<Vec<(usize, &str)>> = vec![vec![]];

    for line in lines {
        numbers.push(find_numbers(line));
        symbols.push(find_symbols(line));
    }

    let mut res = 0;

    // println!("{:?}", numbers);

    // for (line_number, line) in numbers.iter().enumerate() {
    //     for num in line {
    //         let range= (num.0.saturating_sub(1), (num.0 + num.1.len() ));
    //         for (j, line) in symbols.iter().enumerate() {
    //             let mut added = false;
    //             if (j >= line_number.saturating_sub(1) && j <= line_number +1) {
    //                 for symbol in line {
    //                     if symbol.0 >= range.0 && symbol.0 <= range.1 {
    //                         res += num.1.parse::<i32>().unwrap();
    //                         println!("adding {} @ {}:{}, because of symbol @ {}:{}", num.1, line_number, num.0, j, symbol.0);
    //                         added = true;
    //                         break;
    //                     }
    //                 }
    //             }
    //             if added {break;}
    //         }
    //     }
    // }

    let mut vec_symbols: Vec<Symbol> = vec![];

    for (line_n, line) in symbols.iter().enumerate() {
        for symbol in line {
            let mut newsymbol = Symbol{
                symbol: symbol.1.to_string(),
                pos: (symbol.0, line_n),
                part_numbers: vec![],
            };
            for (i, number_line) in numbers.iter().enumerate() {
                if i >= line_n.saturating_sub(1) && i <= line_n + 1 {
                    for number in number_line {
                        if symbol.0 >= number.0.saturating_sub(1) && symbol.0 <= number.0 + number.1.len() {
                            newsymbol.part_numbers.push(PartNumber{
                                num: number.1.to_string(),
                                pos: (symbol.0, i),
                            })
                        }
                    }
                }
            }
            vec_symbols.push(newsymbol);
        }
    }

    let mut res2 = 0;
    for symbol in vec_symbols {
        if symbol.symbol == "*" && symbol.part_numbers.len() == 2 {
            res2 += symbol.part_numbers.get(0).unwrap().num.parse::<i32>().unwrap() * symbol.part_numbers.get(1).unwrap().num.parse::<i32>().unwrap();
        }
    }

    println!("sum of numbers with a symbol next to them (hopefully): {}", res);
    println!("sum of gear rations: {}", res2);
}

fn find_numbers(line: &str) -> Vec<(usize, String)> {
    // let numbers: Vec<(usize, &str)> = line.split(|c:char| c=='.' || !c.is_numeric()).filter(|s| !s.is_empty()).map(
    //     |number| {
    //         (line.find(number).unwrap(), number.trim_matches(|c| c !='.' && !char::is_numeric(c) ))
    //     }
    // ).filter(|n|
    //     !n.1.is_empty()
    // ).collect();
    // let numbers: Vec<&str> = line.split(|c| !char::is_numeric(c)).collect();
    // let mut res:Vec<(usize, &str)> = vec![];
    // for (i, el) in numbers.iter().enumerate() {
    //     if !el.is_empty() {
    //         let last_index = res.last().unwrap_or(&(i.saturating_sub(1), " ")).0; //i=2 for first line
    //         let last_len = res.last().unwrap_or(&(i, " ")).1.chars().count(); //len = 1
    //         let index = i + last_len ;
    //         res.push(
    //         (index , el)); } //
    // }

    let mut res:Vec<(usize, String)> = vec![];

    let digits: Vec<(usize, &str)> = line.match_indices(char::is_numeric).collect();
    // println!("{:?}", digits);
    for digit in digits {
        if res.last().is_some() {
            if digit.0 < res.last().unwrap().0 + res.last().unwrap().1.len() + 1 {
                res.last_mut().unwrap().1.push(digit.1.chars().next().unwrap());
            } else {
                res.push((digit.0, digit.1.to_string()));
            }

        } else {
            res.push((digit.0, digit.1.to_string()));
        }
    }

    // println!("{:?}", res);

    res
}

fn find_symbols(line: &str) -> Vec<(usize, &str)> {
    let symbols = line.match_indices(|c| c !='.' && !char::is_numeric(c) ).collect();

    // println!("{:?}", symbols);

    symbols
}

struct PartNumber {
    num: String,
    pos: (usize, usize),
}

struct Symbol {
    symbol: String,
    pos: (usize, usize),
    part_numbers: Vec<PartNumber>
}