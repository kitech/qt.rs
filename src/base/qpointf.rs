// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN7QPointFC1Edd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> i32;
  fn _ZN7QPointFC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK7QPointF15manhattanLengthEv() -> i32;
  fn _ZNK7QPointF7toPointEv() -> i32;
  fn _ZN7QPointF2rxEv() -> i32;
  fn _ZNK7QPointF1yEv() -> i32;
  fn _ZNK7QPointF6isNullEv() -> i32;
  fn _ZNK7QPointF1xEv() -> i32;
  fn _ZN7QPointFC1ERK6QPoint(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN7QPointF4setXEd(arg0: c_double) -> i32;
  fn _ZN7QPointF2ryEv() -> i32;
  fn _ZN7QPointF10dotProductERKS_S1_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN7QPointF4setYEd(arg0: c_double) -> i32;
}

// body block begin
// class sizeof(QPointF)=16
pub struct QPointF {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPointF {
  pub fn NewQPointF<T: QPointF_NewQPointF>(value: T) -> QPointF {
    let rsthis = value.NewQPointF();
    return rsthis;
    // return 1;
  }
}

pub trait QPointF_NewQPointF {
  fn NewQPointF(self) -> QPointF;
}

// proto: void QPointF::NewQPointF(qreal xpos, qreal ypos);
impl<'a> /*trait*/ QPointF_NewQPointF for (f64, f64) {
  fn NewQPointF(self) -> QPointF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointFC1Edd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN7QPointFC1Edd(qthis, arg0, arg1)};
    let rsthis = QPointF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPointF::NewQPointF();
impl<'a> /*trait*/ QPointF_NewQPointF for () {
  fn NewQPointF(self) -> QPointF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointFC1Ev()};
    unsafe {_ZN7QPointFC1Ev(qthis)};
    let rsthis = QPointF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn manhattanLength<T: QPointF_manhattanLength>(&mut self, value: T) -> i32 {
    value.manhattanLength(self);
    return 1;
  }
}

pub trait QPointF_manhattanLength {
  fn manhattanLength(self, this: &mut QPointF) -> i32;
}

// proto: double QPointF::manhattanLength();
impl<'a> /*trait*/ QPointF_manhattanLength for () {
  fn manhattanLength(self, this: &mut QPointF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF15manhattanLengthEv()};
    unsafe {_ZNK7QPointF15manhattanLengthEv()};
    return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn toPoint<T: QPointF_toPoint>(&mut self, value: T) -> i32 {
    value.toPoint(self);
    return 1;
  }
}

pub trait QPointF_toPoint {
  fn toPoint(self, this: &mut QPointF) -> i32;
}

// proto: QPoint QPointF::toPoint();
impl<'a> /*trait*/ QPointF_toPoint for () {
  fn toPoint(self, this: &mut QPointF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF7toPointEv()};
    unsafe {_ZNK7QPointF7toPointEv()};
    return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn rx<T: QPointF_rx>(&mut self, value: T) -> i32 {
    value.rx(self);
    return 1;
  }
}

pub trait QPointF_rx {
  fn rx(self, this: &mut QPointF) -> i32;
}

// proto: qreal & QPointF::rx();
impl<'a> /*trait*/ QPointF_rx for () {
  fn rx(self, this: &mut QPointF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF2rxEv()};
    unsafe {_ZN7QPointF2rxEv()};
    return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn y<T: QPointF_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QPointF_y {
  fn y(self, this: &mut QPointF) -> i32;
}

// proto: double QPointF::y();
impl<'a> /*trait*/ QPointF_y for () {
  fn y(self, this: &mut QPointF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF1yEv()};
    unsafe {_ZNK7QPointF1yEv()};
    return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn isNull<T: QPointF_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QPointF_isNull {
  fn isNull(self, this: &mut QPointF) -> i32;
}

// proto: bool QPointF::isNull();
impl<'a> /*trait*/ QPointF_isNull for () {
  fn isNull(self, this: &mut QPointF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF6isNullEv()};
    unsafe {_ZNK7QPointF6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn x<T: QPointF_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QPointF_x {
  fn x(self, this: &mut QPointF) -> i32;
}

// proto: double QPointF::x();
impl<'a> /*trait*/ QPointF_x for () {
  fn x(self, this: &mut QPointF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF1xEv()};
    unsafe {_ZNK7QPointF1xEv()};
    return 1;
  }
}

// proto: void QPointF::NewQPointF(const QPoint & p);
impl<'a> /*trait*/ QPointF_NewQPointF for (&'a  QPoint) {
  fn NewQPointF(self) -> QPointF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointFC1ERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QPointFC1ERK6QPoint(qthis, arg0)};
    let rsthis = QPointF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn setX<T: QPointF_setX>(&mut self, value: T) -> i32 {
    value.setX(self);
    return 1;
  }
}

pub trait QPointF_setX {
  fn setX(self, this: &mut QPointF) -> i32;
}

// proto: void QPointF::setX(qreal x);
impl<'a> /*trait*/ QPointF_setX for (f64) {
  fn setX(self, this: &mut QPointF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF4setXEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN7QPointF4setXEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn ry<T: QPointF_ry>(&mut self, value: T) -> i32 {
    value.ry(self);
    return 1;
  }
}

pub trait QPointF_ry {
  fn ry(self, this: &mut QPointF) -> i32;
}

// proto: qreal & QPointF::ry();
impl<'a> /*trait*/ QPointF_ry for () {
  fn ry(self, this: &mut QPointF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF2ryEv()};
    unsafe {_ZN7QPointF2ryEv()};
    return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn dotProduct<T: QPointF_dotProduct>(&mut self, value: T) -> i32 {
    value.dotProduct(self);
    return 1;
  }
}

pub trait QPointF_dotProduct {
  fn dotProduct(self, this: &mut QPointF) -> i32;
}

// proto: double QPointF::dotProduct(const QPointF & p1, const QPointF & p2);
impl<'a> /*trait*/ QPointF_dotProduct for (&'a  QPointF, &'a  QPointF) {
  fn dotProduct(self, this: &mut QPointF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QPointF10dotProductERKS_S1_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn setY<T: QPointF_setY>(&mut self, value: T) -> i32 {
    value.setY(self);
    return 1;
  }
}

pub trait QPointF_setY {
  fn setY(self, this: &mut QPointF) -> i32;
}

// proto: void QPointF::setY(qreal y);
impl<'a> /*trait*/ QPointF_setY for (f64) {
  fn setY(self, this: &mut QPointF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF4setYEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN7QPointF4setYEd(arg0)};
    return 1;
  }
}

