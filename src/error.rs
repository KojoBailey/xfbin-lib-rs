pub struct MismatchedFileSignatureData {
    given_file_signature: u32,
}

pub struct MismatchedVersionData {
    given_version_data: u32,
}

pub enum XfbinError {
    NullFile,
    CutShort,
    MismatchedFileSignature(MismatchedFileSignatureData),
    MismatchedVersion(MismatchedVersionData),
}

impl XfbinError {
    pub fn to_int(&self) -> u32 {
        match self {
            XfbinError::NullFile                   => 000,
            XfbinError::CutShort                   => 001,
            XfbinError::MismatchedFileSignature(_) => 100,
            XfbinError::MismatchedVersion(_)       => 101,
        }
    }
}
