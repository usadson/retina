// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::fmt::Display;

const BYTES_PER_KIB: usize = 1024;
const BYTES_PER_MIB: usize = BYTES_PER_KIB * 1024;
const BYTES_PER_GIB: usize = BYTES_PER_MIB * 1024;

pub struct ByteUnitFormatWrapper(pub usize);

pub trait ByteUnitFormat {
    fn format_bytes(&self) -> ByteUnitFormatWrapper;
}

impl ByteUnitFormat for usize {
    fn format_bytes(&self) -> ByteUnitFormatWrapper {
        ByteUnitFormatWrapper(*self)
    }
}

impl Display for ByteUnitFormatWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_bytes = self.0;

        let gibs = total_bytes / BYTES_PER_GIB;
        let mibs = total_bytes / BYTES_PER_MIB;
        let kibs = total_bytes / BYTES_PER_KIB;

        if gibs > 0 {
            f.write_fmt(format_args!("{gibs} GiB"))
        } else if mibs > 0 {
            f.write_fmt(format_args!("{mibs} MiB"))
        } else if kibs > 0 {
            f.write_fmt(format_args!("{kibs} KiB"))
        } else {
            f.write_fmt(format_args!("{total_bytes} bytes"))
        }
    }
}
