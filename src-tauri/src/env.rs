pub mod env {
    use dotenvy;

    static DEFAULT_HASH: &str = "fja;qrupzx/.m";

    #[allow(dead_code)]
    pub fn get_salt_for_pw_hash() -> String {
        dotenvy::dotenv().ok();
        std::env::var("PW_HASH_SALT").unwrap_or_else(|_| { DEFAULT_HASH.to_string() })
    }


    #[cfg(test)]
    mod tests {
        use std::{env, fs, panic};
        use std::fs::File;
        use std::io::Write;
        use std::path::Path;
        use crate::env::env::{DEFAULT_HASH, get_salt_for_pw_hash};

        fn show_dir(relative_path: &str) {
            // Get current directory
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let path = current_dir.join(relative_path);

            println!("absolute path for [{}] is [{}]", relative_path, path.display())
        }

        fn exists_dotenv_file() -> bool {
            let dotenv_file = Path::new(".env");
            dotenv_file.exists()
        }

        fn move_temporary_dotenv_file() {
            fs::rename(".env", ".env_backup_for_unittest").expect("Not found .env")
        }

        fn recover_dotenv_file() {
            fs::rename(".env_backup_for_unittest", ".env").expect("Not found .env")
        }

        fn remove_dotenv_file() {
            if exists_dotenv_file() {
                fs::remove_file(".env").expect("Failed to remove dotenv file.");
            }
        }

        fn write_to_file(path: &str, text: &str) -> std::io::Result<()> {
            // ファイルを作成
            let mut file = File::create(path)?;

            // 文字列を書き込む
            file.write_all(text.as_bytes())?;

            // 文字列を書き込んだら、`Ok(())` を返して終了します。
            Ok(())
        }

        #[test]
        fn can_get_from_dotenv_file() {
            show_dir(".env");
            let is_backup_dotenv = exists_dotenv_file();
            if is_backup_dotenv {
                move_temporary_dotenv_file();
            }
            let result = panic::catch_unwind(|| {
                assert_eq!(DEFAULT_HASH.to_string(), get_salt_for_pw_hash(), ".envファイルがない場合はDEFAULT_HASHが返却される");

                let write_result = write_to_file(".env", "PW_HASH_SALT=dot_env_salt_strings");
                if write_result.is_err() {
                    panic!("Failed to create .env file for testing.")
                }

                assert_eq!("dot_env_salt_strings".to_string(), get_salt_for_pw_hash(), ".envファイルがある場合はPW_HASH_SALTの値が返却される");
            });

            if exists_dotenv_file() {
                remove_dotenv_file();
            }
            if is_backup_dotenv {
                recover_dotenv_file();
            }
            match result {
                Ok(_) => (),
                Err(err) => {
                    println!("{:?}", err);
                    panic!()
                }
            }
        }
    }
}