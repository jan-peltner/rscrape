mod parser;

use parser::AttrParser;

fn main() -> Result<(), String> {
    let raw_html = r#"
        <!DOCTYPE html>
        <html>
        <head>
        <meta charset="utf-8">
        <title>Hello, world!</title>
        </head>
        <ul>
            <li><a href="www.foo.com">Link</a></li>
            <li><a href="www.bar.com">Link</a></li>
            <li><a href="www.baz.com">Link</a></li>
            <li><a href="www.pow.com">Link</a></li>
        </ul>
        </html>
    "#;
    let parser = AttrParser::from_str(raw_html);
    let attrs = parser.parse_attr_from_tag(String::from("a"), String::from("href"))?;
    dbg!(&attrs);
    Ok(())
}
