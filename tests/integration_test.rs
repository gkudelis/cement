use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile;

use cement::prototypes::file_select;
use cement::prototypes::artifact;

fn in_tempdir(f: impl FnOnce(&Path) -> ()) {
    let last_cwd = env::current_dir().unwrap();
    let temp_dir = tempfile::tempdir().unwrap();
    env::set_current_dir(&temp_dir).unwrap();

    f(temp_dir.as_ref());

    env::set_current_dir(&last_cwd).unwrap();
}

#[test]
fn artifact_writes_a_file() {
    in_tempdir(|dirname| {
        // create file for matching
        let mut dir = PathBuf::from(dirname);
        dir.push("content/posts/relative/path");
        let mut file = PathBuf::from(&dir);
        file.push("post.md");
        fs::create_dir_all(&dir).unwrap();
        fs::File::create(&file).unwrap();

        // match file and create artifact
        let posts = file_select::files_matching("content/posts/**/*.md");
        let artifacts = posts.map(|post_path| {
            let mut artifact_path = PathBuf::from(post_path);
            artifact_path.set_extension("html");
            artifact::Artifact { location: artifact_path }
        }).collect::<Vec<_>>();

        // write artifact
        for artifact in &artifacts {
            artifact.write("some stuff here");
        }

        // check artifact exists
        for artifact in &artifacts {
            let content = fs::read_to_string(&artifact.location).unwrap();
            assert_eq!(content, "some stuff here");
        }
    });
}
