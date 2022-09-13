pub fn main(available_versions: Vec<&String>, version: &String) -> String {
    let prefix = &version[0..1];

    match prefix {
        "^" => {
            let major = &version[1..2];
            if major == "0" {
                // let minor = &version[2..3];
                // let patch = &version[3..4];
                // registry_available_versions.iter().for_each(|(_v, _d)| {

                // });
                String::from("")
            } else {
                String::from("")
            }
        },
        "~" => {
            String::from("")
        },
        _other => {
            String::from("")
        },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn gets_correct_version_with_caret() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
