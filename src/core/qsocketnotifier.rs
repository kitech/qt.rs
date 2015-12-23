// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
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
use super::qobject::QObject; // 773
use std::ops::Deref;
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QSocketNotifier::QSocketNotifier(const QSocketNotifier & );
  fn _ZN15QSocketNotifierC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qintptr QSocketNotifier::socket();
  fn _ZNK15QSocketNotifier6socketEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QSocketNotifier::isEnabled();
  fn _ZNK15QSocketNotifier9isEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  void QSocketNotifier::setEnabled(bool );
  fn _ZN15QSocketNotifier10setEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  const QMetaObject * QSocketNotifier::metaObject();
  fn _ZNK15QSocketNotifier10metaObjectEv(qthis: *mut c_void);
  // proto:  void QSocketNotifier::~QSocketNotifier();
  fn _ZN15QSocketNotifierD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSocketNotifier)=1
pub struct QSocketNotifier {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSocketNotifier {
  pub fn inheritFrom(qthis: *mut c_void) -> QSocketNotifier {
    return QSocketNotifier{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
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
  // proto:  void QSocketNotifier::QSocketNotifier(const QSocketNotifier & );
impl /*struct*/ QSocketNotifier {
  pub fn New<T: QSocketNotifier_New>(value: T) -> QSocketNotifier {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QSocketNotifier_New {
  fn New(self) -> QSocketNotifier;
}

  // proto:  void QSocketNotifier::QSocketNotifier(const QSocketNotifier & );
impl<'a> /*trait*/ QSocketNotifier_New for (&'a QSocketNotifier) {
  fn New(self) -> QSocketNotifier {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSocketNotifierC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QSocketNotifierC1ERKS_(qthis, arg0)};
    let rsthis = QSocketNotifier{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
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
    let mut ret = unsafe {_ZNK15QSocketNotifier6socketEv(rsthis.qclsinst)};
    return ret as i32;
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
    let mut ret = unsafe {_ZNK15QSocketNotifier9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
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
     unsafe {_ZN15QSocketNotifier10setEnabledEb(rsthis.qclsinst, arg0)};
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
impl<'a> /*trait*/ QSocketNotifier_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSocketNotifier) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSocketNotifier10metaObjectEv()};
     unsafe {_ZNK15QSocketNotifier10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSocketNotifier::~QSocketNotifier();
impl /*struct*/ QSocketNotifier {
  pub fn Free<RetType, T: QSocketNotifier_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QSocketNotifier_Free<RetType> {
  fn Free(self , rsthis: & QSocketNotifier) -> RetType;
}

  // proto:  void QSocketNotifier::~QSocketNotifier();
impl<'a> /*trait*/ QSocketNotifier_Free<()> for () {
  fn Free(self , rsthis: & QSocketNotifier) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSocketNotifierD0Ev()};
     unsafe {_ZN15QSocketNotifierD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

