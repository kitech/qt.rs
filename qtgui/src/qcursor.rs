

// mod ::gui::QCursor
// package qtgui
// /usr/include/qt/QtGui/qcursor.h
// #include <qcursor.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 33
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
#[derive(Default)] // class sizeof(QCursor)=8
pub struct QCursor {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCursor_ITF interface {
//    QCursor_PTR() *QCursor
//}
//func (ptr *QCursor) QCursor_PTR() *QCursor { return ptr }

impl /*struct*/ QCursor {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCursor {
    return QCursor{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCursor {
//  type Target = QCursorBASE;
//
//  fn deref(&self) -> &QCursorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCursorBASE> for QCursor {
//  fn as_ref(& self) -> & QCursorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qcursor.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QCursor()

/*
Constructs a cursor with the default arrow shape.
*/
// QCursor() ctx.fn_proto_cpp
impl /*struct*/ QCursor {
  pub fn QCursor_0<T: QCursor_QCursor_0>(value: T) -> QCursor {
    let rsthis = value.QCursor_0();
    return rsthis;
    // return 1;
  }
}

pub trait QCursor_QCursor_0 {
  fn QCursor_0(self) -> QCursor;
}
// QCursor() ctx.fn_proto_cpp
impl<'a> /*trait*/ QCursor_QCursor_0 for () {
  fn QCursor_0(self) -> QCursor {
    // unsafe{_ZN7QCursorC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QCursorC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCursor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcursor.h:83
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QCursor(Qt::CursorShape)

/*
Constructs a cursor with the default arrow shape.
*/
// QCursor(Qt::CursorShape) ctx.fn_proto_cpp
impl /*struct*/ QCursor {
  pub fn QCursor_1<T: QCursor_QCursor_1>(value: T) -> QCursor {
    let rsthis = value.QCursor_1();
    return rsthis;
    // return 1;
  }
}

pub trait QCursor_QCursor_1 {
  fn QCursor_1(self) -> QCursor;
}
// QCursor(Qt::CursorShape) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCursor_QCursor_1 for (i32) {
  fn QCursor_1(self) -> QCursor {
    // unsafe{_ZN7QCursorC2EN2Qt11CursorShapeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QCursorC2EN2Qt11CursorShapeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCursor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcursor.h:84
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QCursor(const QBitmap &, const QBitmap &, int, int)

/*
Constructs a cursor with the default arrow shape.
*/
// QCursor(const QBitmap &, const QBitmap &, int, int) ctx.fn_proto_cpp
impl /*struct*/ QCursor {
  pub fn QCursor_2<T: QCursor_QCursor_2>(value: T) -> QCursor {
    let rsthis = value.QCursor_2();
    return rsthis;
    // return 1;
  }
}

pub trait QCursor_QCursor_2 {
  fn QCursor_2(self) -> QCursor;
}
// QCursor(const QBitmap &, const QBitmap &, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCursor_QCursor_2 for (usize,usize,i32,i32) {
  fn QCursor_2(self) -> QCursor {
    // unsafe{_ZN7QCursorC2ERK7QBitmapS2_ii()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QCursorC2ERK7QBitmapS2_ii", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCursor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcursor.h:85
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QCursor(const QPixmap &, int, int)

/*
Constructs a cursor with the default arrow shape.
*/
// QCursor(const QPixmap &, int, int) ctx.fn_proto_cpp
impl /*struct*/ QCursor {
  pub fn QCursor_3<T: QCursor_QCursor_3>(value: T) -> QCursor {
    let rsthis = value.QCursor_3();
    return rsthis;
    // return 1;
  }
}

pub trait QCursor_QCursor_3 {
  fn QCursor_3(self) -> QCursor;
}
// QCursor(const QPixmap &, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCursor_QCursor_3 for (usize,i32,i32) {
  fn QCursor_3(self) -> QCursor {
    // unsafe{_ZN7QCursorC2ERK7QPixmapii()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QCursorC2ERK7QPixmapii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCursor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcursor.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QCursor()

/*

*/
pub fn DeleteQCursor(this :*mut QCursor) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QCursorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qcursor.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] QCursor & operator=(const QCursor &)

/*

*/
impl /*struct*/ QCursor {
  pub fn operator_equal_0<RetType, T: QCursor_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QCursor_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QCursor) -> RetType;
}
impl<'a> /*trait*/ QCursor_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QCursoraSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcursor.h:91
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QCursor & operator=(QCursor &&)

/*

*/
impl /*struct*/ QCursor {
  pub fn operator_equal_1<RetType, T: QCursor_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QCursor_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QCursor) -> RetType;
}
impl<'a> /*trait*/ QCursor_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QCursoraSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcursor.h:95
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QCursor &)

/*

*/
impl /*struct*/ QCursor {
  pub fn swap_0<RetType, T: QCursor_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QCursor_swap_0<RetType> {
  fn swap_0(self , rsthis: & QCursor) -> RetType;
}
impl<'a> /*trait*/ QCursor_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QCursor4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcursor.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::CursorShape shape() const

/*
Returns the cursor shape identifier. The return value is one of the Qt::CursorShape enum values (cast to an int).

See also setShape().
*/
impl /*struct*/ QCursor {
  pub fn shape_0<RetType, T: QCursor_shape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shape_0(self);
    // return 1;
  }
}
pub trait QCursor_shape_0<RetType> {
  fn shape_0(self , rsthis: & QCursor) -> RetType;
}
impl<'a> /*trait*/ QCursor_shape_0<i32> for () {
  fn shape_0(self , rsthis: & QCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QCursor5shapeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcursor.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setShape(Qt::CursorShape)

/*
Sets the cursor to the shape identified by shape.

See Qt::CursorShape for the list of cursor shapes.

See also shape().
*/
impl /*struct*/ QCursor {
  pub fn setShape_0<RetType, T: QCursor_setShape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setShape_0(self);
    // return 1;
  }
}
pub trait QCursor_setShape_0<RetType> {
  fn setShape_0(self , rsthis: & QCursor) -> RetType;
}
impl<'a> /*trait*/ QCursor_setShape_0<(/*void*/)> for (i32) {
  fn setShape_0(self , rsthis: & QCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QCursor8setShapeEN2Qt11CursorShapeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcursor.h:102
// index:0
// Public Visibility=Default Availability=Available
// [8] const QBitmap * bitmap() const

/*
Returns the cursor bitmap, or 0 if it is one of the standard cursors.
*/
impl /*struct*/ QCursor {
  pub fn bitmap_0<RetType, T: QCursor_bitmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bitmap_0(self);
    // return 1;
  }
}
pub trait QCursor_bitmap_0<RetType> {
  fn bitmap_0(self , rsthis: & QCursor) -> RetType;
}
impl<'a> /*trait*/ QCursor_bitmap_0<usize> for () {
  fn bitmap_0(self , rsthis: & QCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QCursor6bitmapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcursor.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] const QBitmap * mask() const

/*
Returns the cursor bitmap mask, or 0 if it is one of the standard cursors.
*/
impl /*struct*/ QCursor {
  pub fn mask_0<RetType, T: QCursor_mask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mask_0(self);
    // return 1;
  }
}
pub trait QCursor_mask_0<RetType> {
  fn mask_0(self , rsthis: & QCursor) -> RetType;
}
impl<'a> /*trait*/ QCursor_mask_0<usize> for () {
  fn mask_0(self , rsthis: & QCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QCursor4maskEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcursor.h:104
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap pixmap() const

/*
Returns the cursor pixmap. This is only valid if the cursor is a pixmap cursor.
*/
impl /*struct*/ QCursor {
  pub fn pixmap_0<RetType, T: QCursor_pixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixmap_0(self);
    // return 1;
  }
}
pub trait QCursor_pixmap_0<RetType> {
  fn pixmap_0(self , rsthis: & QCursor) -> RetType;
}
impl<'a> /*trait*/ QCursor_pixmap_0<usize> for () {
  fn pixmap_0(self , rsthis: & QCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QCursor6pixmapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcursor.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint hotSpot() const

/*
Returns the cursor hot spot, or (0, 0) if it is one of the standard cursors.
*/
impl /*struct*/ QCursor {
  pub fn hotSpot_0<RetType, T: QCursor_hotSpot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hotSpot_0(self);
    // return 1;
  }
}
pub trait QCursor_hotSpot_0<RetType> {
  fn hotSpot_0(self , rsthis: & QCursor) -> RetType;
}
impl<'a> /*trait*/ QCursor_hotSpot_0<usize> for () {
  fn hotSpot_0(self , rsthis: & QCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QCursor7hotSpotEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcursor.h:107
// index:0
// Public static Visibility=Default Availability=Available
// [8] QPoint pos()

/*
Returns the position of the cursor (hot spot) of the primary screen in global screen coordinates.

You can call QWidget::mapFromGlobal() to translate it to widget coordinates.

Note: The position is queried from the windowing system. If mouse events are generated via other means (e.g., via QWindowSystemInterface in a unit test), those fake mouse moves will not be reflected in the returned value.

Note: On platforms where there is no windowing system or cursors are not available, the returned position is based on the mouse move events generated via QWindowSystemInterface.

See also setPos(), QWidget::mapFromGlobal(), QWidget::mapToGlobal(), and QGuiApplication::primaryScreen().
*/
impl /*struct*/ QCursor {
  pub fn pos_0<RetType, T: QCursor_pos_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.pos_0();
    // return 1;
  }
}
pub trait QCursor_pos_0<RetType> {
  fn pos_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCursor_pos_0<usize> for () {
  fn pos_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QCursor3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcursor.h:108
// index:1
// Public static Visibility=Default Availability=Available
// [8] QPoint pos(const QScreen *)

/*
Returns the position of the cursor (hot spot) of the primary screen in global screen coordinates.

You can call QWidget::mapFromGlobal() to translate it to widget coordinates.

Note: The position is queried from the windowing system. If mouse events are generated via other means (e.g., via QWindowSystemInterface in a unit test), those fake mouse moves will not be reflected in the returned value.

Note: On platforms where there is no windowing system or cursors are not available, the returned position is based on the mouse move events generated via QWindowSystemInterface.

See also setPos(), QWidget::mapFromGlobal(), QWidget::mapToGlobal(), and QGuiApplication::primaryScreen().
*/
impl /*struct*/ QCursor {
  pub fn pos_1<RetType, T: QCursor_pos_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.pos_1();
    // return 1;
  }
}
pub trait QCursor_pos_1<RetType> {
  fn pos_1(self ) -> RetType;
}
impl<'a> /*trait*/ QCursor_pos_1<usize> for (usize) {
  fn pos_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QCursor3posEPK7QScreen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcursor.h:109
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setPos(int, int)

/*
Moves the cursor (hot spot) of the primary screen to the global screen position (x, y).

You can call QWidget::mapToGlobal() to translate widget coordinates to global screen coordinates.

See also pos(), QWidget::mapFromGlobal(), QWidget::mapToGlobal(), and QGuiApplication::primaryScreen().
*/
impl /*struct*/ QCursor {
  pub fn setPos_0<RetType, T: QCursor_setPos_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPos_0();
    // return 1;
  }
}
pub trait QCursor_setPos_0<RetType> {
  fn setPos_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCursor_setPos_0<(/*void*/)> for (i32,i32) {
  fn setPos_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QCursor6setPosEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcursor.h:110
// index:1
// Public static Visibility=Default Availability=Available
// [-2] void setPos(QScreen *, int, int)

/*
Moves the cursor (hot spot) of the primary screen to the global screen position (x, y).

You can call QWidget::mapToGlobal() to translate widget coordinates to global screen coordinates.

See also pos(), QWidget::mapFromGlobal(), QWidget::mapToGlobal(), and QGuiApplication::primaryScreen().
*/
impl /*struct*/ QCursor {
  pub fn setPos_1<RetType, T: QCursor_setPos_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPos_1();
    // return 1;
  }
}
pub trait QCursor_setPos_1<RetType> {
  fn setPos_1(self ) -> RetType;
}
impl<'a> /*trait*/ QCursor_setPos_1<(/*void*/)> for (usize,i32,i32) {
  fn setPos_1(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QCursor6setPosEP7QScreenii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcursor.h:111
// index:2
// Public static inline Visibility=Default Availability=Available
// [-2] void setPos(const QPoint &)

/*
Moves the cursor (hot spot) of the primary screen to the global screen position (x, y).

You can call QWidget::mapToGlobal() to translate widget coordinates to global screen coordinates.

See also pos(), QWidget::mapFromGlobal(), QWidget::mapToGlobal(), and QGuiApplication::primaryScreen().
*/
impl /*struct*/ QCursor {
  pub fn setPos_2<RetType, T: QCursor_setPos_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPos_2();
    // return 1;
  }
}
pub trait QCursor_setPos_2<RetType> {
  fn setPos_2(self ) -> RetType;
}
impl<'a> /*trait*/ QCursor_setPos_2<(/*void*/)> for (usize) {
  fn setPos_2(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QCursor6setPosERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcursor.h:112
// index:3
// Public static inline Visibility=Default Availability=Available
// [-2] void setPos(QScreen *, const QPoint &)

/*
Moves the cursor (hot spot) of the primary screen to the global screen position (x, y).

You can call QWidget::mapToGlobal() to translate widget coordinates to global screen coordinates.

See also pos(), QWidget::mapFromGlobal(), QWidget::mapToGlobal(), and QGuiApplication::primaryScreen().
*/
impl /*struct*/ QCursor {
  pub fn setPos_3<RetType, T: QCursor_setPos_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPos_3();
    // return 1;
  }
}
pub trait QCursor_setPos_3<RetType> {
  fn setPos_3(self ) -> RetType;
}
impl<'a> /*trait*/ QCursor_setPos_3<(/*void*/)> for (usize,usize) {
  fn setPos_3(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QCursor6setPosEP7QScreenRK6QPoint", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
