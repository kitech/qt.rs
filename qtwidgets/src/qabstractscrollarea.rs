

// mod ::widgets::QAbstractScrollArea
// package qtwidgets
// /usr/include/qt/QtWidgets/qabstractscrollarea.h
// #include <qabstractscrollarea.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 16
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

// void setViewportMargins(int, int, int, int)
// func (this *QAbstractScrollArea) InheritSetViewportMargins(f func(left int, top int, right int, bottom int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setViewportMargins", f)
// }

// QMargins viewportMargins()
// func (this *QAbstractScrollArea) InheritViewportMargins(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "viewportMargins", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QAbstractScrollArea) InheritEventFilter(f func(arg0 *qtcore.QObject/*777 QObject **/, arg1 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// bool event(QEvent *)
// func (this *QAbstractScrollArea) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// bool viewportEvent(QEvent *)
// func (this *QAbstractScrollArea) InheritViewportEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "viewportEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QAbstractScrollArea) InheritResizeEvent(f func(arg0 *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QAbstractScrollArea) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QAbstractScrollArea) InheritMousePressEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QAbstractScrollArea) InheritMouseReleaseEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QMouseEvent *)
// func (this *QAbstractScrollArea) InheritMouseDoubleClickEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QAbstractScrollArea) InheritMouseMoveEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void wheelEvent(QWheelEvent *)
// func (this *QAbstractScrollArea) InheritWheelEvent(f func(arg0 *qtgui.QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void contextMenuEvent(QContextMenuEvent *)
// func (this *QAbstractScrollArea) InheritContextMenuEvent(f func(arg0 *qtgui.QContextMenuEvent/*777 QContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void dragEnterEvent(QDragEnterEvent *)
// func (this *QAbstractScrollArea) InheritDragEnterEvent(f func(arg0 *qtgui.QDragEnterEvent/*777 QDragEnterEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragEnterEvent", f)
// }

// void dragMoveEvent(QDragMoveEvent *)
// func (this *QAbstractScrollArea) InheritDragMoveEvent(f func(arg0 *qtgui.QDragMoveEvent/*777 QDragMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragMoveEvent", f)
// }

// void dragLeaveEvent(QDragLeaveEvent *)
// func (this *QAbstractScrollArea) InheritDragLeaveEvent(f func(arg0 *qtgui.QDragLeaveEvent/*777 QDragLeaveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragLeaveEvent", f)
// }

// void dropEvent(QDropEvent *)
// func (this *QAbstractScrollArea) InheritDropEvent(f func(arg0 *qtgui.QDropEvent/*777 QDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QAbstractScrollArea) InheritKeyPressEvent(f func(arg0 *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void scrollContentsBy(int, int)
// func (this *QAbstractScrollArea) InheritScrollContentsBy(f func(dx int, dy int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scrollContentsBy", f)
// }

// QSize viewportSizeHint()
// func (this *QAbstractScrollArea) InheritViewportSizeHint(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "viewportSizeHint", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAbstractScrollArea)=48
pub struct QAbstractScrollArea {
  qbase: QFrame,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractScrollArea_ITF interface {
//    QFrame_ITF
//    QAbstractScrollArea_PTR() *QAbstractScrollArea
//}
//func (ptr *QAbstractScrollArea) QAbstractScrollArea_PTR() *QAbstractScrollArea { return ptr }

impl /*struct*/ QAbstractScrollArea {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractScrollArea {
    return QAbstractScrollArea{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractScrollArea {
//  type Target = QAbstractScrollAreaBASE;
//
//  fn deref(&self) -> &QAbstractScrollAreaBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractScrollAreaBASE> for QAbstractScrollArea {
//  fn as_ref(& self) -> & QAbstractScrollAreaBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qabstractscrollarea.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractScrollArea {
  pub fn metaObject_0<RetType, T: QAbstractScrollArea_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractScrollArea10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractScrollArea(QWidget *)

/*
Constructs a viewport.

The parent argument is sent to the QWidget constructor.
*/
// QAbstractScrollArea(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractScrollArea {
  pub fn QAbstractScrollArea_0<T: QAbstractScrollArea_QAbstractScrollArea_0>(value: T) -> QAbstractScrollArea {
    let rsthis = value.QAbstractScrollArea_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractScrollArea_QAbstractScrollArea_0 {
  fn QAbstractScrollArea_0(self) -> QAbstractScrollArea;
}
// QAbstractScrollArea(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractScrollArea_QAbstractScrollArea_0 for (usize) {
  fn QAbstractScrollArea_0(self) -> QAbstractScrollArea {
    // unsafe{_ZN19QAbstractScrollAreaC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QAbstractScrollAreaC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractScrollArea{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractScrollArea()

/*

*/
pub fn DeleteQAbstractScrollArea(this :*mut QAbstractScrollArea) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QAbstractScrollAreaD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qabstractscrollarea.h:74
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ScrollBarPolicy verticalScrollBarPolicy() const

/*

*/
impl /*struct*/ QAbstractScrollArea {
  pub fn verticalScrollBarPolicy_0<RetType, T: QAbstractScrollArea_verticalScrollBarPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalScrollBarPolicy_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_verticalScrollBarPolicy_0<RetType> {
  fn verticalScrollBarPolicy_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_verticalScrollBarPolicy_0<i32> for () {
  fn verticalScrollBarPolicy_0(self , rsthis: & QAbstractScrollArea) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractScrollArea23verticalScrollBarPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalScrollBarPolicy(Qt::ScrollBarPolicy)

/*

*/
impl /*struct*/ QAbstractScrollArea {
  pub fn setVerticalScrollBarPolicy_0<RetType, T: QAbstractScrollArea_setVerticalScrollBarPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalScrollBarPolicy_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_setVerticalScrollBarPolicy_0<RetType> {
  fn setVerticalScrollBarPolicy_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_setVerticalScrollBarPolicy_0<(/*void*/)> for (i32) {
  fn setVerticalScrollBarPolicy_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea26setVerticalScrollBarPolicyEN2Qt15ScrollBarPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] QScrollBar * verticalScrollBar() const

/*
Returns the vertical scroll bar.

See also setVerticalScrollBar(), verticalScrollBarPolicy, and horizontalScrollBar().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn verticalScrollBar_0<RetType, T: QAbstractScrollArea_verticalScrollBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalScrollBar_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_verticalScrollBar_0<RetType> {
  fn verticalScrollBar_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_verticalScrollBar_0<usize> for () {
  fn verticalScrollBar_0(self , rsthis: & QAbstractScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractScrollArea17verticalScrollBarEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalScrollBar(QScrollBar *)

/*
Replaces the existing vertical scroll bar with scrollBar, and sets all the former scroll bar's slider properties on the new scroll bar. The former scroll bar is then deleted.

QAbstractScrollArea already provides vertical and horizontal scroll bars by default. You can call this function to replace the default vertical scroll bar with your own custom scroll bar.

This function was introduced in  Qt 4.2.

See also verticalScrollBar() and setHorizontalScrollBar().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn setVerticalScrollBar_0<RetType, T: QAbstractScrollArea_setVerticalScrollBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalScrollBar_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_setVerticalScrollBar_0<RetType> {
  fn setVerticalScrollBar_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_setVerticalScrollBar_0<(/*void*/)> for (usize) {
  fn setVerticalScrollBar_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea20setVerticalScrollBarEP10QScrollBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:79
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ScrollBarPolicy horizontalScrollBarPolicy() const

/*

*/
impl /*struct*/ QAbstractScrollArea {
  pub fn horizontalScrollBarPolicy_0<RetType, T: QAbstractScrollArea_horizontalScrollBarPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalScrollBarPolicy_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_horizontalScrollBarPolicy_0<RetType> {
  fn horizontalScrollBarPolicy_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_horizontalScrollBarPolicy_0<i32> for () {
  fn horizontalScrollBarPolicy_0(self , rsthis: & QAbstractScrollArea) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractScrollArea25horizontalScrollBarPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHorizontalScrollBarPolicy(Qt::ScrollBarPolicy)

/*

*/
impl /*struct*/ QAbstractScrollArea {
  pub fn setHorizontalScrollBarPolicy_0<RetType, T: QAbstractScrollArea_setHorizontalScrollBarPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalScrollBarPolicy_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_setHorizontalScrollBarPolicy_0<RetType> {
  fn setHorizontalScrollBarPolicy_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_setHorizontalScrollBarPolicy_0<(/*void*/)> for (i32) {
  fn setHorizontalScrollBarPolicy_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea28setHorizontalScrollBarPolicyEN2Qt15ScrollBarPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:81
// index:0
// Public Visibility=Default Availability=Available
// [8] QScrollBar * horizontalScrollBar() const

/*
Returns the horizontal scroll bar.

See also setHorizontalScrollBar(), horizontalScrollBarPolicy, and verticalScrollBar().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn horizontalScrollBar_0<RetType, T: QAbstractScrollArea_horizontalScrollBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalScrollBar_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_horizontalScrollBar_0<RetType> {
  fn horizontalScrollBar_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_horizontalScrollBar_0<usize> for () {
  fn horizontalScrollBar_0(self , rsthis: & QAbstractScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractScrollArea19horizontalScrollBarEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHorizontalScrollBar(QScrollBar *)

/*
Replaces the existing horizontal scroll bar with scrollBar, and sets all the former scroll bar's slider properties on the new scroll bar. The former scroll bar is then deleted.

QAbstractScrollArea already provides horizontal and vertical scroll bars by default. You can call this function to replace the default horizontal scroll bar with your own custom scroll bar.

This function was introduced in  Qt 4.2.

See also horizontalScrollBar() and setVerticalScrollBar().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn setHorizontalScrollBar_0<RetType, T: QAbstractScrollArea_setHorizontalScrollBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalScrollBar_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_setHorizontalScrollBar_0<RetType> {
  fn setHorizontalScrollBar_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_setHorizontalScrollBar_0<(/*void*/)> for (usize) {
  fn setHorizontalScrollBar_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea22setHorizontalScrollBarEP10QScrollBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:84
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * cornerWidget() const

/*
Returns the widget in the corner between the two scroll bars.

By default, no corner widget is present.

This function was introduced in  Qt 4.2.

See also setCornerWidget().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn cornerWidget_0<RetType, T: QAbstractScrollArea_cornerWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cornerWidget_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_cornerWidget_0<RetType> {
  fn cornerWidget_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_cornerWidget_0<usize> for () {
  fn cornerWidget_0(self , rsthis: & QAbstractScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractScrollArea12cornerWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCornerWidget(QWidget *)

/*
Sets the widget in the corner between the two scroll bars to be widget.

You will probably also want to set at least one of the scroll bar modes to AlwaysOn.

Passing 0 shows no widget in the corner.

Any previous corner widget is hidden.

You may call setCornerWidget() with the same widget at different times.

All widgets set here will be deleted by the scroll area when it is destroyed unless you separately reparent the widget after setting some other corner widget (or 0).

Any newly set widget should have no current parent.

By default, no corner widget is present.

This function was introduced in  Qt 4.2.

See also cornerWidget(), horizontalScrollBarPolicy, and horizontalScrollBarPolicy.
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn setCornerWidget_0<RetType, T: QAbstractScrollArea_setCornerWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCornerWidget_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_setCornerWidget_0<RetType> {
  fn setCornerWidget_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_setCornerWidget_0<(/*void*/)> for (usize) {
  fn setCornerWidget_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea15setCornerWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addScrollBarWidget(QWidget *, Qt::Alignment)

/*
Adds widget as a scroll bar widget in the location specified by alignment.

Scroll bar widgets are shown next to the horizontal or vertical scroll bar, and can be placed on either side of it. If you want the scroll bar widgets to be always visible, set the scrollBarPolicy for the corresponding scroll bar to AlwaysOn.

alignment must be one of Qt::Alignleft and Qt::AlignRight, which maps to the horizontal scroll bar, or Qt::AlignTop and Qt::AlignBottom, which maps to the vertical scroll bar.

A scroll bar widget can be removed by either re-parenting the widget or deleting it. It's also possible to hide a widget with QWidget::hide()

The scroll bar widget will be resized to fit the scroll bar geometry for the current style. The following describes the case for scroll bar widgets on the horizontal scroll bar:

The height of the widget will be set to match the height of the scroll bar. To control the width of the widget, use QWidget::setMinimumWidth and QWidget::setMaximumWidth, or implement QWidget::sizeHint() and set a horizontal size policy. If you want a square widget, call QStyle::pixelMetric(QStyle::PM_ScrollBarExtent) and set the width to this value.

This function was introduced in  Qt 4.2.

See also scrollBarWidgets().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn addScrollBarWidget_0<RetType, T: QAbstractScrollArea_addScrollBarWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addScrollBarWidget_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_addScrollBarWidget_0<RetType> {
  fn addScrollBarWidget_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_addScrollBarWidget_0<(/*void*/)> for (usize,i32) {
  fn addScrollBarWidget_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea18addScrollBarWidgetEP7QWidget6QFlagsIN2Qt13AlignmentFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] QWidgetList scrollBarWidgets(Qt::Alignment)

/*
Returns a list of the currently set scroll bar widgets. alignment can be any combination of the four location flags.

This function was introduced in  Qt 4.2.

See also addScrollBarWidget().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn scrollBarWidgets_0<RetType, T: QAbstractScrollArea_scrollBarWidgets_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollBarWidgets_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_scrollBarWidgets_0<RetType> {
  fn scrollBarWidgets_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_scrollBarWidgets_0<usize> for (i32) {
  fn scrollBarWidgets_0(self , rsthis: & QAbstractScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea16scrollBarWidgetsE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:90
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * viewport() const

/*
Returns the viewport widget.

Use the QScrollArea::widget() function to retrieve the contents of the viewport widget.

See also setViewport() and QScrollArea::widget().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn viewport_0<RetType, T: QAbstractScrollArea_viewport_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewport_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_viewport_0<RetType> {
  fn viewport_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_viewport_0<usize> for () {
  fn viewport_0(self , rsthis: & QAbstractScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractScrollArea8viewportEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setViewport(QWidget *)

/*
Sets the viewport to be the given widget. The QAbstractScrollArea will take ownership of the given widget.

If widget is 0, QAbstractScrollArea will assign a new QWidget instance for the viewport.

This function was introduced in  Qt 4.2.

See also viewport().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn setViewport_0<RetType, T: QAbstractScrollArea_setViewport_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setViewport_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_setViewport_0<RetType> {
  fn setViewport_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_setViewport_0<(/*void*/)> for (usize) {
  fn setViewport_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea11setViewportEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:92
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize maximumViewportSize() const

/*
Returns the size of the viewport as if the scroll bars had no valid scrolling range.
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn maximumViewportSize_0<RetType, T: QAbstractScrollArea_maximumViewportSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumViewportSize_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_maximumViewportSize_0<RetType> {
  fn maximumViewportSize_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_maximumViewportSize_0<usize> for () {
  fn maximumViewportSize_0(self , rsthis: & QAbstractScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractScrollArea19maximumViewportSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:94
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn minimumSizeHint_0<RetType, T: QAbstractScrollArea_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QAbstractScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractScrollArea15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:96
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().

Returns the sizeHint property of the scroll area. The size is determined by using viewportSizeHint() plus some extra space for scroll bars, if needed.
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn sizeHint_0<RetType, T: QAbstractScrollArea_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QAbstractScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractScrollArea8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:98
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setupViewport(QWidget *)

/*
This slot is called by QAbstractScrollArea after setViewport(viewport) has been called. Reimplement this function in a subclass of QAbstractScrollArea to initialize the new viewport before it is used.

See also setViewport().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn setupViewport_0<RetType, T: QAbstractScrollArea_setupViewport_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setupViewport_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_setupViewport_0<RetType> {
  fn setupViewport_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_setupViewport_0<(/*void*/)> for (usize) {
  fn setupViewport_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea13setupViewportEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:100
// index:0
// Public Visibility=Default Availability=Available
// [4] QAbstractScrollArea::SizeAdjustPolicy sizeAdjustPolicy() const

/*

*/
impl /*struct*/ QAbstractScrollArea {
  pub fn sizeAdjustPolicy_0<RetType, T: QAbstractScrollArea_sizeAdjustPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeAdjustPolicy_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_sizeAdjustPolicy_0<RetType> {
  fn sizeAdjustPolicy_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_sizeAdjustPolicy_0<i32> for () {
  fn sizeAdjustPolicy_0(self , rsthis: & QAbstractScrollArea) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractScrollArea16sizeAdjustPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSizeAdjustPolicy(QAbstractScrollArea::SizeAdjustPolicy)

/*

*/
impl /*struct*/ QAbstractScrollArea {
  pub fn setSizeAdjustPolicy_0<RetType, T: QAbstractScrollArea_setSizeAdjustPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizeAdjustPolicy_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_setSizeAdjustPolicy_0<RetType> {
  fn setSizeAdjustPolicy_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_setSizeAdjustPolicy_0<(/*void*/)> for (i32) {
  fn setSizeAdjustPolicy_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea19setSizeAdjustPolicyENS_16SizeAdjustPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:105
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setViewportMargins(int, int, int, int)

/*
Sets the margins around the scrolling area to left, top, right and bottom. This is useful for applications such as spreadsheets with "locked" rows and columns. The marginal space is is left blank; put widgets in the unused area.

Note that this function is frequently called by QTreeView and QTableView, so margins must be implemented by QAbstractScrollArea subclasses. Also, if the subclasses are to be used in item views, they should not call this function.

By default all margins are zero.

See also viewportMargins().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn setViewportMargins_0<RetType, T: QAbstractScrollArea_setViewportMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setViewportMargins_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_setViewportMargins_0<RetType> {
  fn setViewportMargins_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_setViewportMargins_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn setViewportMargins_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea18setViewportMarginsEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:106
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void setViewportMargins(const QMargins &)

/*
Sets the margins around the scrolling area to left, top, right and bottom. This is useful for applications such as spreadsheets with "locked" rows and columns. The marginal space is is left blank; put widgets in the unused area.

Note that this function is frequently called by QTreeView and QTableView, so margins must be implemented by QAbstractScrollArea subclasses. Also, if the subclasses are to be used in item views, they should not call this function.

By default all margins are zero.

See also viewportMargins().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn setViewportMargins_1<RetType, T: QAbstractScrollArea_setViewportMargins_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setViewportMargins_1(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_setViewportMargins_1<RetType> {
  fn setViewportMargins_1(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_setViewportMargins_1<(/*void*/)> for (usize) {
  fn setViewportMargins_1(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea18setViewportMarginsERK8QMargins", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:107
// index:0
// Protected Visibility=Default Availability=Available
// [16] QMargins viewportMargins() const

/*
Returns the margins around the scrolling area. By default all the margins are zero.

This function was introduced in  Qt 5.5.

See also setViewportMargins().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn viewportMargins_0<RetType, T: QAbstractScrollArea_viewportMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportMargins_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_viewportMargins_0<RetType> {
  fn viewportMargins_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_viewportMargins_0<usize> for () {
  fn viewportMargins_0(self , rsthis: & QAbstractScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractScrollArea15viewportMarginsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:109
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*

*/
impl /*struct*/ QAbstractScrollArea {
  pub fn eventFilter_0<RetType, T: QAbstractScrollArea_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QAbstractScrollArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:110
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().

This is the main event handler for the QAbstractScrollArea widget (not the scrolling area viewport()). The specified event is a general event object that may need to be cast to the appropriate class depending on its type.

See also QEvent::type().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn event_0<RetType, T: QAbstractScrollArea_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_event_0<RetType> {
  fn event_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QAbstractScrollArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:111
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool viewportEvent(QEvent *)

/*
The main event handler for the scrolling area (the viewport() widget). It handles the event specified, and can be called by subclasses to provide reasonable default behavior.

Returns true to indicate to the event system that the event has been handled, and needs no further processing; otherwise returns false to indicate that the event should be propagated further.

You can reimplement this function in a subclass, but we recommend using one of the specialized event handlers instead.

Specialized handlers for viewport events are: paintEvent(), mousePressEvent(), mouseReleaseEvent(), mouseDoubleClickEvent(), mouseMoveEvent(), wheelEvent(), dragEnterEvent(), dragMoveEvent(), dragLeaveEvent(), dropEvent(), contextMenuEvent(), and resizeEvent().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn viewportEvent_0<RetType, T: QAbstractScrollArea_viewportEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_viewportEvent_0<RetType> {
  fn viewportEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_viewportEvent_0<bool> for (usize) {
  fn viewportEvent_0(self , rsthis: & QAbstractScrollArea) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea13viewportEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:113
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().

This event handler can be reimplemented in a subclass to receive resize events (passed in event), for the viewport() widget.

When resizeEvent() is called, the viewport already has its new geometry: Its new size is accessible through the QResizeEvent::size() function, and the old size through QResizeEvent::oldSize().

See also QWidget::resizeEvent().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn resizeEvent_0<RetType, T: QAbstractScrollArea_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:114
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().

This event handler can be reimplemented in a subclass to receive paint events (passed in event), for the viewport() widget.

Note: If you open a painter, make sure to open it on the viewport().

See also QWidget::paintEvent().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn paintEvent_0<RetType, T: QAbstractScrollArea_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:115
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().

This event handler can be reimplemented in a subclass to receive mouse press events for the viewport() widget. The event is passed in e.

See also QWidget::mousePressEvent().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn mousePressEvent_0<RetType, T: QAbstractScrollArea_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:116
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().

This event handler can be reimplemented in a subclass to receive mouse release events for the viewport() widget. The event is passed in e.

See also QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn mouseReleaseEvent_0<RetType, T: QAbstractScrollArea_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:117
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseDoubleClickEvent().

This event handler can be reimplemented in a subclass to receive mouse double click events for the viewport() widget. The event is passed in e.

See also QWidget::mouseDoubleClickEvent().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn mouseDoubleClickEvent_0<RetType, T: QAbstractScrollArea_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea21mouseDoubleClickEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:118
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().

This event handler can be reimplemented in a subclass to receive mouse move events for the viewport() widget. The event is passed in e.

See also QWidget::mouseMoveEvent().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn mouseMoveEvent_0<RetType, T: QAbstractScrollArea_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:120
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
Reimplemented from QWidget::wheelEvent().

This event handler can be reimplemented in a subclass to receive wheel events for the viewport() widget. The event is passed in e.

See also QWidget::wheelEvent().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn wheelEvent_0<RetType, T: QAbstractScrollArea_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:123
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QContextMenuEvent *)

/*
Reimplemented from QWidget::contextMenuEvent().

This event handler can be reimplemented in a subclass to receive context menu events for the viewport() widget. The event is passed in e.

See also QWidget::contextMenuEvent().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn contextMenuEvent_0<RetType, T: QAbstractScrollArea_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea16contextMenuEventEP17QContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:126
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragEnterEvent(QDragEnterEvent *)

/*
Reimplemented from QWidget::dragEnterEvent().

This event handler can be reimplemented in a subclass to receive drag enter events (passed in event), for the viewport() widget.

See also QWidget::dragEnterEvent().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn dragEnterEvent_0<RetType, T: QAbstractScrollArea_dragEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragEnterEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_dragEnterEvent_0<RetType> {
  fn dragEnterEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_dragEnterEvent_0<(/*void*/)> for (usize) {
  fn dragEnterEvent_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea14dragEnterEventEP15QDragEnterEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:127
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragMoveEvent(QDragMoveEvent *)

/*
Reimplemented from QWidget::dragMoveEvent().

This event handler can be reimplemented in a subclass to receive drag move events (passed in event), for the viewport() widget.

See also QWidget::dragMoveEvent().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn dragMoveEvent_0<RetType, T: QAbstractScrollArea_dragMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMoveEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_dragMoveEvent_0<RetType> {
  fn dragMoveEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_dragMoveEvent_0<(/*void*/)> for (usize) {
  fn dragMoveEvent_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea13dragMoveEventEP14QDragMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:128
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragLeaveEvent(QDragLeaveEvent *)

/*
Reimplemented from QWidget::dragLeaveEvent().

This event handler can be reimplemented in a subclass to receive drag leave events (passed in event), for the viewport() widget.

See also QWidget::dragLeaveEvent().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn dragLeaveEvent_0<RetType, T: QAbstractScrollArea_dragLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_dragLeaveEvent_0<RetType> {
  fn dragLeaveEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_dragLeaveEvent_0<(/*void*/)> for (usize) {
  fn dragLeaveEvent_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea14dragLeaveEventEP15QDragLeaveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:129
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QDropEvent *)

/*
Reimplemented from QWidget::dropEvent().

This event handler can be reimplemented in a subclass to receive drop events (passed in event), for the viewport() widget.

See also QWidget::dropEvent().
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn dropEvent_0<RetType, T: QAbstractScrollArea_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea9dropEventEP10QDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:132
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().

This function is called with key event e when key presses occur. It handles PageUp, PageDown, Up, Down, Left, and Right, and ignores all other key presses.
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn keyPressEvent_0<RetType, T: QAbstractScrollArea_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:134
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void scrollContentsBy(int, int)

/*
This virtual handler is called when the scroll bars are moved by dx, dy, and consequently the viewport's contents should be scrolled accordingly.

The default implementation simply calls update() on the entire viewport(), subclasses can reimplement this handler for optimization purposes, or - like QScrollArea - to move a contents widget. The parameters dx and dy are there for convenience, so that the class knows how much should be scrolled (useful e.g. when doing pixel-shifts). You may just as well ignore these values and scroll directly to the position the scroll bars indicate.

Calling this function in order to scroll programmatically is an error, use the scroll bars instead (e.g. by calling QScrollBar::setValue() directly).
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn scrollContentsBy_0<RetType, T: QAbstractScrollArea_scrollContentsBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollContentsBy_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_scrollContentsBy_0<RetType> {
  fn scrollContentsBy_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_scrollContentsBy_0<(/*void*/)> for (i32,i32) {
  fn scrollContentsBy_0(self , rsthis: & QAbstractScrollArea) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractScrollArea16scrollContentsByEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractscrollarea.h:136
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QSize viewportSizeHint() const

/*
Returns the recommended size for the viewport. The default implementation returns viewport()->sizeHint(). Note that the size is just the viewport's size, without any scroll bars visible.

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QAbstractScrollArea {
  pub fn viewportSizeHint_0<RetType, T: QAbstractScrollArea_viewportSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportSizeHint_0(self);
    // return 1;
  }
}
pub trait QAbstractScrollArea_viewportSizeHint_0<RetType> {
  fn viewportSizeHint_0(self , rsthis: & QAbstractScrollArea) -> RetType;
}
impl<'a> /*trait*/ QAbstractScrollArea_viewportSizeHint_0<usize> for () {
  fn viewportSizeHint_0(self , rsthis: & QAbstractScrollArea) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractScrollArea16viewportSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum specifies how the size hint of the QAbstractScrollArea should adjust when the size of the viewport changes.



This enum was introduced or modified in  Qt 5.2.

*/
pub type QAbstractScrollArea__SizeAdjustPolicy = i32;
// The scroll area will behave like before - and not do any adjust.
pub const QAbstractScrollArea__AdjustIgnored :QAbstractScrollArea__SizeAdjustPolicy = 0;
// The scroll area will adjust to its viewport the first time it is shown.
pub const QAbstractScrollArea__AdjustToContentsOnFirstShow :QAbstractScrollArea__SizeAdjustPolicy = 1;
// The scroll area will always adjust to the viewport
pub const QAbstractScrollArea__AdjustToContents :QAbstractScrollArea__SizeAdjustPolicy = 2;
pub fn QAbstractScrollArea_SizeAdjustPolicyItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractScrollArea", val);
}
pub fn QAbstractScrollArea_SizeAdjustPolicyItemName_s(val: i32) ->String {
  //var nilthis *QAbstractScrollArea
  //return nilthis.SizeAdjustPolicyItemName(val);
  return QAbstractScrollArea_SizeAdjustPolicyItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
