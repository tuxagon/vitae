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

Once Node is installed, run
```
$ npm install
```
to install the node dependencies.

If you want to run the Neon CLI manually, you can use the Node scoping mechanism
to run the local project installed version with 
```
$ npx neon
```

Starting the electron app, should be as easy as running
```
$ npm start
```


