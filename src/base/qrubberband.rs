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
  // proto:  void QRubberBand::resize(const QSize & s);
  fn _ZN11QRubberBand6resizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRubberBand::setGeometry(int x, int y, int w, int h);
  fn _ZN11QRubberBand11setGeometryEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QRubberBand::move_(const QPoint & p);
  fn _ZN11QRubberBand4moveERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRubberBand::FreeQRubberBand();
  fn _ZN11QRubberBandD0Ev(qthis: *mut c_void) ;
  // proto:  void QRubberBand::move_(int x, int y);
  fn _ZN11QRubberBand4moveEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  const QMetaObject * QRubberBand::metaObject();
  fn _ZNK11QRubberBand10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QRubberBand::setGeometry(const QRect & r);
  fn _ZN11QRubberBand11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRubberBand::resize(int w, int h);
  fn _ZN11QRubberBand6resizeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
}

// body block begin
// class sizeof(QRubberBand)=1
pub struct QRubberBand {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRubberBand {
  pub fn resize<T: QRubberBand_resize>(&mut self, value: T)  {
     value.resize(self);
    // return 1;
  }
}

pub trait QRubberBand_resize {
  fn resize(self, rsthis: &mut QRubberBand) ;
}

// proto:  void QRubberBand::resize(const QSize & s);
impl<'a> /*trait*/ QRubberBand_resize for (&'a  QSize) {
  fn resize(self, rsthis: &mut QRubberBand)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand6resizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QRubberBand6resizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRubberBand {
  pub fn setGeometry<T: QRubberBand_setGeometry>(&mut self, value: T)  {
     value.setGeometry(self);
    // return 1;
  }
}

pub trait QRubberBand_setGeometry {
  fn setGeometry(self, rsthis: &mut QRubberBand) ;
}

// proto:  void QRubberBand::setGeometry(int x, int y, int w, int h);
impl<'a> /*trait*/ QRubberBand_setGeometry for (i32, i32, i32, i32) {
  fn setGeometry(self, rsthis: &mut QRubberBand)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand11setGeometryEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN11QRubberBand11setGeometryEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QRubberBand {
  pub fn move_<T: QRubberBand_move_>(&mut self, value: T)  {
     value.move_(self);
    // return 1;
  }
}

pub trait QRubberBand_move_ {
  fn move_(self, rsthis: &mut QRubberBand) ;
}

// proto:  void QRubberBand::move_(const QPoint & p);
impl<'a> /*trait*/ QRubberBand_move_ for (&'a  QPoint) {
  fn move_(self, rsthis: &mut QRubberBand)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand4moveERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QRubberBand4moveERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRubberBand {
  pub fn FreeQRubberBand<T: QRubberBand_FreeQRubberBand>(&mut self, value: T)  {
     value.FreeQRubberBand(self);
    // return 1;
  }
}

pub trait QRubberBand_FreeQRubberBand {
  fn FreeQRubberBand(self, rsthis: &mut QRubberBand) ;
}

// proto:  void QRubberBand::FreeQRubberBand();
impl<'a> /*trait*/ QRubberBand_FreeQRubberBand for () {
  fn FreeQRubberBand(self, rsthis: &mut QRubberBand)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBandD0Ev()};
     unsafe {_ZN11QRubberBandD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QRubberBand::move_(int x, int y);
impl<'a> /*trait*/ QRubberBand_move_ for (i32, i32) {
  fn move_(self, rsthis: &mut QRubberBand)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand4moveEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QRubberBand4moveEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QRubberBand {
  pub fn metaObject<T: QRubberBand_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QRubberBand_metaObject {
  fn metaObject(self, rsthis: &mut QRubberBand) ;
}

// proto:  const QMetaObject * QRubberBand::metaObject();
impl<'a> /*trait*/ QRubberBand_metaObject for () {
  fn metaObject(self, rsthis: &mut QRubberBand)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QRubberBand10metaObjectEv()};
     unsafe {_ZNK11QRubberBand10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QRubberBand::setGeometry(const QRect & r);
impl<'a> /*trait*/ QRubberBand_setGeometry for (&'a  QRect) {
  fn setGeometry(self, rsthis: &mut QRubberBand)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QRubberBand11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QRubberBand::resize(int w, int h);
impl<'a> /*trait*/ QRubberBand_resize for (i32, i32) {
  fn resize(self, rsthis: &mut QRubberBand)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand6resizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QRubberBand6resizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

