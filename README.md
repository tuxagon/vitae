# vitae-core

## setup

In an effort to manage versions, [asdf](https://asdf-vm.com/#/) is currently
used. Once installed, add the
[asdf-rust](https://github.com/code-lever/asdf-rust) and
[asdf-nodejs](https://github.com/asdf-vm/asdf-nodejs) plugins, then run
```
$ asdf install
```
to get the correct versions.

Once Node is installed, install `yarn` by running
```
$ npm install --global yarn
```

To start the Tauri app, run
```
$ yarn tauri dev
```

## development

Ensure you start the devserver for the UI first by running the following in separate tabs from the `src-frontend` directory

```
$ npm run start
```
```
$ npm run server
```

In a third tab, you can start the tauri app with the following from the root directory

```
$ npx tauri dev
```
or
```
$ yarn tauri dev
```
