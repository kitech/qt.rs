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
  // proto:  void QTextOption::QTextOption(const QTextOption & o);
  fn _ZN11QTextOptionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QTextOption::tabStop();
  fn _ZNK11QTextOption7tabStopEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextOption::setUseDesignMetrics(bool b);
  fn _ZN11QTextOption19setUseDesignMetricsEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextOption::setTabStop(qreal tabStop);
  fn _ZN11QTextOption10setTabStopEd(qthis: *mut c_void, arg0: c_double);
  // proto:  bool QTextOption::useDesignMetrics();
  fn _ZNK11QTextOption16useDesignMetricsEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextOption::QTextOption();
  fn _ZN11QTextOptionC1Ev(qthis: *mut c_void);
  // proto:  QList<qreal> QTextOption::tabArray();
  fn _ZNK11QTextOption8tabArrayEv(qthis: *mut c_void);
  // proto:  void QTextOption::~QTextOption();
  fn _ZN11QTextOptionD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QTextOption)=32
pub struct QTextOption {
  pub qclsinst: *mut c_void,
}

  // proto:  void QTextOption::QTextOption(const QTextOption & o);
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

  // proto:  void QTextOption::QTextOption(const QTextOption & o);
impl<'a> /*trait*/ QTextOption_NewQTextOption for (QTextOption) {
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

  // proto:  qreal QTextOption::tabStop();
impl /*struct*/ QTextOption {
  pub fn tabStop<RetType, T: QTextOption_tabStop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabStop(self);
    // return 1;
  }
}

pub trait QTextOption_tabStop<RetType> {
  fn tabStop(self , rsthis: &mut QTextOption) -> RetType;
}

  // proto:  qreal QTextOption::tabStop();
impl<'a> /*trait*/ QTextOption_tabStop<f64> for () {
  fn tabStop(self , rsthis: &mut QTextOption) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextOption7tabStopEv()};
    let mut ret = unsafe {_ZNK11QTextOption7tabStopEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextOption::setUseDesignMetrics(bool b);
impl /*struct*/ QTextOption {
  pub fn setUseDesignMetrics<RetType, T: QTextOption_setUseDesignMetrics<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setUseDesignMetrics(self);
    // return 1;
  }
}

pub trait QTextOption_setUseDesignMetrics<RetType> {
  fn setUseDesignMetrics(self , rsthis: &mut QTextOption) -> RetType;
}

  // proto:  void QTextOption::setUseDesignMetrics(bool b);
impl<'a> /*trait*/ QTextOption_setUseDesignMetrics<()> for (i8) {
  fn setUseDesignMetrics(self , rsthis: &mut QTextOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextOption19setUseDesignMetricsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QTextOption19setUseDesignMetricsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextOption::setTabStop(qreal tabStop);
impl /*struct*/ QTextOption {
  pub fn setTabStop<RetType, T: QTextOption_setTabStop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTabStop(self);
    // return 1;
  }
}

pub trait QTextOption_setTabStop<RetType> {
  fn setTabStop(self , rsthis: &mut QTextOption) -> RetType;
}

  // proto:  void QTextOption::setTabStop(qreal tabStop);
impl<'a> /*trait*/ QTextOption_setTabStop<()> for (f64) {
  fn setTabStop(self , rsthis: &mut QTextOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextOption10setTabStopEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN11QTextOption10setTabStopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextOption::useDesignMetrics();
impl /*struct*/ QTextOption {
  pub fn useDesignMetrics<RetType, T: QTextOption_useDesignMetrics<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.useDesignMetrics(self);
    // return 1;
  }
}

pub trait QTextOption_useDesignMetrics<RetType> {
  fn useDesignMetrics(self , rsthis: &mut QTextOption) -> RetType;
}

  // proto:  bool QTextOption::useDesignMetrics();
impl<'a> /*trait*/ QTextOption_useDesignMetrics<i8> for () {
  fn useDesignMetrics(self , rsthis: &mut QTextOption) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextOption16useDesignMetricsEv()};
    let mut ret = unsafe {_ZNK11QTextOption16useDesignMetricsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextOption::QTextOption();
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

  // proto:  QList<qreal> QTextOption::tabArray();
impl /*struct*/ QTextOption {
  pub fn tabArray<RetType, T: QTextOption_tabArray<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabArray(self);
    // return 1;
  }
}

pub trait QTextOption_tabArray<RetType> {
  fn tabArray(self , rsthis: &mut QTextOption) -> RetType;
}

  // proto:  QList<qreal> QTextOption::tabArray();
impl<'a> /*trait*/ QTextOption_tabArray<()> for () {
  fn tabArray(self , rsthis: &mut QTextOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextOption8tabArrayEv()};
     unsafe {_ZNK11QTextOption8tabArrayEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextOption::~QTextOption();
impl /*struct*/ QTextOption {
  pub fn FreeQTextOption<RetType, T: QTextOption_FreeQTextOption<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTextOption(self);
    // return 1;
  }
}

pub trait QTextOption_FreeQTextOption<RetType> {
  fn FreeQTextOption(self , rsthis: &mut QTextOption) -> RetType;
}

  // proto:  void QTextOption::~QTextOption();
impl<'a> /*trait*/ QTextOption_FreeQTextOption<()> for () {
  fn FreeQTextOption(self , rsthis: &mut QTextOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextOptionD0Ev()};
     unsafe {_ZN11QTextOptionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

