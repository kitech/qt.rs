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
  // proto:  int QDoubleValidator::decimals();
  fn _ZNK16QDoubleValidator8decimalsEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDoubleValidator::decimalsChanged(int decimals);
  fn _ZN16QDoubleValidator15decimalsChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QDoubleValidator::FreeQDoubleValidator();
  fn _ZN16QDoubleValidatorD0Ev(qthis: *mut c_void) ;
  // proto:  double QDoubleValidator::top();
  fn _ZNK16QDoubleValidator3topEv(qthis: *mut c_void) -> c_double;
  // proto:  void QDoubleValidator::bottomChanged(double bottom);
  fn _ZN16QDoubleValidator13bottomChangedEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QDoubleValidator::bottom();
  fn _ZNK16QDoubleValidator6bottomEv(qthis: *mut c_void) -> c_double;
  // proto:  void QDoubleValidator::setDecimals(int );
  fn _ZN16QDoubleValidator11setDecimalsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QDoubleValidator::NewQDoubleValidator(const QDoubleValidator & );
  fn _ZN16QDoubleValidatorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDoubleValidator::setBottom(double );
  fn _ZN16QDoubleValidator9setBottomEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QDoubleValidator::setRange(double bottom, double top, int decimals);
  fn _ZN16QDoubleValidator8setRangeEddi(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_int) ;
  // proto:  void QDoubleValidator::NewQDoubleValidator(QObject * parent);
  fn _ZN16QDoubleValidatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDoubleValidator::NewQDoubleValidator(double bottom, double top, int decimals, QObject * parent);
  fn _ZN16QDoubleValidatorC1EddiP7QObject(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_int, arg3: *mut c_void) ;
  // proto:  void QDoubleValidator::topChanged(double top);
  fn _ZN16QDoubleValidator10topChangedEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  const QMetaObject * QDoubleValidator::metaObject();
  fn _ZNK16QDoubleValidator10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QDoubleValidator::setTop(double );
  fn _ZN16QDoubleValidator6setTopEd(qthis: *mut c_void, arg0: c_double) ;
}

// body block begin
// class sizeof(QDoubleValidator)=1
pub struct QDoubleValidator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDoubleValidator {
  pub fn decimals<RetType, T: QDoubleValidator_decimals<RetType>>(&mut self, value: T) -> RetType {
    return value.decimals(self);
    // return 1;
  }
}

pub trait QDoubleValidator_decimals<RetType> {
  fn decimals(self, rsthis: &mut QDoubleValidator) -> RetType;
}

// proto:  int QDoubleValidator::decimals();
impl<'a> /*trait*/ QDoubleValidator_decimals<i32> for () {
  fn decimals(self, rsthis: &mut QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator8decimalsEv()};
    let mut ret = unsafe {_ZNK16QDoubleValidator8decimalsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn decimalsChanged<RetType, T: QDoubleValidator_decimalsChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.decimalsChanged(self);
    // return 1;
  }
}

pub trait QDoubleValidator_decimalsChanged<RetType> {
  fn decimalsChanged(self, rsthis: &mut QDoubleValidator) -> RetType;
}

// proto:  void QDoubleValidator::decimalsChanged(int decimals);
impl<'a> /*trait*/ QDoubleValidator_decimalsChanged<()> for (i32) {
  fn decimalsChanged(self, rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator15decimalsChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QDoubleValidator15decimalsChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn FreeQDoubleValidator<RetType, T: QDoubleValidator_FreeQDoubleValidator<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQDoubleValidator(self);
    // return 1;
  }
}

pub trait QDoubleValidator_FreeQDoubleValidator<RetType> {
  fn FreeQDoubleValidator(self, rsthis: &mut QDoubleValidator) -> RetType;
}

// proto:  void QDoubleValidator::FreeQDoubleValidator();
impl<'a> /*trait*/ QDoubleValidator_FreeQDoubleValidator<()> for () {
  fn FreeQDoubleValidator(self, rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidatorD0Ev()};
     unsafe {_ZN16QDoubleValidatorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn top<RetType, T: QDoubleValidator_top<RetType>>(&mut self, value: T) -> RetType {
    return value.top(self);
    // return 1;
  }
}

pub trait QDoubleValidator_top<RetType> {
  fn top(self, rsthis: &mut QDoubleValidator) -> RetType;
}

// proto:  double QDoubleValidator::top();
impl<'a> /*trait*/ QDoubleValidator_top<f64> for () {
  fn top(self, rsthis: &mut QDoubleValidator) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator3topEv()};
    let mut ret = unsafe {_ZNK16QDoubleValidator3topEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn bottomChanged<RetType, T: QDoubleValidator_bottomChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.bottomChanged(self);
    // return 1;
  }
}

pub trait QDoubleValidator_bottomChanged<RetType> {
  fn bottomChanged(self, rsthis: &mut QDoubleValidator) -> RetType;
}

// proto:  void QDoubleValidator::bottomChanged(double bottom);
impl<'a> /*trait*/ QDoubleValidator_bottomChanged<()> for (f64) {
  fn bottomChanged(self, rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator13bottomChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QDoubleValidator13bottomChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn bottom<RetType, T: QDoubleValidator_bottom<RetType>>(&mut self, value: T) -> RetType {
    return value.bottom(self);
    // return 1;
  }
}

pub trait QDoubleValidator_bottom<RetType> {
  fn bottom(self, rsthis: &mut QDoubleValidator) -> RetType;
}

// proto:  double QDoubleValidator::bottom();
impl<'a> /*trait*/ QDoubleValidator_bottom<f64> for () {
  fn bottom(self, rsthis: &mut QDoubleValidator) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator6bottomEv()};
    let mut ret = unsafe {_ZNK16QDoubleValidator6bottomEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn setDecimals<RetType, T: QDoubleValidator_setDecimals<RetType>>(&mut self, value: T) -> RetType {
    return value.setDecimals(self);
    // return 1;
  }
}

pub trait QDoubleValidator_setDecimals<RetType> {
  fn setDecimals(self, rsthis: &mut QDoubleValidator) -> RetType;
}

// proto:  void QDoubleValidator::setDecimals(int );
impl<'a> /*trait*/ QDoubleValidator_setDecimals<()> for (i32) {
  fn setDecimals(self, rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator11setDecimalsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QDoubleValidator11setDecimalsEi(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QDoubleValidatorC1ERKS_(qthis, arg0)};
    let rsthis = QDoubleValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn setBottom<RetType, T: QDoubleValidator_setBottom<RetType>>(&mut self, value: T) -> RetType {
    return value.setBottom(self);
    // return 1;
  }
}

pub trait QDoubleValidator_setBottom<RetType> {
  fn setBottom(self, rsthis: &mut QDoubleValidator) -> RetType;
}

// proto:  void QDoubleValidator::setBottom(double );
impl<'a> /*trait*/ QDoubleValidator_setBottom<()> for (f64) {
  fn setBottom(self, rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator9setBottomEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QDoubleValidator9setBottomEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn setRange<RetType, T: QDoubleValidator_setRange<RetType>>(&mut self, value: T) -> RetType {
    return value.setRange(self);
    // return 1;
  }
}

pub trait QDoubleValidator_setRange<RetType> {
  fn setRange(self, rsthis: &mut QDoubleValidator) -> RetType;
}

// proto:  void QDoubleValidator::setRange(double bottom, double top, int decimals);
impl<'a> /*trait*/ QDoubleValidator_setRange<()> for (f64, f64, i32) {
  fn setRange(self, rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator8setRangeEddi()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_int;
     unsafe {_ZN16QDoubleValidator8setRangeEddi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
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
  pub fn topChanged<RetType, T: QDoubleValidator_topChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.topChanged(self);
    // return 1;
  }
}

pub trait QDoubleValidator_topChanged<RetType> {
  fn topChanged(self, rsthis: &mut QDoubleValidator) -> RetType;
}

// proto:  void QDoubleValidator::topChanged(double top);
impl<'a> /*trait*/ QDoubleValidator_topChanged<()> for (f64) {
  fn topChanged(self, rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator10topChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QDoubleValidator10topChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn metaObject<RetType, T: QDoubleValidator_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QDoubleValidator_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QDoubleValidator) -> RetType;
}

// proto:  const QMetaObject * QDoubleValidator::metaObject();
impl<'a> /*trait*/ QDoubleValidator_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDoubleValidator10metaObjectEv()};
     unsafe {_ZNK16QDoubleValidator10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDoubleValidator {
  pub fn setTop<RetType, T: QDoubleValidator_setTop<RetType>>(&mut self, value: T) -> RetType {
    return value.setTop(self);
    // return 1;
  }
}

pub trait QDoubleValidator_setTop<RetType> {
  fn setTop(self, rsthis: &mut QDoubleValidator) -> RetType;
}

// proto:  void QDoubleValidator::setTop(double );
impl<'a> /*trait*/ QDoubleValidator_setTop<()> for (f64) {
  fn setTop(self, rsthis: &mut QDoubleValidator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDoubleValidator6setTopEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QDoubleValidator6setTopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

