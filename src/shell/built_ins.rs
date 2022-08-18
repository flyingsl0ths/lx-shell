pub mod cd {
    use std::{env, path};

    pub fn expand_home_path(
        path: &str,
        home_dir: &Option<String>,
    ) -> Option<String> {
        let mut chars = path.chars();

        if chars.next().unwrap() != '~' || home_dir.is_none() {
            return None;
        }

        let home_dir = home_dir.as_ref().unwrap().clone();

        Some(if path.len() == 1usize {
            home_dir
        } else {
            home_dir + &chars.as_str()
        })
    }

    pub fn cmd(
        home_dir: &Option<String>,
        path: Option<String>,
    ) -> Result<(), std::io::Error> {
        let on_no_path_given = || match home_dir.as_ref() {
            Some(home_dir) => env::set_current_dir(path::Path::new(home_dir)),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Home directory not found",
            )),
        };

        let on_path_given = |mut path: String| {
            if let Some(expanded_path) = expand_home_path(&path, home_dir) {
                path = expanded_path
            }

            if path::Path::new(&path).is_relative() {
                let cwd = env::current_dir()?;

                path = cwd.to_str().unwrap().to_owned() + "/" + &path;
            }

            env::set_current_dir(path::Path::new(&path))
        };

        match path {
            Some(path) => on_path_given(path),
            None => on_no_path_given(),
        }
    }
}

#[cfg(test)]
mod tests {
    mod cd {
        use std::env;

        use super::super::cd::*;

        #[test]
        fn returns_none_when_home_path_is_not_present() {
            assert_eq!(None, expand_home_path("...", &None));
        }

        #[test]
        fn expands_when_path_contains_only_a_tilde() {
            let home_dir = "/home/flyingsloths";

            assert_eq!(
                Some(home_dir.to_string()),
                expand_home_path("~", &Some(home_dir.to_string()))
            );
        }

        #[test]
        fn expands_when_path_contains_a_tilde() {
            let home_dir = "/home/flyingsloths";

            assert_eq!(
                Some(home_dir.to_string() + "/.config/"),
                expand_home_path("~/.config/", &Some(home_dir.to_string()))
            );
        }

        #[test]
        fn when_no_path_is_given_changes_to_home_dir() {
            let test_home_dir = |test_dir_path: &str| {
                env::set_var("HOME", test_dir_path);
                if cmd(&None, Some(test_dir_path.to_string())).is_err() {
                    assert!(false, "Unable to set HOME directory");
                } else {
                    let home_dir = env::var("HOME");
                    assert!(
                        home_dir.is_ok() && home_dir.unwrap() == test_dir_path
                    );
                }
            };

            with_test_dir(test_home_dir, || {
                assert!(false, "Unable to create test directory")
            });
        }

        #[test]
        #[should_panic]
        fn when_path_and_home_dir_not_present_errors() {
            cmd(&None, None).unwrap();
        }

        fn with_test_dir(
            f: impl Fn(&str) -> (),
            on_dir_not_created: impl Fn() -> (),
        ) {
            let test_dir = "test";

            if let Ok(temp_dir) = tempdir::TempDir::new(test_dir) {
                match temp_dir.path().to_str() {
                    Some(temp_dir) => f(temp_dir),
                    None => on_dir_not_created(),
                }
            } else {
                on_dir_not_created();
            }
        }
    }
}
