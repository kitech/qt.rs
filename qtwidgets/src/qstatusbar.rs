

// mod ::widgets::QStatusBar
// package qtwidgets
// /usr/include/qt/QtWidgets/qstatusbar.h
// #include <qstatusbar.h>
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

// void showEvent(QShowEvent *)
// func (this *QStatusBar) InheritShowEvent(f func(arg0 *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QStatusBar) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QStatusBar) InheritResizeEvent(f func(arg0 *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void reformat()
// func (this *QStatusBar) InheritReformat(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "reformat", f)
// }

// void hideOrShow()
// func (this *QStatusBar) InheritHideOrShow(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hideOrShow", f)
// }

// bool event(QEvent *)
// func (this *QStatusBar) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QStatusBar)=48
pub struct QStatusBar {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStatusBar_ITF interface {
//    QWidget_ITF
//    QStatusBar_PTR() *QStatusBar
//}
//func (ptr *QStatusBar) QStatusBar_PTR() *QStatusBar { return ptr }

impl /*struct*/ QStatusBar {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStatusBar {
    return QStatusBar{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStatusBar {
//  type Target = QStatusBarBASE;
//
//  fn deref(&self) -> &QStatusBarBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStatusBarBASE> for QStatusBar {
//  fn as_ref(& self) -> & QStatusBarBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstatusbar.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QStatusBar {
  pub fn metaObject_0<RetType, T: QStatusBar_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QStatusBar_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QStatusBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStatusBar10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStatusBar(QWidget *)

/*
Constructs a status bar with a size grip and the given parent.

See also setSizeGripEnabled().
*/
// QStatusBar(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QStatusBar {
  pub fn QStatusBar_0<T: QStatusBar_QStatusBar_0>(value: T) -> QStatusBar {
    let rsthis = value.QStatusBar_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStatusBar_QStatusBar_0 {
  fn QStatusBar_0(self) -> QStatusBar;
}
// QStatusBar(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStatusBar_QStatusBar_0 for (usize) {
  fn QStatusBar_0(self) -> QStatusBar {
    // unsafe{_ZN10QStatusBarC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QStatusBarC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStatusBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QStatusBar()

/*

*/
pub fn DeleteQStatusBar(this :*mut QStatusBar) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QStatusBarD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qstatusbar.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addWidget(QWidget *, int)

/*
Adds the given widget to this status bar, reparenting the widget if it isn't already a child of this QStatusBar object. The stretch parameter is used to compute a suitable size for the given widget as the status bar grows and shrinks. The default stretch factor is 0, i.e giving the widget a minimum of space.

The widget is located to the far left of the first permanent widget (see addPermanentWidget()) and may be obscured by temporary messages.

See also insertWidget(), removeWidget(), and addPermanentWidget().
*/
impl /*struct*/ QStatusBar {
  pub fn addWidget_0<RetType, T: QStatusBar_addWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addWidget_0(self);
    // return 1;
  }
}
pub trait QStatusBar_addWidget_0<RetType> {
  fn addWidget_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_addWidget_0<(/*void*/)> for (usize,i32) {
  fn addWidget_0(self , rsthis: & QStatusBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QStatusBar9addWidgetEP7QWidgeti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:63
// index:0
// Public Visibility=Default Availability=Available
// [4] int insertWidget(int, QWidget *, int)

/*
Inserts the given widget at the given index to this status bar, reparenting the widget if it isn't already a child of this QStatusBar object. If index is out of range, the widget is appended (in which case it is the actual index of the widget that is returned).

The stretch parameter is used to compute a suitable size for the given widget as the status bar grows and shrinks. The default stretch factor is 0, i.e giving the widget a minimum of space.

The widget is located to the far left of the first permanent widget (see addPermanentWidget()) and may be obscured by temporary messages.

This function was introduced in  Qt 4.2.

See also addWidget(), removeWidget(), and addPermanentWidget().
*/
impl /*struct*/ QStatusBar {
  pub fn insertWidget_0<RetType, T: QStatusBar_insertWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertWidget_0(self);
    // return 1;
  }
}
pub trait QStatusBar_insertWidget_0<RetType> {
  fn insertWidget_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_insertWidget_0<i32> for (i32,usize,i32) {
  fn insertWidget_0(self , rsthis: & QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QStatusBar12insertWidgetEiP7QWidgeti", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addPermanentWidget(QWidget *, int)

/*
Adds the given widget permanently to this status bar, reparenting the widget if it isn't already a child of this QStatusBar object. The stretch parameter is used to compute a suitable size for the given widget as the status bar grows and shrinks. The default stretch factor is 0, i.e giving the widget a minimum of space.

Permanently means that the widget may not be obscured by temporary messages. It is is located at the far right of the status bar.

See also insertPermanentWidget(), removeWidget(), and addWidget().
*/
impl /*struct*/ QStatusBar {
  pub fn addPermanentWidget_0<RetType, T: QStatusBar_addPermanentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addPermanentWidget_0(self);
    // return 1;
  }
}
pub trait QStatusBar_addPermanentWidget_0<RetType> {
  fn addPermanentWidget_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_addPermanentWidget_0<(/*void*/)> for (usize,i32) {
  fn addPermanentWidget_0(self , rsthis: & QStatusBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QStatusBar18addPermanentWidgetEP7QWidgeti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:65
// index:0
// Public Visibility=Default Availability=Available
// [4] int insertPermanentWidget(int, QWidget *, int)

/*
Inserts the given widget at the given index permanently to this status bar, reparenting the widget if it isn't already a child of this QStatusBar object. If index is out of range, the widget is appended (in which case it is the actual index of the widget that is returned).

The stretch parameter is used to compute a suitable size for the given widget as the status bar grows and shrinks. The default stretch factor is 0, i.e giving the widget a minimum of space.

Permanently means that the widget may not be obscured by temporary messages. It is is located at the far right of the status bar.

This function was introduced in  Qt 4.2.

See also addPermanentWidget(), removeWidget(), and addWidget().
*/
impl /*struct*/ QStatusBar {
  pub fn insertPermanentWidget_0<RetType, T: QStatusBar_insertPermanentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertPermanentWidget_0(self);
    // return 1;
  }
}
pub trait QStatusBar_insertPermanentWidget_0<RetType> {
  fn insertPermanentWidget_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_insertPermanentWidget_0<i32> for (i32,usize,i32) {
  fn insertPermanentWidget_0(self , rsthis: & QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QStatusBar21insertPermanentWidgetEiP7QWidgeti", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeWidget(QWidget *)

/*
Removes the specified widget from the status bar.

Note: This function does not delete the widget but hides it. To add the widget again, you must call both the addWidget() and show() functions.

See also addWidget(), addPermanentWidget(), and clearMessage().
*/
impl /*struct*/ QStatusBar {
  pub fn removeWidget_0<RetType, T: QStatusBar_removeWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeWidget_0(self);
    // return 1;
  }
}
pub trait QStatusBar_removeWidget_0<RetType> {
  fn removeWidget_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_removeWidget_0<(/*void*/)> for (usize) {
  fn removeWidget_0(self , rsthis: & QStatusBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QStatusBar12removeWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSizeGripEnabled(bool)

/*

*/
impl /*struct*/ QStatusBar {
  pub fn setSizeGripEnabled_0<RetType, T: QStatusBar_setSizeGripEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizeGripEnabled_0(self);
    // return 1;
  }
}
pub trait QStatusBar_setSizeGripEnabled_0<RetType> {
  fn setSizeGripEnabled_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_setSizeGripEnabled_0<(/*void*/)> for (bool) {
  fn setSizeGripEnabled_0(self , rsthis: & QStatusBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QStatusBar18setSizeGripEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:69
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSizeGripEnabled() const

/*

*/
impl /*struct*/ QStatusBar {
  pub fn isSizeGripEnabled_0<RetType, T: QStatusBar_isSizeGripEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSizeGripEnabled_0(self);
    // return 1;
  }
}
pub trait QStatusBar_isSizeGripEnabled_0<RetType> {
  fn isSizeGripEnabled_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_isSizeGripEnabled_0<bool> for () {
  fn isSizeGripEnabled_0(self , rsthis: & QStatusBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStatusBar17isSizeGripEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:71
// index:0
// Public Visibility=Default Availability=Available
// [8] QString currentMessage() const

/*
Returns the temporary message currently shown, or an empty string if there is no such message.

See also showMessage().
*/
impl /*struct*/ QStatusBar {
  pub fn currentMessage_0<RetType, T: QStatusBar_currentMessage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentMessage_0(self);
    // return 1;
  }
}
pub trait QStatusBar_currentMessage_0<RetType> {
  fn currentMessage_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_currentMessage_0<usize> for () {
  fn currentMessage_0(self , rsthis: & QStatusBar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStatusBar14currentMessageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showMessage(const QString &, int)

/*
Hides the normal status indications and displays the given message for the specified number of milli-seconds (timeout). If timeout is 0 (default), the message remains displayed until the clearMessage() slot is called or until the showMessage() slot is called again to change the message.

Note that showMessage() is called to show temporary explanations of tool tip texts, so passing a timeout of 0 is not sufficient to display a permanent message.

See also messageChanged(), currentMessage(), and clearMessage().
*/
impl /*struct*/ QStatusBar {
  pub fn showMessage_0<RetType, T: QStatusBar_showMessage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showMessage_0(self);
    // return 1;
  }
}
pub trait QStatusBar_showMessage_0<RetType> {
  fn showMessage_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_showMessage_0<(/*void*/)> for (usize,i32) {
  fn showMessage_0(self , rsthis: & QStatusBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QStatusBar11showMessageERK7QStringi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearMessage()

/*
Removes any temporary message being shown.

See also currentMessage(), showMessage(), and removeWidget().
*/
impl /*struct*/ QStatusBar {
  pub fn clearMessage_0<RetType, T: QStatusBar_clearMessage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearMessage_0(self);
    // return 1;
  }
}
pub trait QStatusBar_clearMessage_0<RetType> {
  fn clearMessage_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_clearMessage_0<(/*void*/)> for () {
  fn clearMessage_0(self , rsthis: & QStatusBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QStatusBar12clearMessageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void messageChanged(const QString &)

/*
This signal is emitted whenever the temporary status message changes. The new temporary message is passed in the message parameter which is a null-string when the message has been removed.

See also showMessage() and clearMessage().
*/
impl /*struct*/ QStatusBar {
  pub fn messageChanged_0<RetType, T: QStatusBar_messageChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.messageChanged_0(self);
    // return 1;
  }
}
pub trait QStatusBar_messageChanged_0<RetType> {
  fn messageChanged_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_messageChanged_0<(/*void*/)> for (usize) {
  fn messageChanged_0(self , rsthis: & QStatusBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QStatusBar14messageChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:82
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QStatusBar {
  pub fn showEvent_0<RetType, T: QStatusBar_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QStatusBar_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QStatusBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QStatusBar9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:83
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().

Shows the temporary message, if appropriate, in response to the paint event.
*/
impl /*struct*/ QStatusBar {
  pub fn paintEvent_0<RetType, T: QStatusBar_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QStatusBar_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QStatusBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QStatusBar10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:84
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QStatusBar {
  pub fn resizeEvent_0<RetType, T: QStatusBar_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QStatusBar_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QStatusBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QStatusBar11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:87
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void reformat()

/*
Changes the status bar's appearance to account for item changes.

Special subclasses may need this function, but geometry management will usually take care of any necessary rearrangements.
*/
impl /*struct*/ QStatusBar {
  pub fn reformat_0<RetType, T: QStatusBar_reformat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reformat_0(self);
    // return 1;
  }
}
pub trait QStatusBar_reformat_0<RetType> {
  fn reformat_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_reformat_0<(/*void*/)> for () {
  fn reformat_0(self , rsthis: & QStatusBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QStatusBar8reformatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:88
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void hideOrShow()

/*
Ensures that the right widgets are visible.

Used by the showMessage() and clearMessage() functions.
*/
impl /*struct*/ QStatusBar {
  pub fn hideOrShow_0<RetType, T: QStatusBar_hideOrShow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideOrShow_0(self);
    // return 1;
  }
}
pub trait QStatusBar_hideOrShow_0<RetType> {
  fn hideOrShow_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_hideOrShow_0<(/*void*/)> for () {
  fn hideOrShow_0(self , rsthis: & QStatusBar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QStatusBar10hideOrShowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstatusbar.h:89
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QStatusBar {
  pub fn event_0<RetType, T: QStatusBar_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QStatusBar_event_0<RetType> {
  fn event_0(self , rsthis: & QStatusBar) -> RetType;
}
impl<'a> /*trait*/ QStatusBar_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QStatusBar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QStatusBar5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
