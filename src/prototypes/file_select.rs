use crate::util::*;

use glob;
use std::path::PathBuf;

static PARSE_ERROR_MESSAGE: &str = "Could not parse glob pattern";
static GLOB_ERROR_MESSAGE: &str = "Glob iteration error";


pub fn files_matching<S>(pattern: S) -> impl Iterator<Item=PathBuf>
where S: AsRef<str>
{
    glob::glob(pattern.as_ref())
        .expect(PARSE_ERROR_MESSAGE)
        .filter_map(get_path)
}

fn get_path(glob_result: glob::GlobResult) -> Option<PathBuf> {
    glob_result.or_else(complain(GLOB_ERROR_MESSAGE)).ok()
}
