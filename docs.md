# URL Parser Grammar

This documentation explains the rules and structure of the URL parser grammar used in this project.

## Rules

### WHITESPACE
This rule matches any whitespace characters (space, tab, newline).

```pest
WHITESPACE = _{ " " | "\t" | "\n" }
```

### STRICT_SYMBOL
Matches symbols that are alphanumeric or one of the following: `-`, `_`, `.`.

```pest
STRICT_SYMBOL = _{ ASCII_ALPHANUMERIC | "-" | "_" | "." }
```

### SYMBOL
Matches a more extensive set of symbols that includes all strict symbols, plus special characters like `~`, `!`, `$`, `&`, etc.

```pest
SYMBOL = _{ STRICT_SYMBOL | "~" | "!" | "$" | "&" | "'" | "(" | ")" | "*" | "+" | "," | ";" | "=" | ":" | "/" | "?" }
```

### url
The `url` rule is the entry point for parsing the URL structure. It matches the following pattern:

```
protocol://[subdomain.]domain[:port][/path][?query][#fragment]
```

#### Example URL:
```
https://www.example.com:8080/path/to/resource?key=value#section1
```

```pest
url = { SOI ~ protocol ~ "://" ~ (subdomain ~ ".")? ~ domain ~ port? ~ path? ~ query? ~ fragment? ~ EOI }
```
