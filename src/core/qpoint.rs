// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qpoint.h
// dst-file: /src/core/qpoint.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
// use super::qpoint::QPoint; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  int & QPoint::ry();
  fn _ZN6QPoint2ryEv(qthis: *mut c_void);
  // proto: static int QPoint::dotProduct(const QPoint & p1, const QPoint & p2);
  fn _ZN6QPoint10dotProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  int QPoint::x();
  fn _ZNK6QPoint1xEv(qthis: *mut c_void);
  // proto:  void QPoint::QPoint(int xpos, int ypos);
  fn _ZN6QPointC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  int QPoint::y();
  fn _ZNK6QPoint1yEv(qthis: *mut c_void);
  // proto:  void QPoint::setX(int x);
  fn _ZN6QPoint4setXEi(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QPoint::isNull();
  fn _ZNK6QPoint6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPoint::QPoint();
  fn _ZN6QPointC1Ev(qthis: *mut c_void);
  // proto:  void QPoint::setY(int y);
  fn _ZN6QPoint4setYEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int & QPoint::rx();
  fn _ZN6QPoint2rxEv(qthis: *mut c_void);
  // proto:  int QPoint::manhattanLength();
  fn _ZNK6QPoint15manhattanLengthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QPointF::QPointF(qreal xpos, qreal ypos);
  fn _ZN7QPointFC1Edd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QPointF::QPointF();
  fn _ZN7QPointFC1Ev(qthis: *mut c_void);
  // proto:  qreal QPointF::manhattanLength();
  fn _ZNK7QPointF15manhattanLengthEv(qthis: *mut c_void) -> c_double;
  // proto:  QPoint QPointF::toPoint();
  fn _ZNK7QPointF7toPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal & QPointF::rx();
  fn _ZN7QPointF2rxEv(qthis: *mut c_void);
  // proto:  qreal QPointF::y();
  fn _ZNK7QPointF1yEv(qthis: *mut c_void);
  // proto:  bool QPointF::isNull();
  fn _ZNK7QPointF6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  qreal QPointF::x();
  fn _ZNK7QPointF1xEv(qthis: *mut c_void);
  // proto:  void QPointF::QPointF(const QPoint & p);
  fn _ZN7QPointFC1ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPointF::setX(qreal x);
  fn _ZN7QPointF4setXEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal & QPointF::ry();
  fn _ZN7QPointF2ryEv(qthis: *mut c_void);
  // proto: static qreal QPointF::dotProduct(const QPointF & p1, const QPointF & p2);
  fn _ZN7QPointF10dotProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_double;
  // proto:  void QPointF::setY(qreal y);
  fn _ZN7QPointF4setYEd(qthis: *mut c_void, arg0: c_double);
} // <= ext block end

// body block begin =>
// class sizeof(QPoint)=8
pub struct QPoint {
  pub qclsinst: *mut c_void,
}

// class sizeof(QPointF)=16
pub struct QPointF {
  pub qclsinst: *mut c_void,
}

  // proto:  int & QPoint::ry();
impl /*struct*/ QPoint {
  pub fn ry<RetType, T: QPoint_ry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ry(self);
    // return 1;
  }
}

pub trait QPoint_ry<RetType> {
  fn ry(self , rsthis: &mut QPoint) -> RetType;
}

  // proto:  int & QPoint::ry();
impl<'a> /*trait*/ QPoint_ry<()> for () {
  fn ry(self , rsthis: &mut QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint2ryEv()};
     unsafe {_ZN6QPoint2ryEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static int QPoint::dotProduct(const QPoint & p1, const QPoint & p2);
impl /*struct*/ QPoint {
  pub fn dotProduct_s<RetType, T: QPoint_dotProduct_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.dotProduct_s();
    // return 1;
  }
}

pub trait QPoint_dotProduct_s<RetType> {
  fn dotProduct_s(self ) -> RetType;
}

  // proto: static int QPoint::dotProduct(const QPoint & p1, const QPoint & p2);
impl<'a> /*trait*/ QPoint_dotProduct_s<i32> for (QPoint, QPoint) {
  fn dotProduct_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN6QPoint10dotProductERKS_S1_(arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPoint::x();
impl /*struct*/ QPoint {
  pub fn x<RetType, T: QPoint_x<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QPoint_x<RetType> {
  fn x(self , rsthis: &mut QPoint) -> RetType;
}

  // proto:  int QPoint::x();
impl<'a> /*trait*/ QPoint_x<()> for () {
  fn x(self , rsthis: &mut QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint1xEv()};
     unsafe {_ZNK6QPoint1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPoint::QPoint(int xpos, int ypos);
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

  // proto:  void QPoint::QPoint(int xpos, int ypos);
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

  // proto:  int QPoint::y();
impl /*struct*/ QPoint {
  pub fn y<RetType, T: QPoint_y<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QPoint_y<RetType> {
  fn y(self , rsthis: &mut QPoint) -> RetType;
}

  // proto:  int QPoint::y();
impl<'a> /*trait*/ QPoint_y<()> for () {
  fn y(self , rsthis: &mut QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint1yEv()};
     unsafe {_ZNK6QPoint1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPoint::setX(int x);
impl /*struct*/ QPoint {
  pub fn setX<RetType, T: QPoint_setX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QPoint_setX<RetType> {
  fn setX(self , rsthis: &mut QPoint) -> RetType;
}

  // proto:  void QPoint::setX(int x);
impl<'a> /*trait*/ QPoint_setX<()> for (i32) {
  fn setX(self , rsthis: &mut QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint4setXEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QPoint4setXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPoint::isNull();
impl /*struct*/ QPoint {
  pub fn isNull<RetType, T: QPoint_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QPoint_isNull<RetType> {
  fn isNull(self , rsthis: &mut QPoint) -> RetType;
}

  // proto:  bool QPoint::isNull();
impl<'a> /*trait*/ QPoint_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QPoint) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint6isNullEv()};
    let mut ret = unsafe {_ZNK6QPoint6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPoint::QPoint();
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

  // proto:  void QPoint::setY(int y);
impl /*struct*/ QPoint {
  pub fn setY<RetType, T: QPoint_setY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QPoint_setY<RetType> {
  fn setY(self , rsthis: &mut QPoint) -> RetType;
}

  // proto:  void QPoint::setY(int y);
impl<'a> /*trait*/ QPoint_setY<()> for (i32) {
  fn setY(self , rsthis: &mut QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint4setYEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QPoint4setYEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int & QPoint::rx();
impl /*struct*/ QPoint {
  pub fn rx<RetType, T: QPoint_rx<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rx(self);
    // return 1;
  }
}

pub trait QPoint_rx<RetType> {
  fn rx(self , rsthis: &mut QPoint) -> RetType;
}

  // proto:  int & QPoint::rx();
impl<'a> /*trait*/ QPoint_rx<()> for () {
  fn rx(self , rsthis: &mut QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint2rxEv()};
     unsafe {_ZN6QPoint2rxEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QPoint::manhattanLength();
impl /*struct*/ QPoint {
  pub fn manhattanLength<RetType, T: QPoint_manhattanLength<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.manhattanLength(self);
    // return 1;
  }
}

pub trait QPoint_manhattanLength<RetType> {
  fn manhattanLength(self , rsthis: &mut QPoint) -> RetType;
}

  // proto:  int QPoint::manhattanLength();
impl<'a> /*trait*/ QPoint_manhattanLength<i32> for () {
  fn manhattanLength(self , rsthis: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint15manhattanLengthEv()};
    let mut ret = unsafe {_ZNK6QPoint15manhattanLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPointF::QPointF(qreal xpos, qreal ypos);
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

  // proto:  void QPointF::QPointF(qreal xpos, qreal ypos);
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

  // proto:  void QPointF::QPointF();
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

  // proto:  qreal QPointF::manhattanLength();
impl /*struct*/ QPointF {
  pub fn manhattanLength<RetType, T: QPointF_manhattanLength<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.manhattanLength(self);
    // return 1;
  }
}

pub trait QPointF_manhattanLength<RetType> {
  fn manhattanLength(self , rsthis: &mut QPointF) -> RetType;
}

  // proto:  qreal QPointF::manhattanLength();
impl<'a> /*trait*/ QPointF_manhattanLength<f64> for () {
  fn manhattanLength(self , rsthis: &mut QPointF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF15manhattanLengthEv()};
    let mut ret = unsafe {_ZNK7QPointF15manhattanLengthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPoint QPointF::toPoint();
impl /*struct*/ QPointF {
  pub fn toPoint<RetType, T: QPointF_toPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toPoint(self);
    // return 1;
  }
}

pub trait QPointF_toPoint<RetType> {
  fn toPoint(self , rsthis: &mut QPointF) -> RetType;
}

  // proto:  QPoint QPointF::toPoint();
impl<'a> /*trait*/ QPointF_toPoint<QPoint> for () {
  fn toPoint(self , rsthis: &mut QPointF) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF7toPointEv()};
    let mut ret = unsafe {_ZNK7QPointF7toPointEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal & QPointF::rx();
impl /*struct*/ QPointF {
  pub fn rx<RetType, T: QPointF_rx<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rx(self);
    // return 1;
  }
}

pub trait QPointF_rx<RetType> {
  fn rx(self , rsthis: &mut QPointF) -> RetType;
}

  // proto:  qreal & QPointF::rx();
impl<'a> /*trait*/ QPointF_rx<()> for () {
  fn rx(self , rsthis: &mut QPointF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF2rxEv()};
     unsafe {_ZN7QPointF2rxEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QPointF::y();
impl /*struct*/ QPointF {
  pub fn y<RetType, T: QPointF_y<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QPointF_y<RetType> {
  fn y(self , rsthis: &mut QPointF) -> RetType;
}

  // proto:  qreal QPointF::y();
impl<'a> /*trait*/ QPointF_y<()> for () {
  fn y(self , rsthis: &mut QPointF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF1yEv()};
     unsafe {_ZNK7QPointF1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QPointF::isNull();
impl /*struct*/ QPointF {
  pub fn isNull<RetType, T: QPointF_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QPointF_isNull<RetType> {
  fn isNull(self , rsthis: &mut QPointF) -> RetType;
}

  // proto:  bool QPointF::isNull();
impl<'a> /*trait*/ QPointF_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QPointF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF6isNullEv()};
    let mut ret = unsafe {_ZNK7QPointF6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QPointF::x();
impl /*struct*/ QPointF {
  pub fn x<RetType, T: QPointF_x<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QPointF_x<RetType> {
  fn x(self , rsthis: &mut QPointF) -> RetType;
}

  // proto:  qreal QPointF::x();
impl<'a> /*trait*/ QPointF_x<()> for () {
  fn x(self , rsthis: &mut QPointF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF1xEv()};
     unsafe {_ZNK7QPointF1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPointF::QPointF(const QPoint & p);
impl<'a> /*trait*/ QPointF_NewQPointF for (QPoint) {
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

  // proto:  void QPointF::setX(qreal x);
impl /*struct*/ QPointF {
  pub fn setX<RetType, T: QPointF_setX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QPointF_setX<RetType> {
  fn setX(self , rsthis: &mut QPointF) -> RetType;
}

  // proto:  void QPointF::setX(qreal x);
impl<'a> /*trait*/ QPointF_setX<()> for (f64) {
  fn setX(self , rsthis: &mut QPointF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF4setXEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QPointF4setXEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal & QPointF::ry();
impl /*struct*/ QPointF {
  pub fn ry<RetType, T: QPointF_ry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ry(self);
    // return 1;
  }
}

pub trait QPointF_ry<RetType> {
  fn ry(self , rsthis: &mut QPointF) -> RetType;
}

  // proto:  qreal & QPointF::ry();
impl<'a> /*trait*/ QPointF_ry<()> for () {
  fn ry(self , rsthis: &mut QPointF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF2ryEv()};
     unsafe {_ZN7QPointF2ryEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static qreal QPointF::dotProduct(const QPointF & p1, const QPointF & p2);
impl /*struct*/ QPointF {
  pub fn dotProduct_s<RetType, T: QPointF_dotProduct_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.dotProduct_s();
    // return 1;
  }
}

pub trait QPointF_dotProduct_s<RetType> {
  fn dotProduct_s(self ) -> RetType;
}

  // proto: static qreal QPointF::dotProduct(const QPointF & p1, const QPointF & p2);
impl<'a> /*trait*/ QPointF_dotProduct_s<f64> for (QPointF, QPointF) {
  fn dotProduct_s(self ) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QPointF10dotProductERKS_S1_(arg0, arg1)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPointF::setY(qreal y);
impl /*struct*/ QPointF {
  pub fn setY<RetType, T: QPointF_setY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QPointF_setY<RetType> {
  fn setY(self , rsthis: &mut QPointF) -> RetType;
}

  // proto:  void QPointF::setY(qreal y);
impl<'a> /*trait*/ QPointF_setY<()> for (f64) {
  fn setY(self , rsthis: &mut QPointF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF4setYEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QPointF4setYEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

