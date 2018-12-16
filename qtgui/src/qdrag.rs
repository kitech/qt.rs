

// mod ::gui::QDrag
// package qtgui
// /usr/include/qt/QtGui/qdrag.h
// #include <qdrag.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 3
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
#[derive(Default)] // class sizeof(QDrag)=16
pub struct QDrag {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDrag_ITF interface {
//    qtcore.QObject_ITF
//    QDrag_PTR() *QDrag
//}
//func (ptr *QDrag) QDrag_PTR() *QDrag { return ptr }

impl /*struct*/ QDrag {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDrag {
    return QDrag{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDrag {
//  type Target = QDragBASE;
//
//  fn deref(&self) -> &QDragBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDragBASE> for QDrag {
//  fn as_ref(& self) -> & QDragBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qdrag.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QDrag {
  pub fn metaObject_0<RetType, T: QDrag_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QDrag_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QDrag) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDrag10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qdrag.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDrag(QObject *)

/*
Constructs a new drag object for the widget specified by dragSource.
*/
// QDrag(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QDrag {
  pub fn QDrag_0<T: QDrag_QDrag_0>(value: T) -> QDrag {
    let rsthis = value.QDrag_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDrag_QDrag_0 {
  fn QDrag_0(self) -> QDrag;
}
// QDrag(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDrag_QDrag_0 for (usize) {
  fn QDrag_0(self) -> QDrag {
    // unsafe{_ZN5QDragC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QDragC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDrag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qdrag.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDrag()

/*

*/
pub fn DeleteQDrag(this :*mut QDrag) {
    // let rv = qtrt::InvokeQtFunc6("_ZN5QDragD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qdrag.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMimeData(QMimeData *)

/*
Sets the data to be sent to the given MIME data. Ownership of the data is transferred to the QDrag object.

See also mimeData().
*/
impl /*struct*/ QDrag {
  pub fn setMimeData_0<RetType, T: QDrag_setMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMimeData_0(self);
    // return 1;
  }
}
pub trait QDrag_setMimeData_0<RetType> {
  fn setMimeData_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_setMimeData_0<(/*void*/)> for (usize) {
  fn setMimeData_0(self , rsthis: & QDrag) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QDrag11setMimeDataEP9QMimeData", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qdrag.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] QMimeData * mimeData() const

/*
Returns the MIME data that is encapsulated by the drag object.

See also setMimeData().
*/
impl /*struct*/ QDrag {
  pub fn mimeData_0<RetType, T: QDrag_mimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeData_0(self);
    // return 1;
  }
}
pub trait QDrag_mimeData_0<RetType> {
  fn mimeData_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_mimeData_0<usize> for () {
  fn mimeData_0(self , rsthis: & QDrag) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDrag8mimeDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qdrag.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPixmap(const QPixmap &)

/*
Sets pixmap as the pixmap used to represent the data in a drag and drop operation. You can only set a pixmap before the drag is started.

See also pixmap().
*/
impl /*struct*/ QDrag {
  pub fn setPixmap_0<RetType, T: QDrag_setPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPixmap_0(self);
    // return 1;
  }
}
pub trait QDrag_setPixmap_0<RetType> {
  fn setPixmap_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_setPixmap_0<(/*void*/)> for (usize) {
  fn setPixmap_0(self , rsthis: & QDrag) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QDrag9setPixmapERK7QPixmap", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qdrag.h:69
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap pixmap() const

/*
Returns the pixmap used to represent the data in a drag and drop operation.

See also setPixmap().
*/
impl /*struct*/ QDrag {
  pub fn pixmap_0<RetType, T: QDrag_pixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixmap_0(self);
    // return 1;
  }
}
pub trait QDrag_pixmap_0<RetType> {
  fn pixmap_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_pixmap_0<usize> for () {
  fn pixmap_0(self , rsthis: & QDrag) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDrag6pixmapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qdrag.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHotSpot(const QPoint &)

/*
Sets the position of the hot spot relative to the top-left corner of the pixmap used to the point specified by hotspot.

Note: on X11, the pixmap may not be able to keep up with the mouse movements if the hot spot causes the pixmap to be displayed directly under the cursor.

See also hotSpot().
*/
impl /*struct*/ QDrag {
  pub fn setHotSpot_0<RetType, T: QDrag_setHotSpot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHotSpot_0(self);
    // return 1;
  }
}
pub trait QDrag_setHotSpot_0<RetType> {
  fn setHotSpot_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_setHotSpot_0<(/*void*/)> for (usize) {
  fn setHotSpot_0(self , rsthis: & QDrag) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QDrag10setHotSpotERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qdrag.h:72
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint hotSpot() const

/*
Returns the position of the hot spot relative to the top-left corner of the cursor.

See also setHotSpot().
*/
impl /*struct*/ QDrag {
  pub fn hotSpot_0<RetType, T: QDrag_hotSpot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hotSpot_0(self);
    // return 1;
  }
}
pub trait QDrag_hotSpot_0<RetType> {
  fn hotSpot_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_hotSpot_0<usize> for () {
  fn hotSpot_0(self , rsthis: & QDrag) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDrag7hotSpotEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qdrag.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QObject * source() const

/*
Returns the source of the drag object. This is the widget where the drag and drop operation originated.
*/
impl /*struct*/ QDrag {
  pub fn source_0<RetType, T: QDrag_source_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.source_0(self);
    // return 1;
  }
}
pub trait QDrag_source_0<RetType> {
  fn source_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_source_0<usize> for () {
  fn source_0(self , rsthis: & QDrag) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDrag6sourceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qdrag.h:75
// index:0
// Public Visibility=Default Availability=Available
// [8] QObject * target() const

/*
Returns the target of the drag and drop operation. This is the widget where the drag object was dropped.
*/
impl /*struct*/ QDrag {
  pub fn target_0<RetType, T: QDrag_target_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.target_0(self);
    // return 1;
  }
}
pub trait QDrag_target_0<RetType> {
  fn target_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_target_0<usize> for () {
  fn target_0(self , rsthis: & QDrag) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDrag6targetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qdrag.h:77
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::DropAction start(Qt::DropActions)

/*

*/
impl /*struct*/ QDrag {
  pub fn start_0<RetType, T: QDrag_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QDrag_start_0<RetType> {
  fn start_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_start_0<i32> for (i32) {
  fn start_0(self , rsthis: & QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDrag5startE6QFlagsIN2Qt10DropActionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qdrag.h:78
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::DropAction exec(Qt::DropActions)

/*
Starts the drag and drop operation and returns a value indicating the requested drop action when it is completed. The drop actions that the user can choose from are specified in supportedActions. The default proposed action will be selected among the allowed actions in the following order: Move, Copy and Link.

Note: On Linux and macOS, the drag and drop operation can take some time, but this function does not block the event loop. Other events are still delivered to the application while the operation is performed. On Windows, the Qt event loop is blocked during the operation.

This function was introduced in  Qt 4.3.

See also cancel().
*/
impl /*struct*/ QDrag {
  pub fn exec_0<RetType, T: QDrag_exec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exec_0(self);
    // return 1;
  }
}
pub trait QDrag_exec_0<RetType> {
  fn exec_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_exec_0<i32> for (i32) {
  fn exec_0(self , rsthis: & QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDrag4execE6QFlagsIN2Qt10DropActionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qdrag.h:79
// index:1
// Public Visibility=Default Availability=Available
// [4] Qt::DropAction exec(Qt::DropActions, Qt::DropAction)

/*
Starts the drag and drop operation and returns a value indicating the requested drop action when it is completed. The drop actions that the user can choose from are specified in supportedActions. The default proposed action will be selected among the allowed actions in the following order: Move, Copy and Link.

Note: On Linux and macOS, the drag and drop operation can take some time, but this function does not block the event loop. Other events are still delivered to the application while the operation is performed. On Windows, the Qt event loop is blocked during the operation.

This function was introduced in  Qt 4.3.

See also cancel().
*/
impl /*struct*/ QDrag {
  pub fn exec_1<RetType, T: QDrag_exec_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exec_1(self);
    // return 1;
  }
}
pub trait QDrag_exec_1<RetType> {
  fn exec_1(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_exec_1<i32> for (i32,i32) {
  fn exec_1(self , rsthis: & QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QDrag4execE6QFlagsIN2Qt10DropActionEES2_", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qdrag.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDragCursor(const QPixmap &, Qt::DropAction)

/*
Sets the drag cursor for the action. This allows you to override the default native cursors. To revert to using the native cursor for action pass in a null QPixmap as cursor.

The action can only be CopyAction, MoveAction or LinkAction. All other values of DropAction are ignored.

See also dragCursor().
*/
impl /*struct*/ QDrag {
  pub fn setDragCursor_0<RetType, T: QDrag_setDragCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDragCursor_0(self);
    // return 1;
  }
}
pub trait QDrag_setDragCursor_0<RetType> {
  fn setDragCursor_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_setDragCursor_0<(/*void*/)> for (usize,i32) {
  fn setDragCursor_0(self , rsthis: & QDrag) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QDrag13setDragCursorERK7QPixmapN2Qt10DropActionE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qdrag.h:82
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap dragCursor(Qt::DropAction) const

/*
Returns the drag cursor for the action.

This function was introduced in  Qt 5.0.

See also setDragCursor().
*/
impl /*struct*/ QDrag {
  pub fn dragCursor_0<RetType, T: QDrag_dragCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragCursor_0(self);
    // return 1;
  }
}
pub trait QDrag_dragCursor_0<RetType> {
  fn dragCursor_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_dragCursor_0<usize> for (i32) {
  fn dragCursor_0(self , rsthis: & QDrag) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDrag10dragCursorEN2Qt10DropActionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qdrag.h:84
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::DropActions supportedActions() const

/*
Returns the set of possible drop actions for this drag operation.

See also exec() and defaultAction().
*/
impl /*struct*/ QDrag {
  pub fn supportedActions_0<RetType, T: QDrag_supportedActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedActions_0(self);
    // return 1;
  }
}
pub trait QDrag_supportedActions_0<RetType> {
  fn supportedActions_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_supportedActions_0<i32> for () {
  fn supportedActions_0(self , rsthis: & QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDrag16supportedActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qdrag.h:85
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::DropAction defaultAction() const

/*
Returns the default proposed drop action for this drag operation.

See also exec() and supportedActions().
*/
impl /*struct*/ QDrag {
  pub fn defaultAction_0<RetType, T: QDrag_defaultAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultAction_0(self);
    // return 1;
  }
}
pub trait QDrag_defaultAction_0<RetType> {
  fn defaultAction_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_defaultAction_0<i32> for () {
  fn defaultAction_0(self , rsthis: & QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QDrag13defaultActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qdrag.h:87
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void cancel()

/*
Cancels a drag operation initiated by Qt.

Note: This is currently implemented on Windows and X11.

This function was introduced in  Qt 5.7.

See also exec().
*/
impl /*struct*/ QDrag {
  pub fn cancel_0<RetType, T: QDrag_cancel_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.cancel_0();
    // return 1;
  }
}
pub trait QDrag_cancel_0<RetType> {
  fn cancel_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDrag_cancel_0<(/*void*/)> for () {
  fn cancel_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN5QDrag6cancelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qdrag.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void actionChanged(Qt::DropAction)

/*
This signal is emitted when the action associated with the drag changes.

See also targetChanged().
*/
impl /*struct*/ QDrag {
  pub fn actionChanged_0<RetType, T: QDrag_actionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionChanged_0(self);
    // return 1;
  }
}
pub trait QDrag_actionChanged_0<RetType> {
  fn actionChanged_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_actionChanged_0<(/*void*/)> for (i32) {
  fn actionChanged_0(self , rsthis: & QDrag) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QDrag13actionChangedEN2Qt10DropActionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qdrag.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void targetChanged(QObject *)

/*
This signal is emitted when the target of the drag and drop operation changes, with newTarget the new target.

See also target() and actionChanged().
*/
impl /*struct*/ QDrag {
  pub fn targetChanged_0<RetType, T: QDrag_targetChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.targetChanged_0(self);
    // return 1;
  }
}
pub trait QDrag_targetChanged_0<RetType> {
  fn targetChanged_0(self , rsthis: & QDrag) -> RetType;
}
impl<'a> /*trait*/ QDrag_targetChanged_0<(/*void*/)> for (usize) {
  fn targetChanged_0(self , rsthis: & QDrag) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QDrag13targetChangedEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
