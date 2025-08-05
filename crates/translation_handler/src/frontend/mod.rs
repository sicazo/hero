pub mod getter;
pub mod remover;
pub mod setter;
pub mod updater;



pub enum PathType {
    MessageTsFile,
    TranslationDirectory,
    TranslationExportFile,
    EnGbFile,
}
impl PathType {
    fn create_path(self, path: String) -> String {
        match self {
            PathType::MessageTsFile => match std::env::consts::OS {
                "macos" => {
                    format!("{}/messages.ts", path)
                }
                "windows" => {
                    format!("{}\\messages.ts", path)
                }
                "linux" => {
                    format!("{}/messages.ts", path)
                }
                _ => {
                    format!("{}/messages.ts", path)
                }
            },
            PathType::TranslationDirectory => match std::env::consts::OS {
                "macos" => {
                    format!("{}/locales", path)
                }
                "windows" => {
                    format!("{}\\locales", path)
                }
                "linux" => {
                    format!("{}/locales", path)
                }
                _ => {
                    format!("{}/locales", path)
                }
            },
            PathType::TranslationExportFile => match std::env::consts::OS {
                "macos" => {
                    format!("{}/locales/locales.ts", path)
                }
                "windows" => {
                    format!("{}\\locales\\locales.ts", path)
                }
                "linux" => {
                    format!("{}/locales/locales.ts", path)
                }
                _ => {
                    format!("{}/locales/locales.ts", path)
                }
            },
            PathType::EnGbFile => match std::env::consts::OS {
                "macos" => {
                    format!("{}/locales/en-GB.json", path)
                }
                "windows" => {
                    format!("{}\\locales\\en-GB.json", path)
                }
                "linux" => {
                    format!("{}/locales/en-GB.json", path)
                }
                _ => {
                    format!("{}/locales/en-GB.json", path)
                }
            },
        }
    }
}
