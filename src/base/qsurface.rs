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

// proto:  bool QSurface::supportsOpenGL();
impl /*struct*/ QSurface {
  pub fn supportsOpenGL<RetType, T: QSurface_supportsOpenGL<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.supportsOpenGL(self);
    // return 1;
  }
}

pub trait QSurface_supportsOpenGL<RetType> {
  fn supportsOpenGL(self , rsthis: &mut QSurface) -> RetType;
}

// proto:  bool QSurface::supportsOpenGL();
impl<'a> /*trait*/ QSurface_supportsOpenGL<i8> for () {
  fn supportsOpenGL(self , rsthis: &mut QSurface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface14supportsOpenGLEv()};
    let mut ret = unsafe {_ZNK8QSurface14supportsOpenGLEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QSurfaceFormat QSurface::format();
impl /*struct*/ QSurface {
  pub fn format<RetType, T: QSurface_format<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QSurface_format<RetType> {
  fn format(self , rsthis: &mut QSurface) -> RetType;
}

// proto:  QSurfaceFormat QSurface::format();
impl<'a> /*trait*/ QSurface_format<QSurfaceFormat> for () {
  fn format(self , rsthis: &mut QSurface) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface6formatEv()};
    let mut ret = unsafe {_ZNK8QSurface6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPlatformSurface * QSurface::surfaceHandle();
impl /*struct*/ QSurface {
  pub fn surfaceHandle<RetType, T: QSurface_surfaceHandle<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.surfaceHandle(self);
    // return 1;
  }
}

pub trait QSurface_surfaceHandle<RetType> {
  fn surfaceHandle(self , rsthis: &mut QSurface) -> RetType;
}

// proto:  QPlatformSurface * QSurface::surfaceHandle();
impl<'a> /*trait*/ QSurface_surfaceHandle<()> for () {
  fn surfaceHandle(self , rsthis: &mut QSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface13surfaceHandleEv()};
     unsafe {_ZNK8QSurface13surfaceHandleEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QSize QSurface::size();
impl /*struct*/ QSurface {
  pub fn size<RetType, T: QSurface_size<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QSurface_size<RetType> {
  fn size(self , rsthis: &mut QSurface) -> RetType;
}

// proto:  QSize QSurface::size();
impl<'a> /*trait*/ QSurface_size<QSize> for () {
  fn size(self , rsthis: &mut QSurface) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface4sizeEv()};
    let mut ret = unsafe {_ZNK8QSurface4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QSurface::FreeQSurface();
impl /*struct*/ QSurface {
  pub fn FreeQSurface<RetType, T: QSurface_FreeQSurface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQSurface(self);
    // return 1;
  }
}

pub trait QSurface_FreeQSurface<RetType> {
  fn FreeQSurface(self , rsthis: &mut QSurface) -> RetType;
}

// proto:  void QSurface::FreeQSurface();
impl<'a> /*trait*/ QSurface_FreeQSurface<()> for () {
  fn FreeQSurface(self , rsthis: &mut QSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSurfaceD0Ev()};
     unsafe {_ZN8QSurfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

