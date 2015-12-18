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
  // proto:  int & QPoint::ry();
  fn _ZN6QPoint2ryEv(qthis: *mut c_void) ;
  // proto: static int QPoint::dotProduct(const QPoint & p1, const QPoint & p2);
  fn _ZN6QPoint10dotProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  int QPoint::x();
  fn _ZNK6QPoint1xEv(qthis: *mut c_void) ;
  // proto:  void QPoint::NewQPoint(int xpos, int ypos);
  fn _ZN6QPointC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  int QPoint::y();
  fn _ZNK6QPoint1yEv(qthis: *mut c_void) ;
  // proto:  void QPoint::setX(int x);
  fn _ZN6QPoint4setXEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QPoint::isNull();
  fn _ZNK6QPoint6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPoint::NewQPoint();
  fn _ZN6QPointC1Ev(qthis: *mut c_void) ;
  // proto:  void QPoint::setY(int y);
  fn _ZN6QPoint4setYEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int & QPoint::rx();
  fn _ZN6QPoint2rxEv(qthis: *mut c_void) ;
  // proto:  int QPoint::manhattanLength();
  fn _ZNK6QPoint15manhattanLengthEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QPoint)=8
pub struct QPoint {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPoint {
  pub fn ry<RetType, T: QPoint_ry<RetType>>(&mut self, value: T) -> RetType {
    return value.ry(self);
    // return 1;
  }
}

pub trait QPoint_ry<RetType> {
  fn ry(self, rsthis: &mut QPoint) -> RetType;
}

// proto:  int & QPoint::ry();
impl<'a> /*trait*/ QPoint_ry<()> for () {
  fn ry(self, rsthis: &mut QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint2ryEv()};
     unsafe {_ZN6QPoint2ryEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn dotProduct<RetType, T: QPoint_dotProduct<RetType>>(&mut self, value: T) -> RetType {
    return value.dotProduct(self);
    // return 1;
  }
}

pub trait QPoint_dotProduct<RetType> {
  fn dotProduct(self, rsthis: &mut QPoint) -> RetType;
}

// proto: static int QPoint::dotProduct(const QPoint & p1, const QPoint & p2);
impl<'a> /*trait*/ QPoint_dotProduct<i32> for (&'a  QPoint, &'a  QPoint) {
  fn dotProduct(self, rsthis: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN6QPoint10dotProductERKS_S1_(arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn x<RetType, T: QPoint_x<RetType>>(&mut self, value: T) -> RetType {
    return value.x(self);
    // return 1;
  }
}

pub trait QPoint_x<RetType> {
  fn x(self, rsthis: &mut QPoint) -> RetType;
}

// proto:  int QPoint::x();
impl<'a> /*trait*/ QPoint_x<()> for () {
  fn x(self, rsthis: &mut QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint1xEv()};
     unsafe {_ZNK6QPoint1xEv(rsthis.qclsinst)};
    // return 1;
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
  pub fn y<RetType, T: QPoint_y<RetType>>(&mut self, value: T) -> RetType {
    return value.y(self);
    // return 1;
  }
}

pub trait QPoint_y<RetType> {
  fn y(self, rsthis: &mut QPoint) -> RetType;
}

// proto:  int QPoint::y();
impl<'a> /*trait*/ QPoint_y<()> for () {
  fn y(self, rsthis: &mut QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint1yEv()};
     unsafe {_ZNK6QPoint1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn setX<RetType, T: QPoint_setX<RetType>>(&mut self, value: T) -> RetType {
    return value.setX(self);
    // return 1;
  }
}

pub trait QPoint_setX<RetType> {
  fn setX(self, rsthis: &mut QPoint) -> RetType;
}

// proto:  void QPoint::setX(int x);
impl<'a> /*trait*/ QPoint_setX<()> for (i32) {
  fn setX(self, rsthis: &mut QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint4setXEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QPoint4setXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn isNull<RetType, T: QPoint_isNull<RetType>>(&mut self, value: T) -> RetType {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QPoint_isNull<RetType> {
  fn isNull(self, rsthis: &mut QPoint) -> RetType;
}

// proto:  bool QPoint::isNull();
impl<'a> /*trait*/ QPoint_isNull<i8> for () {
  fn isNull(self, rsthis: &mut QPoint) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint6isNullEv()};
    let mut ret = unsafe {_ZNK6QPoint6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
  pub fn setY<RetType, T: QPoint_setY<RetType>>(&mut self, value: T) -> RetType {
    return value.setY(self);
    // return 1;
  }
}

pub trait QPoint_setY<RetType> {
  fn setY(self, rsthis: &mut QPoint) -> RetType;
}

// proto:  void QPoint::setY(int y);
impl<'a> /*trait*/ QPoint_setY<()> for (i32) {
  fn setY(self, rsthis: &mut QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint4setYEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QPoint4setYEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn rx<RetType, T: QPoint_rx<RetType>>(&mut self, value: T) -> RetType {
    return value.rx(self);
    // return 1;
  }
}

pub trait QPoint_rx<RetType> {
  fn rx(self, rsthis: &mut QPoint) -> RetType;
}

// proto:  int & QPoint::rx();
impl<'a> /*trait*/ QPoint_rx<()> for () {
  fn rx(self, rsthis: &mut QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint2rxEv()};
     unsafe {_ZN6QPoint2rxEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn manhattanLength<RetType, T: QPoint_manhattanLength<RetType>>(&mut self, value: T) -> RetType {
    return value.manhattanLength(self);
    // return 1;
  }
}

pub trait QPoint_manhattanLength<RetType> {
  fn manhattanLength(self, rsthis: &mut QPoint) -> RetType;
}

// proto:  int QPoint::manhattanLength();
impl<'a> /*trait*/ QPoint_manhattanLength<i32> for () {
  fn manhattanLength(self, rsthis: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint15manhattanLengthEv()};
    let mut ret = unsafe {_ZNK6QPoint15manhattanLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

