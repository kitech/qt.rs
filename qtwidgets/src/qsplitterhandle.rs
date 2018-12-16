

// mod ::widgets::QSplitterHandle
// package qtwidgets
// /usr/include/qt/QtWidgets/qsplitter.h
// #include <qsplitter.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 37
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// void paintEvent(QPaintEvent *)
// func (this *QSplitterHandle) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QSplitterHandle) InheritMouseMoveEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QSplitterHandle) InheritMousePressEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QSplitterHandle) InheritMouseReleaseEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QSplitterHandle) InheritResizeEvent(f func(arg0 *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// bool event(QEvent *)
// func (this *QSplitterHandle) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void moveSplitter(int)
// func (this *QSplitterHandle) InheritMoveSplitter(f func(p int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "moveSplitter", f)
// }

// int closestLegalPosition(int)
// func (this *QSplitterHandle) InheritClosestLegalPosition(f func(p int) int) {
//  qtrt.SetAllInheritCallback(this, "closestLegalPosition", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSplitterHandle)=48
pub struct QSplitterHandle {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSplitterHandle_ITF interface {
//    QWidget_ITF
//    QSplitterHandle_PTR() *QSplitterHandle
//}
//func (ptr *QSplitterHandle) QSplitterHandle_PTR() *QSplitterHandle { return ptr }

impl /*struct*/ QSplitterHandle {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSplitterHandle {
    return QSplitterHandle{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSplitterHandle {
//  type Target = QSplitterHandleBASE;
//
//  fn deref(&self) -> &QSplitterHandleBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSplitterHandleBASE> for QSplitterHandle {
//  fn as_ref(& self) -> & QSplitterHandleBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qsplitter.h:138
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSplitterHandle {
  pub fn metaObject_0<RetType, T: QSplitterHandle_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSplitterHandle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSplitterHandle10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSplitterHandle(Qt::Orientation, QSplitter *)

/*

*/
// QSplitterHandle(Qt::Orientation, QSplitter *) ctx.fn_proto_cpp
impl /*struct*/ QSplitterHandle {
  pub fn QSplitterHandle_0<T: QSplitterHandle_QSplitterHandle_0>(value: T) -> QSplitterHandle {
    let rsthis = value.QSplitterHandle_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSplitterHandle_QSplitterHandle_0 {
  fn QSplitterHandle_0(self) -> QSplitterHandle;
}
// QSplitterHandle(Qt::Orientation, QSplitter *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSplitterHandle_QSplitterHandle_0 for (i32,usize) {
  fn QSplitterHandle_0(self) -> QSplitterHandle {
    // unsafe{_ZN15QSplitterHandleC2EN2Qt11OrientationEP9QSplitter()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QSplitterHandleC2EN2Qt11OrientationEP9QSplitter", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSplitterHandle{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:141
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSplitterHandle()

/*

*/
pub fn DeleteQSplitterHandle(this :*mut QSplitterHandle) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QSplitterHandleD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qsplitter.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrientation(Qt::Orientation)

/*

*/
impl /*struct*/ QSplitterHandle {
  pub fn setOrientation_0<RetType, T: QSplitterHandle_setOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrientation_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_setOrientation_0<RetType> {
  fn setOrientation_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_setOrientation_0<(/*void*/)> for (i32) {
  fn setOrientation_0(self , rsthis: & QSplitterHandle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QSplitterHandle14setOrientationEN2Qt11OrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:144
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Orientation orientation() const

/*

*/
impl /*struct*/ QSplitterHandle {
  pub fn orientation_0<RetType, T: QSplitterHandle_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QSplitterHandle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSplitterHandle11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:145
// index:0
// Public Visibility=Default Availability=Available
// [1] bool opaqueResize() const

/*

*/
impl /*struct*/ QSplitterHandle {
  pub fn opaqueResize_0<RetType, T: QSplitterHandle_opaqueResize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opaqueResize_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_opaqueResize_0<RetType> {
  fn opaqueResize_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_opaqueResize_0<bool> for () {
  fn opaqueResize_0(self , rsthis: & QSplitterHandle) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSplitterHandle12opaqueResizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:146
// index:0
// Public Visibility=Default Availability=Available
// [8] QSplitter * splitter() const

/*

*/
impl /*struct*/ QSplitterHandle {
  pub fn splitter_0<RetType, T: QSplitterHandle_splitter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.splitter_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_splitter_0<RetType> {
  fn splitter_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_splitter_0<usize> for () {
  fn splitter_0(self , rsthis: & QSplitterHandle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSplitterHandle8splitterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:148
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QSplitterHandle {
  pub fn sizeHint_0<RetType, T: QSplitterHandle_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QSplitterHandle) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSplitterHandle8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:151
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*

*/
impl /*struct*/ QSplitterHandle {
  pub fn paintEvent_0<RetType, T: QSplitterHandle_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QSplitterHandle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QSplitterHandle10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:152
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*

*/
impl /*struct*/ QSplitterHandle {
  pub fn mouseMoveEvent_0<RetType, T: QSplitterHandle_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QSplitterHandle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QSplitterHandle14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:153
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*

*/
impl /*struct*/ QSplitterHandle {
  pub fn mousePressEvent_0<RetType, T: QSplitterHandle_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QSplitterHandle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QSplitterHandle15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:154
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*

*/
impl /*struct*/ QSplitterHandle {
  pub fn mouseReleaseEvent_0<RetType, T: QSplitterHandle_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QSplitterHandle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QSplitterHandle17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:155
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QSplitterHandle {
  pub fn resizeEvent_0<RetType, T: QSplitterHandle_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QSplitterHandle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QSplitterHandle11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:156
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QSplitterHandle {
  pub fn event_0<RetType, T: QSplitterHandle_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_event_0<RetType> {
  fn event_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QSplitterHandle) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QSplitterHandle5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:158
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void moveSplitter(int)

/*
Moves the left or top edge of the splitter handle at index as close as possible to position pos, which is the distance from the left or top edge of the widget.

For right-to-left languages such as Arabic and Hebrew, the layout of horizontal splitters is reversed. pos is then the distance from the right edge of the widget.

See also splitterMoved(), closestLegalPosition(), and getRange().
*/
impl /*struct*/ QSplitterHandle {
  pub fn moveSplitter_0<RetType, T: QSplitterHandle_moveSplitter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveSplitter_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_moveSplitter_0<RetType> {
  fn moveSplitter_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_moveSplitter_0<(/*void*/)> for (i32) {
  fn moveSplitter_0(self , rsthis: & QSplitterHandle) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QSplitterHandle12moveSplitterEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplitter.h:159
// index:0
// Protected Visibility=Default Availability=Available
// [4] int closestLegalPosition(int)

/*
Returns the closest legal position to pos of the widget at index.

For right-to-left languages such as Arabic and Hebrew, the layout of horizontal splitters is reversed. Positions are then measured from the right edge of the widget.

See also getRange().
*/
impl /*struct*/ QSplitterHandle {
  pub fn closestLegalPosition_0<RetType, T: QSplitterHandle_closestLegalPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closestLegalPosition_0(self);
    // return 1;
  }
}
pub trait QSplitterHandle_closestLegalPosition_0<RetType> {
  fn closestLegalPosition_0(self , rsthis: & QSplitterHandle) -> RetType;
}
impl<'a> /*trait*/ QSplitterHandle_closestLegalPosition_0<i32> for (i32) {
  fn closestLegalPosition_0(self , rsthis: & QSplitterHandle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QSplitterHandle20closestLegalPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
