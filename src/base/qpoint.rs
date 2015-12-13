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
  fn _ZN6QPoint2ryEv() -> i32;
  fn _ZN6QPoint10dotProductERKS_S1_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK6QPoint1xEv() -> i32;
  fn _ZN6QPointC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK6QPoint1yEv() -> i32;
  fn _ZN6QPoint4setXEi(arg0: c_int) -> i32;
  fn _ZNK6QPoint6isNullEv() -> i32;
  fn _ZN6QPointC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN6QPoint4setYEi(arg0: c_int) -> i32;
  fn _ZN6QPoint2rxEv() -> i32;
  fn _ZNK6QPoint15manhattanLengthEv() -> i32;
}

// body block begin
// class sizeof(QPoint)=8
pub struct QPoint {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPoint {
  pub fn ry<T: QPoint_ry>(&mut self, value: T) -> i32 {
    value.ry(self);
    return 1;
  }
}

pub trait QPoint_ry {
  fn ry(self, this: &mut QPoint) -> i32;
}

// proto: int & QPoint::ry();
impl<'a> /*trait*/ QPoint_ry for () {
  fn ry(self, this: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint2ryEv()};
    unsafe {_ZN6QPoint2ryEv()};
    return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn dotProduct<T: QPoint_dotProduct>(&mut self, value: T) -> i32 {
    value.dotProduct(self);
    return 1;
  }
}

pub trait QPoint_dotProduct {
  fn dotProduct(self, this: &mut QPoint) -> i32;
}

// proto: int QPoint::dotProduct(const QPoint & p1, const QPoint & p2);
impl<'a> /*trait*/ QPoint_dotProduct for (&'a  QPoint, &'a  QPoint) {
  fn dotProduct(self, this: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN6QPoint10dotProductERKS_S1_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn x<T: QPoint_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QPoint_x {
  fn x(self, this: &mut QPoint) -> i32;
}

// proto: int QPoint::x();
impl<'a> /*trait*/ QPoint_x for () {
  fn x(self, this: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint1xEv()};
    unsafe {_ZNK6QPoint1xEv()};
    return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn NewQPoint<T: QPoint_NewQPoint>(value: T) -> QPoint {
    let rsthis = value.NewQPoint();
    return rsthis;
    // return 1;
  }
}

pub trait QPoint_NewQPoint {
  fn NewQPoint(self) -> QPoint;
}

// proto: void QPoint::NewQPoint(int xpos, int ypos);
impl<'a> /*trait*/ QPoint_NewQPoint for (i32, i32) {
  fn NewQPoint(self) -> QPoint {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPointC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN6QPointC1Eii(qthis, arg0, arg1)};
    let rsthis = QPoint{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn y<T: QPoint_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QPoint_y {
  fn y(self, this: &mut QPoint) -> i32;
}

// proto: int QPoint::y();
impl<'a> /*trait*/ QPoint_y for () {
  fn y(self, this: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint1yEv()};
    unsafe {_ZNK6QPoint1yEv()};
    return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn setX<T: QPoint_setX>(&mut self, value: T) -> i32 {
    value.setX(self);
    return 1;
  }
}

pub trait QPoint_setX {
  fn setX(self, this: &mut QPoint) -> i32;
}

// proto: void QPoint::setX(int x);
impl<'a> /*trait*/ QPoint_setX for (i32) {
  fn setX(self, this: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint4setXEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QPoint4setXEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn isNull<T: QPoint_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QPoint_isNull {
  fn isNull(self, this: &mut QPoint) -> i32;
}

// proto: bool QPoint::isNull();
impl<'a> /*trait*/ QPoint_isNull for () {
  fn isNull(self, this: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint6isNullEv()};
    unsafe {_ZNK6QPoint6isNullEv()};
    return 1;
  }
}

// proto: void QPoint::NewQPoint();
impl<'a> /*trait*/ QPoint_NewQPoint for () {
  fn NewQPoint(self) -> QPoint {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPointC1Ev()};
    unsafe {_ZN6QPointC1Ev(qthis)};
    let rsthis = QPoint{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn setY<T: QPoint_setY>(&mut self, value: T) -> i32 {
    value.setY(self);
    return 1;
  }
}

pub trait QPoint_setY {
  fn setY(self, this: &mut QPoint) -> i32;
}

// proto: void QPoint::setY(int y);
impl<'a> /*trait*/ QPoint_setY for (i32) {
  fn setY(self, this: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint4setYEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QPoint4setYEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn rx<T: QPoint_rx>(&mut self, value: T) -> i32 {
    value.rx(self);
    return 1;
  }
}

pub trait QPoint_rx {
  fn rx(self, this: &mut QPoint) -> i32;
}

// proto: int & QPoint::rx();
impl<'a> /*trait*/ QPoint_rx for () {
  fn rx(self, this: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint2rxEv()};
    unsafe {_ZN6QPoint2rxEv()};
    return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn manhattanLength<T: QPoint_manhattanLength>(&mut self, value: T) -> i32 {
    value.manhattanLength(self);
    return 1;
  }
}

pub trait QPoint_manhattanLength {
  fn manhattanLength(self, this: &mut QPoint) -> i32;
}

// proto: int QPoint::manhattanLength();
impl<'a> /*trait*/ QPoint_manhattanLength for () {
  fn manhattanLength(self, this: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint15manhattanLengthEv()};
    unsafe {_ZNK6QPoint15manhattanLengthEv()};
    return 1;
  }
}

