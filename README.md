# MakeMD (CLI)


![GitHub](https://img.shields.io/github/license/mashape/apistatus.svg) [![Build Status](https://travis-ci.org/Eonm/makemd-rs.svg?branch=master)](https://travis-ci.org/Eonm/makemd-rs)

Build and lint your academic markdown documents with Pandoc and Zotero.

## Install

MakeMD is available for linux and windows. Download the last release of MakeMD.

OSX  users have to build MakeMD by themselves.

### Dependencies

This dependencies are used to build documents

* pandoc
* pandoc-citeproc
* latex (texlive-full)

## Run

### Init a project

```sh
./makemd-rs init
```

You can tweak your project by editing the `.makemd` file. Or you can use the cross-platform MakeMD GUI.

### Build documents (pdf, presentation)

By default all your documents have to be placed in the `./md/` folder. MakeMD uses recursion to find markdown files.

**Markdon files starting with `[draft]` won't be builded.**

#### Build pdf

```sh
./makemd-rs build --pdf
```
#### Build presentation

```sh
./makemd-rs build --presentation
```

## Maintenance

### Download your bibliograpy

```sh
makemd-rs maintenance --update-bib
```

### Download your csl file

```sh
makemd-rs maintenance --update-csl
```

### Lint

_Comming soon_

## Build MakeMD

```
git clone https://github.com/Eonm/makemd-rs
cd makemd-rs
cargo build --release
```

## License

MakeMD is distributed under the MIT license.
