pub mod grep {
    use std::error::Error;
    use std::{fs, process};
    use std::fmt::{Debug, Display, Formatter};

    pub fn grep<'a>(args: &'a [String]) -> Result<(), Box<dyn Error + 'a>> {
        let config = match Config::new(&args) {
            Ok(config) => config,
            Err(message) => {
                return Err(Box::new(GrepError { message }));
            }
        };
        println!("config:{:?}", config);

        let content_of_file = match read_file(&config.file_name) {
            Ok(content) => content,
            Err(message) => {
                return Err(message);
            }
        };

        let _lines_with_query = match search(&config.query, &content_of_file) {
            Ok(lines_with_query) => {
                print_found_lines(&lines_with_query);
                lines_with_query
            }
            Err(message) => {
                return Err(Box::new(GrepError { message }));
            }
        };
        Ok(())
    }

    struct GrepError {
        message: String,
    }

    impl Debug for GrepError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl Display for GrepError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl Error for GrepError {}

    fn print_found_lines(lines_with_query: &[String]) {
        for line in lines_with_query {
            println!("{}", line);
        }
    }

    fn read_file(file_name: &str) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(file_name)?;
        Ok(content)
    }

    fn search(query: &str, content_of_file: &str) -> Result<Vec<String>, String> {
        let mut lines_with_query = Vec::new();
        for line in content_of_file.lines() {
            if line.contains(query) {
                lines_with_query.push(line.to_string());
            }
        }
        if lines_with_query.is_empty() {
            Err("No Text Matching query was found".to_string())
        } else {
            Ok(lines_with_query)
        }
    }

    #[derive(Debug)]
    struct Config {
        query: String,
        file_name: String,
    }

    impl Config {
        fn new(args: &[String]) -> Result<Config, String> {
            if args.len() < 3 {
                return Err("arguments are less".to_string());
            }
            if args.len() > 3 {
                return Err("arguments are two many".to_string());
            }
            let query = args[1].clone();
            let file_name = args[2].clone();
            Ok(Config {
                file_name,
                query,
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::grep::*;

        #[test]
        fn when_passing_less_arguments_error() {
            let args = vec!["executable".to_string()];
            assert!(Config::new(&args).is_err())
        }

        #[test]
        fn when_passing_excess_arguments_error() {
            let args = vec!["executable".to_string(), "1".to_string(), "1".to_string(), "1".to_string(), "1".to_string()];
            assert!(Config::new(&args).is_err())
        }

        #[test]
        fn when_passing_correct_number_of_arguments_success() {
            let args = vec!["executable".to_string(), "app".to_string(), "poem.txt".to_string()];
            assert!(Config::new(&args).is_ok())
        }

        #[test]
        fn search_passing_query_and_content() {
            let query = "app";
            let content_of_file = "\
me
and you
goin
app the forest
            ";
            match search(query, content_of_file) {
                Ok(lines_of_query) => assert_eq!(vec!["app the forest".to_string()], lines_of_query),
                Err(e) => panic!("Unknown Error")
            }
        }
    }

    #[test]
    fn read_test_file() {
        let expected_content = "test".to_string();
        match read_file("test_file.txt") {
            Ok(content) => assert_eq!(content.trim(), expected_content),
            Err(e) => panic!("Unknown Error:{:?}", e)
        }
    }
}

