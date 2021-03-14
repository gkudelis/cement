use std::env;
use std::fs;
use std::path::Path;
use tempfile;

use cement::prototypes::file_select;
use cement::prototypes::artifact;

fn in_tempdir<T>(f: impl FnOnce(&Path) -> T) -> T {
    let last_cwd = env::current_dir().unwrap();
    let temp_dir = tempfile::tempdir().unwrap();

    env::set_current_dir(&temp_dir).unwrap();
    let result = f(temp_dir.as_ref());
    env::set_current_dir(&last_cwd).unwrap();

    result
}

#[test]
fn artifact_writes_a_file() {
    in_tempdir(|dirname| {
        // create file for matching
        let mut dir = dirname.to_path_buf();
        dir.push("content/posts/relative/path");
        let mut post = dir.clone();
        post.push("post.md");
        fs::create_dir_all(&dir).unwrap();
        fs::File::create(&post).unwrap();

        // match file and create artifact
        let posts = file_select::files_matching("content/posts/**/*.md");
        let artifacts = posts.map(|post_path| {
            let mut artifact_path = post_path;
            artifact_path.set_extension("html");
            artifact::Artifact { location: artifact_path }
        }).collect::<Vec<_>>();

        // should only generate a single artifact
        assert_eq!(artifacts.len(), 1);

        // write artifact
        for artifact in &artifacts { artifact.write("some stuff here"); }

        // check artifact contents
        let mut artifact_path = dirname.to_path_buf();
        artifact_path.push("content/posts/relative/path/post.html");
        let content = fs::read_to_string(artifact_path).unwrap();
        assert_eq!(content, "some stuff here");
    })
}
