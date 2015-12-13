// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QSurface::supportsOpenGL();
  fn _ZNK8QSurface14supportsOpenGLEv() -> i32;
  // proto: QSurfaceFormat QSurface::format();
  fn _ZNK8QSurface6formatEv() -> i32;
  // proto: QPlatformSurface * QSurface::surfaceHandle();
  fn _ZNK8QSurface13surfaceHandleEv() -> i32;
  // proto: QSize QSurface::size();
  fn _ZNK8QSurface4sizeEv() -> i32;
  // proto: void QSurface::FreeQSurface();
  fn _ZN8QSurfaceD0Ev() -> i32;
}

// body block begin
// class sizeof(QSurface)=24
pub struct QSurface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSurface {
  pub fn supportsOpenGL<T: QSurface_supportsOpenGL>(&mut self, value: T) -> i32 {
    value.supportsOpenGL(self);
    return 1;
  }
}

pub trait QSurface_supportsOpenGL {
  fn supportsOpenGL(self, this: &mut QSurface) -> i32;
}

// proto: bool QSurface::supportsOpenGL();
impl<'a> /*trait*/ QSurface_supportsOpenGL for () {
  fn supportsOpenGL(self, this: &mut QSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface14supportsOpenGLEv()};
    unsafe {_ZNK8QSurface14supportsOpenGLEv()};
    return 1;
  }
}

impl /*struct*/ QSurface {
  pub fn format<T: QSurface_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QSurface_format {
  fn format(self, this: &mut QSurface) -> i32;
}

// proto: QSurfaceFormat QSurface::format();
impl<'a> /*trait*/ QSurface_format for () {
  fn format(self, this: &mut QSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface6formatEv()};
    unsafe {_ZNK8QSurface6formatEv()};
    return 1;
  }
}

impl /*struct*/ QSurface {
  pub fn surfaceHandle<T: QSurface_surfaceHandle>(&mut self, value: T) -> i32 {
    value.surfaceHandle(self);
    return 1;
  }
}

pub trait QSurface_surfaceHandle {
  fn surfaceHandle(self, this: &mut QSurface) -> i32;
}

// proto: QPlatformSurface * QSurface::surfaceHandle();
impl<'a> /*trait*/ QSurface_surfaceHandle for () {
  fn surfaceHandle(self, this: &mut QSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface13surfaceHandleEv()};
    unsafe {_ZNK8QSurface13surfaceHandleEv()};
    return 1;
  }
}

impl /*struct*/ QSurface {
  pub fn size<T: QSurface_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QSurface_size {
  fn size(self, this: &mut QSurface) -> i32;
}

// proto: QSize QSurface::size();
impl<'a> /*trait*/ QSurface_size for () {
  fn size(self, this: &mut QSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface4sizeEv()};
    unsafe {_ZNK8QSurface4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QSurface {
  pub fn FreeQSurface<T: QSurface_FreeQSurface>(&mut self, value: T) -> i32 {
    value.FreeQSurface(self);
    return 1;
  }
}

pub trait QSurface_FreeQSurface {
  fn FreeQSurface(self, this: &mut QSurface) -> i32;
}

// proto: void QSurface::FreeQSurface();
impl<'a> /*trait*/ QSurface_FreeQSurface for () {
  fn FreeQSurface(self, this: &mut QSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSurfaceD0Ev()};
    unsafe {_ZN8QSurfaceD0Ev()};
    return 1;
  }
}

