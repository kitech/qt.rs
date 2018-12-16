

// mod ::widgets::QWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qwidget.h
// #include <qwidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 0
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

// bool event(QEvent *)
// func (this *QWidget) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QWidget) InheritMousePressEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QWidget) InheritMouseReleaseEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QMouseEvent *)
// func (this *QWidget) InheritMouseDoubleClickEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QWidget) InheritMouseMoveEvent(f func(event *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void wheelEvent(QWheelEvent *)
// func (this *QWidget) InheritWheelEvent(f func(event *qtgui.QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QWidget) InheritKeyPressEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void keyReleaseEvent(QKeyEvent *)
// func (this *QWidget) InheritKeyReleaseEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyReleaseEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QWidget) InheritFocusInEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QWidget) InheritFocusOutEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void enterEvent(QEvent *)
// func (this *QWidget) InheritEnterEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "enterEvent", f)
// }

// void leaveEvent(QEvent *)
// func (this *QWidget) InheritLeaveEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "leaveEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QWidget) InheritPaintEvent(f func(event *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void moveEvent(QMoveEvent *)
// func (this *QWidget) InheritMoveEvent(f func(event *qtgui.QMoveEvent/*777 QMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "moveEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QWidget) InheritResizeEvent(f func(event *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void closeEvent(QCloseEvent *)
// func (this *QWidget) InheritCloseEvent(f func(event *qtgui.QCloseEvent/*777 QCloseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "closeEvent", f)
// }

// void contextMenuEvent(QContextMenuEvent *)
// func (this *QWidget) InheritContextMenuEvent(f func(event *qtgui.QContextMenuEvent/*777 QContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void tabletEvent(QTabletEvent *)
// func (this *QWidget) InheritTabletEvent(f func(event *qtgui.QTabletEvent/*777 QTabletEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "tabletEvent", f)
// }

// void actionEvent(QActionEvent *)
// func (this *QWidget) InheritActionEvent(f func(event *qtgui.QActionEvent/*777 QActionEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "actionEvent", f)
// }

// void dragEnterEvent(QDragEnterEvent *)
// func (this *QWidget) InheritDragEnterEvent(f func(event *qtgui.QDragEnterEvent/*777 QDragEnterEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragEnterEvent", f)
// }

// void dragMoveEvent(QDragMoveEvent *)
// func (this *QWidget) InheritDragMoveEvent(f func(event *qtgui.QDragMoveEvent/*777 QDragMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragMoveEvent", f)
// }

// void dragLeaveEvent(QDragLeaveEvent *)
// func (this *QWidget) InheritDragLeaveEvent(f func(event *qtgui.QDragLeaveEvent/*777 QDragLeaveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragLeaveEvent", f)
// }

// void dropEvent(QDropEvent *)
// func (this *QWidget) InheritDropEvent(f func(event *qtgui.QDropEvent/*777 QDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QWidget) InheritShowEvent(f func(event *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void hideEvent(QHideEvent *)
// func (this *QWidget) InheritHideEvent(f func(event *qtgui.QHideEvent/*777 QHideEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hideEvent", f)
// }

// bool nativeEvent(const QByteArray &, void *, long *)
// func (this *QWidget) InheritNativeEvent(f func(eventType *qtcore.QByteArray, message unsafe.Pointer /*666*/, result unsafe.Pointer /*666*/) bool) {
//  qtrt.SetAllInheritCallback(this, "nativeEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QWidget) InheritChangeEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// int metric(QPaintDevice::PaintDeviceMetric)
// func (this *QWidget) InheritMetric(f func(arg0 int) int) {
//  qtrt.SetAllInheritCallback(this, "metric", f)
// }

// void initPainter(QPainter *)
// func (this *QWidget) InheritInitPainter(f func(painter *qtgui.QPainter/*777 QPainter **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initPainter", f)
// }

// QPaintDevice * redirected(QPoint *)
// func (this *QWidget) InheritRedirected(f func(offset *qtcore.QPoint/*777 QPoint **/) unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "redirected", f)
// }

// QPainter * sharedPainter()
// func (this *QWidget) InheritSharedPainter(f func() unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "sharedPainter", f)
// }

// void inputMethodEvent(QInputMethodEvent *)
// func (this *QWidget) InheritInputMethodEvent(f func(arg0 *qtgui.QInputMethodEvent/*777 QInputMethodEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "inputMethodEvent", f)
// }

// void updateMicroFocus()
// func (this *QWidget) InheritUpdateMicroFocus(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateMicroFocus", f)
// }

// void create(WId, bool, bool)
// func (this *QWidget) InheritCreate(f func(arg0 uint64, initializeWindow bool, destroyOldWindow bool) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "create", f)
// }

// void destroy(bool, bool)
// func (this *QWidget) InheritDestroy(f func(destroyWindow bool, destroySubWindows bool) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "destroy", f)
// }

// bool focusNextPrevChild(bool)
// func (this *QWidget) InheritFocusNextPrevChild(f func(next bool) bool) {
//  qtrt.SetAllInheritCallback(this, "focusNextPrevChild", f)
// }

// bool focusNextChild()
// func (this *QWidget) InheritFocusNextChild(f func() bool) {
//  qtrt.SetAllInheritCallback(this, "focusNextChild", f)
// }

// bool focusPreviousChild()
// func (this *QWidget) InheritFocusPreviousChild(f func() bool) {
//  qtrt.SetAllInheritCallback(this, "focusPreviousChild", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QWidget)=48
pub struct QWidget {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWidget_ITF interface {
//    qtcore.QObject_ITF
//    qtgui.QPaintDevice_ITF
//    QWidget_PTR() *QWidget
//}
//func (ptr *QWidget) QWidget_PTR() *QWidget { return ptr }

impl /*struct*/ QWidget {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWidget {
    return QWidget{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWidget {
//  type Target = QWidgetBASE;
//
//  fn deref(&self) -> &QWidgetBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWidgetBASE> for QWidget {
//  fn as_ref(& self) -> & QWidgetBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qwidget.h:130
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QWidget {
  pub fn metaObject_0<RetType, T: QWidget_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QWidget_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:214
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QWidget(QWidget *, Qt::WindowFlags)

/*
Constructs a widget which is a child of parent, with widget flags set to f.

If parent is 0, the new widget becomes a window. If parent is another widget, this widget becomes a child window inside parent. The new widget is deleted when its parent is deleted.

The widget flags argument, f, is normally 0, but it can be set to customize the frame of a window (i.e. parent must be 0). To customize the frame, use a value composed from the bitwise OR of any of the window flags.

If you add a child widget to an already visible widget you must explicitly show the child to make it visible.

Note that the X11 version of Qt may not be able to deliver all combinations of style flags on all systems. This is because on X11, Qt can only ask the window manager, and the window manager can override the application's settings. On Windows, Qt can set whatever flags you want.

See also windowFlags.
*/
// QWidget(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QWidget {
  pub fn QWidget_0<T: QWidget_QWidget_0>(value: T) -> QWidget {
    let rsthis = value.QWidget_0();
    return rsthis;
    // return 1;
  }
}

pub trait QWidget_QWidget_0 {
  fn QWidget_0(self) -> QWidget;
}
// QWidget(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWidget_QWidget_0 for (usize,i32) {
  fn QWidget_0(self) -> QWidget {
    // unsafe{_ZN7QWidgetC2EPS_6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QWidgetC2EPS_6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:215
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QWidget()

/*

*/
pub fn DeleteQWidget(this :*mut QWidget) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QWidgetD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qwidget.h:217
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int devType() const

/*

*/
impl /*struct*/ QWidget {
  pub fn devType_0<RetType, T: QWidget_devType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.devType_0(self);
    // return 1;
  }
}
pub trait QWidget_devType_0<RetType> {
  fn devType_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_devType_0<i32> for () {
  fn devType_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget7devTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:219
// index:0
// Public Visibility=Default Availability=Available
// [8] WId winId() const

/*
Returns the window system identifier of the widget.

Portable in principle, but if you use it you are probably about to do something non-portable. Be careful.

If a widget is non-native (alien) and winId() is invoked on it, that widget will be provided a native handle.

This value may change at run-time. An event with type QEvent::WinIdChange will be sent to the widget following a change in window system identifier.

See also find().
*/
impl /*struct*/ QWidget {
  pub fn winId_0<RetType, T: QWidget_winId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.winId_0(self);
    // return 1;
  }
}
pub trait QWidget_winId_0<RetType> {
  fn winId_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_winId_0<u64> for () {
  fn winId_0(self , rsthis: & QWidget) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget5winIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:220
// index:0
// Public Visibility=Default Availability=Available
// [-2] void createWinId()

/*

*/
impl /*struct*/ QWidget {
  pub fn createWinId_0<RetType, T: QWidget_createWinId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createWinId_0(self);
    // return 1;
  }
}
pub trait QWidget_createWinId_0<RetType> {
  fn createWinId_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_createWinId_0<(/*void*/)> for () {
  fn createWinId_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget11createWinIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:221
// index:0
// Public inline Visibility=Default Availability=Available
// [8] WId internalWinId() const

/*

*/
impl /*struct*/ QWidget {
  pub fn internalWinId_0<RetType, T: QWidget_internalWinId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.internalWinId_0(self);
    // return 1;
  }
}
pub trait QWidget_internalWinId_0<RetType> {
  fn internalWinId_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_internalWinId_0<u64> for () {
  fn internalWinId_0(self , rsthis: & QWidget) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget13internalWinIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:222
// index:0
// Public Visibility=Default Availability=Available
// [8] WId effectiveWinId() const

/*
Returns the effective window system identifier of the widget, i.e. the native parent's window system identifier.

If the widget is native, this function returns the native widget ID. Otherwise, the window ID of the first native parent widget, i.e., the top-level widget that contains this widget, is returned.

Note: We recommend that you do not store this value as it is likely to change at run-time.

This function was introduced in  Qt 4.4.

See also nativeParentWidget().
*/
impl /*struct*/ QWidget {
  pub fn effectiveWinId_0<RetType, T: QWidget_effectiveWinId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.effectiveWinId_0(self);
    // return 1;
  }
}
pub trait QWidget_effectiveWinId_0<RetType> {
  fn effectiveWinId_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_effectiveWinId_0<u64> for () {
  fn effectiveWinId_0(self , rsthis: & QWidget) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14effectiveWinIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:225
// index:0
// Public Visibility=Default Availability=Available
// [8] QStyle * style() const

/*
See also QWidget::setStyle(), QApplication::setStyle(), and QApplication::style().
*/
impl /*struct*/ QWidget {
  pub fn style_0<RetType, T: QWidget_style_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.style_0(self);
    // return 1;
  }
}
pub trait QWidget_style_0<RetType> {
  fn style_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_style_0<usize> for () {
  fn style_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget5styleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:226
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStyle(QStyle *)

/*
Sets the widget's GUI style to style. The ownership of the style object is not transferred.

If no style is set, the widget uses the application's style, QApplication::style() instead.

Setting a widget's style has no effect on existing or future child widgets.

Warning: This function is particularly useful for demonstration purposes, where you want to show Qt's styling capabilities. Real applications should avoid it and use one consistent GUI style instead.

Warning: Qt style sheets are currently not supported for custom QStyle subclasses. We plan to address this in some future release.

See also style(), QStyle, QApplication::style(), and QApplication::setStyle().
*/
impl /*struct*/ QWidget {
  pub fn setStyle_0<RetType, T: QWidget_setStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStyle_0(self);
    // return 1;
  }
}
pub trait QWidget_setStyle_0<RetType> {
  fn setStyle_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setStyle_0<(/*void*/)> for (usize) {
  fn setStyle_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget8setStyleEP6QStyle", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:229
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isTopLevel() const

/*

*/
impl /*struct*/ QWidget {
  pub fn isTopLevel_0<RetType, T: QWidget_isTopLevel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTopLevel_0(self);
    // return 1;
  }
}
pub trait QWidget_isTopLevel_0<RetType> {
  fn isTopLevel_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isTopLevel_0<bool> for () {
  fn isTopLevel_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget10isTopLevelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:230
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isWindow() const

/*
Returns true if the widget is an independent window, otherwise returns false.

A window is a widget that isn't visually the child of any other widget and that usually has a frame and a window title.

A window can have a parent widget. It will then be grouped with its parent and deleted when the parent is deleted, minimized when the parent is minimized etc. If supported by the window manager, it will also have a common taskbar entry with its parent.

QDialog and QMainWindow widgets are by default windows, even if a parent widget is specified in the constructor. This behavior is specified by the Qt::Window flag.

See also window(), isModal(), and parentWidget().
*/
impl /*struct*/ QWidget {
  pub fn isWindow_0<RetType, T: QWidget_isWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isWindow_0(self);
    // return 1;
  }
}
pub trait QWidget_isWindow_0<RetType> {
  fn isWindow_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isWindow_0<bool> for () {
  fn isWindow_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget8isWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:232
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isModal() const

/*

*/
impl /*struct*/ QWidget {
  pub fn isModal_0<RetType, T: QWidget_isModal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isModal_0(self);
    // return 1;
  }
}
pub trait QWidget_isModal_0<RetType> {
  fn isModal_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isModal_0<bool> for () {
  fn isModal_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget7isModalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:233
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::WindowModality windowModality() const

/*

*/
impl /*struct*/ QWidget {
  pub fn windowModality_0<RetType, T: QWidget_windowModality_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowModality_0(self);
    // return 1;
  }
}
pub trait QWidget_windowModality_0<RetType> {
  fn windowModality_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowModality_0<i32> for () {
  fn windowModality_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14windowModalityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:234
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowModality(Qt::WindowModality)

/*

*/
impl /*struct*/ QWidget {
  pub fn setWindowModality_0<RetType, T: QWidget_setWindowModality_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowModality_0(self);
    // return 1;
  }
}
pub trait QWidget_setWindowModality_0<RetType> {
  fn setWindowModality_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setWindowModality_0<(/*void*/)> for (i32) {
  fn setWindowModality_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget17setWindowModalityEN2Qt14WindowModalityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:236
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEnabled() const

/*

*/
impl /*struct*/ QWidget {
  pub fn isEnabled_0<RetType, T: QWidget_isEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEnabled_0(self);
    // return 1;
  }
}
pub trait QWidget_isEnabled_0<RetType> {
  fn isEnabled_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isEnabled_0<bool> for () {
  fn isEnabled_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget9isEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:237
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEnabledTo(const QWidget *) const

/*
Returns true if this widget would become enabled if ancestor is enabled; otherwise returns false.

This is the case if neither the widget itself nor every parent up to but excluding ancestor has been explicitly disabled.

isEnabledTo(0) returns false if this widget or any if its ancestors was explicitly disabled.

The word ancestor here means a parent widget within the same window.

Therefore isEnabledTo(0) stops at this widget's window, unlike isEnabled() which also takes parent windows into considerations.

See also setEnabled() and enabled.
*/
impl /*struct*/ QWidget {
  pub fn isEnabledTo_0<RetType, T: QWidget_isEnabledTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEnabledTo_0(self);
    // return 1;
  }
}
pub trait QWidget_isEnabledTo_0<RetType> {
  fn isEnabledTo_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isEnabledTo_0<bool> for (usize) {
  fn isEnabledTo_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11isEnabledToEPKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:238
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEnabledToTLW() const

/*

*/
impl /*struct*/ QWidget {
  pub fn isEnabledToTLW_0<RetType, T: QWidget_isEnabledToTLW_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEnabledToTLW_0(self);
    // return 1;
  }
}
pub trait QWidget_isEnabledToTLW_0<RetType> {
  fn isEnabledToTLW_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isEnabledToTLW_0<bool> for () {
  fn isEnabledToTLW_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14isEnabledToTLWEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:241
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEnabled(bool)

/*

*/
impl /*struct*/ QWidget {
  pub fn setEnabled_0<RetType, T: QWidget_setEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEnabled_0(self);
    // return 1;
  }
}
pub trait QWidget_setEnabled_0<RetType> {
  fn setEnabled_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setEnabled_0<(/*void*/)> for (bool) {
  fn setEnabled_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget10setEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:242
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDisabled(bool)

/*
Disables widget input events if disable is true; otherwise enables input events.

See the enabled documentation for more information.

See also isEnabledTo(), QKeyEvent, QMouseEvent, and changeEvent().
*/
impl /*struct*/ QWidget {
  pub fn setDisabled_0<RetType, T: QWidget_setDisabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDisabled_0(self);
    // return 1;
  }
}
pub trait QWidget_setDisabled_0<RetType> {
  fn setDisabled_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setDisabled_0<(/*void*/)> for (bool) {
  fn setDisabled_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget11setDisabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:243
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowModified(bool)

/*

*/
impl /*struct*/ QWidget {
  pub fn setWindowModified_0<RetType, T: QWidget_setWindowModified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowModified_0(self);
    // return 1;
  }
}
pub trait QWidget_setWindowModified_0<RetType> {
  fn setWindowModified_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setWindowModified_0<(/*void*/)> for (bool) {
  fn setWindowModified_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget17setWindowModifiedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:248
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect frameGeometry() const

/*

*/
impl /*struct*/ QWidget {
  pub fn frameGeometry_0<RetType, T: QWidget_frameGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameGeometry_0(self);
    // return 1;
  }
}
pub trait QWidget_frameGeometry_0<RetType> {
  fn frameGeometry_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_frameGeometry_0<usize> for () {
  fn frameGeometry_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget13frameGeometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:249
// index:0
// Public Visibility=Default Availability=Available
// [16] const QRect & geometry() const

/*

*/
impl /*struct*/ QWidget {
  pub fn geometry_0<RetType, T: QWidget_geometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.geometry_0(self);
    // return 1;
  }
}
pub trait QWidget_geometry_0<RetType> {
  fn geometry_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_geometry_0<usize> for () {
  fn geometry_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget8geometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:250
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect normalGeometry() const

/*

*/
impl /*struct*/ QWidget {
  pub fn normalGeometry_0<RetType, T: QWidget_normalGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalGeometry_0(self);
    // return 1;
  }
}
pub trait QWidget_normalGeometry_0<RetType> {
  fn normalGeometry_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_normalGeometry_0<usize> for () {
  fn normalGeometry_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14normalGeometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:252
// index:0
// Public Visibility=Default Availability=Available
// [4] int x() const

/*

*/
impl /*struct*/ QWidget {
  pub fn x_0<RetType, T: QWidget_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QWidget_x_0<RetType> {
  fn x_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_x_0<i32> for () {
  fn x_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:253
// index:0
// Public Visibility=Default Availability=Available
// [4] int y() const

/*

*/
impl /*struct*/ QWidget {
  pub fn y_0<RetType, T: QWidget_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QWidget_y_0<RetType> {
  fn y_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_y_0<i32> for () {
  fn y_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:254
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint pos() const

/*

*/
impl /*struct*/ QWidget {
  pub fn pos_0<RetType, T: QWidget_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QWidget_pos_0<RetType> {
  fn pos_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:255
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize frameSize() const

/*

*/
impl /*struct*/ QWidget {
  pub fn frameSize_0<RetType, T: QWidget_frameSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameSize_0(self);
    // return 1;
  }
}
pub trait QWidget_frameSize_0<RetType> {
  fn frameSize_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_frameSize_0<usize> for () {
  fn frameSize_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget9frameSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:256
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize size() const

/*

*/
impl /*struct*/ QWidget {
  pub fn size_0<RetType, T: QWidget_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QWidget_size_0<RetType> {
  fn size_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_size_0<usize> for () {
  fn size_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:257
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int width() const

/*

*/
impl /*struct*/ QWidget {
  pub fn width_0<RetType, T: QWidget_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QWidget_width_0<RetType> {
  fn width_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_width_0<i32> for () {
  fn width_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:258
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int height() const

/*

*/
impl /*struct*/ QWidget {
  pub fn height_0<RetType, T: QWidget_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QWidget_height_0<RetType> {
  fn height_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_height_0<i32> for () {
  fn height_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:259
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect rect() const

/*

*/
impl /*struct*/ QWidget {
  pub fn rect_0<RetType, T: QWidget_rect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rect_0(self);
    // return 1;
  }
}
pub trait QWidget_rect_0<RetType> {
  fn rect_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_rect_0<usize> for () {
  fn rect_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget4rectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:260
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect childrenRect() const

/*

*/
impl /*struct*/ QWidget {
  pub fn childrenRect_0<RetType, T: QWidget_childrenRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childrenRect_0(self);
    // return 1;
  }
}
pub trait QWidget_childrenRect_0<RetType> {
  fn childrenRect_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_childrenRect_0<usize> for () {
  fn childrenRect_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget12childrenRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:261
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion childrenRegion() const

/*

*/
impl /*struct*/ QWidget {
  pub fn childrenRegion_0<RetType, T: QWidget_childrenRegion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childrenRegion_0(self);
    // return 1;
  }
}
pub trait QWidget_childrenRegion_0<RetType> {
  fn childrenRegion_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_childrenRegion_0<usize> for () {
  fn childrenRegion_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14childrenRegionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:263
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize minimumSize() const

/*

*/
impl /*struct*/ QWidget {
  pub fn minimumSize_0<RetType, T: QWidget_minimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSize_0(self);
    // return 1;
  }
}
pub trait QWidget_minimumSize_0<RetType> {
  fn minimumSize_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_minimumSize_0<usize> for () {
  fn minimumSize_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11minimumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:264
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize maximumSize() const

/*

*/
impl /*struct*/ QWidget {
  pub fn maximumSize_0<RetType, T: QWidget_maximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumSize_0(self);
    // return 1;
  }
}
pub trait QWidget_maximumSize_0<RetType> {
  fn maximumSize_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_maximumSize_0<usize> for () {
  fn maximumSize_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11maximumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:265
// index:0
// Public Visibility=Default Availability=Available
// [4] int minimumWidth() const

/*

*/
impl /*struct*/ QWidget {
  pub fn minimumWidth_0<RetType, T: QWidget_minimumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumWidth_0(self);
    // return 1;
  }
}
pub trait QWidget_minimumWidth_0<RetType> {
  fn minimumWidth_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_minimumWidth_0<i32> for () {
  fn minimumWidth_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget12minimumWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:266
// index:0
// Public Visibility=Default Availability=Available
// [4] int minimumHeight() const

/*

*/
impl /*struct*/ QWidget {
  pub fn minimumHeight_0<RetType, T: QWidget_minimumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumHeight_0(self);
    // return 1;
  }
}
pub trait QWidget_minimumHeight_0<RetType> {
  fn minimumHeight_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_minimumHeight_0<i32> for () {
  fn minimumHeight_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget13minimumHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:267
// index:0
// Public Visibility=Default Availability=Available
// [4] int maximumWidth() const

/*

*/
impl /*struct*/ QWidget {
  pub fn maximumWidth_0<RetType, T: QWidget_maximumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumWidth_0(self);
    // return 1;
  }
}
pub trait QWidget_maximumWidth_0<RetType> {
  fn maximumWidth_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_maximumWidth_0<i32> for () {
  fn maximumWidth_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget12maximumWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:268
// index:0
// Public Visibility=Default Availability=Available
// [4] int maximumHeight() const

/*

*/
impl /*struct*/ QWidget {
  pub fn maximumHeight_0<RetType, T: QWidget_maximumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumHeight_0(self);
    // return 1;
  }
}
pub trait QWidget_maximumHeight_0<RetType> {
  fn maximumHeight_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_maximumHeight_0<i32> for () {
  fn maximumHeight_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget13maximumHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:269
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumSize(const QSize &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setMinimumSize_0<RetType, T: QWidget_setMinimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumSize_0(self);
    // return 1;
  }
}
pub trait QWidget_setMinimumSize_0<RetType> {
  fn setMinimumSize_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setMinimumSize_0<(/*void*/)> for (usize) {
  fn setMinimumSize_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget14setMinimumSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:270
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setMinimumSize(int, int)

/*

*/
impl /*struct*/ QWidget {
  pub fn setMinimumSize_1<RetType, T: QWidget_setMinimumSize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumSize_1(self);
    // return 1;
  }
}
pub trait QWidget_setMinimumSize_1<RetType> {
  fn setMinimumSize_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setMinimumSize_1<(/*void*/)> for (i32,i32) {
  fn setMinimumSize_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget14setMinimumSizeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:271
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumSize(const QSize &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setMaximumSize_0<RetType, T: QWidget_setMaximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumSize_0(self);
    // return 1;
  }
}
pub trait QWidget_setMaximumSize_0<RetType> {
  fn setMaximumSize_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setMaximumSize_0<(/*void*/)> for (usize) {
  fn setMaximumSize_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget14setMaximumSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:272
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setMaximumSize(int, int)

/*

*/
impl /*struct*/ QWidget {
  pub fn setMaximumSize_1<RetType, T: QWidget_setMaximumSize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumSize_1(self);
    // return 1;
  }
}
pub trait QWidget_setMaximumSize_1<RetType> {
  fn setMaximumSize_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setMaximumSize_1<(/*void*/)> for (i32,i32) {
  fn setMaximumSize_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget14setMaximumSizeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:273
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumWidth(int)

/*

*/
impl /*struct*/ QWidget {
  pub fn setMinimumWidth_0<RetType, T: QWidget_setMinimumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumWidth_0(self);
    // return 1;
  }
}
pub trait QWidget_setMinimumWidth_0<RetType> {
  fn setMinimumWidth_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setMinimumWidth_0<(/*void*/)> for (i32) {
  fn setMinimumWidth_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget15setMinimumWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:274
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumHeight(int)

/*

*/
impl /*struct*/ QWidget {
  pub fn setMinimumHeight_0<RetType, T: QWidget_setMinimumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumHeight_0(self);
    // return 1;
  }
}
pub trait QWidget_setMinimumHeight_0<RetType> {
  fn setMinimumHeight_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setMinimumHeight_0<(/*void*/)> for (i32) {
  fn setMinimumHeight_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget16setMinimumHeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:275
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumWidth(int)

/*

*/
impl /*struct*/ QWidget {
  pub fn setMaximumWidth_0<RetType, T: QWidget_setMaximumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumWidth_0(self);
    // return 1;
  }
}
pub trait QWidget_setMaximumWidth_0<RetType> {
  fn setMaximumWidth_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setMaximumWidth_0<(/*void*/)> for (i32) {
  fn setMaximumWidth_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget15setMaximumWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:276
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumHeight(int)

/*

*/
impl /*struct*/ QWidget {
  pub fn setMaximumHeight_0<RetType, T: QWidget_setMaximumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumHeight_0(self);
    // return 1;
  }
}
pub trait QWidget_setMaximumHeight_0<RetType> {
  fn setMaximumHeight_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setMaximumHeight_0<(/*void*/)> for (i32) {
  fn setMaximumHeight_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget16setMaximumHeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:282
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize sizeIncrement() const

/*

*/
impl /*struct*/ QWidget {
  pub fn sizeIncrement_0<RetType, T: QWidget_sizeIncrement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeIncrement_0(self);
    // return 1;
  }
}
pub trait QWidget_sizeIncrement_0<RetType> {
  fn sizeIncrement_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_sizeIncrement_0<usize> for () {
  fn sizeIncrement_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget13sizeIncrementEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:283
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSizeIncrement(const QSize &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setSizeIncrement_0<RetType, T: QWidget_setSizeIncrement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizeIncrement_0(self);
    // return 1;
  }
}
pub trait QWidget_setSizeIncrement_0<RetType> {
  fn setSizeIncrement_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setSizeIncrement_0<(/*void*/)> for (usize) {
  fn setSizeIncrement_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget16setSizeIncrementERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:284
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setSizeIncrement(int, int)

/*

*/
impl /*struct*/ QWidget {
  pub fn setSizeIncrement_1<RetType, T: QWidget_setSizeIncrement_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizeIncrement_1(self);
    // return 1;
  }
}
pub trait QWidget_setSizeIncrement_1<RetType> {
  fn setSizeIncrement_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setSizeIncrement_1<(/*void*/)> for (i32,i32) {
  fn setSizeIncrement_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget16setSizeIncrementEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:285
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize baseSize() const

/*

*/
impl /*struct*/ QWidget {
  pub fn baseSize_0<RetType, T: QWidget_baseSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.baseSize_0(self);
    // return 1;
  }
}
pub trait QWidget_baseSize_0<RetType> {
  fn baseSize_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_baseSize_0<usize> for () {
  fn baseSize_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget8baseSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:286
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBaseSize(const QSize &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setBaseSize_0<RetType, T: QWidget_setBaseSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBaseSize_0(self);
    // return 1;
  }
}
pub trait QWidget_setBaseSize_0<RetType> {
  fn setBaseSize_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setBaseSize_0<(/*void*/)> for (usize) {
  fn setBaseSize_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget11setBaseSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:287
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setBaseSize(int, int)

/*

*/
impl /*struct*/ QWidget {
  pub fn setBaseSize_1<RetType, T: QWidget_setBaseSize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBaseSize_1(self);
    // return 1;
  }
}
pub trait QWidget_setBaseSize_1<RetType> {
  fn setBaseSize_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setBaseSize_1<(/*void*/)> for (i32,i32) {
  fn setBaseSize_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget11setBaseSizeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:289
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFixedSize(const QSize &)

/*
Sets both the minimum and maximum sizes of the widget to s, thereby preventing it from ever growing or shrinking.

This will override the default size constraints set by QLayout.

To remove constraints, set the size to QWIDGETSIZE_MAX.

Alternatively, if you want the widget to have a fixed size based on its contents, you can call QLayout::setSizeConstraint(QLayout::SetFixedSize);

See also maximumSize and minimumSize.
*/
impl /*struct*/ QWidget {
  pub fn setFixedSize_0<RetType, T: QWidget_setFixedSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFixedSize_0(self);
    // return 1;
  }
}
pub trait QWidget_setFixedSize_0<RetType> {
  fn setFixedSize_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setFixedSize_0<(/*void*/)> for (usize) {
  fn setFixedSize_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget12setFixedSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:290
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setFixedSize(int, int)

/*
Sets both the minimum and maximum sizes of the widget to s, thereby preventing it from ever growing or shrinking.

This will override the default size constraints set by QLayout.

To remove constraints, set the size to QWIDGETSIZE_MAX.

Alternatively, if you want the widget to have a fixed size based on its contents, you can call QLayout::setSizeConstraint(QLayout::SetFixedSize);

See also maximumSize and minimumSize.
*/
impl /*struct*/ QWidget {
  pub fn setFixedSize_1<RetType, T: QWidget_setFixedSize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFixedSize_1(self);
    // return 1;
  }
}
pub trait QWidget_setFixedSize_1<RetType> {
  fn setFixedSize_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setFixedSize_1<(/*void*/)> for (i32,i32) {
  fn setFixedSize_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget12setFixedSizeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:291
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFixedWidth(int)

/*
Sets both the minimum and maximum width of the widget to w without changing the heights. Provided for convenience.

See also sizeHint(), minimumSize(), maximumSize(), and setFixedSize().
*/
impl /*struct*/ QWidget {
  pub fn setFixedWidth_0<RetType, T: QWidget_setFixedWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFixedWidth_0(self);
    // return 1;
  }
}
pub trait QWidget_setFixedWidth_0<RetType> {
  fn setFixedWidth_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setFixedWidth_0<(/*void*/)> for (i32) {
  fn setFixedWidth_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget13setFixedWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:292
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFixedHeight(int)

/*
Sets both the minimum and maximum heights of the widget to h without changing the widths. Provided for convenience.

See also sizeHint(), minimumSize(), maximumSize(), and setFixedSize().
*/
impl /*struct*/ QWidget {
  pub fn setFixedHeight_0<RetType, T: QWidget_setFixedHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFixedHeight_0(self);
    // return 1;
  }
}
pub trait QWidget_setFixedHeight_0<RetType> {
  fn setFixedHeight_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setFixedHeight_0<(/*void*/)> for (i32) {
  fn setFixedHeight_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget14setFixedHeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:296
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint mapToGlobal(const QPoint &) const

/*
Translates the widget coordinate pos to global screen coordinates. For example, mapToGlobal(QPoint(0,0)) would give the global coordinates of the top-left pixel of the widget.

See also mapFromGlobal(), mapTo(), and mapToParent().
*/
impl /*struct*/ QWidget {
  pub fn mapToGlobal_0<RetType, T: QWidget_mapToGlobal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToGlobal_0(self);
    // return 1;
  }
}
pub trait QWidget_mapToGlobal_0<RetType> {
  fn mapToGlobal_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_mapToGlobal_0<usize> for (usize) {
  fn mapToGlobal_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11mapToGlobalERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:297
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint mapFromGlobal(const QPoint &) const

/*
Translates the global screen coordinate pos to widget coordinates.

See also mapToGlobal(), mapFrom(), and mapFromParent().
*/
impl /*struct*/ QWidget {
  pub fn mapFromGlobal_0<RetType, T: QWidget_mapFromGlobal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromGlobal_0(self);
    // return 1;
  }
}
pub trait QWidget_mapFromGlobal_0<RetType> {
  fn mapFromGlobal_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_mapFromGlobal_0<usize> for (usize) {
  fn mapFromGlobal_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget13mapFromGlobalERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:298
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint mapToParent(const QPoint &) const

/*
Translates the widget coordinate pos to a coordinate in the parent widget.

Same as mapToGlobal() if the widget has no parent.

See also mapFromParent(), mapTo(), mapToGlobal(), and underMouse().
*/
impl /*struct*/ QWidget {
  pub fn mapToParent_0<RetType, T: QWidget_mapToParent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToParent_0(self);
    // return 1;
  }
}
pub trait QWidget_mapToParent_0<RetType> {
  fn mapToParent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_mapToParent_0<usize> for (usize) {
  fn mapToParent_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11mapToParentERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:299
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint mapFromParent(const QPoint &) const

/*
Translates the parent widget coordinate pos to widget coordinates.

Same as mapFromGlobal() if the widget has no parent.

See also mapToParent(), mapFrom(), mapFromGlobal(), and underMouse().
*/
impl /*struct*/ QWidget {
  pub fn mapFromParent_0<RetType, T: QWidget_mapFromParent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromParent_0(self);
    // return 1;
  }
}
pub trait QWidget_mapFromParent_0<RetType> {
  fn mapFromParent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_mapFromParent_0<usize> for (usize) {
  fn mapFromParent_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget13mapFromParentERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:300
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint mapTo(const QWidget *, const QPoint &) const

/*
Translates the widget coordinate pos to the coordinate system of parent. The parent must not be 0 and must be a parent of the calling widget.

See also mapFrom(), mapToParent(), mapToGlobal(), and underMouse().
*/
impl /*struct*/ QWidget {
  pub fn mapTo_0<RetType, T: QWidget_mapTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapTo_0(self);
    // return 1;
  }
}
pub trait QWidget_mapTo_0<RetType> {
  fn mapTo_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_mapTo_0<usize> for (usize,usize) {
  fn mapTo_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget5mapToEPKS_RK6QPoint", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:301
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint mapFrom(const QWidget *, const QPoint &) const

/*
Translates the widget coordinate pos from the coordinate system of parent to this widget's coordinate system. The parent must not be 0 and must be a parent of the calling widget.

See also mapTo(), mapFromParent(), mapFromGlobal(), and underMouse().
*/
impl /*struct*/ QWidget {
  pub fn mapFrom_0<RetType, T: QWidget_mapFrom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFrom_0(self);
    // return 1;
  }
}
pub trait QWidget_mapFrom_0<RetType> {
  fn mapFrom_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_mapFrom_0<usize> for (usize,usize) {
  fn mapFrom_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget7mapFromEPKS_RK6QPoint", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:303
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * window() const

/*
Returns the window for this widget, i.e. the next ancestor widget that has (or could have) a window-system frame.

If the widget is a window, the widget itself is returned.

Typical usage is changing the window title:


  aWidget->window()->setWindowTitle("New Window Title");



See also isWindow().
*/
impl /*struct*/ QWidget {
  pub fn window_0<RetType, T: QWidget_window_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.window_0(self);
    // return 1;
  }
}
pub trait QWidget_window_0<RetType> {
  fn window_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_window_0<usize> for () {
  fn window_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget6windowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:304
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * nativeParentWidget() const

/*
Returns the native parent for this widget, i.e. the next ancestor widget that has a system identifier, or 0 if it does not have any native parent.

This function was introduced in  Qt 4.4.

See also effectiveWinId().
*/
impl /*struct*/ QWidget {
  pub fn nativeParentWidget_0<RetType, T: QWidget_nativeParentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nativeParentWidget_0(self);
    // return 1;
  }
}
pub trait QWidget_nativeParentWidget_0<RetType> {
  fn nativeParentWidget_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_nativeParentWidget_0<usize> for () {
  fn nativeParentWidget_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget18nativeParentWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:305
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QWidget * topLevelWidget() const

/*

*/
impl /*struct*/ QWidget {
  pub fn topLevelWidget_0<RetType, T: QWidget_topLevelWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topLevelWidget_0(self);
    // return 1;
  }
}
pub trait QWidget_topLevelWidget_0<RetType> {
  fn topLevelWidget_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_topLevelWidget_0<usize> for () {
  fn topLevelWidget_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14topLevelWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:308
// index:0
// Public Visibility=Default Availability=Available
// [16] const QPalette & palette() const

/*

*/
impl /*struct*/ QWidget {
  pub fn palette_0<RetType, T: QWidget_palette_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.palette_0(self);
    // return 1;
  }
}
pub trait QWidget_palette_0<RetType> {
  fn palette_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_palette_0<usize> for () {
  fn palette_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget7paletteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:309
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPalette(const QPalette &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setPalette_0<RetType, T: QWidget_setPalette_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPalette_0(self);
    // return 1;
  }
}
pub trait QWidget_setPalette_0<RetType> {
  fn setPalette_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setPalette_0<(/*void*/)> for (usize) {
  fn setPalette_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget10setPaletteERK8QPalette", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:311
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBackgroundRole(QPalette::ColorRole)

/*
Sets the background role of the widget to role.

The background role defines the brush from the widget's palette that is used to render the background.

If role is QPalette::NoRole, then the widget inherits its parent's background role.

Note that styles are free to choose any color from the palette. You can modify the palette or set a style sheet if you don't achieve the result you want with setBackgroundRole().

See also backgroundRole() and foregroundRole().
*/
impl /*struct*/ QWidget {
  pub fn setBackgroundRole_0<RetType, T: QWidget_setBackgroundRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundRole_0(self);
    // return 1;
  }
}
pub trait QWidget_setBackgroundRole_0<RetType> {
  fn setBackgroundRole_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setBackgroundRole_0<(/*void*/)> for (i32) {
  fn setBackgroundRole_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget17setBackgroundRoleEN8QPalette9ColorRoleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:312
// index:0
// Public Visibility=Default Availability=Available
// [4] QPalette::ColorRole backgroundRole() const

/*
Returns the background role of the widget.

The background role defines the brush from the widget's palette that is used to render the background.

If no explicit background role is set, the widget inherts its parent widget's background role.

See also setBackgroundRole() and foregroundRole().
*/
impl /*struct*/ QWidget {
  pub fn backgroundRole_0<RetType, T: QWidget_backgroundRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backgroundRole_0(self);
    // return 1;
  }
}
pub trait QWidget_backgroundRole_0<RetType> {
  fn backgroundRole_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_backgroundRole_0<i32> for () {
  fn backgroundRole_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14backgroundRoleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:314
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setForegroundRole(QPalette::ColorRole)

/*
Sets the foreground role of the widget to role.

The foreground role defines the color from the widget's palette that is used to draw the foreground.

If role is QPalette::NoRole, the widget uses a foreground role that contrasts with the background role.

Note that styles are free to choose any color from the palette. You can modify the palette or set a style sheet if you don't achieve the result you want with setForegroundRole().

See also foregroundRole() and backgroundRole().
*/
impl /*struct*/ QWidget {
  pub fn setForegroundRole_0<RetType, T: QWidget_setForegroundRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setForegroundRole_0(self);
    // return 1;
  }
}
pub trait QWidget_setForegroundRole_0<RetType> {
  fn setForegroundRole_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setForegroundRole_0<(/*void*/)> for (i32) {
  fn setForegroundRole_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget17setForegroundRoleEN8QPalette9ColorRoleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:315
// index:0
// Public Visibility=Default Availability=Available
// [4] QPalette::ColorRole foregroundRole() const

/*
Returns the foreground role.

The foreground role defines the color from the widget's palette that is used to draw the foreground.

If no explicit foreground role is set, the function returns a role that contrasts with the background role.

See also setForegroundRole() and backgroundRole().
*/
impl /*struct*/ QWidget {
  pub fn foregroundRole_0<RetType, T: QWidget_foregroundRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.foregroundRole_0(self);
    // return 1;
  }
}
pub trait QWidget_foregroundRole_0<RetType> {
  fn foregroundRole_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_foregroundRole_0<i32> for () {
  fn foregroundRole_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14foregroundRoleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:317
// index:0
// Public Visibility=Default Availability=Available
// [16] const QFont & font() const

/*

*/
impl /*struct*/ QWidget {
  pub fn font_0<RetType, T: QWidget_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QWidget_font_0<RetType> {
  fn font_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_font_0<usize> for () {
  fn font_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:318
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setFont_0<RetType, T: QWidget_setFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_0(self);
    // return 1;
  }
}
pub trait QWidget_setFont_0<RetType> {
  fn setFont_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setFont_0<(/*void*/)> for (usize) {
  fn setFont_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:319
// index:0
// Public Visibility=Default Availability=Available
// [8] QFontMetrics fontMetrics() const

/*
Returns the font metrics for the widget's current font. Equivalent to QFontMetrics(widget->font()).

See also font(), fontInfo(), and setFont().
*/
impl /*struct*/ QWidget {
  pub fn fontMetrics_0<RetType, T: QWidget_fontMetrics_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontMetrics_0(self);
    // return 1;
  }
}
pub trait QWidget_fontMetrics_0<RetType> {
  fn fontMetrics_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_fontMetrics_0<usize> for () {
  fn fontMetrics_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11fontMetricsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:320
// index:0
// Public Visibility=Default Availability=Available
// [8] QFontInfo fontInfo() const

/*
Returns the font info for the widget's current font. Equivalent to QFontInfo(widget->font()).

See also font(), fontMetrics(), and setFont().
*/
impl /*struct*/ QWidget {
  pub fn fontInfo_0<RetType, T: QWidget_fontInfo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontInfo_0(self);
    // return 1;
  }
}
pub trait QWidget_fontInfo_0<RetType> {
  fn fontInfo_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_fontInfo_0<usize> for () {
  fn fontInfo_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget8fontInfoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:323
// index:0
// Public Visibility=Default Availability=Available
// [8] QCursor cursor() const

/*

*/
impl /*struct*/ QWidget {
  pub fn cursor_0<RetType, T: QWidget_cursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursor_0(self);
    // return 1;
  }
}
pub trait QWidget_cursor_0<RetType> {
  fn cursor_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_cursor_0<usize> for () {
  fn cursor_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget6cursorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:324
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCursor(const QCursor &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setCursor_0<RetType, T: QWidget_setCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCursor_0(self);
    // return 1;
  }
}
pub trait QWidget_setCursor_0<RetType> {
  fn setCursor_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setCursor_0<(/*void*/)> for (usize) {
  fn setCursor_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget9setCursorERK7QCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:325
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unsetCursor()

/*

*/
impl /*struct*/ QWidget {
  pub fn unsetCursor_0<RetType, T: QWidget_unsetCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unsetCursor_0(self);
    // return 1;
  }
}
pub trait QWidget_unsetCursor_0<RetType> {
  fn unsetCursor_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_unsetCursor_0<(/*void*/)> for () {
  fn unsetCursor_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget11unsetCursorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:328
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMouseTracking(bool)

/*

*/
impl /*struct*/ QWidget {
  pub fn setMouseTracking_0<RetType, T: QWidget_setMouseTracking_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMouseTracking_0(self);
    // return 1;
  }
}
pub trait QWidget_setMouseTracking_0<RetType> {
  fn setMouseTracking_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setMouseTracking_0<(/*void*/)> for (bool) {
  fn setMouseTracking_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget16setMouseTrackingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:329
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasMouseTracking() const

/*

*/
impl /*struct*/ QWidget {
  pub fn hasMouseTracking_0<RetType, T: QWidget_hasMouseTracking_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasMouseTracking_0(self);
    // return 1;
  }
}
pub trait QWidget_hasMouseTracking_0<RetType> {
  fn hasMouseTracking_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_hasMouseTracking_0<bool> for () {
  fn hasMouseTracking_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget16hasMouseTrackingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:330
// index:0
// Public Visibility=Default Availability=Available
// [1] bool underMouse() const

/*
Returns true if the widget is under the mouse cursor; otherwise returns false.

This value is not updated properly during drag and drop operations.

See also enterEvent() and leaveEvent().
*/
impl /*struct*/ QWidget {
  pub fn underMouse_0<RetType, T: QWidget_underMouse_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.underMouse_0(self);
    // return 1;
  }
}
pub trait QWidget_underMouse_0<RetType> {
  fn underMouse_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_underMouse_0<bool> for () {
  fn underMouse_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget10underMouseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:332
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabletTracking(bool)

/*

*/
impl /*struct*/ QWidget {
  pub fn setTabletTracking_0<RetType, T: QWidget_setTabletTracking_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabletTracking_0(self);
    // return 1;
  }
}
pub trait QWidget_setTabletTracking_0<RetType> {
  fn setTabletTracking_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setTabletTracking_0<(/*void*/)> for (bool) {
  fn setTabletTracking_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget17setTabletTrackingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:333
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasTabletTracking() const

/*

*/
impl /*struct*/ QWidget {
  pub fn hasTabletTracking_0<RetType, T: QWidget_hasTabletTracking_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasTabletTracking_0(self);
    // return 1;
  }
}
pub trait QWidget_hasTabletTracking_0<RetType> {
  fn hasTabletTracking_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_hasTabletTracking_0<bool> for () {
  fn hasTabletTracking_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget17hasTabletTrackingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:335
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMask(const QBitmap &)

/*
Causes only the pixels of the widget for which bitmap has a corresponding 1 bit to be visible. If the region includes pixels outside the rect() of the widget, window system controls in that area may or may not be visible, depending on the platform.

Note that this effect can be slow if the region is particularly complex.

The following code shows how an image with an alpha channel can be used to generate a mask for a widget:


      QLabel topLevelLabel;
      QPixmap pixmap(":/images/tux.png");
      topLevelLabel.setPixmap(pixmap);
      topLevelLabel.setMask(pixmap.mask());



The label shown by this code is masked using the image it contains, giving the appearance that an irregularly-shaped image is being drawn directly onto the screen.

Masked widgets receive mouse events only on their visible portions.

See also mask(), clearMask(), windowOpacity(), and Shaped Clock Example.
*/
impl /*struct*/ QWidget {
  pub fn setMask_0<RetType, T: QWidget_setMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMask_0(self);
    // return 1;
  }
}
pub trait QWidget_setMask_0<RetType> {
  fn setMask_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setMask_0<(/*void*/)> for (usize) {
  fn setMask_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget7setMaskERK7QBitmap", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:336
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setMask(const QRegion &)

/*
Causes only the pixels of the widget for which bitmap has a corresponding 1 bit to be visible. If the region includes pixels outside the rect() of the widget, window system controls in that area may or may not be visible, depending on the platform.

Note that this effect can be slow if the region is particularly complex.

The following code shows how an image with an alpha channel can be used to generate a mask for a widget:


      QLabel topLevelLabel;
      QPixmap pixmap(":/images/tux.png");
      topLevelLabel.setPixmap(pixmap);
      topLevelLabel.setMask(pixmap.mask());



The label shown by this code is masked using the image it contains, giving the appearance that an irregularly-shaped image is being drawn directly onto the screen.

Masked widgets receive mouse events only on their visible portions.

See also mask(), clearMask(), windowOpacity(), and Shaped Clock Example.
*/
impl /*struct*/ QWidget {
  pub fn setMask_1<RetType, T: QWidget_setMask_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMask_1(self);
    // return 1;
  }
}
pub trait QWidget_setMask_1<RetType> {
  fn setMask_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setMask_1<(/*void*/)> for (usize) {
  fn setMask_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget7setMaskERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:337
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion mask() const

/*
Returns the mask currently set on a widget. If no mask is set the return value will be an empty region.

See also setMask(), clearMask(), QRegion::isEmpty(), and Shaped Clock Example.
*/
impl /*struct*/ QWidget {
  pub fn mask_0<RetType, T: QWidget_mask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mask_0(self);
    // return 1;
  }
}
pub trait QWidget_mask_0<RetType> {
  fn mask_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_mask_0<usize> for () {
  fn mask_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget4maskEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:338
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearMask()

/*
Removes any mask set by setMask().

See also setMask().
*/
impl /*struct*/ QWidget {
  pub fn clearMask_0<RetType, T: QWidget_clearMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearMask_0(self);
    // return 1;
  }
}
pub trait QWidget_clearMask_0<RetType> {
  fn clearMask_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_clearMask_0<(/*void*/)> for () {
  fn clearMask_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget9clearMaskEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:340
// index:0
// Public Visibility=Default Availability=Available
// [-2] void render(QPaintDevice *, const QPoint &, const QRegion &, QWidget::RenderFlags)

/*
Renders the sourceRegion of this widget into the target using renderFlags to determine how to render. Rendering starts at targetOffset in the target. For example:


  QPixmap pixmap(widget->size());
  widget->render(&pixmap);



If sourceRegion is a null region, this function will use QWidget::rect() as the region, i.e. the entire widget.

Ensure that you call QPainter::end() for the target device's active painter (if any) before rendering. For example:


  QPainter painter(this);
  ...
  painter.end();
  myWidget->render(this);



Note: To obtain the contents of a QOpenGLWidget, use QOpenGLWidget::grabFramebuffer() instead.

Note: To obtain the contents of a QGLWidget (deprecated), use QGLWidget::grabFrameBuffer() or QGLWidget::renderPixmap() instead.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QWidget {
  pub fn render_0<RetType, T: QWidget_render_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.render_0(self);
    // return 1;
  }
}
pub trait QWidget_render_0<RetType> {
  fn render_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_render_0<(/*void*/)> for (usize,usize,usize,i32) {
  fn render_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget6renderEP12QPaintDeviceRK6QPointRK7QRegion6QFlagsINS_10RenderFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:344
// index:1
// Public Visibility=Default Availability=Available
// [-2] void render(QPainter *, const QPoint &, const QRegion &, QWidget::RenderFlags)

/*
Renders the sourceRegion of this widget into the target using renderFlags to determine how to render. Rendering starts at targetOffset in the target. For example:


  QPixmap pixmap(widget->size());
  widget->render(&pixmap);



If sourceRegion is a null region, this function will use QWidget::rect() as the region, i.e. the entire widget.

Ensure that you call QPainter::end() for the target device's active painter (if any) before rendering. For example:


  QPainter painter(this);
  ...
  painter.end();
  myWidget->render(this);



Note: To obtain the contents of a QOpenGLWidget, use QOpenGLWidget::grabFramebuffer() instead.

Note: To obtain the contents of a QGLWidget (deprecated), use QGLWidget::grabFrameBuffer() or QGLWidget::renderPixmap() instead.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QWidget {
  pub fn render_1<RetType, T: QWidget_render_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.render_1(self);
    // return 1;
  }
}
pub trait QWidget_render_1<RetType> {
  fn render_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_render_1<(/*void*/)> for (usize,usize,usize,i32) {
  fn render_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget6renderEP8QPainterRK6QPointRK7QRegion6QFlagsINS_10RenderFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:348
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap grab(const QRect &)

/*
Renders the widget into a pixmap restricted by the given rectangle. If the widget has any children, then they are also painted in the appropriate positions.

If a rectangle with an invalid size is specified (the default), the entire widget is painted.

This function was introduced in  Qt 5.0.

See also render() and QPixmap.
*/
impl /*struct*/ QWidget {
  pub fn grab_0<RetType, T: QWidget_grab_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.grab_0(self);
    // return 1;
  }
}
pub trait QWidget_grab_0<RetType> {
  fn grab_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_grab_0<usize> for (usize) {
  fn grab_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWidget4grabERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:351
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsEffect * graphicsEffect() const

/*
The graphicsEffect function returns a pointer to the widget's graphics effect.

If the widget has no graphics effect, 0 is returned.

This function was introduced in  Qt 4.6.

See also setGraphicsEffect().
*/
impl /*struct*/ QWidget {
  pub fn graphicsEffect_0<RetType, T: QWidget_graphicsEffect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.graphicsEffect_0(self);
    // return 1;
  }
}
pub trait QWidget_graphicsEffect_0<RetType> {
  fn graphicsEffect_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_graphicsEffect_0<usize> for () {
  fn graphicsEffect_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14graphicsEffectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:352
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGraphicsEffect(QGraphicsEffect *)

/*
The setGraphicsEffect function is for setting the widget's graphics effect.

Sets effect as the widget's effect. If there already is an effect installed on this widget, QWidget will delete the existing effect before installing the new effect.

If effect is the installed effect on a different widget, setGraphicsEffect() will remove the effect from the widget and install it on this widget.

QWidget takes ownership of effect.

Note: This function will apply the effect on itself and all its children.

Note: Graphics effects are not supported for OpenGL-based widgets, such as QGLWidget, QOpenGLWidget and QQuickWidget.

This function was introduced in  Qt 4.6.

See also graphicsEffect().
*/
impl /*struct*/ QWidget {
  pub fn setGraphicsEffect_0<RetType, T: QWidget_setGraphicsEffect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGraphicsEffect_0(self);
    // return 1;
  }
}
pub trait QWidget_setGraphicsEffect_0<RetType> {
  fn setGraphicsEffect_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setGraphicsEffect_0<(/*void*/)> for (usize) {
  fn setGraphicsEffect_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget17setGraphicsEffectEP15QGraphicsEffect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:356
// index:0
// Public Visibility=Default Availability=Available
// [-2] void grabGesture(Qt::GestureType, Qt::GestureFlags)

/*
Subscribes the widget to a given gesture with specific flags.

This function was introduced in  Qt 4.6.

See also ungrabGesture() and QGestureEvent.
*/
impl /*struct*/ QWidget {
  pub fn grabGesture_0<RetType, T: QWidget_grabGesture_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.grabGesture_0(self);
    // return 1;
  }
}
pub trait QWidget_grabGesture_0<RetType> {
  fn grabGesture_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_grabGesture_0<(/*void*/)> for (i32,i32) {
  fn grabGesture_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget11grabGestureEN2Qt11GestureTypeE6QFlagsINS0_11GestureFlagEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:357
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ungrabGesture(Qt::GestureType)

/*
Unsubscribes the widget from a given gesture type

This function was introduced in  Qt 4.6.

See also grabGesture() and QGestureEvent.
*/
impl /*struct*/ QWidget {
  pub fn ungrabGesture_0<RetType, T: QWidget_ungrabGesture_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ungrabGesture_0(self);
    // return 1;
  }
}
pub trait QWidget_ungrabGesture_0<RetType> {
  fn ungrabGesture_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_ungrabGesture_0<(/*void*/)> for (i32) {
  fn ungrabGesture_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget13ungrabGestureEN2Qt11GestureTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:361
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowTitle(const QString &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setWindowTitle_0<RetType, T: QWidget_setWindowTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowTitle_0(self);
    // return 1;
  }
}
pub trait QWidget_setWindowTitle_0<RetType> {
  fn setWindowTitle_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setWindowTitle_0<(/*void*/)> for (usize) {
  fn setWindowTitle_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget14setWindowTitleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:363
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStyleSheet(const QString &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setStyleSheet_0<RetType, T: QWidget_setStyleSheet_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStyleSheet_0(self);
    // return 1;
  }
}
pub trait QWidget_setStyleSheet_0<RetType> {
  fn setStyleSheet_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setStyleSheet_0<(/*void*/)> for (usize) {
  fn setStyleSheet_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget13setStyleSheetERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:367
// index:0
// Public Visibility=Default Availability=Available
// [8] QString styleSheet() const

/*

*/
impl /*struct*/ QWidget {
  pub fn styleSheet_0<RetType, T: QWidget_styleSheet_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styleSheet_0(self);
    // return 1;
  }
}
pub trait QWidget_styleSheet_0<RetType> {
  fn styleSheet_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_styleSheet_0<usize> for () {
  fn styleSheet_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget10styleSheetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:369
// index:0
// Public Visibility=Default Availability=Available
// [8] QString windowTitle() const

/*

*/
impl /*struct*/ QWidget {
  pub fn windowTitle_0<RetType, T: QWidget_windowTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowTitle_0(self);
    // return 1;
  }
}
pub trait QWidget_windowTitle_0<RetType> {
  fn windowTitle_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowTitle_0<usize> for () {
  fn windowTitle_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11windowTitleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:370
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowIcon(const QIcon &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setWindowIcon_0<RetType, T: QWidget_setWindowIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowIcon_0(self);
    // return 1;
  }
}
pub trait QWidget_setWindowIcon_0<RetType> {
  fn setWindowIcon_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setWindowIcon_0<(/*void*/)> for (usize) {
  fn setWindowIcon_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget13setWindowIconERK5QIcon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:371
// index:0
// Public Visibility=Default Availability=Available
// [8] QIcon windowIcon() const

/*

*/
impl /*struct*/ QWidget {
  pub fn windowIcon_0<RetType, T: QWidget_windowIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowIcon_0(self);
    // return 1;
  }
}
pub trait QWidget_windowIcon_0<RetType> {
  fn windowIcon_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowIcon_0<usize> for () {
  fn windowIcon_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget10windowIconEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:372
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowIconText(const QString &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setWindowIconText_0<RetType, T: QWidget_setWindowIconText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowIconText_0(self);
    // return 1;
  }
}
pub trait QWidget_setWindowIconText_0<RetType> {
  fn setWindowIconText_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setWindowIconText_0<(/*void*/)> for (usize) {
  fn setWindowIconText_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget17setWindowIconTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:373
// index:0
// Public Visibility=Default Availability=Available
// [8] QString windowIconText() const

/*

*/
impl /*struct*/ QWidget {
  pub fn windowIconText_0<RetType, T: QWidget_windowIconText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowIconText_0(self);
    // return 1;
  }
}
pub trait QWidget_windowIconText_0<RetType> {
  fn windowIconText_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowIconText_0<usize> for () {
  fn windowIconText_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14windowIconTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:374
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowRole(const QString &)

/*
Sets the window's role to role. This only makes sense for windows on X11.

See also windowRole().
*/
impl /*struct*/ QWidget {
  pub fn setWindowRole_0<RetType, T: QWidget_setWindowRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowRole_0(self);
    // return 1;
  }
}
pub trait QWidget_setWindowRole_0<RetType> {
  fn setWindowRole_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setWindowRole_0<(/*void*/)> for (usize) {
  fn setWindowRole_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget13setWindowRoleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:375
// index:0
// Public Visibility=Default Availability=Available
// [8] QString windowRole() const

/*
Returns the window's role, or an empty string.

See also setWindowRole(), windowIcon, and windowTitle.
*/
impl /*struct*/ QWidget {
  pub fn windowRole_0<RetType, T: QWidget_windowRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowRole_0(self);
    // return 1;
  }
}
pub trait QWidget_windowRole_0<RetType> {
  fn windowRole_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowRole_0<usize> for () {
  fn windowRole_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget10windowRoleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:376
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowFilePath(const QString &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setWindowFilePath_0<RetType, T: QWidget_setWindowFilePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowFilePath_0(self);
    // return 1;
  }
}
pub trait QWidget_setWindowFilePath_0<RetType> {
  fn setWindowFilePath_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setWindowFilePath_0<(/*void*/)> for (usize) {
  fn setWindowFilePath_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget17setWindowFilePathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:377
// index:0
// Public Visibility=Default Availability=Available
// [8] QString windowFilePath() const

/*

*/
impl /*struct*/ QWidget {
  pub fn windowFilePath_0<RetType, T: QWidget_windowFilePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowFilePath_0(self);
    // return 1;
  }
}
pub trait QWidget_windowFilePath_0<RetType> {
  fn windowFilePath_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowFilePath_0<usize> for () {
  fn windowFilePath_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14windowFilePathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:379
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowOpacity(qreal)

/*

*/
impl /*struct*/ QWidget {
  pub fn setWindowOpacity_0<RetType, T: QWidget_setWindowOpacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowOpacity_0(self);
    // return 1;
  }
}
pub trait QWidget_setWindowOpacity_0<RetType> {
  fn setWindowOpacity_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setWindowOpacity_0<(/*void*/)> for (f64) {
  fn setWindowOpacity_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget16setWindowOpacityEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:380
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal windowOpacity() const

/*

*/
impl /*struct*/ QWidget {
  pub fn windowOpacity_0<RetType, T: QWidget_windowOpacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowOpacity_0(self);
    // return 1;
  }
}
pub trait QWidget_windowOpacity_0<RetType> {
  fn windowOpacity_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowOpacity_0<f64> for () {
  fn windowOpacity_0(self , rsthis: & QWidget) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget13windowOpacityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:382
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isWindowModified() const

/*

*/
impl /*struct*/ QWidget {
  pub fn isWindowModified_0<RetType, T: QWidget_isWindowModified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isWindowModified_0(self);
    // return 1;
  }
}
pub trait QWidget_isWindowModified_0<RetType> {
  fn isWindowModified_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isWindowModified_0<bool> for () {
  fn isWindowModified_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget16isWindowModifiedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:384
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setToolTip(const QString &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setToolTip_0<RetType, T: QWidget_setToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolTip_0(self);
    // return 1;
  }
}
pub trait QWidget_setToolTip_0<RetType> {
  fn setToolTip_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setToolTip_0<(/*void*/)> for (usize) {
  fn setToolTip_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget10setToolTipERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:385
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toolTip() const

/*

*/
impl /*struct*/ QWidget {
  pub fn toolTip_0<RetType, T: QWidget_toolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolTip_0(self);
    // return 1;
  }
}
pub trait QWidget_toolTip_0<RetType> {
  fn toolTip_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_toolTip_0<usize> for () {
  fn toolTip_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget7toolTipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:386
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setToolTipDuration(int)

/*

*/
impl /*struct*/ QWidget {
  pub fn setToolTipDuration_0<RetType, T: QWidget_setToolTipDuration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolTipDuration_0(self);
    // return 1;
  }
}
pub trait QWidget_setToolTipDuration_0<RetType> {
  fn setToolTipDuration_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setToolTipDuration_0<(/*void*/)> for (i32) {
  fn setToolTipDuration_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget18setToolTipDurationEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:387
// index:0
// Public Visibility=Default Availability=Available
// [4] int toolTipDuration() const

/*

*/
impl /*struct*/ QWidget {
  pub fn toolTipDuration_0<RetType, T: QWidget_toolTipDuration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolTipDuration_0(self);
    // return 1;
  }
}
pub trait QWidget_toolTipDuration_0<RetType> {
  fn toolTipDuration_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_toolTipDuration_0<i32> for () {
  fn toolTipDuration_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget15toolTipDurationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:390
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStatusTip(const QString &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setStatusTip_0<RetType, T: QWidget_setStatusTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStatusTip_0(self);
    // return 1;
  }
}
pub trait QWidget_setStatusTip_0<RetType> {
  fn setStatusTip_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setStatusTip_0<(/*void*/)> for (usize) {
  fn setStatusTip_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget12setStatusTipERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:391
// index:0
// Public Visibility=Default Availability=Available
// [8] QString statusTip() const

/*

*/
impl /*struct*/ QWidget {
  pub fn statusTip_0<RetType, T: QWidget_statusTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.statusTip_0(self);
    // return 1;
  }
}
pub trait QWidget_statusTip_0<RetType> {
  fn statusTip_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_statusTip_0<usize> for () {
  fn statusTip_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget9statusTipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:394
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWhatsThis(const QString &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setWhatsThis_0<RetType, T: QWidget_setWhatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis_0(self);
    // return 1;
  }
}
pub trait QWidget_setWhatsThis_0<RetType> {
  fn setWhatsThis_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setWhatsThis_0<(/*void*/)> for (usize) {
  fn setWhatsThis_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget12setWhatsThisERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:395
// index:0
// Public Visibility=Default Availability=Available
// [8] QString whatsThis() const

/*

*/
impl /*struct*/ QWidget {
  pub fn whatsThis_0<RetType, T: QWidget_whatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.whatsThis_0(self);
    // return 1;
  }
}
pub trait QWidget_whatsThis_0<RetType> {
  fn whatsThis_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_whatsThis_0<usize> for () {
  fn whatsThis_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget9whatsThisEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:398
// index:0
// Public Visibility=Default Availability=Available
// [8] QString accessibleName() const

/*

*/
impl /*struct*/ QWidget {
  pub fn accessibleName_0<RetType, T: QWidget_accessibleName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accessibleName_0(self);
    // return 1;
  }
}
pub trait QWidget_accessibleName_0<RetType> {
  fn accessibleName_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_accessibleName_0<usize> for () {
  fn accessibleName_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14accessibleNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:399
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAccessibleName(const QString &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setAccessibleName_0<RetType, T: QWidget_setAccessibleName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAccessibleName_0(self);
    // return 1;
  }
}
pub trait QWidget_setAccessibleName_0<RetType> {
  fn setAccessibleName_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setAccessibleName_0<(/*void*/)> for (usize) {
  fn setAccessibleName_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget17setAccessibleNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:400
// index:0
// Public Visibility=Default Availability=Available
// [8] QString accessibleDescription() const

/*

*/
impl /*struct*/ QWidget {
  pub fn accessibleDescription_0<RetType, T: QWidget_accessibleDescription_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accessibleDescription_0(self);
    // return 1;
  }
}
pub trait QWidget_accessibleDescription_0<RetType> {
  fn accessibleDescription_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_accessibleDescription_0<usize> for () {
  fn accessibleDescription_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget21accessibleDescriptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:401
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAccessibleDescription(const QString &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setAccessibleDescription_0<RetType, T: QWidget_setAccessibleDescription_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAccessibleDescription_0(self);
    // return 1;
  }
}
pub trait QWidget_setAccessibleDescription_0<RetType> {
  fn setAccessibleDescription_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setAccessibleDescription_0<(/*void*/)> for (usize) {
  fn setAccessibleDescription_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget24setAccessibleDescriptionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:404
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLayoutDirection(Qt::LayoutDirection)

/*

*/
impl /*struct*/ QWidget {
  pub fn setLayoutDirection_0<RetType, T: QWidget_setLayoutDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLayoutDirection_0(self);
    // return 1;
  }
}
pub trait QWidget_setLayoutDirection_0<RetType> {
  fn setLayoutDirection_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setLayoutDirection_0<(/*void*/)> for (i32) {
  fn setLayoutDirection_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget18setLayoutDirectionEN2Qt15LayoutDirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:405
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::LayoutDirection layoutDirection() const

/*

*/
impl /*struct*/ QWidget {
  pub fn layoutDirection_0<RetType, T: QWidget_layoutDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layoutDirection_0(self);
    // return 1;
  }
}
pub trait QWidget_layoutDirection_0<RetType> {
  fn layoutDirection_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_layoutDirection_0<i32> for () {
  fn layoutDirection_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget15layoutDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:406
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unsetLayoutDirection()

/*

*/
impl /*struct*/ QWidget {
  pub fn unsetLayoutDirection_0<RetType, T: QWidget_unsetLayoutDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unsetLayoutDirection_0(self);
    // return 1;
  }
}
pub trait QWidget_unsetLayoutDirection_0<RetType> {
  fn unsetLayoutDirection_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_unsetLayoutDirection_0<(/*void*/)> for () {
  fn unsetLayoutDirection_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget20unsetLayoutDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:408
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLocale(const QLocale &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setLocale_0<RetType, T: QWidget_setLocale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLocale_0(self);
    // return 1;
  }
}
pub trait QWidget_setLocale_0<RetType> {
  fn setLocale_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setLocale_0<(/*void*/)> for (usize) {
  fn setLocale_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget9setLocaleERK7QLocale", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:409
// index:0
// Public Visibility=Default Availability=Available
// [8] QLocale locale() const

/*

*/
impl /*struct*/ QWidget {
  pub fn locale_0<RetType, T: QWidget_locale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.locale_0(self);
    // return 1;
  }
}
pub trait QWidget_locale_0<RetType> {
  fn locale_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_locale_0<usize> for () {
  fn locale_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget6localeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:410
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unsetLocale()

/*

*/
impl /*struct*/ QWidget {
  pub fn unsetLocale_0<RetType, T: QWidget_unsetLocale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unsetLocale_0(self);
    // return 1;
  }
}
pub trait QWidget_unsetLocale_0<RetType> {
  fn unsetLocale_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_unsetLocale_0<(/*void*/)> for () {
  fn unsetLocale_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget11unsetLocaleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:412
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isRightToLeft() const

/*

*/
impl /*struct*/ QWidget {
  pub fn isRightToLeft_0<RetType, T: QWidget_isRightToLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRightToLeft_0(self);
    // return 1;
  }
}
pub trait QWidget_isRightToLeft_0<RetType> {
  fn isRightToLeft_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isRightToLeft_0<bool> for () {
  fn isRightToLeft_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget13isRightToLeftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:413
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isLeftToRight() const

/*

*/
impl /*struct*/ QWidget {
  pub fn isLeftToRight_0<RetType, T: QWidget_isLeftToRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLeftToRight_0(self);
    // return 1;
  }
}
pub trait QWidget_isLeftToRight_0<RetType> {
  fn isLeftToRight_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isLeftToRight_0<bool> for () {
  fn isLeftToRight_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget13isLeftToRightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:416
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFocus()

/*
Gives the keyboard input focus to this widget (or its focus proxy) if this widget or one of its parents is the active window. The reason argument will be passed into any focus event sent from this function, it is used to give an explanation of what caused the widget to get focus. If the window is not active, the widget will be given the focus when the window becomes active.

First, a focus about to change event is sent to the focus widget (if any) to tell it that it is about to lose the focus. Then focus is changed, a focus out event is sent to the previous focus item and a focus in event is sent to the new item to tell it that it just received the focus. (Nothing happens if the focus in and focus out widgets are the same.)

Note: On embedded platforms, setFocus() will not cause an input panel to be opened by the input method. If you want this to happen, you have to send a QEvent::RequestSoftwareInputPanel event to the widget yourself.

setFocus() gives focus to a widget regardless of its focus policy, but does not clear any keyboard grab (see grabKeyboard()).

Be aware that if the widget is hidden, it will not accept focus until it is shown.

Warning: If you call setFocus() in a function which may itself be called from focusOutEvent() or focusInEvent(), you may get an infinite recursion.

See also hasFocus(), clearFocus(), focusInEvent(), focusOutEvent(), setFocusPolicy(), focusWidget(), QApplication::focusWidget(), grabKeyboard(), grabMouse(), Keyboard Focus in Widgets, and QEvent::RequestSoftwareInputPanel.
*/
impl /*struct*/ QWidget {
  pub fn setFocus_0<RetType, T: QWidget_setFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFocus_0(self);
    // return 1;
  }
}
pub trait QWidget_setFocus_0<RetType> {
  fn setFocus_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setFocus_0<(/*void*/)> for () {
  fn setFocus_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget8setFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:423
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setFocus(Qt::FocusReason)

/*
Gives the keyboard input focus to this widget (or its focus proxy) if this widget or one of its parents is the active window. The reason argument will be passed into any focus event sent from this function, it is used to give an explanation of what caused the widget to get focus. If the window is not active, the widget will be given the focus when the window becomes active.

First, a focus about to change event is sent to the focus widget (if any) to tell it that it is about to lose the focus. Then focus is changed, a focus out event is sent to the previous focus item and a focus in event is sent to the new item to tell it that it just received the focus. (Nothing happens if the focus in and focus out widgets are the same.)

Note: On embedded platforms, setFocus() will not cause an input panel to be opened by the input method. If you want this to happen, you have to send a QEvent::RequestSoftwareInputPanel event to the widget yourself.

setFocus() gives focus to a widget regardless of its focus policy, but does not clear any keyboard grab (see grabKeyboard()).

Be aware that if the widget is hidden, it will not accept focus until it is shown.

Warning: If you call setFocus() in a function which may itself be called from focusOutEvent() or focusInEvent(), you may get an infinite recursion.

See also hasFocus(), clearFocus(), focusInEvent(), focusOutEvent(), setFocusPolicy(), focusWidget(), QApplication::focusWidget(), grabKeyboard(), grabMouse(), Keyboard Focus in Widgets, and QEvent::RequestSoftwareInputPanel.
*/
impl /*struct*/ QWidget {
  pub fn setFocus_1<RetType, T: QWidget_setFocus_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFocus_1(self);
    // return 1;
  }
}
pub trait QWidget_setFocus_1<RetType> {
  fn setFocus_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setFocus_1<(/*void*/)> for (i32) {
  fn setFocus_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget8setFocusEN2Qt11FocusReasonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:419
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isActiveWindow() const

/*

*/
impl /*struct*/ QWidget {
  pub fn isActiveWindow_0<RetType, T: QWidget_isActiveWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isActiveWindow_0(self);
    // return 1;
  }
}
pub trait QWidget_isActiveWindow_0<RetType> {
  fn isActiveWindow_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isActiveWindow_0<bool> for () {
  fn isActiveWindow_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14isActiveWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:420
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activateWindow()

/*
Sets the top-level widget containing this widget to be the active window.

An active window is a visible top-level window that has the keyboard input focus.

This function performs the same operation as clicking the mouse on the title bar of a top-level window. On X11, the result depends on the Window Manager. If you want to ensure that the window is stacked on top as well you should also call raise(). Note that the window must be visible, otherwise activateWindow() has no effect.

On Windows, if you are calling this when the application is not currently the active one then it will not make it the active window. It will change the color of the taskbar entry to indicate that the window has changed in some way. This is because Microsoft does not allow an application to interrupt what the user is currently doing in another application.

See also isActiveWindow(), window(), show(), and QWindowsWindowFunctions::setWindowActivationBehavior().
*/
impl /*struct*/ QWidget {
  pub fn activateWindow_0<RetType, T: QWidget_activateWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activateWindow_0(self);
    // return 1;
  }
}
pub trait QWidget_activateWindow_0<RetType> {
  fn activateWindow_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_activateWindow_0<(/*void*/)> for () {
  fn activateWindow_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget14activateWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:421
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearFocus()

/*
Takes keyboard input focus from the widget.

If the widget has active focus, a focus out event is sent to this widget to tell it that it has lost the focus.

This widget must enable focus setting in order to get the keyboard input focus, i.e. it must call setFocusPolicy().

See also hasFocus(), setFocus(), focusInEvent(), focusOutEvent(), setFocusPolicy(), and QApplication::focusWidget().
*/
impl /*struct*/ QWidget {
  pub fn clearFocus_0<RetType, T: QWidget_clearFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearFocus_0(self);
    // return 1;
  }
}
pub trait QWidget_clearFocus_0<RetType> {
  fn clearFocus_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_clearFocus_0<(/*void*/)> for () {
  fn clearFocus_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget10clearFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:424
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::FocusPolicy focusPolicy() const

/*

*/
impl /*struct*/ QWidget {
  pub fn focusPolicy_0<RetType, T: QWidget_focusPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusPolicy_0(self);
    // return 1;
  }
}
pub trait QWidget_focusPolicy_0<RetType> {
  fn focusPolicy_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_focusPolicy_0<i32> for () {
  fn focusPolicy_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11focusPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:425
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFocusPolicy(Qt::FocusPolicy)

/*

*/
impl /*struct*/ QWidget {
  pub fn setFocusPolicy_0<RetType, T: QWidget_setFocusPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFocusPolicy_0(self);
    // return 1;
  }
}
pub trait QWidget_setFocusPolicy_0<RetType> {
  fn setFocusPolicy_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setFocusPolicy_0<(/*void*/)> for (i32) {
  fn setFocusPolicy_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget14setFocusPolicyEN2Qt11FocusPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:426
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasFocus() const

/*

*/
impl /*struct*/ QWidget {
  pub fn hasFocus_0<RetType, T: QWidget_hasFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasFocus_0(self);
    // return 1;
  }
}
pub trait QWidget_hasFocus_0<RetType> {
  fn hasFocus_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_hasFocus_0<bool> for () {
  fn hasFocus_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget8hasFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:427
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setTabOrder(QWidget *, QWidget *)

/*
Puts the second widget after the first widget in the focus order.

It effectively removes the second widget from its focus chain and inserts it after the first widget.

Note that since the tab order of the second widget is changed, you should order a chain like this:


  setTabOrder(a, b); // a to b
  setTabOrder(b, c); // a to b to c
  setTabOrder(c, d); // a to b to c to d



not like this:


  // WRONG
  setTabOrder(c, d); // c to d
  setTabOrder(a, b); // a to b AND c to d
  setTabOrder(b, c); // a to b to c, but not c to d



If first or second has a focus proxy, setTabOrder() correctly substitutes the proxy.

Note: Since Qt 5.10: A widget that has a child as focus proxy is understood as a compound widget. When setting a tab order between one or two compound widgets, the local tab order inside each will be preserved. This means that if both widgets are compound widgets, the resulting tab order will be from the last child inside first, to the first child inside second.

See also setFocusPolicy(), setFocusProxy(), and Keyboard Focus in Widgets.
*/
impl /*struct*/ QWidget {
  pub fn setTabOrder_0<RetType, T: QWidget_setTabOrder_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setTabOrder_0();
    // return 1;
  }
}
pub trait QWidget_setTabOrder_0<RetType> {
  fn setTabOrder_0(self ) -> RetType;
}
impl<'a> /*trait*/ QWidget_setTabOrder_0<(/*void*/)> for (usize,usize) {
  fn setTabOrder_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget11setTabOrderEPS_S0_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:428
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFocusProxy(QWidget *)

/*
Sets the widget's focus proxy to widget w. If w is 0, the function resets this widget to have no focus proxy.

Some widgets can "have focus", but create a child widget, such as QLineEdit, to actually handle the focus. In this case, the widget can set the line edit to be its focus proxy.

setFocusProxy() sets the widget which will actually get focus when "this widget" gets it. If there is a focus proxy, setFocus() and hasFocus() operate on the focus proxy.

See also focusProxy().
*/
impl /*struct*/ QWidget {
  pub fn setFocusProxy_0<RetType, T: QWidget_setFocusProxy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFocusProxy_0(self);
    // return 1;
  }
}
pub trait QWidget_setFocusProxy_0<RetType> {
  fn setFocusProxy_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setFocusProxy_0<(/*void*/)> for (usize) {
  fn setFocusProxy_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget13setFocusProxyEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:429
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * focusProxy() const

/*
Returns the focus proxy, or 0 if there is no focus proxy.

See also setFocusProxy().
*/
impl /*struct*/ QWidget {
  pub fn focusProxy_0<RetType, T: QWidget_focusProxy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusProxy_0(self);
    // return 1;
  }
}
pub trait QWidget_focusProxy_0<RetType> {
  fn focusProxy_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_focusProxy_0<usize> for () {
  fn focusProxy_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget10focusProxyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:430
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ContextMenuPolicy contextMenuPolicy() const

/*

*/
impl /*struct*/ QWidget {
  pub fn contextMenuPolicy_0<RetType, T: QWidget_contextMenuPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuPolicy_0(self);
    // return 1;
  }
}
pub trait QWidget_contextMenuPolicy_0<RetType> {
  fn contextMenuPolicy_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_contextMenuPolicy_0<i32> for () {
  fn contextMenuPolicy_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget17contextMenuPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:431
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setContextMenuPolicy(Qt::ContextMenuPolicy)

/*

*/
impl /*struct*/ QWidget {
  pub fn setContextMenuPolicy_0<RetType, T: QWidget_setContextMenuPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setContextMenuPolicy_0(self);
    // return 1;
  }
}
pub trait QWidget_setContextMenuPolicy_0<RetType> {
  fn setContextMenuPolicy_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setContextMenuPolicy_0<(/*void*/)> for (i32) {
  fn setContextMenuPolicy_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget20setContextMenuPolicyEN2Qt17ContextMenuPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:434
// index:0
// Public Visibility=Default Availability=Available
// [-2] void grabMouse()

/*
Grabs the mouse input.

This widget receives all mouse events until releaseMouse() is called; other widgets get no mouse events at all. Keyboard events are not affected. Use grabKeyboard() if you want to grab that.

Warning: Bugs in mouse-grabbing applications very often lock the terminal. Use this function with extreme caution, and consider using the -nograb command line option while debugging.

It is almost never necessary to grab the mouse when using Qt, as Qt grabs and releases it sensibly. In particular, Qt grabs the mouse when a mouse button is pressed and keeps it until the last button is released.

Note: Only visible widgets can grab mouse input. If isVisible() returns false for a widget, that widget cannot call grabMouse().

Note: On Windows, grabMouse() only works when the mouse is inside a window owned by the process. On macOS, grabMouse() only works when the mouse is inside the frame of that widget.

See also releaseMouse(), grabKeyboard(), and releaseKeyboard().
*/
impl /*struct*/ QWidget {
  pub fn grabMouse_0<RetType, T: QWidget_grabMouse_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.grabMouse_0(self);
    // return 1;
  }
}
pub trait QWidget_grabMouse_0<RetType> {
  fn grabMouse_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_grabMouse_0<(/*void*/)> for () {
  fn grabMouse_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget9grabMouseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:436
// index:1
// Public Visibility=Default Availability=Available
// [-2] void grabMouse(const QCursor &)

/*
Grabs the mouse input.

This widget receives all mouse events until releaseMouse() is called; other widgets get no mouse events at all. Keyboard events are not affected. Use grabKeyboard() if you want to grab that.

Warning: Bugs in mouse-grabbing applications very often lock the terminal. Use this function with extreme caution, and consider using the -nograb command line option while debugging.

It is almost never necessary to grab the mouse when using Qt, as Qt grabs and releases it sensibly. In particular, Qt grabs the mouse when a mouse button is pressed and keeps it until the last button is released.

Note: Only visible widgets can grab mouse input. If isVisible() returns false for a widget, that widget cannot call grabMouse().

Note: On Windows, grabMouse() only works when the mouse is inside a window owned by the process. On macOS, grabMouse() only works when the mouse is inside the frame of that widget.

See also releaseMouse(), grabKeyboard(), and releaseKeyboard().
*/
impl /*struct*/ QWidget {
  pub fn grabMouse_1<RetType, T: QWidget_grabMouse_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.grabMouse_1(self);
    // return 1;
  }
}
pub trait QWidget_grabMouse_1<RetType> {
  fn grabMouse_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_grabMouse_1<(/*void*/)> for (usize) {
  fn grabMouse_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget9grabMouseERK7QCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:438
// index:0
// Public Visibility=Default Availability=Available
// [-2] void releaseMouse()

/*
Releases the mouse grab.

See also grabMouse(), grabKeyboard(), and releaseKeyboard().
*/
impl /*struct*/ QWidget {
  pub fn releaseMouse_0<RetType, T: QWidget_releaseMouse_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.releaseMouse_0(self);
    // return 1;
  }
}
pub trait QWidget_releaseMouse_0<RetType> {
  fn releaseMouse_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_releaseMouse_0<(/*void*/)> for () {
  fn releaseMouse_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget12releaseMouseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:439
// index:0
// Public Visibility=Default Availability=Available
// [-2] void grabKeyboard()

/*
Grabs the keyboard input.

This widget receives all keyboard events until releaseKeyboard() is called; other widgets get no keyboard events at all. Mouse events are not affected. Use grabMouse() if you want to grab that.

The focus widget is not affected, except that it doesn't receive any keyboard events. setFocus() moves the focus as usual, but the new focus widget receives keyboard events only after releaseKeyboard() is called.

If a different widget is currently grabbing keyboard input, that widget's grab is released first.

See also releaseKeyboard(), grabMouse(), releaseMouse(), and focusWidget().
*/
impl /*struct*/ QWidget {
  pub fn grabKeyboard_0<RetType, T: QWidget_grabKeyboard_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.grabKeyboard_0(self);
    // return 1;
  }
}
pub trait QWidget_grabKeyboard_0<RetType> {
  fn grabKeyboard_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_grabKeyboard_0<(/*void*/)> for () {
  fn grabKeyboard_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget12grabKeyboardEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:440
// index:0
// Public Visibility=Default Availability=Available
// [-2] void releaseKeyboard()

/*
Releases the keyboard grab.

See also grabKeyboard(), grabMouse(), and releaseMouse().
*/
impl /*struct*/ QWidget {
  pub fn releaseKeyboard_0<RetType, T: QWidget_releaseKeyboard_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.releaseKeyboard_0(self);
    // return 1;
  }
}
pub trait QWidget_releaseKeyboard_0<RetType> {
  fn releaseKeyboard_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_releaseKeyboard_0<(/*void*/)> for () {
  fn releaseKeyboard_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget15releaseKeyboardEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:442
// index:0
// Public Visibility=Default Availability=Available
// [4] int grabShortcut(const QKeySequence &, Qt::ShortcutContext)

/*
Adds a shortcut to Qt's shortcut system that watches for the given key sequence in the given context. If the context is Qt::ApplicationShortcut, the shortcut applies to the application as a whole. Otherwise, it is either local to this widget, Qt::WidgetShortcut, or to the window itself, Qt::WindowShortcut.

If the same key sequence has been grabbed by several widgets, when the key sequence occurs a QEvent::Shortcut event is sent to all the widgets to which it applies in a non-deterministic order, but with the ``ambiguous'' flag set to true.

Warning: You should not normally need to use this function; instead create QActions with the shortcut key sequences you require (if you also want equivalent menu options and toolbar buttons), or create QShortcuts if you just need key sequences. Both QAction and QShortcut handle all the event filtering for you, and provide signals which are triggered when the user triggers the key sequence, so are much easier to use than this low-level function.

See also releaseShortcut() and setShortcutEnabled().
*/
impl /*struct*/ QWidget {
  pub fn grabShortcut_0<RetType, T: QWidget_grabShortcut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.grabShortcut_0(self);
    // return 1;
  }
}
pub trait QWidget_grabShortcut_0<RetType> {
  fn grabShortcut_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_grabShortcut_0<i32> for (usize,i32) {
  fn grabShortcut_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWidget12grabShortcutERK12QKeySequenceN2Qt15ShortcutContextE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:443
// index:0
// Public Visibility=Default Availability=Available
// [-2] void releaseShortcut(int)

/*
Removes the shortcut with the given id from Qt's shortcut system. The widget will no longer receive QEvent::Shortcut events for the shortcut's key sequence (unless it has other shortcuts with the same key sequence).

Warning: You should not normally need to use this function since Qt's shortcut system removes shortcuts automatically when their parent widget is destroyed. It is best to use QAction or QShortcut to handle shortcuts, since they are easier to use than this low-level function. Note also that this is an expensive operation.

See also grabShortcut() and setShortcutEnabled().
*/
impl /*struct*/ QWidget {
  pub fn releaseShortcut_0<RetType, T: QWidget_releaseShortcut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.releaseShortcut_0(self);
    // return 1;
  }
}
pub trait QWidget_releaseShortcut_0<RetType> {
  fn releaseShortcut_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_releaseShortcut_0<(/*void*/)> for (i32) {
  fn releaseShortcut_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget15releaseShortcutEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:444
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setShortcutEnabled(int, bool)

/*
If enable is true, the shortcut with the given id is enabled; otherwise the shortcut is disabled.

Warning: You should not normally need to use this function since Qt's shortcut system enables/disables shortcuts automatically as widgets become hidden/visible and gain or lose focus. It is best to use QAction or QShortcut to handle shortcuts, since they are easier to use than this low-level function.

See also grabShortcut() and releaseShortcut().
*/
impl /*struct*/ QWidget {
  pub fn setShortcutEnabled_0<RetType, T: QWidget_setShortcutEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setShortcutEnabled_0(self);
    // return 1;
  }
}
pub trait QWidget_setShortcutEnabled_0<RetType> {
  fn setShortcutEnabled_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setShortcutEnabled_0<(/*void*/)> for (i32,bool) {
  fn setShortcutEnabled_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget18setShortcutEnabledEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:445
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setShortcutAutoRepeat(int, bool)

/*
If enable is true, auto repeat of the shortcut with the given id is enabled; otherwise it is disabled.

This function was introduced in  Qt 4.2.

See also grabShortcut() and releaseShortcut().
*/
impl /*struct*/ QWidget {
  pub fn setShortcutAutoRepeat_0<RetType, T: QWidget_setShortcutAutoRepeat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setShortcutAutoRepeat_0(self);
    // return 1;
  }
}
pub trait QWidget_setShortcutAutoRepeat_0<RetType> {
  fn setShortcutAutoRepeat_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setShortcutAutoRepeat_0<(/*void*/)> for (i32,bool) {
  fn setShortcutAutoRepeat_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget21setShortcutAutoRepeatEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:447
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWidget * mouseGrabber()

/*
Returns the widget that is currently grabbing the mouse input.

If no widget in this application is currently grabbing the mouse, 0 is returned.

See also grabMouse() and keyboardGrabber().
*/
impl /*struct*/ QWidget {
  pub fn mouseGrabber_0<RetType, T: QWidget_mouseGrabber_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.mouseGrabber_0();
    // return 1;
  }
}
pub trait QWidget_mouseGrabber_0<RetType> {
  fn mouseGrabber_0(self ) -> RetType;
}
impl<'a> /*trait*/ QWidget_mouseGrabber_0<usize> for () {
  fn mouseGrabber_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWidget12mouseGrabberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:448
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWidget * keyboardGrabber()

/*
Returns the widget that is currently grabbing the keyboard input.

If no widget in this application is currently grabbing the keyboard, 0 is returned.

See also grabMouse() and mouseGrabber().
*/
impl /*struct*/ QWidget {
  pub fn keyboardGrabber_0<RetType, T: QWidget_keyboardGrabber_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.keyboardGrabber_0();
    // return 1;
  }
}
pub trait QWidget_keyboardGrabber_0<RetType> {
  fn keyboardGrabber_0(self ) -> RetType;
}
impl<'a> /*trait*/ QWidget_keyboardGrabber_0<usize> for () {
  fn keyboardGrabber_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWidget15keyboardGrabberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:451
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool updatesEnabled() const

/*

*/
impl /*struct*/ QWidget {
  pub fn updatesEnabled_0<RetType, T: QWidget_updatesEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updatesEnabled_0(self);
    // return 1;
  }
}
pub trait QWidget_updatesEnabled_0<RetType> {
  fn updatesEnabled_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_updatesEnabled_0<bool> for () {
  fn updatesEnabled_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14updatesEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:452
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUpdatesEnabled(bool)

/*

*/
impl /*struct*/ QWidget {
  pub fn setUpdatesEnabled_0<RetType, T: QWidget_setUpdatesEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUpdatesEnabled_0(self);
    // return 1;
  }
}
pub trait QWidget_setUpdatesEnabled_0<RetType> {
  fn setUpdatesEnabled_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setUpdatesEnabled_0<(/*void*/)> for (bool) {
  fn setUpdatesEnabled_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget17setUpdatesEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:455
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsProxyWidget * graphicsProxyWidget() const

/*
Returns the proxy widget for the corresponding embedded widget in a graphics view; otherwise returns 0.

This function was introduced in  Qt 4.5.

See also QGraphicsProxyWidget::createProxyForChildWidget() and QGraphicsScene::addWidget().
*/
impl /*struct*/ QWidget {
  pub fn graphicsProxyWidget_0<RetType, T: QWidget_graphicsProxyWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.graphicsProxyWidget_0(self);
    // return 1;
  }
}
pub trait QWidget_graphicsProxyWidget_0<RetType> {
  fn graphicsProxyWidget_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_graphicsProxyWidget_0<usize> for () {
  fn graphicsProxyWidget_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget19graphicsProxyWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:459
// index:0
// Public Visibility=Default Availability=Available
// [-2] void update()

/*
Updates the widget unless updates are disabled or the widget is hidden.

This function does not cause an immediate repaint; instead it schedules a paint event for processing when Qt returns to the main event loop. This permits Qt to optimize for more speed and less flicker than a call to repaint() does.

Calling update() several times normally results in just one paintEvent() call.

Qt normally erases the widget's area before the paintEvent() call. If the Qt::WA_OpaquePaintEvent widget attribute is set, the widget is responsible for painting all its pixels with an opaque color.

See also repaint(), paintEvent(), setUpdatesEnabled(), and Analog Clock Example.
*/
impl /*struct*/ QWidget {
  pub fn update_0<RetType, T: QWidget_update_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_0(self);
    // return 1;
  }
}
pub trait QWidget_update_0<RetType> {
  fn update_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_update_0<(/*void*/)> for () {
  fn update_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget6updateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:463
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void update(int, int, int, int)

/*
Updates the widget unless updates are disabled or the widget is hidden.

This function does not cause an immediate repaint; instead it schedules a paint event for processing when Qt returns to the main event loop. This permits Qt to optimize for more speed and less flicker than a call to repaint() does.

Calling update() several times normally results in just one paintEvent() call.

Qt normally erases the widget's area before the paintEvent() call. If the Qt::WA_OpaquePaintEvent widget attribute is set, the widget is responsible for painting all its pixels with an opaque color.

See also repaint(), paintEvent(), setUpdatesEnabled(), and Analog Clock Example.
*/
impl /*struct*/ QWidget {
  pub fn update_1<RetType, T: QWidget_update_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_1(self);
    // return 1;
  }
}
pub trait QWidget_update_1<RetType> {
  fn update_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_update_1<(/*void*/)> for (i32,i32,i32,i32) {
  fn update_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget6updateEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:464
// index:2
// Public Visibility=Default Availability=Available
// [-2] void update(const QRect &)

/*
Updates the widget unless updates are disabled or the widget is hidden.

This function does not cause an immediate repaint; instead it schedules a paint event for processing when Qt returns to the main event loop. This permits Qt to optimize for more speed and less flicker than a call to repaint() does.

Calling update() several times normally results in just one paintEvent() call.

Qt normally erases the widget's area before the paintEvent() call. If the Qt::WA_OpaquePaintEvent widget attribute is set, the widget is responsible for painting all its pixels with an opaque color.

See also repaint(), paintEvent(), setUpdatesEnabled(), and Analog Clock Example.
*/
impl /*struct*/ QWidget {
  pub fn update_2<RetType, T: QWidget_update_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_2(self);
    // return 1;
  }
}
pub trait QWidget_update_2<RetType> {
  fn update_2(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_update_2<(/*void*/)> for (usize) {
  fn update_2(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget6updateERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:465
// index:3
// Public Visibility=Default Availability=Available
// [-2] void update(const QRegion &)

/*
Updates the widget unless updates are disabled or the widget is hidden.

This function does not cause an immediate repaint; instead it schedules a paint event for processing when Qt returns to the main event loop. This permits Qt to optimize for more speed and less flicker than a call to repaint() does.

Calling update() several times normally results in just one paintEvent() call.

Qt normally erases the widget's area before the paintEvent() call. If the Qt::WA_OpaquePaintEvent widget attribute is set, the widget is responsible for painting all its pixels with an opaque color.

See also repaint(), paintEvent(), setUpdatesEnabled(), and Analog Clock Example.
*/
impl /*struct*/ QWidget {
  pub fn update_3<RetType, T: QWidget_update_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_3(self);
    // return 1;
  }
}
pub trait QWidget_update_3<RetType> {
  fn update_3(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_update_3<(/*void*/)> for (usize) {
  fn update_3(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget6updateERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:460
// index:0
// Public Visibility=Default Availability=Available
// [-2] void repaint()

/*
Repaints the widget directly by calling paintEvent() immediately, unless updates are disabled or the widget is hidden.

We suggest only using repaint() if you need an immediate repaint, for example during animation. In almost all circumstances update() is better, as it permits Qt to optimize for speed and minimize flicker.

Warning: If you call repaint() in a function which may itself be called from paintEvent(), you may get infinite recursion. The update() function never causes recursion.

See also update(), paintEvent(), and setUpdatesEnabled().
*/
impl /*struct*/ QWidget {
  pub fn repaint_0<RetType, T: QWidget_repaint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.repaint_0(self);
    // return 1;
  }
}
pub trait QWidget_repaint_0<RetType> {
  fn repaint_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_repaint_0<(/*void*/)> for () {
  fn repaint_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget7repaintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:467
// index:1
// Public Visibility=Default Availability=Available
// [-2] void repaint(int, int, int, int)

/*
Repaints the widget directly by calling paintEvent() immediately, unless updates are disabled or the widget is hidden.

We suggest only using repaint() if you need an immediate repaint, for example during animation. In almost all circumstances update() is better, as it permits Qt to optimize for speed and minimize flicker.

Warning: If you call repaint() in a function which may itself be called from paintEvent(), you may get infinite recursion. The update() function never causes recursion.

See also update(), paintEvent(), and setUpdatesEnabled().
*/
impl /*struct*/ QWidget {
  pub fn repaint_1<RetType, T: QWidget_repaint_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.repaint_1(self);
    // return 1;
  }
}
pub trait QWidget_repaint_1<RetType> {
  fn repaint_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_repaint_1<(/*void*/)> for (i32,i32,i32,i32) {
  fn repaint_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget7repaintEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:468
// index:2
// Public Visibility=Default Availability=Available
// [-2] void repaint(const QRect &)

/*
Repaints the widget directly by calling paintEvent() immediately, unless updates are disabled or the widget is hidden.

We suggest only using repaint() if you need an immediate repaint, for example during animation. In almost all circumstances update() is better, as it permits Qt to optimize for speed and minimize flicker.

Warning: If you call repaint() in a function which may itself be called from paintEvent(), you may get infinite recursion. The update() function never causes recursion.

See also update(), paintEvent(), and setUpdatesEnabled().
*/
impl /*struct*/ QWidget {
  pub fn repaint_2<RetType, T: QWidget_repaint_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.repaint_2(self);
    // return 1;
  }
}
pub trait QWidget_repaint_2<RetType> {
  fn repaint_2(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_repaint_2<(/*void*/)> for (usize) {
  fn repaint_2(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget7repaintERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:469
// index:3
// Public Visibility=Default Availability=Available
// [-2] void repaint(const QRegion &)

/*
Repaints the widget directly by calling paintEvent() immediately, unless updates are disabled or the widget is hidden.

We suggest only using repaint() if you need an immediate repaint, for example during animation. In almost all circumstances update() is better, as it permits Qt to optimize for speed and minimize flicker.

Warning: If you call repaint() in a function which may itself be called from paintEvent(), you may get infinite recursion. The update() function never causes recursion.

See also update(), paintEvent(), and setUpdatesEnabled().
*/
impl /*struct*/ QWidget {
  pub fn repaint_3<RetType, T: QWidget_repaint_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.repaint_3(self);
    // return 1;
  }
}
pub trait QWidget_repaint_3<RetType> {
  fn repaint_3(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_repaint_3<(/*void*/)> for (usize) {
  fn repaint_3(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget7repaintERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:474
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*

*/
impl /*struct*/ QWidget {
  pub fn setVisible_0<RetType, T: QWidget_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QWidget_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:475
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHidden(bool)

/*
Convenience function, equivalent to setVisible(!hidden).

See also isHidden().
*/
impl /*struct*/ QWidget {
  pub fn setHidden_0<RetType, T: QWidget_setHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHidden_0(self);
    // return 1;
  }
}
pub trait QWidget_setHidden_0<RetType> {
  fn setHidden_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setHidden_0<(/*void*/)> for (bool) {
  fn setHidden_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget9setHiddenEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:476
// index:0
// Public Visibility=Default Availability=Available
// [-2] void show()

/*
Shows the widget and its child widgets.

This is equivalent to calling showFullScreen(), showMaximized(), or setVisible(true), depending on the platform's default behavior for the window flags.

See also raise(), showEvent(), hide(), setVisible(), showMinimized(), showMaximized(), showNormal(), isVisible(), and windowFlags().
*/
impl /*struct*/ QWidget {
  pub fn show_0<RetType, T: QWidget_show_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.show_0(self);
    // return 1;
  }
}
pub trait QWidget_show_0<RetType> {
  fn show_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_show_0<(/*void*/)> for () {
  fn show_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget4showEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:477
// index:0
// Public Visibility=Default Availability=Available
// [-2] void hide()

/*
Hides the widget. This function is equivalent to setVisible(false).

Note: If you are working with QDialog or its subclasses and you invoke the show() function after this function, the dialog will be displayed in its original position.

See also hideEvent(), isHidden(), show(), setVisible(), isVisible(), and close().
*/
impl /*struct*/ QWidget {
  pub fn hide_0<RetType, T: QWidget_hide_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hide_0(self);
    // return 1;
  }
}
pub trait QWidget_hide_0<RetType> {
  fn hide_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_hide_0<(/*void*/)> for () {
  fn hide_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget4hideEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:479
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showMinimized()

/*
Shows the widget minimized, as an icon.

Calling this function only affects windows.

See also showNormal(), showMaximized(), show(), hide(), isVisible(), and isMinimized().
*/
impl /*struct*/ QWidget {
  pub fn showMinimized_0<RetType, T: QWidget_showMinimized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showMinimized_0(self);
    // return 1;
  }
}
pub trait QWidget_showMinimized_0<RetType> {
  fn showMinimized_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_showMinimized_0<(/*void*/)> for () {
  fn showMinimized_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget13showMinimizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:480
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showMaximized()

/*
Shows the widget maximized.

Calling this function only affects windows.

On X11, this function may not work properly with certain window managers. See the Window Geometry documentation for an explanation.

See also setWindowState(), showNormal(), showMinimized(), show(), hide(), and isVisible().
*/
impl /*struct*/ QWidget {
  pub fn showMaximized_0<RetType, T: QWidget_showMaximized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showMaximized_0(self);
    // return 1;
  }
}
pub trait QWidget_showMaximized_0<RetType> {
  fn showMaximized_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_showMaximized_0<(/*void*/)> for () {
  fn showMaximized_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget13showMaximizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:481
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showFullScreen()

/*
Shows the widget in full-screen mode.

Calling this function only affects windows.

To return from full-screen mode, call showNormal().

Full-screen mode works fine under Windows, but has certain problems under X. These problems are due to limitations of the ICCCM protocol that specifies the communication between X11 clients and the window manager. ICCCM simply does not understand the concept of non-decorated full-screen windows. Therefore, the best we can do is to request a borderless window and place and resize it to fill the entire screen. Depending on the window manager, this may or may not work. The borderless window is requested using MOTIF hints, which are at least partially supported by virtually all modern window managers.

An alternative would be to bypass the window manager entirely and create a window with the Qt::X11BypassWindowManagerHint flag. This has other severe problems though, like totally broken keyboard focus and very strange effects on desktop changes or when the user raises other windows.

X11 window managers that follow modern post-ICCCM specifications support full-screen mode properly.

See also showNormal(), showMaximized(), show(), hide(), and isVisible().
*/
impl /*struct*/ QWidget {
  pub fn showFullScreen_0<RetType, T: QWidget_showFullScreen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showFullScreen_0(self);
    // return 1;
  }
}
pub trait QWidget_showFullScreen_0<RetType> {
  fn showFullScreen_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_showFullScreen_0<(/*void*/)> for () {
  fn showFullScreen_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget14showFullScreenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:482
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showNormal()

/*
Restores the widget after it has been maximized or minimized.

Calling this function only affects windows.

See also setWindowState(), showMinimized(), showMaximized(), show(), hide(), and isVisible().
*/
impl /*struct*/ QWidget {
  pub fn showNormal_0<RetType, T: QWidget_showNormal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showNormal_0(self);
    // return 1;
  }
}
pub trait QWidget_showNormal_0<RetType> {
  fn showNormal_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_showNormal_0<(/*void*/)> for () {
  fn showNormal_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget10showNormalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:484
// index:0
// Public Visibility=Default Availability=Available
// [1] bool close()

/*
Closes this widget. Returns true if the widget was closed; otherwise returns false.

First it sends the widget a QCloseEvent. The widget is hidden if it accepts the close event. If it ignores the event, nothing happens. The default implementation of QWidget::closeEvent() accepts the close event.

If the widget has the Qt::WA_DeleteOnClose flag, the widget is also deleted. A close events is delivered to the widget no matter if the widget is visible or not.

The QApplication::lastWindowClosed() signal is emitted when the last visible primary window (i.e. window with no parent) with the Qt::WA_QuitOnClose attribute set is closed. By default this attribute is set for all widgets except transient windows such as splash screens, tool windows, and popup menus.
*/
impl /*struct*/ QWidget {
  pub fn close_0<RetType, T: QWidget_close_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.close_0(self);
    // return 1;
  }
}
pub trait QWidget_close_0<RetType> {
  fn close_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_close_0<bool> for () {
  fn close_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWidget5closeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:485
// index:0
// Public Visibility=Default Availability=Available
// [-2] void raise()

/*
Raises this widget to the top of the parent widget's stack.

After this call the widget will be visually in front of any overlapping sibling widgets.

Note: When using activateWindow(), you can call this function to ensure that the window is stacked on top.

See also lower() and stackUnder().
*/
impl /*struct*/ QWidget {
  pub fn raise_0<RetType, T: QWidget_raise_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.raise_0(self);
    // return 1;
  }
}
pub trait QWidget_raise_0<RetType> {
  fn raise_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_raise_0<(/*void*/)> for () {
  fn raise_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget5raiseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:486
// index:0
// Public Visibility=Default Availability=Available
// [-2] void lower()

/*
Lowers the widget to the bottom of the parent widget's stack.

After this call the widget will be visually behind (and therefore obscured by) any overlapping sibling widgets.

See also raise() and stackUnder().
*/
impl /*struct*/ QWidget {
  pub fn lower_0<RetType, T: QWidget_lower_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lower_0(self);
    // return 1;
  }
}
pub trait QWidget_lower_0<RetType> {
  fn lower_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_lower_0<(/*void*/)> for () {
  fn lower_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget5lowerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:489
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stackUnder(QWidget *)

/*
Places the widget under w in the parent widget's stack.

To make this work, the widget itself and w must be siblings.

See also raise() and lower().
*/
impl /*struct*/ QWidget {
  pub fn stackUnder_0<RetType, T: QWidget_stackUnder_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stackUnder_0(self);
    // return 1;
  }
}
pub trait QWidget_stackUnder_0<RetType> {
  fn stackUnder_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_stackUnder_0<(/*void*/)> for (usize) {
  fn stackUnder_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget10stackUnderEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:490
// index:0
// Public Visibility=Default Availability=Available
// [-2] void move(int, int)

/*

*/
impl /*struct*/ QWidget {
  pub fn move__0<RetType, T: QWidget_move__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.move__0(self);
    // return 1;
  }
}
pub trait QWidget_move__0<RetType> {
  fn move__0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_move__0<(/*void*/)> for (i32,i32) {
  fn move__0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget4moveEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:491
// index:1
// Public Visibility=Default Availability=Available
// [-2] void move(const QPoint &)

/*

*/
impl /*struct*/ QWidget {
  pub fn move__1<RetType, T: QWidget_move__1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.move__1(self);
    // return 1;
  }
}
pub trait QWidget_move__1<RetType> {
  fn move__1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_move__1<(/*void*/)> for (usize) {
  fn move__1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget4moveERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:492
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resize(int, int)

/*

*/
impl /*struct*/ QWidget {
  pub fn resize_0<RetType, T: QWidget_resize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_0(self);
    // return 1;
  }
}
pub trait QWidget_resize_0<RetType> {
  fn resize_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_resize_0<(/*void*/)> for (i32,i32) {
  fn resize_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget6resizeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:493
// index:1
// Public Visibility=Default Availability=Available
// [-2] void resize(const QSize &)

/*

*/
impl /*struct*/ QWidget {
  pub fn resize_1<RetType, T: QWidget_resize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_1(self);
    // return 1;
  }
}
pub trait QWidget_resize_1<RetType> {
  fn resize_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_resize_1<(/*void*/)> for (usize) {
  fn resize_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget6resizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:494
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setGeometry(int, int, int, int)

/*

*/
impl /*struct*/ QWidget {
  pub fn setGeometry_0<RetType, T: QWidget_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QWidget_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setGeometry_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn setGeometry_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget11setGeometryEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:495
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setGeometry(const QRect &)

/*

*/
impl /*struct*/ QWidget {
  pub fn setGeometry_1<RetType, T: QWidget_setGeometry_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_1(self);
    // return 1;
  }
}
pub trait QWidget_setGeometry_1<RetType> {
  fn setGeometry_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setGeometry_1<(/*void*/)> for (usize) {
  fn setGeometry_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget11setGeometryERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:496
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray saveGeometry() const

/*
Saves the current geometry and state for top-level widgets.

To save the geometry when the window closes, you can implement a close event like this:


  void MyWidget::closeEvent(QCloseEvent *event)
  {
      QSettings settings("MyCompany", "MyApp");
      settings.setValue("geometry", saveGeometry());
      QWidget::closeEvent(event);
  }



See the Window Geometry documentation for an overview of geometry issues with windows.

Use QMainWindow::saveState() to save the geometry and the state of toolbars and dock widgets.

This function was introduced in  Qt 4.2.

See also restoreGeometry(), QMainWindow::saveState(), and QMainWindow::restoreState().
*/
impl /*struct*/ QWidget {
  pub fn saveGeometry_0<RetType, T: QWidget_saveGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.saveGeometry_0(self);
    // return 1;
  }
}
pub trait QWidget_saveGeometry_0<RetType> {
  fn saveGeometry_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_saveGeometry_0<usize> for () {
  fn saveGeometry_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget12saveGeometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:497
// index:0
// Public Visibility=Default Availability=Available
// [1] bool restoreGeometry(const QByteArray &)

/*
Restores the geometry and state of top-level widgets stored in the byte array geometry. Returns true on success; otherwise returns false.

If the restored geometry is off-screen, it will be modified to be inside the available screen geometry.

To restore geometry saved using QSettings, you can use code like this:


  QSettings settings("MyCompany", "MyApp");
  myWidget->restoreGeometry(settings.value("myWidget/geometry").toByteArray());



See the Window Geometry documentation for an overview of geometry issues with windows.

Use QMainWindow::restoreState() to restore the geometry and the state of toolbars and dock widgets.

This function was introduced in  Qt 4.2.

See also saveGeometry(), QSettings, QMainWindow::saveState(), and QMainWindow::restoreState().
*/
impl /*struct*/ QWidget {
  pub fn restoreGeometry_0<RetType, T: QWidget_restoreGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.restoreGeometry_0(self);
    // return 1;
  }
}
pub trait QWidget_restoreGeometry_0<RetType> {
  fn restoreGeometry_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_restoreGeometry_0<bool> for (usize) {
  fn restoreGeometry_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWidget15restoreGeometryERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:498
// index:0
// Public Visibility=Default Availability=Available
// [-2] void adjustSize()

/*
Adjusts the size of the widget to fit its contents.

This function uses sizeHint() if it is valid, i.e., the size hint's width and height are >= 0. Otherwise, it sets the size to the children rectangle that covers all child widgets (the union of all child widget rectangles).

For windows, the screen size is also taken into account. If the sizeHint() is less than (200, 100) and the size policy is expanding, the window will be at least (200, 100). The maximum size of a window is 2/3 of the screen's width and height.

See also sizeHint() and childrenRect().
*/
impl /*struct*/ QWidget {
  pub fn adjustSize_0<RetType, T: QWidget_adjustSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.adjustSize_0(self);
    // return 1;
  }
}
pub trait QWidget_adjustSize_0<RetType> {
  fn adjustSize_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_adjustSize_0<(/*void*/)> for () {
  fn adjustSize_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget10adjustSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:499
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isVisible() const

/*

*/
impl /*struct*/ QWidget {
  pub fn isVisible_0<RetType, T: QWidget_isVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isVisible_0(self);
    // return 1;
  }
}
pub trait QWidget_isVisible_0<RetType> {
  fn isVisible_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isVisible_0<bool> for () {
  fn isVisible_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget9isVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:500
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isVisibleTo(const QWidget *) const

/*
Returns true if this widget would become visible if ancestor is shown; otherwise returns false.

The true case occurs if neither the widget itself nor any parent up to but excluding ancestor has been explicitly hidden.

This function will still return true if the widget is obscured by other windows on the screen, but could be physically visible if it or they were to be moved.

isVisibleTo(0) is identical to isVisible().

See also show(), hide(), and isVisible().
*/
impl /*struct*/ QWidget {
  pub fn isVisibleTo_0<RetType, T: QWidget_isVisibleTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isVisibleTo_0(self);
    // return 1;
  }
}
pub trait QWidget_isVisibleTo_0<RetType> {
  fn isVisibleTo_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isVisibleTo_0<bool> for (usize) {
  fn isVisibleTo_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11isVisibleToEPKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:501
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isHidden() const

/*
Returns true if the widget is hidden, otherwise returns false.

A hidden widget will only become visible when show() is called on it. It will not be automatically shown when the parent is shown.

To check visibility, use !isVisible() instead (notice the exclamation mark).

isHidden() implies !isVisible(), but a widget can be not visible and not hidden at the same time. This is the case for widgets that are children of widgets that are not visible.

Widgets are hidden if:


they were created as independent windows,
they were created as children of visible widgets,
hide() or setVisible(false) was called.
*/
impl /*struct*/ QWidget {
  pub fn isHidden_0<RetType, T: QWidget_isHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isHidden_0(self);
    // return 1;
  }
}
pub trait QWidget_isHidden_0<RetType> {
  fn isHidden_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isHidden_0<bool> for () {
  fn isHidden_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget8isHiddenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:503
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isMinimized() const

/*

*/
impl /*struct*/ QWidget {
  pub fn isMinimized_0<RetType, T: QWidget_isMinimized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isMinimized_0(self);
    // return 1;
  }
}
pub trait QWidget_isMinimized_0<RetType> {
  fn isMinimized_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isMinimized_0<bool> for () {
  fn isMinimized_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11isMinimizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:504
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isMaximized() const

/*

*/
impl /*struct*/ QWidget {
  pub fn isMaximized_0<RetType, T: QWidget_isMaximized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isMaximized_0(self);
    // return 1;
  }
}
pub trait QWidget_isMaximized_0<RetType> {
  fn isMaximized_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isMaximized_0<bool> for () {
  fn isMaximized_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11isMaximizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:505
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFullScreen() const

/*

*/
impl /*struct*/ QWidget {
  pub fn isFullScreen_0<RetType, T: QWidget_isFullScreen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFullScreen_0(self);
    // return 1;
  }
}
pub trait QWidget_isFullScreen_0<RetType> {
  fn isFullScreen_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isFullScreen_0<bool> for () {
  fn isFullScreen_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget12isFullScreenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:507
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::WindowStates windowState() const

/*
Returns the current window state. The window state is a OR'ed combination of Qt::WindowState: Qt::WindowMinimized, Qt::WindowMaximized, Qt::WindowFullScreen, and Qt::WindowActive.

See also Qt::WindowState and setWindowState().
*/
impl /*struct*/ QWidget {
  pub fn windowState_0<RetType, T: QWidget_windowState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowState_0(self);
    // return 1;
  }
}
pub trait QWidget_windowState_0<RetType> {
  fn windowState_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowState_0<i32> for () {
  fn windowState_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11windowStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:508
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowState(Qt::WindowStates)

/*
Sets the window state to windowState. The window state is a OR'ed combination of Qt::WindowState: Qt::WindowMinimized, Qt::WindowMaximized, Qt::WindowFullScreen, and Qt::WindowActive.

If the window is not visible (i.e. isVisible() returns false), the window state will take effect when show() is called. For visible windows, the change is immediate. For example, to toggle between full-screen and normal mode, use the following code:


  w->setWindowState(w->windowState() ^ Qt::WindowFullScreen);



In order to restore and activate a minimized window (while preserving its maximized and/or full-screen state), use the following:


  w->setWindowState((w->windowState() & ~Qt::WindowMinimized) | Qt::WindowActive);



Calling this function will hide the widget. You must call show() to make the widget visible again.

Note: On some window systems Qt::WindowActive is not immediate, and may be ignored in certain cases.

When the window state changes, the widget receives a changeEvent() of type QEvent::WindowStateChange.

See also Qt::WindowState and windowState().
*/
impl /*struct*/ QWidget {
  pub fn setWindowState_0<RetType, T: QWidget_setWindowState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowState_0(self);
    // return 1;
  }
}
pub trait QWidget_setWindowState_0<RetType> {
  fn setWindowState_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setWindowState_0<(/*void*/)> for (i32) {
  fn setWindowState_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget14setWindowStateE6QFlagsIN2Qt11WindowStateEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:509
// index:0
// Public Visibility=Default Availability=Available
// [-2] void overrideWindowState(Qt::WindowStates)

/*

*/
impl /*struct*/ QWidget {
  pub fn overrideWindowState_0<RetType, T: QWidget_overrideWindowState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.overrideWindowState_0(self);
    // return 1;
  }
}
pub trait QWidget_overrideWindowState_0<RetType> {
  fn overrideWindowState_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_overrideWindowState_0<(/*void*/)> for (i32) {
  fn overrideWindowState_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget19overrideWindowStateE6QFlagsIN2Qt11WindowStateEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:511
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*

*/
impl /*struct*/ QWidget {
  pub fn sizeHint_0<RetType, T: QWidget_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QWidget_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:512
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*

*/
impl /*struct*/ QWidget {
  pub fn minimumSizeHint_0<RetType, T: QWidget_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QWidget_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:514
// index:0
// Public Visibility=Default Availability=Available
// [4] QSizePolicy sizePolicy() const

/*

*/
impl /*struct*/ QWidget {
  pub fn sizePolicy_0<RetType, T: QWidget_sizePolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizePolicy_0(self);
    // return 1;
  }
}
pub trait QWidget_sizePolicy_0<RetType> {
  fn sizePolicy_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_sizePolicy_0<usize> for () {
  fn sizePolicy_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget10sizePolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:515
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSizePolicy(QSizePolicy)

/*

*/
impl /*struct*/ QWidget {
  pub fn setSizePolicy_0<RetType, T: QWidget_setSizePolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizePolicy_0(self);
    // return 1;
  }
}
pub trait QWidget_setSizePolicy_0<RetType> {
  fn setSizePolicy_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setSizePolicy_0<(/*void*/)> for (usize) {
  fn setSizePolicy_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget13setSizePolicyE11QSizePolicy", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:516
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setSizePolicy(QSizePolicy::Policy, QSizePolicy::Policy)

/*

*/
impl /*struct*/ QWidget {
  pub fn setSizePolicy_1<RetType, T: QWidget_setSizePolicy_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizePolicy_1(self);
    // return 1;
  }
}
pub trait QWidget_setSizePolicy_1<RetType> {
  fn setSizePolicy_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setSizePolicy_1<(/*void*/)> for (i32,i32) {
  fn setSizePolicy_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget13setSizePolicyEN11QSizePolicy6PolicyES1_", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:517
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int heightForWidth(int) const

/*
Returns the preferred height for this widget, given the width w.

If this widget has a layout, the default implementation returns the layout's preferred height. if there is no layout, the default implementation returns -1 indicating that the preferred height does not depend on the width.
*/
impl /*struct*/ QWidget {
  pub fn heightForWidth_0<RetType, T: QWidget_heightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth_0(self);
    // return 1;
  }
}
pub trait QWidget_heightForWidth_0<RetType> {
  fn heightForWidth_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_heightForWidth_0<i32> for (i32) {
  fn heightForWidth_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget14heightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:518
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasHeightForWidth() const

/*
Returns true if the widget's preferred height depends on its width; otherwise returns false.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QWidget {
  pub fn hasHeightForWidth_0<RetType, T: QWidget_hasHeightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth_0(self);
    // return 1;
  }
}
pub trait QWidget_hasHeightForWidth_0<RetType> {
  fn hasHeightForWidth_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_hasHeightForWidth_0<bool> for () {
  fn hasHeightForWidth_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget17hasHeightForWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:520
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion visibleRegion() const

/*
Returns the unobscured region where paint events can occur.

For visible widgets, this is an approximation of the area not covered by other widgets; otherwise, this is an empty region.

The repaint() function calls this function if necessary, so in general you do not need to call it.
*/
impl /*struct*/ QWidget {
  pub fn visibleRegion_0<RetType, T: QWidget_visibleRegion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visibleRegion_0(self);
    // return 1;
  }
}
pub trait QWidget_visibleRegion_0<RetType> {
  fn visibleRegion_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_visibleRegion_0<usize> for () {
  fn visibleRegion_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget13visibleRegionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:522
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setContentsMargins(int, int, int, int)

/*
Sets the margins around the contents of the widget to have the sizes left, top, right, and bottom. The margins are used by the layout system, and may be used by subclasses to specify the area to draw in (e.g. excluding the frame).

Changing the margins will trigger a resizeEvent().

See also contentsMargins(), contentsRect(), and getContentsMargins().
*/
impl /*struct*/ QWidget {
  pub fn setContentsMargins_0<RetType, T: QWidget_setContentsMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setContentsMargins_0(self);
    // return 1;
  }
}
pub trait QWidget_setContentsMargins_0<RetType> {
  fn setContentsMargins_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setContentsMargins_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn setContentsMargins_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget18setContentsMarginsEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:523
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setContentsMargins(const QMargins &)

/*
Sets the margins around the contents of the widget to have the sizes left, top, right, and bottom. The margins are used by the layout system, and may be used by subclasses to specify the area to draw in (e.g. excluding the frame).

Changing the margins will trigger a resizeEvent().

See also contentsMargins(), contentsRect(), and getContentsMargins().
*/
impl /*struct*/ QWidget {
  pub fn setContentsMargins_1<RetType, T: QWidget_setContentsMargins_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setContentsMargins_1(self);
    // return 1;
  }
}
pub trait QWidget_setContentsMargins_1<RetType> {
  fn setContentsMargins_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setContentsMargins_1<(/*void*/)> for (usize) {
  fn setContentsMargins_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget18setContentsMarginsERK8QMargins", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:524
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getContentsMargins(int *, int *, int *, int *) const

/*
Returns the widget's contents margins for left, top, right, and bottom.

See also setContentsMargins() and contentsRect().
*/
impl /*struct*/ QWidget {
  pub fn getContentsMargins_0<RetType, T: QWidget_getContentsMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getContentsMargins_0(self);
    // return 1;
  }
}
pub trait QWidget_getContentsMargins_0<RetType> {
  fn getContentsMargins_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_getContentsMargins_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getContentsMargins_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK7QWidget18getContentsMarginsEPiS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:525
// index:0
// Public Visibility=Default Availability=Available
// [16] QMargins contentsMargins() const

/*
The contentsMargins function returns the widget's contents margins.

This function was introduced in  Qt 4.6.

See also getContentsMargins(), setContentsMargins(), and contentsRect().
*/
impl /*struct*/ QWidget {
  pub fn contentsMargins_0<RetType, T: QWidget_contentsMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentsMargins_0(self);
    // return 1;
  }
}
pub trait QWidget_contentsMargins_0<RetType> {
  fn contentsMargins_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_contentsMargins_0<usize> for () {
  fn contentsMargins_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget15contentsMarginsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:527
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect contentsRect() const

/*
Returns the area inside the widget's margins.

See also setContentsMargins() and getContentsMargins().
*/
impl /*struct*/ QWidget {
  pub fn contentsRect_0<RetType, T: QWidget_contentsRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentsRect_0(self);
    // return 1;
  }
}
pub trait QWidget_contentsRect_0<RetType> {
  fn contentsRect_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_contentsRect_0<usize> for () {
  fn contentsRect_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget12contentsRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:530
// index:0
// Public Visibility=Default Availability=Available
// [8] QLayout * layout() const

/*
Returns the layout manager that is installed on this widget, or 0 if no layout manager is installed.

The layout manager sets the geometry of the widget's children that have been added to the layout.

See also setLayout(), sizePolicy(), and Layout Management.
*/
impl /*struct*/ QWidget {
  pub fn layout_0<RetType, T: QWidget_layout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layout_0(self);
    // return 1;
  }
}
pub trait QWidget_layout_0<RetType> {
  fn layout_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_layout_0<usize> for () {
  fn layout_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget6layoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:531
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLayout(QLayout *)

/*
Sets the layout manager for this widget to layout.

If there already is a layout manager installed on this widget, QWidget won't let you install another. You must first delete the existing layout manager (returned by layout()) before you can call setLayout() with the new layout.

If layout is the layout manager on a different widget, setLayout() will reparent the layout and make it the layout manager for this widget.

Example:


      QVBoxLayout *layout = new QVBoxLayout;
      layout->addWidget(formWidget);
      setLayout(layout);



An alternative to calling this function is to pass this widget to the layout's constructor.

The QWidget will take ownership of layout.

See also layout() and Layout Management.
*/
impl /*struct*/ QWidget {
  pub fn setLayout_0<RetType, T: QWidget_setLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLayout_0(self);
    // return 1;
  }
}
pub trait QWidget_setLayout_0<RetType> {
  fn setLayout_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setLayout_0<(/*void*/)> for (usize) {
  fn setLayout_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget9setLayoutEP7QLayout", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:532
// index:0
// Public Visibility=Default Availability=Available
// [-2] void updateGeometry()

/*
Notifies the layout system that this widget has changed and may need to change geometry.

Call this function if the sizeHint() or sizePolicy() have changed.

For explicitly hidden widgets, updateGeometry() is a no-op. The layout system will be notified as soon as the widget is shown.
*/
impl /*struct*/ QWidget {
  pub fn updateGeometry_0<RetType, T: QWidget_updateGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateGeometry_0(self);
    // return 1;
  }
}
pub trait QWidget_updateGeometry_0<RetType> {
  fn updateGeometry_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_updateGeometry_0<(/*void*/)> for () {
  fn updateGeometry_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget14updateGeometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:534
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setParent(QWidget *)

/*
Sets the parent of the widget to parent, and resets the window flags. The widget is moved to position (0, 0) in its new parent.

If the new parent widget is in a different window, the reparented widget and its children are appended to the end of the tab chain of the new parent widget, in the same internal order as before. If one of the moved widgets had keyboard focus, setParent() calls clearFocus() for that widget.

If the new parent widget is in the same window as the old parent, setting the parent doesn't change the tab order or keyboard focus.

If the "new" parent widget is the old parent widget, this function does nothing.

Note: The widget becomes invisible as part of changing its parent, even if it was previously visible. You must call show() to make the widget visible again.

Warning: It is very unlikely that you will ever need this function. If you have a widget that changes its content dynamically, it is far easier to use QStackedWidget.

See also setWindowFlags().
*/
impl /*struct*/ QWidget {
  pub fn setParent_0<RetType, T: QWidget_setParent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setParent_0(self);
    // return 1;
  }
}
pub trait QWidget_setParent_0<RetType> {
  fn setParent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setParent_0<(/*void*/)> for (usize) {
  fn setParent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget9setParentEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:535
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setParent(QWidget *, Qt::WindowFlags)

/*
Sets the parent of the widget to parent, and resets the window flags. The widget is moved to position (0, 0) in its new parent.

If the new parent widget is in a different window, the reparented widget and its children are appended to the end of the tab chain of the new parent widget, in the same internal order as before. If one of the moved widgets had keyboard focus, setParent() calls clearFocus() for that widget.

If the new parent widget is in the same window as the old parent, setting the parent doesn't change the tab order or keyboard focus.

If the "new" parent widget is the old parent widget, this function does nothing.

Note: The widget becomes invisible as part of changing its parent, even if it was previously visible. You must call show() to make the widget visible again.

Warning: It is very unlikely that you will ever need this function. If you have a widget that changes its content dynamically, it is far easier to use QStackedWidget.

See also setWindowFlags().
*/
impl /*struct*/ QWidget {
  pub fn setParent_1<RetType, T: QWidget_setParent_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setParent_1(self);
    // return 1;
  }
}
pub trait QWidget_setParent_1<RetType> {
  fn setParent_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setParent_1<(/*void*/)> for (usize,i32) {
  fn setParent_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget9setParentEPS_6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:537
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scroll(int, int)

/*
Scrolls the widget including its children dx pixels to the right and dy downward. Both dx and dy may be negative.

After scrolling, the widgets will receive paint events for the areas that need to be repainted. For widgets that Qt knows to be opaque, this is only the newly exposed parts. For example, if an opaque widget is scrolled 8 pixels to the left, only an 8-pixel wide stripe at the right edge needs updating.

Since widgets propagate the contents of their parents by default, you need to set the autoFillBackground property, or use setAttribute() to set the Qt::WA_OpaquePaintEvent attribute, to make a widget opaque.

For widgets that use contents propagation, a scroll will cause an update of the entire scroll area.

See also Transparency and Double Buffering.
*/
impl /*struct*/ QWidget {
  pub fn scroll_0<RetType, T: QWidget_scroll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scroll_0(self);
    // return 1;
  }
}
pub trait QWidget_scroll_0<RetType> {
  fn scroll_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_scroll_0<(/*void*/)> for (i32,i32) {
  fn scroll_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget6scrollEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:538
// index:1
// Public Visibility=Default Availability=Available
// [-2] void scroll(int, int, const QRect &)

/*
Scrolls the widget including its children dx pixels to the right and dy downward. Both dx and dy may be negative.

After scrolling, the widgets will receive paint events for the areas that need to be repainted. For widgets that Qt knows to be opaque, this is only the newly exposed parts. For example, if an opaque widget is scrolled 8 pixels to the left, only an 8-pixel wide stripe at the right edge needs updating.

Since widgets propagate the contents of their parents by default, you need to set the autoFillBackground property, or use setAttribute() to set the Qt::WA_OpaquePaintEvent attribute, to make a widget opaque.

For widgets that use contents propagation, a scroll will cause an update of the entire scroll area.

See also Transparency and Double Buffering.
*/
impl /*struct*/ QWidget {
  pub fn scroll_1<RetType, T: QWidget_scroll_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scroll_1(self);
    // return 1;
  }
}
pub trait QWidget_scroll_1<RetType> {
  fn scroll_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_scroll_1<(/*void*/)> for (i32,i32,usize) {
  fn scroll_1(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget6scrollEiiRK5QRect", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:542
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * focusWidget() const

/*
Returns the last child of this widget that setFocus had been called on. For top level widgets this is the widget that will get focus in case this window gets activated

This is not the same as QApplication::focusWidget(), which returns the focus widget in the currently active window.
*/
impl /*struct*/ QWidget {
  pub fn focusWidget_0<RetType, T: QWidget_focusWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusWidget_0(self);
    // return 1;
  }
}
pub trait QWidget_focusWidget_0<RetType> {
  fn focusWidget_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_focusWidget_0<usize> for () {
  fn focusWidget_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11focusWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:543
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * nextInFocusChain() const

/*
Returns the next widget in this widget's focus chain.

See also previousInFocusChain().
*/
impl /*struct*/ QWidget {
  pub fn nextInFocusChain_0<RetType, T: QWidget_nextInFocusChain_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nextInFocusChain_0(self);
    // return 1;
  }
}
pub trait QWidget_nextInFocusChain_0<RetType> {
  fn nextInFocusChain_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_nextInFocusChain_0<usize> for () {
  fn nextInFocusChain_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget16nextInFocusChainEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:544
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * previousInFocusChain() const

/*
The previousInFocusChain function returns the previous widget in this widget's focus chain.

This function was introduced in  Qt 4.6.

See also nextInFocusChain().
*/
impl /*struct*/ QWidget {
  pub fn previousInFocusChain_0<RetType, T: QWidget_previousInFocusChain_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.previousInFocusChain_0(self);
    // return 1;
  }
}
pub trait QWidget_previousInFocusChain_0<RetType> {
  fn previousInFocusChain_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_previousInFocusChain_0<usize> for () {
  fn previousInFocusChain_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget20previousInFocusChainEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:547
// index:0
// Public Visibility=Default Availability=Available
// [1] bool acceptDrops() const

/*

*/
impl /*struct*/ QWidget {
  pub fn acceptDrops_0<RetType, T: QWidget_acceptDrops_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.acceptDrops_0(self);
    // return 1;
  }
}
pub trait QWidget_acceptDrops_0<RetType> {
  fn acceptDrops_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_acceptDrops_0<bool> for () {
  fn acceptDrops_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11acceptDropsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:548
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAcceptDrops(bool)

/*

*/
impl /*struct*/ QWidget {
  pub fn setAcceptDrops_0<RetType, T: QWidget_setAcceptDrops_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAcceptDrops_0(self);
    // return 1;
  }
}
pub trait QWidget_setAcceptDrops_0<RetType> {
  fn setAcceptDrops_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setAcceptDrops_0<(/*void*/)> for (bool) {
  fn setAcceptDrops_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget14setAcceptDropsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:552
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addAction(QAction *)

/*
Appends the action action to this widget's list of actions.

All QWidgets have a list of QActions, however they can be represented graphically in many different ways. The default use of the QAction list (as returned by actions()) is to create a context QMenu.

A QWidget should only have one of each action and adding an action it already has will not cause the same action to be in the widget twice.

The ownership of action is not transferred to this QWidget.

See also removeAction(), insertAction(), actions(), and QMenu.
*/
impl /*struct*/ QWidget {
  pub fn addAction_0<RetType, T: QWidget_addAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_0(self);
    // return 1;
  }
}
pub trait QWidget_addAction_0<RetType> {
  fn addAction_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_addAction_0<(/*void*/)> for (usize) {
  fn addAction_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget9addActionEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:560
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertAction(QAction *, QAction *)

/*
Inserts the action action to this widget's list of actions, before the action before. It appends the action if before is 0 or before is not a valid action for this widget.

A QWidget should only have one of each action.

See also removeAction(), addAction(), QMenu, contextMenuPolicy, and actions().
*/
impl /*struct*/ QWidget {
  pub fn insertAction_0<RetType, T: QWidget_insertAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertAction_0(self);
    // return 1;
  }
}
pub trait QWidget_insertAction_0<RetType> {
  fn insertAction_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_insertAction_0<(/*void*/)> for (usize,usize) {
  fn insertAction_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget12insertActionEP7QActionS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:561
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeAction(QAction *)

/*
Removes the action action from this widget's list of actions.

See also insertAction(), actions(), and insertAction().
*/
impl /*struct*/ QWidget {
  pub fn removeAction_0<RetType, T: QWidget_removeAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeAction_0(self);
    // return 1;
  }
}
pub trait QWidget_removeAction_0<RetType> {
  fn removeAction_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_removeAction_0<(/*void*/)> for (usize) {
  fn removeAction_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget12removeActionEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:565
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * parentWidget() const

/*
Returns the parent of this widget, or 0 if it does not have any parent widget.
*/
impl /*struct*/ QWidget {
  pub fn parentWidget_0<RetType, T: QWidget_parentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parentWidget_0(self);
    // return 1;
  }
}
pub trait QWidget_parentWidget_0<RetType> {
  fn parentWidget_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_parentWidget_0<usize> for () {
  fn parentWidget_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget12parentWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:567
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowFlags(Qt::WindowFlags)

/*

*/
impl /*struct*/ QWidget {
  pub fn setWindowFlags_0<RetType, T: QWidget_setWindowFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowFlags_0(self);
    // return 1;
  }
}
pub trait QWidget_setWindowFlags_0<RetType> {
  fn setWindowFlags_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setWindowFlags_0<(/*void*/)> for (i32) {
  fn setWindowFlags_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget14setWindowFlagsE6QFlagsIN2Qt10WindowTypeEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:568
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::WindowFlags windowFlags() const

/*

*/
impl /*struct*/ QWidget {
  pub fn windowFlags_0<RetType, T: QWidget_windowFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowFlags_0(self);
    // return 1;
  }
}
pub trait QWidget_windowFlags_0<RetType> {
  fn windowFlags_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowFlags_0<i32> for () {
  fn windowFlags_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11windowFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:569
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowFlag(Qt::WindowType, bool)

/*
Sets the window flag flag on this widget if on is true; otherwise clears the flag.

This function was introduced in  Qt 5.9.

See also setWindowFlags(), windowFlags(), and windowType().
*/
impl /*struct*/ QWidget {
  pub fn setWindowFlag_0<RetType, T: QWidget_setWindowFlag_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowFlag_0(self);
    // return 1;
  }
}
pub trait QWidget_setWindowFlag_0<RetType> {
  fn setWindowFlag_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setWindowFlag_0<(/*void*/)> for (i32,bool) {
  fn setWindowFlag_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget13setWindowFlagEN2Qt10WindowTypeEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:570
// index:0
// Public Visibility=Default Availability=Available
// [-2] void overrideWindowFlags(Qt::WindowFlags)

/*
Sets the window flags for the widget to flags, without telling the window system.

Warning: Do not call this function unless you really know what you're doing.

See also setWindowFlags().
*/
impl /*struct*/ QWidget {
  pub fn overrideWindowFlags_0<RetType, T: QWidget_overrideWindowFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.overrideWindowFlags_0(self);
    // return 1;
  }
}
pub trait QWidget_overrideWindowFlags_0<RetType> {
  fn overrideWindowFlags_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_overrideWindowFlags_0<(/*void*/)> for (i32) {
  fn overrideWindowFlags_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget19overrideWindowFlagsE6QFlagsIN2Qt10WindowTypeEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:572
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::WindowType windowType() const

/*
Returns the window type of this widget. This is identical to windowFlags() & Qt::WindowType_Mask.

See also windowFlags.
*/
impl /*struct*/ QWidget {
  pub fn windowType_0<RetType, T: QWidget_windowType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowType_0(self);
    // return 1;
  }
}
pub trait QWidget_windowType_0<RetType> {
  fn windowType_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowType_0<i32> for () {
  fn windowType_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget10windowTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:574
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWidget * find(WId)

/*
Returns a pointer to the widget with window identifer/handle id.

The window identifier type depends on the underlying window system, see qwindowdefs.h for the actual definition. If there is no widget with this identifier, 0 is returned.
*/
impl /*struct*/ QWidget {
  pub fn find_0<RetType, T: QWidget_find_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.find_0();
    // return 1;
  }
}
pub trait QWidget_find_0<RetType> {
  fn find_0(self ) -> RetType;
}
impl<'a> /*trait*/ QWidget_find_0<usize> for (u64) {
  fn find_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWidget4findEy", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:575
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QWidget * childAt(int, int) const

/*
Returns the visible child widget at the position (x, y) in the widget's coordinate system. If there is no visible child widget at the specified position, the function returns 0.
*/
impl /*struct*/ QWidget {
  pub fn childAt_0<RetType, T: QWidget_childAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childAt_0(self);
    // return 1;
  }
}
pub trait QWidget_childAt_0<RetType> {
  fn childAt_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_childAt_0<usize> for (i32,i32) {
  fn childAt_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget7childAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:576
// index:1
// Public Visibility=Default Availability=Available
// [8] QWidget * childAt(const QPoint &) const

/*
Returns the visible child widget at the position (x, y) in the widget's coordinate system. If there is no visible child widget at the specified position, the function returns 0.
*/
impl /*struct*/ QWidget {
  pub fn childAt_1<RetType, T: QWidget_childAt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childAt_1(self);
    // return 1;
  }
}
pub trait QWidget_childAt_1<RetType> {
  fn childAt_1(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_childAt_1<usize> for (usize) {
  fn childAt_1(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget7childAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:578
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAttribute(Qt::WidgetAttribute, bool)

/*
Sets the attribute attribute on this widget if on is true; otherwise clears the attribute.

See also testAttribute().
*/
impl /*struct*/ QWidget {
  pub fn setAttribute_0<RetType, T: QWidget_setAttribute_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAttribute_0(self);
    // return 1;
  }
}
pub trait QWidget_setAttribute_0<RetType> {
  fn setAttribute_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setAttribute_0<(/*void*/)> for (i32,bool) {
  fn setAttribute_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget12setAttributeEN2Qt15WidgetAttributeEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:579
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool testAttribute(Qt::WidgetAttribute) const

/*
Returns true if attribute attribute is set on this widget; otherwise returns false.

See also setAttribute().
*/
impl /*struct*/ QWidget {
  pub fn testAttribute_0<RetType, T: QWidget_testAttribute_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testAttribute_0(self);
    // return 1;
  }
}
pub trait QWidget_testAttribute_0<RetType> {
  fn testAttribute_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_testAttribute_0<bool> for (i32) {
  fn testAttribute_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget13testAttributeEN2Qt15WidgetAttributeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:581
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QPaintEngine * paintEngine() const

/*
Reimplemented from QPaintDevice::paintEngine().

Returns the widget's paint engine.

Note that this function should not be called explicitly by the user, since it's meant for reimplementation purposes only. The function is called by Qt internally, and the default implementation may not always return a valid pointer.
*/
impl /*struct*/ QWidget {
  pub fn paintEngine_0<RetType, T: QWidget_paintEngine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEngine_0(self);
    // return 1;
  }
}
pub trait QWidget_paintEngine_0<RetType> {
  fn paintEngine_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_paintEngine_0<usize> for () {
  fn paintEngine_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget11paintEngineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:583
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ensurePolished() const

/*
Ensures that the widget and its children have been polished by QStyle (i.e., have a proper font and palette).

QWidget calls this function after it has been fully constructed but before it is shown the very first time. You can call this function if you want to ensure that the widget is polished before doing an operation, e.g., the correct font size might be needed in the widget's sizeHint() reimplementation. Note that this function is called from the default implementation of sizeHint().

Polishing is useful for final initialization that must happen after all constructors (from base classes as well as from subclasses) have been called.

If you need to change some settings when a widget is polished, reimplement event() and handle the QEvent::Polish event type.

Note: The function is declared const so that it can be called from other const functions (e.g., sizeHint()).

See also event().
*/
impl /*struct*/ QWidget {
  pub fn ensurePolished_0<RetType, T: QWidget_ensurePolished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ensurePolished_0(self);
    // return 1;
  }
}
pub trait QWidget_ensurePolished_0<RetType> {
  fn ensurePolished_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_ensurePolished_0<(/*void*/)> for () {
  fn ensurePolished_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZNK7QWidget14ensurePolishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:585
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isAncestorOf(const QWidget *) const

/*
Returns true if this widget is a parent, (or grandparent and so on to any level), of the given child, and both widgets are within the same window; otherwise returns false.
*/
impl /*struct*/ QWidget {
  pub fn isAncestorOf_0<RetType, T: QWidget_isAncestorOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAncestorOf_0(self);
    // return 1;
  }
}
pub trait QWidget_isAncestorOf_0<RetType> {
  fn isAncestorOf_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_isAncestorOf_0<bool> for (usize) {
  fn isAncestorOf_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget12isAncestorOfEPKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:592
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoFillBackground() const

/*

*/
impl /*struct*/ QWidget {
  pub fn autoFillBackground_0<RetType, T: QWidget_autoFillBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoFillBackground_0(self);
    // return 1;
  }
}
pub trait QWidget_autoFillBackground_0<RetType> {
  fn autoFillBackground_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_autoFillBackground_0<bool> for () {
  fn autoFillBackground_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget18autoFillBackgroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:593
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoFillBackground(bool)

/*

*/
impl /*struct*/ QWidget {
  pub fn setAutoFillBackground_0<RetType, T: QWidget_setAutoFillBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoFillBackground_0(self);
    // return 1;
  }
}
pub trait QWidget_setAutoFillBackground_0<RetType> {
  fn setAutoFillBackground_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setAutoFillBackground_0<(/*void*/)> for (bool) {
  fn setAutoFillBackground_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget21setAutoFillBackgroundEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:595
// index:0
// Public Visibility=Default Availability=Available
// [8] QBackingStore * backingStore() const

/*
Returns the QBackingStore this widget will be drawn into.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QWidget {
  pub fn backingStore_0<RetType, T: QWidget_backingStore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backingStore_0(self);
    // return 1;
  }
}
pub trait QWidget_backingStore_0<RetType> {
  fn backingStore_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_backingStore_0<usize> for () {
  fn backingStore_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget12backingStoreEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:597
// index:0
// Public Visibility=Default Availability=Available
// [8] QWindow * windowHandle() const

/*
If this is a native widget, return the associated QWindow. Otherwise return null.

Native widgets include toplevel widgets, QGLWidget, and child widgets on which winId() was called.

This function was introduced in  Qt 5.0.

See also winId().
*/
impl /*struct*/ QWidget {
  pub fn windowHandle_0<RetType, T: QWidget_windowHandle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowHandle_0(self);
    // return 1;
  }
}
pub trait QWidget_windowHandle_0<RetType> {
  fn windowHandle_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowHandle_0<usize> for () {
  fn windowHandle_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget12windowHandleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:599
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWidget * createWindowContainer(QWindow *, QWidget *, Qt::WindowFlags)

/*
Creates a QWidget that makes it possible to embed window into a QWidget-based application.

The window container is created as a child of parent and with window flags flags.

Once the window has been embedded into the container, the container will control the window's geometry and visibility. Explicit calls to QWindow::setGeometry(), QWindow::show() or QWindow::hide() on an embedded window is not recommended.

The container takes over ownership of window. The window can be removed from the window container with a call to QWindow::setParent().

The window container is attached as a native child window to the toplevel window it is a child of. When a window container is used as a child of a QAbstractScrollArea or QMdiArea, it will create a native window for every widget in its parent chain to allow for proper stacking and clipping in this use case. Creating a native window for the window container also allows for proper stacking and clipping. This must be done before showing the window container. Applications with many native child windows may suffer from performance issues.

The window container has a number of known limitations:


Stacking order; The embedded window will stack on top of the widget hierarchy as an opaque box. The stacking order of multiple overlapping window container instances is undefined.
Rendering Integration; The window container does not interoperate with QGraphicsProxyWidget, QWidget::render() or similar functionality.
Focus Handling; It is possible to let the window container instance have any focus policy and it will delegate focus to the window via a call to QWindow::requestActivate(). However, returning to the normal focus chain from the QWindow instance will be up to the QWindow instance implementation itself. For instance, when entering a Qt Quick based window with tab focus, it is quite likely that further tab presses will only cycle inside the QML application. Also, whether QWindow::requestActivate() actually gives the window focus, is platform dependent.
Using many window container instances in a QWidget-based application can greatly hurt the overall performance of the application.
*/
impl /*struct*/ QWidget {
  pub fn createWindowContainer_0<RetType, T: QWidget_createWindowContainer_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.createWindowContainer_0();
    // return 1;
  }
}
pub trait QWidget_createWindowContainer_0<RetType> {
  fn createWindowContainer_0(self ) -> RetType;
}
impl<'a> /*trait*/ QWidget_createWindowContainer_0<usize> for (usize,usize,i32) {
  fn createWindowContainer_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWidget21createWindowContainerEP7QWindowPS_6QFlagsIN2Qt10WindowTypeEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:604
// index:0
// Public Visibility=Default Availability=Available
// [-2] void windowTitleChanged(const QString &)

/*
This signal is emitted when the window's title has changed, with the new title as an argument.

This function was introduced in  Qt 5.2.

Note: Notifier signal for property windowTitle.
*/
impl /*struct*/ QWidget {
  pub fn windowTitleChanged_0<RetType, T: QWidget_windowTitleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowTitleChanged_0(self);
    // return 1;
  }
}
pub trait QWidget_windowTitleChanged_0<RetType> {
  fn windowTitleChanged_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowTitleChanged_0<(/*void*/)> for (usize) {
  fn windowTitleChanged_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget18windowTitleChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:605
// index:0
// Public Visibility=Default Availability=Available
// [-2] void windowIconChanged(const QIcon &)

/*
This signal is emitted when the window's icon has changed, with the new icon as an argument.

This function was introduced in  Qt 5.2.

Note: Notifier signal for property windowIcon.
*/
impl /*struct*/ QWidget {
  pub fn windowIconChanged_0<RetType, T: QWidget_windowIconChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowIconChanged_0(self);
    // return 1;
  }
}
pub trait QWidget_windowIconChanged_0<RetType> {
  fn windowIconChanged_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowIconChanged_0<(/*void*/)> for (usize) {
  fn windowIconChanged_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget17windowIconChangedERK5QIcon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:606
// index:0
// Public Visibility=Default Availability=Available
// [-2] void windowIconTextChanged(const QString &)

/*

*/
impl /*struct*/ QWidget {
  pub fn windowIconTextChanged_0<RetType, T: QWidget_windowIconTextChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowIconTextChanged_0(self);
    // return 1;
  }
}
pub trait QWidget_windowIconTextChanged_0<RetType> {
  fn windowIconTextChanged_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_windowIconTextChanged_0<(/*void*/)> for (usize) {
  fn windowIconTextChanged_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget21windowIconTextChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:607
// index:0
// Public Visibility=Default Availability=Available
// [-2] void customContextMenuRequested(const QPoint &)

/*
This signal is emitted when the widget's contextMenuPolicy is Qt::CustomContextMenu, and the user has requested a context menu on the widget. The position pos is the position of the context menu event that the widget receives. Normally this is in widget coordinates. The exception to this rule is QAbstractScrollArea and its subclasses that map the context menu event to coordinates of the viewport().

See also mapToGlobal(), QMenu, and contextMenuPolicy.
*/
impl /*struct*/ QWidget {
  pub fn customContextMenuRequested_0<RetType, T: QWidget_customContextMenuRequested_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.customContextMenuRequested_0(self);
    // return 1;
  }
}
pub trait QWidget_customContextMenuRequested_0<RetType> {
  fn customContextMenuRequested_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_customContextMenuRequested_0<(/*void*/)> for (usize) {
  fn customContextMenuRequested_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget26customContextMenuRequestedERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:611
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().

This is the main event handler; it handles event event. You can reimplement this function in a subclass, but we recommend using one of the specialized event handlers instead.

Key press and release events are treated differently from other events. event() checks for Tab and Shift+Tab and tries to move the focus appropriately. If there is no widget to move the focus to (or the key press is not Tab or Shift+Tab), event() calls keyPressEvent().

Mouse and tablet event handling is also slightly special: only when the widget is enabled, event() will call the specialized handlers such as mousePressEvent(); otherwise it will discard the event.

This function returns true if the event was recognized, otherwise it returns false. If the recognized event was accepted (see QEvent::accepted), any further processing such as event propagation to the parent widget stops.

See also closeEvent(), focusInEvent(), focusOutEvent(), enterEvent(), keyPressEvent(), keyReleaseEvent(), leaveEvent(), mouseDoubleClickEvent(), mouseMoveEvent(), mousePressEvent(), mouseReleaseEvent(), moveEvent(), paintEvent(), resizeEvent(), QObject::event(), and QObject::timerEvent().
*/
impl /*struct*/ QWidget {
  pub fn event_0<RetType, T: QWidget_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QWidget_event_0<RetType> {
  fn event_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWidget5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:612
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive mouse press events for the widget.

If you create new widgets in the mousePressEvent() the mouseReleaseEvent() may not end up where you expect, depending on the underlying window system (or X11 window manager), the widgets' location and maybe more.

The default implementation implements the closing of popup widgets when you click outside the window. For other widget types it does nothing.

See also mouseReleaseEvent(), mouseDoubleClickEvent(), mouseMoveEvent(), event(), QMouseEvent, and Scribble Example.
*/
impl /*struct*/ QWidget {
  pub fn mousePressEvent_0<RetType, T: QWidget_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:613
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive mouse release events for the widget.

See also mousePressEvent(), mouseDoubleClickEvent(), mouseMoveEvent(), event(), QMouseEvent, and Scribble Example.
*/
impl /*struct*/ QWidget {
  pub fn mouseReleaseEvent_0<RetType, T: QWidget_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:614
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QMouseEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive mouse double click events for the widget.

The default implementation calls mousePressEvent().

Note: The widget will also receive mouse press and mouse release events in addition to the double click event. It is up to the developer to ensure that the application interprets these events correctly.

See also mousePressEvent(), mouseReleaseEvent(), mouseMoveEvent(), event(), and QMouseEvent.
*/
impl /*struct*/ QWidget {
  pub fn mouseDoubleClickEvent_0<RetType, T: QWidget_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget21mouseDoubleClickEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:615
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive mouse move events for the widget.

If mouse tracking is switched off, mouse move events only occur if a mouse button is pressed while the mouse is being moved. If mouse tracking is switched on, mouse move events occur even if no mouse button is pressed.

QMouseEvent::pos() reports the position of the mouse cursor, relative to this widget. For press and release events, the position is usually the same as the position of the last mouse move event, but it might be different if the user's hand shakes. This is a feature of the underlying window system, not Qt.

If you want to show a tooltip immediately, while the mouse is moving (e.g., to get the mouse coordinates with QMouseEvent::pos() and show them as a tooltip), you must first enable mouse tracking as described above. Then, to ensure that the tooltip is updated immediately, you must call QToolTip::showText() instead of setToolTip() in your implementation of mouseMoveEvent().

See also setMouseTracking(), mousePressEvent(), mouseReleaseEvent(), mouseDoubleClickEvent(), event(), QMouseEvent, and Scribble Example.
*/
impl /*struct*/ QWidget {
  pub fn mouseMoveEvent_0<RetType, T: QWidget_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:617
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive wheel events for the widget.

If you reimplement this handler, it is very important that you ignore() the event if you do not handle it, so that the widget's parent can interpret it.

The default implementation ignores the event.

See also QEvent::ignore(), QEvent::accept(), event(), and QWheelEvent.
*/
impl /*struct*/ QWidget {
  pub fn wheelEvent_0<RetType, T: QWidget_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:619
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive key press events for the widget.

A widget must call setFocusPolicy() to accept focus initially and have focus in order to receive a key press event.

If you reimplement this handler, it is very important that you call the base class implementation if you do not act upon the key.

The default implementation closes popup widgets if the user presses the key sequence for QKeySequence::Cancel (typically the Escape key). Otherwise the event is ignored, so that the widget's parent can interpret it.

Note that QKeyEvent starts with isAccepted() == true, so you do not need to call QKeyEvent::accept() - just do not call the base class implementation if you act upon the key.

See also keyReleaseEvent(), setFocusPolicy(), focusInEvent(), focusOutEvent(), event(), QKeyEvent, and Tetrix Example.
*/
impl /*struct*/ QWidget {
  pub fn keyPressEvent_0<RetType, T: QWidget_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:620
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyReleaseEvent(QKeyEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive key release events for the widget.

A widget must accept focus initially and have focus in order to receive a key release event.

If you reimplement this handler, it is very important that you call the base class implementation if you do not act upon the key.

The default implementation ignores the event, so that the widget's parent can interpret it.

Note that QKeyEvent starts with isAccepted() == true, so you do not need to call QKeyEvent::accept() - just do not call the base class implementation if you act upon the key.

See also keyPressEvent(), QEvent::ignore(), setFocusPolicy(), focusInEvent(), focusOutEvent(), event(), and QKeyEvent.
*/
impl /*struct*/ QWidget {
  pub fn keyReleaseEvent_0<RetType, T: QWidget_keyReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_keyReleaseEvent_0<RetType> {
  fn keyReleaseEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_keyReleaseEvent_0<(/*void*/)> for (usize) {
  fn keyReleaseEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget15keyReleaseEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:621
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
This event handler can be reimplemented in a subclass to receive keyboard focus events (focus received) for the widget. The event is passed in the event parameter

A widget normally must setFocusPolicy() to something other than Qt::NoFocus in order to receive focus events. (Note that the application programmer can call setFocus() on any widget, even those that do not normally accept focus.)

The default implementation updates the widget (except for windows that do not specify a focusPolicy()).

See also focusOutEvent(), setFocusPolicy(), keyPressEvent(), keyReleaseEvent(), event(), and QFocusEvent.
*/
impl /*struct*/ QWidget {
  pub fn focusInEvent_0<RetType, T: QWidget_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:622
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
This event handler can be reimplemented in a subclass to receive keyboard focus events (focus lost) for the widget. The events is passed in the event parameter.

A widget normally must setFocusPolicy() to something other than Qt::NoFocus in order to receive focus events. (Note that the application programmer can call setFocus() on any widget, even those that do not normally accept focus.)

The default implementation updates the widget (except for windows that do not specify a focusPolicy()).

See also focusInEvent(), setFocusPolicy(), keyPressEvent(), keyReleaseEvent(), event(), and QFocusEvent.
*/
impl /*struct*/ QWidget {
  pub fn focusOutEvent_0<RetType, T: QWidget_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:623
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void enterEvent(QEvent *)

/*
This event handler can be reimplemented in a subclass to receive widget enter events which are passed in the event parameter.

An event is sent to the widget when the mouse cursor enters the widget.

See also leaveEvent(), mouseMoveEvent(), and event().
*/
impl /*struct*/ QWidget {
  pub fn enterEvent_0<RetType, T: QWidget_enterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.enterEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_enterEvent_0<RetType> {
  fn enterEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_enterEvent_0<(/*void*/)> for (usize) {
  fn enterEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget10enterEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:624
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void leaveEvent(QEvent *)

/*
This event handler can be reimplemented in a subclass to receive widget leave events which are passed in the event parameter.

A leave event is sent to the widget when the mouse cursor leaves the widget.

See also enterEvent(), mouseMoveEvent(), and event().
*/
impl /*struct*/ QWidget {
  pub fn leaveEvent_0<RetType, T: QWidget_leaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leaveEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_leaveEvent_0<RetType> {
  fn leaveEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_leaveEvent_0<(/*void*/)> for (usize) {
  fn leaveEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget10leaveEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:625
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
This event handler can be reimplemented in a subclass to receive paint events passed in event.

A paint event is a request to repaint all or part of a widget. It can happen for one of the following reasons:


repaint() or update() was invoked,
the widget was obscured and has now been uncovered, or
many other reasons.


Many widgets can simply repaint their entire surface when asked to, but some slow widgets need to optimize by painting only the requested region: QPaintEvent::region(). This speed optimization does not change the result, as painting is clipped to that region during event processing. QListView and QTableView do this, for example.

Qt also tries to speed up painting by merging multiple paint events into one. When update() is called several times or the window system sends several paint events, Qt merges these events into one event with a larger region (see QRegion::united()). The repaint() function does not permit this optimization, so we suggest using update() whenever possible.

When the paint event occurs, the update region has normally been erased, so you are painting on the widget's background.

The background can be set using setBackgroundRole() and setPalette().

Since Qt 4.0, QWidget automatically double-buffers its painting, so there is no need to write double-buffering code in paintEvent() to avoid flicker.

Note: Generally, you should refrain from calling update() or repaint() inside a paintEvent(). For example, calling update() or repaint() on children inside a paintEvent() results in undefined behavior; the child may or may not get a paint event.

Warning: If you are using a custom paint engine without Qt's backingstore, Qt::WA_PaintOnScreen must be set. Otherwise, QWidget::paintEngine() will never be called; the backingstore will be used instead.

See also event(), repaint(), update(), QPainter, QPixmap, QPaintEvent, and Analog Clock Example.
*/
impl /*struct*/ QWidget {
  pub fn paintEvent_0<RetType, T: QWidget_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:626
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void moveEvent(QMoveEvent *)

/*
This event handler can be reimplemented in a subclass to receive widget move events which are passed in the event parameter. When the widget receives this event, it is already at the new position.

The old position is accessible through QMoveEvent::oldPos().

See also resizeEvent(), event(), move(), and QMoveEvent.
*/
impl /*struct*/ QWidget {
  pub fn moveEvent_0<RetType, T: QWidget_moveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_moveEvent_0<RetType> {
  fn moveEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_moveEvent_0<(/*void*/)> for (usize) {
  fn moveEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget9moveEventEP10QMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:627
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
This event handler can be reimplemented in a subclass to receive widget resize events which are passed in the event parameter. When resizeEvent() is called, the widget already has its new geometry. The old size is accessible through QResizeEvent::oldSize().

The widget will be erased and receive a paint event immediately after processing the resize event. No drawing need be (or should be) done inside this handler.

See also moveEvent(), event(), resize(), QResizeEvent, paintEvent(), and Scribble Example.
*/
impl /*struct*/ QWidget {
  pub fn resizeEvent_0<RetType, T: QWidget_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:628
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void closeEvent(QCloseEvent *)

/*
This event handler is called with the given event when Qt receives a window close request for a top-level widget from the window system.

By default, the event is accepted and the widget is closed. You can reimplement this function to change the way the widget responds to window close requests. For example, you can prevent the window from closing by calling ignore() on all events.

Main window applications typically use reimplementations of this function to check whether the user's work has been saved and ask for permission before closing. For example, the Application Example uses a helper function to determine whether or not to close the window:


  void MainWindow::closeEvent(QCloseEvent *event)
  {
      if (maybeSave()) {
          writeSettings();
          event->accept();
      } else {
          event->ignore();
      }
  }



See also event(), hide(), close(), QCloseEvent, and Application Example.
*/
impl /*struct*/ QWidget {
  pub fn closeEvent_0<RetType, T: QWidget_closeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_closeEvent_0<RetType> {
  fn closeEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_closeEvent_0<(/*void*/)> for (usize) {
  fn closeEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget10closeEventEP11QCloseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:630
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QContextMenuEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive widget context menu events.

The handler is called when the widget's contextMenuPolicy is Qt::DefaultContextMenu.

The default implementation ignores the context event. See the QContextMenuEvent documentation for more details.

See also event(), QContextMenuEvent, and customContextMenuRequested().
*/
impl /*struct*/ QWidget {
  pub fn contextMenuEvent_0<RetType, T: QWidget_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget16contextMenuEventEP17QContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:633
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void tabletEvent(QTabletEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive tablet events for the widget.

If you reimplement this handler, it is very important that you ignore() the event if you do not handle it, so that the widget's parent can interpret it.

The default implementation ignores the event.

If tablet tracking is switched off, tablet move events only occur if the stylus is in contact with the tablet, or at least one stylus button is pressed, while the stylus is being moved. If tablet tracking is switched on, tablet move events occur even while the stylus is hovering in proximity of the tablet, with no buttons pressed.

See also QEvent::ignore(), QEvent::accept(), event(), setTabletTracking(), and QTabletEvent.
*/
impl /*struct*/ QWidget {
  pub fn tabletEvent_0<RetType, T: QWidget_tabletEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabletEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_tabletEvent_0<RetType> {
  fn tabletEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_tabletEvent_0<(/*void*/)> for (usize) {
  fn tabletEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget11tabletEventEP12QTabletEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:636
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void actionEvent(QActionEvent *)

/*
This event handler is called with the given event whenever the widget's actions are changed.

See also addAction(), insertAction(), removeAction(), actions(), and QActionEvent.
*/
impl /*struct*/ QWidget {
  pub fn actionEvent_0<RetType, T: QWidget_actionEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_actionEvent_0<RetType> {
  fn actionEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_actionEvent_0<(/*void*/)> for (usize) {
  fn actionEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget11actionEventEP12QActionEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:640
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragEnterEvent(QDragEnterEvent *)

/*
This event handler is called when a drag is in progress and the mouse enters this widget. The event is passed in the event parameter.

If the event is ignored, the widget won't receive any drag move events.

See the Drag-and-drop documentation for an overview of how to provide drag-and-drop in your application.

See also QDrag and QDragEnterEvent.
*/
impl /*struct*/ QWidget {
  pub fn dragEnterEvent_0<RetType, T: QWidget_dragEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragEnterEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_dragEnterEvent_0<RetType> {
  fn dragEnterEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_dragEnterEvent_0<(/*void*/)> for (usize) {
  fn dragEnterEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget14dragEnterEventEP15QDragEnterEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:641
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragMoveEvent(QDragMoveEvent *)

/*
This event handler is called if a drag is in progress, and when any of the following conditions occur: the cursor enters this widget, the cursor moves within this widget, or a modifier key is pressed on the keyboard while this widget has the focus. The event is passed in the event parameter.

See the Drag-and-drop documentation for an overview of how to provide drag-and-drop in your application.

See also QDrag and QDragMoveEvent.
*/
impl /*struct*/ QWidget {
  pub fn dragMoveEvent_0<RetType, T: QWidget_dragMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMoveEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_dragMoveEvent_0<RetType> {
  fn dragMoveEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_dragMoveEvent_0<(/*void*/)> for (usize) {
  fn dragMoveEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget13dragMoveEventEP14QDragMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:642
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragLeaveEvent(QDragLeaveEvent *)

/*
This event handler is called when a drag is in progress and the mouse leaves this widget. The event is passed in the event parameter.

See the Drag-and-drop documentation for an overview of how to provide drag-and-drop in your application.

See also QDrag and QDragLeaveEvent.
*/
impl /*struct*/ QWidget {
  pub fn dragLeaveEvent_0<RetType, T: QWidget_dragLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_dragLeaveEvent_0<RetType> {
  fn dragLeaveEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_dragLeaveEvent_0<(/*void*/)> for (usize) {
  fn dragLeaveEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget14dragLeaveEventEP15QDragLeaveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:643
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QDropEvent *)

/*
This event handler is called when the drag is dropped on this widget. The event is passed in the event parameter.

See the Drag-and-drop documentation for an overview of how to provide drag-and-drop in your application.

See also QDrag and QDropEvent.
*/
impl /*struct*/ QWidget {
  pub fn dropEvent_0<RetType, T: QWidget_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget9dropEventEP10QDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:646
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
This event handler can be reimplemented in a subclass to receive widget show events which are passed in the event parameter.

Non-spontaneous show events are sent to widgets immediately before they are shown. The spontaneous show events of windows are delivered afterwards.

Note: A widget receives spontaneous show and hide events when its mapping status is changed by the window system, e.g. a spontaneous hide event when the user minimizes the window, and a spontaneous show event when the window is restored again. After receiving a spontaneous hide event, a widget is still considered visible in the sense of isVisible().

See also visible, event(), and QShowEvent.
*/
impl /*struct*/ QWidget {
  pub fn showEvent_0<RetType, T: QWidget_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:647
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hideEvent(QHideEvent *)

/*
This event handler can be reimplemented in a subclass to receive widget hide events. The event is passed in the event parameter.

Hide events are sent to widgets immediately after they have been hidden.

Note: A widget receives spontaneous show and hide events when its mapping status is changed by the window system, e.g. a spontaneous hide event when the user minimizes the window, and a spontaneous show event when the window is restored again. After receiving a spontaneous hide event, a widget is still considered visible in the sense of isVisible().

See also visible, event(), and QHideEvent.
*/
impl /*struct*/ QWidget {
  pub fn hideEvent_0<RetType, T: QWidget_hideEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_hideEvent_0<RetType> {
  fn hideEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_hideEvent_0<(/*void*/)> for (usize) {
  fn hideEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget9hideEventEP10QHideEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:648
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool nativeEvent(const QByteArray &, void *, long *)

/*
This special event handler can be reimplemented in a subclass to receive native platform events identified by eventType which are passed in the message parameter.

In your reimplementation of this function, if you want to stop the event being handled by Qt, return true and set result. If you return false, this native event is passed back to Qt, which translates the event into a Qt event and sends it to the widget.

Note: Events are only delivered to this event handler if the widget is has a native Window handle.

Note: This function superseedes the event filter functions x11Event(), winEvent() and macEvent() of Qt 4.


 PlatformEvent Type IdentifierMessage TypeResult Type
Windows"windows_generic_MSG"MSG *LRESULT
macOS"NSEvent"NSEvent *
*/
impl /*struct*/ QWidget {
  pub fn nativeEvent_0<RetType, T: QWidget_nativeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nativeEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_nativeEvent_0<RetType> {
  fn nativeEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_nativeEvent_0<bool> for (usize,usize,usize) {
  fn nativeEvent_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWidget11nativeEventERK10QByteArrayPvPl", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:651
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
This event handler can be reimplemented to handle state changes.

The state being changed in this event can be retrieved through the event supplied.

Change events include: QEvent::ToolBarChange, QEvent::ActivationChange, QEvent::EnabledChange, QEvent::FontChange, QEvent::StyleChange, QEvent::PaletteChange, QEvent::WindowTitleChange, QEvent::IconTextChange, QEvent::ModifiedChange, QEvent::MouseTrackingChange, QEvent::ParentChange, QEvent::WindowStateChange, QEvent::LanguageChange, QEvent::LocaleChange, QEvent::LayoutDirectionChange, QEvent::ReadOnlyChange.
*/
impl /*struct*/ QWidget {
  pub fn changeEvent_0<RetType, T: QWidget_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:653
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int metric(QPaintDevice::PaintDeviceMetric) const

/*
Reimplemented from QPaintDevice::metric().

Internal implementation of the virtual QPaintDevice::metric() function.

m is the metric to get.
*/
impl /*struct*/ QWidget {
  pub fn metric_0<RetType, T: QWidget_metric_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metric_0(self);
    // return 1;
  }
}
pub trait QWidget_metric_0<RetType> {
  fn metric_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_metric_0<i32> for (i32) {
  fn metric_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget6metricEN12QPaintDevice17PaintDeviceMetricE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:654
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void initPainter(QPainter *) const

/*
Initializes the painter pen, background and font to the same as the given widget's. This function is called automatically when the painter is opened on a QWidget.
*/
impl /*struct*/ QWidget {
  pub fn initPainter_0<RetType, T: QWidget_initPainter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initPainter_0(self);
    // return 1;
  }
}
pub trait QWidget_initPainter_0<RetType> {
  fn initPainter_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_initPainter_0<(/*void*/)> for (usize) {
  fn initPainter_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK7QWidget11initPainterEP8QPainter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:655
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QPaintDevice * redirected(QPoint *) const

/*

*/
impl /*struct*/ QWidget {
  pub fn redirected_0<RetType, T: QWidget_redirected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redirected_0(self);
    // return 1;
  }
}
pub trait QWidget_redirected_0<RetType> {
  fn redirected_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_redirected_0<usize> for (usize) {
  fn redirected_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget10redirectedEP6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:656
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QPainter * sharedPainter() const

/*

*/
impl /*struct*/ QWidget {
  pub fn sharedPainter_0<RetType, T: QWidget_sharedPainter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sharedPainter_0(self);
    // return 1;
  }
}
pub trait QWidget_sharedPainter_0<RetType> {
  fn sharedPainter_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_sharedPainter_0<usize> for () {
  fn sharedPainter_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget13sharedPainterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:658
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void inputMethodEvent(QInputMethodEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive Input Method composition events. This handler is called when the state of the input method changes.

Note that when creating custom text editing widgets, the Qt::WA_InputMethodEnabled window attribute must be set explicitly (using the setAttribute() function) in order to receive input method events.

The default implementation calls event->ignore(), which rejects the Input Method event. See the QInputMethodEvent documentation for more details.

See also event() and QInputMethodEvent.
*/
impl /*struct*/ QWidget {
  pub fn inputMethodEvent_0<RetType, T: QWidget_inputMethodEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodEvent_0(self);
    // return 1;
  }
}
pub trait QWidget_inputMethodEvent_0<RetType> {
  fn inputMethodEvent_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_inputMethodEvent_0<(/*void*/)> for (usize) {
  fn inputMethodEvent_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget16inputMethodEventEP17QInputMethodEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:660
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery) const

/*
This method is only relevant for input widgets. It is used by the input method to query a set of properties of the widget to be able to support complex input method operations as support for surrounding text and reconversions.

query specifies which property is queried.

See also inputMethodEvent(), QInputMethodEvent, QInputMethodQueryEvent, and inputMethodHints.
*/
impl /*struct*/ QWidget {
  pub fn inputMethodQuery_0<RetType, T: QWidget_inputMethodQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_0(self);
    // return 1;
  }
}
pub trait QWidget_inputMethodQuery_0<RetType> {
  fn inputMethodQuery_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_inputMethodQuery_0<usize> for (i32) {
  fn inputMethodQuery_0(self , rsthis: & QWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget16inputMethodQueryEN2Qt16InputMethodQueryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:662
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::InputMethodHints inputMethodHints() const

/*

*/
impl /*struct*/ QWidget {
  pub fn inputMethodHints_0<RetType, T: QWidget_inputMethodHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodHints_0(self);
    // return 1;
  }
}
pub trait QWidget_inputMethodHints_0<RetType> {
  fn inputMethodHints_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_inputMethodHints_0<i32> for () {
  fn inputMethodHints_0(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWidget16inputMethodHintsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:663
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInputMethodHints(Qt::InputMethodHints)

/*

*/
impl /*struct*/ QWidget {
  pub fn setInputMethodHints_0<RetType, T: QWidget_setInputMethodHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInputMethodHints_0(self);
    // return 1;
  }
}
pub trait QWidget_setInputMethodHints_0<RetType> {
  fn setInputMethodHints_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_setInputMethodHints_0<(/*void*/)> for (i32) {
  fn setInputMethodHints_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget19setInputMethodHintsE6QFlagsIN2Qt15InputMethodHintEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:666
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void updateMicroFocus()

/*
Updates the widget's micro focus.
*/
impl /*struct*/ QWidget {
  pub fn updateMicroFocus_0<RetType, T: QWidget_updateMicroFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateMicroFocus_0(self);
    // return 1;
  }
}
pub trait QWidget_updateMicroFocus_0<RetType> {
  fn updateMicroFocus_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_updateMicroFocus_0<(/*void*/)> for () {
  fn updateMicroFocus_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWidget16updateMicroFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:669
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void create(WId, bool, bool)

/*
Creates a new widget window.

The parameter window is ignored in Qt 5. Please use QWindow::fromWinId() to create a QWindow wrapping a foreign window and pass it to QWidget::createWindowContainer() instead.

Initializes the window (sets the geometry etc.) if initializeWindow is true. If initializeWindow is false, no initialization is performed. This parameter only makes sense if window is a valid window.

Destroys the old window if destroyOldWindow is true. If destroyOldWindow is false, you are responsible for destroying the window yourself (using platform native code).

The QWidget constructor calls create(0,true,true) to create a window for this widget.

See also createWindowContainer() and QWindow::fromWinId().
*/
impl /*struct*/ QWidget {
  pub fn create_0<RetType, T: QWidget_create_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.create_0(self);
    // return 1;
  }
}
pub trait QWidget_create_0<RetType> {
  fn create_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_create_0<(/*void*/)> for (u64,bool,bool) {
  fn create_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u64 as usize;
    let arg1 = (&self.1) as *const bool as usize;
    let arg2 = (&self.2) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget6createEybb", 3,qtrt::FFITY_UINT64,qtrt::FFITY_SINT8,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:671
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void destroy(bool, bool)

/*
Frees up window system resources. Destroys the widget window if destroyWindow is true.

destroy() calls itself recursively for all the child widgets, passing destroySubWindows for the destroyWindow parameter. To have more control over destruction of subwidgets, destroy subwidgets selectively first.

This function is usually called from the QWidget destructor.
*/
impl /*struct*/ QWidget {
  pub fn destroy_0<RetType, T: QWidget_destroy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.destroy_0(self);
    // return 1;
  }
}
pub trait QWidget_destroy_0<RetType> {
  fn destroy_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_destroy_0<(/*void*/)> for (bool,bool) {
  fn destroy_0(self , rsthis: & QWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const bool as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWidget7destroyEbb", 2,qtrt::FFITY_SINT8,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:675
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool focusNextPrevChild(bool)

/*
Finds a new widget to give the keyboard focus to, as appropriate for Tab and Shift+Tab, and returns true if it can find a new widget, or false if it can't.

If next is true, this function searches forward, if next is false, it searches backward.

Sometimes, you will want to reimplement this function. For example, a web browser might reimplement it to move its "current active link" forward or backward, and call focusNextPrevChild() only when it reaches the last or first link on the "page".

Child widgets call focusNextPrevChild() on their parent widgets, but only the window that contains the child widgets decides where to redirect focus. By reimplementing this function for an object, you thus gain control of focus traversal for all child widgets.

See also focusNextChild() and focusPreviousChild().
*/
impl /*struct*/ QWidget {
  pub fn focusNextPrevChild_0<RetType, T: QWidget_focusNextPrevChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild_0(self);
    // return 1;
  }
}
pub trait QWidget_focusNextPrevChild_0<RetType> {
  fn focusNextPrevChild_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_focusNextPrevChild_0<bool> for (bool) {
  fn focusNextPrevChild_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWidget18focusNextPrevChildEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:676
// index:0
// Protected inline Visibility=Default Availability=Available
// [1] bool focusNextChild()

/*
Finds a new widget to give the keyboard focus to, as appropriate for Tab, and returns true if it can find a new widget, or false if it can't.

See also focusPreviousChild().
*/
impl /*struct*/ QWidget {
  pub fn focusNextChild_0<RetType, T: QWidget_focusNextChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextChild_0(self);
    // return 1;
  }
}
pub trait QWidget_focusNextChild_0<RetType> {
  fn focusNextChild_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_focusNextChild_0<bool> for () {
  fn focusNextChild_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWidget14focusNextChildEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidget.h:677
// index:0
// Protected inline Visibility=Default Availability=Available
// [1] bool focusPreviousChild()

/*
Finds a new widget to give the keyboard focus to, as appropriate for Shift+Tab, and returns true if it can find a new widget, or false if it can't.

See also focusNextChild().
*/
impl /*struct*/ QWidget {
  pub fn focusPreviousChild_0<RetType, T: QWidget_focusPreviousChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusPreviousChild_0(self);
    // return 1;
  }
}
pub trait QWidget_focusPreviousChild_0<RetType> {
  fn focusPreviousChild_0(self , rsthis: & QWidget) -> RetType;
}
impl<'a> /*trait*/ QWidget_focusPreviousChild_0<bool> for () {
  fn focusPreviousChild_0(self , rsthis: & QWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWidget18focusPreviousChildEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


*/
pub type QWidget__RenderFlag = i32;
// 
pub const QWidget__DrawWindowBackground :QWidget__RenderFlag = 1;
// 
pub const QWidget__DrawChildren :QWidget__RenderFlag = 2;
// 
pub const QWidget__IgnoreMask :QWidget__RenderFlag = 4;
pub fn QWidget_RenderFlagItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QWidget", val);
}
pub fn QWidget_RenderFlagItemName_s(val: i32) ->String {
  //var nilthis *QWidget
  //return nilthis.RenderFlagItemName(val);
  return QWidget_RenderFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
