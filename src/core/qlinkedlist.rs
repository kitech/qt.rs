// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
// src-file: /QtCore/qlinkedlist.h
// dst-file: /src/core/qlinkedlist.rs
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
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
} // <= ext block end

// body block begin =>
// class sizeof(QLinkedListData)=1
pub struct QLinkedListData {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLinkedListData {
  pub fn inheritFrom(qthis: *mut c_void) -> QLinkedListData {
    return QLinkedListData{qclsinst: qthis};
  }
}
// <= body block end

