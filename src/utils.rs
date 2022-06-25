use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum DecodeType {
    Hex,
    Base64,
}

#[derive(Debug)]
pub struct DecodeError {
    decode_type: DecodeType,
    msg: String,
}

impl DecodeError {
    pub fn new(decode_type: DecodeType, msg: &str) -> DecodeError {
        DecodeError {
            decode_type,
            msg: String::from(msg),
        }
    }
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DecodeError decoding {:?}: {}",
            self.decode_type, &self.msg
        )
    }
}

impl Error for DecodeError {}

#[derive(Debug)]
pub struct Profile {
        pub email: String,
        pub role: String,
        pub uid: u32,
}

impl Profile {
        pub fn from(map: &HashMap<String, Vec<String>>) -> Profile {
                let mut email = String::from("");
                let mut uid = 0u32;
                let mut role = String::from("");
                if let Some(map_emails) = map.get("email") {
                        email = map_emails[0].clone();
                }
                if let Some(map_roles) = map.get("role") {
                        role = map_roles[0].clone();
                }
                if let Some(map_uids) = map.get("uid") {
                        uid = map_uids[0].parse().unwrap_or(0);
                }
                Profile { email, role, uid }
        }

        pub fn decode(encoded: &str) -> Result<Self, String> {
                let map = kv_string_to_map(encoded)?;
                Ok(Self::from(&map))
        }

        pub fn encode(&self) -> String {
                let mut stripped_email = self.email.clone();
                stripped_email.retain(|c| {
                        c != '=' && c != '&'
                });
                let mut stripped_role = self.role.clone();
                stripped_role.retain(|c| {
                        c != '=' && c != '&'
                });

                format!("email={}&role={}&uid={}", stripped_email, stripped_role, self.uid)
        }
}

enum ParseState {
    ParsingKey,
    ParsingVal(String),
}

pub fn kv_string_to_map(input: &str) -> Result<HashMap<String, Vec<String>>, String> {
    let mut parsed = HashMap::new();
    let mut state = ParseState::ParsingKey;
    let mut current = String::new();

    for byte in input.as_bytes() {
        match (byte, &state) {
            (b'=', ParseState::ParsingKey) if current.len() != 0 => {
                state = ParseState::ParsingVal(current);
                current = String::new();
            },
            (b'&', ParseState::ParsingVal(key)) => {
                let total = parsed.entry(key.clone()).or_insert(vec![]);
                total.push(current);
                current = String::new();
                state = ParseState::ParsingKey;
            },
            (character, _) if *character != b'=' && *character != b'&' => {
                current.push(*character as char);
            },
            (_, _) => {
                return Err(String::from("Unexpected character"));
            }
        }
    };

    match state {
        ParseState::ParsingVal(key) => {
            let total = parsed.entry(key.clone()).or_insert(vec![]);
            total.push(current);
            Ok(parsed)
        },
        ParseState::ParsingKey => {
            Err(String::from("Unexpected end of input"))
        }
    }
}

pub fn map_to_kv_string(map: &HashMap<String, Vec<String>>) -> Result<String, String> {
    let mut kv = String::new();

    for (key, vals) in map {
        if key.len() == 0 {
            return Err(String::from("Empty key in map"));
        }
        if key.contains("=") || key.contains("&") {
            return Err(String::from("Invalid characters in map items"));
        }
        if vals.iter().any(|val| val.contains("=") || val.contains("&")) {
            return Err(String::from("Invalid characters in map items"));
        }
        for val in vals {
            let pair = if kv.len() != 0 {
                format!("&{}={}", key, val)
            } else {
                format!("{}={}", key, val)
            };
            kv.push_str(&pair);
        }
    }

    return Ok(kv);
}

#[test]
fn test_kv_parser_valid() {
    let mut input = "foo=bar&baz=qux&idk=morewords";
    let mut parsed = kv_string_to_map(input).expect("parsing failed unexpectedly");
    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed.get(&String::from("foo")), Some(&vec![String::from("bar")]));
    assert_eq!(parsed.get(&String::from("baz")), Some(&vec![String::from("qux")]));
    assert_eq!(parsed.get(&String::from("idk")), Some(&vec![String::from("morewords")]));

    input = "foo=";
    parsed = kv_string_to_map(input).expect("parsing failed unexpectedly");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed.get(&String::from("foo")), Some(&vec![String::from("")]));
}

#[test]
fn test_kv_parser_invalid() {
    let mut input = "foo=bar&baz&bling=blong";
    assert_eq!(kv_string_to_map(input), Err(String::from("Unexpected character")));

    input = "foo=bar&baz=qux=idk";
    assert_eq!(kv_string_to_map(input), Err(String::from("Unexpected character")));

    input = "foo=bar&";
    assert_eq!(kv_string_to_map(input), Err(String::from("Unexpected end of input")));

    input = "foo=bar&bax";
    assert_eq!(kv_string_to_map(input), Err(String::from("Unexpected end of input")));
}

#[test]
fn test_kv_serializer_valid() {
    let mut map = HashMap::from([
        (String::from("foo"), vec![String::from("bar")]),
        (String::from("baz"), vec![String::from("qux")])
    ]);
    let mut serialized = map_to_kv_string(&map).expect("serializing failed");
    assert!(serialized == String::from("foo=bar&baz=qux") || serialized == String::from("baz=qux&foo=bar"));

    map = HashMap::from([]);
    serialized = map_to_kv_string(&map).expect("serializing failed");
    assert_eq!(serialized, String::from(""));

    map = HashMap::from([(String::from("empty"), vec![String::from("")])]);
    serialized = map_to_kv_string(&map).expect("serializing failed");
    assert_eq!(serialized, String::from("empty="));
}

#[test]
fn test_kv_serializer_invalid() {
    let mut map = HashMap::from([
        (String::from("foo&"), vec![String::from("bar")]),
        (String::from("bax"), vec![String::from("qux")])
    ]);
    assert_eq!(map_to_kv_string(&map), Err(String::from("Invalid characters in map items")));

    map = HashMap::from([
        (String::from("foo"), vec![String::from("bar")]),
        (String::from("bax"), vec![String::from("qux=")])
    ]);
    assert_eq!(map_to_kv_string(&map), Err(String::from("Invalid characters in map items")));

    map = HashMap::from([
        (String::from(""), vec![String::from("bar")]),
        (String::from("bax"), vec![String::from("qux")])
    ]);
    assert_eq!(map_to_kv_string(&map), Err(String::from("Empty key in map")));
}
