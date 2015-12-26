// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qrunnable.h
// dst-file: /src/core/qrunnable.rs
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
  fn QRunnable_Class_Size() -> c_int;
  // proto:  void QRunnable::run();
  fn _ZN9QRunnable3runEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QRunnable)=16
pub struct QRunnable {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRunnable {
  pub fn inheritFrom(qthis: *mut c_void) -> QRunnable {
    return QRunnable{qclsinst: qthis};
  }
}
  // proto:  void QRunnable::run();
impl /*struct*/ QRunnable {
  pub fn run<RetType, T: QRunnable_run<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.run(self);
    // return 1;
  }
}

pub trait QRunnable_run<RetType> {
  fn run(self , rsthis: & QRunnable) -> RetType;
}

  // proto:  void QRunnable::run();
impl<'a> /*trait*/ QRunnable_run<()> for () {
  fn run(self , rsthis: & QRunnable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QRunnable3runEv()};
     unsafe {_ZN9QRunnable3runEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

