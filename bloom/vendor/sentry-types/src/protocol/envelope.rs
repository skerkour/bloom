use std::io::Write;

use uuid::Uuid;

use super::v7::Event;
use super::v7::SessionUpdate;
use super::v7::Transaction;

/// An Envelope Item.
///
/// See the [documentation on Items](https://develop.sentry.dev/sdk/envelopes/#items)
/// for more details.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum EnvelopeItem {
    /// An Event Item.
    ///
    /// See the [Event Item documentation](https://develop.sentry.dev/sdk/envelopes/#event)
    /// for more details.
    Event(Event<'static>),
    /// A Session Item.
    ///
    /// See the [Session Item documentation](https://develop.sentry.dev/sdk/envelopes/#session)
    /// for more details.
    SessionUpdate(SessionUpdate<'static>),
    /// A Transaction Item.
    ///
    /// See the [Transaction Item documentation](https://develop.sentry.dev/sdk/envelopes/#transaction)
    /// for more details.
    Transaction(Transaction<'static>),
    // TODO:
    // * Attachment,
    // etc…
}

impl From<Event<'static>> for EnvelopeItem {
    fn from(event: Event<'static>) -> Self {
        EnvelopeItem::Event(event)
    }
}

impl From<SessionUpdate<'static>> for EnvelopeItem {
    fn from(session: SessionUpdate<'static>) -> Self {
        EnvelopeItem::SessionUpdate(session)
    }
}

impl From<Transaction<'static>> for EnvelopeItem {
    fn from(session: Transaction<'static>) -> Self {
        EnvelopeItem::Transaction(session)
    }
}

/// An Iterator over the items of an Envelope.
#[derive(Clone)]
pub struct EnvelopeItemIter<'s> {
    inner: std::slice::Iter<'s, EnvelopeItem>,
}

impl<'s> Iterator for EnvelopeItemIter<'s> {
    type Item = &'s EnvelopeItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// A Sentry Envelope.
///
/// An Envelope is the data format that Sentry uses for Ingestion. It can contain
/// multiple Items, some of which are related, such as Events, and Event Attachments.
/// Other Items, such as Sessions are independent.
///
/// See the [documentation on Envelopes](https://develop.sentry.dev/sdk/envelopes/)
/// for more details.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Envelope {
    event_id: Option<Uuid>,
    items: Vec<EnvelopeItem>,
}

impl Envelope {
    /// Creates a new empty Envelope.
    pub fn new() -> Envelope {
        Default::default()
    }

    /// Add a new Envelope Item.
    pub fn add_item<I>(&mut self, item: I)
    where
        I: Into<EnvelopeItem>,
    {
        let item = item.into();
        if self.event_id.is_none() {
            if let EnvelopeItem::Event(ref event) = item {
                self.event_id = Some(event.event_id);
            } else if let EnvelopeItem::Transaction(ref transaction) = item {
                self.event_id = Some(transaction.event_id);
            }
        }
        self.items.push(item);
    }

    /// Create an [`Iterator`] over all the [`EnvelopeItem`]s.
    pub fn items(&self) -> EnvelopeItemIter {
        EnvelopeItemIter {
            inner: self.items.iter(),
        }
    }

    /// Returns the Envelopes Uuid, if any.
    pub fn uuid(&self) -> Option<&Uuid> {
        self.event_id.as_ref()
    }

    /// Returns the [`Event`] contained in this Envelope, if any.
    ///
    /// [`Event`]: struct.Event.html
    pub fn event(&self) -> Option<&Event<'static>> {
        self.items
            .iter()
            .filter_map(|item| match item {
                EnvelopeItem::Event(event) => Some(event),
                _ => None,
            })
            .next()
    }

    /// Serialize the Envelope into the given [`Write`].
    ///
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    pub fn to_writer<W>(&self, mut writer: W) -> std::io::Result<()>
    where
        W: Write,
    {
        let mut item_buf = Vec::new();

        // write the headers:
        let event_id = self.uuid();
        match event_id {
            Some(uuid) => writeln!(writer, r#"{{"event_id":"{}"}}"#, uuid)?,
            _ => writeln!(writer, "{{}}")?,
        }

        // write each item:
        for item in &self.items {
            // we write them to a temporary buffer first, since we need their length
            match item {
                EnvelopeItem::Event(event) => serde_json::to_writer(&mut item_buf, event)?,
                EnvelopeItem::SessionUpdate(session) => {
                    serde_json::to_writer(&mut item_buf, session)?
                }
                EnvelopeItem::Transaction(transaction) => {
                    serde_json::to_writer(&mut item_buf, transaction)?
                }
            }
            let item_type = match item {
                EnvelopeItem::Event(_) => "event",
                EnvelopeItem::SessionUpdate(_) => "session",
                EnvelopeItem::Transaction(_) => "transaction",
            };
            writeln!(
                writer,
                r#"{{"type":"{}","length":{}}}"#,
                item_type,
                item_buf.len()
            )?;
            writer.write_all(&item_buf)?;
            writeln!(writer)?;
            item_buf.clear();
        }

        Ok(())
    }
}

impl From<Event<'static>> for Envelope {
    fn from(event: Event<'static>) -> Self {
        let mut envelope = Self::default();
        envelope.add_item(event);
        envelope
    }
}

impl From<Transaction<'static>> for Envelope {
    fn from(transaction: Transaction<'static>) -> Self {
        let mut envelope = Self::default();
        envelope.add_item(transaction);
        envelope
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Utc};

    use super::*;
    use crate::protocol::v7::{SessionAttributes, SessionStatus, Span};

    fn to_str(envelope: Envelope) -> String {
        let mut vec = Vec::new();
        envelope.to_writer(&mut vec).unwrap();
        String::from_utf8_lossy(&vec).to_string()
    }

    #[test]
    fn test_empty() {
        assert_eq!(to_str(Envelope::new()), "{}\n");
    }

    #[test]
    fn test_event() {
        let event_id = Uuid::parse_str("22d00b3f-d1b1-4b5d-8d20-49d138cd8a9c").unwrap();
        let timestamp = "2020-07-20T14:51:14.296Z".parse::<DateTime<Utc>>().unwrap();
        let event = Event {
            event_id,
            timestamp,
            ..Default::default()
        };
        let envelope: Envelope = event.into();
        assert_eq!(
            to_str(envelope),
            r#"{"event_id":"22d00b3f-d1b1-4b5d-8d20-49d138cd8a9c"}
{"type":"event","length":74}
{"event_id":"22d00b3fd1b14b5d8d2049d138cd8a9c","timestamp":1595256674.296}
"#
        )
    }

    #[test]
    fn test_session() {
        let session_id = Uuid::parse_str("22d00b3f-d1b1-4b5d-8d20-49d138cd8a9c").unwrap();
        let started = "2020-07-20T14:51:14.296Z".parse::<DateTime<Utc>>().unwrap();
        let session = SessionUpdate {
            session_id,
            distinct_id: Some("foo@bar.baz".to_owned()),
            sequence: None,
            timestamp: None,
            started,
            init: true,
            duration: Some(1.234),
            status: SessionStatus::Ok,
            errors: 123,
            attributes: SessionAttributes {
                release: "foo-bar@1.2.3".into(),
                environment: Some("production".into()),
                ip_address: None,
                user_agent: None,
            },
        };
        let mut envelope = Envelope::new();
        envelope.add_item(session);
        assert_eq!(
            to_str(envelope),
            r#"{}
{"type":"session","length":222}
{"sid":"22d00b3f-d1b1-4b5d-8d20-49d138cd8a9c","did":"foo@bar.baz","started":"2020-07-20T14:51:14.296Z","init":true,"duration":1.234,"status":"ok","errors":123,"attrs":{"release":"foo-bar@1.2.3","environment":"production"}}
"#
        )
    }

    #[test]
    fn test_transaction() {
        let event_id = Uuid::parse_str("22d00b3f-d1b1-4b5d-8d20-49d138cd8a9c").unwrap();
        let span_id = Uuid::parse_str("d42cee9f-c3e7-4f5c-ada9-47ab601a14d2").unwrap();
        let trace_id = Uuid::parse_str("335e53d6-1447-4acc-9f89-e632b776cc28").unwrap();
        let start_timestamp = "2020-07-20T14:51:14.296Z".parse::<DateTime<Utc>>().unwrap();
        let spans = vec![Span {
            span_id,
            trace_id,
            start_timestamp,
            ..Default::default()
        }];
        let transaction = Transaction {
            event_id,
            start_timestamp,
            spans,
            ..Default::default()
        };
        let envelope: Envelope = transaction.into();
        assert_eq!(
            to_str(envelope),
            r#"{"event_id":"22d00b3f-d1b1-4b5d-8d20-49d138cd8a9c"}
{"type":"transaction","length":216}
{"event_id":"22d00b3fd1b14b5d8d2049d138cd8a9c","start_timestamp":1595256674.296,"spans":[{"span_id":"d42cee9fc3e74f5cada947ab601a14d2","trace_id":"335e53d614474acc9f89e632b776cc28","start_timestamp":1595256674.296}]}
"#
        )
    }
}
