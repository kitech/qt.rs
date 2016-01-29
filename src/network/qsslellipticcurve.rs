// auto generated, do not modify.
// created: Tue Jan 19 21:53:37 2016
// src-file: /QtNetwork/qsslellipticcurve.h
// dst-file: /src/network/qsslellipticcurve.rs
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
  fn QSslEllipticCurve_Class_Size() -> c_int;
  // proto:  void QSslEllipticCurve::QSslEllipticCurve();
  fn _ZN17QSslEllipticCurveC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QSslEllipticCurve::isTlsNamedCurve();
  fn _ZNK17QSslEllipticCurve15isTlsNamedCurveEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QSslEllipticCurve QSslEllipticCurve::fromShortName(const QString & name);
  fn _ZN17QSslEllipticCurve13fromShortNameERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QSslEllipticCurve::isValid();
  fn _ZNK17QSslEllipticCurve7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QSslEllipticCurve::longName();
  fn _ZNK17QSslEllipticCurve8longNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QSslEllipticCurve::shortName();
  fn _ZNK17QSslEllipticCurve9shortNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QSslEllipticCurve QSslEllipticCurve::fromLongName(const QString & name);
  fn _ZN17QSslEllipticCurve12fromLongNameERK7QString(arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QSslEllipticCurve)=4
#[derive(Default)]
pub struct QSslEllipticCurve {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSslEllipticCurve {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSslEllipticCurve {
    return QSslEllipticCurve{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QSslEllipticCurve::QSslEllipticCurve();
impl /*struct*/ QSslEllipticCurve {
  pub fn new<T: QSslEllipticCurve_new>(value: T) -> QSslEllipticCurve {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSslEllipticCurve_new {
  fn new(self) -> QSslEllipticCurve;
}

  // proto:  void QSslEllipticCurve::QSslEllipticCurve();
impl<'a> /*trait*/ QSslEllipticCurve_new for () {
  fn new(self) -> QSslEllipticCurve {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslEllipticCurveC2Ev()};
    let ctysz: c_int = unsafe{QSslEllipticCurve_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN17QSslEllipticCurveC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSslEllipticCurve{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QSslEllipticCurve::isTlsNamedCurve();
impl /*struct*/ QSslEllipticCurve {
  pub fn isTlsNamedCurve<RetType, T: QSslEllipticCurve_isTlsNamedCurve<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTlsNamedCurve(self);
    // return 1;
  }
}

pub trait QSslEllipticCurve_isTlsNamedCurve<RetType> {
  fn isTlsNamedCurve(self , rsthis: & QSslEllipticCurve) -> RetType;
}

  // proto:  bool QSslEllipticCurve::isTlsNamedCurve();
impl<'a> /*trait*/ QSslEllipticCurve_isTlsNamedCurve<i8> for () {
  fn isTlsNamedCurve(self , rsthis: & QSslEllipticCurve) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslEllipticCurve15isTlsNamedCurveEv()};
    let mut ret = unsafe {_ZNK17QSslEllipticCurve15isTlsNamedCurveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QSslEllipticCurve QSslEllipticCurve::fromShortName(const QString & name);
impl /*struct*/ QSslEllipticCurve {
  pub fn fromShortName_s<RetType, T: QSslEllipticCurve_fromShortName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromShortName_s();
    // return 1;
  }
}

pub trait QSslEllipticCurve_fromShortName_s<RetType> {
  fn fromShortName_s(self ) -> RetType;
}

  // proto: static QSslEllipticCurve QSslEllipticCurve::fromShortName(const QString & name);
impl<'a> /*trait*/ QSslEllipticCurve_fromShortName_s<QSslEllipticCurve> for (&'a QString) {
  fn fromShortName_s(self ) -> QSslEllipticCurve {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslEllipticCurve13fromShortNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QSslEllipticCurve13fromShortNameERK7QString(arg0)};
    let mut ret1 = QSslEllipticCurve::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSslEllipticCurve::isValid();
impl /*struct*/ QSslEllipticCurve {
  pub fn isValid<RetType, T: QSslEllipticCurve_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QSslEllipticCurve_isValid<RetType> {
  fn isValid(self , rsthis: & QSslEllipticCurve) -> RetType;
}

  // proto:  bool QSslEllipticCurve::isValid();
impl<'a> /*trait*/ QSslEllipticCurve_isValid<i8> for () {
  fn isValid(self , rsthis: & QSslEllipticCurve) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslEllipticCurve7isValidEv()};
    let mut ret = unsafe {_ZNK17QSslEllipticCurve7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QSslEllipticCurve::longName();
impl /*struct*/ QSslEllipticCurve {
  pub fn longName<RetType, T: QSslEllipticCurve_longName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.longName(self);
    // return 1;
  }
}

pub trait QSslEllipticCurve_longName<RetType> {
  fn longName(self , rsthis: & QSslEllipticCurve) -> RetType;
}

  // proto:  QString QSslEllipticCurve::longName();
impl<'a> /*trait*/ QSslEllipticCurve_longName<QString> for () {
  fn longName(self , rsthis: & QSslEllipticCurve) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslEllipticCurve8longNameEv()};
    let mut ret = unsafe {_ZNK17QSslEllipticCurve8longNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QSslEllipticCurve::shortName();
impl /*struct*/ QSslEllipticCurve {
  pub fn shortName<RetType, T: QSslEllipticCurve_shortName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shortName(self);
    // return 1;
  }
}

pub trait QSslEllipticCurve_shortName<RetType> {
  fn shortName(self , rsthis: & QSslEllipticCurve) -> RetType;
}

  // proto:  QString QSslEllipticCurve::shortName();
impl<'a> /*trait*/ QSslEllipticCurve_shortName<QString> for () {
  fn shortName(self , rsthis: & QSslEllipticCurve) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSslEllipticCurve9shortNameEv()};
    let mut ret = unsafe {_ZNK17QSslEllipticCurve9shortNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QSslEllipticCurve QSslEllipticCurve::fromLongName(const QString & name);
impl /*struct*/ QSslEllipticCurve {
  pub fn fromLongName_s<RetType, T: QSslEllipticCurve_fromLongName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromLongName_s();
    // return 1;
  }
}

pub trait QSslEllipticCurve_fromLongName_s<RetType> {
  fn fromLongName_s(self ) -> RetType;
}

  // proto: static QSslEllipticCurve QSslEllipticCurve::fromLongName(const QString & name);
impl<'a> /*trait*/ QSslEllipticCurve_fromLongName_s<QSslEllipticCurve> for (&'a QString) {
  fn fromLongName_s(self ) -> QSslEllipticCurve {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSslEllipticCurve12fromLongNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QSslEllipticCurve12fromLongNameERK7QString(arg0)};
    let mut ret1 = QSslEllipticCurve::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

