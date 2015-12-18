// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QStyleOption::FreeQStyleOption();
  fn _ZN12QStyleOptionD0Ev(qthis: *mut c_void) ;
  // proto:  void QStyleOption::init(const QWidget * w);
  fn _ZN12QStyleOption4initEPK7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStyleOption::NewQStyleOption(const QStyleOption & other);
  fn _ZN12QStyleOptionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStyleOption::NewQStyleOption(int version, int type);
  fn _ZN12QStyleOptionC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QStyleOption::initFrom(const QWidget * w);
  fn _ZN12QStyleOption8initFromEPK7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QStyleOption)=1
pub struct QStyleOption {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleOption {
  pub fn FreeQStyleOption<RetType, T: QStyleOption_FreeQStyleOption<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQStyleOption(self);
    // return 1;
  }
}

pub trait QStyleOption_FreeQStyleOption<RetType> {
  fn FreeQStyleOption(self, rsthis: &mut QStyleOption) -> RetType;
}

// proto:  void QStyleOption::FreeQStyleOption();
impl<'a> /*trait*/ QStyleOption_FreeQStyleOption<()> for () {
  fn FreeQStyleOption(self, rsthis: &mut QStyleOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOptionD0Ev()};
     unsafe {_ZN12QStyleOptionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStyleOption {
  pub fn init<RetType, T: QStyleOption_init<RetType>>(&mut self, value: T) -> RetType {
    return value.init(self);
    // return 1;
  }
}

pub trait QStyleOption_init<RetType> {
  fn init(self, rsthis: &mut QStyleOption) -> RetType;
}

// proto:  void QStyleOption::init(const QWidget * w);
impl<'a> /*trait*/ QStyleOption_init<()> for (&'a  QWidget) {
  fn init(self, rsthis: &mut QStyleOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOption4initEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QStyleOption4initEPK7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStyleOption {
  pub fn NewQStyleOption<T: QStyleOption_NewQStyleOption>(value: T) -> QStyleOption {
    let rsthis = value.NewQStyleOption();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOption_NewQStyleOption {
  fn NewQStyleOption(self) -> QStyleOption;
}

// proto: void QStyleOption::NewQStyleOption(const QStyleOption & other);
impl<'a> /*trait*/ QStyleOption_NewQStyleOption for (&'a  QStyleOption) {
  fn NewQStyleOption(self) -> QStyleOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOptionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QStyleOptionC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOption::NewQStyleOption(int version, int type);
impl<'a> /*trait*/ QStyleOption_NewQStyleOption for (i32, i32) {
  fn NewQStyleOption(self) -> QStyleOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOptionC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QStyleOptionC1Eii(qthis, arg0, arg1)};
    let rsthis = QStyleOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOption {
  pub fn initFrom<RetType, T: QStyleOption_initFrom<RetType>>(&mut self, value: T) -> RetType {
    return value.initFrom(self);
    // return 1;
  }
}

pub trait QStyleOption_initFrom<RetType> {
  fn initFrom(self, rsthis: &mut QStyleOption) -> RetType;
}

// proto:  void QStyleOption::initFrom(const QWidget * w);
impl<'a> /*trait*/ QStyleOption_initFrom<()> for (&'a  QWidget) {
  fn initFrom(self, rsthis: &mut QStyleOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOption8initFromEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QStyleOption8initFromEPK7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

