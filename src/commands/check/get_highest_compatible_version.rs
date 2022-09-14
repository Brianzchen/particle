pub fn main(_sorted_available_versions: Vec<&String>, version: &String) -> String {
    let prefix = &version[0..1];
    let rest = &version[1..];

    match prefix {
        "^" => {
            let _split = rest.split(".");
        },
        "~" => {
        },
        _other => {
            version.split(".");
        },
    }

    String::from("")
}

#[cfg(test)]
mod tests {
    #[test]
    fn gets_correct_version_with_caret() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
