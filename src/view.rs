use web_view::*;
use web_view::Color;


pub fn with_build<F>(body:&str,style:&str,js:&str, theme:Color, invok_handler:F ) -> WVResult
where F: FnMut(&mut web_view::WebView<'_, ()>, &str) -> WVResult
 {
    let default_css  = include_str!("template/default.css");
    let css_boot  = include_str!("template/bootstrap-4/css/bootstrap.min.css");
    let default_js  = include_str!("template/default.js");
    // css_boot.push_str(&default_css);
    // let default_html = include_str!("template/index.html");
    let mut theme_css = format!(r#"
	background-color: rgb{theme};
    "#, theme=&format!("({},{},{})",theme.r,theme.g,theme.b));
    theme_css = theme_css.replace("++", "{");
    theme_css = theme_css.replace("--", "}");
    let html = format!(r#"
<!doctype html>
	<html style = "{theme}">
    <head>
        <style type="text/css" >
        {default_css}
        </style>
        <style type="text/css" >{css}</style>
    </head>
    <body style="{theme}">
        {body}
        <script type="text/javascript">{scripts}</script>
    </body>
</html>
"#, body=body, css=style, default_css=&format!("{}\n{}",css_boot , default_css), theme=theme_css, scripts=&format!("{}\n{}",default_js , js));
    let mut webview = web_view::builder()
        .title("example")
        .content(Content::Html(html))
        .size(360, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(invok_handler)
        .build()?;
    webview.set_color(theme);
    webview.run()
}




