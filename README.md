# URL Parser

*Link:* https://crates.io/crates/url_pest_parser
*Docs:* https://docs.rs/url_pest_parser/latest/url_pest_parser/

A Rust-based URL parser using the `pest` parsing library with ability to extract various components like protocol, domain, subdomain, port, path segments, query parameters, and fragments.

## Parsing Logic

The grammar is defined using `pest` and covers the following components:

- **Protocol**: Either "http" or "https".
- **Subdomain**: Optional subdomain that appears before the main domain.
- **Domain**: The main domain name.
- **Port**: Optional port number, appearing after a colon.
- **Path**: Segmented by `/`, representing directories or resources.
- **Query Parameters**: Key-value pairs in the query string, separated by `&`.
- **Fragment**: The portion of the URL following a `#`.

### How It Works

The parser processes URLs by breaking them into components using predefined grammar. Then the results are encapsulated in a structured `ParsedURL` object.

## Example

```text
protocol://subdomain.domain:port/path/to/resource?param1=value1&param2=value2#section
   └── protocol = "https"
   └── subdomain = "sub"
   └── domain = "example.com"
   └── port = 8080
   └── path = ["path", "to", "resource"]
   └── query = [("param1", "value1"), ("param2", "value2")]
   └── fragment = "section"
