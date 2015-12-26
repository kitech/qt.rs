// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtGui/qpolygon.h
// dst-file: /src/gui/qpolygon.rs
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
use std::ops::Deref;
use super::super::core::qrect::QRect; // 771
use super::super::core::qpoint::QPoint; // 771
// use super::qpolygon::QPolygon; // 773
use super::super::core::qrect::QRectF; // 771
use super::super::core::qpoint::QPointF; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPolygon_Class_Size() -> c_int;
  // proto:  QRect QPolygon::boundingRect();
  fn _ZNK8QPolygon12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPolygon::setPoint(int index, const QPoint & p);
  fn _ZN8QPolygon8setPointEiRK6QPoint(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QPolygon::setPoint(int index, int x, int y);
  fn _ZN8QPolygon8setPointEiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int);
  // proto:  void QPolygon::QPolygon(int nPoints, const int * points);
  fn dector_ZN8QPolygonC1EiPKi(arg0: c_int, arg1: *mut c_int) -> *mut c_void;
  fn _ZN8QPolygonC1EiPKi(qthis: *mut c_void, arg0: c_int, arg1: *mut c_int);
  // proto:  void QPolygon::setPoints(int nPoints, const int * points);
  fn _ZN8QPolygon9setPointsEiPKi(qthis: *mut c_void, arg0: c_int, arg1: *mut c_int);
  // proto:  void QPolygon::putPoints(int index, int nPoints, const int * points);
  fn _ZN8QPolygon9putPointsEiiPKi(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_int);
  // proto:  void QPolygon::translate(int dx, int dy);
  fn _ZN8QPolygon9translateEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QPolygon::putPoints(int index, int nPoints, const QPolygon & from, int fromIndex);
  fn _ZN8QPolygon9putPointsEiiRKS_i(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void, arg3: c_int);
  // proto:  QPolygon QPolygon::subtracted(const QPolygon & r);
  fn _ZNK8QPolygon10subtractedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPoint QPolygon::point(int i);
  fn _ZNK8QPolygon5pointEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QPolygon::QPolygon(int size);
  fn dector_ZN8QPolygonC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN8QPolygonC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  QPolygon QPolygon::translated(int dx, int dy);
  fn _ZNK8QPolygon10translatedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QPolygon::putPoints(int index, int nPoints, int firstx, int firsty);
  fn _ZN8QPolygon9putPointsEiiiiz(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  QPolygon QPolygon::translated(const QPoint & offset);
  fn _ZNK8QPolygon10translatedERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPolygon::QPolygon(const QRect & r, bool closed);
  fn dector_ZN8QPolygonC1ERK5QRectb(arg0: *mut c_void, arg1: c_char) -> *mut c_void;
  fn _ZN8QPolygonC1ERK5QRectb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  QPolygon QPolygon::united(const QPolygon & r);
  fn _ZNK8QPolygon6unitedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygon QPolygon::intersected(const QPolygon & r);
  fn _ZNK8QPolygon11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPolygon::setPoints(int nPoints, int firstx, int firsty);
  fn _ZN8QPolygon9setPointsEiiiz(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int);
  // proto:  void QPolygon::translate(const QPoint & offset);
  fn _ZN8QPolygon9translateERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPolygon::point(int i, int * x, int * y);
  fn _ZNK8QPolygon5pointEiPiS0_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_int, arg2: *mut c_int);
  fn QPolygonF_Class_Size() -> c_int;
  // proto:  void QPolygonF::QPolygonF(const QPolygon & a);
  fn dector_ZN9QPolygonFC1ERK8QPolygon(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QPolygonFC1ERK8QPolygon(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QPolygonF::boundingRect();
  fn _ZNK9QPolygonF12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPolygonF::QPolygonF(const QRectF & r);
  fn dector_ZN9QPolygonFC1ERK6QRectF(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QPolygonFC1ERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPolygon QPolygonF::toPolygon();
  fn _ZNK9QPolygonF9toPolygonEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPolygonF::QPolygonF(int size);
  fn dector_ZN9QPolygonFC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN9QPolygonFC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  QPolygonF QPolygonF::subtracted(const QPolygonF & r);
  fn _ZNK9QPolygonF10subtractedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPolygonF::translate(const QPointF & offset);
  fn _ZN9QPolygonF9translateERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPolygonF QPolygonF::translated(const QPointF & offset);
  fn _ZNK9QPolygonF10translatedERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPolygonF::translate(qreal dx, qreal dy);
  fn _ZN9QPolygonF9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  QPolygonF QPolygonF::translated(qreal dx, qreal dy);
  fn _ZNK9QPolygonF10translatedEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  QPolygonF QPolygonF::intersected(const QPolygonF & r);
  fn _ZNK9QPolygonF11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygonF QPolygonF::united(const QPolygonF & r);
  fn _ZNK9QPolygonF6unitedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QPolygon)=1
pub struct QPolygon {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QPolygonF)=1
pub struct QPolygonF {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPolygon {
  pub fn inheritFrom(qthis: *mut c_void) -> QPolygon {
    return QPolygon{qclsinst: qthis};
  }
}
  // proto:  QRect QPolygon::boundingRect();
impl /*struct*/ QPolygon {
  pub fn boundingRect<RetType, T: QPolygon_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QPolygon_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  QRect QPolygon::boundingRect();
impl<'a> /*trait*/ QPolygon_boundingRect<QRect> for () {
  fn boundingRect(self , rsthis: & QPolygon) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon12boundingRectEv()};
    let mut ret = unsafe {_ZNK8QPolygon12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygon::setPoint(int index, const QPoint & p);
impl /*struct*/ QPolygon {
  pub fn setPoint<RetType, T: QPolygon_setPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPoint(self);
    // return 1;
  }
}

pub trait QPolygon_setPoint<RetType> {
  fn setPoint(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  void QPolygon::setPoint(int index, const QPoint & p);
impl<'a> /*trait*/ QPolygon_setPoint<()> for (i32, &'a QPoint) {
  fn setPoint(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon8setPointEiRK6QPoint()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPolygon8setPointEiRK6QPoint(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPolygon::setPoint(int index, int x, int y);
impl<'a> /*trait*/ QPolygon_setPoint<()> for (i32, i32, i32) {
  fn setPoint(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon8setPointEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPolygon8setPointEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPolygon::QPolygon(int nPoints, const int * points);
impl /*struct*/ QPolygon {
  pub fn New<T: QPolygon_New>(value: T) -> QPolygon {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygon_New {
  fn New(self) -> QPolygon;
}

  // proto:  void QPolygon::QPolygon(int nPoints, const int * points);
impl<'a> /*trait*/ QPolygon_New for (i32, &'a  Vec<i32>) {
  fn New(self) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC1EiPKi()};
    let ctysz: c_int = unsafe{QPolygon_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    // unsafe {_ZN8QPolygonC1EiPKi(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN8QPolygonC1EiPKi(arg0, arg1)};
    let rsthis = QPolygon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPolygon::setPoints(int nPoints, const int * points);
impl /*struct*/ QPolygon {
  pub fn setPoints<RetType, T: QPolygon_setPoints<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPoints(self);
    // return 1;
  }
}

pub trait QPolygon_setPoints<RetType> {
  fn setPoints(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  void QPolygon::setPoints(int nPoints, const int * points);
impl<'a> /*trait*/ QPolygon_setPoints<()> for (i32, &'a  Vec<i32>) {
  fn setPoints(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9setPointsEiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
     unsafe {_ZN8QPolygon9setPointsEiPKi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPolygon::putPoints(int index, int nPoints, const int * points);
impl /*struct*/ QPolygon {
  pub fn putPoints<RetType, T: QPolygon_putPoints<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.putPoints(self);
    // return 1;
  }
}

pub trait QPolygon_putPoints<RetType> {
  fn putPoints(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  void QPolygon::putPoints(int index, int nPoints, const int * points);
impl<'a> /*trait*/ QPolygon_putPoints<()> for (i32, i32, &'a  Vec<i32>) {
  fn putPoints(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9putPointsEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZN8QPolygon9putPointsEiiPKi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPolygon::translate(int dx, int dy);
impl /*struct*/ QPolygon {
  pub fn translate<RetType, T: QPolygon_translate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QPolygon_translate<RetType> {
  fn translate(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  void QPolygon::translate(int dx, int dy);
impl<'a> /*trait*/ QPolygon_translate<()> for (i32, i32) {
  fn translate(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9translateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPolygon9translateEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPolygon::putPoints(int index, int nPoints, const QPolygon & from, int fromIndex);
impl<'a> /*trait*/ QPolygon_putPoints<()> for (i32, i32, &'a QPolygon, i32) {
  fn putPoints(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9putPointsEiiRKS_i()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
     unsafe {_ZN8QPolygon9putPointsEiiRKS_i(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QPolygon QPolygon::subtracted(const QPolygon & r);
impl /*struct*/ QPolygon {
  pub fn subtracted<RetType, T: QPolygon_subtracted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.subtracted(self);
    // return 1;
  }
}

pub trait QPolygon_subtracted<RetType> {
  fn subtracted(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  QPolygon QPolygon::subtracted(const QPolygon & r);
impl<'a> /*trait*/ QPolygon_subtracted<QPolygon> for (&'a QPolygon) {
  fn subtracted(self , rsthis: & QPolygon) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon10subtractedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QPolygon10subtractedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QPolygon::point(int i);
impl /*struct*/ QPolygon {
  pub fn point<RetType, T: QPolygon_point<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.point(self);
    // return 1;
  }
}

pub trait QPolygon_point<RetType> {
  fn point(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  QPoint QPolygon::point(int i);
impl<'a> /*trait*/ QPolygon_point<QPoint> for (i32) {
  fn point(self , rsthis: & QPolygon) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon5pointEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK8QPolygon5pointEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygon::QPolygon(int size);
impl<'a> /*trait*/ QPolygon_New for (i32) {
  fn New(self) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC1Ei()};
    let ctysz: c_int = unsafe{QPolygon_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self  as c_int;
    // unsafe {_ZN8QPolygonC1Ei(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN8QPolygonC1Ei(arg0)};
    let rsthis = QPolygon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPolygon QPolygon::translated(int dx, int dy);
impl /*struct*/ QPolygon {
  pub fn translated<RetType, T: QPolygon_translated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translated(self);
    // return 1;
  }
}

pub trait QPolygon_translated<RetType> {
  fn translated(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  QPolygon QPolygon::translated(int dx, int dy);
impl<'a> /*trait*/ QPolygon_translated<QPolygon> for (i32, i32) {
  fn translated(self , rsthis: & QPolygon) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK8QPolygon10translatedEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygon::putPoints(int index, int nPoints, int firstx, int firsty);
impl<'a> /*trait*/ QPolygon_putPoints<()> for (i32, i32, i32, i32) {
  fn putPoints(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9putPointsEiiiiz()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN8QPolygon9putPointsEiiiiz(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QPolygon QPolygon::translated(const QPoint & offset);
impl<'a> /*trait*/ QPolygon_translated<QPolygon> for (&'a QPoint) {
  fn translated(self , rsthis: & QPolygon) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QPolygon10translatedERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygon::QPolygon(const QRect & r, bool closed);
impl<'a> /*trait*/ QPolygon_New for (&'a QRect, i8) {
  fn New(self) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC1ERK5QRectb()};
    let ctysz: c_int = unsafe{QPolygon_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
    // unsafe {_ZN8QPolygonC1ERK5QRectb(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN8QPolygonC1ERK5QRectb(arg0, arg1)};
    let rsthis = QPolygon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPolygon QPolygon::united(const QPolygon & r);
impl /*struct*/ QPolygon {
  pub fn united<RetType, T: QPolygon_united<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.united(self);
    // return 1;
  }
}

pub trait QPolygon_united<RetType> {
  fn united(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  QPolygon QPolygon::united(const QPolygon & r);
impl<'a> /*trait*/ QPolygon_united<QPolygon> for (&'a QPolygon) {
  fn united(self , rsthis: & QPolygon) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon6unitedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QPolygon6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygon QPolygon::intersected(const QPolygon & r);
impl /*struct*/ QPolygon {
  pub fn intersected<RetType, T: QPolygon_intersected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intersected(self);
    // return 1;
  }
}

pub trait QPolygon_intersected<RetType> {
  fn intersected(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  QPolygon QPolygon::intersected(const QPolygon & r);
impl<'a> /*trait*/ QPolygon_intersected<QPolygon> for (&'a QPolygon) {
  fn intersected(self , rsthis: & QPolygon) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QPolygon11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygon::setPoints(int nPoints, int firstx, int firsty);
impl<'a> /*trait*/ QPolygon_setPoints<()> for (i32, i32, i32) {
  fn setPoints(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9setPointsEiiiz()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPolygon9setPointsEiiiz(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPolygon::translate(const QPoint & offset);
impl<'a> /*trait*/ QPolygon_translate<()> for (&'a QPoint) {
  fn translate(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPolygon9translateERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPolygon::point(int i, int * x, int * y);
impl<'a> /*trait*/ QPolygon_point<()> for (i32, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn point(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon5pointEiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZNK8QPolygon5pointEiPiS0_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPolygonF {
  pub fn inheritFrom(qthis: *mut c_void) -> QPolygonF {
    return QPolygonF{qclsinst: qthis};
  }
}
  // proto:  void QPolygonF::QPolygonF(const QPolygon & a);
impl /*struct*/ QPolygonF {
  pub fn New<T: QPolygonF_New>(value: T) -> QPolygonF {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygonF_New {
  fn New(self) -> QPolygonF;
}

  // proto:  void QPolygonF::QPolygonF(const QPolygon & a);
impl<'a> /*trait*/ QPolygonF_New for (&'a QPolygon) {
  fn New(self) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC1ERK8QPolygon()};
    let ctysz: c_int = unsafe{QPolygonF_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QPolygonFC1ERK8QPolygon(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QPolygonFC1ERK8QPolygon(arg0)};
    let rsthis = QPolygonF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRectF QPolygonF::boundingRect();
impl /*struct*/ QPolygonF {
  pub fn boundingRect<RetType, T: QPolygonF_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QPolygonF_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QPolygonF) -> RetType;
}

  // proto:  QRectF QPolygonF::boundingRect();
impl<'a> /*trait*/ QPolygonF_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QPolygonF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF12boundingRectEv()};
    let mut ret = unsafe {_ZNK9QPolygonF12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygonF::QPolygonF(const QRectF & r);
impl<'a> /*trait*/ QPolygonF_New for (&'a QRectF) {
  fn New(self) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC1ERK6QRectF()};
    let ctysz: c_int = unsafe{QPolygonF_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QPolygonFC1ERK6QRectF(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QPolygonFC1ERK6QRectF(arg0)};
    let rsthis = QPolygonF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPolygon QPolygonF::toPolygon();
impl /*struct*/ QPolygonF {
  pub fn toPolygon<RetType, T: QPolygonF_toPolygon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPolygon(self);
    // return 1;
  }
}

pub trait QPolygonF_toPolygon<RetType> {
  fn toPolygon(self , rsthis: & QPolygonF) -> RetType;
}

  // proto:  QPolygon QPolygonF::toPolygon();
impl<'a> /*trait*/ QPolygonF_toPolygon<QPolygon> for () {
  fn toPolygon(self , rsthis: & QPolygonF) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF9toPolygonEv()};
    let mut ret = unsafe {_ZNK9QPolygonF9toPolygonEv(rsthis.qclsinst)};
    let mut ret1 = QPolygon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygonF::QPolygonF(int size);
impl<'a> /*trait*/ QPolygonF_New for (i32) {
  fn New(self) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC1Ei()};
    let ctysz: c_int = unsafe{QPolygonF_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self  as c_int;
    // unsafe {_ZN9QPolygonFC1Ei(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QPolygonFC1Ei(arg0)};
    let rsthis = QPolygonF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPolygonF QPolygonF::subtracted(const QPolygonF & r);
impl /*struct*/ QPolygonF {
  pub fn subtracted<RetType, T: QPolygonF_subtracted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.subtracted(self);
    // return 1;
  }
}

pub trait QPolygonF_subtracted<RetType> {
  fn subtracted(self , rsthis: & QPolygonF) -> RetType;
}

  // proto:  QPolygonF QPolygonF::subtracted(const QPolygonF & r);
impl<'a> /*trait*/ QPolygonF_subtracted<QPolygonF> for (&'a QPolygonF) {
  fn subtracted(self , rsthis: & QPolygonF) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF10subtractedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QPolygonF10subtractedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygonF::translate(const QPointF & offset);
impl /*struct*/ QPolygonF {
  pub fn translate<RetType, T: QPolygonF_translate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QPolygonF_translate<RetType> {
  fn translate(self , rsthis: & QPolygonF) -> RetType;
}

  // proto:  void QPolygonF::translate(const QPointF & offset);
impl<'a> /*trait*/ QPolygonF_translate<()> for (&'a QPointF) {
  fn translate(self , rsthis: & QPolygonF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonF9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QPolygonF9translateERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QPolygonF::translated(const QPointF & offset);
impl /*struct*/ QPolygonF {
  pub fn translated<RetType, T: QPolygonF_translated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translated(self);
    // return 1;
  }
}

pub trait QPolygonF_translated<RetType> {
  fn translated(self , rsthis: & QPolygonF) -> RetType;
}

  // proto:  QPolygonF QPolygonF::translated(const QPointF & offset);
impl<'a> /*trait*/ QPolygonF_translated<QPolygonF> for (&'a QPointF) {
  fn translated(self , rsthis: & QPolygonF) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF10translatedERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QPolygonF10translatedERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygonF::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QPolygonF_translate<()> for (f64, f64) {
  fn translate(self , rsthis: & QPolygonF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonF9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN9QPolygonF9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPolygonF QPolygonF::translated(qreal dx, qreal dy);
impl<'a> /*trait*/ QPolygonF_translated<QPolygonF> for (f64, f64) {
  fn translated(self , rsthis: & QPolygonF) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF10translatedEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK9QPolygonF10translatedEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygonF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygonF QPolygonF::intersected(const QPolygonF & r);
impl /*struct*/ QPolygonF {
  pub fn intersected<RetType, T: QPolygonF_intersected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intersected(self);
    // return 1;
  }
}

pub trait QPolygonF_intersected<RetType> {
  fn intersected(self , rsthis: & QPolygonF) -> RetType;
}

  // proto:  QPolygonF QPolygonF::intersected(const QPolygonF & r);
impl<'a> /*trait*/ QPolygonF_intersected<QPolygonF> for (&'a QPolygonF) {
  fn intersected(self , rsthis: & QPolygonF) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QPolygonF11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygonF QPolygonF::united(const QPolygonF & r);
impl /*struct*/ QPolygonF {
  pub fn united<RetType, T: QPolygonF_united<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.united(self);
    // return 1;
  }
}

pub trait QPolygonF_united<RetType> {
  fn united(self , rsthis: & QPolygonF) -> RetType;
}

  // proto:  QPolygonF QPolygonF::united(const QPolygonF & r);
impl<'a> /*trait*/ QPolygonF_united<QPolygonF> for (&'a QPolygonF) {
  fn united(self , rsthis: & QPolygonF) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF6unitedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QPolygonF6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

