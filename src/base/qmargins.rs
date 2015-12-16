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
  // proto:  void QMargins::setLeft(int left);
  fn _ZN8QMargins7setLeftEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QMargins::setRight(int right);
  fn _ZN8QMargins8setRightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QMargins::left();
  fn _ZNK8QMargins4leftEv(qthis: *mut c_void) -> c_int;
  // proto:  int QMargins::top();
  fn _ZNK8QMargins3topEv(qthis: *mut c_void) -> c_int;
  // proto:  void QMargins::setTop(int top);
  fn _ZN8QMargins6setTopEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QMargins::setBottom(int bottom);
  fn _ZN8QMargins9setBottomEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QMargins::right();
  fn _ZNK8QMargins5rightEv(qthis: *mut c_void) -> c_int;
  // proto:  int QMargins::bottom();
  fn _ZNK8QMargins6bottomEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QMargins::isNull();
  fn _ZNK8QMargins6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMargins::NewQMargins();
  fn _ZN8QMarginsC1Ev(qthis: *mut c_void) ;
  // proto:  void QMargins::NewQMargins(int left, int top, int right, int bottom);
  fn _ZN8QMarginsC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
}

// body block begin
// class sizeof(QMargins)=16
pub struct QMargins {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMargins {
  pub fn setLeft<T: QMargins_setLeft>(&mut self, value: T)  {
     value.setLeft(self);
    // return 1;
  }
}

pub trait QMargins_setLeft {
  fn setLeft(self, rsthis: &mut QMargins) ;
}

// proto:  void QMargins::setLeft(int left);
impl<'a> /*trait*/ QMargins_setLeft for (i32) {
  fn setLeft(self, rsthis: &mut QMargins)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMargins7setLeftEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QMargins7setLeftEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn setRight<T: QMargins_setRight>(&mut self, value: T)  {
     value.setRight(self);
    // return 1;
  }
}

pub trait QMargins_setRight {
  fn setRight(self, rsthis: &mut QMargins) ;
}

// proto:  void QMargins::setRight(int right);
impl<'a> /*trait*/ QMargins_setRight for (i32) {
  fn setRight(self, rsthis: &mut QMargins)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMargins8setRightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QMargins8setRightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn left<T: QMargins_left>(&mut self, value: T) -> i32 {
    return value.left(self);
    // return 1;
  }
}

pub trait QMargins_left {
  fn left(self, rsthis: &mut QMargins) -> i32;
}

// proto:  int QMargins::left();
impl<'a> /*trait*/ QMargins_left for () {
  fn left(self, rsthis: &mut QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins4leftEv()};
    let mut ret = unsafe {_ZNK8QMargins4leftEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn top<T: QMargins_top>(&mut self, value: T) -> i32 {
    return value.top(self);
    // return 1;
  }
}

pub trait QMargins_top {
  fn top(self, rsthis: &mut QMargins) -> i32;
}

// proto:  int QMargins::top();
impl<'a> /*trait*/ QMargins_top for () {
  fn top(self, rsthis: &mut QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins3topEv()};
    let mut ret = unsafe {_ZNK8QMargins3topEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn setTop<T: QMargins_setTop>(&mut self, value: T)  {
     value.setTop(self);
    // return 1;
  }
}

pub trait QMargins_setTop {
  fn setTop(self, rsthis: &mut QMargins) ;
}

// proto:  void QMargins::setTop(int top);
impl<'a> /*trait*/ QMargins_setTop for (i32) {
  fn setTop(self, rsthis: &mut QMargins)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMargins6setTopEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QMargins6setTopEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn setBottom<T: QMargins_setBottom>(&mut self, value: T)  {
     value.setBottom(self);
    // return 1;
  }
}

pub trait QMargins_setBottom {
  fn setBottom(self, rsthis: &mut QMargins) ;
}

// proto:  void QMargins::setBottom(int bottom);
impl<'a> /*trait*/ QMargins_setBottom for (i32) {
  fn setBottom(self, rsthis: &mut QMargins)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMargins9setBottomEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QMargins9setBottomEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn right<T: QMargins_right>(&mut self, value: T) -> i32 {
    return value.right(self);
    // return 1;
  }
}

pub trait QMargins_right {
  fn right(self, rsthis: &mut QMargins) -> i32;
}

// proto:  int QMargins::right();
impl<'a> /*trait*/ QMargins_right for () {
  fn right(self, rsthis: &mut QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins5rightEv()};
    let mut ret = unsafe {_ZNK8QMargins5rightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn bottom<T: QMargins_bottom>(&mut self, value: T) -> i32 {
    return value.bottom(self);
    // return 1;
  }
}

pub trait QMargins_bottom {
  fn bottom(self, rsthis: &mut QMargins) -> i32;
}

// proto:  int QMargins::bottom();
impl<'a> /*trait*/ QMargins_bottom for () {
  fn bottom(self, rsthis: &mut QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins6bottomEv()};
    let mut ret = unsafe {_ZNK8QMargins6bottomEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMargins {
  pub fn isNull<T: QMargins_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QMargins_isNull {
  fn isNull(self, rsthis: &mut QMargins) -> i8;
}

// proto:  bool QMargins::isNull();
impl<'a> /*trait*/ QMargins_isNull for () {
  fn isNull(self, rsthis: &mut QMargins) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMargins6isNullEv()};
    let mut ret = unsafe {_ZNK8QMargins6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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

