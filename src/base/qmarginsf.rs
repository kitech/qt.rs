// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmargins::QMargins;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK9QMarginsF9toMarginsEv() -> i32;
  fn _ZNK9QMarginsF5rightEv() -> i32;
  fn _ZNK9QMarginsF6isNullEv() -> i32;
  fn _ZN9QMarginsF8setRightEd(arg0: c_double) -> i32;
  fn _ZN9QMarginsF6setTopEd(arg0: c_double) -> i32;
  fn _ZNK9QMarginsF4leftEv() -> i32;
  fn _ZN9QMarginsFC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN9QMarginsFC1Edddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  fn _ZNK9QMarginsF6bottomEv() -> i32;
  fn _ZN9QMarginsFC1ERK8QMargins(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN9QMarginsF9setBottomEd(arg0: c_double) -> i32;
  fn _ZNK9QMarginsF3topEv() -> i32;
  fn _ZN9QMarginsF7setLeftEd(arg0: c_double) -> i32;
}

// body block begin
// class sizeof(QMarginsF)=32
pub struct QMarginsF {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMarginsF {
  pub fn toMargins<T: QMarginsF_toMargins>(&mut self, value: T) -> i32 {
    value.toMargins(self);
    return 1;
  }
}

pub trait QMarginsF_toMargins {
  fn toMargins(self, this: &mut QMarginsF) -> i32;
}

// proto: QMargins QMarginsF::toMargins();
impl<'a> /*trait*/ QMarginsF_toMargins for () {
  fn toMargins(self, this: &mut QMarginsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF9toMarginsEv()};
    unsafe {_ZNK9QMarginsF9toMarginsEv()};
    return 1;
  }
}

impl /*struct*/ QMarginsF {
  pub fn right<T: QMarginsF_right>(&mut self, value: T) -> i32 {
    value.right(self);
    return 1;
  }
}

pub trait QMarginsF_right {
  fn right(self, this: &mut QMarginsF) -> i32;
}

// proto: double QMarginsF::right();
impl<'a> /*trait*/ QMarginsF_right for () {
  fn right(self, this: &mut QMarginsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF5rightEv()};
    unsafe {_ZNK9QMarginsF5rightEv()};
    return 1;
  }
}

impl /*struct*/ QMarginsF {
  pub fn isNull<T: QMarginsF_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QMarginsF_isNull {
  fn isNull(self, this: &mut QMarginsF) -> i32;
}

// proto: bool QMarginsF::isNull();
impl<'a> /*trait*/ QMarginsF_isNull for () {
  fn isNull(self, this: &mut QMarginsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF6isNullEv()};
    unsafe {_ZNK9QMarginsF6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QMarginsF {
  pub fn setRight<T: QMarginsF_setRight>(&mut self, value: T) -> i32 {
    value.setRight(self);
    return 1;
  }
}

pub trait QMarginsF_setRight {
  fn setRight(self, this: &mut QMarginsF) -> i32;
}

// proto: void QMarginsF::setRight(qreal right);
impl<'a> /*trait*/ QMarginsF_setRight for (f64) {
  fn setRight(self, this: &mut QMarginsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsF8setRightEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN9QMarginsF8setRightEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QMarginsF {
  pub fn setTop<T: QMarginsF_setTop>(&mut self, value: T) -> i32 {
    value.setTop(self);
    return 1;
  }
}

pub trait QMarginsF_setTop {
  fn setTop(self, this: &mut QMarginsF) -> i32;
}

// proto: void QMarginsF::setTop(qreal top);
impl<'a> /*trait*/ QMarginsF_setTop for (f64) {
  fn setTop(self, this: &mut QMarginsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsF6setTopEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN9QMarginsF6setTopEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QMarginsF {
  pub fn left<T: QMarginsF_left>(&mut self, value: T) -> i32 {
    value.left(self);
    return 1;
  }
}

pub trait QMarginsF_left {
  fn left(self, this: &mut QMarginsF) -> i32;
}

// proto: double QMarginsF::left();
impl<'a> /*trait*/ QMarginsF_left for () {
  fn left(self, this: &mut QMarginsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF4leftEv()};
    unsafe {_ZNK9QMarginsF4leftEv()};
    return 1;
  }
}

impl /*struct*/ QMarginsF {
  pub fn NewQMarginsF<T: QMarginsF_NewQMarginsF>(value: T) -> QMarginsF {
    let rsthis = value.NewQMarginsF();
    return rsthis;
    // return 1;
  }
}

pub trait QMarginsF_NewQMarginsF {
  fn NewQMarginsF(self) -> QMarginsF;
}

// proto: void QMarginsF::NewQMarginsF();
impl<'a> /*trait*/ QMarginsF_NewQMarginsF for () {
  fn NewQMarginsF(self) -> QMarginsF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsFC1Ev()};
    unsafe {_ZN9QMarginsFC1Ev(qthis)};
    let rsthis = QMarginsF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QMarginsF::NewQMarginsF(qreal left, qreal top, qreal right, qreal bottom);
impl<'a> /*trait*/ QMarginsF_NewQMarginsF for (f64, f64, f64, f64) {
  fn NewQMarginsF(self) -> QMarginsF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsFC1Edddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN9QMarginsFC1Edddd(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QMarginsF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMarginsF {
  pub fn bottom<T: QMarginsF_bottom>(&mut self, value: T) -> i32 {
    value.bottom(self);
    return 1;
  }
}

pub trait QMarginsF_bottom {
  fn bottom(self, this: &mut QMarginsF) -> i32;
}

// proto: double QMarginsF::bottom();
impl<'a> /*trait*/ QMarginsF_bottom for () {
  fn bottom(self, this: &mut QMarginsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF6bottomEv()};
    unsafe {_ZNK9QMarginsF6bottomEv()};
    return 1;
  }
}

// proto: void QMarginsF::NewQMarginsF(const QMargins & margins);
impl<'a> /*trait*/ QMarginsF_NewQMarginsF for (&'a  QMargins) {
  fn NewQMarginsF(self) -> QMarginsF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsFC1ERK8QMargins()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QMarginsFC1ERK8QMargins(qthis, arg0)};
    let rsthis = QMarginsF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMarginsF {
  pub fn setBottom<T: QMarginsF_setBottom>(&mut self, value: T) -> i32 {
    value.setBottom(self);
    return 1;
  }
}

pub trait QMarginsF_setBottom {
  fn setBottom(self, this: &mut QMarginsF) -> i32;
}

// proto: void QMarginsF::setBottom(qreal bottom);
impl<'a> /*trait*/ QMarginsF_setBottom for (f64) {
  fn setBottom(self, this: &mut QMarginsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsF9setBottomEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN9QMarginsF9setBottomEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QMarginsF {
  pub fn top<T: QMarginsF_top>(&mut self, value: T) -> i32 {
    value.top(self);
    return 1;
  }
}

pub trait QMarginsF_top {
  fn top(self, this: &mut QMarginsF) -> i32;
}

// proto: double QMarginsF::top();
impl<'a> /*trait*/ QMarginsF_top for () {
  fn top(self, this: &mut QMarginsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF3topEv()};
    unsafe {_ZNK9QMarginsF3topEv()};
    return 1;
  }
}

impl /*struct*/ QMarginsF {
  pub fn setLeft<T: QMarginsF_setLeft>(&mut self, value: T) -> i32 {
    value.setLeft(self);
    return 1;
  }
}

pub trait QMarginsF_setLeft {
  fn setLeft(self, this: &mut QMarginsF) -> i32;
}

// proto: void QMarginsF::setLeft(qreal left);
impl<'a> /*trait*/ QMarginsF_setLeft for (f64) {
  fn setLeft(self, this: &mut QMarginsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsF7setLeftEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN9QMarginsF7setLeftEd(arg0)};
    return 1;
  }
}

