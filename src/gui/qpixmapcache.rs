// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtGui/qpixmapcache.h
// dst-file: /src/gui/qpixmapcache.rs
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
use super::qpixmap::QPixmap; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPixmapCache_Class_Size() -> c_int;
  // proto: static int QPixmapCache::cacheLimit();
  fn _ZN12QPixmapCache10cacheLimitEv() -> c_int;
  // proto: static void QPixmapCache::clear();
  fn _ZN12QPixmapCache5clearEv();
  // proto: static bool QPixmapCache::insert(const QString & key, const QPixmap & pixmap);
  fn _ZN12QPixmapCache6insertERK7QStringRK7QPixmap(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static bool QPixmapCache::find(const QString & key, QPixmap & pixmap);
  fn _ZN12QPixmapCache4findERK7QStringR7QPixmap(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static QPixmap * QPixmapCache::find(const QString & key);
  fn _ZN12QPixmapCache4findERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static bool QPixmapCache::find(const QString & key, QPixmap * pixmap);
  fn _ZN12QPixmapCache4findERK7QStringP7QPixmap(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static void QPixmapCache::remove(const QString & key);
  fn _ZN12QPixmapCache6removeERK7QString(arg0: *mut c_void);
  // proto: static void QPixmapCache::setCacheLimit(int );
  fn _ZN12QPixmapCache13setCacheLimitEi(arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QPixmapCache)=1
pub struct QPixmapCache {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPixmapCache {
  pub fn inheritFrom(qthis: *mut c_void) -> QPixmapCache {
    return QPixmapCache{qclsinst: qthis};
  }
}
  // proto: static int QPixmapCache::cacheLimit();
impl /*struct*/ QPixmapCache {
  pub fn cacheLimit_s<RetType, T: QPixmapCache_cacheLimit_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.cacheLimit_s();
    // return 1;
  }
}

pub trait QPixmapCache_cacheLimit_s<RetType> {
  fn cacheLimit_s(self ) -> RetType;
}

  // proto: static int QPixmapCache::cacheLimit();
impl<'a> /*trait*/ QPixmapCache_cacheLimit_s<i32> for () {
  fn cacheLimit_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache10cacheLimitEv()};
    let mut ret = unsafe {_ZN12QPixmapCache10cacheLimitEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto: static void QPixmapCache::clear();
impl /*struct*/ QPixmapCache {
  pub fn clear_s<RetType, T: QPixmapCache_clear_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.clear_s();
    // return 1;
  }
}

pub trait QPixmapCache_clear_s<RetType> {
  fn clear_s(self ) -> RetType;
}

  // proto: static void QPixmapCache::clear();
impl<'a> /*trait*/ QPixmapCache_clear_s<()> for () {
  fn clear_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache5clearEv()};
     unsafe {_ZN12QPixmapCache5clearEv()};
    // return 1;
  }
}

  // proto: static bool QPixmapCache::insert(const QString & key, const QPixmap & pixmap);
impl /*struct*/ QPixmapCache {
  pub fn insert_s<RetType, T: QPixmapCache_insert_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.insert_s();
    // return 1;
  }
}

pub trait QPixmapCache_insert_s<RetType> {
  fn insert_s(self ) -> RetType;
}

  // proto: static bool QPixmapCache::insert(const QString & key, const QPixmap & pixmap);
impl<'a> /*trait*/ QPixmapCache_insert_s<i8> for (&'a QString, &'a QPixmap) {
  fn insert_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache6insertERK7QStringRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QPixmapCache6insertERK7QStringRK7QPixmap(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QPixmapCache::find(const QString & key, QPixmap & pixmap);
impl /*struct*/ QPixmapCache {
  pub fn find_s<RetType, T: QPixmapCache_find_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.find_s();
    // return 1;
  }
}

pub trait QPixmapCache_find_s<RetType> {
  fn find_s(self ) -> RetType;
}

  // proto: static bool QPixmapCache::find(const QString & key, QPixmap & pixmap);
impl<'a> /*trait*/ QPixmapCache_find_s<i8> for (&'a QString, &'a QPixmap) {
  fn find_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache4findERK7QStringR7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QPixmapCache4findERK7QStringR7QPixmap(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QPixmap * QPixmapCache::find(const QString & key);
impl<'a> /*trait*/ QPixmapCache_find_s<QPixmap> for (&'a QString) {
  fn find_s(self ) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache4findERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QPixmapCache4findERK7QString(arg0)};
    let mut ret1 = QPixmap::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QPixmapCache::remove(const QString & key);
impl /*struct*/ QPixmapCache {
  pub fn remove_s<RetType, T: QPixmapCache_remove_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.remove_s();
    // return 1;
  }
}

pub trait QPixmapCache_remove_s<RetType> {
  fn remove_s(self ) -> RetType;
}

  // proto: static void QPixmapCache::remove(const QString & key);
impl<'a> /*trait*/ QPixmapCache_remove_s<()> for (&'a QString) {
  fn remove_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache6removeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPixmapCache6removeERK7QString(arg0)};
    // return 1;
  }
}

  // proto: static void QPixmapCache::setCacheLimit(int );
impl /*struct*/ QPixmapCache {
  pub fn setCacheLimit_s<RetType, T: QPixmapCache_setCacheLimit_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setCacheLimit_s();
    // return 1;
  }
}

pub trait QPixmapCache_setCacheLimit_s<RetType> {
  fn setCacheLimit_s(self ) -> RetType;
}

  // proto: static void QPixmapCache::setCacheLimit(int );
impl<'a> /*trait*/ QPixmapCache_setCacheLimit_s<()> for (i32) {
  fn setCacheLimit_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache13setCacheLimitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QPixmapCache13setCacheLimitEi(arg0)};
    // return 1;
  }
}

// <= body block end

