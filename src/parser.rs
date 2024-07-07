use scraper::{Html, Selector};

pub struct AttrParser {
    html: Html,
}

impl AttrParser {
    pub fn from_str(html: &str) -> Self {
        AttrParser {
            html: Html::parse_document(&html),
        }
    }

    pub fn parse_attr_from_tag(&self, tag: String, attr: String) -> Result<Vec<&str>, String> {
        let mut attrs: Vec<&str> = Vec::new();
        let selector = Selector::parse(&tag)
            .map_err(|_| String::from(String::from("Couldn't parse CSS selector!")))?;
        for node in self.html.select(&selector) {
            if let Some(val) = node.attr(attr.as_str()) {
                attrs.push(val);
            }
        }
        Ok(attrs)
    }
}
