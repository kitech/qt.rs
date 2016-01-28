// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtGui/qsurface.h
// dst-file: /src/gui/qsurface.rs
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
use super::qsurfaceformat::*; // 773
// use super::qplatformsurface::*; // 775
use super::super::core::qsize::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSurface_Class_Size() -> c_int;
  // proto:  bool QSurface::supportsOpenGL();
  fn C_ZNK8QSurface14supportsOpenGLEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSurfaceFormat QSurface::format();
  fn C_ZNK8QSurface6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPlatformSurface * QSurface::surfaceHandle();
  fn C_ZNK8QSurface13surfaceHandleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QSurface::size();
  fn C_ZNK8QSurface4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSurface::~QSurface();
  fn C_ZN8QSurfaceD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QSurface)=24
#[derive(Default)]
pub struct QSurface {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSurface {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSurface {
    return QSurface{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QSurface::supportsOpenGL();
impl /*struct*/ QSurface {
  pub fn supportsOpenGL<RetType, T: QSurface_supportsOpenGL<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.supportsOpenGL(self);
    // return 1;
  }
}

pub trait QSurface_supportsOpenGL<RetType> {
  fn supportsOpenGL(self , rsthis: & QSurface) -> RetType;
}

  // proto:  bool QSurface::supportsOpenGL();
impl<'a> /*trait*/ QSurface_supportsOpenGL<i8> for () {
  fn supportsOpenGL(self , rsthis: & QSurface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface14supportsOpenGLEv()};
    let mut ret = unsafe {C_ZNK8QSurface14supportsOpenGLEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QSurfaceFormat QSurface::format();
impl /*struct*/ QSurface {
  pub fn format<RetType, T: QSurface_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QSurface_format<RetType> {
  fn format(self , rsthis: & QSurface) -> RetType;
}

  // proto:  QSurfaceFormat QSurface::format();
impl<'a> /*trait*/ QSurface_format<QSurfaceFormat> for () {
  fn format(self , rsthis: & QSurface) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface6formatEv()};
    let mut ret = unsafe {C_ZNK8QSurface6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPlatformSurface * QSurface::surfaceHandle();
impl /*struct*/ QSurface {
  pub fn surfaceHandle<RetType, T: QSurface_surfaceHandle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.surfaceHandle(self);
    // return 1;
  }
}

pub trait QSurface_surfaceHandle<RetType> {
  fn surfaceHandle(self , rsthis: & QSurface) -> RetType;
}

  // proto:  QPlatformSurface * QSurface::surfaceHandle();
impl<'a> /*trait*/ QSurface_surfaceHandle<u64> for () {
  fn surfaceHandle(self , rsthis: & QSurface) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface13surfaceHandleEv()};
    let mut ret = unsafe {C_ZNK8QSurface13surfaceHandleEv(rsthis.qclsinst)};
    return ret as u64; // 4
    // return 1;
  }
}

  // proto:  QSize QSurface::size();
impl /*struct*/ QSurface {
  pub fn size<RetType, T: QSurface_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QSurface_size<RetType> {
  fn size(self , rsthis: & QSurface) -> RetType;
}

  // proto:  QSize QSurface::size();
impl<'a> /*trait*/ QSurface_size<QSize> for () {
  fn size(self , rsthis: & QSurface) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QSurface4sizeEv()};
    let mut ret = unsafe {C_ZNK8QSurface4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSurface::~QSurface();
impl /*struct*/ QSurface {
  pub fn free<RetType, T: QSurface_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSurface_free<RetType> {
  fn free(self , rsthis: & QSurface) -> RetType;
}

  // proto:  void QSurface::~QSurface();
impl<'a> /*trait*/ QSurface_free<()> for () {
  fn free(self , rsthis: & QSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSurfaceD2Ev()};
     unsafe {C_ZN8QSurfaceD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

