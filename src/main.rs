use std::collections::HashMap;
use std::io;
use std::fs::{File, OpenOptions};
use std::io::*;
use csv::ReaderBuilder;

// Function to return the number of lines in a text file
fn count_lines(file_path: &str) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let line_count = reader.lines().count();

    Ok(line_count)
}

// Function to capitalize the first letter in every word
fn uppercase_words(data: &str) -> String {
    // Uppercase first letter in string, and letters after spaces.
    let mut result = String::new();
    let mut first = true;
    for value in data.chars() {
        if first {
            result.push(value.to_ascii_uppercase());
            first = false;
        } else {
            result.push(value.to_ascii_lowercase());
            if value == ' ' {
                first = true;
            }
        }
    }
    result
}

// Function to write to an output file
fn write_to_file(text: &str, file_path: &str) -> io::Result<()> {
    let mut out_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    out_file.write_all(text.as_bytes())?;
    out_file.write_all("\n".as_bytes())?;
    
    Ok(())
}

// Function to wrap a string to lines of no more than 80 characters
fn wrap_text(text: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut curr_line = String::new();

    for word in text.split_whitespace() {
        let word_len = word.len();

        if !curr_line.is_empty() && curr_line.len() + word_len + 1 > 80 {
            lines.push(curr_line.clone());
            curr_line.clear();
        }

        if !curr_line.is_empty() {
            curr_line.push(' ');
        }

        curr_line.push_str(word);
    }

    if !curr_line.is_empty() {
        lines.push(curr_line);
    }

    lines.join("\n")
}

fn search_bible(book: &str, chapter: &str, verse: &str, file: &str) -> io::Result<()> {
    let line_count = count_lines(file).unwrap() - 1;

    let book_name = &(book.to_string())[12..];

    let mut found_book = false;
    let mut found_chapter = false;

    let file = File::open(file)?;
    let reader = BufReader::new(file);

    // Begin scanning through the whole Bible
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;

        // Found the book
        if line.contains(&book) {
            found_book = true;
            continue;
        }

        // Found the chapter
        if line.contains(&chapter) && found_book {
            found_chapter = true;
            continue;
        }

        // Found the verse
        if line.starts_with(&verse) && found_chapter {
            let chapter_num: Vec<&str> = chapter.split(' ').collect();
            let chapter_num = chapter_num[1];

            let mut out_string = "".to_string() + book_name + " " + chapter_num + ":" + &line;

            out_string = wrap_text(&out_string);

            write_to_file(&out_string, "src/verses.txt")?;
            println!("{}", out_string);
            break;
        }

        // Verse not found error
        if (((line.contains("THE BOOK OF")) || (line.contains("CHAPTER")) || (line.contains("PSALM")) 
            || (line_number == line_count)) && found_book && found_chapter) || verse == "0" {
            
            let chapter_num = uppercase_words(chapter);

            println!("{} of {} does not have a verse {}", chapter_num, book_name, verse);
            break;
        }

        // Chapter not found error
        if (line.contains("THE BOOK OF") || line_number == line_count) && found_book {
            println!("The book of {} does not have a {}", book_name, chapter.to_lowercase());
            break;
        }

        // Book not found error
        if line_number == line_count {
            println!("The Bible does not contain the book of {}", book_name);
            break;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // Initialize variables
    let bible_text_path = "./src/Bible.txt";
    let bible_abbr_path = "./src/Bible_Abbreviations.csv";

    let abbr_file = File::open(bible_abbr_path);
    let mut bible_abbr: HashMap<String, String> = HashMap::new();

    // Create reader on CSV file
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(abbr_file?);

    // Store CSV into map
    for row in reader.records() {
        let record = row?;

        if let Some(column1) = record.get(0) {
            if let Some(column2) = record.get(1) {
                bible_abbr.insert(column1.to_string().to_uppercase(), column2.to_string().to_uppercase());
            }
        }
    }

    // Main searching loop
    loop {
        //Get user input
        let mut book_name = String::new();
        let mut chapter_name = String::new();
        let mut verse_num = String::new();

        println!("Please enter the reference of the verse you would like to retrieve");
        print!(" the book: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut book_name).unwrap();
        print!(" the chapter: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut chapter_name).unwrap();
        print!(" the verse: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut verse_num).unwrap();

        // Format inputs
        book_name = book_name.trim().to_uppercase();
        if bible_abbr.contains_key(&book_name) {
            book_name = bible_abbr.get(&book_name).unwrap().to_string();
        }
        book_name = "THE BOOK OF ".to_string() + &book_name;

        if book_name.contains("PSALMS") {
            chapter_name = "PSALM ".to_string() + &chapter_name.trim();
        }
        else {
            chapter_name = "CHAPTER ".to_string() + &chapter_name.trim();
        }

        verse_num = verse_num.trim().to_string();

        // Search the bible
        search_bible(&book_name, &chapter_name, &verse_num, &bible_text_path)
            .expect("Something went wrong");
        
        // check for exit
        print!("Would you like to find another verse? (Y/N): ");
        io::stdout().flush().unwrap();
        let mut repeat = String::new();
        std::io::stdin().read_line(&mut repeat).unwrap();
        
        repeat = repeat.to_uppercase();
        if !repeat.starts_with("Y") { break; }

    }

    Ok(())
}
