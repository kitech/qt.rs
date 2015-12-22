// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtWidgets/qdrawutil.h
// dst-file: /src/widgets/qdrawutil.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
} // <= ext block end

// body block begin =>
// class sizeof(QTileRules)=8
pub struct QTileRules {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTileRules {
  pub fn inheritFrom(qthis: *mut c_void) -> QTileRules {
    return QTileRules{qclsinst: qthis};
  }
}
// <= body block end

