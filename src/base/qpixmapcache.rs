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
  // proto: int QPixmapCache::cacheLimit();
  fn _ZN12QPixmapCache10cacheLimitEv() -> i32;
  // proto: void QPixmapCache::clear();
  fn _ZN12QPixmapCache5clearEv() -> i32;
  // proto: bool QPixmapCache::insert(const QString & key, const QPixmap & pixmap);
  fn _ZN12QPixmapCache6insertERK7QStringRK7QPixmap(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: bool QPixmapCache::find(const QString & key, QPixmap & pixmap);
  fn _ZN12QPixmapCache4findERK7QStringR7QPixmap(arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: QPixmap * QPixmapCache::find(const QString & key);
  fn _ZN12QPixmapCache4findERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QPixmapCache::find(const QString & key, QPixmap * pixmap);
  fn _ZN12QPixmapCache4findERK7QStringP7QPixmap(arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QPixmapCache::remove(const QString & key);
  fn _ZN12QPixmapCache6removeERK7QString(arg0: *const c_void) -> i32;
  // proto: void QPixmapCache::setCacheLimit(int );
  fn _ZN12QPixmapCache13setCacheLimitEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QPixmapCache)=1
pub struct QPixmapCache {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPixmapCache {
  pub fn cacheLimit<T: QPixmapCache_cacheLimit>(&mut self, value: T) -> i32 {
    value.cacheLimit(self);
    return 1;
  }
}

pub trait QPixmapCache_cacheLimit {
  fn cacheLimit(self, this: &mut QPixmapCache) -> i32;
}

// proto: int QPixmapCache::cacheLimit();
impl<'a> /*trait*/ QPixmapCache_cacheLimit for () {
  fn cacheLimit(self, this: &mut QPixmapCache) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache10cacheLimitEv()};
    unsafe {_ZN12QPixmapCache10cacheLimitEv()};
    return 1;
  }
}

impl /*struct*/ QPixmapCache {
  pub fn clear<T: QPixmapCache_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QPixmapCache_clear {
  fn clear(self, this: &mut QPixmapCache) -> i32;
}

// proto: void QPixmapCache::clear();
impl<'a> /*trait*/ QPixmapCache_clear for () {
  fn clear(self, this: &mut QPixmapCache) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache5clearEv()};
    unsafe {_ZN12QPixmapCache5clearEv()};
    return 1;
  }
}

impl /*struct*/ QPixmapCache {
  pub fn insert<T: QPixmapCache_insert>(&mut self, value: T) -> i32 {
    value.insert(self);
    return 1;
  }
}

pub trait QPixmapCache_insert {
  fn insert(self, this: &mut QPixmapCache) -> i32;
}

// proto: bool QPixmapCache::insert(const QString & key, const QPixmap & pixmap);
impl<'a> /*trait*/ QPixmapCache_insert for (&'a  QString, &'a  QPixmap) {
  fn insert(self, this: &mut QPixmapCache) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache6insertERK7QStringRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN12QPixmapCache6insertERK7QStringRK7QPixmap(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPixmapCache {
  pub fn find<T: QPixmapCache_find>(&mut self, value: T) -> i32 {
    value.find(self);
    return 1;
  }
}

pub trait QPixmapCache_find {
  fn find(self, this: &mut QPixmapCache) -> i32;
}

// proto: bool QPixmapCache::find(const QString & key, QPixmap & pixmap);
impl<'a> /*trait*/ QPixmapCache_find for (&'a  QString, &'a mut QPixmap) {
  fn find(self, this: &mut QPixmapCache) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache4findERK7QStringR7QPixmap()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QPixmapCache4findERK7QStringR7QPixmap(arg0, arg1)};
    return 1;
  }
}

// proto: QPixmap * QPixmapCache::find(const QString & key);
impl<'a> /*trait*/ QPixmapCache_find for (&'a  QString) {
  fn find(self, this: &mut QPixmapCache) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache4findERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QPixmapCache4findERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QPixmapCache {
  pub fn remove<T: QPixmapCache_remove>(&mut self, value: T) -> i32 {
    value.remove(self);
    return 1;
  }
}

pub trait QPixmapCache_remove {
  fn remove(self, this: &mut QPixmapCache) -> i32;
}

// proto: void QPixmapCache::remove(const QString & key);
impl<'a> /*trait*/ QPixmapCache_remove for (&'a  QString) {
  fn remove(self, this: &mut QPixmapCache) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache6removeERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QPixmapCache6removeERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QPixmapCache {
  pub fn setCacheLimit<T: QPixmapCache_setCacheLimit>(&mut self, value: T) -> i32 {
    value.setCacheLimit(self);
    return 1;
  }
}

pub trait QPixmapCache_setCacheLimit {
  fn setCacheLimit(self, this: &mut QPixmapCache) -> i32;
}

// proto: void QPixmapCache::setCacheLimit(int );
impl<'a> /*trait*/ QPixmapCache_setCacheLimit for (i32) {
  fn setCacheLimit(self, this: &mut QPixmapCache) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixmapCache13setCacheLimitEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QPixmapCache13setCacheLimitEi(arg0)};
    return 1;
  }
}

