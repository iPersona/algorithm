pub trait MyStringExtension {
    fn substring(&self, start: usize, len: usize) -> &str;
}

impl MyStringExtension for String {
    fn substring(&self, start: usize, len: usize) -> &str {
        let s = self.char_indices().nth(start).map(|(i, _)| i).unwrap();
        let e = self.char_indices()
            .nth(start + len)
            .map(|(i, _)| i)
            .unwrap();
        &self.chars().as_str()[s..e]
    }
}


#[test]
fn test_string_extension_substring_chinese() {
    use utils::string::MyStringExtension;
    let txt = "中文测试";
    let txt_string = String::from(txt);
    let sub_str = txt_string.substring(1, 2);
    assert_eq!("文测", sub_str);
}

#[test]
fn test_string_extension_substring_asiic() {
    use utils::string::MyStringExtension;
    let txt = "hello";
    let txt_string = String::from(txt);
    let sub_str = txt_string.substring(1, 2);
    assert_eq!("el", sub_str);
}

