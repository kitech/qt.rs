// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlinfo.h
// dst-file: /src/qml/qqmlinfo.rs
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
use super::super::core::qdebug::QDebug; // 771
use std::ops::Deref;
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlInfo_Class_Size() -> c_int;
  // proto:  void QQmlInfo::QQmlInfo(const QQmlInfo & );
  fn _ZN8QQmlInfoC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlInfo::~QQmlInfo();
  fn _ZN8QQmlInfoD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlInfo)=16
#[derive(Default)]
pub struct QQmlInfo {
  qbase: QDebug,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlInfo {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlInfo {
    return QQmlInfo{qbase: QDebug::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQmlInfo {
  type Target = QDebug;

  fn deref(&self) -> &QDebug {
    return & self.qbase;
  }
}
impl AsRef<QDebug> for QQmlInfo {
  fn as_ref(& self) -> & QDebug {
    return & self.qbase;
  }
}
  // proto:  void QQmlInfo::QQmlInfo(const QQmlInfo & );
impl /*struct*/ QQmlInfo {
  pub fn new<T: QQmlInfo_new>(value: T) -> QQmlInfo {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlInfo_new {
  fn new(self) -> QQmlInfo;
}

  // proto:  void QQmlInfo::QQmlInfo(const QQmlInfo & );
impl<'a> /*trait*/ QQmlInfo_new for (&'a QQmlInfo) {
  fn new(self) -> QQmlInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlInfoC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlInfo_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QQmlInfoC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlInfo{qbase: QDebug::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlInfo::~QQmlInfo();
impl /*struct*/ QQmlInfo {
  pub fn free<RetType, T: QQmlInfo_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlInfo_free<RetType> {
  fn free(self , rsthis: & QQmlInfo) -> RetType;
}

  // proto:  void QQmlInfo::~QQmlInfo();
impl<'a> /*trait*/ QQmlInfo_free<()> for () {
  fn free(self , rsthis: & QQmlInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QQmlInfoD2Ev()};
     unsafe {_ZN8QQmlInfoD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

