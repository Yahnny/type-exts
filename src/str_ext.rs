pub trait StringExtensions {
    // fn find_next_char(&self, target_char: char) -> Option<usize>;
    fn find_start_at(&self, search_term: &str, starting_index: usize) -> Option<usize>;
    fn find_start_at(&self, search_terms: Vec<&str>, starting_index: usize) -> Option<usize>;
    // fn index_of_next(&self, target_char: char) -> Option<usize>;
    fn char_at(&self, index: usize) -> Option<char>;
    fn substring(&self, start: usize, len: usize) -> Option<String>;
    fn slice(&self, start: usize, end: usize) -> Option<String>;
}

impl StringExtensions for String {
    fn substring(&self, start: usize, len: usize) -> Option<String> {
        substring(self, start, len)
    }

    fn char_at(&self, index: usize) -> Option<char> {
        char_at(self, index)
    }

    fn find_start_at(&self, search_term: &str, starting_index: usize) -> Option<usize> {
        return find_start_at(self, search_term, starting_index);
    }
    
    fn find_start_at(&self, search_terms: Vec<&str>, starting_index: usize) -> Option<usize> {
        let mut closest_index = 0;
        for search_term in search_terms {
            let start = find_start_at(search_term, starting_index);

            match start {
                Some(start) => {
                    if start <= closest_index {
                        closest_index = start
                    }
                },
                None => panic!("find_start_at: failed")
            }
        }
    }

    fn slice(&self, start: usize, end: usize) -> Option<String> {
        return slice(&self, start, end);
    }
}

impl StringExtensions for &str {
    fn substring(&self, start: usize, len: usize) -> Option<String> {
        substring(&self, start, len)
    }

    fn char_at(&self, index: usize) -> Option<char> {
        char_at(self, index)
    }

    fn find_start_at(&self, search_term: &str, starting_index: usize) -> Option<usize> {
        return find_start_at(&self, search_term, starting_index);
    }

    fn slice(&self, start: usize, end: usize) -> Option<String> {
       return slice(&self, start, end); 
    }
}

// write tests for this to prove out
fn substring(string: &str, start: usize, len: usize) -> Option<String> {
    let content = string.to_string();
    let mut index = start;
    let end = start + len;
    let mut result = String::new();

    if end >= string.chars().count() {
        return None;    
    }

    while index < end {
        let ch = content.char_at(index);
        match ch {
            Some(ch) => result.push(ch),
            None => return None
        } 

        index += 1;
    } 

    Some(result)
}

fn char_at(string: &str, index: usize) -> Option<char> {
    let length = string.chars().count();

    if length > 0 || length > index {
        let char = string.chars().nth(index);
        return char;
    }

    return None;
}

fn slice(string: &str, start: usize, end: usize) -> Option<String> {
    if  end >= string.chars().count() {
        return None;    
    }
    
    let mut result = String::new();
    for (i, ch) in string.char_indices() {
        if i < start {
            continue;
        } 
        else if i > end {
            break;
        }

        result.push(ch);
    }

    Some(result)
}

fn find_start_at(string: &str, search_term: &str, starting_index: usize) -> Option<usize> {
    let slice = &string[starting_index..];
    let index_in_slice = slice.find(search_term);

    match index_in_slice {
        Some(index_in_slice) => return Some(starting_index + index_in_slice),
        None => return None
    };
}
// fn find_next_char(&self, target_char: char) -> Option<usize> {
//     return self.chars().position(|c| c == target_char);
// }

// fn index_of_next(&self, target_char: char) -> Option<usize> {
//     return self.chars().position(|c| c == target_char);
// }


// fn find_start_at(&self, search_term: &str, starting_index: usize) -> Option<usize> {
//     let slice = &self[starting_index..];
//     let index_in_slice = slice.find(search_term);

//     match index_in_slice {
//         Some(index_in_slice) => return Some(starting_index + index_in_slice),
//         None => return None
//     };
// }
//unit tests
#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    // char_at tests
    #[test]
    fn test_char_at() {

        let input = String::from("test");
        let index = 3;

        let actual = input.char_at(index);

        match actual {
            Some(actual) => assert_eq!('t', actual),
            None => panic!()
        }
    }

    #[test]
    fn test_char_at_out_of_index_returns_none() {

        let input = String::from("test");
        let index = 4;

        let actual = input.char_at(index);
        
        assert_eq!(false, actual.is_some());
    }

    // find_start_at tests
    #[test]
    fn find_start_at_returns_correct_index() {
        let str = String::from("TestTestTest");
        let expected = 5;

        let actual = str.find_start_at("est", 3);

        match actual {
            Some(actual) => assert_eq!(expected, actual),
            None => panic!()
        }
    }

    #[test]
    fn slice_returns_correct_string() {
        let str = String::from("This is an example string");
        let expected = "example";

        let actual = str.slice(11, 17);

        match actual {
            Some(actual) => assert_eq!(expected, actual),
            None => panic!()
        }
    }

    #[test]
    fn slice_returns_none_when_end_is_out_of_bounds() {
        let str = String::from("This is an example string");

        let actual = str.slice(11, 25);

        assert!(actual.is_none());
    }

    #[test]
    fn substring_returns_correct_string() {
        let str = String::from("This is an example string");
        let expected = "example";

        let actual = str.substring(11, 7);

        match actual {
            Some(actual) => assert_eq!(expected, actual),
            None => panic!()
        }
    }

    #[test]
    fn substring_returns_none_when_end_is_out_of_bounds() {
        let str = String::from("This is an example string");

        let actual = str.substring(11, 25);

        assert!(actual.is_none());
    }
}