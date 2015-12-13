// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qbitmap::QBitmap;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QRegion::isNull();
  fn _ZNK7QRegion6isNullEv() -> i32;
  // proto: QRect QRegion::boundingRect();
  fn _ZNK7QRegion12boundingRectEv() -> i32;
  // proto: void QRegion::NewQRegion(const QRegion & region);
  fn _ZN7QRegionC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QRegion::rectCount();
  fn _ZNK7QRegion9rectCountEv() -> i32;
  // proto: void QRegion::translate(int dx, int dy);
  fn _ZN7QRegion9translateEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QRegion QRegion::united(const QRegion & r);
  fn _ZNK7QRegion6unitedERKS_(arg0: *const c_void) -> i32;
  // proto: QRegion QRegion::translated(const QPoint & p);
  fn _ZNK7QRegion10translatedERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QRegion::swap(QRegion & other);
  fn _ZN7QRegion4swapERS_(arg0: *mut c_void) -> i32;
  // proto: void QRegion::NewQRegion(const QBitmap & bitmap);
  fn _ZN7QRegionC1ERK7QBitmap(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QRegion::FreeQRegion();
  fn _ZN7QRegionD0Ev() -> i32;
  // proto: void QRegion::translate(const QPoint & p);
  fn _ZN7QRegion9translateERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QRegion::NewQRegion();
  fn _ZN7QRegionC1Ev(qthis: *mut c_void) -> i32;
  // proto: bool QRegion::contains(const QRect & r);
  fn _ZNK7QRegion8containsERK5QRect(arg0: *const c_void) -> i32;
  // proto: bool QRegion::isEmpty();
  fn _ZNK7QRegion7isEmptyEv() -> i32;
  // proto: QRegion QRegion::intersected(const QRect & r);
  fn _ZNK7QRegion11intersectedERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QRegion::setRects(const QRect * rect, int num);
  fn _ZN7QRegion8setRectsEPK5QRecti(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: QVector<QRect> QRegion::rects();
  fn _ZNK7QRegion5rectsEv() -> i32;
  // proto: QRegion QRegion::subtracted(const QRegion & r);
  fn _ZNK7QRegion10subtractedERKS_(arg0: *const c_void) -> i32;
  // proto: bool QRegion::intersects(const QRect & r);
  fn _ZNK7QRegion10intersectsERK5QRect(arg0: *const c_void) -> i32;
  // proto: QRegion QRegion::translated(int dx, int dy);
  fn _ZNK7QRegion10translatedEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QRegion QRegion::intersected(const QRegion & r);
  fn _ZNK7QRegion11intersectedERKS_(arg0: *const c_void) -> i32;
  // proto: QRegion QRegion::united(const QRect & r);
  fn _ZNK7QRegion6unitedERK5QRect(arg0: *const c_void) -> i32;
  // proto: QRegion QRegion::xored(const QRegion & r);
  fn _ZNK7QRegion5xoredERKS_(arg0: *const c_void) -> i32;
  // proto: bool QRegion::intersects(const QRegion & r);
  fn _ZNK7QRegion10intersectsERKS_(arg0: *const c_void) -> i32;
  // proto: bool QRegion::contains(const QPoint & p);
  fn _ZNK7QRegion8containsERK6QPoint(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QRegion)=8
pub struct QRegion {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRegion {
  pub fn isNull<T: QRegion_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QRegion_isNull {
  fn isNull(self, this: &mut QRegion) -> i32;
}

// proto: bool QRegion::isNull();
impl<'a> /*trait*/ QRegion_isNull for () {
  fn isNull(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion6isNullEv()};
    unsafe {_ZNK7QRegion6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn boundingRect<T: QRegion_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QRegion_boundingRect {
  fn boundingRect(self, this: &mut QRegion) -> i32;
}

// proto: QRect QRegion::boundingRect();
impl<'a> /*trait*/ QRegion_boundingRect for () {
  fn boundingRect(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion12boundingRectEv()};
    unsafe {_ZNK7QRegion12boundingRectEv()};
    return 1;
  }
}

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

// proto: void QRegion::NewQRegion(const QRegion & region);
impl<'a> /*trait*/ QRegion_NewQRegion for (&'a  QRegion) {
  fn NewQRegion(self) -> QRegion {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegionC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QRegionC1ERKS_(qthis, arg0)};
    let rsthis = QRegion{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn rectCount<T: QRegion_rectCount>(&mut self, value: T) -> i32 {
    value.rectCount(self);
    return 1;
  }
}

pub trait QRegion_rectCount {
  fn rectCount(self, this: &mut QRegion) -> i32;
}

// proto: int QRegion::rectCount();
impl<'a> /*trait*/ QRegion_rectCount for () {
  fn rectCount(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion9rectCountEv()};
    unsafe {_ZNK7QRegion9rectCountEv()};
    return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn translate<T: QRegion_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QRegion_translate {
  fn translate(self, this: &mut QRegion) -> i32;
}

// proto: void QRegion::translate(int dx, int dy);
impl<'a> /*trait*/ QRegion_translate for (i32, i32) {
  fn translate(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegion9translateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QRegion9translateEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn united<T: QRegion_united>(&mut self, value: T) -> i32 {
    value.united(self);
    return 1;
  }
}

pub trait QRegion_united {
  fn united(self, this: &mut QRegion) -> i32;
}

// proto: QRegion QRegion::united(const QRegion & r);
impl<'a> /*trait*/ QRegion_united for (&'a  QRegion) {
  fn united(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion6unitedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QRegion6unitedERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn translated<T: QRegion_translated>(&mut self, value: T) -> i32 {
    value.translated(self);
    return 1;
  }
}

pub trait QRegion_translated {
  fn translated(self, this: &mut QRegion) -> i32;
}

// proto: QRegion QRegion::translated(const QPoint & p);
impl<'a> /*trait*/ QRegion_translated for (&'a  QPoint) {
  fn translated(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QRegion10translatedERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn swap<T: QRegion_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QRegion_swap {
  fn swap(self, this: &mut QRegion) -> i32;
}

// proto: void QRegion::swap(QRegion & other);
impl<'a> /*trait*/ QRegion_swap for (&'a mut QRegion) {
  fn swap(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegion4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QRegion4swapERS_(arg0)};
    return 1;
  }
}

// proto: void QRegion::NewQRegion(const QBitmap & bitmap);
impl<'a> /*trait*/ QRegion_NewQRegion for (&'a  QBitmap) {
  fn NewQRegion(self) -> QRegion {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegionC1ERK7QBitmap()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QRegionC1ERK7QBitmap(qthis, arg0)};
    let rsthis = QRegion{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn FreeQRegion<T: QRegion_FreeQRegion>(&mut self, value: T) -> i32 {
    value.FreeQRegion(self);
    return 1;
  }
}

pub trait QRegion_FreeQRegion {
  fn FreeQRegion(self, this: &mut QRegion) -> i32;
}

// proto: void QRegion::FreeQRegion();
impl<'a> /*trait*/ QRegion_FreeQRegion for () {
  fn FreeQRegion(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegionD0Ev()};
    unsafe {_ZN7QRegionD0Ev()};
    return 1;
  }
}

// proto: void QRegion::translate(const QPoint & p);
impl<'a> /*trait*/ QRegion_translate for (&'a  QPoint) {
  fn translate(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegion9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QRegion9translateERK6QPoint(arg0)};
    return 1;
  }
}

// proto: void QRegion::NewQRegion();
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

impl /*struct*/ QRegion {
  pub fn contains<T: QRegion_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QRegion_contains {
  fn contains(self, this: &mut QRegion) -> i32;
}

// proto: bool QRegion::contains(const QRect & r);
impl<'a> /*trait*/ QRegion_contains for (&'a  QRect) {
  fn contains(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion8containsERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QRegion8containsERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn isEmpty<T: QRegion_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QRegion_isEmpty {
  fn isEmpty(self, this: &mut QRegion) -> i32;
}

// proto: bool QRegion::isEmpty();
impl<'a> /*trait*/ QRegion_isEmpty for () {
  fn isEmpty(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion7isEmptyEv()};
    unsafe {_ZNK7QRegion7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn intersected<T: QRegion_intersected>(&mut self, value: T) -> i32 {
    value.intersected(self);
    return 1;
  }
}

pub trait QRegion_intersected {
  fn intersected(self, this: &mut QRegion) -> i32;
}

// proto: QRegion QRegion::intersected(const QRect & r);
impl<'a> /*trait*/ QRegion_intersected for (&'a  QRect) {
  fn intersected(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion11intersectedERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QRegion11intersectedERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn setRects<T: QRegion_setRects>(&mut self, value: T) -> i32 {
    value.setRects(self);
    return 1;
  }
}

pub trait QRegion_setRects {
  fn setRects(self, this: &mut QRegion) -> i32;
}

// proto: void QRegion::setRects(const QRect * rect, int num);
impl<'a> /*trait*/ QRegion_setRects for (&'a  QRect, i32) {
  fn setRects(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegion8setRectsEPK5QRecti()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QRegion8setRectsEPK5QRecti(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn rects<T: QRegion_rects>(&mut self, value: T) -> i32 {
    value.rects(self);
    return 1;
  }
}

pub trait QRegion_rects {
  fn rects(self, this: &mut QRegion) -> i32;
}

// proto: QVector<QRect> QRegion::rects();
impl<'a> /*trait*/ QRegion_rects for () {
  fn rects(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion5rectsEv()};
    unsafe {_ZNK7QRegion5rectsEv()};
    return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn subtracted<T: QRegion_subtracted>(&mut self, value: T) -> i32 {
    value.subtracted(self);
    return 1;
  }
}

pub trait QRegion_subtracted {
  fn subtracted(self, this: &mut QRegion) -> i32;
}

// proto: QRegion QRegion::subtracted(const QRegion & r);
impl<'a> /*trait*/ QRegion_subtracted for (&'a  QRegion) {
  fn subtracted(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10subtractedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QRegion10subtractedERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn intersects<T: QRegion_intersects>(&mut self, value: T) -> i32 {
    value.intersects(self);
    return 1;
  }
}

pub trait QRegion_intersects {
  fn intersects(self, this: &mut QRegion) -> i32;
}

// proto: bool QRegion::intersects(const QRect & r);
impl<'a> /*trait*/ QRegion_intersects for (&'a  QRect) {
  fn intersects(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10intersectsERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QRegion10intersectsERK5QRect(arg0)};
    return 1;
  }
}

// proto: QRegion QRegion::translated(int dx, int dy);
impl<'a> /*trait*/ QRegion_translated for (i32, i32) {
  fn translated(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QRegion10translatedEii(arg0, arg1)};
    return 1;
  }
}

// proto: QRegion QRegion::intersected(const QRegion & r);
impl<'a> /*trait*/ QRegion_intersected for (&'a  QRegion) {
  fn intersected(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion11intersectedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QRegion11intersectedERKS_(arg0)};
    return 1;
  }
}

// proto: QRegion QRegion::united(const QRect & r);
impl<'a> /*trait*/ QRegion_united for (&'a  QRect) {
  fn united(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion6unitedERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QRegion6unitedERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QRegion {
  pub fn xored<T: QRegion_xored>(&mut self, value: T) -> i32 {
    value.xored(self);
    return 1;
  }
}

pub trait QRegion_xored {
  fn xored(self, this: &mut QRegion) -> i32;
}

// proto: QRegion QRegion::xored(const QRegion & r);
impl<'a> /*trait*/ QRegion_xored for (&'a  QRegion) {
  fn xored(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion5xoredERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QRegion5xoredERKS_(arg0)};
    return 1;
  }
}

// proto: bool QRegion::intersects(const QRegion & r);
impl<'a> /*trait*/ QRegion_intersects for (&'a  QRegion) {
  fn intersects(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10intersectsERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QRegion10intersectsERKS_(arg0)};
    return 1;
  }
}

// proto: bool QRegion::contains(const QPoint & p);
impl<'a> /*trait*/ QRegion_contains for (&'a  QPoint) {
  fn contains(self, this: &mut QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion8containsERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QRegion8containsERK6QPoint(arg0)};
    return 1;
  }
}

