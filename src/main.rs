use ini_rs::Ini;
use std::io::{self, Write};

fn pause() {
    let mut stdout = io::stdout();
    write!(stdout, "Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new()).unwrap();
}


fn main() {
    println!("Testing starting");

    println!("Test 1 = Load a file");

    let mut test1 = match Ini::new(r".\good.ini".to_string()) {
        Ok(ini) => ini,
        Err(e) => panic!("Test failed. {}", e),
    };

    println!("Test passed");

    println!("Test 3 = Dump out contents, check if direct data access works");
    println!();

    for (section_k, section_v) in &test1.config_map {
        println!("[{}]", section_k);

        for (k,v) in section_v {
            println!("{}={}", k, v);
        }
    }

    println!();
    println!("Cannot auto detect pass/fail. Please read the output");

    println!("Test 4 = Change entry using set");

    test1.set("New", "test", "123");

    println!("Next test will also validate this");

    println!("Test 5 = Get value added and check its value");

    match test1.get("New", "test") {
        Some(x) => {
            if x == "123" {
                println!("Test passed");
            }
            else {
                println!("Test failed but value was added. {}", x);
            }
        }
        None => {
            panic!("Test failed")
        },
    }

    println!("Test 6 = Write to file");

    match test1.save() {
        Ok(_) => println!("Test passed"),
        Err(e) => println!("Test failed. {}", e)
    }
    
    println!("Check the good.ini file");
    pause();
    
    test1.remove("New", "test");
    test1.save().unwrap();
    
    println!("Check good.ini file");
    pause();
    
    test1.remove_section("New");
    test1.save().unwrap();
    
    println!("Check good.ini file");
    pause();

    println!("Test 7 = Load an invalid file");

    let test2 = match Ini::new(r".\bad.ini".to_string()) {
        Ok(q) => Some(q),
        Err(_) => None
    };

    match test2 {
        Some(q) => {
            println!("Test failed.");

            for (section_k, section_v) in &q.config_map {
                println!("[{}]", section_k);

                for (k,v) in section_v {
                    println!("{}={}", k, v);
                }
            }
            return;
        }
        None => {
            println!("Test passed");
        }
    }

    println!("Testing complete");
    pause();
}
