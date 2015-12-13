// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QRect QPolygon::boundingRect();
  fn _ZNK8QPolygon12boundingRectEv() -> i32;
  // proto: void QPolygon::setPoint(int index, int x, int y);
  fn _ZN8QPolygon8setPointEiii(arg0: c_int, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QPolygon::FreeQPolygon();
  fn _ZN8QPolygonD0Ev() -> i32;
  // proto: void QPolygon::putPoints(int index, int nPoints, const QPolygon & from, int fromIndex);
  fn _ZN8QPolygon9putPointsEiiRKS_i(arg0: c_int, arg1: c_int, arg2: *const c_void, arg3: c_int) -> i32;
  // proto: QPolygon QPolygon::translated(const QPoint & offset);
  fn _ZNK8QPolygon10translatedERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QPolygon QPolygon::subtracted(const QPolygon & r);
  fn _ZNK8QPolygon10subtractedERKS_(arg0: *const c_void) -> i32;
  // proto: QPolygon QPolygon::intersected(const QPolygon & r);
  fn _ZNK8QPolygon11intersectedERKS_(arg0: *const c_void) -> i32;
  // proto: void QPolygon::setPoint(int index, const QPoint & p);
  fn _ZN8QPolygon8setPointEiRK6QPoint(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QPolygon::point(int i, int * x, int * y);
  fn _ZNK8QPolygon5pointEiPiS0_(arg0: c_int, arg1: *mut c_int, arg2: *mut c_int) -> i32;
  // proto: void QPolygon::translate(int dx, int dy);
  fn _ZN8QPolygon9translateEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QPolygon::putPoints(int index, int nPoints, int firstx, int firsty);
  fn _ZN8QPolygon9putPointsEiiiiz(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QPolygon::setPoints(int nPoints, int firstx, int firsty);
  fn _ZN8QPolygon9setPointsEiiiz(arg0: c_int, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QPolygon::translate(const QPoint & offset);
  fn _ZN8QPolygon9translateERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QPolygon::swap(QPolygon & other);
  fn _ZN8QPolygon4swapERS_(arg0: *mut c_void) -> i32;
  // proto: QPoint QPolygon::point(int i);
  fn _ZNK8QPolygon5pointEi(arg0: c_int) -> i32;
  // proto: void QPolygon::NewQPolygon(const QPolygon & a);
  fn _ZN8QPolygonC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QPolygon::NewQPolygon(int nPoints, const int * points);
  fn _ZN8QPolygonC1EiPKi(qthis: *mut c_void, arg0: c_int, arg1: *const c_int) -> i32;
  // proto: QPolygon QPolygon::united(const QPolygon & r);
  fn _ZNK8QPolygon6unitedERKS_(arg0: *const c_void) -> i32;
  // proto: QPolygon QPolygon::translated(int dx, int dy);
  fn _ZNK8QPolygon10translatedEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QPolygon::putPoints(int index, int nPoints, const int * points);
  fn _ZN8QPolygon9putPointsEiiPKi(arg0: c_int, arg1: c_int, arg2: *const c_int) -> i32;
  // proto: void QPolygon::setPoints(int nPoints, const int * points);
  fn _ZN8QPolygon9setPointsEiPKi(arg0: c_int, arg1: *const c_int) -> i32;
  // proto: void QPolygon::NewQPolygon(int size);
  fn _ZN8QPolygonC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  // proto: void QPolygon::NewQPolygon();
  fn _ZN8QPolygonC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QPolygon::NewQPolygon(const QRect & r, bool closed);
  fn _ZN8QPolygonC1ERK5QRectb(qthis: *mut c_void, arg0: *const c_void, arg1: int8_t) -> i32;
}

// body block begin
// class sizeof(QPolygon)=1
pub struct QPolygon {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPolygon {
  pub fn boundingRect<T: QPolygon_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QPolygon_boundingRect {
  fn boundingRect(self, this: &mut QPolygon) -> i32;
}

// proto: QRect QPolygon::boundingRect();
impl<'a> /*trait*/ QPolygon_boundingRect for () {
  fn boundingRect(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon12boundingRectEv()};
    unsafe {_ZNK8QPolygon12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn setPoint<T: QPolygon_setPoint>(&mut self, value: T) -> i32 {
    value.setPoint(self);
    return 1;
  }
}

pub trait QPolygon_setPoint {
  fn setPoint(self, this: &mut QPolygon) -> i32;
}

// proto: void QPolygon::setPoint(int index, int x, int y);
impl<'a> /*trait*/ QPolygon_setPoint for (i32, i32, i32) {
  fn setPoint(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon8setPointEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN8QPolygon8setPointEiii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn FreeQPolygon<T: QPolygon_FreeQPolygon>(&mut self, value: T) -> i32 {
    value.FreeQPolygon(self);
    return 1;
  }
}

pub trait QPolygon_FreeQPolygon {
  fn FreeQPolygon(self, this: &mut QPolygon) -> i32;
}

// proto: void QPolygon::FreeQPolygon();
impl<'a> /*trait*/ QPolygon_FreeQPolygon for () {
  fn FreeQPolygon(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonD0Ev()};
    unsafe {_ZN8QPolygonD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn putPoints<T: QPolygon_putPoints>(&mut self, value: T) -> i32 {
    value.putPoints(self);
    return 1;
  }
}

pub trait QPolygon_putPoints {
  fn putPoints(self, this: &mut QPolygon) -> i32;
}

// proto: void QPolygon::putPoints(int index, int nPoints, const QPolygon & from, int fromIndex);
impl<'a> /*trait*/ QPolygon_putPoints for (i32, i32, &'a  QPolygon, i32) {
  fn putPoints(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9putPointsEiiRKS_i()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3  as c_int;
    unsafe {_ZN8QPolygon9putPointsEiiRKS_i(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn translated<T: QPolygon_translated>(&mut self, value: T) -> i32 {
    value.translated(self);
    return 1;
  }
}

pub trait QPolygon_translated {
  fn translated(self, this: &mut QPolygon) -> i32;
}

// proto: QPolygon QPolygon::translated(const QPoint & offset);
impl<'a> /*trait*/ QPolygon_translated for (&'a  QPoint) {
  fn translated(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK8QPolygon10translatedERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn subtracted<T: QPolygon_subtracted>(&mut self, value: T) -> i32 {
    value.subtracted(self);
    return 1;
  }
}

pub trait QPolygon_subtracted {
  fn subtracted(self, this: &mut QPolygon) -> i32;
}

// proto: QPolygon QPolygon::subtracted(const QPolygon & r);
impl<'a> /*trait*/ QPolygon_subtracted for (&'a  QPolygon) {
  fn subtracted(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon10subtractedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK8QPolygon10subtractedERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn intersected<T: QPolygon_intersected>(&mut self, value: T) -> i32 {
    value.intersected(self);
    return 1;
  }
}

pub trait QPolygon_intersected {
  fn intersected(self, this: &mut QPolygon) -> i32;
}

// proto: QPolygon QPolygon::intersected(const QPolygon & r);
impl<'a> /*trait*/ QPolygon_intersected for (&'a  QPolygon) {
  fn intersected(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon11intersectedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK8QPolygon11intersectedERKS_(arg0)};
    return 1;
  }
}

// proto: void QPolygon::setPoint(int index, const QPoint & p);
impl<'a> /*trait*/ QPolygon_setPoint for (i32, &'a  QPoint) {
  fn setPoint(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon8setPointEiRK6QPoint()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPolygon8setPointEiRK6QPoint(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn point<T: QPolygon_point>(&mut self, value: T) -> i32 {
    value.point(self);
    return 1;
  }
}

pub trait QPolygon_point {
  fn point(self, this: &mut QPolygon) -> i32;
}

// proto: void QPolygon::point(int i, int * x, int * y);
impl<'a> /*trait*/ QPolygon_point for (i32, &'a mut i32, &'a mut i32) {
  fn point(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon5pointEiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    unsafe {_ZNK8QPolygon5pointEiPiS0_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn translate<T: QPolygon_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QPolygon_translate {
  fn translate(self, this: &mut QPolygon) -> i32;
}

// proto: void QPolygon::translate(int dx, int dy);
impl<'a> /*trait*/ QPolygon_translate for (i32, i32) {
  fn translate(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9translateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPolygon9translateEii(arg0, arg1)};
    return 1;
  }
}

// proto: void QPolygon::putPoints(int index, int nPoints, int firstx, int firsty);
impl<'a> /*trait*/ QPolygon_putPoints for (i32, i32, i32, i32) {
  fn putPoints(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9putPointsEiiiiz()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN8QPolygon9putPointsEiiiiz(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn setPoints<T: QPolygon_setPoints>(&mut self, value: T) -> i32 {
    value.setPoints(self);
    return 1;
  }
}

pub trait QPolygon_setPoints {
  fn setPoints(self, this: &mut QPolygon) -> i32;
}

// proto: void QPolygon::setPoints(int nPoints, int firstx, int firsty);
impl<'a> /*trait*/ QPolygon_setPoints for (i32, i32, i32) {
  fn setPoints(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9setPointsEiiiz()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN8QPolygon9setPointsEiiiz(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QPolygon::translate(const QPoint & offset);
impl<'a> /*trait*/ QPolygon_translate for (&'a  QPoint) {
  fn translate(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPolygon9translateERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn swap<T: QPolygon_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QPolygon_swap {
  fn swap(self, this: &mut QPolygon) -> i32;
}

// proto: void QPolygon::swap(QPolygon & other);
impl<'a> /*trait*/ QPolygon_swap for (&'a mut QPolygon) {
  fn swap(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QPolygon4swapERS_(arg0)};
    return 1;
  }
}

// proto: QPoint QPolygon::point(int i);
impl<'a> /*trait*/ QPolygon_point for (i32) {
  fn point(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon5pointEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK8QPolygon5pointEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn NewQPolygon<T: QPolygon_NewQPolygon>(value: T) -> QPolygon {
    let rsthis = value.NewQPolygon();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygon_NewQPolygon {
  fn NewQPolygon(self) -> QPolygon;
}

// proto: void QPolygon::NewQPolygon(const QPolygon & a);
impl<'a> /*trait*/ QPolygon_NewQPolygon for (&'a  QPolygon) {
  fn NewQPolygon(self) -> QPolygon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPolygonC1ERKS_(qthis, arg0)};
    let rsthis = QPolygon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPolygon::NewQPolygon(int nPoints, const int * points);
impl<'a> /*trait*/ QPolygon_NewQPolygon for (i32, &'a  i32) {
  fn NewQPolygon(self) -> QPolygon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC1EiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_int;
    unsafe {_ZN8QPolygonC1EiPKi(qthis, arg0, arg1)};
    let rsthis = QPolygon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn united<T: QPolygon_united>(&mut self, value: T) -> i32 {
    value.united(self);
    return 1;
  }
}

pub trait QPolygon_united {
  fn united(self, this: &mut QPolygon) -> i32;
}

// proto: QPolygon QPolygon::united(const QPolygon & r);
impl<'a> /*trait*/ QPolygon_united for (&'a  QPolygon) {
  fn united(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon6unitedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK8QPolygon6unitedERKS_(arg0)};
    return 1;
  }
}

// proto: QPolygon QPolygon::translated(int dx, int dy);
impl<'a> /*trait*/ QPolygon_translated for (i32, i32) {
  fn translated(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK8QPolygon10translatedEii(arg0, arg1)};
    return 1;
  }
}

// proto: void QPolygon::putPoints(int index, int nPoints, const int * points);
impl<'a> /*trait*/ QPolygon_putPoints for (i32, i32, &'a  i32) {
  fn putPoints(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9putPointsEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_int;
    unsafe {_ZN8QPolygon9putPointsEiiPKi(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QPolygon::setPoints(int nPoints, const int * points);
impl<'a> /*trait*/ QPolygon_setPoints for (i32, &'a  i32) {
  fn setPoints(self, this: &mut QPolygon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9setPointsEiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_int;
    unsafe {_ZN8QPolygon9setPointsEiPKi(arg0, arg1)};
    return 1;
  }
}

// proto: void QPolygon::NewQPolygon(int size);
impl<'a> /*trait*/ QPolygon_NewQPolygon for (i32) {
  fn NewQPolygon(self) -> QPolygon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QPolygonC1Ei(qthis, arg0)};
    let rsthis = QPolygon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPolygon::NewQPolygon();
impl<'a> /*trait*/ QPolygon_NewQPolygon for () {
  fn NewQPolygon(self) -> QPolygon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC1Ev()};
    unsafe {_ZN8QPolygonC1Ev(qthis)};
    let rsthis = QPolygon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPolygon::NewQPolygon(const QRect & r, bool closed);
impl<'a> /*trait*/ QPolygon_NewQPolygon for (&'a  QRect, i8) {
  fn NewQPolygon(self) -> QPolygon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC1ERK5QRectb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN8QPolygonC1ERK5QRectb(qthis, arg0, arg1)};
    let rsthis = QPolygon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

