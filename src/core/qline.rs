// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qline.h
// dst-file: /src/core/qline.rs
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
use super::qpoint::QPoint; // 773
use super::qpoint::QPointF; // 773
// use super::qline::QLine; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  bool QLine::isNull();
  fn _ZNK5QLine6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  QLine QLine::translated(const QPoint & p);
  fn _ZNK5QLine10translatedERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QLine::setP2(const QPoint & p2);
  fn _ZN5QLine5setP2ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QLine::x2();
  fn _ZNK5QLine2x2Ev(qthis: *mut c_void) -> c_int;
  // proto:  void QLine::QLine(const QPoint & pt1, const QPoint & pt2);
  fn _ZN5QLineC1ERK6QPointS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QLine::setP1(const QPoint & p1);
  fn _ZN5QLine5setP1ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QLine::translate(const QPoint & p);
  fn _ZN5QLine9translateERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QLine::dx();
  fn _ZNK5QLine2dxEv(qthis: *mut c_void) -> c_int;
  // proto:  int QLine::y2();
  fn _ZNK5QLine2y2Ev(qthis: *mut c_void) -> c_int;
  // proto:  int QLine::dy();
  fn _ZNK5QLine2dyEv(qthis: *mut c_void) -> c_int;
  // proto:  int QLine::y1();
  fn _ZNK5QLine2y1Ev(qthis: *mut c_void) -> c_int;
  // proto:  QPoint QLine::p1();
  fn _ZNK5QLine2p1Ev(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QLine::p2();
  fn _ZNK5QLine2p2Ev(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLine::QLine(int x1, int y1, int x2, int y2);
  fn _ZN5QLineC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QLine::translate(int dx, int dy);
  fn _ZN5QLine9translateEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  QLine QLine::translated(int dx, int dy);
  fn _ZNK5QLine10translatedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QLine::setPoints(const QPoint & p1, const QPoint & p2);
  fn _ZN5QLine9setPointsERK6QPointS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QLine::setLine(int x1, int y1, int x2, int y2);
  fn _ZN5QLine7setLineEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  int QLine::x1();
  fn _ZNK5QLine2x1Ev(qthis: *mut c_void) -> c_int;
  // proto:  void QLine::QLine();
  fn _ZN5QLineC1Ev(qthis: *mut c_void);
  // proto:  void QLineF::translate(qreal dx, qreal dy);
  fn _ZN6QLineF9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QLineF::setPoints(const QPointF & p1, const QPointF & p2);
  fn _ZN6QLineF9setPointsERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QLineF::setP2(const QPointF & p2);
  fn _ZN6QLineF5setP2ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QLineF QLineF::translated(qreal dx, qreal dy);
  fn _ZNK6QLineF10translatedEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QLineF::setLength(qreal len);
  fn _ZN6QLineF9setLengthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QLineF::x1();
  fn _ZNK6QLineF2x1Ev(qthis: *mut c_void) -> c_double;
  // proto:  qreal QLineF::angle();
  fn _ZNK6QLineF5angleEv(qthis: *mut c_void) -> c_double;
  // proto:  void QLineF::QLineF(const QPointF & pt1, const QPointF & pt2);
  fn _ZN6QLineFC1ERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  qreal QLineF::length();
  fn _ZNK6QLineF6lengthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QLineF::QLineF(const QLine & line);
  fn _ZN6QLineFC1ERK5QLine(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QLineF::setAngle(qreal angle);
  fn _ZN6QLineF8setAngleEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QLineF::x2();
  fn _ZNK6QLineF2x2Ev(qthis: *mut c_void) -> c_double;
  // proto:  void QLineF::translate(const QPointF & p);
  fn _ZN6QLineF9translateERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QLineF::dx();
  fn _ZNK6QLineF2dxEv(qthis: *mut c_void) -> c_double;
  // proto:  void QLineF::QLineF();
  fn _ZN6QLineFC1Ev(qthis: *mut c_void);
  // proto:  QPointF QLineF::p1();
  fn _ZNK6QLineF2p1Ev(qthis: *mut c_void) -> *mut c_void;
  // proto:  QLineF QLineF::normalVector();
  fn _ZNK6QLineF12normalVectorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QLine QLineF::toLine();
  fn _ZNK6QLineF6toLineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QLineF::pointAt(qreal t);
  fn _ZNK6QLineF7pointAtEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  QPointF QLineF::p2();
  fn _ZNK6QLineF2p2Ev(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QLineF::y2();
  fn _ZNK6QLineF2y2Ev(qthis: *mut c_void) -> c_double;
  // proto:  void QLineF::QLineF(qreal x1, qreal y1, qreal x2, qreal y2);
  fn _ZN6QLineFC1Edddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  qreal QLineF::dy();
  fn _ZNK6QLineF2dyEv(qthis: *mut c_void) -> c_double;
  // proto:  QLineF QLineF::unitVector();
  fn _ZNK6QLineF10unitVectorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QLineF::isNull();
  fn _ZNK6QLineF6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  qreal QLineF::y1();
  fn _ZNK6QLineF2y1Ev(qthis: *mut c_void) -> c_double;
  // proto:  qreal QLineF::angleTo(const QLineF & l);
  fn _ZNK6QLineF7angleToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
  // proto:  QLineF QLineF::translated(const QPointF & p);
  fn _ZNK6QLineF10translatedERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QLineF::setLine(qreal x1, qreal y1, qreal x2, qreal y2);
  fn _ZN6QLineF7setLineEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto: static QLineF QLineF::fromPolar(qreal length, qreal angle);
  fn _ZN6QLineF9fromPolarEdd(arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QLineF::setP1(const QPointF & p1);
  fn _ZN6QLineF5setP1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QLineF::angle(const QLineF & l);
  fn _ZNK6QLineF5angleERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
} // <= ext block end

// body block begin =>
// class sizeof(QLine)=16
pub struct QLine {
  pub qclsinst: *mut c_void,
}

// class sizeof(QLineF)=32
pub struct QLineF {
  pub qclsinst: *mut c_void,
}

  // proto:  bool QLine::isNull();
impl /*struct*/ QLine {
  pub fn isNull<RetType, T: QLine_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QLine_isNull<RetType> {
  fn isNull(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  bool QLine::isNull();
impl<'a> /*trait*/ QLine_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QLine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine6isNullEv()};
    let mut ret = unsafe {_ZNK5QLine6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QLine QLine::translated(const QPoint & p);
impl /*struct*/ QLine {
  pub fn translated<RetType, T: QLine_translated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translated(self);
    // return 1;
  }
}

pub trait QLine_translated<RetType> {
  fn translated(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  QLine QLine::translated(const QPoint & p);
impl<'a> /*trait*/ QLine_translated<QLine> for (QPoint) {
  fn translated(self , rsthis: &mut QLine) -> QLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QLine10translatedERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLine::setP2(const QPoint & p2);
impl /*struct*/ QLine {
  pub fn setP2<RetType, T: QLine_setP2<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setP2(self);
    // return 1;
  }
}

pub trait QLine_setP2<RetType> {
  fn setP2(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  void QLine::setP2(const QPoint & p2);
impl<'a> /*trait*/ QLine_setP2<()> for (QPoint) {
  fn setP2(self , rsthis: &mut QLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine5setP2ERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QLine5setP2ERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QLine::x2();
impl /*struct*/ QLine {
  pub fn x2<RetType, T: QLine_x2<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.x2(self);
    // return 1;
  }
}

pub trait QLine_x2<RetType> {
  fn x2(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  int QLine::x2();
impl<'a> /*trait*/ QLine_x2<i32> for () {
  fn x2(self , rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2x2Ev()};
    let mut ret = unsafe {_ZNK5QLine2x2Ev(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QLine::QLine(const QPoint & pt1, const QPoint & pt2);
impl /*struct*/ QLine {
  pub fn NewQLine<T: QLine_NewQLine>(value: T) -> QLine {
    let rsthis = value.NewQLine();
    return rsthis;
    // return 1;
  }
}

pub trait QLine_NewQLine {
  fn NewQLine(self) -> QLine;
}

  // proto:  void QLine::QLine(const QPoint & pt1, const QPoint & pt2);
impl<'a> /*trait*/ QLine_NewQLine for (QPoint, QPoint) {
  fn NewQLine(self) -> QLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLineC1ERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QLineC1ERK6QPointS2_(qthis, arg0, arg1)};
    let rsthis = QLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLine::setP1(const QPoint & p1);
impl /*struct*/ QLine {
  pub fn setP1<RetType, T: QLine_setP1<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setP1(self);
    // return 1;
  }
}

pub trait QLine_setP1<RetType> {
  fn setP1(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  void QLine::setP1(const QPoint & p1);
impl<'a> /*trait*/ QLine_setP1<()> for (QPoint) {
  fn setP1(self , rsthis: &mut QLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine5setP1ERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QLine5setP1ERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLine::translate(const QPoint & p);
impl /*struct*/ QLine {
  pub fn translate<RetType, T: QLine_translate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QLine_translate<RetType> {
  fn translate(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  void QLine::translate(const QPoint & p);
impl<'a> /*trait*/ QLine_translate<()> for (QPoint) {
  fn translate(self , rsthis: &mut QLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QLine9translateERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QLine::dx();
impl /*struct*/ QLine {
  pub fn dx<RetType, T: QLine_dx<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dx(self);
    // return 1;
  }
}

pub trait QLine_dx<RetType> {
  fn dx(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  int QLine::dx();
impl<'a> /*trait*/ QLine_dx<i32> for () {
  fn dx(self , rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2dxEv()};
    let mut ret = unsafe {_ZNK5QLine2dxEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QLine::y2();
impl /*struct*/ QLine {
  pub fn y2<RetType, T: QLine_y2<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.y2(self);
    // return 1;
  }
}

pub trait QLine_y2<RetType> {
  fn y2(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  int QLine::y2();
impl<'a> /*trait*/ QLine_y2<i32> for () {
  fn y2(self , rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2y2Ev()};
    let mut ret = unsafe {_ZNK5QLine2y2Ev(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QLine::dy();
impl /*struct*/ QLine {
  pub fn dy<RetType, T: QLine_dy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dy(self);
    // return 1;
  }
}

pub trait QLine_dy<RetType> {
  fn dy(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  int QLine::dy();
impl<'a> /*trait*/ QLine_dy<i32> for () {
  fn dy(self , rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2dyEv()};
    let mut ret = unsafe {_ZNK5QLine2dyEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QLine::y1();
impl /*struct*/ QLine {
  pub fn y1<RetType, T: QLine_y1<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.y1(self);
    // return 1;
  }
}

pub trait QLine_y1<RetType> {
  fn y1(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  int QLine::y1();
impl<'a> /*trait*/ QLine_y1<i32> for () {
  fn y1(self , rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2y1Ev()};
    let mut ret = unsafe {_ZNK5QLine2y1Ev(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPoint QLine::p1();
impl /*struct*/ QLine {
  pub fn p1<RetType, T: QLine_p1<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.p1(self);
    // return 1;
  }
}

pub trait QLine_p1<RetType> {
  fn p1(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  QPoint QLine::p1();
impl<'a> /*trait*/ QLine_p1<QPoint> for () {
  fn p1(self , rsthis: &mut QLine) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2p1Ev()};
    let mut ret = unsafe {_ZNK5QLine2p1Ev(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QLine::p2();
impl /*struct*/ QLine {
  pub fn p2<RetType, T: QLine_p2<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.p2(self);
    // return 1;
  }
}

pub trait QLine_p2<RetType> {
  fn p2(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  QPoint QLine::p2();
impl<'a> /*trait*/ QLine_p2<QPoint> for () {
  fn p2(self , rsthis: &mut QLine) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2p2Ev()};
    let mut ret = unsafe {_ZNK5QLine2p2Ev(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLine::QLine(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QLine_NewQLine for (i32, i32, i32, i32) {
  fn NewQLine(self) -> QLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLineC1Eiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN5QLineC1Eiiii(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLine::translate(int dx, int dy);
impl<'a> /*trait*/ QLine_translate<()> for (i32, i32) {
  fn translate(self , rsthis: &mut QLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine9translateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN5QLine9translateEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QLine QLine::translated(int dx, int dy);
impl<'a> /*trait*/ QLine_translated<QLine> for (i32, i32) {
  fn translated(self , rsthis: &mut QLine) -> QLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK5QLine10translatedEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLine::setPoints(const QPoint & p1, const QPoint & p2);
impl /*struct*/ QLine {
  pub fn setPoints<RetType, T: QLine_setPoints<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPoints(self);
    // return 1;
  }
}

pub trait QLine_setPoints<RetType> {
  fn setPoints(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  void QLine::setPoints(const QPoint & p1, const QPoint & p2);
impl<'a> /*trait*/ QLine_setPoints<()> for (QPoint, QPoint) {
  fn setPoints(self , rsthis: &mut QLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine9setPointsERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN5QLine9setPointsERK6QPointS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLine::setLine(int x1, int y1, int x2, int y2);
impl /*struct*/ QLine {
  pub fn setLine<RetType, T: QLine_setLine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLine(self);
    // return 1;
  }
}

pub trait QLine_setLine<RetType> {
  fn setLine(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  void QLine::setLine(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QLine_setLine<()> for (i32, i32, i32, i32) {
  fn setLine(self , rsthis: &mut QLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine7setLineEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN5QLine7setLineEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  int QLine::x1();
impl /*struct*/ QLine {
  pub fn x1<RetType, T: QLine_x1<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.x1(self);
    // return 1;
  }
}

pub trait QLine_x1<RetType> {
  fn x1(self , rsthis: &mut QLine) -> RetType;
}

  // proto:  int QLine::x1();
impl<'a> /*trait*/ QLine_x1<i32> for () {
  fn x1(self , rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2x1Ev()};
    let mut ret = unsafe {_ZNK5QLine2x1Ev(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QLine::QLine();
impl<'a> /*trait*/ QLine_NewQLine for () {
  fn NewQLine(self) -> QLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLineC1Ev()};
    unsafe {_ZN5QLineC1Ev(qthis)};
    let rsthis = QLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLineF::translate(qreal dx, qreal dy);
impl /*struct*/ QLineF {
  pub fn translate<RetType, T: QLineF_translate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QLineF_translate<RetType> {
  fn translate(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  void QLineF::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QLineF_translate<()> for (f64, f64) {
  fn translate(self , rsthis: &mut QLineF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN6QLineF9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLineF::setPoints(const QPointF & p1, const QPointF & p2);
impl /*struct*/ QLineF {
  pub fn setPoints<RetType, T: QLineF_setPoints<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPoints(self);
    // return 1;
  }
}

pub trait QLineF_setPoints<RetType> {
  fn setPoints(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  void QLineF::setPoints(const QPointF & p1, const QPointF & p2);
impl<'a> /*trait*/ QLineF_setPoints<()> for (QPointF, QPointF) {
  fn setPoints(self , rsthis: &mut QLineF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9setPointsERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN6QLineF9setPointsERK7QPointFS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLineF::setP2(const QPointF & p2);
impl /*struct*/ QLineF {
  pub fn setP2<RetType, T: QLineF_setP2<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setP2(self);
    // return 1;
  }
}

pub trait QLineF_setP2<RetType> {
  fn setP2(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  void QLineF::setP2(const QPointF & p2);
impl<'a> /*trait*/ QLineF_setP2<()> for (QPointF) {
  fn setP2(self , rsthis: &mut QLineF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF5setP2ERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLineF5setP2ERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QLineF QLineF::translated(qreal dx, qreal dy);
impl /*struct*/ QLineF {
  pub fn translated<RetType, T: QLineF_translated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translated(self);
    // return 1;
  }
}

pub trait QLineF_translated<RetType> {
  fn translated(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  QLineF QLineF::translated(qreal dx, qreal dy);
impl<'a> /*trait*/ QLineF_translated<QLineF> for (f64, f64) {
  fn translated(self , rsthis: &mut QLineF) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF10translatedEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK6QLineF10translatedEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QLineF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLineF::setLength(qreal len);
impl /*struct*/ QLineF {
  pub fn setLength<RetType, T: QLineF_setLength<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLength(self);
    // return 1;
  }
}

pub trait QLineF_setLength<RetType> {
  fn setLength(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  void QLineF::setLength(qreal len);
impl<'a> /*trait*/ QLineF_setLength<()> for (f64) {
  fn setLength(self , rsthis: &mut QLineF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9setLengthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QLineF9setLengthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QLineF::x1();
impl /*struct*/ QLineF {
  pub fn x1<RetType, T: QLineF_x1<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.x1(self);
    // return 1;
  }
}

pub trait QLineF_x1<RetType> {
  fn x1(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  qreal QLineF::x1();
impl<'a> /*trait*/ QLineF_x1<f64> for () {
  fn x1(self , rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2x1Ev()};
    let mut ret = unsafe {_ZNK6QLineF2x1Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QLineF::angle();
impl /*struct*/ QLineF {
  pub fn angle<RetType, T: QLineF_angle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.angle(self);
    // return 1;
  }
}

pub trait QLineF_angle<RetType> {
  fn angle(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  qreal QLineF::angle();
impl<'a> /*trait*/ QLineF_angle<f64> for () {
  fn angle(self , rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF5angleEv()};
    let mut ret = unsafe {_ZNK6QLineF5angleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QLineF::QLineF(const QPointF & pt1, const QPointF & pt2);
impl /*struct*/ QLineF {
  pub fn NewQLineF<T: QLineF_NewQLineF>(value: T) -> QLineF {
    let rsthis = value.NewQLineF();
    return rsthis;
    // return 1;
  }
}

pub trait QLineF_NewQLineF {
  fn NewQLineF(self) -> QLineF;
}

  // proto:  void QLineF::QLineF(const QPointF & pt1, const QPointF & pt2);
impl<'a> /*trait*/ QLineF_NewQLineF for (QPointF, QPointF) {
  fn NewQLineF(self) -> QLineF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineFC1ERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN6QLineFC1ERK7QPointFS2_(qthis, arg0, arg1)};
    let rsthis = QLineF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QLineF::length();
impl /*struct*/ QLineF {
  pub fn length<RetType, T: QLineF_length<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QLineF_length<RetType> {
  fn length(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  qreal QLineF::length();
impl<'a> /*trait*/ QLineF_length<f64> for () {
  fn length(self , rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF6lengthEv()};
    let mut ret = unsafe {_ZNK6QLineF6lengthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QLineF::QLineF(const QLine & line);
impl<'a> /*trait*/ QLineF_NewQLineF for (QLine) {
  fn NewQLineF(self) -> QLineF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineFC1ERK5QLine()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QLineFC1ERK5QLine(qthis, arg0)};
    let rsthis = QLineF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLineF::setAngle(qreal angle);
impl /*struct*/ QLineF {
  pub fn setAngle<RetType, T: QLineF_setAngle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAngle(self);
    // return 1;
  }
}

pub trait QLineF_setAngle<RetType> {
  fn setAngle(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  void QLineF::setAngle(qreal angle);
impl<'a> /*trait*/ QLineF_setAngle<()> for (f64) {
  fn setAngle(self , rsthis: &mut QLineF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF8setAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QLineF8setAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QLineF::x2();
impl /*struct*/ QLineF {
  pub fn x2<RetType, T: QLineF_x2<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.x2(self);
    // return 1;
  }
}

pub trait QLineF_x2<RetType> {
  fn x2(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  qreal QLineF::x2();
impl<'a> /*trait*/ QLineF_x2<f64> for () {
  fn x2(self , rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2x2Ev()};
    let mut ret = unsafe {_ZNK6QLineF2x2Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QLineF::translate(const QPointF & p);
impl<'a> /*trait*/ QLineF_translate<()> for (QPointF) {
  fn translate(self , rsthis: &mut QLineF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLineF9translateERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QLineF::dx();
impl /*struct*/ QLineF {
  pub fn dx<RetType, T: QLineF_dx<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dx(self);
    // return 1;
  }
}

pub trait QLineF_dx<RetType> {
  fn dx(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  qreal QLineF::dx();
impl<'a> /*trait*/ QLineF_dx<f64> for () {
  fn dx(self , rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2dxEv()};
    let mut ret = unsafe {_ZNK6QLineF2dxEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QLineF::QLineF();
impl<'a> /*trait*/ QLineF_NewQLineF for () {
  fn NewQLineF(self) -> QLineF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineFC1Ev()};
    unsafe {_ZN6QLineFC1Ev(qthis)};
    let rsthis = QLineF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QLineF::p1();
impl /*struct*/ QLineF {
  pub fn p1<RetType, T: QLineF_p1<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.p1(self);
    // return 1;
  }
}

pub trait QLineF_p1<RetType> {
  fn p1(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  QPointF QLineF::p1();
impl<'a> /*trait*/ QLineF_p1<QPointF> for () {
  fn p1(self , rsthis: &mut QLineF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2p1Ev()};
    let mut ret = unsafe {_ZNK6QLineF2p1Ev(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QLineF QLineF::normalVector();
impl /*struct*/ QLineF {
  pub fn normalVector<RetType, T: QLineF_normalVector<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.normalVector(self);
    // return 1;
  }
}

pub trait QLineF_normalVector<RetType> {
  fn normalVector(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  QLineF QLineF::normalVector();
impl<'a> /*trait*/ QLineF_normalVector<QLineF> for () {
  fn normalVector(self , rsthis: &mut QLineF) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF12normalVectorEv()};
    let mut ret = unsafe {_ZNK6QLineF12normalVectorEv(rsthis.qclsinst)};
    let mut ret1 = QLineF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QLine QLineF::toLine();
impl /*struct*/ QLineF {
  pub fn toLine<RetType, T: QLineF_toLine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toLine(self);
    // return 1;
  }
}

pub trait QLineF_toLine<RetType> {
  fn toLine(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  QLine QLineF::toLine();
impl<'a> /*trait*/ QLineF_toLine<QLine> for () {
  fn toLine(self , rsthis: &mut QLineF) -> QLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF6toLineEv()};
    let mut ret = unsafe {_ZNK6QLineF6toLineEv(rsthis.qclsinst)};
    let mut ret1 = QLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QLineF::pointAt(qreal t);
impl /*struct*/ QLineF {
  pub fn pointAt<RetType, T: QLineF_pointAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pointAt(self);
    // return 1;
  }
}

pub trait QLineF_pointAt<RetType> {
  fn pointAt(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  QPointF QLineF::pointAt(qreal t);
impl<'a> /*trait*/ QLineF_pointAt<QPointF> for (f64) {
  fn pointAt(self , rsthis: &mut QLineF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF7pointAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK6QLineF7pointAtEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QLineF::p2();
impl /*struct*/ QLineF {
  pub fn p2<RetType, T: QLineF_p2<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.p2(self);
    // return 1;
  }
}

pub trait QLineF_p2<RetType> {
  fn p2(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  QPointF QLineF::p2();
impl<'a> /*trait*/ QLineF_p2<QPointF> for () {
  fn p2(self , rsthis: &mut QLineF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2p2Ev()};
    let mut ret = unsafe {_ZNK6QLineF2p2Ev(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QLineF::y2();
impl /*struct*/ QLineF {
  pub fn y2<RetType, T: QLineF_y2<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.y2(self);
    // return 1;
  }
}

pub trait QLineF_y2<RetType> {
  fn y2(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  qreal QLineF::y2();
impl<'a> /*trait*/ QLineF_y2<f64> for () {
  fn y2(self , rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2y2Ev()};
    let mut ret = unsafe {_ZNK6QLineF2y2Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QLineF::QLineF(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QLineF_NewQLineF for (f64, f64, f64, f64) {
  fn NewQLineF(self) -> QLineF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineFC1Edddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QLineFC1Edddd(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QLineF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QLineF::dy();
impl /*struct*/ QLineF {
  pub fn dy<RetType, T: QLineF_dy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dy(self);
    // return 1;
  }
}

pub trait QLineF_dy<RetType> {
  fn dy(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  qreal QLineF::dy();
impl<'a> /*trait*/ QLineF_dy<f64> for () {
  fn dy(self , rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2dyEv()};
    let mut ret = unsafe {_ZNK6QLineF2dyEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QLineF QLineF::unitVector();
impl /*struct*/ QLineF {
  pub fn unitVector<RetType, T: QLineF_unitVector<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unitVector(self);
    // return 1;
  }
}

pub trait QLineF_unitVector<RetType> {
  fn unitVector(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  QLineF QLineF::unitVector();
impl<'a> /*trait*/ QLineF_unitVector<QLineF> for () {
  fn unitVector(self , rsthis: &mut QLineF) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF10unitVectorEv()};
    let mut ret = unsafe {_ZNK6QLineF10unitVectorEv(rsthis.qclsinst)};
    let mut ret1 = QLineF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QLineF::isNull();
impl /*struct*/ QLineF {
  pub fn isNull<RetType, T: QLineF_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QLineF_isNull<RetType> {
  fn isNull(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  bool QLineF::isNull();
impl<'a> /*trait*/ QLineF_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QLineF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF6isNullEv()};
    let mut ret = unsafe {_ZNK6QLineF6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QLineF::y1();
impl /*struct*/ QLineF {
  pub fn y1<RetType, T: QLineF_y1<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.y1(self);
    // return 1;
  }
}

pub trait QLineF_y1<RetType> {
  fn y1(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  qreal QLineF::y1();
impl<'a> /*trait*/ QLineF_y1<f64> for () {
  fn y1(self , rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2y1Ev()};
    let mut ret = unsafe {_ZNK6QLineF2y1Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QLineF::angleTo(const QLineF & l);
impl /*struct*/ QLineF {
  pub fn angleTo<RetType, T: QLineF_angleTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.angleTo(self);
    // return 1;
  }
}

pub trait QLineF_angleTo<RetType> {
  fn angleTo(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  qreal QLineF::angleTo(const QLineF & l);
impl<'a> /*trait*/ QLineF_angleTo<f64> for (QLineF) {
  fn angleTo(self , rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF7angleToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QLineF7angleToERKS_(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QLineF QLineF::translated(const QPointF & p);
impl<'a> /*trait*/ QLineF_translated<QLineF> for (QPointF) {
  fn translated(self , rsthis: &mut QLineF) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF10translatedERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QLineF10translatedERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QLineF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLineF::setLine(qreal x1, qreal y1, qreal x2, qreal y2);
impl /*struct*/ QLineF {
  pub fn setLine<RetType, T: QLineF_setLine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLine(self);
    // return 1;
  }
}

pub trait QLineF_setLine<RetType> {
  fn setLine(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  void QLineF::setLine(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QLineF_setLine<()> for (f64, f64, f64, f64) {
  fn setLine(self , rsthis: &mut QLineF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF7setLineEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN6QLineF7setLineEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto: static QLineF QLineF::fromPolar(qreal length, qreal angle);
impl /*struct*/ QLineF {
  pub fn fromPolar_s<RetType, T: QLineF_fromPolar_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromPolar_s();
    // return 1;
  }
}

pub trait QLineF_fromPolar_s<RetType> {
  fn fromPolar_s(self ) -> RetType;
}

  // proto: static QLineF QLineF::fromPolar(qreal length, qreal angle);
impl<'a> /*trait*/ QLineF_fromPolar_s<QLineF> for (f64, f64) {
  fn fromPolar_s(self ) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9fromPolarEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN6QLineF9fromPolarEdd(arg0, arg1)};
    let mut ret1 = QLineF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLineF::setP1(const QPointF & p1);
impl /*struct*/ QLineF {
  pub fn setP1<RetType, T: QLineF_setP1<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setP1(self);
    // return 1;
  }
}

pub trait QLineF_setP1<RetType> {
  fn setP1(self , rsthis: &mut QLineF) -> RetType;
}

  // proto:  void QLineF::setP1(const QPointF & p1);
impl<'a> /*trait*/ QLineF_setP1<()> for (QPointF) {
  fn setP1(self , rsthis: &mut QLineF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF5setP1ERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLineF5setP1ERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QLineF::angle(const QLineF & l);
impl<'a> /*trait*/ QLineF_angle<f64> for (QLineF) {
  fn angle(self , rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF5angleERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QLineF5angleERKS_(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

// <= body block end

