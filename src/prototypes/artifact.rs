use std::fs;
use std::path::PathBuf;

static ARTIFACT_WRITE_ERROR: &str = "Could not write artifact to file";

pub struct Artifact {
    pub location: PathBuf,
}

impl Artifact {
    pub fn write<S>(&self, content: S)
    where S: AsRef<str>
    {
        let content_bytes = content.as_ref().as_bytes();
        fs::write(&self.location, content_bytes).expect(ARTIFACT_WRITE_ERROR);
    }
}
