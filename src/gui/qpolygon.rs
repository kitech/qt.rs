// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
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
use super::super::core::qrect::QRectF; // 771
// use super::qpolygon::QPolygon; // 773
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
  fn C_ZNK8QPolygon12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPolygon::setPoint(int index, int x, int y);
  fn C_ZN8QPolygon8setPointEiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int);
  // proto:  void QPolygon::~QPolygon();
  fn C_ZN8QPolygonD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QPolygon::putPoints(int index, int nPoints, const QPolygon & from, int fromIndex);
  fn C_ZN8QPolygon9putPointsEiiRKS_i(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void, arg3: c_int);
  // proto:  QPolygon QPolygon::translated(const QPoint & offset);
  fn C_ZNK8QPolygon10translatedERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygon QPolygon::subtracted(const QPolygon & r);
  fn C_ZNK8QPolygon10subtractedERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygon QPolygon::intersected(const QPolygon & r);
  fn C_ZNK8QPolygon11intersectedERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPolygon::setPoint(int index, const QPoint & p);
  fn C_ZN8QPolygon8setPointEiRK6QPoint(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QPolygon::point(int i, int * x, int * y);
  fn C_ZNK8QPolygon5pointEiPiS0_(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_int, arg2: *mut c_int);
  // proto:  void QPolygon::translate(int dx, int dy);
  fn C_ZN8QPolygon9translateEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QPolygon::putPoints(int index, int nPoints, int firstx, int firsty);
  fn C_ZN8QPolygon9putPointsEiiiiz(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QPolygon::setPoints(int nPoints, int firstx, int firsty);
  fn C_ZN8QPolygon9setPointsEiiiz(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int);
  // proto:  void QPolygon::translate(const QPoint & offset);
  fn C_ZN8QPolygon9translateERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPolygon::swap(QPolygon & other);
  fn C_ZN8QPolygon4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPoint QPolygon::point(int i);
  fn C_ZNK8QPolygon5pointEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QPolygon::QPolygon(const QPolygon & a);
  fn C_ZN8QPolygonC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QPolygon::QPolygon(int nPoints, const int * points);
  fn C_ZN8QPolygonC2EiPKi(arg0: c_int, arg1: *mut c_int) -> u64;
  // proto:  QPolygon QPolygon::united(const QPolygon & r);
  fn C_ZNK8QPolygon6unitedERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygon QPolygon::translated(int dx, int dy);
  fn C_ZNK8QPolygon10translatedEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QPolygon::putPoints(int index, int nPoints, const int * points);
  fn C_ZN8QPolygon9putPointsEiiPKi(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_int);
  // proto:  void QPolygon::setPoints(int nPoints, const int * points);
  fn C_ZN8QPolygon9setPointsEiPKi(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_int);
  // proto:  void QPolygon::QPolygon(int size);
  fn C_ZN8QPolygonC2Ei(arg0: c_int) -> u64;
  // proto:  void QPolygon::QPolygon();
  fn C_ZN8QPolygonC2Ev() -> u64;
  // proto:  void QPolygon::QPolygon(const QRect & r, bool closed);
  fn C_ZN8QPolygonC2ERK5QRectb(arg0: *mut c_void, arg1: c_char) -> u64;
  fn QPolygonF_Class_Size() -> c_int;
  // proto:  QRectF QPolygonF::boundingRect();
  fn C_ZNK9QPolygonF12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPolygonF QPolygonF::intersected(const QPolygonF & r);
  fn C_ZNK9QPolygonF11intersectedERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPolygonF::QPolygonF(const QPolygon & a);
  fn C_ZN9QPolygonFC2ERK8QPolygon(arg0: *mut c_void) -> u64;
  // proto:  void QPolygonF::QPolygonF(const QRectF & r);
  fn C_ZN9QPolygonFC2ERK6QRectF(arg0: *mut c_void) -> u64;
  // proto:  QPolygon QPolygonF::toPolygon();
  fn C_ZNK9QPolygonF9toPolygonEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPolygonF::~QPolygonF();
  fn C_ZN9QPolygonFD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QPolygonF::QPolygonF(int size);
  fn C_ZN9QPolygonFC2Ei(arg0: c_int) -> u64;
  // proto:  QPolygonF QPolygonF::subtracted(const QPolygonF & r);
  fn C_ZNK9QPolygonF10subtractedERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPolygonF::QPolygonF();
  fn C_ZN9QPolygonFC2Ev() -> u64;
  // proto:  void QPolygonF::translate(const QPointF & offset);
  fn C_ZN9QPolygonF9translateERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPolygonF::swap(QPolygonF & other);
  fn C_ZN9QPolygonF4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPolygonF QPolygonF::translated(const QPointF & offset);
  fn C_ZNK9QPolygonF10translatedERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPolygonF::translate(qreal dx, qreal dy);
  fn C_ZN9QPolygonF9translateEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  void QPolygonF::QPolygonF(const QPolygonF & a);
  fn C_ZN9QPolygonFC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  QPolygonF QPolygonF::translated(qreal dx, qreal dy);
  fn C_ZNK9QPolygonF10translatedEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  bool QPolygonF::isClosed();
  fn C_ZNK9QPolygonF8isClosedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QPolygonF QPolygonF::united(const QPolygonF & r);
  fn C_ZNK9QPolygonF6unitedERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QPolygon)=1
#[derive(Default)]
pub struct QPolygon {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QPolygonF)=1
#[derive(Default)]
pub struct QPolygonF {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPolygon {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPolygon {
    return QPolygon{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK8QPolygon12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygon::setPoint(int index, int x, int y);
impl /*struct*/ QPolygon {
  pub fn setPoint<RetType, T: QPolygon_setPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPoint(self);
    // return 1;
  }
}

pub trait QPolygon_setPoint<RetType> {
  fn setPoint(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  void QPolygon::setPoint(int index, int x, int y);
impl<'a> /*trait*/ QPolygon_setPoint<()> for (i32, i32, i32) {
  fn setPoint(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon8setPointEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {C_ZN8QPolygon8setPointEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPolygon::~QPolygon();
impl /*struct*/ QPolygon {
  pub fn free<RetType, T: QPolygon_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPolygon_free<RetType> {
  fn free(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  void QPolygon::~QPolygon();
impl<'a> /*trait*/ QPolygon_free<()> for () {
  fn free(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonD2Ev()};
     unsafe {C_ZN8QPolygonD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPolygon::putPoints(int index, int nPoints, const QPolygon & from, int fromIndex);
impl /*struct*/ QPolygon {
  pub fn putPoints<RetType, T: QPolygon_putPoints<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.putPoints(self);
    // return 1;
  }
}

pub trait QPolygon_putPoints<RetType> {
  fn putPoints(self , rsthis: & QPolygon) -> RetType;
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
     unsafe {C_ZN8QPolygon9putPointsEiiRKS_i(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QPolygon QPolygon::translated(const QPoint & offset);
impl /*struct*/ QPolygon {
  pub fn translated<RetType, T: QPolygon_translated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translated(self);
    // return 1;
  }
}

pub trait QPolygon_translated<RetType> {
  fn translated(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  QPolygon QPolygon::translated(const QPoint & offset);
impl<'a> /*trait*/ QPolygon_translated<QPolygon> for (&'a QPoint) {
  fn translated(self , rsthis: & QPolygon) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK8QPolygon10translatedERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {C_ZNK8QPolygon10subtractedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK8QPolygon11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygon::setPoint(int index, const QPoint & p);
impl<'a> /*trait*/ QPolygon_setPoint<()> for (i32, &'a QPoint) {
  fn setPoint(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon8setPointEiRK6QPoint()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN8QPolygon8setPointEiRK6QPoint(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPolygon::point(int i, int * x, int * y);
impl /*struct*/ QPolygon {
  pub fn point<RetType, T: QPolygon_point<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.point(self);
    // return 1;
  }
}

pub trait QPolygon_point<RetType> {
  fn point(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  void QPolygon::point(int i, int * x, int * y);
impl<'a> /*trait*/ QPolygon_point<()> for (i32, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn point(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon5pointEiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {C_ZNK8QPolygon5pointEiPiS0_(rsthis.qclsinst, arg0, arg1, arg2)};
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
     unsafe {C_ZN8QPolygon9translateEii(rsthis.qclsinst, arg0, arg1)};
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
     unsafe {C_ZN8QPolygon9putPointsEiiiiz(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QPolygon::setPoints(int nPoints, int firstx, int firsty);
impl /*struct*/ QPolygon {
  pub fn setPoints<RetType, T: QPolygon_setPoints<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPoints(self);
    // return 1;
  }
}

pub trait QPolygon_setPoints<RetType> {
  fn setPoints(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  void QPolygon::setPoints(int nPoints, int firstx, int firsty);
impl<'a> /*trait*/ QPolygon_setPoints<()> for (i32, i32, i32) {
  fn setPoints(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9setPointsEiiiz()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {C_ZN8QPolygon9setPointsEiiiz(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPolygon::translate(const QPoint & offset);
impl<'a> /*trait*/ QPolygon_translate<()> for (&'a QPoint) {
  fn translate(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN8QPolygon9translateERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPolygon::swap(QPolygon & other);
impl /*struct*/ QPolygon {
  pub fn swap<RetType, T: QPolygon_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPolygon_swap<RetType> {
  fn swap(self , rsthis: & QPolygon) -> RetType;
}

  // proto:  void QPolygon::swap(QPolygon & other);
impl<'a> /*trait*/ QPolygon_swap<()> for (&'a QPolygon) {
  fn swap(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN8QPolygon4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPoint QPolygon::point(int i);
impl<'a> /*trait*/ QPolygon_point<QPoint> for (i32) {
  fn point(self , rsthis: & QPolygon) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon5pointEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK8QPolygon5pointEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygon::QPolygon(const QPolygon & a);
impl /*struct*/ QPolygon {
  pub fn new<T: QPolygon_new>(value: T) -> QPolygon {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygon_new {
  fn new(self) -> QPolygon;
}

  // proto:  void QPolygon::QPolygon(const QPolygon & a);
impl<'a> /*trait*/ QPolygon_new for (&'a QPolygon) {
  fn new(self) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC2ERKS_()};
    let ctysz: c_int = unsafe{QPolygon_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN8QPolygonC2ERKS_(arg0)};
    let rsthis = QPolygon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPolygon::QPolygon(int nPoints, const int * points);
impl<'a> /*trait*/ QPolygon_new for (i32, &'a  Vec<i32>) {
  fn new(self) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC2EiPKi()};
    let ctysz: c_int = unsafe{QPolygon_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let qthis: u64 = unsafe {C_ZN8QPolygonC2EiPKi(arg0, arg1)};
    let rsthis = QPolygon{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK8QPolygon6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygon QPolygon::translated(int dx, int dy);
impl<'a> /*trait*/ QPolygon_translated<QPolygon> for (i32, i32) {
  fn translated(self , rsthis: & QPolygon) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK8QPolygon10translatedEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygon::putPoints(int index, int nPoints, const int * points);
impl<'a> /*trait*/ QPolygon_putPoints<()> for (i32, i32, &'a  Vec<i32>) {
  fn putPoints(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9putPointsEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {C_ZN8QPolygon9putPointsEiiPKi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPolygon::setPoints(int nPoints, const int * points);
impl<'a> /*trait*/ QPolygon_setPoints<()> for (i32, &'a  Vec<i32>) {
  fn setPoints(self , rsthis: & QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9setPointsEiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
     unsafe {C_ZN8QPolygon9setPointsEiPKi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPolygon::QPolygon(int size);
impl<'a> /*trait*/ QPolygon_new for (i32) {
  fn new(self) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC2Ei()};
    let ctysz: c_int = unsafe{QPolygon_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    let qthis: u64 = unsafe {C_ZN8QPolygonC2Ei(arg0)};
    let rsthis = QPolygon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPolygon::QPolygon();
impl<'a> /*trait*/ QPolygon_new for () {
  fn new(self) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC2Ev()};
    let ctysz: c_int = unsafe{QPolygon_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN8QPolygonC2Ev()};
    let rsthis = QPolygon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPolygon::QPolygon(const QRect & r, bool closed);
impl<'a> /*trait*/ QPolygon_new for (&'a QRect, i8) {
  fn new(self) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC2ERK5QRectb()};
    let ctysz: c_int = unsafe{QPolygon_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
    let qthis: u64 = unsafe {C_ZN8QPolygonC2ERK5QRectb(arg0, arg1)};
    let rsthis = QPolygon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPolygonF {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPolygonF {
    return QPolygonF{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK9QPolygonF12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK9QPolygonF11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygonF::QPolygonF(const QPolygon & a);
impl /*struct*/ QPolygonF {
  pub fn new<T: QPolygonF_new>(value: T) -> QPolygonF {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygonF_new {
  fn new(self) -> QPolygonF;
}

  // proto:  void QPolygonF::QPolygonF(const QPolygon & a);
impl<'a> /*trait*/ QPolygonF_new for (&'a QPolygon) {
  fn new(self) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC2ERK8QPolygon()};
    let ctysz: c_int = unsafe{QPolygonF_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QPolygonFC2ERK8QPolygon(arg0)};
    let rsthis = QPolygonF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPolygonF::QPolygonF(const QRectF & r);
impl<'a> /*trait*/ QPolygonF_new for (&'a QRectF) {
  fn new(self) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC2ERK6QRectF()};
    let ctysz: c_int = unsafe{QPolygonF_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QPolygonFC2ERK6QRectF(arg0)};
    let rsthis = QPolygonF{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK9QPolygonF9toPolygonEv(rsthis.qclsinst)};
    let mut ret1 = QPolygon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygonF::~QPolygonF();
impl /*struct*/ QPolygonF {
  pub fn free<RetType, T: QPolygonF_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPolygonF_free<RetType> {
  fn free(self , rsthis: & QPolygonF) -> RetType;
}

  // proto:  void QPolygonF::~QPolygonF();
impl<'a> /*trait*/ QPolygonF_free<()> for () {
  fn free(self , rsthis: & QPolygonF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFD2Ev()};
     unsafe {C_ZN9QPolygonFD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPolygonF::QPolygonF(int size);
impl<'a> /*trait*/ QPolygonF_new for (i32) {
  fn new(self) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC2Ei()};
    let ctysz: c_int = unsafe{QPolygonF_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    let qthis: u64 = unsafe {C_ZN9QPolygonFC2Ei(arg0)};
    let rsthis = QPolygonF{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK9QPolygonF10subtractedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygonF::QPolygonF();
impl<'a> /*trait*/ QPolygonF_new for () {
  fn new(self) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC2Ev()};
    let ctysz: c_int = unsafe{QPolygonF_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN9QPolygonFC2Ev()};
    let rsthis = QPolygonF{qclsinst: qthis, ..Default::default()};
    return rsthis;
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
     unsafe {C_ZN9QPolygonF9translateERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPolygonF::swap(QPolygonF & other);
impl /*struct*/ QPolygonF {
  pub fn swap<RetType, T: QPolygonF_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPolygonF_swap<RetType> {
  fn swap(self , rsthis: & QPolygonF) -> RetType;
}

  // proto:  void QPolygonF::swap(QPolygonF & other);
impl<'a> /*trait*/ QPolygonF_swap<()> for (&'a QPolygonF) {
  fn swap(self , rsthis: & QPolygonF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonF4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QPolygonF4swapERS_(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK9QPolygonF10translatedERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
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
     unsafe {C_ZN9QPolygonF9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPolygonF::QPolygonF(const QPolygonF & a);
impl<'a> /*trait*/ QPolygonF_new for (&'a QPolygonF) {
  fn new(self) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC2ERKS_()};
    let ctysz: c_int = unsafe{QPolygonF_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QPolygonFC2ERKS_(arg0)};
    let rsthis = QPolygonF{qclsinst: qthis, ..Default::default()};
    return rsthis;
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
    let mut ret = unsafe {C_ZNK9QPolygonF10translatedEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPolygonF::isClosed();
impl /*struct*/ QPolygonF {
  pub fn isClosed<RetType, T: QPolygonF_isClosed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isClosed(self);
    // return 1;
  }
}

pub trait QPolygonF_isClosed<RetType> {
  fn isClosed(self , rsthis: & QPolygonF) -> RetType;
}

  // proto:  bool QPolygonF::isClosed();
impl<'a> /*trait*/ QPolygonF_isClosed<i8> for () {
  fn isClosed(self , rsthis: & QPolygonF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF8isClosedEv()};
    let mut ret = unsafe {C_ZNK9QPolygonF8isClosedEv(rsthis.qclsinst)};
    return ret as i8;
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
    let mut ret = unsafe {C_ZNK9QPolygonF6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

