use super::*;

use serde::de;

use std::borrow::Cow;
use std::iter::Iterator;
use std::slice::Iter;
use std::str;

macro_rules! tu {
    ($x:expr) => {
        match $x {
            Some(x) => *x,
            None => return Err(de::Error::custom("query string ended before expected")),
        }
    };
}

impl<'a> Level<'a> {
    /// If this `Level` value is indeed a map, then attempt to insert
    /// `value` for key `key`.
    /// Returns error if `self` is not a map, or already has an entry for that
    /// key.
    fn insert_map_value(&mut self, key: Cow<'a, str>, value: Cow<'a, str>) {
        if let Level::Nested(ref mut map) = *self {
            match map.entry(key) {
                Entry::Occupied(mut o) => {
                    // Throw away old result; map is now invalid anyway.
                    let _ = o.insert(Level::Invalid("Multiple values for one key"));
                }
                Entry::Vacant(vm) => {
                    // Map is empty, result is None
                    let _ = vm.insert(Level::Flat(value));
                }
            }
        } else if let Level::Uninitialised = *self {
            let mut map = BTreeMap::default();
            let _ = map.insert(key, Level::Flat(value));
            *self = Level::Nested(map);
        } else {
            *self = Level::Invalid(
                "Attempted to insert map value into \
                 non-map structure",
            );
        }
    }

    /// If this `Level` value is indeed a seq, then push a new value
    fn insert_ord_seq_value(&mut self, key: usize, value: Cow<'a, str>) {
        if let Level::OrderedSeq(ref mut map) = *self {
            match map.entry(key) {
                Entry::Occupied(mut o) => {
                    // Throw away old result; map is now invalid anyway.
                    let _ = o.insert(Level::Invalid("Multiple values for one key"));
                }
                Entry::Vacant(vm) => {
                    // Map is empty, result is None
                    let _ = vm.insert(Level::Flat(value));
                }
            }
        } else if let Level::Uninitialised = *self {
            // To reach here, self is either an OrderedSeq or nothing.
            let mut map = BTreeMap::default();
            let _ = map.insert(key, Level::Flat(value));
            *self = Level::OrderedSeq(map);
        } else {
            *self = Level::Invalid(
                "Attempted to insert seq value into \
                 non-seq structure",
            );
        }
    }

    /// If this `Level` value is indeed a seq, then attempt to insert
    /// `value` for key `key`.
    /// Returns error if `self` is not a seq, or already has an entry for that
    /// key.
    fn insert_seq_value(&mut self, value: Cow<'a, str>) {
        // Reached the end of the key string
        if let Level::Sequence(ref mut seq) = *self {
            seq.push(Level::Flat(value));
        } else if let Level::Uninitialised = *self {
            let mut seq = Vec::new();
            seq.push(Level::Flat(value));
            *self = Level::Sequence(seq);
        } else {
            *self = Level::Invalid(
                "Attempted to insert seq value into \
                 non-seq structure",
            );
        }
    }
}

/// The `Parser` struct is a stateful querystring parser.
/// It iterates over a slice of bytes, with a range to track the current
/// start/end points of a value.
/// The parser additionally supports peeking values, which allows them to be
/// re-used (precisely once, unlike with `Peekable` from `std::iter`).
pub struct Parser<'a> {
    inner: &'a [u8],
    iter: Iter<'a, u8>,
    index: usize,
    acc: (usize, usize),
    peeked: Option<&'a u8>,
    depth: usize, // stores the current depth, for use in bounded-depth parsing
    strict: bool,
    state: ParsingState,
}

/// The parsing logic varies slightly based on whether it is a key or a value
/// (determines how encoded brackets are parse in non-strict mode)
/// This tracks the state.
enum ParsingState {
    Init,
    Key,
    Value,
}

impl<'a> Iterator for Parser<'a> {
    type Item = &'a u8;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let preparse_brackets = match self.state {
            ParsingState::Value => false,
            _ => !self.strict,
        };
        if preparse_brackets {
            // in non-strict mode, we will happily decode any bracket
            match self.peeked.take() {
                Some(v) => Some(v),
                None => {
                    self.index += 1;
                    self.acc.1 += 1;
                    match self.iter.next() {
                        Some(v) if v == &b'%' && self.iter.len() >= 2 => {
                            match &self.iter.as_slice()[..2] {
                                b"5B" => {
                                    // skip the next two characters
                                    let _ = self.iter.next();
                                    let _ = self.iter.next();
                                    self.index += 2;
                                    Some(&b'[')
                                }
                                b"5D" => {
                                    // skip the next two characters
                                    let _ = self.iter.next();
                                    let _ = self.iter.next();
                                    self.index += 2;
                                    Some(&b']')
                                }
                                _ => Some(v),
                            }
                        }
                        Some(v) => Some(v),
                        None => None,
                    }
                }
            }
        } else {
            match self.peeked.take() {
                Some(v) => Some(v),
                None => {
                    self.index += 1;
                    self.acc.1 += 1;
                    self.iter.next()
                }
            }
        }
    }
}

impl<'a> Parser<'a> {
    #[inline]
    fn peek(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.peeked.is_some() {
            self.peeked
        } else if let Some(x) = self.next() {
            self.peeked = Some(x);
            Some(x)
        } else {
            None
        }
    }
}

/// Replace b'+' with b' '
/// Copied from [`form_urlencoded`](https://github.com/servo/rust-url/blob/380be29859adb859e861c2d765897c22ec878e01/src/form_urlencoded.rs#L125).
fn replace_plus(input: &[u8]) -> Cow<[u8]> {
    match input.iter().position(|&b| b == b'+') {
        None => Cow::Borrowed(input),
        Some(first_position) => {
            let mut replaced = input.to_owned();
            replaced[first_position] = b' ';
            for byte in &mut replaced[first_position + 1..] {
                if *byte == b'+' {
                    *byte = b' ';
                }
            }

            Cow::Owned(replaced)
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(encoded: &'a [u8], depth: usize, strict: bool) -> Self {
        Parser {
            inner: encoded,
            iter: encoded.iter(),
            acc: (0, 0),
            index: 0,
            peeked: None,
            depth,
            strict,
            state: ParsingState::Init,
        }
    }

    /// Resets the accumulator range by setting `(start, end)` to `(end, end)`.
    fn clear_acc(&mut self) {
        self.acc = (self.index, self.index);
    }

    /// Extracts a string from the internal byte slice from the range tracked by
    /// the parser.
    /// Avoids allocations when neither percent encoded, nor `'+'` values are
    /// present.
    fn collect_str(&mut self) -> Result<Cow<'a, str>> {
        let replaced = replace_plus(&self.inner[self.acc.0..self.acc.1 - 1]);
        let ret: Result<Cow<'a, str>> =
            match percent_encoding::percent_decode(&replaced).decode_utf8()? {
                Cow::Borrowed(_) => {
                    match replaced {
                        Cow::Borrowed(_) => {
                            // In this case, neither method made replacements, so we
                            // reuse the original bytes
                            let res = str::from_utf8(&self.inner[self.acc.0..self.acc.1 - 1])?;
                            Ok(Cow::Borrowed(res))
                        }
                        Cow::Owned(owned) => {
                            let res = String::from_utf8(owned)?;
                            Ok(Cow::Owned(res))
                        }
                    }
                }
                Cow::Owned(owned) => Ok(Cow::Owned(owned)),
            };
        self.clear_acc();
        ret.map_err(Error::from)
    }

    /// In some ways the main way to use a `Parser`, this runs the parsing step
    /// and outputs a simple `Deserializer` over the parsed map.
    pub(crate) fn as_deserializer(&mut self) -> Result<QsDeserializer<'a>> {
        let map = BTreeMap::default();
        let mut root = Level::Nested(map);

        // Parses all top level nodes into the `root` map.
        while self.parse(&mut root)? {}
        let iter = match root {
            Level::Nested(map) => map.into_iter(),
            _ => BTreeMap::default().into_iter(),
        };
        Ok(QsDeserializer { iter, value: None })
    }

    /// This is the top level parsing function. It checks the first character to
    /// decide the type of key (nested, sequence, etc.) and to call the
    /// approprate parsing function.
    ///
    /// Returns `Ok(false)` when there is no more string to parse.
    fn parse(&mut self, node: &mut Level<'a>) -> Result<bool> {
        // First character determines parsing type
        if self.depth == 0 {
            // Hit the maximum depth level, so parse everything as a key
            let key = self.parse_key(b'=', false)?;
            self.parse_map_value(key, node)?;
            return Ok(true);
        }
        match self.next() {
            Some(x) => {
                match *x {
                    b'[' => {
                        loop {
                            self.clear_acc();
                            // Only peek at the next value to determine the key type.
                            match tu!(self.peek()) {
                                // key is of the form "[..=", not really allowed.
                                b'[' => {
                                    // If we're in strict mode, error, otherwise just ignore it.
                                    if self.strict {
                                        return Err(super::Error::parse_err("found another opening bracket before the closed bracket", self.index));
                                    } else {
                                        let _ = self.next();
                                    }
                                }
                                // key is simply "[]", so treat as a seq.
                                b']' => {
                                    // throw away the bracket
                                    let _ = self.next();
                                    self.clear_acc();
                                    self.parse_seq_value(node)?;
                                    return Ok(true);
                                }
                                // First character is an integer, attempt to parse it as an integer key
                                b'0'..=b'9' => {
                                    let key = self.parse_key(b']', true)?;
                                    let key =
                                        usize::from_str_radix(&key, 10).map_err(Error::from)?;
                                    self.parse_ord_seq_value(key, node)?;
                                    return Ok(true);
                                }
                                // Key is "[a..=" so parse up to the closing "]"
                                0x20..=0x2f | 0x3a..=0x5a | 0x5c | 0x5e..=0x7e => {
                                    let key = self.parse_key(b']', true)?;
                                    self.parse_map_value(key, node)?;
                                    return Ok(true);
                                }
                                c => {
                                    if self.strict {
                                        return Err(super::Error::parse_err(
                                            &format!(
                                                "unexpected character: {}",
                                                String::from_utf8_lossy(&[c])
                                            ),
                                            self.index,
                                        ));
                                    } else {
                                        let _ = self.next();
                                    }
                                }
                            }
                        }
                    }
                    // Skip empty byte sequences (e.g. leading `&`, trailing `&`, `&&`, ...)
                    b'&' => {
                        self.clear_acc();
                        Ok(true)
                    }
                    // This means the key should be a root key
                    // of the form "abc" or "abc[..=]"
                    // We do actually allow integer keys here since they cannot
                    // be confused with sequences
                    _ => {
                        let key = { self.parse_key(b'[', false)? };
                        // Root keys are _always_ map values
                        self.parse_map_value(key, node)?;
                        Ok(true)
                    }
                }
            }
            // Ran out of characters to parse
            None => Ok(false),
        }
    }

    /// The iterator is currently pointing at a key, so parse up until the
    /// `end_on` value. This will either be `'['` when the key is the root key,
    /// or `']'` when the key is a nested key. In the former case, `'='` will
    /// also finish the key parsing.
    ///
    /// The `consume` flag determines whether the end character should be
    /// returned to the buffer to be peeked. This is important when
    /// parsing keys like `abc[def][ghi]` since the `'['` character is
    /// needed to for the next iteration of `parse`.
    fn parse_key(&mut self, end_on: u8, consume: bool) -> Result<Cow<'a, str>> {
        self.state = ParsingState::Key;
        loop {
            if let Some(x) = self.next() {
                match *x {
                    c if c == end_on => {
                        // Add this character back to the buffer for peek.
                        if !consume {
                            self.peeked = Some(x);
                        }
                        return self.collect_str();
                    }
                    b'=' => {
                        // Allow the '=' byte only when parsing keys within []
                        if end_on != b']' {
                            // Otherwise, we have reached the end of the key
                            // Add this character back to the buffer for peek.
                            self.peeked = Some(x);
                            return self.collect_str();
                        }

                        // otherwise do nothing, so '=' is accumulated
                    }
                    b'&' => {
                        // important to keep the `&` character so we know the
                        // key-value is of the form `key&..=` (i.e. no value)
                        self.peeked = Some(&b'&');
                        return self.collect_str();
                    }
                    _ => {
                        // for any other character
                        // do nothing, keep adding to key
                    }
                }
            } else {
                // no more string to parse
                return self.collect_str();
            }
        }
    }

    /// The `(key,value)` pair is determined to be corresponding to a map entry,
    /// so parse it as such. The first part of the `key` has been parsed.
    fn parse_map_value(&mut self, key: Cow<'a, str>, node: &mut Level<'a>) -> Result<()> {
        self.state = ParsingState::Key;
        let res = loop {
            if let Some(x) = self.peek() {
                match *x {
                    b'=' => {
                        // Key is finished, parse up until the '&' as the value
                        self.clear_acc();
                        self.state = ParsingState::Value;
                        for _ in self.take_while(|b| *b != &b'&') {}
                        let value: Cow<'a, str> = self.collect_str()?;
                        node.insert_map_value(key, value);
                        break Ok(());
                    }
                    b'&' => {
                        // No value
                        node.insert_map_value(key, Cow::Borrowed(""));
                        break Ok(());
                    }
                    b'[' => {
                        // The key continues to another level of nested.
                        // Add a new unitialised level for this node and continue.
                        if let Level::Uninitialised = *node {
                            *node = Level::Nested(BTreeMap::default());
                        }
                        if let Level::Nested(ref mut map) = *node {
                            // By parsing we drop down another level
                            self.depth -= 1;
                            // Either take the existing entry, or add a new
                            // unitialised level
                            // Use this new node to keep parsing
                            let _ = self.parse(map.entry(key).or_insert(Level::Uninitialised))?;
                            break Ok(());
                        } else {
                            // We expected to parse into a map here.
                            break Err(super::Error::parse_err(
                                &format!(
                                    "tried to insert a \
                                     new key into {:?}",
                                    node
                                ),
                                self.index,
                            ));
                        }
                    }
                    c => {
                        // Anything else is unexpected since we just finished
                        // parsing a key.
                        if self.strict {
                            break Err(super::Error::parse_err(
                                format!(
                                    "Unexpected character: '{}' found when parsing",
                                    String::from_utf8_lossy(&[c])
                                ),
                                self.index,
                            ));
                        } else {
                            let _ = self.next();
                        }
                    }
                }
            } else {
                // The string has ended, so the value is empty.
                node.insert_map_value(key, Cow::Borrowed(""));
                break Ok(());
            }
        };
        // We have finished parsing this level, so go back up a level.
        self.depth += 1;
        res
    }

    /// The `(key,value)` pair is determined to be corresponding to an
    /// ordered sequence.
    /// Basically the same as the above, but we insert into `OrderedSeq`
    /// Can potentially be merged?
    fn parse_ord_seq_value(&mut self, key: usize, node: &mut Level<'a>) -> Result<()> {
        self.state = ParsingState::Key;
        let res = loop {
            if let Some(x) = self.peek() {
                match *x {
                    b'=' => {
                        // Key is finished, parse up until the '&' as the value
                        self.clear_acc();
                        self.state = ParsingState::Value;
                        for _ in self.take_while(|b| *b != &b'&') {}
                        let value = self.collect_str()?;
                        // Reached the end of the key string
                        node.insert_ord_seq_value(key, value);
                        break Ok(());
                    }
                    b'&' => {
                        // No value
                        node.insert_ord_seq_value(key, Cow::Borrowed(""));
                        break Ok(());
                    }
                    b'[' => {
                        // The key continues to another level of nested.
                        // Add a new unitialised level for this node and continue.
                        if let Level::Uninitialised = *node {
                            *node = Level::OrderedSeq(BTreeMap::default());
                        }
                        if let Level::OrderedSeq(ref mut map) = *node {
                            // By parsing we drop down another level
                            self.depth -= 1;
                            let _ = self.parse(
                                // Either take the existing entry, or add a new
                                // unitialised level
                                // Use this new node to keep parsing
                                map.entry(key).or_insert(Level::Uninitialised),
                            )?;
                            break Ok(());
                        } else {
                            // We expected to parse into a seq here.
                            break Err(super::Error::parse_err(
                                &format!(
                                    "tried to insert a \
                                     new key into {:?}",
                                    node
                                ),
                                self.index,
                            ));
                        }
                    }
                    c => {
                        // Anything else is unexpected since we just finished
                        // parsing a key.
                        if self.strict {
                            break Err(super::Error::parse_err(
                                format!("Unexpected character: {:?} found when parsing", c),
                                self.index,
                            ));
                        } else {
                            let _ = self.next();
                        }
                    }
                }
            } else {
                // The string has ended, so the value is empty.
                node.insert_ord_seq_value(key, Cow::Borrowed(""));
                break Ok(());
            }
        };
        // We have finished parsing this level, so go back up a level.
        self.depth += 1;
        res
    }

    /// The `(key,value)` pair is determined to be corresponding to an
    /// unordered sequence.
    /// This must be the final level of nesting, so assume we have a value
    fn parse_seq_value(&mut self, node: &mut Level<'a>) -> Result<()> {
        self.state = ParsingState::Key;
        let res = match self.peek() {
            Some(x) => {
                match *x {
                    b'=' => {
                        // Key is finished, parse up until the '&' as the value
                        self.clear_acc();
                        self.state = ParsingState::Value;
                        for _ in self.take_while(|b| *b != &b'&') {}
                        let value = self.collect_str()?;
                        node.insert_seq_value(value);
                        Ok(())
                    }
                    b'&' => {
                        // key value is empty
                        node.insert_seq_value(Cow::Borrowed(""));
                        Ok(())
                    }
                    _ => Err(super::Error::parse_err(
                        "non-indexed sequence of \
                         structs not supported",
                        self.index,
                    )),
                }
            }
            None => {
                // The string has ended, so the value is empty.
                node.insert_seq_value(Cow::Borrowed(""));
                Ok(())
            }
        };
        // We have finished parsing this level, so go back up a level.
        self.depth += 1;
        res
    }
}
