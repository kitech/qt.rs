// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTextOption::NewQTextOption(const QTextOption & o);
  fn _ZN11QTextOptionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QTextOption::tabStop();
  fn _ZNK11QTextOption7tabStopEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextOption::setUseDesignMetrics(bool b);
  fn _ZN11QTextOption19setUseDesignMetricsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextOption::setTabStop(qreal tabStop);
  fn _ZN11QTextOption10setTabStopEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  bool QTextOption::useDesignMetrics();
  fn _ZNK11QTextOption16useDesignMetricsEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextOption::NewQTextOption();
  fn _ZN11QTextOptionC1Ev(qthis: *mut c_void) ;
  // proto:  QList<qreal> QTextOption::tabArray();
  fn _ZNK11QTextOption8tabArrayEv(qthis: *mut c_void) ;
  // proto:  void QTextOption::FreeQTextOption();
  fn _ZN11QTextOptionD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QTextOption)=32
pub struct QTextOption {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextOption {
  pub fn NewQTextOption<T: QTextOption_NewQTextOption>(value: T) -> QTextOption {
    let rsthis = value.NewQTextOption();
    return rsthis;
    // return 1;
  }
}

pub trait QTextOption_NewQTextOption {
  fn NewQTextOption(self) -> QTextOption;
}

// proto: void QTextOption::NewQTextOption(const QTextOption & o);
impl<'a> /*trait*/ QTextOption_NewQTextOption for (&'a  QTextOption) {
  fn NewQTextOption(self) -> QTextOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextOptionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextOptionC1ERKS_(qthis, arg0)};
    let rsthis = QTextOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextOption {
  pub fn tabStop<T: QTextOption_tabStop>(&mut self, value: T) -> f64 {
    return value.tabStop(self);
    // return 1;
  }
}

pub trait QTextOption_tabStop {
  fn tabStop(self, rsthis: &mut QTextOption) -> f64;
}

// proto:  double QTextOption::tabStop();
impl<'a> /*trait*/ QTextOption_tabStop for () {
  fn tabStop(self, rsthis: &mut QTextOption) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextOption7tabStopEv()};
    let mut ret = unsafe {_ZNK11QTextOption7tabStopEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextOption {
  pub fn setUseDesignMetrics<T: QTextOption_setUseDesignMetrics>(&mut self, value: T)  {
     value.setUseDesignMetrics(self);
    // return 1;
  }
}

pub trait QTextOption_setUseDesignMetrics {
  fn setUseDesignMetrics(self, rsthis: &mut QTextOption) ;
}

// proto:  void QTextOption::setUseDesignMetrics(bool b);
impl<'a> /*trait*/ QTextOption_setUseDesignMetrics for (i8) {
  fn setUseDesignMetrics(self, rsthis: &mut QTextOption)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextOption19setUseDesignMetricsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QTextOption19setUseDesignMetricsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextOption {
  pub fn setTabStop<T: QTextOption_setTabStop>(&mut self, value: T)  {
     value.setTabStop(self);
    // return 1;
  }
}

pub trait QTextOption_setTabStop {
  fn setTabStop(self, rsthis: &mut QTextOption) ;
}

// proto:  void QTextOption::setTabStop(qreal tabStop);
impl<'a> /*trait*/ QTextOption_setTabStop for (f64) {
  fn setTabStop(self, rsthis: &mut QTextOption)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextOption10setTabStopEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN11QTextOption10setTabStopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextOption {
  pub fn useDesignMetrics<T: QTextOption_useDesignMetrics>(&mut self, value: T) -> i8 {
    return value.useDesignMetrics(self);
    // return 1;
  }
}

pub trait QTextOption_useDesignMetrics {
  fn useDesignMetrics(self, rsthis: &mut QTextOption) -> i8;
}

// proto:  bool QTextOption::useDesignMetrics();
impl<'a> /*trait*/ QTextOption_useDesignMetrics for () {
  fn useDesignMetrics(self, rsthis: &mut QTextOption) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextOption16useDesignMetricsEv()};
    let mut ret = unsafe {_ZNK11QTextOption16useDesignMetricsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QTextOption::NewQTextOption();
impl<'a> /*trait*/ QTextOption_NewQTextOption for () {
  fn NewQTextOption(self) -> QTextOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextOptionC1Ev()};
    unsafe {_ZN11QTextOptionC1Ev(qthis)};
    let rsthis = QTextOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextOption {
  pub fn tabArray<T: QTextOption_tabArray>(&mut self, value: T)  {
     value.tabArray(self);
    // return 1;
  }
}

pub trait QTextOption_tabArray {
  fn tabArray(self, rsthis: &mut QTextOption) ;
}

// proto:  QList<qreal> QTextOption::tabArray();
impl<'a> /*trait*/ QTextOption_tabArray for () {
  fn tabArray(self, rsthis: &mut QTextOption)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextOption8tabArrayEv()};
     unsafe {_ZNK11QTextOption8tabArrayEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextOption {
  pub fn FreeQTextOption<T: QTextOption_FreeQTextOption>(&mut self, value: T)  {
     value.FreeQTextOption(self);
    // return 1;
  }
}

pub trait QTextOption_FreeQTextOption {
  fn FreeQTextOption(self, rsthis: &mut QTextOption) ;
}

// proto:  void QTextOption::FreeQTextOption();
impl<'a> /*trait*/ QTextOption_FreeQTextOption for () {
  fn FreeQTextOption(self, rsthis: &mut QTextOption)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextOptionD0Ev()};
     unsafe {_ZN11QTextOptionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

