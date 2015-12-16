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
  // proto:  void QPointF::NewQPointF(qreal xpos, qreal ypos);
  fn _ZN7QPointFC1Edd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QPointF::NewQPointF();
  fn _ZN7QPointFC1Ev(qthis: *mut c_void) ;
  // proto:  double QPointF::manhattanLength();
  fn _ZNK7QPointF15manhattanLengthEv(qthis: *mut c_void) -> c_double;
  // proto:  QPoint QPointF::toPoint();
  fn _ZNK7QPointF7toPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal & QPointF::rx();
  fn _ZN7QPointF2rxEv(qthis: *mut c_void) ;
  // proto:  double QPointF::y();
  fn _ZNK7QPointF1yEv(qthis: *mut c_void) ;
  // proto:  bool QPointF::isNull();
  fn _ZNK7QPointF6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  double QPointF::x();
  fn _ZNK7QPointF1xEv(qthis: *mut c_void) ;
  // proto:  void QPointF::NewQPointF(const QPoint & p);
  fn _ZN7QPointFC1ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPointF::setX(qreal x);
  fn _ZN7QPointF4setXEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  qreal & QPointF::ry();
  fn _ZN7QPointF2ryEv(qthis: *mut c_void) ;
  // proto: static double QPointF::dotProduct(const QPointF & p1, const QPointF & p2);
  fn _ZN7QPointF10dotProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_double;
  // proto:  void QPointF::setY(qreal y);
  fn _ZN7QPointF4setYEd(qthis: *mut c_void, arg0: c_double) ;
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
  pub fn manhattanLength<T: QPointF_manhattanLength>(&mut self, value: T) -> f64 {
    return value.manhattanLength(self);
    // return 1;
  }
}

pub trait QPointF_manhattanLength {
  fn manhattanLength(self, rsthis: &mut QPointF) -> f64;
}

// proto:  double QPointF::manhattanLength();
impl<'a> /*trait*/ QPointF_manhattanLength for () {
  fn manhattanLength(self, rsthis: &mut QPointF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF15manhattanLengthEv()};
    let mut ret = unsafe {_ZNK7QPointF15manhattanLengthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn toPoint<T: QPointF_toPoint>(&mut self, value: T) -> QPoint {
    return value.toPoint(self);
    // return 1;
  }
}

pub trait QPointF_toPoint {
  fn toPoint(self, rsthis: &mut QPointF) -> QPoint;
}

// proto:  QPoint QPointF::toPoint();
impl<'a> /*trait*/ QPointF_toPoint for () {
  fn toPoint(self, rsthis: &mut QPointF) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF7toPointEv()};
    let mut ret = unsafe {_ZNK7QPointF7toPointEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn rx<T: QPointF_rx>(&mut self, value: T)  {
     value.rx(self);
    // return 1;
  }
}

pub trait QPointF_rx {
  fn rx(self, rsthis: &mut QPointF) ;
}

// proto:  qreal & QPointF::rx();
impl<'a> /*trait*/ QPointF_rx for () {
  fn rx(self, rsthis: &mut QPointF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF2rxEv()};
     unsafe {_ZN7QPointF2rxEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn y<T: QPointF_y>(&mut self, value: T)  {
     value.y(self);
    // return 1;
  }
}

pub trait QPointF_y {
  fn y(self, rsthis: &mut QPointF) ;
}

// proto:  double QPointF::y();
impl<'a> /*trait*/ QPointF_y for () {
  fn y(self, rsthis: &mut QPointF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF1yEv()};
     unsafe {_ZNK7QPointF1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn isNull<T: QPointF_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QPointF_isNull {
  fn isNull(self, rsthis: &mut QPointF) -> i8;
}

// proto:  bool QPointF::isNull();
impl<'a> /*trait*/ QPointF_isNull for () {
  fn isNull(self, rsthis: &mut QPointF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF6isNullEv()};
    let mut ret = unsafe {_ZNK7QPointF6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn x<T: QPointF_x>(&mut self, value: T)  {
     value.x(self);
    // return 1;
  }
}

pub trait QPointF_x {
  fn x(self, rsthis: &mut QPointF) ;
}

// proto:  double QPointF::x();
impl<'a> /*trait*/ QPointF_x for () {
  fn x(self, rsthis: &mut QPointF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF1xEv()};
     unsafe {_ZNK7QPointF1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QPointF::NewQPointF(const QPoint & p);
impl<'a> /*trait*/ QPointF_NewQPointF for (&'a  QPoint) {
  fn NewQPointF(self) -> QPointF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointFC1ERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QPointFC1ERK6QPoint(qthis, arg0)};
    let rsthis = QPointF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn setX<T: QPointF_setX>(&mut self, value: T)  {
     value.setX(self);
    // return 1;
  }
}

pub trait QPointF_setX {
  fn setX(self, rsthis: &mut QPointF) ;
}

// proto:  void QPointF::setX(qreal x);
impl<'a> /*trait*/ QPointF_setX for (f64) {
  fn setX(self, rsthis: &mut QPointF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF4setXEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QPointF4setXEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn ry<T: QPointF_ry>(&mut self, value: T)  {
     value.ry(self);
    // return 1;
  }
}

pub trait QPointF_ry {
  fn ry(self, rsthis: &mut QPointF) ;
}

// proto:  qreal & QPointF::ry();
impl<'a> /*trait*/ QPointF_ry for () {
  fn ry(self, rsthis: &mut QPointF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF2ryEv()};
     unsafe {_ZN7QPointF2ryEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn dotProduct<T: QPointF_dotProduct>(&mut self, value: T) -> f64 {
    return value.dotProduct(self);
    // return 1;
  }
}

pub trait QPointF_dotProduct {
  fn dotProduct(self, rsthis: &mut QPointF) -> f64;
}

// proto: static double QPointF::dotProduct(const QPointF & p1, const QPointF & p2);
impl<'a> /*trait*/ QPointF_dotProduct for (&'a  QPointF, &'a  QPointF) {
  fn dotProduct(self, rsthis: &mut QPointF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QPointF10dotProductERKS_S1_(arg0, arg1)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn setY<T: QPointF_setY>(&mut self, value: T)  {
     value.setY(self);
    // return 1;
  }
}

pub trait QPointF_setY {
  fn setY(self, rsthis: &mut QPointF) ;
}

// proto:  void QPointF::setY(qreal y);
impl<'a> /*trait*/ QPointF_setY for (f64) {
  fn setY(self, rsthis: &mut QPointF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF4setYEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QPointF4setYEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

