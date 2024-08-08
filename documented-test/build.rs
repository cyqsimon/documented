fn main() {
    extract_readme_example();
}

fn extract_readme_example() {
    use std::{env, fmt::Write, fs, path::Path};

    println!("cargo:rerun-if-changed=../README.md");

    // load README
    let readme = fs::read_to_string("../README.md").unwrap();
    let lines = readme.lines().collect::<Vec<_>>();

    // find code fragment indices
    let mut lines_it = lines.iter().enumerate();
    let mut examples_ranges = vec![];
    loop {
        let start =
            lines_it.find_map(|(idx, &line)| matches!(line, "```rust" | "```rs").then_some(idx));
        let Some(start) = start else { break };

        let end = lines_it.find_map(|(idx, &line)| matches!(line, "```").then_some(idx));
        let Some(end) = end else {
            panic!(
                "Cannot find a matching code block terminator (```) after line {}.",
                start + 1 // line numbers are conventionally 1-indexed
            );
        };

        examples_ranges.push(start + 1..end);
    }

    // collect fragments
    let examples =
        examples_ranges
            .into_iter()
            .enumerate()
            .fold(String::new(), |mut out, (idx, range)| {
                let code = lines[range].join("\n");
                write!(
                    &mut out,
                    "\
#[test]
fn example_fragment{idx}() {{
    {code}
}}
"
                )
                .unwrap();
                out
            });

    // write
    let out_path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("readme_examples.rs");
    fs::write(out_path, examples).unwrap();
}
