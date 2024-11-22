mod cli;

use std::process::ExitCode;

fn print_response(response: ureq::Response) {
    println!(">{} {}", response.http_version(), response.status_text());
    let content = response.content_type();

    println!(">Headers:");
    let headers = [
        "Access-Control-Allow-Origin",
        "Connection",
        "Content-Length",
        "Content-Encoding",
        "Content-Type",
        "Date",
        "Etag",
        "Keep-Alive",
        "Last-Modified",
        "Server",
        "Set-Cookie",
        "Transfer-Encoding",
        "Vary"
    ];

    for header in headers {
        if let Some(value) = response.header(header) {
            println!("    {header}: {value}");
        }
    }

    if content.contains("text") || content.contains("json") {
        match response.into_string() {
            Ok(text) => println!(">Body:\n{text}"),
            Err(_) => println!(">Body: <non UTF-8>"),
        }
    } else if let Some(size) = response.header("Content-Length").and_then(|text| text.parse::<usize>().ok()) {
        println!(">Body: <Binary {size} bytes>");
    } else {
        println!(">Body: <Empty>");
    }
}

fn main() -> ExitCode {
    let args = cli::args();
    let http = args.build_http();

    let method = args.method.as_str();
    let response = http.request(method, &args.url).set("Content-Length", "0").call();
    match response {
        Ok(response) => {
            print_response(response);
            ExitCode::SUCCESS
        },
        Err(error) => match error {
            ureq::Error::Status(_, response) => {
                print_response(response);
                ExitCode::FAILURE
            },
            ureq::Error::Transport(error) => {
                eprintln!("!>>{error}");
                ExitCode::FAILURE
            },
        }
    }
}
