// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    cssparser::SourceLocation,
    std::path::{Path, PathBuf},
};

quick_error! {
    /// Represents all the things that can go wrong when loading and saving stylesheets.
    #[derive(Debug)]
    pub enum StylesheetError
    {
        /// An input-output error occurred, typically when loading or creating a file.
        Io(path: PathBuf, cause: ::std::io::Error)
        {
            cause(cause)
            description(cause.description())
            display("I/O error with {:?} was '{}'", path, cause)
            context(path: &'a Path, cause: ::std::io::Error) -> (path.to_path_buf(), cause)
        }

        /// An error occurred during a std::fmt::write (only happens when saving).
        Format(path: PathBuf, cause: ::std::fmt::Error)
        {
            cause(cause)
            description(cause.description())
            display("Format error with {:?} was '{}'", path, cause)
            context(path: &'a Path, cause: ::std::fmt::Error) -> (path.to_path_buf(), cause)
        }

        /// An error occurred during a parse.
        Parse(path: PathBuf, source_location: SourceLocation, reason: String)
        {
            description(&reason)
            display("Parse error with {:?} at '{:?}' was '{}'", path, source_location, &reason)
        }
    }
}
