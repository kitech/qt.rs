## Qt5 binding for Rust language.

## qt.rs
This project provides bindings that allow the QT Gui toolkit to be used from the Rust Programming language.

## Compiling

#### depends

    git clone https://github.com/kitech/qt.inline.git
    mkdir qt.inline/build
    cd qt.inline/build
    cmake -DCMAKE_INSTALL_PREFIX=/usr ..
    make && make install

#### qt.rs

    git clone https://github.com/kitech/qt.rs.git
    cd qt.rs
    cargo build

#### build memory usage

* core 1.4G
* gui 2.7G
* widgets 5.0G

## Examples

    cargo build --example button

## Supported Qt5 modules

* Core
* Gui
* Widgets
* Network
* Qml
* Quick

## TODOS

* More Qt modules support
* Global Qt functions support
* Qt enums support
* Qt generic container classes support
* Operator methods support


## workspace struct

see wstree.txt

