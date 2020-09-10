# Roll Lang

Roll Lang is a domain specific language for interpreting common RPG and tabletop game dice rolls and math operations. You can check out the [demo](https://roll.quaternion.site), [help](https://roll.quaternion.site/help), or [about](https://roll.quaternion.site/about) pages to find out more.

## Building

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

Note: wasm-pack will give some errors but it is fine

you can find the website at `localhost:9080`