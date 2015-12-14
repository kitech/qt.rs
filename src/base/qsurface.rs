// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsurfaceformat::QSurfaceFormat;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QSurface::supportsOpenGL();
  fn _ZNK8QSurface14supportsOpenGLEv(qthis: *mut c_void) -> int8_t;
  // proto:  QSurfaceFormat QSurface::format();
  fn _ZNK8QSurface6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPlatformSurface * QSurface::surfaceHandle();
  fn _ZNK8QSurface13surfaceHandleEv(qthis: *mut c_void) ;
  // proto:  QSize QSurface::size();
  fn _ZNK8QSurface4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSurface::FreeQSurface();
  fn _ZN8QSurfaceD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QSurface)=24
pub struct QSurface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSurface {
  pub fn supportsOpenGL<T: QSurface_supportsOpenGL>(&mut self, value: T) -> i8 {
    return value.supportsOpenGL(self);
    // return 1;
  }
}

pub trait QSurface_supportsOpenGL {
  fn supportsOpenGL(self, rsthis: &mut QSurface) -> i8;
}

// proto:  bool QSurface::supportsOpenGL();
impl<'a> /*trait*/ QSurface_supportsOpenGL for () {
  fn supportsOpenGL(self, rsthis: &mut QSurface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface14supportsOpenGLEv()};
    let mut ret = unsafe {_ZNK8QSurface14supportsOpenGLEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSurface {
  pub fn format<T: QSurface_format>(&mut self, value: T) -> QSurfaceFormat {
    return value.format(self);
    // return 1;
  }
}

pub trait QSurface_format {
  fn format(self, rsthis: &mut QSurface) -> QSurfaceFormat;
}

// proto:  QSurfaceFormat QSurface::format();
impl<'a> /*trait*/ QSurface_format for () {
  fn format(self, rsthis: &mut QSurface) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface6formatEv()};
    let mut ret = unsafe {_ZNK8QSurface6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSurface {
  pub fn surfaceHandle<T: QSurface_surfaceHandle>(&mut self, value: T)  {
     value.surfaceHandle(self);
    // return 1;
  }
}

pub trait QSurface_surfaceHandle {
  fn surfaceHandle(self, rsthis: &mut QSurface) ;
}

// proto:  QPlatformSurface * QSurface::surfaceHandle();
impl<'a> /*trait*/ QSurface_surfaceHandle for () {
  fn surfaceHandle(self, rsthis: &mut QSurface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface13surfaceHandleEv()};
     unsafe {_ZNK8QSurface13surfaceHandleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSurface {
  pub fn size<T: QSurface_size>(&mut self, value: T) -> QSize {
    return value.size(self);
    // return 1;
  }
}

pub trait QSurface_size {
  fn size(self, rsthis: &mut QSurface) -> QSize;
}

// proto:  QSize QSurface::size();
impl<'a> /*trait*/ QSurface_size for () {
  fn size(self, rsthis: &mut QSurface) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface4sizeEv()};
    let mut ret = unsafe {_ZNK8QSurface4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSurface {
  pub fn FreeQSurface<T: QSurface_FreeQSurface>(&mut self, value: T)  {
     value.FreeQSurface(self);
    // return 1;
  }
}

pub trait QSurface_FreeQSurface {
  fn FreeQSurface(self, rsthis: &mut QSurface) ;
}

// proto:  void QSurface::FreeQSurface();
impl<'a> /*trait*/ QSurface_FreeQSurface for () {
  fn FreeQSurface(self, rsthis: &mut QSurface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSurfaceD0Ev()};
     unsafe {_ZN8QSurfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

