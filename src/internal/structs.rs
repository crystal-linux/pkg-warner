pub enum AppExitCode {
    Success = 1, // Success technically is still wrong since the user used the wrong package manager
    CalledDirectly = 2,
}
