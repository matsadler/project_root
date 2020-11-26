use std::{
    error::Error,
    fmt,
    path::{Path, PathBuf},
};

use log::{trace, warn};

/// An error that can be returned from [`find_root`].
#[derive(Debug)]
pub enum FindError<'a> {
    Io(std::io::Error),
    NotFound(&'a str),
}

impl fmt::Display for FindError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FindError::Io(e) => e.fmt(f),
            FindError::NotFound(s) => {
                write!(f, r#"Project root not found, could not find "{}""#, s)
            }
        }
    }
}

impl Error for FindError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FindError::Io(e) => Some(e),
            FindError::NotFound(_) => None,
        }
    }
}

impl From<std::io::Error> for FindError<'_> {
    fn from(error: std::io::Error) -> Self {
        FindError::Io(error)
    }
}

/// Looks for the marker file and returns the first directory containing it.
/// If `top_down` is false, starts in the current working directory, walking up
/// through the parent directories looking for the marker. If `top_down` is
/// true the order is reversed.
///
/// # Errors
///
/// Returns an `Err` if current working directory value is invalid, or if the
/// marker file is not found.
pub fn find_root(marker: &str, top_down: bool) -> Result<PathBuf, FindError> {
    let current_dir = std::env::current_dir()?;
    let path = if top_down {
        find_root_in(
            marker,
            current_dir
                .ancestors()
                .collect::<Vec<_>>()
                .into_iter()
                .rev(),
        )
    } else {
        find_root_in(marker, current_dir.ancestors())
    };
    path.ok_or_else(|| FindError::NotFound(marker))
}

/// Returns the first directory in `iter` that contains a file named as the
/// `marker`.
fn find_root_in<'a, I>(marker: &str, iter: I) -> Option<PathBuf>
where
    I: Iterator<Item = &'a Path>,
{
    iter.inspect(|p| trace!("testing {:?}", p))
        .find(|p| p.join(marker).exists())
        .map(|p| {
            trace!("matched {:?}", p);
            p.to_owned()
        })
}
/// An error that can be returned from [`prepare_output`].
#[derive(Debug)]
pub enum OutputError<'a> {
    BadPath(&'a Path),
    Utf8(&'a Path),
}

impl fmt::Display for OutputError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputError::BadPath(p) => write!(f, "Could not get basename for path {:?}", p),
            OutputError::Utf8(p) => write!(f, "Path {:?} contains non-UTF8 characters", p),
        }
    }
}

impl Error for OutputError<'_> {}

/// Returns a printable `&str` for `path`.
///
/// If `basename` is `true` only the basename will be returned, if `false` the
/// whole path is returned.
///
/// # Errors
///
/// Returns an `Err` if the path contains non-UTF8 characters, or `basename`
/// was true and `path` can not be converted to a basename.
pub fn prepare_output(path: &Path, basename: bool) -> Result<&str, OutputError> {
    if basename {
        let name = path.file_name().ok_or_else(|| OutputError::BadPath(path))?;
        name.to_str().ok_or_else(|| OutputError::Utf8(path))
    } else {
        path.to_str().ok_or_else(|| OutputError::Utf8(path))
    }
}

/// The `Exit` trait adds an [`Exit::exit`] method to [`Result`], similar to
/// [`Result::unwrap`].
pub trait Exit {
    /// The type of the value returned from `exit`.
    type T;

    fn exit(self, code: i32) -> Self::T;
}

impl<T, E> Exit for Result<T, E>
where
    E: Error,
{
    type T = T;

    /// Returns the contained value if result is `Ok`, otherwise exits the
    /// process with the given code.
    ///
    /// Prints an error message provided by `Err`'s value if the log level is
    /// 'warn' or above.
    fn exit(self, code: i32) -> Self::T {
        match self {
            Ok(v) => v,
            Err(e) => {
                warn!("{}", e);
                std::process::exit(code);
            }
        }
    }
}
