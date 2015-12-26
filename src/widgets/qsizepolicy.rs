// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtWidgets/qsizepolicy.h
// dst-file: /src/widgets/qsizepolicy.rs
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
  fn QSizePolicy_Class_Size() -> c_int;
  // proto:  void QSizePolicy::transpose();
  fn _ZN11QSizePolicy9transposeEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSizePolicy)=4
pub struct QSizePolicy {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSizePolicy {
  pub fn inheritFrom(qthis: *mut c_void) -> QSizePolicy {
    return QSizePolicy{qclsinst: qthis};
  }
}
  // proto:  void QSizePolicy::transpose();
impl /*struct*/ QSizePolicy {
  pub fn transpose<RetType, T: QSizePolicy_transpose<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transpose(self);
    // return 1;
  }
}

pub trait QSizePolicy_transpose<RetType> {
  fn transpose(self , rsthis: & QSizePolicy) -> RetType;
}

  // proto:  void QSizePolicy::transpose();
impl<'a> /*trait*/ QSizePolicy_transpose<()> for () {
  fn transpose(self , rsthis: & QSizePolicy) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy9transposeEv()};
     unsafe {_ZN11QSizePolicy9transposeEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

