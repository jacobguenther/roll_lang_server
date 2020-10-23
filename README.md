# Roll Lang

Roll Lang is a domain specific language for interpreting common RPG and tabletop game dice rolls and math operations. You can check out the [demo](https://roll.quaternion.site), [help](https://roll.quaternion.site/help), or [about](https://roll.quaternion.site/about) pages to find out more.

## Building and Running Locally

Clone this repository

`git clone https://github.com/jacobguenther/roll_lang_server.git`

`cd roll_lang_server`

Get the submodules

`git submodule init`

`git submodule update`

Install wasm-pack

`curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`

You will have to setup PostgreSQL on your own for now.

Install diesel cli for managing database migrations

`cargo install diesel-cli`

`diesel migration run`

Then build the project

`bash build.sh`

To launch the application enter

`cargo run`

you can find the website at `localhost:9080`


## Developers

This repository and its companion repositories [roll_lang_frontend](https://github.com/jacobguenther/roll_lang_frontend) and [roll_lang](https://github.com/jacobguenther/roll_lang) use [github projects](https://github.com/jacobguenther?tab=projects) to track development.

This project uses the (git-flow)[https://nvie.com/posts/a-successful-git-branching-model/] branching model.

## License

The majority of the code in this repository is licensed under the [GNU General Public License v3](https://www.gnu.org/licenses/gpl-3.0.en.html).