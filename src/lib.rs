#![doc = include_str!("../docs.md")]

use ::pest_derive::Parser;
use pest::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct URLParser;

#[derive(Debug)]
pub struct ParsedURL {
    pub protocol: String,
    pub subdomain: Option<String>,
    pub domain: String,
    pub port: Option<u16>,
    pub path: Option<Vec<String>>,
    pub query: Option<Vec<(String, String)>>,
    pub fragment: Option<String>,
}

#[derive(Debug, Error)]
pub enum URLError {
    #[error("Failed to parse URL: {0}")]
    ParseError(String),
}

impl ParsedURL {
    pub fn from_url(url: &str) -> Result<Self, URLError> {
        let pairs =
            URLParser::parse(Rule::url, url).map_err(|e| URLError::ParseError(e.to_string()))?;
        let mut protocol = String::new();
        let mut subdomain = None;
        let mut domain = String::new();
        let mut port = None;
        let mut path = None;
        let mut query = None;
        let mut fragment = None;

        for pair in pairs {
            if pair.as_rule() == Rule::url {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::protocol => protocol = inner_pair.as_str().to_string(),
                        Rule::subdomain => subdomain = Some(inner_pair.as_str().to_string()),
                        Rule::domain => domain = inner_pair.as_str().to_string(),
                        Rule::port => {
                            port = Some(
                                inner_pair.as_str()[1..]
                                    .parse::<u16>()
                                    .expect("Port should be a valid u16"),
                            );
                        }
                        Rule::path => {
                            path = Some(
                                inner_pair
                                    .into_inner()
                                    .filter_map(|segment| {
                                        if segment.as_rule() == Rule::segment {
                                            Some(segment.as_str().to_string())
                                        } else {
                                            None
                                        }
                                    })
                                    .collect(),
                            );
                        }
                        Rule::query => {
                            query = Some(
                                inner_pair
                                    .into_inner()
                                    .map(|param| {
                                        let mut inner_rules = param.into_inner();
                                        let key = inner_rules.next().unwrap().as_str().to_string();
                                        let value =
                                            inner_rules.next().unwrap().as_str().to_string();
                                        (key, value)
                                    })
                                    .collect(),
                            );
                        }
                        Rule::fragment => fragment = Some(inner_pair.as_str()[1..].to_string()),
                        _ => {}
                    }
                }
            }
        }

        Ok(ParsedURL {
            protocol,
            subdomain,
            domain,
            port,
            path,
            query,
            fragment,
        })
    }
}
