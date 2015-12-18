// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qpixmap::QPixmap;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static int QPixmapCache::cacheLimit();
  fn _ZN12QPixmapCache10cacheLimitEv() -> c_int;
  // proto: static void QPixmapCache::clear();
  fn _ZN12QPixmapCache5clearEv() ;
  // proto: static bool QPixmapCache::insert(const QString & key, const QPixmap & pixmap);
  fn _ZN12QPixmapCache6insertERK7QStringRK7QPixmap(arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto: static void QPixmapCache::remove(const QString & key);
  fn _ZN12QPixmapCache6removeERK7QString(arg0: *mut c_void) ;
  // proto: static void QPixmapCache::setCacheLimit(int );
  fn _ZN12QPixmapCache13setCacheLimitEi(arg0: c_int) ;
}

// body block begin
// class sizeof(QPixmapCache)=1
pub struct QPixmapCache {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPixmapCache {
  pub fn cacheLimit<RetType, T: QPixmapCache_cacheLimit<RetType>>(&mut self, value: T) -> RetType {
    return value.cacheLimit(self);
    // return 1;
  }
}

pub trait QPixmapCache_cacheLimit<RetType> {
  fn cacheLimit(self, rsthis: &mut QPixmapCache) -> RetType;
}

// proto: static int QPixmapCache::cacheLimit();
impl<'a> /*trait*/ QPixmapCache_cacheLimit<i32> for () {
  fn cacheLimit(self, rsthis: &mut QPixmapCache) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache10cacheLimitEv()};
    let mut ret = unsafe {_ZN12QPixmapCache10cacheLimitEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPixmapCache {
  pub fn clear<RetType, T: QPixmapCache_clear<RetType>>(&mut self, value: T) -> RetType {
    return value.clear(self);
    // return 1;
  }
}

pub trait QPixmapCache_clear<RetType> {
  fn clear(self, rsthis: &mut QPixmapCache) -> RetType;
}

// proto: static void QPixmapCache::clear();
impl<'a> /*trait*/ QPixmapCache_clear<()> for () {
  fn clear(self, rsthis: &mut QPixmapCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache5clearEv()};
     unsafe {_ZN12QPixmapCache5clearEv()};
    // return 1;
  }
}

impl /*struct*/ QPixmapCache {
  pub fn insert<RetType, T: QPixmapCache_insert<RetType>>(&mut self, value: T) -> RetType {
    return value.insert(self);
    // return 1;
  }
}

pub trait QPixmapCache_insert<RetType> {
  fn insert(self, rsthis: &mut QPixmapCache) -> RetType;
}

// proto: static bool QPixmapCache::insert(const QString & key, const QPixmap & pixmap);
impl<'a> /*trait*/ QPixmapCache_insert<i8> for (&'a  QString, &'a  QPixmap) {
  fn insert(self, rsthis: &mut QPixmapCache) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache6insertERK7QStringRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QPixmapCache6insertERK7QStringRK7QPixmap(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPixmapCache {
  pub fn remove<RetType, T: QPixmapCache_remove<RetType>>(&mut self, value: T) -> RetType {
    return value.remove(self);
    // return 1;
  }
}

pub trait QPixmapCache_remove<RetType> {
  fn remove(self, rsthis: &mut QPixmapCache) -> RetType;
}

// proto: static void QPixmapCache::remove(const QString & key);
impl<'a> /*trait*/ QPixmapCache_remove<()> for (&'a  QString) {
  fn remove(self, rsthis: &mut QPixmapCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache6removeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPixmapCache6removeERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QPixmapCache {
  pub fn setCacheLimit<RetType, T: QPixmapCache_setCacheLimit<RetType>>(&mut self, value: T) -> RetType {
    return value.setCacheLimit(self);
    // return 1;
  }
}

pub trait QPixmapCache_setCacheLimit<RetType> {
  fn setCacheLimit(self, rsthis: &mut QPixmapCache) -> RetType;
}

// proto: static void QPixmapCache::setCacheLimit(int );
impl<'a> /*trait*/ QPixmapCache_setCacheLimit<()> for (i32) {
  fn setCacheLimit(self, rsthis: &mut QPixmapCache) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache13setCacheLimitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QPixmapCache13setCacheLimitEi(arg0)};
    // return 1;
  }
}

