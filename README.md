# Roll Lang

Roll Lang is a domain specific language for interpreting common RPG and tabletop game dice rolls and math operations. You can check out the [demo](https://roll.quaternion.site), [help](https://roll.quaternion.site/help), or [about](https://roll.quaternion.site/about) pages to find out more.

This repository and its companion repositories [roll_lang_frontend](https://github.com/jacobguenther/roll_lang_frontend) and [roll_lang](https://github.com/jacobguenther/roll_lang) use [github projects](https://github.com/jacobguenther?tab=projects) to manage development.

## Building and Running Locally

clone this repository

`git clone https://github.com/jacobguenther/roll_lang_server.git`

`cd roll_lang_server`

get the submodules

`git submodule init`

`git submodule update`

install wasm-pack

`curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`

then run

`bash build.sh`

you can find the website at `localhost:9080`
