pub enum XfbinError {
    NullFile,
    CutShort,
    MismatchedFileSignature { given_file_signature: u32 },
    MismatchedVersion { given_version: u32 },
}

impl XfbinError {
    pub fn to_int(&self) -> u32 {
        match self {
            XfbinError::NullFile                    => 000,
            XfbinError::CutShort                    => 001,
            XfbinError::MismatchedFileSignature{..} => 100,
            XfbinError::MismatchedVersion{..}       => 101,
        }
    }
}
