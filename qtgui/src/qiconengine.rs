

// mod ::gui::QIconEngine
// package qtgui
// /usr/include/qt/QtGui/qiconengine.h
// #include <qiconengine.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 66
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
#[derive(Default)] // class sizeof(QIconEngine)=8
pub struct QIconEngine {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QIconEngine_ITF interface {
//    QIconEngine_PTR() *QIconEngine
//}
//func (ptr *QIconEngine) QIconEngine_PTR() *QIconEngine { return ptr }

impl /*struct*/ QIconEngine {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QIconEngine {
    return QIconEngine{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QIconEngine {
//  type Target = QIconEngineBASE;
//
//  fn deref(&self) -> &QIconEngineBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QIconEngineBASE> for QIconEngine {
//  fn as_ref(& self) -> & QIconEngineBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qiconengine.h:53
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QIconEngine()

/*
Constructs the icon engine.

This function was introduced in  Qt 5.6.
*/
// QIconEngine() ctx.fn_proto_cpp
impl /*struct*/ QIconEngine {
  pub fn QIconEngine_0<T: QIconEngine_QIconEngine_0>(value: T) -> QIconEngine {
    let rsthis = value.QIconEngine_0();
    return rsthis;
    // return 1;
  }
}

pub trait QIconEngine_QIconEngine_0 {
  fn QIconEngine_0(self) -> QIconEngine;
}
// QIconEngine() ctx.fn_proto_cpp
impl<'a> /*trait*/ QIconEngine_QIconEngine_0 for () {
  fn QIconEngine_0(self) -> QIconEngine {
    // unsafe{_ZN11QIconEngineC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QIconEngineC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QIconEngine{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QIconEngine()

/*

*/
pub fn DeleteQIconEngine(this :*mut QIconEngine) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QIconEngineD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qiconengine.h:56
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void paint(QPainter *, const QRect &, QIcon::Mode, QIcon::State)

/*
Uses the given painter to paint the icon with the required mode and state into the rectangle rect.
*/
impl /*struct*/ QIconEngine {
  pub fn paint_0<RetType, T: QIconEngine_paint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paint_0(self);
    // return 1;
  }
}
pub trait QIconEngine_paint_0<RetType> {
  fn paint_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_paint_0<(/*void*/)> for (usize,usize,i32,i32) {
  fn paint_0(self , rsthis: & QIconEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QIconEngine5paintEP8QPainterRK5QRectN5QIcon4ModeENS5_5StateE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize actualSize(const QSize &, QIcon::Mode, QIcon::State)

/*
Returns the actual size of the icon the engine provides for the requested size, mode and state. The default implementation returns the given size.
*/
impl /*struct*/ QIconEngine {
  pub fn actualSize_0<RetType, T: QIconEngine_actualSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actualSize_0(self);
    // return 1;
  }
}
pub trait QIconEngine_actualSize_0<RetType> {
  fn actualSize_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_actualSize_0<usize> for (usize,i32,i32) {
  fn actualSize_0(self , rsthis: & QIconEngine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QIconEngine10actualSizeERK5QSizeN5QIcon4ModeENS3_5StateE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [32] QPixmap pixmap(const QSize &, QIcon::Mode, QIcon::State)

/*
Returns the icon as a pixmap with the required size, mode, and state. The default implementation creates a new pixmap and calls paint() to fill it.
*/
impl /*struct*/ QIconEngine {
  pub fn pixmap_0<RetType, T: QIconEngine_pixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixmap_0(self);
    // return 1;
  }
}
pub trait QIconEngine_pixmap_0<RetType> {
  fn pixmap_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_pixmap_0<usize> for (usize,i32,i32) {
  fn pixmap_0(self , rsthis: & QIconEngine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QIconEngine6pixmapERK5QSizeN5QIcon4ModeENS3_5StateE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void addPixmap(const QPixmap &, QIcon::Mode, QIcon::State)

/*
Called by QIcon::addPixmap(). Adds a specialized pixmap for the given mode and state. The default pixmap-based engine stores any supplied pixmaps, and it uses them instead of scaled pixmaps if the size of a pixmap matches the size of icon requested. Custom icon engines that implement scalable vector formats are free to ignores any extra pixmaps.
*/
impl /*struct*/ QIconEngine {
  pub fn addPixmap_0<RetType, T: QIconEngine_addPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addPixmap_0(self);
    // return 1;
  }
}
pub trait QIconEngine_addPixmap_0<RetType> {
  fn addPixmap_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_addPixmap_0<(/*void*/)> for (usize,i32,i32) {
  fn addPixmap_0(self , rsthis: & QIconEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QIconEngine9addPixmapERK7QPixmapN5QIcon4ModeENS3_5StateE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void addFile(const QString &, const QSize &, QIcon::Mode, QIcon::State)

/*
Called by QIcon::addFile(). Adds a specialized pixmap from the file with the given fileName, size, mode and state. The default pixmap-based engine stores any supplied file names, and it loads the pixmaps on demand instead of using scaled pixmaps if the size of a pixmap matches the size of icon requested. Custom icon engines that implement scalable vector formats are free to ignores any extra files.
*/
impl /*struct*/ QIconEngine {
  pub fn addFile_0<RetType, T: QIconEngine_addFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addFile_0(self);
    // return 1;
  }
}
pub trait QIconEngine_addFile_0<RetType> {
  fn addFile_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_addFile_0<(/*void*/)> for (usize,usize,i32,i32) {
  fn addFile_0(self , rsthis: & QIconEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QIconEngine7addFileERK7QStringRK5QSizeN5QIcon4ModeENS6_5StateE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString key() const

/*
Returns a key that identifies this icon engine.
*/
impl /*struct*/ QIconEngine {
  pub fn key_0<RetType, T: QIconEngine_key_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.key_0(self);
    // return 1;
  }
}
pub trait QIconEngine_key_0<RetType> {
  fn key_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_key_0<usize> for () {
  fn key_0(self , rsthis: & QIconEngine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QIconEngine3keyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:64
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QIconEngine * clone() const

/*
Reimplement this method to return a clone of this icon engine.
*/
impl /*struct*/ QIconEngine {
  pub fn clone_0<RetType, T: QIconEngine_clone_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clone_0(self);
    // return 1;
  }
}
pub trait QIconEngine_clone_0<RetType> {
  fn clone_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_clone_0<usize> for () {
  fn clone_0(self , rsthis: & QIconEngine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QIconEngine5cloneEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool read(QDataStream &)

/*
Reads icon engine contents from the QDataStream in. Returns true if the contents were read; otherwise returns false.

QIconEngine's default implementation always return false.
*/
impl /*struct*/ QIconEngine {
  pub fn read_0<RetType, T: QIconEngine_read_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.read_0(self);
    // return 1;
  }
}
pub trait QIconEngine_read_0<RetType> {
  fn read_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_read_0<bool> for (usize) {
  fn read_0(self , rsthis: & QIconEngine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QIconEngine4readER11QDataStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool write(QDataStream &) const

/*
Writes the contents of this engine to the QDataStream out. Returns true if the contents were written; otherwise returns false.

QIconEngine's default implementation always return false.
*/
impl /*struct*/ QIconEngine {
  pub fn write_0<RetType, T: QIconEngine_write_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_0(self);
    // return 1;
  }
}
pub trait QIconEngine_write_0<RetType> {
  fn write_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_write_0<bool> for (usize) {
  fn write_0(self , rsthis: & QIconEngine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QIconEngine5writeER11QDataStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:77
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QList<QSize> availableSizes(QIcon::Mode, QIcon::State) const

/*
Returns sizes of all images that are contained in the engine for the specific mode and state.

Note: This is a helper method and the actual work is done by the virtual_hook() method, hence this method depends on icon engine support and may not work with all icon engines.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QIconEngine {
  pub fn availableSizes_0<RetType, T: QIconEngine_availableSizes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.availableSizes_0(self);
    // return 1;
  }
}
pub trait QIconEngine_availableSizes_0<RetType> {
  fn availableSizes_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_availableSizes_0<usize> for (i32,i32) {
  fn availableSizes_0(self , rsthis: & QIconEngine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QIconEngine14availableSizesEN5QIcon4ModeENS0_5StateE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString iconName() const

/*
Returns the name used to create the engine, if available.

Note: This is a helper method and the actual work is done by the virtual_hook() method, hence this method depends on icon engine support and may not work with all icon engines.

This function was introduced in  Qt 4.7.
*/
impl /*struct*/ QIconEngine {
  pub fn iconName_0<RetType, T: QIconEngine_iconName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconName_0(self);
    // return 1;
  }
}
pub trait QIconEngine_iconName_0<RetType> {
  fn iconName_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_iconName_0<usize> for () {
  fn iconName_0(self , rsthis: & QIconEngine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QIconEngine8iconNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:81
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if this icon engine represent a null QIcon.

Note: This is a helper method and the actual work is done by the virtual_hook() method, hence this method depends on icon engine support and may not work with all icon engines.

This function was introduced in  Qt 5.7.
*/
impl /*struct*/ QIconEngine {
  pub fn isNull_0<RetType, T: QIconEngine_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QIconEngine_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QIconEngine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QIconEngine6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:82
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap scaledPixmap(const QSize &, QIcon::Mode, QIcon::State, qreal)

/*
Returns a pixmap for the given size, mode, state and scale.

The scale argument is typically equal to the device pixel ratio of the display.

Note: This is a helper method and the actual work is done by the virtual_hook() method, hence this method depends on icon engine support and may not work with all icon engines.

Note: Some engines may cast scale to an integer.

This function was introduced in  Qt 5.9.

See also ScaledPixmapArgument.
*/
impl /*struct*/ QIconEngine {
  pub fn scaledPixmap_0<RetType, T: QIconEngine_scaledPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaledPixmap_0(self);
    // return 1;
  }
}
pub trait QIconEngine_scaledPixmap_0<RetType> {
  fn scaledPixmap_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_scaledPixmap_0<usize> for (usize,i32,i32,f64) {
  fn scaledPixmap_0(self , rsthis: & QIconEngine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QIconEngine12scaledPixmapERK5QSizeN5QIcon4ModeENS3_5StateEd", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qiconengine.h:93
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void virtual_hook(int, void *)

/*
Additional method to allow extending QIconEngine without adding new virtual methods (and without breaking binary compatibility). The actual action and format of data depends on id argument which is in fact a constant from IconEngineHook enum.

This function was introduced in  Qt 4.5.

See also IconEngineHook.
*/
impl /*struct*/ QIconEngine {
  pub fn virtual_hook_0<RetType, T: QIconEngine_virtual_hook_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.virtual_hook_0(self);
    // return 1;
  }
}
pub trait QIconEngine_virtual_hook_0<RetType> {
  fn virtual_hook_0(self , rsthis: & QIconEngine) -> RetType;
}
impl<'a> /*trait*/ QIconEngine_virtual_hook_0<(/*void*/)> for (i32,usize) {
  fn virtual_hook_0(self , rsthis: & QIconEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QIconEngine12virtual_hookEiPv", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
These enum values are used for virtual_hook() to allow additional queries to icon engine without breaking binary compatibility.



This enum was introduced or modified in  Qt 4.5.

See also virtual_hook().

*/
pub type QIconEngine__IconEngineHook = i32;
// Allows to query the sizes of the contained pixmaps for pixmap-based engines. The data argument of the virtual_hook() function is a AvailableSizesArgument pointer that should be filled with icon sizes. Engines that work in terms of a scalable, vectorial format normally return an empty list.
pub const QIconEngine__AvailableSizesHook :QIconEngine__IconEngineHook = 1;
// Allows to query the name used to create the icon, for example when instantiating an icon using QIcon::fromTheme().
pub const QIconEngine__IconNameHook :QIconEngine__IconEngineHook = 2;
// 
pub const QIconEngine__IsNullHook :QIconEngine__IconEngineHook = 3;
// 
pub const QIconEngine__ScaledPixmapHook :QIconEngine__IconEngineHook = 4;
pub fn QIconEngine_IconEngineHookItemName(val: i32) ->String {
  match val {
     QIconEngine__AvailableSizesHook => // 1
     {return String::from("AvailableSizesHook");}
     QIconEngine__IconNameHook => // 2
     {return String::from("IconNameHook");}
     QIconEngine__IsNullHook => // 3
     {return String::from("IsNullHook");}
     QIconEngine__ScaledPixmapHook => // 4
     {return String::from("ScaledPixmapHook");}
  _ => {return format!("{}", val);}
}
}
pub fn QIconEngine_IconEngineHookItemName_s(val: i32) ->String {
  //var nilthis *QIconEngine
  //return nilthis.IconEngineHookItemName(val);
  return QIconEngine_IconEngineHookItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
