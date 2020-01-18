### A naive Rust HTTP server


This is the outcome of following the instructions from [Chapter 20](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) of the [Rust Book](https://doc.rust-lang.org/book/).

[`lib.rs`](src/lib.rs) is where you would find the ThreadPool object that handles the incoming HTTP requests.

[`parsers.rs`](src/parsers.rs) contains functions that help with parsing the HTTP requests.

[`main.rs`](src/main.rs) uses the tools from [`lib.rs`](src/lib.rs) to handle incoming requests. It only handles `/` and `/sleep*` endpoints, hence the naive qualification.

The [`runner.py`](runner.py) script is written to test how well the server handles requests. You can invoke it with the base url of the http server, along with a number of GET requests to send.
The script uses python threads to try and send concurrent requests to the http server. This is really not so bad with the `GIL` and all, since we're dealing mostly with IO ops here.

