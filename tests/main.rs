use assert_cmd::Command;

#[test]
fn quote_tests() {
    quote_test("", "");
    quote_test(" ", ">\n");
    quote_test("  ", ">\n");
    quote_test("\n", ">\n");
    quote_test("\n\n", ">\n>\n");
    quote_test("a", "> a\n");
    quote_test("a\n", "> a\n");
    quote_test("a\nb", "> a\n> b\n");
    quote_test("a\nb\n", "> a\n> b\n");
    quote_test("a\n\nb", "> a\n>\n> b\n");
    quote_test("xyz\nabc", "> xyz\n> abc\n");
    quote_test("> a", ">> a\n");
    quote_test("> a\n>b", ">> a\n>>b\n");
    quote_test("a\n> b", "> a\n>> b\n");
}

fn quote_test(input: &'static str, expected: &'static str) {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .code(0)
        .stdout(expected);
}

#[test]
fn quote_file_tests() {
    quote_file_test(&["tests/data/empty.txt"], "");
    quote_file_test(&["tests/data/1line.txt"], "> abc\n");
    quote_file_test(&["tests/data/2lines.txt"], "> xyz\n> hijkl\n");
    quote_file_test(
        &["tests/data/1line.txt", "tests/data/2lines.txt"],
        "> abc\n> xyz\n> hijkl\n",
    );
}

fn quote_file_test(input_files: &[&'static str], expected: &'static str) {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(input_files)
        .assert()
        .success()
        .code(0)
        .stdout(expected);
}
