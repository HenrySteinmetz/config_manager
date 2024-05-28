#[derive(Debug)]
pub enum ConfigCliError {
    // FS Errors
    FsReadError(std::io::Error),
    FsWriteError(std::io::Error),
    FileCreationError(std::io::Error),
    CopyError(std::io::Error),
    DeleteError(std::io::Error),
    SymlinkError(std::io::Error),
    RenameError(std::io::Error),
    // Fileconversion errors
    DeserializeError(toml::de::Error),
    SerializeError(toml::ser::Error),
    StringConversionError(std::str::Utf8Error),
    // Cli Error
    ShellInitError(std::io::Error),
    GitCommandError(String),
    // Internal error
    InvalidThemeName(String),
    InvalidConfigName(String),
    InvalidConfigLocation(String),
    InvalidDependencyName(String),
    ConfigLocationUsed(String),
    NoPackageWithName(String),
    DependencyAlreadyExists(String),
    UnableToFindHomeDir,
    NoThemeSelecected,
}

impl std::fmt::Display for ConfigCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ConfigCliError::*;
        match self {
            FsReadError(err) => write!(f, "Read Error: \n{}", err),
            FsWriteError(err) => write!(f, "Write Error: \n{}", err),
            FileCreationError(err) => write!(f, "Creation Error: \n{}", err),
            CopyError(err) => write!(f, "Copy Error: \n{}", err),
            DeleteError(err) => write!(f, "Delete Error: \n{}", err),
            SymlinkError(err) => write!(f, "Symlink Error: \n{}", err),
            RenameError(err) => write!(f, "Rename Error: \n{}", err),
            SerializeError(err) => write!(f, "TOML Parse Error while serializing: \n{}", err),
            DeserializeError(err) => write!(f, "TOML Parse Error while deserializing: \n{}", err),
            StringConversionError(err) => write!(f, "Invalid String: \n{}", err),
            ShellInitError(err) => write!(f, "Failed to initialize shell: \n{}", err),
            GitCommandError(err) => write!(f, "Git Command Error: \n{}", err),
            InvalidThemeName(err) => write!(f, "Invalid Theme Name:  \n{}", err),
            InvalidConfigName(err) => write!(f, "Invalid Config Name: \n{}", err),
            InvalidDependencyName(err) => write!(f, "Invalid Dependency Name: \n{}", err),
            InvalidConfigLocation(err) => write!(f, "Invalid Config Location: \n{}", err),
            ConfigLocationUsed(err) => write!(f, "Config Location {} already used", err),
            NoPackageWithName(err) => write!(f, "No Package with name: \n{}", err),
            DependencyAlreadyExists(err) => write!(f, "Dependency already exists: \n{}", err),
            UnableToFindHomeDir => write!(f, "Unable to find home directory"),
            NoThemeSelecected => write!(f, "No theme selecected"),
        }
    }
}

impl std::error::Error for ConfigCliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ConfigCliError::*;
        match self {
            FsReadError(x) => Some(x),
            FsWriteError(x) => Some(x),
            FileCreationError(x) => Some(x),
            CopyError(x) => Some(x),
            DeleteError(x) => Some(x),
            SymlinkError(x) => Some(x),
            RenameError(x) => Some(x),
            SerializeError(x) => Some(x),
            DeserializeError(x) => Some(x),
            StringConversionError(x) => Some(x),
            ShellInitError(x) => Some(x),
            GitCommandError(_) => None,
            InvalidThemeName(_) => None,
            InvalidConfigName(_) => None,
            InvalidDependencyName(_) => None,
            InvalidConfigLocation(_) => None,
            ConfigLocationUsed(_) => None,
            NoPackageWithName(_) => None,
            DependencyAlreadyExists(_) => None,
            UnableToFindHomeDir => None,
            NoThemeSelecected => None,
        }
    }
}
