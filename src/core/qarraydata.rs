// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qarraydata.h
// dst-file: /src/core/qarraydata.rs
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
  fn QArrayData_Class_Size() -> c_int;
  // proto: static void QArrayData::deallocate(QArrayData * data, int objectSize, int alignment);
  fn _ZN10QArrayData10deallocateEPS_ii(arg0: *mut c_void, arg1: c_int, arg2: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QArrayData)=1
pub struct QArrayData {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QArrayData {
  pub fn inheritFrom(qthis: *mut c_void) -> QArrayData {
    return QArrayData{qclsinst: qthis};
  }
}
  // proto: static void QArrayData::deallocate(QArrayData * data, int objectSize, int alignment);
impl /*struct*/ QArrayData {
  pub fn deallocate_s<RetType, T: QArrayData_deallocate_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.deallocate_s();
    // return 1;
  }
}

pub trait QArrayData_deallocate_s<RetType> {
  fn deallocate_s(self ) -> RetType;
}

  // proto: static void QArrayData::deallocate(QArrayData * data, int objectSize, int alignment);
impl<'a> /*trait*/ QArrayData_deallocate_s<()> for (&'a QArrayData, i32, i32) {
  fn deallocate_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QArrayData10deallocateEPS_ii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN10QArrayData10deallocateEPS_ii(arg0, arg1, arg2)};
    // return 1;
  }
}

// <= body block end

