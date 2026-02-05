use std::collections::HashMap;
use std::io;
use std::process::exit;

type Department = HashMap<String, Vec<String>>;

fn main() {
    // 1
    let mut l = [3, 5, 2, 1, 8, 2, 1, 1, 2, 3, 4];
    l.sort();
    println!("{:?}", l);
    let median = l[l.len() / 2];
    println!("The median is: {}", median);

    let mut occurances: HashMap<i32, i32> = HashMap::new();
    for i in l {
        let count = occurances.entry(i).or_insert(0);
        *count += 1;
    }
    println!("Occurances of each nmber in the list");
    println!("{:#?}", occurances);

    // 2
    let word = String::from("first");
    let first = word.chars().next().unwrap_or('.');
    let vowel = "aeiou";
    println!("{}", first);
    if vowel.contains(first) {
        let result = format!("{}-hay", word);
        println!("{}", result);
    } else {
        let result = format!("{}-{}ay", &word[1..], first);
        println!("{}", result);
    }

    // 3
    let mut departments: Department = HashMap::new();
    'game_loop: loop {
        println!("Enter your choice: 1-4");
        println!("1. Add a new department");
        println!("2. Add a new employee to an existing department");
        println!("3. See a list of all departments");
        println!("4. Quit");

        let mut choice = String::new();
        match io::stdin().read_line(&mut choice) {
            Ok(val) => val,
            Err(_) => {
                println!("Couldn't read the input. Try again.");
                continue 'game_loop;
            }
        };

        let choice: u8 = match choice.trim().parse() {
            Ok(val) => val,
            _ => {
                println!("Input invalid. See instructions.");
                continue 'game_loop;
            }
        };

        match choice {
            1 => {
                println!("Enter the name of the new department");
                let mut choice = String::new();
                match io::stdin().read_line(&mut choice) {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Couldn't read the input. Try again.");
                        continue 'game_loop;
                    }
                };
                add_department(choice.clone(), &mut departments);
                println!("Added department: {}", choice);
            }
            2 => {
                println!("Enter the name of the employee");
                let mut emp = String::new();
                match io::stdin().read_line(&mut emp) {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Couldn't read the input. Try again.");
                        continue 'game_loop;
                    }
                };

                println!("Enter the name of the department");
                let mut dep = String::new();
                match io::stdin().read_line(&mut dep) {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Couldn't read the input. Try again.");
                        continue 'game_loop;
                    }
                };
                add_employee(emp.clone(), dep.clone(), &mut departments);
                println!("Added employee: {} to department {}", emp, dep);
            }
            3 => print_departments(&departments),
            4 => quit(),
            _ => {
                println!("Input invalid. See instructions.");
                continue;
            }
        }
    }
}

fn add_department(new_dep: String, departments: &mut Department) {
    departments.entry(new_dep).or_insert(vec![]);
}
fn add_employee(emp: String, dep: String, departments: &mut Department) {
    if let Some(employees) = departments.get_mut(&dep) {
        employees.push(emp);
    }
}
fn print_departments(dep: &Department) {
    println!("{:#?}", dep);
}
fn quit() {
    println!("Hope to see you soon!");
    exit(0);
}
