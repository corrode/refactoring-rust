//! Verify intentional starter failures and compile callers against the actual solutions.

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

struct TempDir(PathBuf);

impl TempDir {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        for _ in 0..100 {
            let path = env::temp_dir().join(format!(
                "refactoring-domain-modeling-{}-{timestamp}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed),
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("create temporary directory {}: {error}", path.display()),
            }
        }
        panic!("could not allocate a unique temporary directory");
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.0) {
            if error.kind() != io::ErrorKind::NotFound {
                // Do not cause a second panic while unwinding a failed assertion.
                eprintln!("remove temporary directory {}: {error}", self.0.display());
            }
        }
    }
}

fn diagnostics(output: &Output) -> String {
    format!(
        "status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    )
}

fn check(solution: &str, body: &str, expected_error: Option<&str>) {
    let temp = TempDir::new();
    let module = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("solutions")
        .join(solution);
    let source = temp.0.join("fixture.rs");
    let executable = temp.0.join(format!("fixture{}", env::consts::EXE_SUFFIX));
    // Debug formatting quotes and escapes the absolute path as a Rust string literal.
    fs::write(
        &source,
        format!(
            "#[path = {module:?}]\nmod solution;\nuse solution::*;\nfn main() {{\n{body}\n}}\n"
        ),
    )
    .unwrap();

    let output = Command::new(env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
        .args(["--edition=2024", "--color=never", "--error-format=human"])
        .arg(&source)
        .arg("-o")
        .arg(&executable)
        .output()
        .expect("run rustc for domain-modeling fixture");
    match expected_error {
        Some(code) => {
            assert!(
                !output.status.success(),
                "fixture unexpectedly compiled: {body}"
            );
            let stderr = String::from_utf8_lossy(&output.stderr);
            let errors: Vec<_> = stderr
                .lines()
                .filter_map(|line| {
                    line.strip_prefix("error[")?
                        .split_once(']')
                        .map(|(code, _)| code)
                })
                .collect();
            assert_eq!(errors, [code], "{body}\n{}", diagnostics(&output));
        }
        None => {
            assert!(output.status.success(), "{body}\n{}", diagnostics(&output));
            let run = Command::new(&executable)
                .output()
                .expect("run positive fixture");
            assert!(run.status.success(), "{body}\n{}", diagnostics(&run));
        }
    }
}

fn check_exercise_test(source: &str, test: &str, expected_failure: Option<&str>) {
    let temp = TempDir::new();
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(source);
    let executable = temp.0.join(format!("exercise{}", env::consts::EXE_SUFFIX));
    let output = Command::new(env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
        .args([
            "--edition=2024",
            "--color=never",
            "--crate-name=exercise",
            "--test",
        ])
        .arg(&source)
        .arg("-o")
        .arg(&executable)
        .output()
        .expect("compile exercise test harness");
    assert!(output.status.success(), "{}", diagnostics(&output));

    let run = Command::new(&executable)
        .args(["--exact", test, "--color=never", "--format=pretty"])
        .output()
        .expect("run exercise test");
    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(stdout.contains("running 1 test\n"), "{}", diagnostics(&run));
    if let Some(message) = expected_failure {
        assert_eq!(run.status.code(), Some(101), "{}", diagnostics(&run));
        assert!(
            stdout.contains(&format!("test {test} ... FAILED")) && stdout.contains(message),
            "expected failure {message:?} for {test}\n{}",
            diagnostics(&run)
        );
    } else {
        assert!(run.status.success(), "{}", diagnostics(&run));
        assert!(
            stdout.contains(&format!("test {test} ... ok")),
            "{}",
            diagnostics(&run)
        );
    }
}

#[test]
fn typed_ids_starter_fails_desired_behavior_and_solution_passes() {
    check_exercise_test(
        "examples/31_typed_ids.rs",
        "tests::variable_and_declaration_ids_are_distinct",
        Some("variable and declaration IDs must be distinct types"),
    );
    check_exercise_test(
        "solutions/31_typed_ids.rs",
        "tests::ids_have_distinct_types",
        None,
    );
}

#[test]
fn secret_token_starter_fails_desired_behavior_and_solution_passes() {
    check_exercise_test(
        "examples/33_secret_token.rs",
        "tests::debug_does_not_leak_token",
        Some("Debug output must not contain the token"),
    );
    check_exercise_test(
        "solutions/33_secret_token.rs",
        "tests::credentials_keep_username_visible_and_token_explicit",
        None,
    );
}

#[test]
fn typed_ids_allow_matching_categories() {
    check(
        "31_typed_ids.rs",
        r#"
        let engine = Engine::new(vec!["count".into()], vec!["print".into()]);
        assert_eq!(engine.variable(VarId(0)), Some("count"));
        assert_eq!(engine.declaration(DeclId(0)), Some("print"));
        assert_eq!(engine.variable(VarId(usize::MAX)), None);
        assert_eq!(engine.declaration(DeclId(usize::MAX)), None);
        "#,
        None,
    );
}

#[test]
fn variable_lookup_rejects_declaration_id() {
    check(
        "31_typed_ids.rs",
        "let engine = Engine::new(vec![], vec![]); engine.variable(DeclId(0));",
        Some("E0308"),
    );
}

#[test]
fn declaration_lookup_rejects_variable_id() {
    check(
        "31_typed_ids.rs",
        "let engine = Engine::new(vec![], vec![]); engine.declaration(VarId(0));",
        Some("E0308"),
    );
}

#[test]
fn validation_feedback_renders_typed_categories() {
    check(
        "32_validation_feedback.rs",
        r#"
        let messages: Vec<&'static str> = {
            let report = ValidationReport {
                feedback: vec![Feedback::MissingDigit, Feedback::TooShort],
            };
            report.suggestions()
        };
        assert_eq!(messages, ["Include a digit.", "Use at least eight characters."]);
        "#,
        None,
    );
}

#[test]
fn validation_feedback_rejects_string_categories() {
    check(
        "32_validation_feedback.rs",
        r#"let _ = ValidationReport { feedback: vec![String::from("missing digit")] };"#,
        Some("E0308"),
    );
}

#[test]
fn validation_feedback_requires_all_categories_in_a_match() {
    check(
        "32_validation_feedback.rs",
        r#"
        let feedback = Feedback::TooShort;
        let _ = match feedback {
            Feedback::TooShort => "Use at least eight characters.",
            Feedback::MissingUppercase => "Include an uppercase letter.",
        };
        "#,
        Some("E0004"),
    );
}

#[test]
fn token_allows_explicit_exposure_and_redacted_debug() {
    check(
        "33_secret_token.rs",
        r#"
        let credentials = Credentials::new("alice".into(), Token::new("secret".into()));
        assert_eq!(credentials.username, "alice");
        assert_eq!(credentials.token.expose(), "secret");
        assert_eq!(format!("{:?}", credentials.token), "[REDACTED]");
        assert_eq!(format!("{:#?}", credentials.token), "[REDACTED]");
        assert_eq!(format!("{credentials:?}"),
            "Credentials { username: \"alice\", token: [REDACTED] }");
        "#,
        None,
    );
}

#[test]
fn token_storage_is_private() {
    check(
        "33_secret_token.rs",
        "let token = Token::new(String::new()); let _ = token.0;",
        Some("E0616"),
    );
}

#[test]
fn token_does_not_implement_display() {
    check(
        "33_secret_token.rs",
        r#"let token = Token::new(String::new()); let _ = format!("{token}");"#,
        Some("E0277"),
    );
}

#[test]
fn configured_builder_builds() {
    check(
        "34_config_typestate.rs",
        r#"
        let ready: Ready = NeedsRoots.with_roots(RootStore::new("roots".into()));
        let config: ClientConfig = ready.build();
        assert_eq!(config.roots, RootStore::new("roots".into()));
        "#,
        None,
    );
}

#[test]
fn builder_cannot_build_before_receiving_roots() {
    check(
        "34_config_typestate.rs",
        "NeedsRoots.build();",
        Some("E0599"),
    );
}

#[test]
fn ready_cannot_be_reused_after_build() {
    check(
        "34_config_typestate.rs",
        r#"
        let ready = NeedsRoots.with_roots(RootStore::new(String::new()));
        let _ = ready.build();
        let _ = ready.build();
        "#,
        Some("E0382"),
    );
}

#[test]
fn ready_cannot_be_constructed_directly() {
    check(
        "34_config_typestate.rs",
        "let _ = Ready { roots: RootStore::new(String::new()) };",
        Some("E0451"),
    );
}
