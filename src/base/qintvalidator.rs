// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QIntValidator::NewQIntValidator(QObject * parent);
  fn _ZN13QIntValidatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QIntValidator::setBottom(int );
  fn _ZN13QIntValidator9setBottomEi(arg0: c_int) -> i32;
  // proto: void QIntValidator::setRange(int bottom, int top);
  fn _ZN13QIntValidator8setRangeEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: const QMetaObject * QIntValidator::metaObject();
  fn _ZNK13QIntValidator10metaObjectEv() -> i32;
  // proto: void QIntValidator::NewQIntValidator(const QIntValidator & );
  fn _ZN13QIntValidatorC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QIntValidator::top();
  fn _ZNK13QIntValidator3topEv() -> i32;
  // proto: void QIntValidator::fixup(QString & input);
  fn _ZNK13QIntValidator5fixupER7QString(arg0: *mut c_void) -> i32;
  // proto: void QIntValidator::FreeQIntValidator();
  fn _ZN13QIntValidatorD0Ev() -> i32;
  // proto: void QIntValidator::bottomChanged(int bottom);
  fn _ZN13QIntValidator13bottomChangedEi(arg0: c_int) -> i32;
  // proto: void QIntValidator::topChanged(int top);
  fn _ZN13QIntValidator10topChangedEi(arg0: c_int) -> i32;
  // proto: void QIntValidator::setTop(int );
  fn _ZN13QIntValidator6setTopEi(arg0: c_int) -> i32;
  // proto: int QIntValidator::bottom();
  fn _ZNK13QIntValidator6bottomEv() -> i32;
  // proto: void QIntValidator::NewQIntValidator(int bottom, int top, QObject * parent);
  fn _ZN13QIntValidatorC1EiiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QIntValidator)=1
pub struct QIntValidator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QIntValidator {
  pub fn NewQIntValidator<T: QIntValidator_NewQIntValidator>(value: T) -> QIntValidator {
    let rsthis = value.NewQIntValidator();
    return rsthis;
    // return 1;
  }
}

pub trait QIntValidator_NewQIntValidator {
  fn NewQIntValidator(self) -> QIntValidator;
}

// proto: void QIntValidator::NewQIntValidator(QObject * parent);
impl<'a> /*trait*/ QIntValidator_NewQIntValidator for (&'a mut QObject) {
  fn NewQIntValidator(self) -> QIntValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidatorC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QIntValidatorC1EP7QObject(qthis, arg0)};
    let rsthis = QIntValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QIntValidator {
  pub fn setBottom<T: QIntValidator_setBottom>(&mut self, value: T) -> i32 {
    value.setBottom(self);
    return 1;
  }
}

pub trait QIntValidator_setBottom {
  fn setBottom(self, this: &mut QIntValidator) -> i32;
}

// proto: void QIntValidator::setBottom(int );
impl<'a> /*trait*/ QIntValidator_setBottom for (i32) {
  fn setBottom(self, this: &mut QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidator9setBottomEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QIntValidator9setBottomEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QIntValidator {
  pub fn setRange<T: QIntValidator_setRange>(&mut self, value: T) -> i32 {
    value.setRange(self);
    return 1;
  }
}

pub trait QIntValidator_setRange {
  fn setRange(self, this: &mut QIntValidator) -> i32;
}

// proto: void QIntValidator::setRange(int bottom, int top);
impl<'a> /*trait*/ QIntValidator_setRange for (i32, i32) {
  fn setRange(self, this: &mut QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidator8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN13QIntValidator8setRangeEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QIntValidator {
  pub fn metaObject<T: QIntValidator_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QIntValidator_metaObject {
  fn metaObject(self, this: &mut QIntValidator) -> i32;
}

// proto: const QMetaObject * QIntValidator::metaObject();
impl<'a> /*trait*/ QIntValidator_metaObject for () {
  fn metaObject(self, this: &mut QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QIntValidator10metaObjectEv()};
    unsafe {_ZNK13QIntValidator10metaObjectEv()};
    return 1;
  }
}

// proto: void QIntValidator::NewQIntValidator(const QIntValidator & );
impl<'a> /*trait*/ QIntValidator_NewQIntValidator for (&'a  QIntValidator) {
  fn NewQIntValidator(self) -> QIntValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidatorC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QIntValidatorC1ERKS_(qthis, arg0)};
    let rsthis = QIntValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QIntValidator {
  pub fn top<T: QIntValidator_top>(&mut self, value: T) -> i32 {
    value.top(self);
    return 1;
  }
}

pub trait QIntValidator_top {
  fn top(self, this: &mut QIntValidator) -> i32;
}

// proto: int QIntValidator::top();
impl<'a> /*trait*/ QIntValidator_top for () {
  fn top(self, this: &mut QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QIntValidator3topEv()};
    unsafe {_ZNK13QIntValidator3topEv()};
    return 1;
  }
}

impl /*struct*/ QIntValidator {
  pub fn fixup<T: QIntValidator_fixup>(&mut self, value: T) -> i32 {
    value.fixup(self);
    return 1;
  }
}

pub trait QIntValidator_fixup {
  fn fixup(self, this: &mut QIntValidator) -> i32;
}

// proto: void QIntValidator::fixup(QString & input);
impl<'a> /*trait*/ QIntValidator_fixup for (&'a mut QString) {
  fn fixup(self, this: &mut QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QIntValidator5fixupER7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK13QIntValidator5fixupER7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QIntValidator {
  pub fn FreeQIntValidator<T: QIntValidator_FreeQIntValidator>(&mut self, value: T) -> i32 {
    value.FreeQIntValidator(self);
    return 1;
  }
}

pub trait QIntValidator_FreeQIntValidator {
  fn FreeQIntValidator(self, this: &mut QIntValidator) -> i32;
}

// proto: void QIntValidator::FreeQIntValidator();
impl<'a> /*trait*/ QIntValidator_FreeQIntValidator for () {
  fn FreeQIntValidator(self, this: &mut QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidatorD0Ev()};
    unsafe {_ZN13QIntValidatorD0Ev()};
    return 1;
  }
}

impl /*struct*/ QIntValidator {
  pub fn bottomChanged<T: QIntValidator_bottomChanged>(&mut self, value: T) -> i32 {
    value.bottomChanged(self);
    return 1;
  }
}

pub trait QIntValidator_bottomChanged {
  fn bottomChanged(self, this: &mut QIntValidator) -> i32;
}

// proto: void QIntValidator::bottomChanged(int bottom);
impl<'a> /*trait*/ QIntValidator_bottomChanged for (i32) {
  fn bottomChanged(self, this: &mut QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidator13bottomChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QIntValidator13bottomChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QIntValidator {
  pub fn topChanged<T: QIntValidator_topChanged>(&mut self, value: T) -> i32 {
    value.topChanged(self);
    return 1;
  }
}

pub trait QIntValidator_topChanged {
  fn topChanged(self, this: &mut QIntValidator) -> i32;
}

// proto: void QIntValidator::topChanged(int top);
impl<'a> /*trait*/ QIntValidator_topChanged for (i32) {
  fn topChanged(self, this: &mut QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidator10topChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QIntValidator10topChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QIntValidator {
  pub fn setTop<T: QIntValidator_setTop>(&mut self, value: T) -> i32 {
    value.setTop(self);
    return 1;
  }
}

pub trait QIntValidator_setTop {
  fn setTop(self, this: &mut QIntValidator) -> i32;
}

// proto: void QIntValidator::setTop(int );
impl<'a> /*trait*/ QIntValidator_setTop for (i32) {
  fn setTop(self, this: &mut QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidator6setTopEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QIntValidator6setTopEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QIntValidator {
  pub fn bottom<T: QIntValidator_bottom>(&mut self, value: T) -> i32 {
    value.bottom(self);
    return 1;
  }
}

pub trait QIntValidator_bottom {
  fn bottom(self, this: &mut QIntValidator) -> i32;
}

// proto: int QIntValidator::bottom();
impl<'a> /*trait*/ QIntValidator_bottom for () {
  fn bottom(self, this: &mut QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QIntValidator6bottomEv()};
    unsafe {_ZNK13QIntValidator6bottomEv()};
    return 1;
  }
}

// proto: void QIntValidator::NewQIntValidator(int bottom, int top, QObject * parent);
impl<'a> /*trait*/ QIntValidator_NewQIntValidator for (i32, i32, &'a mut QObject) {
  fn NewQIntValidator(self) -> QIntValidator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QIntValidatorC1EiiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN13QIntValidatorC1EiiP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QIntValidator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

