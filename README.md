# Minigrep

As a Rust beginner, the book walks me through an I/O project: Building a Command Line Program

Reference: <https://doc.rust-lang.org/book/ch12-00-an-io-project.html>

## Running

```bash
# --- bash ----------
# cargo run <query> <file>
$ IGNORE_CASE=1 cargo run bOdy poem.txt
# $ IGNORE_CASE=1 cargo run bOdy poem.txt > output.txt # writes output into a .txt file
```

`IGNORE_CASE` is an environment variable
