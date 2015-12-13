// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QDoubleValidator::decimals();
  fn _ZNK16QDoubleValidator8decimalsEv() -> i32;
  // proto: void QDoubleValidator::decimalsChanged(int decimals);
  fn _ZN16QDoubleValidator15decimalsChangedEi(arg0: c_int) -> i32;
  // proto: void QDoubleValidator::FreeQDoubleValidator();
  fn _ZN16QDoubleValidatorD0Ev() -> i32;
  // proto: double QDoubleValidator::top();
  fn _ZNK16QDoubleValidator3topEv() -> i32;
  // proto: void QDoubleValidator::bottomChanged(double bottom);
  fn _ZN16QDoubleValidator13bottomChangedEd(arg0: c_double) -> i32;
  // proto: double QDoubleValidator::bottom();
  fn _ZNK16QDoubleValidator6bottomEv() -> i32;
  // proto: void QDoubleValidator::setDecimals(int );
  fn _ZN16QDoubleValidator11setDecimalsEi(arg0: c_int) -> i32;
  // proto: void QDoubleValidator::NewQDoubleValidator(const QDoubleValidator & );
  fn _ZN16QDoubleValidatorC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QDoubleValidator::setBottom(double );
  fn _ZN16QDoubleValidator9setBottomEd(arg0: c_double) -> i32;
  // proto: void QDoubleValidator::setRange(double bottom, double top, int decimals);
  fn _ZN16QDoubleValidator8setRangeEddi(arg0: c_double, arg1: c_double, arg2: c_int) -> i32;
  // proto: void QDoubleValidator::NewQDoubleValidator(QObject * parent);
  fn _ZN16QDoubleValidatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QDoubleValidator::NewQDoubleValidator(double bottom, double top, int decimals, QObject * parent);
  fn _ZN16QDoubleValidatorC1EddiP7QObject(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_int, arg3: *mut c_void) -> i32;
  // proto: void QDoubleValidator::topChanged(double top);
  fn _ZN16QDoubleValidator10topChangedEd(arg0: c_double) -> i32;
  // proto: const QMetaObject * QDoubleValidator::metaObject();
  fn _ZNK16QDoubleValidator10metaObjectEv() -> i32;
  // proto: void QDoubleValidator::setTop(double );
  fn _ZN16QDoubleValidator6setTopEd(arg0: c_double) -> i32;
}

// body block begin
// class sizeof(QDoubleValidator)=1
pub struct QDoubleValidator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDoubleValidator {
  pub fn decimals<T: QDoubleValidator_decimals>(&mut self, value: T) -> i32 {
    value.decimals(self);
    return 1;
  }
}

pub trait QDoubleValidator_decimals {
  fn decimals(self, this: &mut QDoubleValidator) -> i32;
}

// proto: int QDoubleValidator::decimals();
impl<'a> /*trait*/ QDoubleValidator_decimals for () {
  fn decimals(self, this: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator8decimalsEv()};
    unsafe {_ZNK16QDoubleValidator8decimalsEv()};
    return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn decimalsChanged<T: QDoubleValidator_decimalsChanged>(&mut self, value: T) -> i32 {
    value.decimalsChanged(self);
    return 1;
  }
}

pub trait QDoubleValidator_decimalsChanged {
  fn decimalsChanged(self, this: &mut QDoubleValidator) -> i32;
}

// proto: void QDoubleValidator::decimalsChanged(int decimals);
impl<'a> /*trait*/ QDoubleValidator_decimalsChanged for (i32) {
  fn decimalsChanged(self, this: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator15decimalsChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN16QDoubleValidator15decimalsChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn FreeQDoubleValidator<T: QDoubleValidator_FreeQDoubleValidator>(&mut self, value: T) -> i32 {
    value.FreeQDoubleValidator(self);
    return 1;
  }
}

pub trait QDoubleValidator_FreeQDoubleValidator {
  fn FreeQDoubleValidator(self, this: &mut QDoubleValidator) -> i32;
}

// proto: void QDoubleValidator::FreeQDoubleValidator();
impl<'a> /*trait*/ QDoubleValidator_FreeQDoubleValidator for () {
  fn FreeQDoubleValidator(self, this: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidatorD0Ev()};
    unsafe {_ZN16QDoubleValidatorD0Ev()};
    return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn top<T: QDoubleValidator_top>(&mut self, value: T) -> i32 {
    value.top(self);
    return 1;
  }
}

pub trait QDoubleValidator_top {
  fn top(self, this: &mut QDoubleValidator) -> i32;
}

// proto: double QDoubleValidator::top();
impl<'a> /*trait*/ QDoubleValidator_top for () {
  fn top(self, this: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator3topEv()};
    unsafe {_ZNK16QDoubleValidator3topEv()};
    return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn bottomChanged<T: QDoubleValidator_bottomChanged>(&mut self, value: T) -> i32 {
    value.bottomChanged(self);
    return 1;
  }
}

pub trait QDoubleValidator_bottomChanged {
  fn bottomChanged(self, this: &mut QDoubleValidator) -> i32;
}

// proto: void QDoubleValidator::bottomChanged(double bottom);
impl<'a> /*trait*/ QDoubleValidator_bottomChanged for (f64) {
  fn bottomChanged(self, this: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator13bottomChangedEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QDoubleValidator13bottomChangedEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn bottom<T: QDoubleValidator_bottom>(&mut self, value: T) -> i32 {
    value.bottom(self);
    return 1;
  }
}

pub trait QDoubleValidator_bottom {
  fn bottom(self, this: &mut QDoubleValidator) -> i32;
}

// proto: double QDoubleValidator::bottom();
impl<'a> /*trait*/ QDoubleValidator_bottom for () {
  fn bottom(self, this: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator6bottomEv()};
    unsafe {_ZNK16QDoubleValidator6bottomEv()};
    return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn setDecimals<T: QDoubleValidator_setDecimals>(&mut self, value: T) -> i32 {
    value.setDecimals(self);
    return 1;
  }
}

pub trait QDoubleValidator_setDecimals {
  fn setDecimals(self, this: &mut QDoubleValidator) -> i32;
}

// proto: void QDoubleValidator::setDecimals(int );
impl<'a> /*trait*/ QDoubleValidator_setDecimals for (i32) {
  fn setDecimals(self, this: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator11setDecimalsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN16QDoubleValidator11setDecimalsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn NewQDoubleValidator<T: QDoubleValidator_NewQDoubleValidator>(value: T) -> QDoubleValidator {
    let rsthis = value.NewQDoubleValidator();
    return rsthis;
    // return 1;
  }
}

pub trait QDoubleValidator_NewQDoubleValidator {
  fn NewQDoubleValidator(self) -> QDoubleValidator;
}

// proto: void QDoubleValidator::NewQDoubleValidator(const QDoubleValidator & );
impl<'a> /*trait*/ QDoubleValidator_NewQDoubleValidator for (&'a  QDoubleValidator) {
  fn NewQDoubleValidator(self) -> QDoubleValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidatorC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QDoubleValidatorC1ERKS_(qthis, arg0)};
    let rsthis = QDoubleValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn setBottom<T: QDoubleValidator_setBottom>(&mut self, value: T) -> i32 {
    value.setBottom(self);
    return 1;
  }
}

pub trait QDoubleValidator_setBottom {
  fn setBottom(self, this: &mut QDoubleValidator) -> i32;
}

// proto: void QDoubleValidator::setBottom(double );
impl<'a> /*trait*/ QDoubleValidator_setBottom for (f64) {
  fn setBottom(self, this: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator9setBottomEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QDoubleValidator9setBottomEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn setRange<T: QDoubleValidator_setRange>(&mut self, value: T) -> i32 {
    value.setRange(self);
    return 1;
  }
}

pub trait QDoubleValidator_setRange {
  fn setRange(self, this: &mut QDoubleValidator) -> i32;
}

// proto: void QDoubleValidator::setRange(double bottom, double top, int decimals);
impl<'a> /*trait*/ QDoubleValidator_setRange for (f64, f64, i32) {
  fn setRange(self, this: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator8setRangeEddi()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_int;
    unsafe {_ZN16QDoubleValidator8setRangeEddi(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QDoubleValidator::NewQDoubleValidator(QObject * parent);
impl<'a> /*trait*/ QDoubleValidator_NewQDoubleValidator for (&'a mut QObject) {
  fn NewQDoubleValidator(self) -> QDoubleValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidatorC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QDoubleValidatorC1EP7QObject(qthis, arg0)};
    let rsthis = QDoubleValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QDoubleValidator::NewQDoubleValidator(double bottom, double top, int decimals, QObject * parent);
impl<'a> /*trait*/ QDoubleValidator_NewQDoubleValidator for (f64, f64, i32, &'a mut QObject) {
  fn NewQDoubleValidator(self) -> QDoubleValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidatorC1EddiP7QObject()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN16QDoubleValidatorC1EddiP7QObject(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QDoubleValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn topChanged<T: QDoubleValidator_topChanged>(&mut self, value: T) -> i32 {
    value.topChanged(self);
    return 1;
  }
}

pub trait QDoubleValidator_topChanged {
  fn topChanged(self, this: &mut QDoubleValidator) -> i32;
}

// proto: void QDoubleValidator::topChanged(double top);
impl<'a> /*trait*/ QDoubleValidator_topChanged for (f64) {
  fn topChanged(self, this: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator10topChangedEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QDoubleValidator10topChangedEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn metaObject<T: QDoubleValidator_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QDoubleValidator_metaObject {
  fn metaObject(self, this: &mut QDoubleValidator) -> i32;
}

// proto: const QMetaObject * QDoubleValidator::metaObject();
impl<'a> /*trait*/ QDoubleValidator_metaObject for () {
  fn metaObject(self, this: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator10metaObjectEv()};
    unsafe {_ZNK16QDoubleValidator10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn setTop<T: QDoubleValidator_setTop>(&mut self, value: T) -> i32 {
    value.setTop(self);
    return 1;
  }
}

pub trait QDoubleValidator_setTop {
  fn setTop(self, this: &mut QDoubleValidator) -> i32;
}

// proto: void QDoubleValidator::setTop(double );
impl<'a> /*trait*/ QDoubleValidator_setTop for (f64) {
  fn setTop(self, this: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator6setTopEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QDoubleValidator6setTopEd(arg0)};
    return 1;
  }
}

