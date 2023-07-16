// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::io::Read;

use bytes::Buf;

use crate::{WoffResult, constants::WOFF2_SIGNATURE, WoffError};

pub struct Woff2Header {
    /// 0x774F4632 'wOF2'
    pub signature: u32,

    /// The "sfnt version" of the input font.
    pub flavor: u32,

    /// Total size of the WOFF file.
    pub length: u32,

    /// Number of entries in directory of font tables.
    pub num_tables: u16,

    /// Reserved; set to 0.
    pub _reserved: u16,

    /// Total size needed for the uncompressed font data, including the sfnt
    /// header, directory, and font tables (including padding).
    ///
    /// # Note
    /// The "totalSfntSize" value in the WOFF2 Header is intended to be used
    /// for reference purposes only. It may represent the size of the
    /// uncompressed input font file, but if the transformed 'glyf' and
    /// 'loca' tables are present, the uncompressed size of the reconstructed
    /// tables and the total decompressed font size may differ substantially
    /// from the original total size specified in the WOFF2 Header.
    #[deprecated = "don't use this value"]
    pub total_sfnt_size: u32,

    /// Total length of the compressed data block.
    pub total_compressed_size: u32,

    /// Major version of the WOFF file.
    pub major_version: u16,

    /// Minor version of the WOFF file.
    pub minor_version: u16,

    /// Offset to metadata block, from beginning of WOFF file.
    pub meta_offset: u32,

    /// Length of compressed metadata block.
    pub meta_length: u32,

    /// Uncompressed size of metadata block.
    pub meta_orig_length: u32,

    //// Offset to private data block, from beginning of WOFF file.
    pub priv_offset: u32,

    /// Length of private data block.
    pub priv_length: u32,
}

impl Woff2Header {
    pub fn parse<R>(mut reader: R) -> WoffResult<Self>
            where R: Read {
        let mut buf = [0u8; 48];
        reader.read_exact(&mut buf)?;

        let mut buf = std::io::Cursor::new(buf);

        let signature = buf.get_u32();

        // The signature field in the WOFF2 header MUST contain the value of
        // 0x774F4632 ('wOF2'), which distinguishes it from WOFF 1.0 files.
        // If the field does not contain this value, user agents MUST reject
        // the file as invalid.
        if signature != WOFF2_SIGNATURE {
            return Err(WoffError::InvalidSignature {
                expected: WOFF2_SIGNATURE,
                found: signature,
            });
        }

        let flavor = buf.get_u32();
        let length = buf.get_u32();
        let num_tables = buf.get_u16();
        let _reserved = buf.get_u16();
        let total_sfnt_size = buf.get_u32();
        let total_compressed_size = buf.get_u32();
        let major_version = buf.get_u16();
        let minor_version = buf.get_u16();
        let meta_offset = buf.get_u32();
        let meta_length = buf.get_u32();
        let meta_orig_length = buf.get_u32();
        let priv_offset = buf.get_u32();
        let priv_length = buf.get_u32();

        #[allow(deprecated)]
        Ok(Self {
            signature,
            flavor,
            length,
            num_tables,
            _reserved,

            total_sfnt_size,

            total_compressed_size,
            major_version,
            minor_version,
            meta_offset,
            meta_length,
            meta_orig_length,
            priv_offset,
            priv_length,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_size() {
        assert_eq!(std::mem::size_of::<Woff2Header>(), 48);
    }


}
