use CLI_App::grep;

#[test]
fn grep_test() {
    assert!(grep::grep(&vec!["executable.exe".to_string(), "app".to_string(), "poem.txt".to_string()]).is_ok())
}