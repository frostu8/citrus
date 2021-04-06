use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::error::Error;

/// General GL error.
#[derive(Debug)]
pub enum GlError {
    ShaderCompile(ShaderCompileError),
    ProgramLink(ProgramLinkError),
    UniformNotFound(UniformNotFoundError),
}

impl Display for GlError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::ShaderCompile(e) => write!(f, "{}", e),
            Self::ProgramLink(e) => write!(f, "{}", e),
            Self::UniformNotFound(e) => write!(f, "{}", e),
        }
    }
}

impl Error for GlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ShaderCompile(e) => Some(e),
            Self::ProgramLink(e) => Some(e),
            Self::UniformNotFound(e) => Some(e),
        }
    }
}

impl From<ShaderCompileError> for GlError {
    fn from(e: ShaderCompileError) -> GlError {
        GlError::ShaderCompile(e)
    }
}

impl From<ProgramLinkError> for GlError {
    fn from(e: ProgramLinkError) -> GlError {
        GlError::ProgramLink(e)
    }
}

impl From<UniformNotFoundError> for GlError {
    fn from(e: UniformNotFoundError) -> GlError {
        GlError::UniformNotFound(e)
    }
}

/// Error trying to get uniform.
pub struct UniformNotFoundError(String);

impl UniformNotFoundError {
    pub fn new(log: impl Into<String>) -> UniformNotFoundError {
        UniformNotFoundError(log.into())
    }

    pub fn errors(&self) -> impl Iterator<Item = &str> {
        self.0.lines()
    }
}

impl Debug for UniformNotFoundError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("ShaderCompileError(\n")?;
        for error in self.errors() { write!(f, "\t{}\n", error)?; }
        f.write_str(")")
    }
}

impl Display for UniformNotFoundError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "shader compile error with {} errors.", self.errors().count())
    }
}

impl Error for UniformNotFoundError { }

/// Error during shader compilation.
pub struct ShaderCompileError(String);

impl ShaderCompileError {
    pub fn new(log: impl Into<String>) -> ShaderCompileError {
        ShaderCompileError(log.into())
    }

    pub fn errors(&self) -> impl Iterator<Item = &str> {
        self.0.lines()
    }
}

impl Debug for ShaderCompileError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("ShaderCompileError(\n")?;
        for error in self.errors() { write!(f, "\t{}\n", error)?; }
        f.write_str(")")
    }
}

impl Display for ShaderCompileError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "shader compile error with {} errors.", self.errors().count())
    }
}

impl Error for ShaderCompileError { }

/// Error during program linking.
pub struct ProgramLinkError(String);

impl ProgramLinkError {
    pub fn new(log: impl Into<String>) -> ProgramLinkError {
        ProgramLinkError(log.into())
    }

    pub fn errors(&self) -> impl Iterator<Item = &str> {
        self.0.lines()
    }
}

impl Debug for ProgramLinkError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("ProgramLinkError(\n")?;
        for error in self.errors() { write!(f, "\t{}\n", error)?; }
        f.write_str(")")
    }
}

impl Display for ProgramLinkError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "program link error with {} errors.", self.errors().count())
    }
}

impl Error for ProgramLinkError { }
