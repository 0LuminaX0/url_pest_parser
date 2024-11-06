use anyhow::Result;
use pest::Parser;
use url_pest_parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    mod protocol {
        use super::*;

        #[test]
        fn valid_protocols() -> Result<()> {
            let pair = URLParser::parse(Rule::protocol, "http")?.next().unwrap();
            assert_eq!(pair.as_str(), "http");

            let pair = URLParser::parse(Rule::protocol, "https")?.next().unwrap();
            assert_eq!(pair.as_str(), "https");

            Ok(())
        }

        #[test]
        fn invalid_protocols() {
            assert!(
                URLParser::parse(Rule::protocol, "ftp").is_err(),
                "Invalid protocol should fail"
            );
            assert!(
                URLParser::parse(Rule::protocol, "htt").is_err(),
                "Incomplete protocol should fail"
            );
        }
    }

    mod subdomain {
        use super::*;

        #[test]
        fn valid_subdomain() -> Result<()> {
            let pair = URLParser::parse(Rule::subdomain, "blog")?.next().unwrap();
            assert_eq!(pair.as_str(), "blog");
            Ok(())
        }
    }

    mod domain {
        use super::*;

        #[test]
        fn valid_domain() -> Result<()> {
            let pair = URLParser::parse(Rule::domain, "example.com")?
                .next()
                .unwrap();
            assert_eq!(pair.as_str(), "example.com");
            Ok(())
        }
    }

    mod path {
        use super::*;

        #[test]
        fn valid_path() -> Result<()> {
            let pair = URLParser::parse(Rule::path, "/path/to/resource")?
                .next()
                .unwrap();
            assert_eq!(pair.as_str(), "/path/to/resource");
            Ok(())
        }
    }

    mod query {
        use super::*;

        #[test]
        fn valid_query() -> Result<()> {
            let pair = URLParser::parse(Rule::query, "?key1=value1&key2=value2")?
                .next()
                .unwrap();
            assert_eq!(pair.as_str(), "?key1=value1&key2=value2");
            Ok(())
        }
    }

    mod fragment {
        use super::*;

        #[test]
        fn valid_fragment() -> Result<()> {
            let pair = URLParser::parse(Rule::fragment, "#section")?
                .next()
                .unwrap();
            assert_eq!(pair.as_str(), "#section");
            Ok(())
        }
    }

    #[test]
    fn test_invalid_symbols_in_url() {
        let invalid_url = "https://blog.example.com:8080/path/to#res@urce?param=value#section";
        assert!(
            URLParser::parse(Rule::url, invalid_url).is_err(),
            "URL with invalid symbols should fail"
        );
    }

    #[test]
    fn test_complete_url() {
        let url =
            "https://sub.example.com:8080/path/to/resource?param1=value1&param2=value2#section";
        let parsed = ParsedURL::from_url(url).expect("Failed to parse URL");
        assert_eq!(parsed.protocol, "https");
        assert_eq!(parsed.subdomain, Some("sub".to_string()));
        assert_eq!(parsed.domain, "example.com");
        assert_eq!(parsed.port, Some(8080));
        assert_eq!(
            parsed.path,
            Some(vec![
                "path".to_string(),
                "to".to_string(),
                "resource".to_string()
            ])
        );
        assert_eq!(
            parsed.query,
            Some(vec![
                ("param1".to_string(), "value1".to_string()),
                ("param2".to_string(), "value2".to_string())
            ])
        );
        assert_eq!(parsed.fragment, Some("section".to_string()));
    }

    #[test]
    fn test_minimal_url() {
        let url = "http://example.com";
        let parsed = ParsedURL::from_url(url).expect("Failed to parse URL");
        assert_eq!(parsed.protocol, "http");
        assert_eq!(parsed.subdomain, Some("example".to_string()));
        assert_eq!(parsed.domain, "com");
        assert_eq!(parsed.port, None);
        assert_eq!(parsed.path, None);
        assert_eq!(parsed.query, None);
        assert_eq!(parsed.fragment, None);
    }
}
