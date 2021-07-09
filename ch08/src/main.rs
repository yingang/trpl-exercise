use std::collections::HashMap;

fn ex01() {
    let mut input = vec![3, 7, 5, 4, 3, 1, 2, 9, 0];

    let total: i32 = input.iter().sum();
    let average = total as f32 / input.len() as f32;
    println!("the average is {}", average);

    input.sort();
    println!("the sorted input is {:?}", input);
    println!("the item in the middle position is {}", input[input.len() / 2]);

    let mut count = HashMap::new();
    for i in &input {
        let entry = count.entry(i).or_insert(0);
        *entry += 1;
    }
    println!("the item count is {:?}", count);

    let mut most_appeared = 0;
    let mut appeared_times = 0;
    for (&&k, &v) in &count {
        if v > appeared_times {
            most_appeared = k;
            appeared_times = v;
        }
    }
    println!("the most appeared is {}", most_appeared);
}

fn is_vowel(ch: char) -> bool {
    match ch {
        'a'|'e'|'i'|'o'|'u' => true,
        _ => false,
    }
}

fn pig_latin(input: &str) -> String {
    let mut result = String::new();
    let mut starts_with_vowel = false;
    let mut first_ch = ' ';

    let mut first = true;
    for ch in input.chars() {
        if first {
            first_ch = ch;
            starts_with_vowel = is_vowel(first_ch);
            if starts_with_vowel {
                result.push(ch);
            }
            first = false;
        } else {
            result.push(ch);
        }
    }

    if starts_with_vowel {
        result.push_str("hay");
    } else {
        result.push(first_ch);
        result.push_str("ay");
    }
    result
}

fn ex02() {
    println!("first => {}", pig_latin("first"));
    println!("apple => {}", pig_latin("apple"));
}

struct Company {
    departments: HashMap<String, Vec<String>>, // department => employees
}

impl Company {
    fn new() -> Self { Self { departments: HashMap::new() } }

    pub fn add_employee(&mut self, name: &str, to_department: &str) {
        let dept = self.departments.entry(to_department.to_string()).or_insert(Vec::new());
        dept.push(name.to_string());
    }

    pub fn print_all_employees(&self) {
        let mut names = Vec::new();

        for (_, v) in &self.departments {
            for name in v {
                names.push(name);
            }
        }
        names.sort();
        println!("All: {:?}", names);
    }

    pub fn print_employees(&mut self, department: &str) {
        let dept = self.departments.get(department);
        match dept {
            Some(names) => {
                let mut names = names.clone();
                names.sort();
                println!("{}: {:?}", department, names);
            },
            None => {
                println!("the specified department {} does not exist!", department);
            },
        }
    }
}

fn ex03() {
    let mut company = Company::new();

    company.add_employee("Bill Gates", "EM");
    company.add_employee("Anders Hejlsberg", "DEV");
    company.add_employee("Herb Sutter", "DEV");
    company.add_employee("Steve Ballmer", "SALES");

    company.print_all_employees();
    company.print_employees("DEV");
    company.print_employees("SHIELD");
}

fn main() {
    ex01();
    ex02();
    ex03();
}
