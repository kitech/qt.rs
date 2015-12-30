// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtCore/qtextboundaryfinder.h
// dst-file: /src/core/qtextboundaryfinder.rs
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
use super::qchar::QChar; // 773
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextBoundaryFinder_Class_Size() -> c_int;
  // proto:  bool QTextBoundaryFinder::isAtBoundary();
  fn _ZNK19QTextBoundaryFinder12isAtBoundaryEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QTextBoundaryFinder::toNextBoundary();
  fn _ZN19QTextBoundaryFinder14toNextBoundaryEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextBoundaryFinder::toEnd();
  fn _ZN19QTextBoundaryFinder5toEndEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextBoundaryFinder::QTextBoundaryFinder(const QTextBoundaryFinder & other);
  fn dector_ZN19QTextBoundaryFinderC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QTextBoundaryFinderC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextBoundaryFinder::setPosition(int position);
  fn _ZN19QTextBoundaryFinder11setPositionEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTextBoundaryFinder::QTextBoundaryFinder();
  fn dector_ZN19QTextBoundaryFinderC1Ev() -> *mut c_void;
  fn _ZN19QTextBoundaryFinderC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QTextBoundaryFinder::toPreviousBoundary();
  fn _ZN19QTextBoundaryFinder18toPreviousBoundaryEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QTextBoundaryFinder::isValid();
  fn demth_ZNK19QTextBoundaryFinder7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextBoundaryFinder::~QTextBoundaryFinder();
  fn _ZN19QTextBoundaryFinderD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QTextBoundaryFinder::string();
  fn _ZNK19QTextBoundaryFinder6stringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextBoundaryFinder::toStart();
  fn _ZN19QTextBoundaryFinder7toStartEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QTextBoundaryFinder::position();
  fn _ZNK19QTextBoundaryFinder8positionEv(qthis: u64 /* *mut c_void*/) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QTextBoundaryFinder)=48
#[derive(Default)]
pub struct QTextBoundaryFinder {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextBoundaryFinder {
    return QTextBoundaryFinder{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QTextBoundaryFinder::isAtBoundary();
impl /*struct*/ QTextBoundaryFinder {
  pub fn isAtBoundary<RetType, T: QTextBoundaryFinder_isAtBoundary<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAtBoundary(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_isAtBoundary<RetType> {
  fn isAtBoundary(self , rsthis: & QTextBoundaryFinder) -> RetType;
}

  // proto:  bool QTextBoundaryFinder::isAtBoundary();
impl<'a> /*trait*/ QTextBoundaryFinder_isAtBoundary<i8> for () {
  fn isAtBoundary(self , rsthis: & QTextBoundaryFinder) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder12isAtBoundaryEv()};
    let mut ret = unsafe {_ZNK19QTextBoundaryFinder12isAtBoundaryEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTextBoundaryFinder::toNextBoundary();
impl /*struct*/ QTextBoundaryFinder {
  pub fn toNextBoundary<RetType, T: QTextBoundaryFinder_toNextBoundary<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toNextBoundary(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_toNextBoundary<RetType> {
  fn toNextBoundary(self , rsthis: & QTextBoundaryFinder) -> RetType;
}

  // proto:  int QTextBoundaryFinder::toNextBoundary();
impl<'a> /*trait*/ QTextBoundaryFinder_toNextBoundary<i32> for () {
  fn toNextBoundary(self , rsthis: & QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder14toNextBoundaryEv()};
    let mut ret = unsafe {_ZN19QTextBoundaryFinder14toNextBoundaryEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextBoundaryFinder::toEnd();
impl /*struct*/ QTextBoundaryFinder {
  pub fn toEnd<RetType, T: QTextBoundaryFinder_toEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toEnd(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_toEnd<RetType> {
  fn toEnd(self , rsthis: & QTextBoundaryFinder) -> RetType;
}

  // proto:  void QTextBoundaryFinder::toEnd();
impl<'a> /*trait*/ QTextBoundaryFinder_toEnd<()> for () {
  fn toEnd(self , rsthis: & QTextBoundaryFinder) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder5toEndEv()};
     unsafe {_ZN19QTextBoundaryFinder5toEndEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBoundaryFinder::QTextBoundaryFinder(const QTextBoundaryFinder & other);
impl /*struct*/ QTextBoundaryFinder {
  pub fn New<T: QTextBoundaryFinder_New>(value: T) -> QTextBoundaryFinder {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBoundaryFinder_New {
  fn New(self) -> QTextBoundaryFinder;
}

  // proto:  void QTextBoundaryFinder::QTextBoundaryFinder(const QTextBoundaryFinder & other);
impl<'a> /*trait*/ QTextBoundaryFinder_New for (&'a QTextBoundaryFinder) {
  fn New(self) -> QTextBoundaryFinder {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinderC1ERKS_()};
    let ctysz: c_int = unsafe{QTextBoundaryFinder_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QTextBoundaryFinderC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QTextBoundaryFinderC1ERKS_(arg0)} as u64;
    let rsthis = QTextBoundaryFinder{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextBoundaryFinder::setPosition(int position);
impl /*struct*/ QTextBoundaryFinder {
  pub fn setPosition<RetType, T: QTextBoundaryFinder_setPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPosition(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_setPosition<RetType> {
  fn setPosition(self , rsthis: & QTextBoundaryFinder) -> RetType;
}

  // proto:  void QTextBoundaryFinder::setPosition(int position);
impl<'a> /*trait*/ QTextBoundaryFinder_setPosition<()> for (i32) {
  fn setPosition(self , rsthis: & QTextBoundaryFinder) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder11setPositionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN19QTextBoundaryFinder11setPositionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBoundaryFinder::QTextBoundaryFinder();
impl<'a> /*trait*/ QTextBoundaryFinder_New for () {
  fn New(self) -> QTextBoundaryFinder {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinderC1Ev()};
    let ctysz: c_int = unsafe{QTextBoundaryFinder_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN19QTextBoundaryFinderC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN19QTextBoundaryFinderC1Ev()} as u64;
    let rsthis = QTextBoundaryFinder{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTextBoundaryFinder::toPreviousBoundary();
impl /*struct*/ QTextBoundaryFinder {
  pub fn toPreviousBoundary<RetType, T: QTextBoundaryFinder_toPreviousBoundary<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPreviousBoundary(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_toPreviousBoundary<RetType> {
  fn toPreviousBoundary(self , rsthis: & QTextBoundaryFinder) -> RetType;
}

  // proto:  int QTextBoundaryFinder::toPreviousBoundary();
impl<'a> /*trait*/ QTextBoundaryFinder_toPreviousBoundary<i32> for () {
  fn toPreviousBoundary(self , rsthis: & QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder18toPreviousBoundaryEv()};
    let mut ret = unsafe {_ZN19QTextBoundaryFinder18toPreviousBoundaryEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTextBoundaryFinder::isValid();
impl /*struct*/ QTextBoundaryFinder {
  pub fn isValid<RetType, T: QTextBoundaryFinder_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_isValid<RetType> {
  fn isValid(self , rsthis: & QTextBoundaryFinder) -> RetType;
}

  // proto:  bool QTextBoundaryFinder::isValid();
impl<'a> /*trait*/ QTextBoundaryFinder_isValid<i8> for () {
  fn isValid(self , rsthis: & QTextBoundaryFinder) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder7isValidEv()};
    let mut ret = unsafe {demth_ZNK19QTextBoundaryFinder7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextBoundaryFinder::~QTextBoundaryFinder();
impl /*struct*/ QTextBoundaryFinder {
  pub fn Free<RetType, T: QTextBoundaryFinder_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_Free<RetType> {
  fn Free(self , rsthis: & QTextBoundaryFinder) -> RetType;
}

  // proto:  void QTextBoundaryFinder::~QTextBoundaryFinder();
impl<'a> /*trait*/ QTextBoundaryFinder_Free<()> for () {
  fn Free(self , rsthis: & QTextBoundaryFinder) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinderD0Ev()};
     unsafe {_ZN19QTextBoundaryFinderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QTextBoundaryFinder::string();
impl /*struct*/ QTextBoundaryFinder {
  pub fn string<RetType, T: QTextBoundaryFinder_string<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.string(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_string<RetType> {
  fn string(self , rsthis: & QTextBoundaryFinder) -> RetType;
}

  // proto:  QString QTextBoundaryFinder::string();
impl<'a> /*trait*/ QTextBoundaryFinder_string<QString> for () {
  fn string(self , rsthis: & QTextBoundaryFinder) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder6stringEv()};
    let mut ret = unsafe {_ZNK19QTextBoundaryFinder6stringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextBoundaryFinder::toStart();
impl /*struct*/ QTextBoundaryFinder {
  pub fn toStart<RetType, T: QTextBoundaryFinder_toStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toStart(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_toStart<RetType> {
  fn toStart(self , rsthis: & QTextBoundaryFinder) -> RetType;
}

  // proto:  void QTextBoundaryFinder::toStart();
impl<'a> /*trait*/ QTextBoundaryFinder_toStart<()> for () {
  fn toStart(self , rsthis: & QTextBoundaryFinder) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder7toStartEv()};
     unsafe {_ZN19QTextBoundaryFinder7toStartEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTextBoundaryFinder::position();
impl /*struct*/ QTextBoundaryFinder {
  pub fn position<RetType, T: QTextBoundaryFinder_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_position<RetType> {
  fn position(self , rsthis: & QTextBoundaryFinder) -> RetType;
}

  // proto:  int QTextBoundaryFinder::position();
impl<'a> /*trait*/ QTextBoundaryFinder_position<i32> for () {
  fn position(self , rsthis: & QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder8positionEv()};
    let mut ret = unsafe {_ZNK19QTextBoundaryFinder8positionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

