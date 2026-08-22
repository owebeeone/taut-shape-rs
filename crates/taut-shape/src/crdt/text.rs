//! Deterministic `text_crdt.profile/v1` projection over [`super::CrdtNode`].

use alloc::{
    collections::{BTreeMap, BTreeSet},
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{char, fmt::Write as _, str};

use super::CrdtNode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextProjection {
    pub text: String,
    pub diagnostics: Vec<String>,
}

#[derive(Clone, Debug)]
enum Json {
    Null,
    Bool(bool),
    String(String),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

impl Json {
    fn object(&self) -> Option<&BTreeMap<String, Json>> {
        if let Self::Object(value) = self {
            Some(value)
        } else {
            None
        }
    }
    fn string(&self) -> Option<&str> {
        if let Self::String(value) = self {
            Some(value)
        } else {
            None
        }
    }
    fn bool(&self) -> Option<bool> {
        if let Self::Bool(value) = self {
            Some(*value)
        } else {
            None
        }
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn parse(bytes: &'a [u8]) -> Result<Json, ()> {
        let mut parser = Self { bytes, pos: 0 };
        parser.ws();
        let value = parser.value()?;
        parser.ws();
        if parser.pos == bytes.len() {
            Ok(value)
        } else {
            Err(())
        }
    }
    fn ws(&mut self) {
        while self
            .bytes
            .get(self.pos)
            .is_some_and(u8::is_ascii_whitespace)
        {
            self.pos += 1;
        }
    }
    fn take(&mut self, byte: u8) -> Result<(), ()> {
        self.ws();
        if self.bytes.get(self.pos) == Some(&byte) {
            self.pos += 1;
            Ok(())
        } else {
            Err(())
        }
    }
    fn literal(&mut self, value: &[u8], result: Json) -> Result<Json, ()> {
        if self.bytes.get(self.pos..self.pos + value.len()) == Some(value) {
            self.pos += value.len();
            Ok(result)
        } else {
            Err(())
        }
    }
    fn value(&mut self) -> Result<Json, ()> {
        self.ws();
        match self.bytes.get(self.pos) {
            Some(b'n') => self.literal(b"null", Json::Null),
            Some(b't') => self.literal(b"true", Json::Bool(true)),
            Some(b'f') => self.literal(b"false", Json::Bool(false)),
            Some(b'"') => self.string_value().map(Json::String),
            Some(b'[') => self.array(),
            Some(b'{') => self.object(),
            _ => Err(()),
        }
    }
    fn string_value(&mut self) -> Result<String, ()> {
        self.take(b'"')?;
        let mut output = String::new();
        loop {
            let byte = *self.bytes.get(self.pos).ok_or(())?;
            self.pos += 1;
            match byte {
                b'"' => return Ok(output),
                b'\\' => {
                    let escaped = *self.bytes.get(self.pos).ok_or(())?;
                    self.pos += 1;
                    match escaped {
                        b'"' => output.push('"'),
                        b'\\' => output.push('\\'),
                        b'/' => output.push('/'),
                        b'b' => output.push('\u{8}'),
                        b'f' => output.push('\u{c}'),
                        b'n' => output.push('\n'),
                        b'r' => output.push('\r'),
                        b't' => output.push('\t'),
                        b'u' => output.push(self.unicode_escape()?),
                        _ => return Err(()),
                    }
                }
                0..=0x1f => return Err(()),
                0x20..=0x7f => output.push(char::from(byte)),
                _ => {
                    self.pos -= 1;
                    let rest = str::from_utf8(&self.bytes[self.pos..]).map_err(|_| ())?;
                    let character = rest.chars().next().ok_or(())?;
                    self.pos += character.len_utf8();
                    output.push(character);
                }
            }
        }
    }
    fn unicode_escape(&mut self) -> Result<char, ()> {
        let first = self.hex4()?;
        if (0xd800..=0xdbff).contains(&first) {
            if self.bytes.get(self.pos..self.pos + 2) != Some(b"\\u") {
                return Err(());
            }
            self.pos += 2;
            let second = self.hex4()?;
            if !(0xdc00..=0xdfff).contains(&second) {
                return Err(());
            }
            char::from_u32(0x10000 + ((first - 0xd800) << 10) + second - 0xdc00).ok_or(())
        } else {
            char::from_u32(first).ok_or(())
        }
    }
    fn hex4(&mut self) -> Result<u32, ()> {
        let mut value = 0;
        for _ in 0..4 {
            let byte = *self.bytes.get(self.pos).ok_or(())?;
            self.pos += 1;
            value = value * 16
                + match byte {
                    b'0'..=b'9' => u32::from(byte - b'0'),
                    b'a'..=b'f' => u32::from(byte - b'a' + 10),
                    b'A'..=b'F' => u32::from(byte - b'A' + 10),
                    _ => return Err(()),
                };
        }
        Ok(value)
    }
    fn array(&mut self) -> Result<Json, ()> {
        self.take(b'[')?;
        let mut values = Vec::new();
        self.ws();
        if self.bytes.get(self.pos) == Some(&b']') {
            self.pos += 1;
            return Ok(Json::Array(values));
        }
        loop {
            values.push(self.value()?);
            self.ws();
            match self.bytes.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    return Ok(Json::Array(values));
                }
                _ => return Err(()),
            }
        }
    }
    fn object(&mut self) -> Result<Json, ()> {
        self.take(b'{')?;
        let mut values = BTreeMap::new();
        self.ws();
        if self.bytes.get(self.pos) == Some(&b'}') {
            self.pos += 1;
            return Ok(Json::Object(values));
        }
        loop {
            let key = self.string_value()?;
            self.take(b':')?;
            if values.insert(key, self.value()?).is_some() {
                return Err(());
            }
            self.ws();
            match self.bytes.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(Json::Object(values));
                }
                _ => return Err(()),
            }
        }
    }
}

fn quoted(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{8}' => output.push_str("\\b"),
            '\u{c}' => output.push_str("\\f"),
            value if value < '\u{20}' => {
                let _ = write!(output, "\\u{:04x}", u32::from(value));
            }
            value => output.push(value),
        }
    }
    output.push('"');
    output
}

pub fn encode_text_insert(atom_id: &str, after: Option<&str>, text: &str) -> Vec<u8> {
    format!(
        "{{\"after\":{},\"atom_id\":{},\"kind\":\"insert\",\"text\":{}}}",
        after.map_or_else(|| "null".to_string(), quoted),
        quoted(atom_id),
        quoted(text)
    )
    .into_bytes()
}

pub fn encode_text_delete(atom_id: &str) -> Vec<u8> {
    format!("{{\"atom_id\":{},\"kind\":\"delete\"}}", quoted(atom_id)).into_bytes()
}

#[derive(Clone)]
struct Atom {
    after: Option<String>,
    text: String,
    key: (String, i64),
}

fn candidate_key(atom: &Atom) -> (&str, &str, &str, i64) {
    (
        atom.after.as_deref().unwrap_or(""),
        &atom.text,
        &atom.key.0,
        atom.key.1,
    )
}

pub fn project_text(node: &CrdtNode) -> TextProjection {
    let mut atoms: BTreeMap<String, Atom> = BTreeMap::new();
    let mut deleted = BTreeSet::new();
    let mut diagnostics = BTreeSet::new();
    if let Some(bootstrap) = node.bootstrap() {
        let valid = Parser::parse(&bootstrap.state).ok().and_then(|value| {
            let object = value.object()?;
            if object.len() != 1 {
                return None;
            }
            let Json::Array(rows) = object.get("atoms")? else {
                return None;
            };
            Some(rows.clone())
        });
        if let Some(rows) = valid {
            for row in rows {
                let Some(object) = row.object() else {
                    diagnostics.insert("invalid_bootstrap".to_string());
                    continue;
                };
                let parsed = (|| {
                    if object.len() != 4 {
                        return None;
                    }
                    let atom_id = object.get("atom_id")?.string()?;
                    let after = match object.get("after")? {
                        Json::Null => None,
                        Json::String(value) => Some(value.clone()),
                        _ => return None,
                    };
                    let text = object.get("text")?.string()?;
                    let is_deleted = object.get("deleted")?.bool()?;
                    if atom_id.is_empty() || text.is_empty() {
                        return None;
                    }
                    Some((atom_id.to_string(), after, text.to_string(), is_deleted))
                })();
                if let Some((atom_id, after, text, is_deleted)) = parsed {
                    atoms.insert(
                        atom_id.clone(),
                        Atom {
                            after,
                            text,
                            key: (String::new(), 0),
                        },
                    );
                    if is_deleted {
                        deleted.insert(atom_id);
                    }
                } else {
                    diagnostics.insert("invalid_bootstrap".to_string());
                }
            }
        } else {
            diagnostics.insert("invalid_bootstrap".to_string());
        }
    }
    for op in node.operations() {
        let invalid = || format!("invalid_payload:{}:{}", op.origin, op.seq);
        let Ok(value) = Parser::parse(&op.payload) else {
            diagnostics.insert(invalid());
            continue;
        };
        let Some(object) = value.object() else {
            diagnostics.insert(invalid());
            continue;
        };
        let Some(atom_id) = object.get("atom_id").and_then(Json::string) else {
            diagnostics.insert(invalid());
            continue;
        };
        if atom_id.is_empty() {
            diagnostics.insert(invalid());
            continue;
        }
        match object.get("kind").and_then(Json::string) {
            Some("delete") if object.len() == 2 => {
                deleted.insert(atom_id.to_string());
            }
            Some("insert") if object.len() == 4 => {
                let after = match object.get("after") {
                    Some(Json::Null) => None,
                    Some(Json::String(value)) => Some(value.clone()),
                    _ => {
                        diagnostics.insert(invalid());
                        continue;
                    }
                };
                let Some(text) = object
                    .get("text")
                    .and_then(Json::string)
                    .filter(|value| !value.is_empty())
                else {
                    diagnostics.insert(invalid());
                    continue;
                };
                let candidate = Atom {
                    after,
                    text: text.to_string(),
                    key: (op.origin.clone(), op.seq),
                };
                if let Some(previous) = atoms.get(atom_id) {
                    if previous.after != candidate.after
                        || previous.text != candidate.text
                        || previous.key != candidate.key
                    {
                        diagnostics.insert(format!("atom_equivocation:{atom_id}"));
                        if candidate_key(&candidate) < candidate_key(previous) {
                            atoms.insert(atom_id.to_string(), candidate);
                        }
                    }
                } else {
                    atoms.insert(atom_id.to_string(), candidate);
                }
            }
            _ => {
                diagnostics.insert(invalid());
            }
        }
    }
    let mut children: BTreeMap<Option<String>, Vec<String>> = BTreeMap::new();
    for (atom_id, atom) in &atoms {
        if atom
            .after
            .as_ref()
            .is_some_and(|parent| !atoms.contains_key(parent))
        {
            diagnostics.insert(format!(
                "missing_parent:{atom_id}:{}",
                atom.after.as_deref().unwrap_or("")
            ));
        } else {
            children
                .entry(atom.after.clone())
                .or_default()
                .push(atom_id.clone());
        }
    }
    let mut output = String::new();
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    #[allow(clippy::too_many_arguments, reason = "explicit DFS state keeps the no_std projection allocation-free beyond its owned sets")]
    fn visit(
        atom_id: &str,
        atoms: &BTreeMap<String, Atom>,
        children: &BTreeMap<Option<String>, Vec<String>>,
        deleted: &BTreeSet<String>,
        diagnostics: &mut BTreeSet<String>,
        visiting: &mut BTreeSet<String>,
        visited: &mut BTreeSet<String>,
        output: &mut String,
    ) {
        if visiting.contains(atom_id) {
            diagnostics.insert(format!("cycle:{atom_id}"));
            return;
        }
        if visited.contains(atom_id) {
            return;
        }
        visiting.insert(atom_id.to_string());
        if !deleted.contains(atom_id) {
            output.push_str(&atoms[atom_id].text);
        }
        if let Some(descendants) = children.get(&Some(atom_id.to_string())) {
            for child in descendants {
                visit(
                    child,
                    atoms,
                    children,
                    deleted,
                    diagnostics,
                    visiting,
                    visited,
                    output,
                );
            }
        }
        visiting.remove(atom_id);
        visited.insert(atom_id.to_string());
    }
    if let Some(roots) = children.get(&None) {
        for root in roots {
            visit(
                root,
                &atoms,
                &children,
                &deleted,
                &mut diagnostics,
                &mut visiting,
                &mut visited,
                &mut output,
            );
        }
    }
    for atom_id in atoms.keys() {
        if !visited.contains(atom_id)
            && atoms[atom_id]
                .after
                .as_ref()
                .is_some_and(|parent| atoms.contains_key(parent))
        {
            visit(
                atom_id,
                &atoms,
                &children,
                &deleted,
                &mut diagnostics,
                &mut visiting,
                &mut visited,
                &mut output,
            );
        }
    }
    TextProjection {
        text: output,
        diagnostics: diagnostics.into_iter().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crdt::CrdtInput;
    use crate::generated_crdt::{CrdtApply, CrdtClock, CrdtOp};

    #[test]
    fn text_converges_for_reordered_insert_and_delete() {
        let operations = [
            CrdtInput::Apply(CrdtApply {
                op: CrdtOp {
                    origin: "a".into(),
                    seq: 1,
                    deps: CrdtClock {
                        entries: Vec::new(),
                    },
                    payload: encode_text_insert("x", None, "X"),
                },
            }),
            CrdtInput::Apply(CrdtApply {
                op: CrdtOp {
                    origin: "b".into(),
                    seq: 1,
                    deps: CrdtClock {
                        entries: Vec::new(),
                    },
                    payload: encode_text_delete("x"),
                },
            }),
        ];
        let mut left = CrdtNode::default();
        let mut right = CrdtNode::default();
        left.handle(operations[0].clone());
        left.handle(operations[1].clone());
        right.handle(operations[1].clone());
        right.handle(operations[0].clone());
        assert_eq!(project_text(&left), project_text(&right));
        assert_eq!(project_text(&left).text, "");
    }
}
