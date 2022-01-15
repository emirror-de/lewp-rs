// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

/// Does not handle ErrorKind::Interrupted, so only suitable for blocking IO.
pub(crate) struct BlockingIoOnlyStdFmtWriteToStdIoWriteAdaptor<
    W: ::std::io::Write,
>(pub W);

impl<W: ::std::io::Write> ::std::fmt::Write
    for BlockingIoOnlyStdFmtWriteToStdIoWriteAdaptor<W>
{
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> Result<(), ::std::fmt::Error> {
        self.write_all(s.as_bytes())
    }

    #[inline(always)]
    fn write_char(&mut self, c: char) -> Result<(), ::std::fmt::Error> {
        //let mut buffer: [u8; 4] = unsafe { ::std::mem::MaybeUninit() };
        let mut buffer: [u8; 4] = [0; 4];
        c.encode_utf8(&mut buffer);

        self.write_all(&buffer[0..c.len_utf8()])
    }

    #[inline(always)]
    fn write_fmt(
        &mut self,
        args: ::std::fmt::Arguments,
    ) -> Result<(), ::std::fmt::Error> {
        Self::handle_io_error(self.0.write_fmt(args))
    }
}

impl<W: ::std::io::Write> BlockingIoOnlyStdFmtWriteToStdIoWriteAdaptor<W> {
    #[inline(always)]
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), ::std::fmt::Error> {
        Self::handle_io_error(self.0.write_all(bytes))
    }

    #[inline(always)]
    fn handle_io_error<R>(
        result: Result<R, ::std::io::Error>,
    ) -> Result<(), ::std::fmt::Error> {
        if result.is_err() {
            Err(::std::fmt::Error)
        } else {
            Ok(())
        }
    }
}
