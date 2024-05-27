
pub enum ConfigCliError {
    FsReadError(std::io::Error),
    FsWriteError(std::io::Error),
    InvalidTheme(String),
    ParseError(toml::de::Error),
    InvalidConfigName(String),
    StringConversionError(std::str::Utf8Error),
    NoPackageWithName(String),
    DependencyAlreadyExists(String),
    UnableToFindHomeDir,
    NoThemeSelecected,
    CopyError(std::io::Error),
    DeleteError(std::io::Error),

}


impl std::error::Error for ConfigCliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ConfigCliError::*;
        match self {
            FsReadError(x) => Some(x),
            FsWriteError(x) => Some(x),
            InvalidTheme(x) => Some(x),
            ParseError(x) => Some(x),
            InvalidConfigName(x) => Some(x),
            StringConversionError(x) => Some(x),
            NoPackageWithName(x) => Some(x),
            DependencyAlreadyExists(x) => Some(x),
            UnableToFindHomeDir => None, 
            NoThemeSelecected => None, 
            CopyError(x) => Some(x),
            DeleteError(x) => Some(x),
        }
    }
}
