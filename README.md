# ConHost Crash Repro
Make sure you have a newer version of Rust installed, like 1.31.

Run in a fresh terminal:

```bash
cargo run
```

Your terminal should close. The crash is not 100% reproducable, but I found that adding extra logging around my Win32 calls (writing to a file) caused it to occur more often.