# thruster-basic-template

A minimal template for starting a [thruster](https://github.com/trezm/Thruster) website project with :
- tests
- coverage 100%
- browser refresh

## Help

- First install [cargo generate](https://github.com/ashleygwilliams/cargo-generate):  ``cargo install --force cargo-generate``
- ``cargo generate --git https://github.com/ami44/thruster-basic-template.git --name myproject``
- ``cd myproject``
- ``fastmod -d . 'YOURORGANISATION' 'myorganisation'``

# ===============================

# {{project-name}}

## Install

- ``git clone https://github.com/YOURORGANISATION/{{project-name}}.git``
- ``cd {{project-name}}``
- ``cargo run``
- ``open http://localhost:4321``

## Develop

### Watcher

- install catflap + cargo-cmd + cargo-watch beforehand ``cargo install --force catflap cargo-cmd cargo-watch``
- ``export RUST_LOG=myapp=trace`` - optional trace|debug|info|warn|error
- ``export PORT=4321`` - optional, default 4321
- ``cargo cmd watch_debug`` or ``cargo cmd watch`` (alias to ``watch_debug``)

### Liverelaod

> Firefox, Chrome, Edge but your html must contain the tag ``<script src="http://127.0.0.1:35729/livereload.js"></script>``

- install [nodejs](https://nodejs.org) beforehand
- ``npm install nodemon -g`` to add cmd ``nodemon``
- ``npm install make-livereload -g`` to add cmd ``tiny-lr``
- ``tiny-lr`` start livereload server, port=35729
- navigate to http://localhost:4321
- ``nodemon --delay 3000ms --watch target/debug/myapp --exec 'curl --noproxy "*" http://localhost:35729/changed?files=index.html'`` (adapt delay according to your compilation time)

