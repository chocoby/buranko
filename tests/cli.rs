use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Fixture(std::path::PathBuf);
impl Fixture {
    fn new(repository: bool) -> Self {
        let path = std::env::temp_dir().join(format!(
            "buranko-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&path).unwrap();
        if repository {
            let repo = git2::Repository::init(&path).unwrap();
            let tree_id = repo.index().unwrap().write_tree().unwrap();
            let tree = repo.find_tree(tree_id).unwrap();
            let sig = git2::Signature::now("Test", "test@example.com").unwrap();
            repo.commit(
                Some("refs/heads/feature/123-test"),
                &sig,
                &sig,
                "initial",
                &tree,
                &[],
            )
            .unwrap();
            repo.set_head("refs/heads/feature/123-test").unwrap();
        }
        Self(path)
    }
    fn run(&self, args: &[&str], input: &[u8]) -> std::process::Output {
        let mut child = Command::new(env!("CARGO_BIN_EXE_buranko"))
            .current_dir(&self.0)
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env("GIT_CONFIG_GLOBAL", self.0.join("no-global-config"))
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(input).unwrap();
        child.wait_with_output().unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn empty_stdin_uses_current_branch() {
    let output = Fixture::new(true).run(&[], b"");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"123");
}

#[test]
fn missing_repository_fails() {
    let output = Fixture::new(false).run(&[], b"");
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}

#[test]
fn invalid_stdin_fails() {
    let output = Fixture::new(false).run(&[], &[0xff]);
    assert!(!output.status.success());
}

#[test]
fn missing_template_fails_without_stdout() {
    let output = Fixture::new(true).run(&["--template"], b"feature/123-test");
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}

#[test]
fn template_repository_error_fails() {
    let output = Fixture::new(false).run(&["--template"], b"feature/123-test");
    assert!(!output.status.success());
}

#[test]
fn template_preserves_placeholder_text_in_values() {
    let fixture = Fixture::new(true);
    git2::Repository::open(&fixture.0)
        .unwrap()
        .config()
        .unwrap()
        .set_str("buranko.template", "{FullName}|{Description}|{ID}")
        .unwrap();
    let output = fixture.run(&["--template"], b"feature/123-{ID}");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"feature/123-{ID}|{ID}|123");
}

#[test]
fn output_fields_use_real_cli() {
    let fixture = Fixture::new(false);
    for (field, expected) in [
        ("FullName", "feature/123-test"),
        ("Action", "feature"),
        ("ID", "123"),
        ("LinkID", "#123"),
        ("Description", "test"),
    ] {
        let output = fixture.run(&["--output", field], b"feature/123-test\n");
        assert!(output.status.success());
        assert_eq!(output.stdout, expected.as_bytes());
    }
}
