use crate::{Nbt, NbtCompound};
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnbtParseError {
    position: usize,
    expectation: &'static str,
}

impl SnbtParseError {
    const fn new(position: usize, expectation: &'static str) -> Self {
        Self {
            position,
            expectation,
        }
    }

    pub const fn position(&self) -> usize {
        self.position
    }
}

impl Display for SnbtParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "expected {} at SNBT character {}",
            self.expectation, self.position
        )
    }
}

impl Error for SnbtParseError {}

pub fn parse_snbt_compound(input: &str) -> Result<NbtCompound, SnbtParseError> {
    let mut parser = SnbtParser::new(input);
    let compound = parser.parse_compound()?;
    parser.skip_whitespace();
    if parser.peek().is_some() {
        return Err(parser.error("end of input"));
    }
    Ok(compound)
}

struct SnbtParser {
    characters: Vec<char>,
    position: usize,
}

impl SnbtParser {
    fn new(input: &str) -> Self {
        Self {
            characters: input.chars().collect(),
            position: 0,
        }
    }

    fn parse_compound(&mut self) -> Result<NbtCompound, SnbtParseError> {
        self.expect('{')?;
        self.skip_whitespace();
        let mut compound = NbtCompound::new();
        if self.consume('}') {
            return Ok(compound);
        }
        loop {
            let key = self.parse_key()?;
            self.skip_whitespace();
            self.expect(':')?;
            let value = self.parse_value()?;
            compound.insert(key, value);
            self.skip_whitespace();
            if self.consume('}') {
                return Ok(compound);
            }
            self.expect(',')?;
            self.skip_whitespace();
        }
    }

    fn parse_key(&mut self) -> Result<String, SnbtParseError> {
        self.skip_whitespace();
        match self.peek() {
            Some('"') | Some('\'') => self.parse_quoted_string(),
            Some(_) => {
                let key = self.take_while(|character| {
                    !character.is_whitespace() && !matches!(character, ':' | ',' | '{' | '}')
                });
                if key.is_empty() {
                    return Err(self.error("compound key"));
                }
                Ok(key)
            }
            None => Err(self.error("compound key")),
        }
    }

    fn parse_value(&mut self) -> Result<Nbt, SnbtParseError> {
        self.skip_whitespace();
        match self.peek() {
            Some('{') => self.parse_compound().map(Nbt::Compound),
            Some('[') => self.parse_list_or_array(),
            Some('"') | Some('\'') => self.parse_quoted_string().map(Nbt::String),
            Some(_) => self.parse_unquoted_value(),
            None => Err(self.error("value")),
        }
    }

    fn parse_list_or_array(&mut self) -> Result<Nbt, SnbtParseError> {
        self.expect('[')?;
        self.skip_whitespace();
        let typed_array = self.peek().and_then(|character| match character {
            'B' | 'b' => Some(TypedArray::Byte),
            'I' | 'i' => Some(TypedArray::Int),
            'L' | 'l' => Some(TypedArray::Long),
            _ => None,
        });
        if let Some(typed_array) = typed_array {
            let marker_position = self.position;
            self.advance();
            self.skip_whitespace();
            if self.consume(';') {
                return self.parse_typed_array(typed_array);
            }
            self.position = marker_position;
        }
        self.parse_list()
    }

    fn parse_list(&mut self) -> Result<Nbt, SnbtParseError> {
        self.skip_whitespace();
        let mut values = Vec::new();
        if self.consume(']') {
            return Ok(Nbt::List(values.into_boxed_slice()));
        }
        loop {
            let value = self.parse_value()?;
            if let Some(first) = values.first()
                && first.get_type_id() != value.get_type_id()
            {
                return Err(self.error("list value with matching type"));
            }
            values.push(value);
            self.skip_whitespace();
            if self.consume(']') {
                return Ok(Nbt::List(values.into_boxed_slice()));
            }
            self.expect(',')?;
            self.skip_whitespace();
        }
    }

    fn parse_typed_array(&mut self, array_type: TypedArray) -> Result<Nbt, SnbtParseError> {
        self.skip_whitespace();
        let mut values = Vec::new();
        if self.consume(']') {
            return Ok(array_type.empty());
        }
        loop {
            let token = self.parse_unquoted_token()?;
            values.push(array_type.parse_value(&token, self.position)?);
            self.skip_whitespace();
            if self.consume(']') {
                return array_type.values(values, self.position);
            }
            self.expect(',')?;
            self.skip_whitespace();
        }
    }

    fn parse_unquoted_value(&mut self) -> Result<Nbt, SnbtParseError> {
        let token = self.parse_unquoted_token()?;
        Ok(parse_scalar(&token))
    }

    fn parse_unquoted_token(&mut self) -> Result<String, SnbtParseError> {
        let token = self.take_while(|character| {
            !character.is_whitespace() && !matches!(character, ',' | ']' | '}')
        });
        if token.is_empty() {
            return Err(self.error("value"));
        }
        Ok(token)
    }

    fn parse_quoted_string(&mut self) -> Result<String, SnbtParseError> {
        let quote = self.advance().ok_or_else(|| self.error("quote"))?;
        let mut value = String::new();
        loop {
            let character = self.advance().ok_or_else(|| self.error("closing quote"))?;
            if character == quote {
                return Ok(value);
            }
            if character != '\\' {
                value.push(character);
                continue;
            }
            let escaped = self
                .advance()
                .ok_or_else(|| self.error("escaped character"))?;
            value.push(match escaped {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                other => other,
            });
        }
    }

    fn expect(&mut self, expected: char) -> Result<(), SnbtParseError> {
        self.skip_whitespace();
        if self.consume(expected) {
            return Ok(());
        }
        Err(self.error(match expected {
            '{' => "'{'",
            '}' => "'}'",
            '[' => "'['",
            ']' => "']'",
            ':' => "':'",
            ',' => "','",
            ';' => "';'",
            _ => "expected character",
        }))
    }

    fn consume(&mut self, expected: char) -> bool {
        if self.peek() != Some(expected) {
            return false;
        }
        self.position += 1;
        true
    }

    fn take_while(&mut self, predicate: impl Fn(char) -> bool) -> String {
        let start = self.position;
        while self.peek().is_some_and(&predicate) {
            self.position += 1;
        }
        self.characters[start..self.position].iter().collect()
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.position += 1;
        }
    }

    fn peek(&self) -> Option<char> {
        self.characters.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let character = self.peek()?;
        self.position += 1;
        Some(character)
    }

    const fn error(&self, expectation: &'static str) -> SnbtParseError {
        SnbtParseError::new(self.position, expectation)
    }
}

#[derive(Clone, Copy)]
enum TypedArray {
    Byte,
    Int,
    Long,
}

impl TypedArray {
    fn parse_value(self, token: &str, position: usize) -> Result<i64, SnbtParseError> {
        let normalized = match self {
            Self::Byte => token
                .strip_suffix(['b', 'B'])
                .unwrap_or(token)
                .parse::<i8>()
                .map(i64::from),
            Self::Int => token.parse::<i32>().map(i64::from),
            Self::Long => token
                .strip_suffix(['l', 'L'])
                .unwrap_or(token)
                .parse::<i64>(),
        };
        normalized.map_err(|_| SnbtParseError::new(position, "typed array number"))
    }

    fn empty(self) -> Nbt {
        match self {
            Self::Byte => Nbt::ByteArray(Box::default()),
            Self::Int => Nbt::IntArray(Box::default()),
            Self::Long => Nbt::LongArray(Box::default()),
        }
    }

    fn values(self, values: Vec<i64>, position: usize) -> Result<Nbt, SnbtParseError> {
        match self {
            Self::Byte => values
                .into_iter()
                .map(i8::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map(|values| {
                    Nbt::ByteArray(
                        values
                            .into_iter()
                            .map(|value| value as u8)
                            .collect::<Vec<_>>()
                            .into_boxed_slice(),
                    )
                })
                .map_err(|_| SnbtParseError::new(position, "byte array value")),
            Self::Int => values
                .into_iter()
                .map(i32::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map(|values| Nbt::IntArray(values.into_boxed_slice()))
                .map_err(|_| SnbtParseError::new(position, "int array value")),
            Self::Long => Ok(Nbt::LongArray(values.into_boxed_slice())),
        }
    }
}

fn parse_scalar(token: &str) -> Nbt {
    if token.eq_ignore_ascii_case("true") {
        return Nbt::Byte(1);
    }
    if token.eq_ignore_ascii_case("false") {
        return Nbt::Byte(0);
    }
    if let Some(value) = strip_suffix(token, 'b').and_then(|value| value.parse::<i8>().ok()) {
        return Nbt::Byte(value);
    }
    if let Some(value) = strip_suffix(token, 's').and_then(|value| value.parse::<i16>().ok()) {
        return Nbt::Short(value);
    }
    if let Some(value) = strip_suffix(token, 'l').and_then(|value| value.parse::<i64>().ok()) {
        return Nbt::Long(value);
    }
    if let Some(value) = strip_suffix(token, 'f').and_then(|value| value.parse::<f32>().ok()) {
        return Nbt::Float(value);
    }
    if let Some(value) = strip_suffix(token, 'd').and_then(|value| value.parse::<f64>().ok()) {
        return Nbt::Double(value);
    }
    if let Ok(value) = token.parse::<i32>() {
        return Nbt::Int(value);
    }
    if let Ok(value) = token.parse::<f64>() {
        return Nbt::Double(value);
    }
    Nbt::String(token.to_owned())
}

fn strip_suffix(token: &str, suffix: char) -> Option<&str> {
    token
        .strip_suffix(suffix)
        .or_else(|| token.strip_suffix(suffix.to_ascii_uppercase()))
}
