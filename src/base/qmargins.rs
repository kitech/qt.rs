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
  fn _ZN8QMargins7setLeftEi(arg0: c_int) -> i32;
  fn _ZN8QMargins8setRightEi(arg0: c_int) -> i32;
  fn _ZNK8QMargins4leftEv() -> i32;
  fn _ZNK8QMargins3topEv() -> i32;
  fn _ZN8QMargins6setTopEi(arg0: c_int) -> i32;
  fn _ZN8QMargins9setBottomEi(arg0: c_int) -> i32;
  fn _ZNK8QMargins5rightEv() -> i32;
  fn _ZNK8QMargins6bottomEv() -> i32;
  fn _ZNK8QMargins6isNullEv() -> i32;
  fn _ZN8QMarginsC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN8QMarginsC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
}

// body block begin
// class sizeof(QMargins)=16
pub struct QMargins {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMargins {
  pub fn setLeft<T: QMargins_setLeft>(&mut self, value: T) -> i32 {
    value.setLeft(self);
    return 1;
  }
}

pub trait QMargins_setLeft {
  fn setLeft(self, this: &mut QMargins) -> i32;
}

// proto: void QMargins::setLeft(int left);
impl<'a> /*trait*/ QMargins_setLeft for (i32) {
  fn setLeft(self, this: &mut QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMargins7setLeftEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QMargins7setLeftEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn setRight<T: QMargins_setRight>(&mut self, value: T) -> i32 {
    value.setRight(self);
    return 1;
  }
}

pub trait QMargins_setRight {
  fn setRight(self, this: &mut QMargins) -> i32;
}

// proto: void QMargins::setRight(int right);
impl<'a> /*trait*/ QMargins_setRight for (i32) {
  fn setRight(self, this: &mut QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMargins8setRightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QMargins8setRightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn left<T: QMargins_left>(&mut self, value: T) -> i32 {
    value.left(self);
    return 1;
  }
}

pub trait QMargins_left {
  fn left(self, this: &mut QMargins) -> i32;
}

// proto: int QMargins::left();
impl<'a> /*trait*/ QMargins_left for () {
  fn left(self, this: &mut QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins4leftEv()};
    unsafe {_ZNK8QMargins4leftEv()};
    return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn top<T: QMargins_top>(&mut self, value: T) -> i32 {
    value.top(self);
    return 1;
  }
}

pub trait QMargins_top {
  fn top(self, this: &mut QMargins) -> i32;
}

// proto: int QMargins::top();
impl<'a> /*trait*/ QMargins_top for () {
  fn top(self, this: &mut QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins3topEv()};
    unsafe {_ZNK8QMargins3topEv()};
    return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn setTop<T: QMargins_setTop>(&mut self, value: T) -> i32 {
    value.setTop(self);
    return 1;
  }
}

pub trait QMargins_setTop {
  fn setTop(self, this: &mut QMargins) -> i32;
}

// proto: void QMargins::setTop(int top);
impl<'a> /*trait*/ QMargins_setTop for (i32) {
  fn setTop(self, this: &mut QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMargins6setTopEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QMargins6setTopEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn setBottom<T: QMargins_setBottom>(&mut self, value: T) -> i32 {
    value.setBottom(self);
    return 1;
  }
}

pub trait QMargins_setBottom {
  fn setBottom(self, this: &mut QMargins) -> i32;
}

// proto: void QMargins::setBottom(int bottom);
impl<'a> /*trait*/ QMargins_setBottom for (i32) {
  fn setBottom(self, this: &mut QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMargins9setBottomEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QMargins9setBottomEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn right<T: QMargins_right>(&mut self, value: T) -> i32 {
    value.right(self);
    return 1;
  }
}

pub trait QMargins_right {
  fn right(self, this: &mut QMargins) -> i32;
}

// proto: int QMargins::right();
impl<'a> /*trait*/ QMargins_right for () {
  fn right(self, this: &mut QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins5rightEv()};
    unsafe {_ZNK8QMargins5rightEv()};
    return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn bottom<T: QMargins_bottom>(&mut self, value: T) -> i32 {
    value.bottom(self);
    return 1;
  }
}

pub trait QMargins_bottom {
  fn bottom(self, this: &mut QMargins) -> i32;
}

// proto: int QMargins::bottom();
impl<'a> /*trait*/ QMargins_bottom for () {
  fn bottom(self, this: &mut QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins6bottomEv()};
    unsafe {_ZNK8QMargins6bottomEv()};
    return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn isNull<T: QMargins_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QMargins_isNull {
  fn isNull(self, this: &mut QMargins) -> i32;
}

// proto: bool QMargins::isNull();
impl<'a> /*trait*/ QMargins_isNull for () {
  fn isNull(self, this: &mut QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins6isNullEv()};
    unsafe {_ZNK8QMargins6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn NewQMargins<T: QMargins_NewQMargins>(value: T) -> QMargins {
    let rsthis = value.NewQMargins();
    return rsthis;
    // return 1;
  }
}

pub trait QMargins_NewQMargins {
  fn NewQMargins(self) -> QMargins;
}

// proto: void QMargins::NewQMargins();
impl<'a> /*trait*/ QMargins_NewQMargins for () {
  fn NewQMargins(self) -> QMargins {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMarginsC1Ev()};
    unsafe {_ZN8QMarginsC1Ev(qthis)};
    let rsthis = QMargins{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QMargins::NewQMargins(int left, int top, int right, int bottom);
impl<'a> /*trait*/ QMargins_NewQMargins for (i32, i32, i32, i32) {
  fn NewQMargins(self) -> QMargins {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMarginsC1Eiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN8QMarginsC1Eiiii(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QMargins{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

