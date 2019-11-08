use std::collections::HashMap;
///! This module provides functions to parse HTTP requests.
///! For now, the only public function provided is `parse_request` which
///! takes the string value of a request, consumed from the stream.

fn parse_method(line: &str) -> HashMap<String, String> {
    ["method", "endpoint", "http"]
        .iter()
        .map(|c| String::from(*c))
        .zip(line.split(" ").map(|c| String::from(c)))
        .collect()
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
            out.insert(
                chunks.nth(0).unwrap_or("").to_lowercase(),
                String::from(chunks.nth(0).unwrap_or("")),
            );
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::parse_request;

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
        assert!(parsed.get("endpoint").unwrap() == "/sleep/" && parsed.get("method").unwrap() == "GET");
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
}
