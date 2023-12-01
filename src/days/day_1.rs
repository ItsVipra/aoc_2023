pub fn exec(input_data: String) {
    let lines = input_data.lines();

    let mut res = 0;

    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let smushed_numbers = [("oneight", "18"), ("twone", "21"), ("threeight", "38"), ("fiveight", "58"), ("sevenine", "79"), ("eightwo", "82"), ("eighthree", "83"), ("nineight", "98")];

    for line in lines {
        let mut newline = line.to_owned();
        // line.replace("one", "1");
        for num in smushed_numbers {
            newline = newline.replace(num.0, num.1);
        }
        for (i,num) in numbers.iter().enumerate() {
            newline = newline.replace(num, &*(i+1).to_string());
        }
        print!("original: {}, newline: {}", line, newline);

        // let v = line.matches(numbers).collect();
        let num1 = newline.find(char::is_numeric);
        let num2 = newline.rfind(char::is_numeric);

        if let (Some(num1), Some(num2)) = (num1, num2) {
            let digit1 = newline.chars().nth(num1).unwrap().to_digit(10).unwrap();
            let digit2 = newline.chars().nth(num2).unwrap().to_digit(10).unwrap();

            println!(" --- found digits {}, {}", digit1, digit2);
            let number=  digit1 * 10 + digit2;
            res += number;
        }
    }

    println!("Sum of all calibration values: {}", res);
}