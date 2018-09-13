pub fn render(main: String) -> String {

    // -------------------------------------------------------------------
    // top
    // -------------------------------------------------------------------
    let top = format!(r###"<!DOCTYPE html>
    <html>
        <head>
            <title>{{project-name}}</title>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/uikit/3.0.0-rc.16/css/uikit.min.css" />
            <script src="https://cdnjs.cloudflare.com/ajax/libs/uikit/3.0.0-rc.16/js/uikit.min.js"></script>
            <script src="https://cdnjs.cloudflare.com/ajax/libs/uikit/3.0.0-rc.16/js/uikit-icons.min.js"></script>
        </head>
        <body>
        <nav class="uk-navbar-container uk-margin">
                <div class="uk-container">
                    <div class="uk-navbar">
                        <div class="uk-navbar-left">
                            <a class="uk-navbar-item uk-logo" href="/index.html">{{project-name}}</a>
                            <ul class="uk-navbar-nav">
                                <li class="uk-active"><a href="/index.html">Home</a></li>
                                <!--<li><a href="#">Parent</a></li>-->
                                <!--<li><a href="#">Item</a></li>-->
                                <!--<li><a href="#">Item</a></li>-->
                            </ul>
                        </div>
                    </div>
                </div>
        </nav>
        <div class="uk-container">
    "###);

    // -------------------------------------------------------------------
    // bottom
    // -------------------------------------------------------------------
    let bottom=r###"
        </div>
        <script src="http://127.0.0.1:35729/livereload.js"></script>
    </body>
    </html>
    "###;

    // -------------------------------------------------------------------
    // return
    // -------------------------------------------------------------------
    format!("{top}{main}{bottom}", top=&top, main=&main, bottom=&bottom)

}