// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmldebug.h
// dst-file: /src/qml/qqmldebug.rs
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
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlDebuggingEnabler_Class_Size() -> c_int;
  // proto:  void QQmlDebuggingEnabler::QQmlDebuggingEnabler(bool printWarning);
  fn _ZN20QQmlDebuggingEnablerC2Eb(qthis: u64 /* *mut c_void*/, arg0: c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlDebuggingEnabler)=1
#[derive(Default)]
pub struct QQmlDebuggingEnabler {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlDebuggingEnabler {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlDebuggingEnabler {
    return QQmlDebuggingEnabler{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QQmlDebuggingEnabler::QQmlDebuggingEnabler(bool printWarning);
impl /*struct*/ QQmlDebuggingEnabler {
  pub fn new<T: QQmlDebuggingEnabler_new>(value: T) -> QQmlDebuggingEnabler {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlDebuggingEnabler_new {
  fn new(self) -> QQmlDebuggingEnabler;
}

  // proto:  void QQmlDebuggingEnabler::QQmlDebuggingEnabler(bool printWarning);
impl<'a> /*trait*/ QQmlDebuggingEnabler_new for (i8) {
  fn new(self) -> QQmlDebuggingEnabler {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QQmlDebuggingEnablerC2Eb()};
    let ctysz: c_int = unsafe{QQmlDebuggingEnabler_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_char;
    unsafe {_ZN20QQmlDebuggingEnablerC2Eb(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlDebuggingEnabler{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

