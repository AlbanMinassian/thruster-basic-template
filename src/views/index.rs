pub fn render() -> String {

    // -------------------------------------------------------------------
    // content
    // -------------------------------------------------------------------
    let content= r###"
    <h1>Hello Thruster</h1>
    <ul>
        <li><a href="/index.html">/index.html</a></li>
        <li><a href="/">/ (redirect to index.html)</a></li>
        <li><a href="/example/text">/example/text (plain text)</a></li>
        <li><a href="/example/json">/example/json</a></li>
        <li><a href="/notexist">404 page not exist</a></li>
    </ul>
    "###.to_string();

    // -------------------------------------------------------------------
    // return
    // -------------------------------------------------------------------
    content

}