use log::kv::{Key, Source, ToKey, ToValue, Value, Visitor, Error};

/// Delimit a span.
///
/// Spans delimit two arbitrary points in time. Usually a measurement between those two points
/// points to something useful.
///
/// The key of this enum is serialized as `span_mark`, the value is either `start` or `end`.
#[derive(Debug)]
pub enum Span {
    /// Marks the end of a span.
    End,
    /// Marks the start of a span.
    Start,
}

impl ToKey for Span {
    fn to_key(&self) -> Key<'_> {
        "span_mark".into()
    }
}

impl ToValue for Span {
    fn to_value(&self) -> Value<'_> {
        match self {
            Self::End => "end".into(),
            Self::Start => "start".into(),
        }
    }
}

impl Source for Span {
    fn visit<'kvs>(&'kvs self, visitor: &mut dyn Visitor<'kvs>) -> Result<(), Error> where Self: Sized {
        visitor.visit_pair(self.to_key(), self.to_value())?;
        Ok(())
    }

    #[inline]
    fn count(&self) -> usize {
        1
    }
}
