use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn parse_method(line: &str) -> HashMap<String, String> {
    ["method", "endpoint", "http"]
        .iter()
        .map(|c| String::from(*c))
        .zip(line.split(" ").map(|c| String::from(c)))
        .collect()
}

/// Handle incoming TCP/IP requests by parsing the stream and consuming the
/// header data.
pub fn handle_stream(stream: &mut TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let contents = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", contents);
    let request = parse_request(&contents);
    let (status, html) = if is_valid(&request) {
        if is_get(&request) || is_post(&request) {
            if is_sleep(&request) {
                thread::sleep(Duration::from_millis(5000));
                (
                    "HTTP/1.1 200 OK\r\n\r\n",
                    fs::read_to_string("html/sleep.html").unwrap(),
                )
            } else {
                (
                    "HTTP/1.1 200 OK\r\n\r\n",
                    fs::read_to_string("html/hello.html").unwrap(),
                )
            }
        } else {
            (
                "HTTP/1.1 405 Method Not Allowed\r\n\r\n",
                fs::read_to_string("html/405.html").unwrap(),
            )
        }
    } else {
        (
            "HTTP/1.1 404 Not Found\r\n\r\n",
            fs::read_to_string("html/404.html").unwrap(),
        )
    };
    let info = list_info(&request);
    let response = format!("{}{}", status, html.replace("{{}}", &info));
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Response: {}", status);
    println!("Body: {}", html);
}

fn list_info(req: &HashMap<String, String>) -> String {
    let v: Vec<String> = req
        .iter()
        .map(|tpl| format!("<li><strong>{}</strong>: {}</li>", tpl.0, tpl.1))
        .collect();
    v.join("\n")
}

fn is_get(req: &HashMap<String, String>) -> bool {
    let s = String::new();
    req.get("method").unwrap_or(&s).contains("GET")
}

fn is_post(req: &HashMap<String, String>) -> bool {
    let s = String::new();
    req.get("method").unwrap_or(&s).contains("POST")
}

fn is_valid(req: &HashMap<String, String>) -> bool {
    let s = String::new();
    let endpoint = req.get("endpoint").unwrap_or(&s);
    endpoint == "/" || endpoint.contains("/sleep")
}

fn is_sleep(req: &HashMap<String, String>) -> bool {
    let s = String::new();
    let endpoint = req.get("endpoint").unwrap_or(&s);
    endpoint.contains("/sleep") || endpoint.contains("/sleep/")
}

/// `parse_request` parses the string that is returned from consuming the TCP stream associated with the HTTP request
pub fn parse_request(req: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for (i, line) in req.lines().enumerate() {
        if i == 0 {
            for (k, v) in parse_method(line).iter() {
                out.insert(String::from(k), String::from(v));
            }
        } else {
            let mut chunks = line.split(": ");
            let key = chunks.nth(0).unwrap_or("").trim().to_lowercase();
            let value = chunks.nth(0).unwrap_or("").trim();
            if key.len() == 0 || value.len() == 0 {
                continue;
            }
            out.insert(key, value.into());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_empty_req() {
        assert_eq!(parse_request(""), HashMap::new());
    }
    #[test]
    fn test_get_sleep_req() {
        let r = "GET /sleep/ HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:70.0) Gecko/20100101 Firefox/70.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Cookie: PGADMIN_KEY=545c5f7d-bd15-44bf-8d8c-008ca33e5a61; PGADMIN_LANGUAGE=en
Upgrade-Insecure-Requests: 1
Cache-Control: max-age=0";
        let parsed = parse_request(r);
        assert!(
            parsed.get("endpoint").unwrap() == "/sleep/" && parsed.get("method").unwrap() == "GET"
        );
    }
    #[test]
    fn test_get_index_req() {
        let r = "GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:70.0) Gecko/20100101 Firefox/70.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Cookie: PGADMIN_KEY=545c5f7d-bd15-44bf-8d8c-008ca33e5a61; PGADMIN_LANGUAGE=en
Upgrade-Insecure-Requests: 1
Cache-Control: max-age=0";
        let parsed = parse_request(r);
        assert!(parsed.get("endpoint").unwrap() == "/" && parsed.get("method").unwrap() == "GET");
    }
    #[test]
    fn is_not_get_request() {
        let is_it = is_get(&HashMap::new());
        assert!(!is_it);
    }
    #[test]
    fn is_get_request() {
        let req: HashMap<_, _> = [("method", "GET")]
            .iter()
            .map(|t| (String::from(t.0), (String::from(t.1))))
            .collect();
        let is_it = is_get(&req);
        assert!(is_it);
    }

}
