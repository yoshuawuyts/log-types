use log::kv::{Error, Key, Source, ToKey, ToValue, Value, Visitor};

/// Delimit an HTTP request
///
/// The key of this enum is serialized as `http_mark`, the value is either `request` or `response`.
#[derive(Debug)]
pub enum Http {
    /// Marks an HTTP Request.
    Request,
    /// Marks an HTTP Response.
    Response,
}

impl ToKey for Http {
    fn to_key(&self) -> Key<'_> {
        "http_mark".into()
    }
}

impl ToValue for Http {
    fn to_value(&self) -> Value<'_> {
        match self {
            Self::Request => "request".into(),
            Self::Response => "response".into(),
        }
    }
}

impl Source for Http {
    fn visit<'kvs>(&'kvs self, visitor: &mut dyn Visitor<'kvs>) -> Result<(), Error>
    where
        Self: Sized,
    {
        visitor.visit_pair(self.to_key(), self.to_value())?;
        Ok(())
    }

    #[inline]
    fn count(&self) -> usize {
        1
    }
}
