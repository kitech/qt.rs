// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qstylefactory.h
// dst-file: /src/widgets/qstylefactory.rs
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
use super::super::core::qstring::*; // 771
use super::qstyle::*; // 773
use super::super::core::qstringlist::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStyleFactory_Class_Size() -> c_int;
  // proto: static QStyle * QStyleFactory::create(const QString & );
  fn C_ZN13QStyleFactory6createERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static QStringList QStyleFactory::keys();
  fn C_ZN13QStyleFactory4keysEv() -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QStyleFactory)=1
#[derive(Default)]
pub struct QStyleFactory {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStyleFactory {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyleFactory {
    return QStyleFactory{qclsinst: qthis, ..Default::default()};
  }
}
  // proto: static QStyle * QStyleFactory::create(const QString & );
impl /*struct*/ QStyleFactory {
  pub fn create_s<RetType, T: QStyleFactory_create_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.create_s();
    // return 1;
  }
}

pub trait QStyleFactory_create_s<RetType> {
  fn create_s(self ) -> RetType;
}

  // proto: static QStyle * QStyleFactory::create(const QString & );
impl<'a> /*trait*/ QStyleFactory_create_s<QStyle> for (&'a QString) {
  fn create_s(self ) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStyleFactory6createERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QStyleFactory6createERK7QString(arg0)};
    let mut ret1 = QStyle::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QStringList QStyleFactory::keys();
impl /*struct*/ QStyleFactory {
  pub fn keys_s<RetType, T: QStyleFactory_keys_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.keys_s();
    // return 1;
  }
}

pub trait QStyleFactory_keys_s<RetType> {
  fn keys_s(self ) -> RetType;
}

  // proto: static QStringList QStyleFactory::keys();
impl<'a> /*trait*/ QStyleFactory_keys_s<QStringList> for () {
  fn keys_s(self ) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStyleFactory4keysEv()};
    let mut ret = unsafe {C_ZN13QStyleFactory4keysEv()};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

