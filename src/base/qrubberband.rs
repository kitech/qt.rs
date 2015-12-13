// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qpoint::QPoint;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QRubberBand::resize(const QSize & s);
  fn _ZN11QRubberBand6resizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: void QRubberBand::setGeometry(int x, int y, int w, int h);
  fn _ZN11QRubberBand11setGeometryEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QRubberBand::move_(const QPoint & p);
  fn _ZN11QRubberBand4moveERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QRubberBand::FreeQRubberBand();
  fn _ZN11QRubberBandD0Ev() -> i32;
  // proto: void QRubberBand::move_(int x, int y);
  fn _ZN11QRubberBand4moveEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: const QMetaObject * QRubberBand::metaObject();
  fn _ZNK11QRubberBand10metaObjectEv() -> i32;
  // proto: void QRubberBand::setGeometry(const QRect & r);
  fn _ZN11QRubberBand11setGeometryERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QRubberBand::resize(int w, int h);
  fn _ZN11QRubberBand6resizeEii(arg0: c_int, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QRubberBand)=1
pub struct QRubberBand {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRubberBand {
  pub fn resize<T: QRubberBand_resize>(&mut self, value: T) -> i32 {
    value.resize(self);
    return 1;
  }
}

pub trait QRubberBand_resize {
  fn resize(self, this: &mut QRubberBand) -> i32;
}

// proto: void QRubberBand::resize(const QSize & s);
impl<'a> /*trait*/ QRubberBand_resize for (&'a  QSize) {
  fn resize(self, this: &mut QRubberBand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand6resizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QRubberBand6resizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QRubberBand {
  pub fn setGeometry<T: QRubberBand_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QRubberBand_setGeometry {
  fn setGeometry(self, this: &mut QRubberBand) -> i32;
}

// proto: void QRubberBand::setGeometry(int x, int y, int w, int h);
impl<'a> /*trait*/ QRubberBand_setGeometry for (i32, i32, i32, i32) {
  fn setGeometry(self, this: &mut QRubberBand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand11setGeometryEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN11QRubberBand11setGeometryEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QRubberBand {
  pub fn move_<T: QRubberBand_move_>(&mut self, value: T) -> i32 {
    value.move_(self);
    return 1;
  }
}

pub trait QRubberBand_move_ {
  fn move_(self, this: &mut QRubberBand) -> i32;
}

// proto: void QRubberBand::move_(const QPoint & p);
impl<'a> /*trait*/ QRubberBand_move_ for (&'a  QPoint) {
  fn move_(self, this: &mut QRubberBand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand4moveERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QRubberBand4moveERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QRubberBand {
  pub fn FreeQRubberBand<T: QRubberBand_FreeQRubberBand>(&mut self, value: T) -> i32 {
    value.FreeQRubberBand(self);
    return 1;
  }
}

pub trait QRubberBand_FreeQRubberBand {
  fn FreeQRubberBand(self, this: &mut QRubberBand) -> i32;
}

// proto: void QRubberBand::FreeQRubberBand();
impl<'a> /*trait*/ QRubberBand_FreeQRubberBand for () {
  fn FreeQRubberBand(self, this: &mut QRubberBand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBandD0Ev()};
    unsafe {_ZN11QRubberBandD0Ev()};
    return 1;
  }
}

// proto: void QRubberBand::move_(int x, int y);
impl<'a> /*trait*/ QRubberBand_move_ for (i32, i32) {
  fn move_(self, this: &mut QRubberBand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand4moveEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QRubberBand4moveEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QRubberBand {
  pub fn metaObject<T: QRubberBand_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QRubberBand_metaObject {
  fn metaObject(self, this: &mut QRubberBand) -> i32;
}

// proto: const QMetaObject * QRubberBand::metaObject();
impl<'a> /*trait*/ QRubberBand_metaObject for () {
  fn metaObject(self, this: &mut QRubberBand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QRubberBand10metaObjectEv()};
    unsafe {_ZNK11QRubberBand10metaObjectEv()};
    return 1;
  }
}

// proto: void QRubberBand::setGeometry(const QRect & r);
impl<'a> /*trait*/ QRubberBand_setGeometry for (&'a  QRect) {
  fn setGeometry(self, this: &mut QRubberBand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QRubberBand11setGeometryERK5QRect(arg0)};
    return 1;
  }
}

// proto: void QRubberBand::resize(int w, int h);
impl<'a> /*trait*/ QRubberBand_resize for (i32, i32) {
  fn resize(self, this: &mut QRubberBand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand6resizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QRubberBand6resizeEii(arg0, arg1)};
    return 1;
  }
}

