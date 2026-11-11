# Changelog

## 0.1.0 (2026-11-11)


### Features

* **cli:** add --print-service ([7572102](https://github.com/wolfika/picomon/commit/7572102ee0358a1d0df22cf07e76774226bb9108))
* **collect:** read CPU, temperature, memory, disk, network and host metrics ([81d2167](https://github.com/wolfika/picomon/commit/81d21670a18ddcaa0f07c30b566fdab913c623b6))
* **contrib:** add hardened systemd unit ([67723b0](https://github.com/wolfika/picomon/commit/67723b076243191065bb1766480fc5c51495430c))
* scaffold Rust binary with CLI and TOML config ([a13968f](https://github.com/wolfika/picomon/commit/a13968f7295dbdfec71e36adc3559441367eb2aa))
* **server:** serve metrics over SSE with optional basic auth ([b9fff99](https://github.com/wolfika/picomon/commit/b9fff99de29a4f4b8ca6db9546244e99197a7f5b))
* **web:** add Catppuccin Latte, Frappé, Macchiato and Mocha themes ([471f7c3](https://github.com/wolfika/picomon/commit/471f7c3153523208ae3aaea083955c568fadf608))
* **web:** add Svelte dashboard with glass, dark, light and nord themes ([80450a1](https://github.com/wolfika/picomon/commit/80450a1c8a4f281cc5c7cf20de5ece4e94599e63))
* **web:** move theme switcher into a header icon menu ([515b036](https://github.com/wolfika/picomon/commit/515b036079b51a96fd4021e89fa2ed7514dd718d))
* **web:** redesign dashboard ([ad76bcc](https://github.com/wolfika/picomon/commit/ad76bcc7f588e1c385f92831e6f100c8ede8c1eb))
* **web:** remove footer ([691793d](https://github.com/wolfika/picomon/commit/691793d11e746b5d4a02588b38c340f5f3a7d64e))


### Bug Fixes

* **build:** make install copy an existing build instead of rebuilding ([bb9fa89](https://github.com/wolfika/picomon/commit/bb9fa89cfaa94465fba941c03f7f1ade7e374875))
* **build:** point the installed unit at PREFIX ([47216d0](https://github.com/wolfika/picomon/commit/47216d094f8968076f77a22d07cbfc8a31cf15dd))
* **build:** rebuild when web/dist changes ([6d1ac54](https://github.com/wolfika/picomon/commit/6d1ac5438603390beb55e8a2c7627b99208031ce))
* **collect:** count threads from /proc/stat ([6e10d78](https://github.com/wolfika/picomon/commit/6e10d78143b3a7ccbc6ac72cc7e59c545dfd8bd3))
* **collect:** show each mount path once ([fd703ed](https://github.com/wolfika/picomon/commit/fd703ed7d9622863482bc66e2e48c9ea8fc391e6))
* **collect:** track disk I/O per device ([031f0d0](https://github.com/wolfika/picomon/commit/031f0d0cac0ae54cb81821b9532b4ab4ac10948b))
* **config:** limit history to 10000 samples ([dbdfd3d](https://github.com/wolfika/picomon/commit/dbdfd3d1ca28c1e946c1da622dcab984b1aff01c))
* de-duplicate live samples by sequence number ([6173900](https://github.com/wolfika/picomon/commit/61739008ceab428863ff38ecce23fe4042eb9381))
* **web:** trim history correctly when it holds one point ([56e3f5e](https://github.com/wolfika/picomon/commit/56e3f5e487e14b8da2dcd703c26e86063dc85457))


### Performance Improvements

* **collect:** round CPU percentages and byte rates ([593fabd](https://github.com/wolfika/picomon/commit/593fabd65b41080ab60d4911e33c73988dc045ca))
