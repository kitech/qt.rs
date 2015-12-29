// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtGui/qtextoption.h
// dst-file: /src/gui/qtextoption.rs
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
  fn QTextOption_Class_Size() -> c_int;
  // proto:  void QTextOption::QTextOption(const QTextOption & o);
  fn dector_ZN11QTextOptionC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QTextOptionC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QTextOption::tabStop();
  fn demth_ZNK11QTextOption7tabStopEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QTextOption::setUseDesignMetrics(bool b);
  fn _ZN11QTextOption19setUseDesignMetricsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextOption::setTabStop(qreal tabStop);
  fn demth_ZN11QTextOption10setTabStopEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  bool QTextOption::useDesignMetrics();
  fn _ZNK11QTextOption16useDesignMetricsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextOption::QTextOption();
  fn dector_ZN11QTextOptionC1Ev() -> *mut c_void;
  fn _ZN11QTextOptionC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QList<qreal> QTextOption::tabArray();
  fn _ZNK11QTextOption8tabArrayEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextOption::~QTextOption();
  fn _ZN11QTextOptionD0Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QTextOption)=32
#[derive(Default)]
pub struct QTextOption {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTextOption {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextOption {
    return QTextOption{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QTextOption::QTextOption(const QTextOption & o);
impl /*struct*/ QTextOption {
  pub fn New<T: QTextOption_New>(value: T) -> QTextOption {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextOption_New {
  fn New(self) -> QTextOption;
}

  // proto:  void QTextOption::QTextOption(const QTextOption & o);
impl<'a> /*trait*/ QTextOption_New for (&'a QTextOption) {
  fn New(self) -> QTextOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextOptionC1ERKS_()};
    let ctysz: c_int = unsafe{QTextOption_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTextOptionC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QTextOptionC1ERKS_(arg0)} as u64;
    let rsthis = QTextOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTextOption::tabStop();
impl /*struct*/ QTextOption {
  pub fn tabStop<RetType, T: QTextOption_tabStop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabStop(self);
    // return 1;
  }
}

pub trait QTextOption_tabStop<RetType> {
  fn tabStop(self , rsthis: & QTextOption) -> RetType;
}

  // proto:  qreal QTextOption::tabStop();
impl<'a> /*trait*/ QTextOption_tabStop<f64> for () {
  fn tabStop(self , rsthis: & QTextOption) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextOption7tabStopEv()};
    let mut ret = unsafe {demth_ZNK11QTextOption7tabStopEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextOption::setUseDesignMetrics(bool b);
impl /*struct*/ QTextOption {
  pub fn setUseDesignMetrics<RetType, T: QTextOption_setUseDesignMetrics<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUseDesignMetrics(self);
    // return 1;
  }
}

pub trait QTextOption_setUseDesignMetrics<RetType> {
  fn setUseDesignMetrics(self , rsthis: & QTextOption) -> RetType;
}

  // proto:  void QTextOption::setUseDesignMetrics(bool b);
impl<'a> /*trait*/ QTextOption_setUseDesignMetrics<()> for (i8) {
  fn setUseDesignMetrics(self , rsthis: & QTextOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextOption19setUseDesignMetricsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QTextOption19setUseDesignMetricsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextOption::setTabStop(qreal tabStop);
impl /*struct*/ QTextOption {
  pub fn setTabStop<RetType, T: QTextOption_setTabStop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabStop(self);
    // return 1;
  }
}

pub trait QTextOption_setTabStop<RetType> {
  fn setTabStop(self , rsthis: & QTextOption) -> RetType;
}

  // proto:  void QTextOption::setTabStop(qreal tabStop);
impl<'a> /*trait*/ QTextOption_setTabStop<()> for (f64) {
  fn setTabStop(self , rsthis: & QTextOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextOption10setTabStopEd()};
    let arg0 = self  as c_double;
     unsafe {demth_ZN11QTextOption10setTabStopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextOption::useDesignMetrics();
impl /*struct*/ QTextOption {
  pub fn useDesignMetrics<RetType, T: QTextOption_useDesignMetrics<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.useDesignMetrics(self);
    // return 1;
  }
}

pub trait QTextOption_useDesignMetrics<RetType> {
  fn useDesignMetrics(self , rsthis: & QTextOption) -> RetType;
}

  // proto:  bool QTextOption::useDesignMetrics();
impl<'a> /*trait*/ QTextOption_useDesignMetrics<i8> for () {
  fn useDesignMetrics(self , rsthis: & QTextOption) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextOption16useDesignMetricsEv()};
    let mut ret = unsafe {_ZNK11QTextOption16useDesignMetricsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextOption::QTextOption();
impl<'a> /*trait*/ QTextOption_New for () {
  fn New(self) -> QTextOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextOptionC1Ev()};
    let ctysz: c_int = unsafe{QTextOption_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN11QTextOptionC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN11QTextOptionC1Ev()} as u64;
    let rsthis = QTextOption{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<qreal> QTextOption::tabArray();
impl /*struct*/ QTextOption {
  pub fn tabArray<RetType, T: QTextOption_tabArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabArray(self);
    // return 1;
  }
}

pub trait QTextOption_tabArray<RetType> {
  fn tabArray(self , rsthis: & QTextOption) -> RetType;
}

  // proto:  QList<qreal> QTextOption::tabArray();
impl<'a> /*trait*/ QTextOption_tabArray<()> for () {
  fn tabArray(self , rsthis: & QTextOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextOption8tabArrayEv()};
     unsafe {_ZNK11QTextOption8tabArrayEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextOption::~QTextOption();
impl /*struct*/ QTextOption {
  pub fn Free<RetType, T: QTextOption_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTextOption_Free<RetType> {
  fn Free(self , rsthis: & QTextOption) -> RetType;
}

  // proto:  void QTextOption::~QTextOption();
impl<'a> /*trait*/ QTextOption_Free<()> for () {
  fn Free(self , rsthis: & QTextOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextOptionD0Ev()};
     unsafe {_ZN11QTextOptionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

