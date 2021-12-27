use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Deserialize, Clone)]
pub struct Quote {
    /// The unique, autoincrementing ID of this quote.
    pub id: u32,

    /// The unique ID of the user who submitted this quote.
    pub author_id: Option<u64>,

    /// The actual content of this quote.
    pub content: String,

    /// The author of the quote.
    /// Not to be confused with ``author_id`` - this is reserved for the person who actually "said the quote", and not who submitted it.
    pub author: Option<String>,
}

impl Serialize for Quote {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Quote", 4)?;

        state.serialize_field("id", &self.id)?;
        state.serialize_field("author_id", &self.author_id)?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("author", &self.author)?;
        state.end()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QuoteData {
    /// The content of this quote.
    pub content: String,

    /// The author of this quote, if any.
    pub author: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QuotesData {
    /// An array of quotes, each quote containing a ``content`` and optionally an ``author`` field.
    pub quotes: Vec<QuoteData>,
}
