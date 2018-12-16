

// mod ::widgets::QSplashScreen
// package qtwidgets
// /usr/include/qt/QtWidgets/qsplashscreen.h
// #include <qsplashscreen.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 25
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
// func (this *QSplashScreen) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void drawContents(QPainter *)
// func (this *QSplashScreen) InheritDrawContents(f func(painter *qtgui.QPainter/*777 QPainter **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawContents", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QSplashScreen) InheritMousePressEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSplashScreen)=48
pub struct QSplashScreen {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSplashScreen_ITF interface {
//    QWidget_ITF
//    QSplashScreen_PTR() *QSplashScreen
//}
//func (ptr *QSplashScreen) QSplashScreen_PTR() *QSplashScreen { return ptr }

impl /*struct*/ QSplashScreen {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSplashScreen {
    return QSplashScreen{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSplashScreen {
//  type Target = QSplashScreenBASE;
//
//  fn deref(&self) -> &QSplashScreenBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSplashScreenBASE> for QSplashScreen {
//  fn as_ref(& self) -> & QSplashScreenBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qsplashscreen.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSplashScreen {
  pub fn metaObject_0<RetType, T: QSplashScreen_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSplashScreen_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSplashScreen) -> RetType;
}
impl<'a> /*trait*/ QSplashScreen_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSplashScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSplashScreen10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplashscreen.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSplashScreen(const QPixmap &, Qt::WindowFlags)

/*
Construct a splash screen that will display the pixmap.

There should be no need to set the widget flags, f, except perhaps Qt::WindowStaysOnTopHint.
*/
// QSplashScreen(const QPixmap &, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QSplashScreen {
  pub fn QSplashScreen_0<T: QSplashScreen_QSplashScreen_0>(value: T) -> QSplashScreen {
    let rsthis = value.QSplashScreen_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSplashScreen_QSplashScreen_0 {
  fn QSplashScreen_0(self) -> QSplashScreen;
}
// QSplashScreen(const QPixmap &, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSplashScreen_QSplashScreen_0 for (usize,i32) {
  fn QSplashScreen_0(self) -> QSplashScreen {
    // unsafe{_ZN13QSplashScreenC2ERK7QPixmap6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QSplashScreenC2ERK7QPixmap6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSplashScreen{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplashscreen.h:58
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QSplashScreen(QWidget *, const QPixmap &, Qt::WindowFlags)

/*
Construct a splash screen that will display the pixmap.

There should be no need to set the widget flags, f, except perhaps Qt::WindowStaysOnTopHint.
*/
// QSplashScreen(QWidget *, const QPixmap &, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QSplashScreen {
  pub fn QSplashScreen_1<T: QSplashScreen_QSplashScreen_1>(value: T) -> QSplashScreen {
    let rsthis = value.QSplashScreen_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSplashScreen_QSplashScreen_1 {
  fn QSplashScreen_1(self) -> QSplashScreen;
}
// QSplashScreen(QWidget *, const QPixmap &, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSplashScreen_QSplashScreen_1 for (usize,usize,i32) {
  fn QSplashScreen_1(self) -> QSplashScreen {
    // unsafe{_ZN13QSplashScreenC2EP7QWidgetRK7QPixmap6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QSplashScreenC2EP7QWidgetRK7QPixmap6QFlagsIN2Qt10WindowTypeEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSplashScreen{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplashscreen.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSplashScreen()

/*

*/
pub fn DeleteQSplashScreen(this :*mut QSplashScreen) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QSplashScreenD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qsplashscreen.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPixmap(const QPixmap &)

/*
Sets the pixmap that will be used as the splash screen's image to pixmap.

See also pixmap().
*/
impl /*struct*/ QSplashScreen {
  pub fn setPixmap_0<RetType, T: QSplashScreen_setPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPixmap_0(self);
    // return 1;
  }
}
pub trait QSplashScreen_setPixmap_0<RetType> {
  fn setPixmap_0(self , rsthis: & QSplashScreen) -> RetType;
}
impl<'a> /*trait*/ QSplashScreen_setPixmap_0<(/*void*/)> for (usize) {
  fn setPixmap_0(self , rsthis: & QSplashScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSplashScreen9setPixmapERK7QPixmap", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplashscreen.h:62
// index:0
// Public Visibility=Default Availability=Available
// [32] const QPixmap pixmap() const

/*
Returns the pixmap that is used in the splash screen. The image does not have any of the text drawn by showMessage() calls.

See also setPixmap().
*/
impl /*struct*/ QSplashScreen {
  pub fn pixmap_0<RetType, T: QSplashScreen_pixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixmap_0(self);
    // return 1;
  }
}
pub trait QSplashScreen_pixmap_0<RetType> {
  fn pixmap_0(self , rsthis: & QSplashScreen) -> RetType;
}
impl<'a> /*trait*/ QSplashScreen_pixmap_0<usize> for () {
  fn pixmap_0(self , rsthis: & QSplashScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSplashScreen6pixmapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplashscreen.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void finish(QWidget *)

/*
Makes the splash screen wait until the widget mainWin is displayed before calling close() on itself.
*/
impl /*struct*/ QSplashScreen {
  pub fn finish_0<RetType, T: QSplashScreen_finish_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.finish_0(self);
    // return 1;
  }
}
pub trait QSplashScreen_finish_0<RetType> {
  fn finish_0(self , rsthis: & QSplashScreen) -> RetType;
}
impl<'a> /*trait*/ QSplashScreen_finish_0<(/*void*/)> for (usize) {
  fn finish_0(self , rsthis: & QSplashScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSplashScreen6finishEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplashscreen.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void repaint()

/*
This overrides QWidget::repaint(). It differs from the standard repaint function in that it also calls QApplication::processEvents() to ensure the updates are displayed, even when there is no event loop present.
*/
impl /*struct*/ QSplashScreen {
  pub fn repaint_0<RetType, T: QSplashScreen_repaint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.repaint_0(self);
    // return 1;
  }
}
pub trait QSplashScreen_repaint_0<RetType> {
  fn repaint_0(self , rsthis: & QSplashScreen) -> RetType;
}
impl<'a> /*trait*/ QSplashScreen_repaint_0<(/*void*/)> for () {
  fn repaint_0(self , rsthis: & QSplashScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QSplashScreen7repaintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplashscreen.h:65
// index:0
// Public Visibility=Default Availability=Available
// [8] QString message() const

/*
Returns the message that is currently displayed on the splash screen.

This function was introduced in  Qt 5.2.

See also showMessage() and clearMessage().
*/
impl /*struct*/ QSplashScreen {
  pub fn message_0<RetType, T: QSplashScreen_message_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.message_0(self);
    // return 1;
  }
}
pub trait QSplashScreen_message_0<RetType> {
  fn message_0(self , rsthis: & QSplashScreen) -> RetType;
}
impl<'a> /*trait*/ QSplashScreen_message_0<usize> for () {
  fn message_0(self , rsthis: & QSplashScreen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSplashScreen7messageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplashscreen.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showMessage(const QString &, int, const QColor &)

/*
Draws the message text onto the splash screen with color color and aligns the text according to the flags in alignment. This function calls repaint() to make sure the splash screen is repainted immediately. As a result the message is kept up to date with what your application is doing (e.g. loading files).

See also Qt::Alignment, clearMessage(), and message().
*/
impl /*struct*/ QSplashScreen {
  pub fn showMessage_0<RetType, T: QSplashScreen_showMessage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showMessage_0(self);
    // return 1;
  }
}
pub trait QSplashScreen_showMessage_0<RetType> {
  fn showMessage_0(self , rsthis: & QSplashScreen) -> RetType;
}
impl<'a> /*trait*/ QSplashScreen_showMessage_0<(/*void*/)> for (usize,i32,usize) {
  fn showMessage_0(self , rsthis: & QSplashScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSplashScreen11showMessageERK7QStringiRK6QColor", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplashscreen.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearMessage()

/*
Removes the message being displayed on the splash screen

See also showMessage().
*/
impl /*struct*/ QSplashScreen {
  pub fn clearMessage_0<RetType, T: QSplashScreen_clearMessage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearMessage_0(self);
    // return 1;
  }
}
pub trait QSplashScreen_clearMessage_0<RetType> {
  fn clearMessage_0(self , rsthis: & QSplashScreen) -> RetType;
}
impl<'a> /*trait*/ QSplashScreen_clearMessage_0<(/*void*/)> for () {
  fn clearMessage_0(self , rsthis: & QSplashScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QSplashScreen12clearMessageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplashscreen.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void messageChanged(const QString &)

/*
This signal is emitted when the message on the splash screen changes. message is the new message and is a null-string when the message has been removed.

See also showMessage() and clearMessage().
*/
impl /*struct*/ QSplashScreen {
  pub fn messageChanged_0<RetType, T: QSplashScreen_messageChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.messageChanged_0(self);
    // return 1;
  }
}
pub trait QSplashScreen_messageChanged_0<RetType> {
  fn messageChanged_0(self , rsthis: & QSplashScreen) -> RetType;
}
impl<'a> /*trait*/ QSplashScreen_messageChanged_0<(/*void*/)> for (usize) {
  fn messageChanged_0(self , rsthis: & QSplashScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSplashScreen14messageChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplashscreen.h:76
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QSplashScreen {
  pub fn event_0<RetType, T: QSplashScreen_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QSplashScreen_event_0<RetType> {
  fn event_0(self , rsthis: & QSplashScreen) -> RetType;
}
impl<'a> /*trait*/ QSplashScreen_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QSplashScreen) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QSplashScreen5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsplashscreen.h:77
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawContents(QPainter *)

/*
Draw the contents of the splash screen using painter painter. The default implementation draws the message passed by showMessage(). Reimplement this function if you want to do your own drawing on the splash screen.
*/
impl /*struct*/ QSplashScreen {
  pub fn drawContents_0<RetType, T: QSplashScreen_drawContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawContents_0(self);
    // return 1;
  }
}
pub trait QSplashScreen_drawContents_0<RetType> {
  fn drawContents_0(self , rsthis: & QSplashScreen) -> RetType;
}
impl<'a> /*trait*/ QSplashScreen_drawContents_0<(/*void*/)> for (usize) {
  fn drawContents_0(self , rsthis: & QSplashScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSplashScreen12drawContentsEP8QPainter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsplashscreen.h:78
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QSplashScreen {
  pub fn mousePressEvent_0<RetType, T: QSplashScreen_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QSplashScreen_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QSplashScreen) -> RetType;
}
impl<'a> /*trait*/ QSplashScreen_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QSplashScreen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSplashScreen15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
