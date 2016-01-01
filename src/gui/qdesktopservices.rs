// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
// src-file: /QtGui/qdesktopservices.h
// dst-file: /src/gui/qdesktopservices.rs
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
use super::super::core::qstring::QString; // 771
use super::super::core::qurl::QUrl; // 771
use super::super::core::qobject::QObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDesktopServices_Class_Size() -> c_int;
  // proto: static void QDesktopServices::unsetUrlHandler(const QString & scheme);
  fn _ZN16QDesktopServices15unsetUrlHandlerERK7QString(arg0: *mut c_void);
  // proto: static bool QDesktopServices::openUrl(const QUrl & url);
  fn _ZN16QDesktopServices7openUrlERK4QUrl(arg0: *mut c_void) -> c_char;
  // proto: static void QDesktopServices::setUrlHandler(const QString & scheme, QObject * receiver, const char * method);
  fn _ZN16QDesktopServices13setUrlHandlerERK7QStringP7QObjectPKc(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QDesktopServices)=1
#[derive(Default)]
pub struct QDesktopServices {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QDesktopServices {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDesktopServices {
    return QDesktopServices{qclsinst: qthis, ..Default::default()};
  }
}
  // proto: static void QDesktopServices::unsetUrlHandler(const QString & scheme);
impl /*struct*/ QDesktopServices {
  pub fn unsetUrlHandler_s<RetType, T: QDesktopServices_unsetUrlHandler_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.unsetUrlHandler_s();
    // return 1;
  }
}

pub trait QDesktopServices_unsetUrlHandler_s<RetType> {
  fn unsetUrlHandler_s(self ) -> RetType;
}

  // proto: static void QDesktopServices::unsetUrlHandler(const QString & scheme);
impl<'a> /*trait*/ QDesktopServices_unsetUrlHandler_s<()> for (&'a QString) {
  fn unsetUrlHandler_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDesktopServices15unsetUrlHandlerERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QDesktopServices15unsetUrlHandlerERK7QString(arg0)};
    // return 1;
  }
}

  // proto: static bool QDesktopServices::openUrl(const QUrl & url);
impl /*struct*/ QDesktopServices {
  pub fn openUrl_s<RetType, T: QDesktopServices_openUrl_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.openUrl_s();
    // return 1;
  }
}

pub trait QDesktopServices_openUrl_s<RetType> {
  fn openUrl_s(self ) -> RetType;
}

  // proto: static bool QDesktopServices::openUrl(const QUrl & url);
impl<'a> /*trait*/ QDesktopServices_openUrl_s<i8> for (&'a QUrl) {
  fn openUrl_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDesktopServices7openUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QDesktopServices7openUrlERK4QUrl(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void QDesktopServices::setUrlHandler(const QString & scheme, QObject * receiver, const char * method);
impl /*struct*/ QDesktopServices {
  pub fn setUrlHandler_s<RetType, T: QDesktopServices_setUrlHandler_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setUrlHandler_s();
    // return 1;
  }
}

pub trait QDesktopServices_setUrlHandler_s<RetType> {
  fn setUrlHandler_s(self ) -> RetType;
}

  // proto: static void QDesktopServices::setUrlHandler(const QString & scheme, QObject * receiver, const char * method);
impl<'a> /*trait*/ QDesktopServices_setUrlHandler_s<()> for (&'a QString, &'a QObject, &'a  String) {
  fn setUrlHandler_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDesktopServices13setUrlHandlerERK7QStringP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *mut c_char;
     unsafe {_ZN16QDesktopServices13setUrlHandlerERK7QStringP7QObjectPKc(arg0, arg1, arg2)};
    // return 1;
  }
}

// <= body block end

