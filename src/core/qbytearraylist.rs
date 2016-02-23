// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qbytearraylist.h
// dst-file: /src/core/qbytearraylist.rs
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
use super::qbytearray::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QListSpecialMethodsLQByteArrayG_Class_Size() -> c_int;
  // proto:  QByteArray QListSpecialMethods<QByteArray>::join(char sep);
  fn C_ZNK19QListSpecialMethodsI10QByteArrayE4joinEc(qthis: u64 /* *mut c_void*/, arg0: c_char) -> *mut c_void;
  // proto:  QByteArray QListSpecialMethods<QByteArray>::join();
  fn C_ZNK19QListSpecialMethodsI10QByteArrayE4joinEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QByteArray QListSpecialMethods<QByteArray>::join(const QByteArray & sep);
  fn C_ZNK19QListSpecialMethodsI10QByteArrayE4joinERKS0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QListSpecialMethodsLQByteArrayG)=1
#[derive(Default)]
pub struct QListSpecialMethodsLQByteArrayG {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QListSpecialMethodsLQByteArrayG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QListSpecialMethodsLQByteArrayG {
    return QListSpecialMethodsLQByteArrayG{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QByteArray QListSpecialMethods<QByteArray>::join(char sep);
impl /*struct*/ QListSpecialMethodsLQByteArrayG {
  pub fn join<RetType, T: QListSpecialMethodsLQByteArrayG_join<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.join(self);
    // return 1;
  }
}

pub trait QListSpecialMethodsLQByteArrayG_join<RetType> {
  fn join(self , rsthis: & QListSpecialMethodsLQByteArrayG) -> RetType;
}

  // proto:  QByteArray QListSpecialMethods<QByteArray>::join(char sep);
impl<'a> /*trait*/ QListSpecialMethodsLQByteArrayG_join<QByteArray> for (i8) {
  fn join(self , rsthis: & QListSpecialMethodsLQByteArrayG) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QListSpecialMethodsI10QByteArrayE4joinEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {C_ZNK19QListSpecialMethodsI10QByteArrayE4joinEc(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QListSpecialMethods<QByteArray>::join();
impl<'a> /*trait*/ QListSpecialMethodsLQByteArrayG_join<QByteArray> for () {
  fn join(self , rsthis: & QListSpecialMethodsLQByteArrayG) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QListSpecialMethodsI10QByteArrayE4joinEv()};
    let mut ret = unsafe {C_ZNK19QListSpecialMethodsI10QByteArrayE4joinEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QListSpecialMethods<QByteArray>::join(const QByteArray & sep);
impl<'a> /*trait*/ QListSpecialMethodsLQByteArrayG_join<QByteArray> for (&'a QByteArray) {
  fn join(self , rsthis: & QListSpecialMethodsLQByteArrayG) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QListSpecialMethodsI10QByteArrayE4joinERKS0_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QListSpecialMethodsI10QByteArrayE4joinERKS0_(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

