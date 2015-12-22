// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtCore/qabstractnativeeventfilter.h
// dst-file: /src/core/qabstractnativeeventfilter.rs
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
use super::qbytearray::QByteArray; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  bool QAbstractNativeEventFilter::nativeEventFilter(const QByteArray & eventType, void * message, long * result);
  fn _ZN26QAbstractNativeEventFilter17nativeEventFilterERK10QByteArrayPvPl(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_long) -> c_char;
  // proto:  void QAbstractNativeEventFilter::QAbstractNativeEventFilter();
  fn _ZN26QAbstractNativeEventFilterC1Ev(qthis: *mut c_void);
  // proto:  void QAbstractNativeEventFilter::~QAbstractNativeEventFilter();
  fn _ZN26QAbstractNativeEventFilterD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractNativeEventFilter)=16
pub struct QAbstractNativeEventFilter {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAbstractNativeEventFilter {
  pub fn inheritFrom(qthis: *mut c_void) -> QAbstractNativeEventFilter {
    return QAbstractNativeEventFilter{qclsinst: qthis};
  }
}
  // proto:  bool QAbstractNativeEventFilter::nativeEventFilter(const QByteArray & eventType, void * message, long * result);
impl /*struct*/ QAbstractNativeEventFilter {
  pub fn nativeEventFilter<RetType, T: QAbstractNativeEventFilter_nativeEventFilter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.nativeEventFilter(self);
    // return 1;
  }
}

pub trait QAbstractNativeEventFilter_nativeEventFilter<RetType> {
  fn nativeEventFilter(self , rsthis: &mut QAbstractNativeEventFilter) -> RetType;
}

  // proto:  bool QAbstractNativeEventFilter::nativeEventFilter(const QByteArray & eventType, void * message, long * result);
impl<'a> /*trait*/ QAbstractNativeEventFilter_nativeEventFilter<i8> for (QByteArray, *mut c_void, &'a mut Vec<i64>) {
  fn nativeEventFilter(self , rsthis: &mut QAbstractNativeEventFilter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAbstractNativeEventFilter17nativeEventFilterERK10QByteArrayPvPl()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *mut c_long;
    let mut ret = unsafe {_ZN26QAbstractNativeEventFilter17nativeEventFilterERK10QByteArrayPvPl(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractNativeEventFilter::QAbstractNativeEventFilter();
impl /*struct*/ QAbstractNativeEventFilter {
  pub fn NewQAbstractNativeEventFilter<T: QAbstractNativeEventFilter_NewQAbstractNativeEventFilter>(value: T) -> QAbstractNativeEventFilter {
    let rsthis = value.NewQAbstractNativeEventFilter();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractNativeEventFilter_NewQAbstractNativeEventFilter {
  fn NewQAbstractNativeEventFilter(self) -> QAbstractNativeEventFilter;
}

  // proto:  void QAbstractNativeEventFilter::QAbstractNativeEventFilter();
impl<'a> /*trait*/ QAbstractNativeEventFilter_NewQAbstractNativeEventFilter for () {
  fn NewQAbstractNativeEventFilter(self) -> QAbstractNativeEventFilter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAbstractNativeEventFilterC1Ev()};
    unsafe {_ZN26QAbstractNativeEventFilterC1Ev(qthis)};
    let rsthis = QAbstractNativeEventFilter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractNativeEventFilter::~QAbstractNativeEventFilter();
impl /*struct*/ QAbstractNativeEventFilter {
  pub fn FreeQAbstractNativeEventFilter<RetType, T: QAbstractNativeEventFilter_FreeQAbstractNativeEventFilter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAbstractNativeEventFilter(self);
    // return 1;
  }
}

pub trait QAbstractNativeEventFilter_FreeQAbstractNativeEventFilter<RetType> {
  fn FreeQAbstractNativeEventFilter(self , rsthis: &mut QAbstractNativeEventFilter) -> RetType;
}

  // proto:  void QAbstractNativeEventFilter::~QAbstractNativeEventFilter();
impl<'a> /*trait*/ QAbstractNativeEventFilter_FreeQAbstractNativeEventFilter<()> for () {
  fn FreeQAbstractNativeEventFilter(self , rsthis: &mut QAbstractNativeEventFilter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAbstractNativeEventFilterD0Ev()};
     unsafe {_ZN26QAbstractNativeEventFilterD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

