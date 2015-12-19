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
  // proto:  QMargins QMarginsF::toMargins();
  fn _ZNK9QMarginsF9toMarginsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QMarginsF::right();
  fn _ZNK9QMarginsF5rightEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QMarginsF::isNull();
  fn _ZNK9QMarginsF6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMarginsF::setRight(qreal right);
  fn _ZN9QMarginsF8setRightEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QMarginsF::setTop(qreal top);
  fn _ZN9QMarginsF6setTopEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QMarginsF::left();
  fn _ZNK9QMarginsF4leftEv(qthis: *mut c_void) -> c_double;
  // proto:  void QMarginsF::NewQMarginsF();
  fn _ZN9QMarginsFC1Ev(qthis: *mut c_void) ;
  // proto:  void QMarginsF::NewQMarginsF(qreal left, qreal top, qreal right, qreal bottom);
  fn _ZN9QMarginsFC1Edddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  double QMarginsF::bottom();
  fn _ZNK9QMarginsF6bottomEv(qthis: *mut c_void) -> c_double;
  // proto:  void QMarginsF::NewQMarginsF(const QMargins & margins);
  fn _ZN9QMarginsFC1ERK8QMargins(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMarginsF::setBottom(qreal bottom);
  fn _ZN9QMarginsF9setBottomEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QMarginsF::top();
  fn _ZNK9QMarginsF3topEv(qthis: *mut c_void) -> c_double;
  // proto:  void QMarginsF::setLeft(qreal left);
  fn _ZN9QMarginsF7setLeftEd(qthis: *mut c_void, arg0: c_double) ;
}

// body block begin
// class sizeof(QMarginsF)=32
pub struct QMarginsF {
  pub qclsinst: *mut c_void,
}

// proto:  QMargins QMarginsF::toMargins();
impl /*struct*/ QMarginsF {
  pub fn toMargins<RetType, T: QMarginsF_toMargins<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toMargins(self);
    // return 1;
  }
}

pub trait QMarginsF_toMargins<RetType> {
  fn toMargins(self , rsthis: &mut QMarginsF) -> RetType;
}

// proto:  QMargins QMarginsF::toMargins();
impl<'a> /*trait*/ QMarginsF_toMargins<QMargins> for () {
  fn toMargins(self , rsthis: &mut QMarginsF) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF9toMarginsEv()};
    let mut ret = unsafe {_ZNK9QMarginsF9toMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMargins{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  double QMarginsF::right();
impl /*struct*/ QMarginsF {
  pub fn right<RetType, T: QMarginsF_right<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.right(self);
    // return 1;
  }
}

pub trait QMarginsF_right<RetType> {
  fn right(self , rsthis: &mut QMarginsF) -> RetType;
}

// proto:  double QMarginsF::right();
impl<'a> /*trait*/ QMarginsF_right<f64> for () {
  fn right(self , rsthis: &mut QMarginsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF5rightEv()};
    let mut ret = unsafe {_ZNK9QMarginsF5rightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  bool QMarginsF::isNull();
impl /*struct*/ QMarginsF {
  pub fn isNull<RetType, T: QMarginsF_isNull<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QMarginsF_isNull<RetType> {
  fn isNull(self , rsthis: &mut QMarginsF) -> RetType;
}

// proto:  bool QMarginsF::isNull();
impl<'a> /*trait*/ QMarginsF_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QMarginsF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF6isNullEv()};
    let mut ret = unsafe {_ZNK9QMarginsF6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QMarginsF::setRight(qreal right);
impl /*struct*/ QMarginsF {
  pub fn setRight<RetType, T: QMarginsF_setRight<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRight(self);
    // return 1;
  }
}

pub trait QMarginsF_setRight<RetType> {
  fn setRight(self , rsthis: &mut QMarginsF) -> RetType;
}

// proto:  void QMarginsF::setRight(qreal right);
impl<'a> /*trait*/ QMarginsF_setRight<()> for (f64) {
  fn setRight(self , rsthis: &mut QMarginsF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsF8setRightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN9QMarginsF8setRightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QMarginsF::setTop(qreal top);
impl /*struct*/ QMarginsF {
  pub fn setTop<RetType, T: QMarginsF_setTop<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setTop(self);
    // return 1;
  }
}

pub trait QMarginsF_setTop<RetType> {
  fn setTop(self , rsthis: &mut QMarginsF) -> RetType;
}

// proto:  void QMarginsF::setTop(qreal top);
impl<'a> /*trait*/ QMarginsF_setTop<()> for (f64) {
  fn setTop(self , rsthis: &mut QMarginsF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsF6setTopEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN9QMarginsF6setTopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  double QMarginsF::left();
impl /*struct*/ QMarginsF {
  pub fn left<RetType, T: QMarginsF_left<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.left(self);
    // return 1;
  }
}

pub trait QMarginsF_left<RetType> {
  fn left(self , rsthis: &mut QMarginsF) -> RetType;
}

// proto:  double QMarginsF::left();
impl<'a> /*trait*/ QMarginsF_left<f64> for () {
  fn left(self , rsthis: &mut QMarginsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF4leftEv()};
    let mut ret = unsafe {_ZNK9QMarginsF4leftEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
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

// proto:  double QMarginsF::bottom();
impl /*struct*/ QMarginsF {
  pub fn bottom<RetType, T: QMarginsF_bottom<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.bottom(self);
    // return 1;
  }
}

pub trait QMarginsF_bottom<RetType> {
  fn bottom(self , rsthis: &mut QMarginsF) -> RetType;
}

// proto:  double QMarginsF::bottom();
impl<'a> /*trait*/ QMarginsF_bottom<f64> for () {
  fn bottom(self , rsthis: &mut QMarginsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF6bottomEv()};
    let mut ret = unsafe {_ZNK9QMarginsF6bottomEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto: void QMarginsF::NewQMarginsF(const QMargins & margins);
impl<'a> /*trait*/ QMarginsF_NewQMarginsF for (&'a  QMargins) {
  fn NewQMarginsF(self) -> QMarginsF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsFC1ERK8QMargins()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QMarginsFC1ERK8QMargins(qthis, arg0)};
    let rsthis = QMarginsF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QMarginsF::setBottom(qreal bottom);
impl /*struct*/ QMarginsF {
  pub fn setBottom<RetType, T: QMarginsF_setBottom<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setBottom(self);
    // return 1;
  }
}

pub trait QMarginsF_setBottom<RetType> {
  fn setBottom(self , rsthis: &mut QMarginsF) -> RetType;
}

// proto:  void QMarginsF::setBottom(qreal bottom);
impl<'a> /*trait*/ QMarginsF_setBottom<()> for (f64) {
  fn setBottom(self , rsthis: &mut QMarginsF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsF9setBottomEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN9QMarginsF9setBottomEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  double QMarginsF::top();
impl /*struct*/ QMarginsF {
  pub fn top<RetType, T: QMarginsF_top<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.top(self);
    // return 1;
  }
}

pub trait QMarginsF_top<RetType> {
  fn top(self , rsthis: &mut QMarginsF) -> RetType;
}

// proto:  double QMarginsF::top();
impl<'a> /*trait*/ QMarginsF_top<f64> for () {
  fn top(self , rsthis: &mut QMarginsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMarginsF3topEv()};
    let mut ret = unsafe {_ZNK9QMarginsF3topEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QMarginsF::setLeft(qreal left);
impl /*struct*/ QMarginsF {
  pub fn setLeft<RetType, T: QMarginsF_setLeft<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLeft(self);
    // return 1;
  }
}

pub trait QMarginsF_setLeft<RetType> {
  fn setLeft(self , rsthis: &mut QMarginsF) -> RetType;
}

// proto:  void QMarginsF::setLeft(qreal left);
impl<'a> /*trait*/ QMarginsF_setLeft<()> for (f64) {
  fn setLeft(self , rsthis: &mut QMarginsF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMarginsF7setLeftEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN9QMarginsF7setLeftEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

