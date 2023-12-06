pub fn exec (input_data: String) {
    let lines = input_data.lines();

    const LIMITS: (i32, i32, i32) = (12, 13, 14);
    let mut res = 0;
    let mut res2 = 0;

    for (i, line) in lines.enumerate() {
        let mut valid = true;
        let split: Vec<&str> = line.split(&[':', ',', ';']).collect();
        println!("split: {:?}", split);

        let mut min = [0, 0, 0];
        for chunk in &split[1..] {
            let mut more_split = chunk.split_whitespace();
            let num:i32 = more_split.next().unwrap().parse().unwrap();
            if let Some(color) = more_split.next() {
                match color {
                    "red" => {
                        if num > min[0] {
                            min[0] = num;
                        }
                        if num > LIMITS.0 {
                        // println!("Game {} impossible - {} {} exceeds limit {}", i+1, num, color, LIMITS.0);
                        valid = false;
                    }},
                    "green" => {
                        if num > min[1] {
                            min[1] = num;
                        }
                        if num > LIMITS.1 {
                        // println!("Game {} impossible - {} {} exceeds limit {}", i+1, num, color, LIMITS.1);
                        valid = false;
                    }},
                    "blue" => {
                        if num > min[2] {
                            min[2] = num;
                        }
                        if num > LIMITS.2 {
                        // println!("Game {} impossible - {} {} exceeds limit {}", i+1, num, color, LIMITS.2);
                        valid = false;
                    }},
                    _ => {}
                }
            }

        }
        if valid { res += i + 1; }

        //calculate power
        res2 += min[0] * min[1] * min[2];

    }

    println!("Sum of IDs of possible games: {}", res);
    println!("Sum of powers of minimum sets of cubes: {}", res2);
}