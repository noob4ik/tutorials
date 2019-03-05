# Guessing game
## Compiling backend to WebAssembly
Please refer to [Fluence Book](https://fluence.network/docs/book/quickstart/backend.html) for instructions on how to setup Rust and WebAssembly, and build Fluence apps.

If you have everything installed, it's very simple:
```bash
$ cd backend/
$ cargo build --release --target wasm32-unknown-unknown
```

And then publish! [Fluence Book](https://fluence.network/docs/book/quickstart/publish.html) also has detailed instructions on publishing your apps.

## Running the frontend
You will need installed `npm`.

```bash
$ cd frontend/
$ npm install
$ npm run start
...

> frontend-template@1.0.0 start /path/to/guessing-game
> webpack-dev-server

ℹ ｢wds｣: Project is running at http://localhost:8080/
```

Now open [http://localhost:8080/](http://localhost:8080/), and have fun!

## Hacking
Currently it's possible to hack the random number generator, and predict what number will be next. Try to fix it :) You can get Rust source code [here](https://github.com/fluencelabs/fluence/tree/master/vm/examples/guessing-game).
