// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlpropertyvaluesource.h
// dst-file: /src/qml/qqmlpropertyvaluesource.rs
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
use super::qqmlproperty::QQmlProperty; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlPropertyValueSource_Class_Size() -> c_int;
  // proto:  void QQmlPropertyValueSource::~QQmlPropertyValueSource();
  fn _ZN23QQmlPropertyValueSourceD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlPropertyValueSource::setTarget(const QQmlProperty & );
  fn _ZN23QQmlPropertyValueSource9setTargetERK12QQmlProperty(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlPropertyValueSource::QQmlPropertyValueSource();
  fn _ZN23QQmlPropertyValueSourceC2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlPropertyValueSource)=8
#[derive(Default)]
pub struct QQmlPropertyValueSource {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlPropertyValueSource {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlPropertyValueSource {
    return QQmlPropertyValueSource{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QQmlPropertyValueSource::~QQmlPropertyValueSource();
impl /*struct*/ QQmlPropertyValueSource {
  pub fn free<RetType, T: QQmlPropertyValueSource_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlPropertyValueSource_free<RetType> {
  fn free(self , rsthis: & QQmlPropertyValueSource) -> RetType;
}

  // proto:  void QQmlPropertyValueSource::~QQmlPropertyValueSource();
impl<'a> /*trait*/ QQmlPropertyValueSource_free<()> for () {
  fn free(self , rsthis: & QQmlPropertyValueSource) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QQmlPropertyValueSourceD2Ev()};
     unsafe {_ZN23QQmlPropertyValueSourceD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlPropertyValueSource::setTarget(const QQmlProperty & );
impl /*struct*/ QQmlPropertyValueSource {
  pub fn setTarget<RetType, T: QQmlPropertyValueSource_setTarget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTarget(self);
    // return 1;
  }
}

pub trait QQmlPropertyValueSource_setTarget<RetType> {
  fn setTarget(self , rsthis: & QQmlPropertyValueSource) -> RetType;
}

  // proto:  void QQmlPropertyValueSource::setTarget(const QQmlProperty & );
impl<'a> /*trait*/ QQmlPropertyValueSource_setTarget<()> for (&'a QQmlProperty) {
  fn setTarget(self , rsthis: & QQmlPropertyValueSource) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QQmlPropertyValueSource9setTargetERK12QQmlProperty()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QQmlPropertyValueSource9setTargetERK12QQmlProperty(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlPropertyValueSource::QQmlPropertyValueSource();
impl /*struct*/ QQmlPropertyValueSource {
  pub fn new<T: QQmlPropertyValueSource_new>(value: T) -> QQmlPropertyValueSource {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlPropertyValueSource_new {
  fn new(self) -> QQmlPropertyValueSource;
}

  // proto:  void QQmlPropertyValueSource::QQmlPropertyValueSource();
impl<'a> /*trait*/ QQmlPropertyValueSource_new for () {
  fn new(self) -> QQmlPropertyValueSource {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QQmlPropertyValueSourceC2Ev()};
    let ctysz: c_int = unsafe{QQmlPropertyValueSource_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN23QQmlPropertyValueSourceC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlPropertyValueSource{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

