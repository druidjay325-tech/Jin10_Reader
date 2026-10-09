# Windows 依赖许可证清单

此清单由 `node tools/third-party-licenses.mjs` 离线生成。目标 `x86_64-pc-windows-msvc`，使用当前默认功能的 Cargo 解析节点，包括进入该目标图的运行、构建和开发依赖；不把锁文件内所有其他平台包都算进 Windows 发行物。清单列的是依赖声明，不是对二进制实际链接代码的逐字节鉴定。

- Cargo.lock SHA-256：`c3c6a6decd2e6842f5f8e5a2fb3c38f735e1f1d1756ce5049b75536e3ad7c250`
- 目标图中的第三方包版本：**350 个**。
- 取得的独立许可/NOTICE 文本：**227 份不同内容**，相同文本按哈希去重，保留全部包与文件归属。
- 尚未覆盖许可文本的包版本：**0 个**。

原始文本保存在 [third-party-notices.txt](third-party-notices.txt)。生成器只读取本机已缓存包的许可、COPYING、COPYRIGHT、NOTICE 等文本，以及已经核对的上游补充许可，没有导出第三方源码或本机绝对路径。源码包附 `license-supplements.json`，原文出处也写入汇编。补充来源绑定包发布物的 VCS 提交并校验 SHA-256；其中 selectors 发布物没有独立许可文件，依其 MPL-2.0 声明及源文件许可头，收录 [Mozilla 官方 MPL 2.0 标准文本](https://www.mozilla.org/media/MPL/2.0/index.txt)，不伪称来自该 crate 的独立 LICENSE。生成器本身仍然全离线。发行源码与二进制附件应同时携带此清单和许可汇编；项目 MIT 不替代第三方许可。

## 许可表达式汇总

以下保留 Cargo 元数据中的原始 SPDX 表达式，不擅自替贡献者选择多许可中的某一个分支。

| 声明 | 包版本数 |
|---|---:|
| (Apache-2.0 OR MIT) AND BSD-3-Clause | 1 |
| (MIT OR Apache-2.0) AND Unicode-3.0 | 1 |
| 0BSD OR MIT OR Apache-2.0 | 1 |
| Apache-2.0 | 4 |
| Apache-2.0 / MIT | 1 |
| Apache-2.0 AND ISC | 1 |
| Apache-2.0 AND MIT | 1 |
| Apache-2.0 OR ISC OR MIT | 2 |
| Apache-2.0 OR MIT | 35 |
| BSD-3-Clause | 3 |
| BSD-3-Clause AND MIT | 1 |
| BSD-3-Clause/MIT | 1 |
| CC0-1.0 OR MIT-0 OR Apache-2.0 | 1 |
| ISC | 2 |
| ISC AND (Apache-2.0 OR ISC) | 1 |
| ISC AND (Apache-2.0 OR ISC) AND Apache-2.0 AND MIT AND BSD-3-Clause AND (Apache-2.0 OR ISC OR MIT) AND (Apache-2.0 OR ISC OR MIT-0) | 1 |
| MIT | 75 |
| MIT OR Apache-2.0 | 164 |
| MIT OR Apache-2.0 OR Zlib | 3 |
| MIT OR Zlib OR Apache-2.0 | 2 |
| MIT/Apache-2.0 | 13 |
| MPL-2.0 | 5 |
| Unicode-3.0 | 18 |
| Unlicense OR MIT | 8 |
| Unlicense/MIT | 2 |
| Zlib | 2 |
| Zlib OR Apache-2.0 OR MIT | 1 |

## 逐包清单

| 包 | 版本 | Cargo 声明 | 收录的包内许可文件 |
|---|---|---|---|
| [adler2](https://crates.io/crates/adler2/2.0.1) | 2.0.1 | 0BSD OR MIT OR Apache-2.0 | `LICENSE-0BSD`、`LICENSE-APACHE`、`LICENSE-MIT` |
| [aho-corasick](https://crates.io/crates/aho-corasick/1.1.5) | 1.1.5 | Unlicense OR MIT | `COPYING`、`LICENSE-MIT`、`UNLICENSE` |
| [alloc-no-stdlib](https://crates.io/crates/alloc-no-stdlib/2.0.4) | 2.0.4 | BSD-3-Clause | `LICENSE` |
| [alloc-stdlib](https://crates.io/crates/alloc-stdlib/0.2.4) | 0.2.4 | BSD-3-Clause | `上游补充：rust-alloc-no-stdlib-ae42d220-license.txt` |
| [anyhow](https://crates.io/crates/anyhow/1.0.104) | 1.0.104 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [atomic-waker](https://crates.io/crates/atomic-waker/1.1.2) | 1.1.2 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT`、`LICENSE-THIRD-PARTY` |
| [auto-launch](https://crates.io/crates/auto-launch/0.5.0) | 0.5.0 | MIT | `LICENSE` |
| [autocfg](https://crates.io/crates/autocfg/1.5.1) | 1.5.1 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [aws-lc-rs](https://crates.io/crates/aws-lc-rs/1.18.0) | 1.18.0 | ISC AND (Apache-2.0 OR ISC) | `LICENSE` |
| [aws-lc-sys](https://crates.io/crates/aws-lc-sys/0.44.0) | 0.44.0 | ISC AND (Apache-2.0 OR ISC) AND Apache-2.0 AND MIT AND BSD-3-Clause AND (Apache-2.0 OR ISC OR MIT) AND (Apache-2.0 OR ISC OR MIT-0) | `LICENSE`、`aws-lc/LICENSE` |
| [base64](https://crates.io/crates/base64/0.22.1) | 0.22.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [base64](https://crates.io/crates/base64/0.23.1) | 0.23.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [bit-set](https://crates.io/crates/bit-set/0.8.0) | 0.8.0 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [bit-vec](https://crates.io/crates/bit-vec/0.8.0) | 0.8.0 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [bitflags](https://crates.io/crates/bitflags/1.3.2) | 1.3.2 | MIT/Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [bitflags](https://crates.io/crates/bitflags/2.13.1) | 2.13.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [block-buffer](https://crates.io/crates/block-buffer/0.10.4) | 0.10.4 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [brotli](https://crates.io/crates/brotli/8.0.4) | 8.0.4 | BSD-3-Clause AND MIT | `LICENSE.BSD-3-Clause`、`LICENSE.MIT` |
| [brotli-decompressor](https://crates.io/crates/brotli-decompressor/5.0.3) | 5.0.3 | BSD-3-Clause/MIT | `LICENSE` |
| [bs58](https://crates.io/crates/bs58/0.5.1) | 0.5.1 | MIT/Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [byteorder](https://crates.io/crates/byteorder/1.5.0) | 1.5.0 | Unlicense OR MIT | `COPYING`、`LICENSE-MIT`、`UNLICENSE` |
| [bytes](https://crates.io/crates/bytes/1.12.1) | 1.12.1 | MIT | `LICENSE` |
| [camino](https://crates.io/crates/camino/1.2.5) | 1.2.5 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [cargo_metadata](https://crates.io/crates/cargo_metadata/0.19.2) | 0.19.2 | MIT | `LICENSE-MIT` |
| [cargo_toml](https://crates.io/crates/cargo_toml/0.22.3) | 0.22.3 | Apache-2.0 OR MIT | `LICENSE` |
| [cargo-platform](https://crates.io/crates/cargo-platform/0.1.9) | 0.1.9 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [cc](https://crates.io/crates/cc/1.4.4) | 1.4.4 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [cfb](https://crates.io/crates/cfb/0.7.3) | 0.7.3 | MIT | `LICENSE` |
| [cfg_aliases](https://crates.io/crates/cfg_aliases/0.2.2) | 0.2.2 | MIT | `LICENSE` |
| [cfg-if](https://crates.io/crates/cfg-if/1.0.4) | 1.0.4 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [chacha20](https://crates.io/crates/chacha20/0.10.2) | 0.10.2 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [chrono](https://crates.io/crates/chrono/0.4.45) | 0.4.45 | MIT OR Apache-2.0 | `LICENSE.txt` |
| [cmake](https://crates.io/crates/cmake/0.1.58) | 0.1.58 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [cookie](https://crates.io/crates/cookie/0.18.2) | 0.18.2 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [cpufeatures](https://crates.io/crates/cpufeatures/0.2.17) | 0.2.17 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [cpufeatures](https://crates.io/crates/cpufeatures/0.3.1) | 0.3.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [crc32fast](https://crates.io/crates/crc32fast/1.5.1) | 1.5.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [crossbeam-channel](https://crates.io/crates/crossbeam-channel/0.5.16) | 0.5.16 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT`、`LICENSE-THIRD-PARTY` |
| [crossbeam-utils](https://crates.io/crates/crossbeam-utils/0.8.22) | 0.8.22 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [crypto-common](https://crates.io/crates/crypto-common/0.1.7) | 0.1.7 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [cssparser](https://crates.io/crates/cssparser/0.36.0) | 0.36.0 | MPL-2.0 | `LICENSE` |
| [cssparser-macros](https://crates.io/crates/cssparser-macros/0.6.1) | 0.6.1 | MPL-2.0 | `LICENSE` |
| [ctor](https://crates.io/crates/ctor/0.8.0) | 0.8.0 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [ctor-proc-macro](https://crates.io/crates/ctor-proc-macro/0.0.7) | 0.0.7 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [darling](https://crates.io/crates/darling/0.23.0) | 0.23.0 | MIT | `LICENSE` |
| [darling](https://crates.io/crates/darling/0.24.1) | 0.24.1 | MIT | `LICENSE` |
| [darling_core](https://crates.io/crates/darling_core/0.23.0) | 0.23.0 | MIT | `LICENSE` |
| [darling_core](https://crates.io/crates/darling_core/0.24.1) | 0.24.1 | MIT | `LICENSE` |
| [darling_macro](https://crates.io/crates/darling_macro/0.23.0) | 0.23.0 | MIT | `LICENSE` |
| [darling_macro](https://crates.io/crates/darling_macro/0.24.1) | 0.24.1 | MIT | `LICENSE` |
| [defmt](https://crates.io/crates/defmt/1.1.1) | 1.1.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [defmt-macros](https://crates.io/crates/defmt-macros/1.1.1) | 1.1.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [defmt-parser](https://crates.io/crates/defmt-parser/1.0.0) | 1.0.0 | MIT OR Apache-2.0 | `上游补充：defmt-4a8cdb44-license-apache.txt`、`上游补充：defmt-4a8cdb44-license-mit.txt` |
| [deranged](https://crates.io/crates/deranged/0.5.8) | 0.5.8 | MIT OR Apache-2.0 | `LICENSE-Apache`、`LICENSE-MIT` |
| [derive_more](https://crates.io/crates/derive_more/2.1.1) | 2.1.1 | MIT | `LICENSE` |
| [derive_more-impl](https://crates.io/crates/derive_more-impl/2.1.1) | 2.1.1 | MIT | `LICENSE` |
| [digest](https://crates.io/crates/digest/0.10.7) | 0.10.7 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [dirs](https://crates.io/crates/dirs/6.0.0) | 6.0.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [dirs-sys](https://crates.io/crates/dirs-sys/0.5.0) | 0.5.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [displaydoc](https://crates.io/crates/displaydoc/0.2.7) | 0.2.7 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [dom_query](https://crates.io/crates/dom_query/0.27.0) | 0.27.0 | MIT | `LICENSE` |
| [dpi](https://crates.io/crates/dpi/0.1.2) | 0.1.2 | Apache-2.0 AND MIT | `LICENSE`、`LICENSE-LIBM-MIT` |
| [dtoa](https://crates.io/crates/dtoa/1.0.11) | 1.0.11 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [dtoa-short](https://crates.io/crates/dtoa-short/0.3.5) | 0.3.5 | MPL-2.0 | `LICENSE` |
| [dtor](https://crates.io/crates/dtor/0.3.0) | 0.3.0 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [dtor-proc-macro](https://crates.io/crates/dtor-proc-macro/0.0.6) | 0.0.6 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [dunce](https://crates.io/crates/dunce/1.0.5) | 1.0.5 | CC0-1.0 OR MIT-0 OR Apache-2.0 | `LICENSE` |
| [dyn-clone](https://crates.io/crates/dyn-clone/1.0.20) | 1.0.20 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [embed-resource](https://crates.io/crates/embed-resource/3.0.11) | 3.0.11 | MIT | `LICENSE` |
| [encoding_rs](https://crates.io/crates/encoding_rs/0.8.35) | 0.8.35 | (Apache-2.0 OR MIT) AND BSD-3-Clause | `COPYRIGHT`、`LICENSE-APACHE`、`LICENSE-MIT`、`LICENSE-WHATWG` |
| [equivalent](https://crates.io/crates/equivalent/1.0.2) | 1.0.2 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [erased-serde](https://crates.io/crates/erased-serde/0.4.10) | 0.4.10 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [fastrand](https://crates.io/crates/fastrand/2.5.0) | 2.5.0 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [fdeflate](https://crates.io/crates/fdeflate/0.3.7) | 0.3.7 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [find-msvc-tools](https://crates.io/crates/find-msvc-tools/0.1.11) | 0.1.11 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [flate2](https://crates.io/crates/flate2/1.1.10) | 1.1.10 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [fnv](https://crates.io/crates/fnv/1.0.7) | 1.0.7 | Apache-2.0 / MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [foldhash](https://crates.io/crates/foldhash/0.2.0) | 0.2.0 | Zlib | `LICENSE` |
| [form_urlencoded](https://crates.io/crates/form_urlencoded/1.2.2) | 1.2.2 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [fs_extra](https://crates.io/crates/fs_extra/1.3.0) | 1.3.0 | MIT | `LICENSE` |
| [futures](https://crates.io/crates/futures/0.3.34) | 0.3.34 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [futures-channel](https://crates.io/crates/futures-channel/0.3.34) | 0.3.34 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [futures-core](https://crates.io/crates/futures-core/0.3.34) | 0.3.34 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [futures-executor](https://crates.io/crates/futures-executor/0.3.34) | 0.3.34 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [futures-io](https://crates.io/crates/futures-io/0.3.34) | 0.3.34 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [futures-macro](https://crates.io/crates/futures-macro/0.3.34) | 0.3.34 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [futures-sink](https://crates.io/crates/futures-sink/0.3.34) | 0.3.34 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [futures-task](https://crates.io/crates/futures-task/0.3.34) | 0.3.34 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [futures-util](https://crates.io/crates/futures-util/0.3.34) | 0.3.34 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [generic-array](https://crates.io/crates/generic-array/0.14.7) | 0.14.7 | MIT | `LICENSE` |
| [getrandom](https://crates.io/crates/getrandom/0.2.17) | 0.2.17 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [getrandom](https://crates.io/crates/getrandom/0.3.4) | 0.3.4 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [getrandom](https://crates.io/crates/getrandom/0.4.3) | 0.4.3 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [glob](https://crates.io/crates/glob/0.3.4) | 0.3.4 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [global-hotkey](https://crates.io/crates/global-hotkey/0.8.0) | 0.8.0 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT`、`LICENSE.spdx` |
| [h2](https://crates.io/crates/h2/0.4.19) | 0.4.19 | MIT | `LICENSE` |
| [hashbrown](https://crates.io/crates/hashbrown/0.12.3) | 0.12.3 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [hashbrown](https://crates.io/crates/hashbrown/0.17.1) | 0.17.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [heck](https://crates.io/crates/heck/0.5.0) | 0.5.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [hex](https://crates.io/crates/hex/0.4.3) | 0.4.3 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [html5ever](https://crates.io/crates/html5ever/0.38.0) | 0.38.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [http](https://crates.io/crates/http/1.5.0) | 1.5.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [http-body](https://crates.io/crates/http-body/1.1.0) | 1.1.0 | MIT | `LICENSE` |
| [http-body-util](https://crates.io/crates/http-body-util/0.1.5) | 0.1.5 | MIT | `LICENSE` |
| [httparse](https://crates.io/crates/httparse/1.10.1) | 1.10.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [hyper](https://crates.io/crates/hyper/1.11.1) | 1.11.1 | MIT | `LICENSE` |
| [hyper-rustls](https://crates.io/crates/hyper-rustls/0.27.9) | 0.27.9 | Apache-2.0 OR ISC OR MIT | `LICENSE-APACHE`、`LICENSE-ISC`、`LICENSE-MIT` |
| [hyper-util](https://crates.io/crates/hyper-util/0.1.20) | 0.1.20 | MIT | `LICENSE` |
| [ico](https://crates.io/crates/ico/0.5.0) | 0.5.0 | MIT | `LICENSE` |
| [icu_collections](https://crates.io/crates/icu_collections/2.3.0) | 2.3.0 | Unicode-3.0 | `LICENSE` |
| [icu_locale_core](https://crates.io/crates/icu_locale_core/2.3.0) | 2.3.0 | Unicode-3.0 | `LICENSE` |
| [icu_normalizer](https://crates.io/crates/icu_normalizer/2.3.0) | 2.3.0 | Unicode-3.0 | `LICENSE` |
| [icu_normalizer_data](https://crates.io/crates/icu_normalizer_data/2.3.0) | 2.3.0 | Unicode-3.0 | `LICENSE` |
| [icu_properties](https://crates.io/crates/icu_properties/2.3.0) | 2.3.0 | Unicode-3.0 | `LICENSE` |
| [icu_properties_data](https://crates.io/crates/icu_properties_data/2.3.0) | 2.3.0 | Unicode-3.0 | `LICENSE` |
| [icu_provider](https://crates.io/crates/icu_provider/2.3.1) | 2.3.1 | Unicode-3.0 | `LICENSE` |
| [ident_case](https://crates.io/crates/ident_case/1.0.1) | 1.0.1 | MIT/Apache-2.0 | `LICENSE` |
| [idna](https://crates.io/crates/idna/1.1.0) | 1.1.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [idna_adapter](https://crates.io/crates/idna_adapter/1.2.2) | 1.2.2 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [indexmap](https://crates.io/crates/indexmap/1.9.3) | 1.9.3 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [indexmap](https://crates.io/crates/indexmap/2.14.1) | 2.14.1 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [infer](https://crates.io/crates/infer/0.19.0) | 0.19.0 | MIT | `LICENSE` |
| [ipnet](https://crates.io/crates/ipnet/2.12.1) | 2.12.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [itoa](https://crates.io/crates/itoa/1.0.18) | 1.0.18 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [jiff](https://crates.io/crates/jiff/0.2.35) | 0.2.35 | Unlicense OR MIT | `COPYING`、`LICENSE-MIT`、`UNLICENSE` |
| [jiff-core](https://crates.io/crates/jiff-core/0.1.0) | 0.1.0 | Unlicense OR MIT | `COPYING`、`LICENSE-MIT`、`UNLICENSE` |
| [jiff-tzdb](https://crates.io/crates/jiff-tzdb/0.1.8) | 0.1.8 | Unlicense OR MIT | `COPYING`、`LICENSE-MIT`、`UNLICENSE` |
| [jiff-tzdb-platform](https://crates.io/crates/jiff-tzdb-platform/0.1.3) | 0.1.3 | Unlicense OR MIT | `COPYING`、`LICENSE-MIT`、`UNLICENSE` |
| [jobserver](https://crates.io/crates/jobserver/0.1.35) | 0.1.35 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [json-patch](https://crates.io/crates/json-patch/3.0.1) | 3.0.1 | MIT/Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [jsonptr](https://crates.io/crates/jsonptr/0.6.3) | 0.6.3 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [keyboard-types](https://crates.io/crates/keyboard-types/0.7.0) | 0.7.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [lazy_static](https://crates.io/crates/lazy_static/1.5.0) | 1.5.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [libc](https://crates.io/crates/libc/0.2.189) | 0.2.189 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [litemap](https://crates.io/crates/litemap/0.8.3) | 0.8.3 | Unicode-3.0 | `LICENSE` |
| [lock_api](https://crates.io/crates/lock_api/0.4.14) | 0.4.14 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [log](https://crates.io/crates/log/0.4.34) | 0.4.34 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [lru-slab](https://crates.io/crates/lru-slab/0.1.2) | 0.1.2 | MIT OR Apache-2.0 OR Zlib | `LICENSE-APACHE`、`LICENSE-MIT`、`LICENSE-ZLIB` |
| [markup5ever](https://crates.io/crates/markup5ever/0.38.0) | 0.38.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [matchers](https://crates.io/crates/matchers/0.2.0) | 0.2.0 | MIT | `LICENSE` |
| [memchr](https://crates.io/crates/memchr/2.8.3) | 2.8.3 | Unlicense OR MIT | `COPYING`、`LICENSE-MIT`、`UNLICENSE` |
| [mime](https://crates.io/crates/mime/0.3.17) | 0.3.17 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [miniz_oxide](https://crates.io/crates/miniz_oxide/0.8.9) | 0.8.9 | MIT OR Zlib OR Apache-2.0 | `LICENSE`、`LICENSE-APACHE.md`、`LICENSE-MIT.md`、`LICENSE-ZLIB.md` |
| [miniz_oxide](https://crates.io/crates/miniz_oxide/0.9.1) | 0.9.1 | MIT OR Zlib OR Apache-2.0 | `LICENSE`、`LICENSE-APACHE.md`、`LICENSE-MIT.md`、`LICENSE-ZLIB.md` |
| [mio](https://crates.io/crates/mio/1.2.2) | 1.2.2 | MIT | `LICENSE` |
| [muda](https://crates.io/crates/muda/0.19.3) | 0.19.3 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT`、`LICENSE.spdx` |
| [new_debug_unreachable](https://crates.io/crates/new_debug_unreachable/1.0.6) | 1.0.6 | MIT | `LICENSE-MIT` |
| [nu-ansi-term](https://crates.io/crates/nu-ansi-term/0.50.3) | 0.50.3 | MIT | `LICENSE` |
| [num-conv](https://crates.io/crates/num-conv/0.2.2) | 0.2.2 | MIT OR Apache-2.0 | `LICENSE-Apache`、`LICENSE-MIT` |
| [num-traits](https://crates.io/crates/num-traits/0.2.19) | 0.2.19 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [once_cell](https://crates.io/crates/once_cell/1.21.4) | 1.21.4 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [open](https://crates.io/crates/open/5.4.2) | 5.4.2 | MIT | `LICENSE.md` |
| [option-ext](https://crates.io/crates/option-ext/0.2.0) | 0.2.0 | MPL-2.0 | `LICENSE.txt` |
| [parking_lot](https://crates.io/crates/parking_lot/0.12.5) | 0.12.5 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [parking_lot_core](https://crates.io/crates/parking_lot_core/0.9.12) | 0.9.12 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [pastey](https://crates.io/crates/pastey/0.2.3) | 0.2.3 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [percent-encoding](https://crates.io/crates/percent-encoding/2.3.2) | 2.3.2 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [phf](https://crates.io/crates/phf/0.13.1) | 0.13.1 | MIT | `LICENSE` |
| [phf_codegen](https://crates.io/crates/phf_codegen/0.13.1) | 0.13.1 | MIT | `LICENSE` |
| [phf_generator](https://crates.io/crates/phf_generator/0.13.1) | 0.13.1 | MIT | `LICENSE` |
| [phf_macros](https://crates.io/crates/phf_macros/0.13.1) | 0.13.1 | MIT | `LICENSE` |
| [phf_shared](https://crates.io/crates/phf_shared/0.13.1) | 0.13.1 | MIT | `LICENSE` |
| [pin-project-lite](https://crates.io/crates/pin-project-lite/0.2.17) | 0.2.17 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [plist](https://crates.io/crates/plist/1.10.0) | 1.10.0 | MIT | `LICENCE` |
| [png](https://crates.io/crates/png/0.17.16) | 0.17.16 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [potential_utf](https://crates.io/crates/potential_utf/0.1.6) | 0.1.6 | Unicode-3.0 | `LICENSE` |
| [powerfmt](https://crates.io/crates/powerfmt/0.2.0) | 0.2.0 | MIT OR Apache-2.0 | `LICENSE-Apache`、`LICENSE-MIT` |
| [precomputed-hash](https://crates.io/crates/precomputed-hash/0.1.1) | 0.1.1 | MIT | `LICENSE` |
| [proc-macro2](https://crates.io/crates/proc-macro2/1.0.107) | 1.0.107 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [quick-xml](https://crates.io/crates/quick-xml/0.41.0) | 0.41.0 | MIT | `LICENSE-MIT.md` |
| [quinn](https://crates.io/crates/quinn/0.11.11) | 0.11.11 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [quinn-proto](https://crates.io/crates/quinn-proto/0.11.17) | 0.11.17 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [quinn-udp](https://crates.io/crates/quinn-udp/0.5.15) | 0.5.15 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [quote](https://crates.io/crates/quote/1.0.47) | 1.0.47 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [rand](https://crates.io/crates/rand/0.10.2) | 0.10.2 | MIT OR Apache-2.0 | `COPYRIGHT`、`LICENSE-APACHE`、`LICENSE-MIT` |
| [rand_core](https://crates.io/crates/rand_core/0.10.1) | 0.10.1 | MIT OR Apache-2.0 | `COPYRIGHT`、`LICENSE-APACHE`、`LICENSE-MIT` |
| [rand_pcg](https://crates.io/crates/rand_pcg/0.10.2) | 0.10.2 | MIT OR Apache-2.0 | `COPYRIGHT`、`LICENSE-APACHE`、`LICENSE-MIT` |
| [raw-window-handle](https://crates.io/crates/raw-window-handle/0.6.2) | 0.6.2 | MIT OR Apache-2.0 OR Zlib | `LICENSE-APACHE.md`、`LICENSE-MIT.md`、`LICENSE-ZLIB.md` |
| [ref-cast](https://crates.io/crates/ref-cast/1.0.27) | 1.0.27 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [ref-cast-impl](https://crates.io/crates/ref-cast-impl/1.0.27) | 1.0.27 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [regex](https://crates.io/crates/regex/1.13.1) | 1.13.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [regex-automata](https://crates.io/crates/regex-automata/0.4.18) | 0.4.18 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [regex-syntax](https://crates.io/crates/regex-syntax/0.8.11) | 0.8.11 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [reqwest](https://crates.io/crates/reqwest/0.13.4) | 0.13.4 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [ring](https://crates.io/crates/ring/0.17.14) | 0.17.14 | Apache-2.0 AND ISC | `LICENSE`、`LICENSE-BoringSSL`、`LICENSE-other-bits` |
| [rmcp](https://crates.io/crates/rmcp/3.1.4) | 3.1.4 | Apache-2.0 | `上游补充：rust-sdk-4a738b9d-license.txt` |
| [rmcp-macros](https://crates.io/crates/rmcp-macros/3.1.4) | 3.1.4 | Apache-2.0 | `上游补充：rust-sdk-4a738b9d-license.txt` |
| [rustc_version](https://crates.io/crates/rustc_version/0.4.1) | 0.4.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [rustc-hash](https://crates.io/crates/rustc-hash/2.1.3) | 2.1.3 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [rustls](https://crates.io/crates/rustls/0.23.45) | 0.23.45 | Apache-2.0 OR ISC OR MIT | `LICENSE-APACHE`、`LICENSE-ISC`、`LICENSE-MIT` |
| [rustls-pki-types](https://crates.io/crates/rustls-pki-types/1.15.1) | 1.15.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [rustls-platform-verifier](https://crates.io/crates/rustls-platform-verifier/0.7.0) | 0.7.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [rustls-webpki](https://crates.io/crates/rustls-webpki/0.103.15) | 0.103.15 | ISC | `LICENSE` |
| [same-file](https://crates.io/crates/same-file/1.0.6) | 1.0.6 | Unlicense/MIT | `COPYING`、`LICENSE-MIT`、`UNLICENSE` |
| [schemars](https://crates.io/crates/schemars/0.8.22) | 0.8.22 | MIT | `LICENSE` |
| [schemars](https://crates.io/crates/schemars/0.9.0) | 0.9.0 | MIT | `LICENSE` |
| [schemars](https://crates.io/crates/schemars/1.2.2) | 1.2.2 | MIT | `LICENSE` |
| [schemars_derive](https://crates.io/crates/schemars_derive/0.8.22) | 0.8.22 | MIT | `LICENSE` |
| [schemars_derive](https://crates.io/crates/schemars_derive/1.2.2) | 1.2.2 | MIT | `LICENSE` |
| [scopeguard](https://crates.io/crates/scopeguard/1.2.0) | 1.2.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [selectors](https://crates.io/crates/selectors/0.36.1) | 0.36.1 | MPL-2.0 | `上游补充：mozilla-mpl-2.0.txt` |
| [semver](https://crates.io/crates/semver/1.0.28) | 1.0.28 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [serde](https://crates.io/crates/serde/1.0.229) | 1.0.229 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [serde_core](https://crates.io/crates/serde_core/1.0.229) | 1.0.229 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [serde_derive](https://crates.io/crates/serde_derive/1.0.229) | 1.0.229 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [serde_derive_internals](https://crates.io/crates/serde_derive_internals/0.29.1) | 0.29.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [serde_derive_internals](https://crates.io/crates/serde_derive_internals/0.30.0) | 0.30.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [serde_json](https://crates.io/crates/serde_json/1.0.151) | 1.0.151 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [serde_repr](https://crates.io/crates/serde_repr/0.1.21) | 0.1.21 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [serde_spanned](https://crates.io/crates/serde_spanned/1.1.1) | 1.1.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [serde_with](https://crates.io/crates/serde_with/3.22.0) | 3.22.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [serde_with_macros](https://crates.io/crates/serde_with_macros/3.22.0) | 3.22.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [serde-untagged](https://crates.io/crates/serde-untagged/0.1.9) | 0.1.9 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [serialize-to-javascript](https://crates.io/crates/serialize-to-javascript/0.1.2) | 0.1.2 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [serialize-to-javascript-impl](https://crates.io/crates/serialize-to-javascript-impl/0.1.2) | 0.1.2 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [servo_arc](https://crates.io/crates/servo_arc/0.4.3) | 0.4.3 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [sha2](https://crates.io/crates/sha2/0.10.9) | 0.10.9 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [sharded-slab](https://crates.io/crates/sharded-slab/0.1.7) | 0.1.7 | MIT | `LICENSE` |
| [shlex](https://crates.io/crates/shlex/2.0.1) | 2.0.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [simd-adler32](https://crates.io/crates/simd-adler32/0.3.10) | 0.3.10 | MIT | `LICENSE.md` |
| [siphasher](https://crates.io/crates/siphasher/1.0.3) | 1.0.3 | MIT/Apache-2.0 | `COPYING` |
| [slab](https://crates.io/crates/slab/0.4.12) | 0.4.12 | MIT | `LICENSE` |
| [smallvec](https://crates.io/crates/smallvec/1.15.2) | 1.15.2 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [socket2](https://crates.io/crates/socket2/0.6.5) | 0.6.5 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [softbuffer](https://crates.io/crates/softbuffer/0.4.8) | 0.4.8 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [sse-stream](https://crates.io/crates/sse-stream/0.2.5) | 0.2.5 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [stable_deref_trait](https://crates.io/crates/stable_deref_trait/1.2.1) | 1.2.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [string_cache](https://crates.io/crates/string_cache/0.9.0) | 0.9.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [string_cache_codegen](https://crates.io/crates/string_cache_codegen/0.6.1) | 0.6.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [strsim](https://crates.io/crates/strsim/0.11.1) | 0.11.1 | MIT | `LICENSE` |
| [subtle](https://crates.io/crates/subtle/2.6.1) | 2.6.1 | BSD-3-Clause | `LICENSE` |
| [symlink](https://crates.io/crates/symlink/0.1.0) | 0.1.0 | MIT/Apache-2.0 | `COPYRIGHT`、`LICENSE-APACHE`、`LICENSE-MIT` |
| [syn](https://crates.io/crates/syn/2.0.119) | 2.0.119 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [syn](https://crates.io/crates/syn/3.0.4) | 3.0.4 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [sync_wrapper](https://crates.io/crates/sync_wrapper/1.0.2) | 1.0.2 | Apache-2.0 | `LICENSE` |
| [synstructure](https://crates.io/crates/synstructure/0.13.2) | 0.13.2 | MIT | `LICENSE` |
| [tao](https://crates.io/crates/tao/0.35.3) | 0.35.3 | Apache-2.0 | `LICENSE`、`LICENSE.spdx` |
| [tauri](https://crates.io/crates/tauri/2.11.5) | 2.11.5 | Apache-2.0 OR MIT | `LICENSE_APACHE-2.0`、`LICENSE_MIT` |
| [tauri-build](https://crates.io/crates/tauri-build/2.6.3) | 2.6.3 | Apache-2.0 OR MIT | `LICENSE_APACHE-2.0`、`LICENSE_MIT` |
| [tauri-codegen](https://crates.io/crates/tauri-codegen/2.6.3) | 2.6.3 | Apache-2.0 OR MIT | `LICENSE_APACHE-2.0`、`LICENSE_MIT` |
| [tauri-macros](https://crates.io/crates/tauri-macros/2.6.3) | 2.6.3 | Apache-2.0 OR MIT | `LICENSE_APACHE-2.0`、`LICENSE_MIT` |
| [tauri-plugin](https://crates.io/crates/tauri-plugin/2.6.3) | 2.6.3 | Apache-2.0 OR MIT | `上游补充：tauri-6f6ab120-license.spdx.txt`、`上游补充：tauri-6f6ab120-license-apache-2.0.txt`、`上游补充：tauri-6f6ab120-license-mit.txt` |
| [tauri-plugin-autostart](https://crates.io/crates/tauri-plugin-autostart/2.5.1) | 2.5.1 | Apache-2.0 OR MIT | `LICENSE.spdx`、`LICENSE_APACHE-2.0`、`LICENSE_MIT` |
| [tauri-plugin-global-shortcut](https://crates.io/crates/tauri-plugin-global-shortcut/2.3.2) | 2.3.2 | Apache-2.0 OR MIT | `LICENSE.spdx`、`LICENSE_APACHE-2.0`、`LICENSE_MIT` |
| [tauri-plugin-opener](https://crates.io/crates/tauri-plugin-opener/2.5.4) | 2.5.4 | Apache-2.0 OR MIT | `LICENSE.spdx`、`LICENSE_APACHE-2.0`、`LICENSE_MIT` |
| [tauri-plugin-single-instance](https://crates.io/crates/tauri-plugin-single-instance/2.4.3) | 2.4.3 | Apache-2.0 OR MIT | `LICENSE.spdx`、`LICENSE_APACHE-2.0`、`LICENSE_MIT` |
| [tauri-runtime](https://crates.io/crates/tauri-runtime/2.11.3) | 2.11.3 | Apache-2.0 OR MIT | `LICENSE_APACHE-2.0`、`LICENSE_MIT` |
| [tauri-runtime-wry](https://crates.io/crates/tauri-runtime-wry/2.11.4) | 2.11.4 | Apache-2.0 OR MIT | `LICENSE_APACHE-2.0`、`LICENSE_MIT` |
| [tauri-utils](https://crates.io/crates/tauri-utils/2.9.3) | 2.9.3 | Apache-2.0 OR MIT | `LICENSE_APACHE-2.0`、`LICENSE_MIT` |
| [tauri-winres](https://crates.io/crates/tauri-winres/0.3.6) | 0.3.6 | MIT | `LICENSE` |
| [tendril](https://crates.io/crates/tendril/0.5.1) | 0.5.1 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [thiserror](https://crates.io/crates/thiserror/1.0.69) | 1.0.69 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [thiserror](https://crates.io/crates/thiserror/2.0.20) | 2.0.20 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [thiserror-impl](https://crates.io/crates/thiserror-impl/1.0.69) | 1.0.69 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [thiserror-impl](https://crates.io/crates/thiserror-impl/2.0.20) | 2.0.20 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [thread_local](https://crates.io/crates/thread_local/1.1.10) | 1.1.10 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [time](https://crates.io/crates/time/0.3.55) | 0.3.55 | MIT OR Apache-2.0 | `LICENSE-Apache`、`LICENSE-MIT` |
| [time-core](https://crates.io/crates/time-core/0.1.9) | 0.1.9 | MIT OR Apache-2.0 | `LICENSE-Apache`、`LICENSE-MIT` |
| [time-macros](https://crates.io/crates/time-macros/0.2.32) | 0.2.32 | MIT OR Apache-2.0 | `LICENSE-Apache`、`LICENSE-MIT` |
| [tinystr](https://crates.io/crates/tinystr/0.8.4) | 0.8.4 | Unicode-3.0 | `LICENSE` |
| [tinyvec](https://crates.io/crates/tinyvec/1.12.0) | 1.12.0 | Zlib OR Apache-2.0 OR MIT | `LICENSE-APACHE.md`、`LICENSE-MIT.md`、`LICENSE-ZLIB.md` |
| [tinyvec_macros](https://crates.io/crates/tinyvec_macros/0.1.1) | 0.1.1 | MIT OR Apache-2.0 OR Zlib | `LICENSE-APACHE.md`、`LICENSE-MIT.md`、`LICENSE-ZLIB.md` |
| [tokio](https://crates.io/crates/tokio/1.53.1) | 1.53.1 | MIT | `LICENSE` |
| [tokio-macros](https://crates.io/crates/tokio-macros/2.7.2) | 2.7.2 | MIT | `LICENSE` |
| [tokio-rustls](https://crates.io/crates/tokio-rustls/0.26.4) | 0.26.4 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [tokio-stream](https://crates.io/crates/tokio-stream/0.1.19) | 0.1.19 | MIT | `LICENSE` |
| [tokio-util](https://crates.io/crates/tokio-util/0.7.19) | 0.7.19 | MIT | `LICENSE` |
| [toml](https://crates.io/crates/toml/0.9.12%2Bspec-1.1.0) | 0.9.12+spec-1.1.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [toml](https://crates.io/crates/toml/1.1.4%2Bspec-1.1.0) | 1.1.4+spec-1.1.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [toml_datetime](https://crates.io/crates/toml_datetime/0.7.5%2Bspec-1.1.0) | 0.7.5+spec-1.1.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [toml_datetime](https://crates.io/crates/toml_datetime/1.1.1%2Bspec-1.1.0) | 1.1.1+spec-1.1.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [toml_parser](https://crates.io/crates/toml_parser/1.1.3%2Bspec-1.1.0) | 1.1.3+spec-1.1.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [toml_writer](https://crates.io/crates/toml_writer/1.1.2%2Bspec-1.1.0) | 1.1.2+spec-1.1.0 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [tower](https://crates.io/crates/tower/0.5.3) | 0.5.3 | MIT | `LICENSE` |
| [tower-http](https://crates.io/crates/tower-http/0.6.11) | 0.6.11 | MIT | `LICENSE` |
| [tower-layer](https://crates.io/crates/tower-layer/0.3.3) | 0.3.3 | MIT | `LICENSE` |
| [tower-service](https://crates.io/crates/tower-service/0.3.3) | 0.3.3 | MIT | `LICENSE` |
| [tracing](https://crates.io/crates/tracing/0.1.44) | 0.1.44 | MIT | `LICENSE` |
| [tracing-appender](https://crates.io/crates/tracing-appender/0.2.5) | 0.2.5 | MIT | `LICENSE` |
| [tracing-attributes](https://crates.io/crates/tracing-attributes/0.1.31) | 0.1.31 | MIT | `LICENSE` |
| [tracing-core](https://crates.io/crates/tracing-core/0.1.36) | 0.1.36 | MIT | `LICENSE` |
| [tracing-log](https://crates.io/crates/tracing-log/0.2.0) | 0.2.0 | MIT | `LICENSE` |
| [tracing-subscriber](https://crates.io/crates/tracing-subscriber/0.3.23) | 0.3.23 | MIT | `LICENSE` |
| [tray-icon](https://crates.io/crates/tray-icon/0.24.2) | 0.24.2 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT`、`LICENSE.spdx` |
| [try-lock](https://crates.io/crates/try-lock/0.2.5) | 0.2.5 | MIT | `LICENSE` |
| [typeid](https://crates.io/crates/typeid/1.0.3) | 1.0.3 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [typenum](https://crates.io/crates/typenum/1.20.1) | 1.20.1 | MIT OR Apache-2.0 | `LICENSE`、`LICENSE-APACHE`、`LICENSE-MIT` |
| [unic-char-property](https://crates.io/crates/unic-char-property/0.9.0) | 0.9.0 | MIT/Apache-2.0 | `上游补充：rust-unic-58786053-copyright.md.txt`、`上游补充：rust-unic-58786053-license-apache.txt`、`上游补充：rust-unic-58786053-license-mit.txt` |
| [unic-char-range](https://crates.io/crates/unic-char-range/0.9.0) | 0.9.0 | MIT/Apache-2.0 | `上游补充：rust-unic-58786053-copyright.md.txt`、`上游补充：rust-unic-58786053-license-apache.txt`、`上游补充：rust-unic-58786053-license-mit.txt` |
| [unic-common](https://crates.io/crates/unic-common/0.9.0) | 0.9.0 | MIT/Apache-2.0 | `上游补充：rust-unic-58786053-copyright.md.txt`、`上游补充：rust-unic-58786053-license-apache.txt`、`上游补充：rust-unic-58786053-license-mit.txt` |
| [unic-ucd-ident](https://crates.io/crates/unic-ucd-ident/0.9.0) | 0.9.0 | MIT/Apache-2.0 | `上游补充：rust-unic-8a6ce830-copyright.md.txt`、`上游补充：rust-unic-8a6ce830-license-apache.txt`、`上游补充：rust-unic-8a6ce830-license-mit.txt` |
| [unic-ucd-version](https://crates.io/crates/unic-ucd-version/0.9.0) | 0.9.0 | MIT/Apache-2.0 | `上游补充：rust-unic-58786053-copyright.md.txt`、`上游补充：rust-unic-58786053-license-apache.txt`、`上游补充：rust-unic-58786053-license-mit.txt` |
| [unicode-ident](https://crates.io/crates/unicode-ident/1.0.24) | 1.0.24 | (MIT OR Apache-2.0) AND Unicode-3.0 | `LICENSE-APACHE`、`LICENSE-MIT`、`LICENSE-UNICODE` |
| [unicode-segmentation](https://crates.io/crates/unicode-segmentation/1.13.3) | 1.13.3 | MIT OR Apache-2.0 | `COPYRIGHT`、`LICENSE-APACHE`、`LICENSE-MIT` |
| [untrusted](https://crates.io/crates/untrusted/0.9.0) | 0.9.0 | ISC | `LICENSE.txt` |
| [url](https://crates.io/crates/url/2.5.8) | 2.5.8 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [urlpattern](https://crates.io/crates/urlpattern/0.3.0) | 0.3.0 | MIT | `LICENSE` |
| [utf8_iter](https://crates.io/crates/utf8_iter/1.0.4) | 1.0.4 | Apache-2.0 OR MIT | `COPYRIGHT`、`LICENSE-APACHE`、`LICENSE-MIT` |
| [uuid](https://crates.io/crates/uuid/1.26.0) | 1.26.0 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [version_check](https://crates.io/crates/version_check/0.9.5) | 0.9.5 | MIT/Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [vswhom](https://crates.io/crates/vswhom/0.1.0) | 0.1.0 | MIT | `LICENSE` |
| [vswhom-sys](https://crates.io/crates/vswhom-sys/0.1.3) | 0.1.3 | MIT | `LICENSE` |
| [walkdir](https://crates.io/crates/walkdir/2.5.0) | 2.5.0 | Unlicense/MIT | `COPYING`、`LICENSE-MIT`、`UNLICENSE` |
| [want](https://crates.io/crates/want/0.3.1) | 0.3.1 | MIT | `LICENSE` |
| [web_atoms](https://crates.io/crates/web_atoms/0.2.6) | 0.2.6 | MIT OR Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [webview2-com](https://crates.io/crates/webview2-com/0.38.2) | 0.38.2 | MIT | `上游补充：webview2-rs-b74dc5e2-license.txt` |
| [webview2-com-macros](https://crates.io/crates/webview2-com-macros/0.8.1) | 0.8.1 | MIT | `上游补充：webview2-rs-dffa41a8-license.txt` |
| [webview2-com-sys](https://crates.io/crates/webview2-com-sys/0.38.2) | 0.38.2 | MIT | `上游补充：webview2-rs-b74dc5e2-license.txt` |
| [winapi](https://crates.io/crates/winapi/0.3.9) | 0.3.9 | MIT/Apache-2.0 | `LICENSE-APACHE`、`LICENSE-MIT` |
| [winapi-util](https://crates.io/crates/winapi-util/0.1.11) | 0.1.11 | Unlicense OR MIT | `COPYING`、`LICENSE-MIT`、`UNLICENSE` |
| [window-vibrancy](https://crates.io/crates/window-vibrancy/0.6.0) | 0.6.0 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT`、`LICENSE.spdx` |
| [windows](https://crates.io/crates/windows/0.61.3) | 0.61.3 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows_x86_64_msvc](https://crates.io/crates/windows_x86_64_msvc/0.52.6) | 0.52.6 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows_x86_64_msvc](https://crates.io/crates/windows_x86_64_msvc/0.53.1) | 0.53.1 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-collections](https://crates.io/crates/windows-collections/0.2.0) | 0.2.0 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-core](https://crates.io/crates/windows-core/0.61.2) | 0.61.2 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-future](https://crates.io/crates/windows-future/0.2.1) | 0.2.1 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-implement](https://crates.io/crates/windows-implement/0.60.2) | 0.60.2 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-interface](https://crates.io/crates/windows-interface/0.59.3) | 0.59.3 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-link](https://crates.io/crates/windows-link/0.1.3) | 0.1.3 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-link](https://crates.io/crates/windows-link/0.2.1) | 0.2.1 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-numerics](https://crates.io/crates/windows-numerics/0.2.0) | 0.2.0 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-registry](https://crates.io/crates/windows-registry/0.6.1) | 0.6.1 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-result](https://crates.io/crates/windows-result/0.3.4) | 0.3.4 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-result](https://crates.io/crates/windows-result/0.4.1) | 0.4.1 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-strings](https://crates.io/crates/windows-strings/0.4.2) | 0.4.2 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-strings](https://crates.io/crates/windows-strings/0.5.1) | 0.5.1 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-sys](https://crates.io/crates/windows-sys/0.59.0) | 0.59.0 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-sys](https://crates.io/crates/windows-sys/0.60.2) | 0.60.2 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-sys](https://crates.io/crates/windows-sys/0.61.2) | 0.61.2 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-targets](https://crates.io/crates/windows-targets/0.52.6) | 0.52.6 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-targets](https://crates.io/crates/windows-targets/0.53.5) | 0.53.5 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-threading](https://crates.io/crates/windows-threading/0.1.0) | 0.1.0 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [windows-version](https://crates.io/crates/windows-version/0.1.7) | 0.1.7 | MIT OR Apache-2.0 | `license-apache-2.0`、`license-mit` |
| [winnow](https://crates.io/crates/winnow/0.7.15) | 0.7.15 | MIT | `LICENSE-MIT` |
| [winnow](https://crates.io/crates/winnow/1.0.4) | 1.0.4 | MIT | `LICENSE-MIT` |
| [winreg](https://crates.io/crates/winreg/0.10.1) | 0.10.1 | MIT | `LICENSE` |
| [winreg](https://crates.io/crates/winreg/0.55.0) | 0.55.0 | MIT | `LICENSE` |
| [writeable](https://crates.io/crates/writeable/0.6.4) | 0.6.4 | Unicode-3.0 | `LICENSE` |
| [wry](https://crates.io/crates/wry/0.55.1) | 0.55.1 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT`、`LICENSE.spdx` |
| [yoke](https://crates.io/crates/yoke/0.8.3) | 0.8.3 | Unicode-3.0 | `LICENSE` |
| [yoke-derive](https://crates.io/crates/yoke-derive/0.8.2) | 0.8.2 | Unicode-3.0 | `LICENSE` |
| [zerofrom](https://crates.io/crates/zerofrom/0.1.8) | 0.1.8 | Unicode-3.0 | `LICENSE` |
| [zerofrom-derive](https://crates.io/crates/zerofrom-derive/0.1.7) | 0.1.7 | Unicode-3.0 | `LICENSE` |
| [zeroize](https://crates.io/crates/zeroize/1.9.0) | 1.9.0 | Apache-2.0 OR MIT | `LICENSE-APACHE`、`LICENSE-MIT` |
| [zerotrie](https://crates.io/crates/zerotrie/0.2.5) | 0.2.5 | Unicode-3.0 | `LICENSE` |
| [zerovec](https://crates.io/crates/zerovec/0.11.8) | 0.11.8 | Unicode-3.0 | `LICENSE` |
| [zerovec-derive](https://crates.io/crates/zerovec-derive/0.11.6) | 0.11.6 | Unicode-3.0 | `LICENSE` |
| [zlib-rs](https://crates.io/crates/zlib-rs/0.6.7) | 0.6.7 | Zlib | `LICENSE` |
| [zmij](https://crates.io/crates/zmij/1.0.23) | 1.0.23 | MIT | `LICENSE-MIT` |

## MPL 2.0 组件的源码获取

下列组件使用发布物原样依赖，本项目未修改其源码。接收二进制的用户可以从以下精确版本的 crates.io 发布物取得对应源码；许可全文见汇编，并遵循 [MPL 2.0 第 3 节](https://www.mozilla.org/en-US/MPL/2.0/#responsibilities)的适用要求。正式发布时应核对这些来源仍可获得；未来若修改依赖源码，需提供与修改后二进制对应的源码获取方式。

- [cssparser 0.36.0 原始源码发布物](https://crates.io/api/v1/crates/cssparser/0.36.0/download)
- [cssparser-macros 0.6.1 原始源码发布物](https://crates.io/api/v1/crates/cssparser-macros/0.6.1/download)
- [dtoa-short 0.3.5 原始源码发布物](https://crates.io/api/v1/crates/dtoa-short/0.3.5/download)
- [option-ext 0.2.0 原始源码发布物](https://crates.io/api/v1/crates/option-ext/0.2.0/download)
- [selectors 0.36.1 原始源码发布物](https://crates.io/api/v1/crates/selectors/0.36.1/download)

## 限制与维护

本次目标图中每个包版本均已关联至少一份包内原文、同提交上游原文或按明确声明取得的许可制定者标准文本。

自动收录范围是包声明的 license_file、包根目录许可/NOTICE，以及常见许可子目录；复杂 vendored 组件可能还有专属声明，需要根据最终分发内容人工复核。Rust 标准库、MSVC 工具链、Windows/WebView2、NSIS 和构建器自身的分发条款不在本 Cargo 项目依赖图内，仍遵循其发行者规则。本文件不证明完整法律审查已经完成。

改变锁文件、功能开关或目标平台后重跑生成器，并复核新增/变更的表达式和 NOTICE。默认全离线，缓存不足即停止，不自动下载其他平台依赖。
