use rust_embed::EmbeddedFile;

pub trait EmbedAsString {
    fn as_string(&self) -> String;
}

impl EmbedAsString for Option<EmbeddedFile> {
    fn as_string(&self) -> String {
        self.as_ref()
            .map(|f| String::from_utf8(f.data.to_vec()).unwrap_or_default())
            .unwrap_or_default()
    }
}
