

// mod ::widgets::QMdiSubWindow
// package qtwidgets
// /usr/include/qt/QtWidgets/qmdisubwindow.h
// #include <qmdisubwindow.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 45
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

// bool eventFilter(QObject *, QEvent *)
// func (this *QMdiSubWindow) InheritEventFilter(f func(object *qtcore.QObject/*777 QObject **/, event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// bool event(QEvent *)
// func (this *QMdiSubWindow) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void showEvent(QShowEvent *)
// func (this *QMdiSubWindow) InheritShowEvent(f func(showEvent *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void hideEvent(QHideEvent *)
// func (this *QMdiSubWindow) InheritHideEvent(f func(hideEvent *qtgui.QHideEvent/*777 QHideEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hideEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QMdiSubWindow) InheritChangeEvent(f func(changeEvent *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void closeEvent(QCloseEvent *)
// func (this *QMdiSubWindow) InheritCloseEvent(f func(closeEvent *qtgui.QCloseEvent/*777 QCloseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "closeEvent", f)
// }

// void leaveEvent(QEvent *)
// func (this *QMdiSubWindow) InheritLeaveEvent(f func(leaveEvent *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "leaveEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QMdiSubWindow) InheritResizeEvent(f func(resizeEvent *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QMdiSubWindow) InheritTimerEvent(f func(timerEvent *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// void moveEvent(QMoveEvent *)
// func (this *QMdiSubWindow) InheritMoveEvent(f func(moveEvent *qtgui.QMoveEvent/*777 QMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "moveEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QMdiSubWindow) InheritPaintEvent(f func(paintEvent *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QMdiSubWindow) InheritMousePressEvent(f func(mouseEvent *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseDoubleClickEvent(QMouseEvent *)
// func (this *QMdiSubWindow) InheritMouseDoubleClickEvent(f func(mouseEvent *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QMdiSubWindow) InheritMouseReleaseEvent(f func(mouseEvent *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QMdiSubWindow) InheritMouseMoveEvent(f func(mouseEvent *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QMdiSubWindow) InheritKeyPressEvent(f func(keyEvent *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void contextMenuEvent(QContextMenuEvent *)
// func (this *QMdiSubWindow) InheritContextMenuEvent(f func(contextMenuEvent *qtgui.QContextMenuEvent/*777 QContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QMdiSubWindow) InheritFocusInEvent(f func(focusInEvent *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QMdiSubWindow) InheritFocusOutEvent(f func(focusOutEvent *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void childEvent(QChildEvent *)
// func (this *QMdiSubWindow) InheritChildEvent(f func(childEvent *qtcore.QChildEvent/*777 QChildEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "childEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QMdiSubWindow)=48
pub struct QMdiSubWindow {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMdiSubWindow_ITF interface {
//    QWidget_ITF
//    QMdiSubWindow_PTR() *QMdiSubWindow
//}
//func (ptr *QMdiSubWindow) QMdiSubWindow_PTR() *QMdiSubWindow { return ptr }

impl /*struct*/ QMdiSubWindow {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMdiSubWindow {
    return QMdiSubWindow{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMdiSubWindow {
//  type Target = QMdiSubWindowBASE;
//
//  fn deref(&self) -> &QMdiSubWindowBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMdiSubWindowBASE> for QMdiSubWindow {
//  fn as_ref(& self) -> & QMdiSubWindowBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qmdisubwindow.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QMdiSubWindow {
  pub fn metaObject_0<RetType, T: QMdiSubWindow_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QMdiSubWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMdiSubWindow10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMdiSubWindow(QWidget *, Qt::WindowFlags)

/*
Constructs a new QMdiSubWindow widget. The parent and flags arguments are passed to QWidget's constructor.

Instead of using addSubWindow(), it is also simply possible to use setParent() when you add the subwindow to a QMdiArea.

Note that only QMdiSubWindows can be set as children of QMdiArea; you cannot, for instance, write:


  //bad code
  QMdiArea mdiArea;
  QTextEdit editor(&mdiArea); // invalid child widget



See also QMdiArea::addSubWindow().
*/
// QMdiSubWindow(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QMdiSubWindow {
  pub fn QMdiSubWindow_0<T: QMdiSubWindow_QMdiSubWindow_0>(value: T) -> QMdiSubWindow {
    let rsthis = value.QMdiSubWindow_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMdiSubWindow_QMdiSubWindow_0 {
  fn QMdiSubWindow_0(self) -> QMdiSubWindow;
}
// QMdiSubWindow(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMdiSubWindow_QMdiSubWindow_0 for (usize,i32) {
  fn QMdiSubWindow_0(self) -> QMdiSubWindow {
    // unsafe{_ZN13QMdiSubWindowC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QMdiSubWindowC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMdiSubWindow{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:70
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QMdiSubWindow()

/*

*/
pub fn DeleteQMdiSubWindow(this :*mut QMdiSubWindow) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QMdiSubWindowD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qmdisubwindow.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn sizeHint_0<RetType, T: QMdiSubWindow_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QMdiSubWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMdiSubWindow8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:73
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn minimumSizeHint_0<RetType, T: QMdiSubWindow_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QMdiSubWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMdiSubWindow15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidget(QWidget *)

/*
Sets widget as the internal widget of this subwindow. The internal widget is displayed in the center of the subwindow beneath the title bar.

QMdiSubWindow takes temporary ownership of widget; you do not have to delete it. Any existing internal widget will be removed and reparented to the root window.

See also widget().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn setWidget_0<RetType, T: QMdiSubWindow_setWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidget_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_setWidget_0<RetType> {
  fn setWidget_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_setWidget_0<(/*void*/)> for (usize) {
  fn setWidget_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow9setWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widget() const

/*
Returns the current internal widget.

See also setWidget().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn widget_0<RetType, T: QMdiSubWindow_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_widget_0<RetType> {
  fn widget_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_widget_0<usize> for () {
  fn widget_0(self , rsthis: & QMdiSubWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMdiSubWindow6widgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * maximizedButtonsWidget() const

/*

*/
impl /*struct*/ QMdiSubWindow {
  pub fn maximizedButtonsWidget_0<RetType, T: QMdiSubWindow_maximizedButtonsWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximizedButtonsWidget_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_maximizedButtonsWidget_0<RetType> {
  fn maximizedButtonsWidget_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_maximizedButtonsWidget_0<usize> for () {
  fn maximizedButtonsWidget_0(self , rsthis: & QMdiSubWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMdiSubWindow22maximizedButtonsWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * maximizedSystemMenuIconWidget() const

/*

*/
impl /*struct*/ QMdiSubWindow {
  pub fn maximizedSystemMenuIconWidget_0<RetType, T: QMdiSubWindow_maximizedSystemMenuIconWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximizedSystemMenuIconWidget_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_maximizedSystemMenuIconWidget_0<RetType> {
  fn maximizedSystemMenuIconWidget_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_maximizedSystemMenuIconWidget_0<usize> for () {
  fn maximizedSystemMenuIconWidget_0(self , rsthis: & QMdiSubWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMdiSubWindow29maximizedSystemMenuIconWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:81
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isShaded() const

/*
Returns true if this window is shaded; otherwise returns false.

A window is shaded if it is collapsed so that only the title bar is visible.
*/
impl /*struct*/ QMdiSubWindow {
  pub fn isShaded_0<RetType, T: QMdiSubWindow_isShaded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isShaded_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_isShaded_0<RetType> {
  fn isShaded_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_isShaded_0<bool> for () {
  fn isShaded_0(self , rsthis: & QMdiSubWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMdiSubWindow8isShadedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOption(QMdiSubWindow::SubWindowOption, bool)

/*
If on is true, option is enabled on the subwindow; otherwise it is disabled. See SubWindowOption for the effect of each option.

See also SubWindowOption and testOption().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn setOption_0<RetType, T: QMdiSubWindow_setOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOption_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_setOption_0<RetType> {
  fn setOption_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_setOption_0<(/*void*/)> for (i32,bool) {
  fn setOption_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow9setOptionENS_15SubWindowOptionEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:84
// index:0
// Public Visibility=Default Availability=Available
// [1] bool testOption(QMdiSubWindow::SubWindowOption) const

/*
Returns true if option is enabled; otherwise returns false.

See also SubWindowOption and setOption().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn testOption_0<RetType, T: QMdiSubWindow_testOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testOption_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_testOption_0<RetType> {
  fn testOption_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_testOption_0<bool> for (i32) {
  fn testOption_0(self , rsthis: & QMdiSubWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMdiSubWindow10testOptionENS_15SubWindowOptionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setKeyboardSingleStep(int)

/*

*/
impl /*struct*/ QMdiSubWindow {
  pub fn setKeyboardSingleStep_0<RetType, T: QMdiSubWindow_setKeyboardSingleStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKeyboardSingleStep_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_setKeyboardSingleStep_0<RetType> {
  fn setKeyboardSingleStep_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_setKeyboardSingleStep_0<(/*void*/)> for (i32) {
  fn setKeyboardSingleStep_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow21setKeyboardSingleStepEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:87
// index:0
// Public Visibility=Default Availability=Available
// [4] int keyboardSingleStep() const

/*

*/
impl /*struct*/ QMdiSubWindow {
  pub fn keyboardSingleStep_0<RetType, T: QMdiSubWindow_keyboardSingleStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyboardSingleStep_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_keyboardSingleStep_0<RetType> {
  fn keyboardSingleStep_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_keyboardSingleStep_0<i32> for () {
  fn keyboardSingleStep_0(self , rsthis: & QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMdiSubWindow18keyboardSingleStepEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setKeyboardPageStep(int)

/*

*/
impl /*struct*/ QMdiSubWindow {
  pub fn setKeyboardPageStep_0<RetType, T: QMdiSubWindow_setKeyboardPageStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKeyboardPageStep_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_setKeyboardPageStep_0<RetType> {
  fn setKeyboardPageStep_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_setKeyboardPageStep_0<(/*void*/)> for (i32) {
  fn setKeyboardPageStep_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow19setKeyboardPageStepEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:90
// index:0
// Public Visibility=Default Availability=Available
// [4] int keyboardPageStep() const

/*

*/
impl /*struct*/ QMdiSubWindow {
  pub fn keyboardPageStep_0<RetType, T: QMdiSubWindow_keyboardPageStep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyboardPageStep_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_keyboardPageStep_0<RetType> {
  fn keyboardPageStep_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_keyboardPageStep_0<i32> for () {
  fn keyboardPageStep_0(self , rsthis: & QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMdiSubWindow16keyboardPageStepEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSystemMenu(QMenu *)

/*
Sets systemMenu as the current system menu for this subwindow.

By default, each QMdiSubWindow has a standard system menu.

QActions for the system menu created by QMdiSubWindow will automatically be updated depending on the current window state; e.g., the minimize action will be disabled after the window is minimized.

QActions added by the user are not updated by QMdiSubWindow.

QMdiSubWindow takes ownership of systemMenu; you do not have to delete it. Any existing menus will be deleted.

See also systemMenu() and showSystemMenu().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn setSystemMenu_0<RetType, T: QMdiSubWindow_setSystemMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSystemMenu_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_setSystemMenu_0<RetType> {
  fn setSystemMenu_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_setSystemMenu_0<(/*void*/)> for (usize) {
  fn setSystemMenu_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow13setSystemMenuEP5QMenu", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:94
// index:0
// Public Visibility=Default Availability=Available
// [8] QMenu * systemMenu() const

/*
Returns a pointer to the current system menu, or zero if no system menu is set. QMdiSubWindow provides a default system menu, but you can also set the menu with setSystemMenu().

See also setSystemMenu() and showSystemMenu().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn systemMenu_0<RetType, T: QMdiSubWindow_systemMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.systemMenu_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_systemMenu_0<RetType> {
  fn systemMenu_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_systemMenu_0<usize> for () {
  fn systemMenu_0(self , rsthis: & QMdiSubWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMdiSubWindow10systemMenuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] QMdiArea * mdiArea() const

/*
Returns the area containing this sub-window, or 0 if there is none.

This function was introduced in  Qt 4.4.

See also QMdiArea::addSubWindow().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn mdiArea_0<RetType, T: QMdiSubWindow_mdiArea_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mdiArea_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_mdiArea_0<RetType> {
  fn mdiArea_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_mdiArea_0<usize> for () {
  fn mdiArea_0(self , rsthis: & QMdiSubWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMdiSubWindow7mdiAreaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void windowStateChanged(Qt::WindowStates, Qt::WindowStates)

/*
QMdiSubWindow emits this signal after the window state changes. oldState is the window state before it changed, and newState is the new, current state.
*/
impl /*struct*/ QMdiSubWindow {
  pub fn windowStateChanged_0<RetType, T: QMdiSubWindow_windowStateChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowStateChanged_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_windowStateChanged_0<RetType> {
  fn windowStateChanged_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_windowStateChanged_0<(/*void*/)> for (i32,i32) {
  fn windowStateChanged_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow18windowStateChangedE6QFlagsIN2Qt11WindowStateEES3_", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void aboutToActivate()

/*
QMdiSubWindow emits this signal immediately before it is activated. After the subwindow has been activated, the QMdiArea that manages the subwindow will also emit the subWindowActivated() signal.

See also QMdiArea::subWindowActivated().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn aboutToActivate_0<RetType, T: QMdiSubWindow_aboutToActivate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.aboutToActivate_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_aboutToActivate_0<RetType> {
  fn aboutToActivate_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_aboutToActivate_0<(/*void*/)> for () {
  fn aboutToActivate_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow15aboutToActivateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showSystemMenu()

/*
Shows the system menu below the system menu icon in the title bar.

See also setSystemMenu() and systemMenu().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn showSystemMenu_0<RetType, T: QMdiSubWindow_showSystemMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showSystemMenu_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_showSystemMenu_0<RetType> {
  fn showSystemMenu_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_showSystemMenu_0<(/*void*/)> for () {
  fn showSystemMenu_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow14showSystemMenuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showShaded()

/*
Calling this function makes the subwindow enter the shaded mode. When the subwindow is shaded, only the title bar is visible.

Although shading is not supported by all styles, this function will still show the subwindow as shaded, regardless of whether support for shading is available. However, when used with styles without shading support, the user will be unable to return from shaded mode through the user interface (e.g., through a shade button in the title bar).

See also isShaded().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn showShaded_0<RetType, T: QMdiSubWindow_showShaded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showShaded_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_showShaded_0<RetType> {
  fn showShaded_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_showShaded_0<(/*void*/)> for () {
  fn showShaded_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow10showShadedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:110
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn eventFilter_0<RetType, T: QMdiSubWindow_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QMdiSubWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:111
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn event_0<RetType, T: QMdiSubWindow_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_event_0<RetType> {
  fn event_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QMdiSubWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:112
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn showEvent_0<RetType, T: QMdiSubWindow_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:113
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hideEvent(QHideEvent *)

/*
Reimplemented from QWidget::hideEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn hideEvent_0<RetType, T: QMdiSubWindow_hideEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_hideEvent_0<RetType> {
  fn hideEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_hideEvent_0<(/*void*/)> for (usize) {
  fn hideEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow9hideEventEP10QHideEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:114
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn changeEvent_0<RetType, T: QMdiSubWindow_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:115
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void closeEvent(QCloseEvent *)

/*
Reimplemented from QWidget::closeEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn closeEvent_0<RetType, T: QMdiSubWindow_closeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_closeEvent_0<RetType> {
  fn closeEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_closeEvent_0<(/*void*/)> for (usize) {
  fn closeEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow10closeEventEP11QCloseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:116
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void leaveEvent(QEvent *)

/*
Reimplemented from QWidget::leaveEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn leaveEvent_0<RetType, T: QMdiSubWindow_leaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leaveEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_leaveEvent_0<RetType> {
  fn leaveEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_leaveEvent_0<(/*void*/)> for (usize) {
  fn leaveEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow10leaveEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:117
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().

Warning: When maximizing or restoring a subwindow, the resulting call to this function may have an invalid QResizeEvent::oldSize().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn resizeEvent_0<RetType, T: QMdiSubWindow_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:118
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn timerEvent_0<RetType, T: QMdiSubWindow_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:119
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void moveEvent(QMoveEvent *)

/*
Reimplemented from QWidget::moveEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn moveEvent_0<RetType, T: QMdiSubWindow_moveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_moveEvent_0<RetType> {
  fn moveEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_moveEvent_0<(/*void*/)> for (usize) {
  fn moveEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow9moveEventEP10QMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:120
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn paintEvent_0<RetType, T: QMdiSubWindow_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:121
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn mousePressEvent_0<RetType, T: QMdiSubWindow_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:122
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseDoubleClickEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn mouseDoubleClickEvent_0<RetType, T: QMdiSubWindow_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow21mouseDoubleClickEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:123
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn mouseReleaseEvent_0<RetType, T: QMdiSubWindow_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:124
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn mouseMoveEvent_0<RetType, T: QMdiSubWindow_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:125
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn keyPressEvent_0<RetType, T: QMdiSubWindow_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:127
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QContextMenuEvent *)

/*
Reimplemented from QWidget::contextMenuEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn contextMenuEvent_0<RetType, T: QMdiSubWindow_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow16contextMenuEventEP17QContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:129
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn focusInEvent_0<RetType, T: QMdiSubWindow_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:130
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusOutEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn focusOutEvent_0<RetType, T: QMdiSubWindow_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmdisubwindow.h:131
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void childEvent(QChildEvent *)

/*
Reimplemented from QObject::childEvent().
*/
impl /*struct*/ QMdiSubWindow {
  pub fn childEvent_0<RetType, T: QMdiSubWindow_childEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childEvent_0(self);
    // return 1;
  }
}
pub trait QMdiSubWindow_childEvent_0<RetType> {
  fn childEvent_0(self , rsthis: & QMdiSubWindow) -> RetType;
}
impl<'a> /*trait*/ QMdiSubWindow_childEvent_0<(/*void*/)> for (usize) {
  fn childEvent_0(self , rsthis: & QMdiSubWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QMdiSubWindow10childEventEP11QChildEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QMdiSubWindow__SubWindowOption = i32;
// 
pub const QMdiSubWindow__AllowOutsideAreaHorizontally :QMdiSubWindow__SubWindowOption = 1;
// 
pub const QMdiSubWindow__AllowOutsideAreaVertically :QMdiSubWindow__SubWindowOption = 2;
// 
pub const QMdiSubWindow__RubberBandResize :QMdiSubWindow__SubWindowOption = 4;
// 
pub const QMdiSubWindow__RubberBandMove :QMdiSubWindow__SubWindowOption = 8;
pub fn QMdiSubWindow_SubWindowOptionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QMdiSubWindow", val);
}
pub fn QMdiSubWindow_SubWindowOptionItemName_s(val: i32) ->String {
  //var nilthis *QMdiSubWindow
  //return nilthis.SubWindowOptionItemName(val);
  return QMdiSubWindow_SubWindowOptionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
