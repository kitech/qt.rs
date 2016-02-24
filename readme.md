## Qt5 binding for Rust language.

## qt.rs
This project provides bindings that allow the QT Gui toolkit to be used from the Rust Programming language.

## Compiling

#### depends

    git clone https://github.com/kitech/qt.inlien.git
    mkdir qt.inline/build
    cd qt.inline/build
    cmake -DCMAKE_INSTALL_PREFIX=/usr ..
    make && make install

#### qt.rs

    git clone https://github.com/kitech/qt.rs.git
    cd qt.rs
    cargo build


## Examples

    cargo build --examle button

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


