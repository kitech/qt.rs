// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtGui/qregion.h
// dst-file: /src/gui/qregion.rs
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
use super::qbitmap::QBitmap; // 773
use super::qpolygon::QPolygon; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  bool QRegion::isNull();
  fn _ZNK7QRegion6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  QRect QRegion::boundingRect();
  fn _ZNK7QRegion12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegion::QRegion(const QRegion & region);
  fn _ZN7QRegionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QRegion::rectCount();
  fn _ZNK7QRegion9rectCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRegion::translate(int dx, int dy);
  fn _ZN7QRegion9translateEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  QRegion QRegion::united(const QRegion & r);
  fn _ZNK7QRegion6unitedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRegion QRegion::translated(const QPoint & p);
  fn _ZNK7QRegion10translatedERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QRegion::swap(QRegion & other);
  fn _ZN7QRegion4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRegion::QRegion(const QBitmap & bitmap);
  fn _ZN7QRegionC1ERK7QBitmap(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRegion::~QRegion();
  fn _ZN7QRegionD0Ev(qthis: *mut c_void);
  // proto:  void QRegion::translate(const QPoint & p);
  fn _ZN7QRegion9translateERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRegion::QRegion();
  fn _ZN7QRegionC1Ev(qthis: *mut c_void);
  // proto:  bool QRegion::contains(const QRect & r);
  fn _ZNK7QRegion8containsERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QRegion::isEmpty();
  fn _ZNK7QRegion7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  QRegion QRegion::intersected(const QRect & r);
  fn _ZNK7QRegion11intersectedERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QRegion::setRects(const QRect * rect, int num);
  fn _ZN7QRegion8setRectsEPK5QRecti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QVector<QRect> QRegion::rects();
  fn _ZNK7QRegion5rectsEv(qthis: *mut c_void);
  // proto:  QRegion QRegion::subtracted(const QRegion & r);
  fn _ZNK7QRegion10subtractedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QRegion::intersects(const QRect & r);
  fn _ZNK7QRegion10intersectsERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QRegion QRegion::translated(int dx, int dy);
  fn _ZNK7QRegion10translatedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QRegion QRegion::intersected(const QRegion & r);
  fn _ZNK7QRegion11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRegion QRegion::united(const QRect & r);
  fn _ZNK7QRegion6unitedERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRegion QRegion::xored(const QRegion & r);
  fn _ZNK7QRegion5xoredERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QRegion::intersects(const QRegion & r);
  fn _ZNK7QRegion10intersectsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QRegion::contains(const QPoint & p);
  fn _ZNK7QRegion8containsERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QRegion)=8
pub struct QRegion {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRegion {
  pub fn inheritFrom(qthis: *mut c_void) -> QRegion {
    return QRegion{qclsinst: qthis};
  }
}
  // proto:  bool QRegion::isNull();
impl /*struct*/ QRegion {
  pub fn isNull<RetType, T: QRegion_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QRegion_isNull<RetType> {
  fn isNull(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  bool QRegion::isNull();
impl<'a> /*trait*/ QRegion_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QRegion) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion6isNullEv()};
    let mut ret = unsafe {_ZNK7QRegion6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QRegion::boundingRect();
impl /*struct*/ QRegion {
  pub fn boundingRect<RetType, T: QRegion_boundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QRegion_boundingRect<RetType> {
  fn boundingRect(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  QRect QRegion::boundingRect();
impl<'a> /*trait*/ QRegion_boundingRect<QRect> for () {
  fn boundingRect(self , rsthis: &mut QRegion) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion12boundingRectEv()};
    let mut ret = unsafe {_ZNK7QRegion12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegion::QRegion(const QRegion & region);
impl /*struct*/ QRegion {
  pub fn NewQRegion<T: QRegion_NewQRegion>(value: T) -> QRegion {
    let rsthis = value.NewQRegion();
    return rsthis;
    // return 1;
  }
}

pub trait QRegion_NewQRegion {
  fn NewQRegion(self) -> QRegion;
}

  // proto:  void QRegion::QRegion(const QRegion & region);
impl<'a> /*trait*/ QRegion_NewQRegion for (QRegion) {
  fn NewQRegion(self) -> QRegion {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QRegionC1ERKS_(qthis, arg0)};
    let rsthis = QRegion{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QRegion::rectCount();
impl /*struct*/ QRegion {
  pub fn rectCount<RetType, T: QRegion_rectCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rectCount(self);
    // return 1;
  }
}

pub trait QRegion_rectCount<RetType> {
  fn rectCount(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  int QRegion::rectCount();
impl<'a> /*trait*/ QRegion_rectCount<i32> for () {
  fn rectCount(self , rsthis: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion9rectCountEv()};
    let mut ret = unsafe {_ZNK7QRegion9rectCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QRegion::translate(int dx, int dy);
impl /*struct*/ QRegion {
  pub fn translate<RetType, T: QRegion_translate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QRegion_translate<RetType> {
  fn translate(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  void QRegion::translate(int dx, int dy);
impl<'a> /*trait*/ QRegion_translate<()> for (i32, i32) {
  fn translate(self , rsthis: &mut QRegion) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegion9translateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QRegion9translateEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QRegion QRegion::united(const QRegion & r);
impl /*struct*/ QRegion {
  pub fn united<RetType, T: QRegion_united<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.united(self);
    // return 1;
  }
}

pub trait QRegion_united<RetType> {
  fn united(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  QRegion QRegion::united(const QRegion & r);
impl<'a> /*trait*/ QRegion_united<QRegion> for (QRegion) {
  fn united(self , rsthis: &mut QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion6unitedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRegion QRegion::translated(const QPoint & p);
impl /*struct*/ QRegion {
  pub fn translated<RetType, T: QRegion_translated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translated(self);
    // return 1;
  }
}

pub trait QRegion_translated<RetType> {
  fn translated(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  QRegion QRegion::translated(const QPoint & p);
impl<'a> /*trait*/ QRegion_translated<QRegion> for (QPoint) {
  fn translated(self , rsthis: &mut QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion10translatedERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegion::swap(QRegion & other);
impl /*struct*/ QRegion {
  pub fn swap<RetType, T: QRegion_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QRegion_swap<RetType> {
  fn swap(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  void QRegion::swap(QRegion & other);
impl<'a> /*trait*/ QRegion_swap<()> for (QRegion) {
  fn swap(self , rsthis: &mut QRegion) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegion4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QRegion4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRegion::QRegion(const QBitmap & bitmap);
impl<'a> /*trait*/ QRegion_NewQRegion for (QBitmap) {
  fn NewQRegion(self) -> QRegion {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegionC1ERK7QBitmap()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QRegionC1ERK7QBitmap(qthis, arg0)};
    let rsthis = QRegion{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegion::~QRegion();
impl /*struct*/ QRegion {
  pub fn FreeQRegion<RetType, T: QRegion_FreeQRegion<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQRegion(self);
    // return 1;
  }
}

pub trait QRegion_FreeQRegion<RetType> {
  fn FreeQRegion(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  void QRegion::~QRegion();
impl<'a> /*trait*/ QRegion_FreeQRegion<()> for () {
  fn FreeQRegion(self , rsthis: &mut QRegion) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegionD0Ev()};
     unsafe {_ZN7QRegionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRegion::translate(const QPoint & p);
impl<'a> /*trait*/ QRegion_translate<()> for (QPoint) {
  fn translate(self , rsthis: &mut QRegion) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegion9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QRegion9translateERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRegion::QRegion();
impl<'a> /*trait*/ QRegion_NewQRegion for () {
  fn NewQRegion(self) -> QRegion {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegionC1Ev()};
    unsafe {_ZN7QRegionC1Ev(qthis)};
    let rsthis = QRegion{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QRegion::contains(const QRect & r);
impl /*struct*/ QRegion {
  pub fn contains<RetType, T: QRegion_contains<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QRegion_contains<RetType> {
  fn contains(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  bool QRegion::contains(const QRect & r);
impl<'a> /*trait*/ QRegion_contains<i8> for (QRect) {
  fn contains(self , rsthis: &mut QRegion) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion8containsERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion8containsERK5QRect(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QRegion::isEmpty();
impl /*struct*/ QRegion {
  pub fn isEmpty<RetType, T: QRegion_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QRegion_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  bool QRegion::isEmpty();
impl<'a> /*trait*/ QRegion_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QRegion) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion7isEmptyEv()};
    let mut ret = unsafe {_ZNK7QRegion7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRegion QRegion::intersected(const QRect & r);
impl /*struct*/ QRegion {
  pub fn intersected<RetType, T: QRegion_intersected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intersected(self);
    // return 1;
  }
}

pub trait QRegion_intersected<RetType> {
  fn intersected(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  QRegion QRegion::intersected(const QRect & r);
impl<'a> /*trait*/ QRegion_intersected<QRegion> for (QRect) {
  fn intersected(self , rsthis: &mut QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion11intersectedERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion11intersectedERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegion::setRects(const QRect * rect, int num);
impl /*struct*/ QRegion {
  pub fn setRects<RetType, T: QRegion_setRects<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRects(self);
    // return 1;
  }
}

pub trait QRegion_setRects<RetType> {
  fn setRects(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  void QRegion::setRects(const QRect * rect, int num);
impl<'a> /*trait*/ QRegion_setRects<()> for (QRect, i32) {
  fn setRects(self , rsthis: &mut QRegion) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegion8setRectsEPK5QRecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QRegion8setRectsEPK5QRecti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QVector<QRect> QRegion::rects();
impl /*struct*/ QRegion {
  pub fn rects<RetType, T: QRegion_rects<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rects(self);
    // return 1;
  }
}

pub trait QRegion_rects<RetType> {
  fn rects(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  QVector<QRect> QRegion::rects();
impl<'a> /*trait*/ QRegion_rects<()> for () {
  fn rects(self , rsthis: &mut QRegion) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion5rectsEv()};
     unsafe {_ZNK7QRegion5rectsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRegion QRegion::subtracted(const QRegion & r);
impl /*struct*/ QRegion {
  pub fn subtracted<RetType, T: QRegion_subtracted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.subtracted(self);
    // return 1;
  }
}

pub trait QRegion_subtracted<RetType> {
  fn subtracted(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  QRegion QRegion::subtracted(const QRegion & r);
impl<'a> /*trait*/ QRegion_subtracted<QRegion> for (QRegion) {
  fn subtracted(self , rsthis: &mut QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10subtractedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion10subtractedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRegion::intersects(const QRect & r);
impl /*struct*/ QRegion {
  pub fn intersects<RetType, T: QRegion_intersects<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intersects(self);
    // return 1;
  }
}

pub trait QRegion_intersects<RetType> {
  fn intersects(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  bool QRegion::intersects(const QRect & r);
impl<'a> /*trait*/ QRegion_intersects<i8> for (QRect) {
  fn intersects(self , rsthis: &mut QRegion) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10intersectsERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion10intersectsERK5QRect(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRegion QRegion::translated(int dx, int dy);
impl<'a> /*trait*/ QRegion_translated<QRegion> for (i32, i32) {
  fn translated(self , rsthis: &mut QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QRegion10translatedEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRegion::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRegion QRegion::intersected(const QRegion & r);
impl<'a> /*trait*/ QRegion_intersected<QRegion> for (QRegion) {
  fn intersected(self , rsthis: &mut QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRegion QRegion::united(const QRect & r);
impl<'a> /*trait*/ QRegion_united<QRegion> for (QRect) {
  fn united(self , rsthis: &mut QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion6unitedERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion6unitedERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRegion QRegion::xored(const QRegion & r);
impl /*struct*/ QRegion {
  pub fn xored<RetType, T: QRegion_xored<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.xored(self);
    // return 1;
  }
}

pub trait QRegion_xored<RetType> {
  fn xored(self , rsthis: &mut QRegion) -> RetType;
}

  // proto:  QRegion QRegion::xored(const QRegion & r);
impl<'a> /*trait*/ QRegion_xored<QRegion> for (QRegion) {
  fn xored(self , rsthis: &mut QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion5xoredERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion5xoredERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRegion::intersects(const QRegion & r);
impl<'a> /*trait*/ QRegion_intersects<i8> for (QRegion) {
  fn intersects(self , rsthis: &mut QRegion) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10intersectsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion10intersectsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QRegion::contains(const QPoint & p);
impl<'a> /*trait*/ QRegion_contains<i8> for (QPoint) {
  fn contains(self , rsthis: &mut QRegion) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion8containsERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion8containsERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

