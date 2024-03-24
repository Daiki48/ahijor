pub const BASE: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="icon" href="{{ICON}}">
    <title>Ahijor template</title>
    <style>{{STYLE}}</style>
  </head>
  <body>
    {{content}}
  </body>
</html>
"#;
