

// mod ::gui::QPixmapCache
// package qtgui
// /usr/include/qt/QtGui/qpixmapcache.h
// #include <qpixmapcache.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 6
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QPixmapCache)=1
pub struct QPixmapCache {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPixmapCache_ITF interface {
//    QPixmapCache_PTR() *QPixmapCache
//}
//func (ptr *QPixmapCache) QPixmapCache_PTR() *QPixmapCache { return ptr }

impl /*struct*/ QPixmapCache {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPixmapCache {
    return QPixmapCache{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPixmapCache {
//  type Target = QPixmapCacheBASE;
//
//  fn deref(&self) -> &QPixmapCacheBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPixmapCacheBASE> for QPixmapCache {
//  fn as_ref(& self) -> & QPixmapCacheBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpixmapcache.h:77
// index:0
// Public static Visibility=Default Availability=Available
// [4] int cacheLimit()

/*
Returns the cache limit (in kilobytes).

The default cache limit is 10240 KB.

See also setCacheLimit().
*/
impl /*struct*/ QPixmapCache {
  pub fn cacheLimit_0<RetType, T: QPixmapCache_cacheLimit_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.cacheLimit_0();
    // return 1;
  }
}
pub trait QPixmapCache_cacheLimit_0<RetType> {
  fn cacheLimit_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmapCache_cacheLimit_0<i32> for () {
  fn cacheLimit_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPixmapCache10cacheLimitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmapcache.h:78
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setCacheLimit(int)

/*
Sets the cache limit to n kilobytes.

The default setting is 10240 KB.

See also cacheLimit().
*/
impl /*struct*/ QPixmapCache {
  pub fn setCacheLimit_0<RetType, T: QPixmapCache_setCacheLimit_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setCacheLimit_0();
    // return 1;
  }
}
pub trait QPixmapCache_setCacheLimit_0<RetType> {
  fn setCacheLimit_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmapCache_setCacheLimit_0<(/*void*/)> for (i32) {
  fn setCacheLimit_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPixmapCache13setCacheLimitEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmapcache.h:79
// index:0
// Public static Visibility=Default Availability=Available
// [8] QPixmap * find(const QString &)

/*
Looks for a cached pixmap associated with the given key in the cache. If the pixmap is found, the function sets pixmap to that pixmap and returns true; otherwise it leaves pixmap alone and returns false.

Example:


  QPixmap pm;
  if (!QPixmapCache::find("my_big_image", &pm)) {
      pm.load("bigimage.png");
      QPixmapCache::insert("my_big_image", pm);
  }
  painter->drawPixmap(0, 0, pm);



This function was introduced in  Qt 4.6.
*/
impl /*struct*/ QPixmapCache {
  pub fn find_0<RetType, T: QPixmapCache_find_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.find_0();
    // return 1;
  }
}
pub trait QPixmapCache_find_0<RetType> {
  fn find_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmapCache_find_0<usize> for (usize) {
  fn find_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPixmapCache4findERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmapcache.h:80
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool find(const QString &, QPixmap &)

/*
Looks for a cached pixmap associated with the given key in the cache. If the pixmap is found, the function sets pixmap to that pixmap and returns true; otherwise it leaves pixmap alone and returns false.

Example:


  QPixmap pm;
  if (!QPixmapCache::find("my_big_image", &pm)) {
      pm.load("bigimage.png");
      QPixmapCache::insert("my_big_image", pm);
  }
  painter->drawPixmap(0, 0, pm);



This function was introduced in  Qt 4.6.
*/
impl /*struct*/ QPixmapCache {
  pub fn find_1<RetType, T: QPixmapCache_find_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.find_1();
    // return 1;
  }
}
pub trait QPixmapCache_find_1<RetType> {
  fn find_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmapCache_find_1<bool> for (usize,usize) {
  fn find_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPixmapCache4findERK7QStringR7QPixmap", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmapcache.h:81
// index:2
// Public static Visibility=Default Availability=Available
// [1] bool find(const QString &, QPixmap *)

/*
Looks for a cached pixmap associated with the given key in the cache. If the pixmap is found, the function sets pixmap to that pixmap and returns true; otherwise it leaves pixmap alone and returns false.

Example:


  QPixmap pm;
  if (!QPixmapCache::find("my_big_image", &pm)) {
      pm.load("bigimage.png");
      QPixmapCache::insert("my_big_image", pm);
  }
  painter->drawPixmap(0, 0, pm);



This function was introduced in  Qt 4.6.
*/
impl /*struct*/ QPixmapCache {
  pub fn find_2<RetType, T: QPixmapCache_find_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.find_2();
    // return 1;
  }
}
pub trait QPixmapCache_find_2<RetType> {
  fn find_2(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmapCache_find_2<bool> for (usize,usize) {
  fn find_2(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPixmapCache4findERK7QStringP7QPixmap", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixmapcache.h:86
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void remove(const QString &)

/*
Removes the pixmap associated with key from the cache.
*/
impl /*struct*/ QPixmapCache {
  pub fn remove_0<RetType, T: QPixmapCache_remove_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.remove_0();
    // return 1;
  }
}
pub trait QPixmapCache_remove_0<RetType> {
  fn remove_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmapCache_remove_0<(/*void*/)> for (usize) {
  fn remove_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPixmapCache6removeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixmapcache.h:88
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void clear()

/*
Removes all pixmaps from the cache.
*/
impl /*struct*/ QPixmapCache {
  pub fn clear_0<RetType, T: QPixmapCache_clear_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.clear_0();
    // return 1;
  }
}
pub trait QPixmapCache_clear_0<RetType> {
  fn clear_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPixmapCache_clear_0<(/*void*/)> for () {
  fn clear_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QPixmapCache5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQPixmapCache(this :*mut QPixmapCache) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN12QPixmapCacheD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
