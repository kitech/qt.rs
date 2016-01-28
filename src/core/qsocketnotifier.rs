// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qsocketnotifier.h
// dst-file: /src/core/qsocketnotifier.rs
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
use super::qobject::*; // 773
use std::ops::Deref;
use super::qobjectdefs::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSocketNotifier_Class_Size() -> c_int;
  // proto:  qintptr QSocketNotifier::socket();
  fn C_ZNK15QSocketNotifier6socketEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QSocketNotifier::isEnabled();
  fn C_ZNK15QSocketNotifier9isEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSocketNotifier::setEnabled(bool );
  fn C_ZN15QSocketNotifier10setEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  const QMetaObject * QSocketNotifier::metaObject();
  fn C_ZNK15QSocketNotifier10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSocketNotifier::~QSocketNotifier();
  fn C_ZN15QSocketNotifierD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QSocketNotifier)=1
#[derive(Default)]
pub struct QSocketNotifier {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _activated: QSocketNotifier_activated_signal,
}

impl /*struct*/ QSocketNotifier {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSocketNotifier {
    return QSocketNotifier{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSocketNotifier {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QSocketNotifier {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  qintptr QSocketNotifier::socket();
impl /*struct*/ QSocketNotifier {
  pub fn socket<RetType, T: QSocketNotifier_socket<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.socket(self);
    // return 1;
  }
}

pub trait QSocketNotifier_socket<RetType> {
  fn socket(self , rsthis: & QSocketNotifier) -> RetType;
}

  // proto:  qintptr QSocketNotifier::socket();
impl<'a> /*trait*/ QSocketNotifier_socket<i32> for () {
  fn socket(self , rsthis: & QSocketNotifier) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSocketNotifier6socketEv()};
    let mut ret = unsafe {C_ZNK15QSocketNotifier6socketEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QSocketNotifier::isEnabled();
impl /*struct*/ QSocketNotifier {
  pub fn isEnabled<RetType, T: QSocketNotifier_isEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEnabled(self);
    // return 1;
  }
}

pub trait QSocketNotifier_isEnabled<RetType> {
  fn isEnabled(self , rsthis: & QSocketNotifier) -> RetType;
}

  // proto:  bool QSocketNotifier::isEnabled();
impl<'a> /*trait*/ QSocketNotifier_isEnabled<i8> for () {
  fn isEnabled(self , rsthis: & QSocketNotifier) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSocketNotifier9isEnabledEv()};
    let mut ret = unsafe {C_ZNK15QSocketNotifier9isEnabledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSocketNotifier::setEnabled(bool );
impl /*struct*/ QSocketNotifier {
  pub fn setEnabled<RetType, T: QSocketNotifier_setEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEnabled(self);
    // return 1;
  }
}

pub trait QSocketNotifier_setEnabled<RetType> {
  fn setEnabled(self , rsthis: & QSocketNotifier) -> RetType;
}

  // proto:  void QSocketNotifier::setEnabled(bool );
impl<'a> /*trait*/ QSocketNotifier_setEnabled<()> for (i8) {
  fn setEnabled(self , rsthis: & QSocketNotifier) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSocketNotifier10setEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN15QSocketNotifier10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QSocketNotifier::metaObject();
impl /*struct*/ QSocketNotifier {
  pub fn metaObject<RetType, T: QSocketNotifier_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSocketNotifier_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSocketNotifier) -> RetType;
}

  // proto:  const QMetaObject * QSocketNotifier::metaObject();
impl<'a> /*trait*/ QSocketNotifier_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QSocketNotifier) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSocketNotifier10metaObjectEv()};
    let mut ret = unsafe {C_ZNK15QSocketNotifier10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSocketNotifier::~QSocketNotifier();
impl /*struct*/ QSocketNotifier {
  pub fn free<RetType, T: QSocketNotifier_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSocketNotifier_free<RetType> {
  fn free(self , rsthis: & QSocketNotifier) -> RetType;
}

  // proto:  void QSocketNotifier::~QSocketNotifier();
impl<'a> /*trait*/ QSocketNotifier_free<()> for () {
  fn free(self , rsthis: & QSocketNotifier) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSocketNotifierD2Ev()};
     unsafe {C_ZN15QSocketNotifierD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QSocketNotifier_activated
pub struct QSocketNotifier_activated_signal{poi:u64}
impl /* struct */ QSocketNotifier {
  pub fn activated(&self) -> QSocketNotifier_activated_signal {
     return QSocketNotifier_activated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSocketNotifier_activated_signal {
  pub fn connect<T: QSocketNotifier_activated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSocketNotifier_activated_signal_connect {
  fn connect(self, sigthis: QSocketNotifier_activated_signal);
}

// <= body block end

