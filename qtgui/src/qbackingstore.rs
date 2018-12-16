

// mod ::gui::QBackingStore
// package qtgui
// /usr/include/qt/QtGui/qbackingstore.h
// #include <qbackingstore.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 146
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
#[derive(Default)] // class sizeof(QBackingStore)=8
pub struct QBackingStore {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QBackingStore_ITF interface {
//    QBackingStore_PTR() *QBackingStore
//}
//func (ptr *QBackingStore) QBackingStore_PTR() *QBackingStore { return ptr }

impl /*struct*/ QBackingStore {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QBackingStore {
    return QBackingStore{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QBackingStore {
//  type Target = QBackingStoreBASE;
//
//  fn deref(&self) -> &QBackingStoreBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QBackingStoreBASE> for QBackingStore {
//  fn as_ref(& self) -> & QBackingStoreBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qbackingstore.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QBackingStore(QWindow *)

/*
Constructs an empty surface for the given top-level window.
*/
// QBackingStore(QWindow *) ctx.fn_proto_cpp
impl /*struct*/ QBackingStore {
  pub fn QBackingStore_0<T: QBackingStore_QBackingStore_0>(value: T) -> QBackingStore {
    let rsthis = value.QBackingStore_0();
    return rsthis;
    // return 1;
  }
}

pub trait QBackingStore_QBackingStore_0 {
  fn QBackingStore_0(self) -> QBackingStore;
}
// QBackingStore(QWindow *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBackingStore_QBackingStore_0 for (usize) {
  fn QBackingStore_0(self) -> QBackingStore {
    // unsafe{_ZN13QBackingStoreC2EP7QWindow()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QBackingStoreC2EP7QWindow", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBackingStore{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbackingstore.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QBackingStore()

/*

*/
pub fn DeleteQBackingStore(this :*mut QBackingStore) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QBackingStoreD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qbackingstore.h:65
// index:0
// Public Visibility=Default Availability=Available
// [8] QWindow * window() const

/*
Returns a pointer to the top-level window associated with this surface.
*/
impl /*struct*/ QBackingStore {
  pub fn window_0<RetType, T: QBackingStore_window_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.window_0(self);
    // return 1;
  }
}
pub trait QBackingStore_window_0<RetType> {
  fn window_0(self , rsthis: & QBackingStore) -> RetType;
}
impl<'a> /*trait*/ QBackingStore_window_0<usize> for () {
  fn window_0(self , rsthis: & QBackingStore) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QBackingStore6windowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbackingstore.h:67
// index:0
// Public Visibility=Default Availability=Available
// [8] QPaintDevice * paintDevice()

/*
Returns the paint device for this surface.

Warning: The device is only valid between calls to beginPaint() and endPaint(). You should not cache the returned value.
*/
impl /*struct*/ QBackingStore {
  pub fn paintDevice_0<RetType, T: QBackingStore_paintDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintDevice_0(self);
    // return 1;
  }
}
pub trait QBackingStore_paintDevice_0<RetType> {
  fn paintDevice_0(self , rsthis: & QBackingStore) -> RetType;
}
impl<'a> /*trait*/ QBackingStore_paintDevice_0<usize> for () {
  fn paintDevice_0(self , rsthis: & QBackingStore) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QBackingStore11paintDeviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbackingstore.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void flush(const QRegion &, QWindow *, const QPoint &)

/*
Flushes the given region from the specified window onto the screen.

The window must either be the top level window represented by this backingstore, or a non-transient child of that window. Passing nullptr falls back to using the backingstore's top level window.

If the window is a child window, the region should be in child window coordinates, and the offset should be the child window's offset in relation to the backingstore's top level window.

You should call this function after ending painting with endPaint().

See also QWindow::transientParent().
*/
impl /*struct*/ QBackingStore {
  pub fn flush_0<RetType, T: QBackingStore_flush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flush_0(self);
    // return 1;
  }
}
pub trait QBackingStore_flush_0<RetType> {
  fn flush_0(self , rsthis: & QBackingStore) -> RetType;
}
impl<'a> /*trait*/ QBackingStore_flush_0<(/*void*/)> for (usize,usize,usize) {
  fn flush_0(self , rsthis: & QBackingStore) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QBackingStore5flushERK7QRegionP7QWindowRK6QPoint", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbackingstore.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resize(const QSize &)

/*
Sets the size of the window surface to size.

See also size().
*/
impl /*struct*/ QBackingStore {
  pub fn resize_0<RetType, T: QBackingStore_resize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_0(self);
    // return 1;
  }
}
pub trait QBackingStore_resize_0<RetType> {
  fn resize_0(self , rsthis: & QBackingStore) -> RetType;
}
impl<'a> /*trait*/ QBackingStore_resize_0<(/*void*/)> for (usize) {
  fn resize_0(self , rsthis: & QBackingStore) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QBackingStore6resizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbackingstore.h:72
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize size() const

/*
Returns the current size of the window surface.
*/
impl /*struct*/ QBackingStore {
  pub fn size_0<RetType, T: QBackingStore_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QBackingStore_size_0<RetType> {
  fn size_0(self , rsthis: & QBackingStore) -> RetType;
}
impl<'a> /*trait*/ QBackingStore_size_0<usize> for () {
  fn size_0(self , rsthis: & QBackingStore) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QBackingStore4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbackingstore.h:74
// index:0
// Public Visibility=Default Availability=Available
// [1] bool scroll(const QRegion &, int, int)

/*
Scrolls the given area dx pixels to the right and dy downward; both dx and dy may be negative.

Returns true if the area was scrolled successfully; false otherwise.
*/
impl /*struct*/ QBackingStore {
  pub fn scroll_0<RetType, T: QBackingStore_scroll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scroll_0(self);
    // return 1;
  }
}
pub trait QBackingStore_scroll_0<RetType> {
  fn scroll_0(self , rsthis: & QBackingStore) -> RetType;
}
impl<'a> /*trait*/ QBackingStore_scroll_0<bool> for (usize,i32,i32) {
  fn scroll_0(self , rsthis: & QBackingStore) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QBackingStore6scrollERK7QRegionii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbackingstore.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void beginPaint(const QRegion &)

/*
Begins painting on the backing store surface in the given region.

You should call this function before using the paintDevice() to paint.

See also endPaint() and paintDevice().
*/
impl /*struct*/ QBackingStore {
  pub fn beginPaint_0<RetType, T: QBackingStore_beginPaint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginPaint_0(self);
    // return 1;
  }
}
pub trait QBackingStore_beginPaint_0<RetType> {
  fn beginPaint_0(self , rsthis: & QBackingStore) -> RetType;
}
impl<'a> /*trait*/ QBackingStore_beginPaint_0<(/*void*/)> for (usize) {
  fn beginPaint_0(self , rsthis: & QBackingStore) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QBackingStore10beginPaintERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbackingstore.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void endPaint()

/*
Ends painting.

You should call this function after painting with the paintDevice() has ended.

See also beginPaint() and paintDevice().
*/
impl /*struct*/ QBackingStore {
  pub fn endPaint_0<RetType, T: QBackingStore_endPaint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endPaint_0(self);
    // return 1;
  }
}
pub trait QBackingStore_endPaint_0<RetType> {
  fn endPaint_0(self , rsthis: & QBackingStore) -> RetType;
}
impl<'a> /*trait*/ QBackingStore_endPaint_0<(/*void*/)> for () {
  fn endPaint_0(self , rsthis: & QBackingStore) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QBackingStore8endPaintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbackingstore.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStaticContents(const QRegion &)

/*
Set region as the static contents of this window.

See also staticContents().
*/
impl /*struct*/ QBackingStore {
  pub fn setStaticContents_0<RetType, T: QBackingStore_setStaticContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStaticContents_0(self);
    // return 1;
  }
}
pub trait QBackingStore_setStaticContents_0<RetType> {
  fn setStaticContents_0(self , rsthis: & QBackingStore) -> RetType;
}
impl<'a> /*trait*/ QBackingStore_setStaticContents_0<(/*void*/)> for (usize) {
  fn setStaticContents_0(self , rsthis: & QBackingStore) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QBackingStore17setStaticContentsERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbackingstore.h:80
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion staticContents() const

/*
Returns a QRegion representing the area of the window that has static contents.

See also setStaticContents().
*/
impl /*struct*/ QBackingStore {
  pub fn staticContents_0<RetType, T: QBackingStore_staticContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.staticContents_0(self);
    // return 1;
  }
}
pub trait QBackingStore_staticContents_0<RetType> {
  fn staticContents_0(self , rsthis: & QBackingStore) -> RetType;
}
impl<'a> /*trait*/ QBackingStore_staticContents_0<usize> for () {
  fn staticContents_0(self , rsthis: & QBackingStore) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QBackingStore14staticContentsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbackingstore.h:81
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasStaticContents() const

/*
Returns a boolean indicating if this window has static contents or not.
*/
impl /*struct*/ QBackingStore {
  pub fn hasStaticContents_0<RetType, T: QBackingStore_hasStaticContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasStaticContents_0(self);
    // return 1;
  }
}
pub trait QBackingStore_hasStaticContents_0<RetType> {
  fn hasStaticContents_0(self , rsthis: & QBackingStore) -> RetType;
}
impl<'a> /*trait*/ QBackingStore_hasStaticContents_0<bool> for () {
  fn hasStaticContents_0(self , rsthis: & QBackingStore) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QBackingStore17hasStaticContentsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
