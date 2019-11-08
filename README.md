### A naive Rust HTTP server


This is the outcome of following the instructions from [Chapter 20](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) of the [Rust Book](https://doc.rust-lang.org/book/).

[`lib.rs`](src/lib.rs) is where you would find the ThreadPool object that handles the incoming HTTP requests.

[`parsers.rs`](src/parsers.rs) contains functions that help with parsing the HTTP requests.

[`main.rs`](src/main.rs) uses the tools from [`lib.rs`](src/lib.rs) to handle incoming requests. It only handles `/` and `/sleep` endpoints, hence the naive qualification.
