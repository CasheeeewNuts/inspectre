use once_cell::sync::Lazy;
use scraper::{Html, Selector};

static SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("a[href]").unwrap());

pub fn from_html(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let a_tag_list = document.select(&SELECTOR);

    return a_tag_list.into_iter()
        .map(|node| node.value().attr("href").unwrap().to_string())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let html = r#"
<html>
    <body>
        <div class="ssss"><ul><li name="nn">NotSelect</li></ul></div>
        <div class="some-list">
            <a href="https://sample.org"></a>
        </div>
    </body>
</html>"#.to_string();

        let links = from_html(&html);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0], "https://sample.org");
    }
}