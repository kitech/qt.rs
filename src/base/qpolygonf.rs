// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpolygon::QPolygon;
use super::qrectf::QRectF;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QRectF QPolygonF::boundingRect();
  fn _ZNK9QPolygonF12boundingRectEv() -> i32;
  // proto: QPolygonF QPolygonF::intersected(const QPolygonF & r);
  fn _ZNK9QPolygonF11intersectedERKS_(arg0: *const c_void) -> i32;
  // proto: void QPolygonF::NewQPolygonF(const QPolygon & a);
  fn _ZN9QPolygonFC1ERK8QPolygon(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QPolygonF::NewQPolygonF(const QRectF & r);
  fn _ZN9QPolygonFC1ERK6QRectF(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QPolygon QPolygonF::toPolygon();
  fn _ZNK9QPolygonF9toPolygonEv() -> i32;
  // proto: void QPolygonF::FreeQPolygonF();
  fn _ZN9QPolygonFD0Ev() -> i32;
  // proto: void QPolygonF::NewQPolygonF(int size);
  fn _ZN9QPolygonFC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  // proto: QPolygonF QPolygonF::subtracted(const QPolygonF & r);
  fn _ZNK9QPolygonF10subtractedERKS_(arg0: *const c_void) -> i32;
  // proto: void QPolygonF::NewQPolygonF();
  fn _ZN9QPolygonFC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QPolygonF::translate(const QPointF & offset);
  fn _ZN9QPolygonF9translateERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QPolygonF::swap(QPolygonF & other);
  fn _ZN9QPolygonF4swapERS_(arg0: *mut c_void) -> i32;
  // proto: QPolygonF QPolygonF::translated(const QPointF & offset);
  fn _ZNK9QPolygonF10translatedERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QPolygonF::translate(qreal dx, qreal dy);
  fn _ZN9QPolygonF9translateEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QPolygonF::NewQPolygonF(const QPolygonF & a);
  fn _ZN9QPolygonFC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QPolygonF QPolygonF::translated(qreal dx, qreal dy);
  fn _ZNK9QPolygonF10translatedEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: bool QPolygonF::isClosed();
  fn _ZNK9QPolygonF8isClosedEv() -> i32;
  // proto: QPolygonF QPolygonF::united(const QPolygonF & r);
  fn _ZNK9QPolygonF6unitedERKS_(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QPolygonF)=1
pub struct QPolygonF {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPolygonF {
  pub fn boundingRect<T: QPolygonF_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QPolygonF_boundingRect {
  fn boundingRect(self, this: &mut QPolygonF) -> i32;
}

// proto: QRectF QPolygonF::boundingRect();
impl<'a> /*trait*/ QPolygonF_boundingRect for () {
  fn boundingRect(self, this: &mut QPolygonF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF12boundingRectEv()};
    unsafe {_ZNK9QPolygonF12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QPolygonF {
  pub fn intersected<T: QPolygonF_intersected>(&mut self, value: T) -> i32 {
    value.intersected(self);
    return 1;
  }
}

pub trait QPolygonF_intersected {
  fn intersected(self, this: &mut QPolygonF) -> i32;
}

// proto: QPolygonF QPolygonF::intersected(const QPolygonF & r);
impl<'a> /*trait*/ QPolygonF_intersected for (&'a  QPolygonF) {
  fn intersected(self, this: &mut QPolygonF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF11intersectedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QPolygonF11intersectedERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QPolygonF {
  pub fn NewQPolygonF<T: QPolygonF_NewQPolygonF>(value: T) -> QPolygonF {
    let rsthis = value.NewQPolygonF();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygonF_NewQPolygonF {
  fn NewQPolygonF(self) -> QPolygonF;
}

// proto: void QPolygonF::NewQPolygonF(const QPolygon & a);
impl<'a> /*trait*/ QPolygonF_NewQPolygonF for (&'a  QPolygon) {
  fn NewQPolygonF(self) -> QPolygonF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC1ERK8QPolygon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QPolygonFC1ERK8QPolygon(qthis, arg0)};
    let rsthis = QPolygonF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPolygonF::NewQPolygonF(const QRectF & r);
impl<'a> /*trait*/ QPolygonF_NewQPolygonF for (&'a  QRectF) {
  fn NewQPolygonF(self) -> QPolygonF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC1ERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QPolygonFC1ERK6QRectF(qthis, arg0)};
    let rsthis = QPolygonF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPolygonF {
  pub fn toPolygon<T: QPolygonF_toPolygon>(&mut self, value: T) -> i32 {
    value.toPolygon(self);
    return 1;
  }
}

pub trait QPolygonF_toPolygon {
  fn toPolygon(self, this: &mut QPolygonF) -> i32;
}

// proto: QPolygon QPolygonF::toPolygon();
impl<'a> /*trait*/ QPolygonF_toPolygon for () {
  fn toPolygon(self, this: &mut QPolygonF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF9toPolygonEv()};
    unsafe {_ZNK9QPolygonF9toPolygonEv()};
    return 1;
  }
}

impl /*struct*/ QPolygonF {
  pub fn FreeQPolygonF<T: QPolygonF_FreeQPolygonF>(&mut self, value: T) -> i32 {
    value.FreeQPolygonF(self);
    return 1;
  }
}

pub trait QPolygonF_FreeQPolygonF {
  fn FreeQPolygonF(self, this: &mut QPolygonF) -> i32;
}

// proto: void QPolygonF::FreeQPolygonF();
impl<'a> /*trait*/ QPolygonF_FreeQPolygonF for () {
  fn FreeQPolygonF(self, this: &mut QPolygonF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFD0Ev()};
    unsafe {_ZN9QPolygonFD0Ev()};
    return 1;
  }
}

// proto: void QPolygonF::NewQPolygonF(int size);
impl<'a> /*trait*/ QPolygonF_NewQPolygonF for (i32) {
  fn NewQPolygonF(self) -> QPolygonF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QPolygonFC1Ei(qthis, arg0)};
    let rsthis = QPolygonF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPolygonF {
  pub fn subtracted<T: QPolygonF_subtracted>(&mut self, value: T) -> i32 {
    value.subtracted(self);
    return 1;
  }
}

pub trait QPolygonF_subtracted {
  fn subtracted(self, this: &mut QPolygonF) -> i32;
}

// proto: QPolygonF QPolygonF::subtracted(const QPolygonF & r);
impl<'a> /*trait*/ QPolygonF_subtracted for (&'a  QPolygonF) {
  fn subtracted(self, this: &mut QPolygonF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF10subtractedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QPolygonF10subtractedERKS_(arg0)};
    return 1;
  }
}

// proto: void QPolygonF::NewQPolygonF();
impl<'a> /*trait*/ QPolygonF_NewQPolygonF for () {
  fn NewQPolygonF(self) -> QPolygonF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC1Ev()};
    unsafe {_ZN9QPolygonFC1Ev(qthis)};
    let rsthis = QPolygonF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPolygonF {
  pub fn translate<T: QPolygonF_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QPolygonF_translate {
  fn translate(self, this: &mut QPolygonF) -> i32;
}

// proto: void QPolygonF::translate(const QPointF & offset);
impl<'a> /*trait*/ QPolygonF_translate for (&'a  QPointF) {
  fn translate(self, this: &mut QPolygonF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonF9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QPolygonF9translateERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPolygonF {
  pub fn swap<T: QPolygonF_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QPolygonF_swap {
  fn swap(self, this: &mut QPolygonF) -> i32;
}

// proto: void QPolygonF::swap(QPolygonF & other);
impl<'a> /*trait*/ QPolygonF_swap for (&'a mut QPolygonF) {
  fn swap(self, this: &mut QPolygonF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonF4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QPolygonF4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QPolygonF {
  pub fn translated<T: QPolygonF_translated>(&mut self, value: T) -> i32 {
    value.translated(self);
    return 1;
  }
}

pub trait QPolygonF_translated {
  fn translated(self, this: &mut QPolygonF) -> i32;
}

// proto: QPolygonF QPolygonF::translated(const QPointF & offset);
impl<'a> /*trait*/ QPolygonF_translated for (&'a  QPointF) {
  fn translated(self, this: &mut QPolygonF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF10translatedERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QPolygonF10translatedERK7QPointF(arg0)};
    return 1;
  }
}

// proto: void QPolygonF::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QPolygonF_translate for (f64, f64) {
  fn translate(self, this: &mut QPolygonF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonF9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN9QPolygonF9translateEdd(arg0, arg1)};
    return 1;
  }
}

// proto: void QPolygonF::NewQPolygonF(const QPolygonF & a);
impl<'a> /*trait*/ QPolygonF_NewQPolygonF for (&'a  QPolygonF) {
  fn NewQPolygonF(self) -> QPolygonF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QPolygonFC1ERKS_(qthis, arg0)};
    let rsthis = QPolygonF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QPolygonF QPolygonF::translated(qreal dx, qreal dy);
impl<'a> /*trait*/ QPolygonF_translated for (f64, f64) {
  fn translated(self, this: &mut QPolygonF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF10translatedEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZNK9QPolygonF10translatedEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPolygonF {
  pub fn isClosed<T: QPolygonF_isClosed>(&mut self, value: T) -> i32 {
    value.isClosed(self);
    return 1;
  }
}

pub trait QPolygonF_isClosed {
  fn isClosed(self, this: &mut QPolygonF) -> i32;
}

// proto: bool QPolygonF::isClosed();
impl<'a> /*trait*/ QPolygonF_isClosed for () {
  fn isClosed(self, this: &mut QPolygonF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF8isClosedEv()};
    unsafe {_ZNK9QPolygonF8isClosedEv()};
    return 1;
  }
}

impl /*struct*/ QPolygonF {
  pub fn united<T: QPolygonF_united>(&mut self, value: T) -> i32 {
    value.united(self);
    return 1;
  }
}

pub trait QPolygonF_united {
  fn united(self, this: &mut QPolygonF) -> i32;
}

// proto: QPolygonF QPolygonF::united(const QPolygonF & r);
impl<'a> /*trait*/ QPolygonF_united for (&'a  QPolygonF) {
  fn united(self, this: &mut QPolygonF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF6unitedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QPolygonF6unitedERKS_(arg0)};
    return 1;
  }
}

