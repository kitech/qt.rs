

// mod ::widgets::QWidgetAction
// package qtwidgets
// /usr/include/qt/QtWidgets/qwidgetaction.h
// #include <qwidgetaction.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 6
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
// func (this *QWidgetAction) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QWidgetAction) InheritEventFilter(f func(arg0 *qtcore.QObject/*777 QObject **/, arg1 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// QWidget * createWidget(QWidget *)
// func (this *QWidgetAction) InheritCreateWidget(f func(parent *QWidget/*777 QWidget **/) unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "createWidget", f)
// }

// void deleteWidget(QWidget *)
// func (this *QWidgetAction) InheritDeleteWidget(f func(widget *QWidget/*777 QWidget **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "deleteWidget", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QWidgetAction)=16
pub struct QWidgetAction {
  qbase: QAction,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWidgetAction_ITF interface {
//    QAction_ITF
//    QWidgetAction_PTR() *QWidgetAction
//}
//func (ptr *QWidgetAction) QWidgetAction_PTR() *QWidgetAction { return ptr }

impl /*struct*/ QWidgetAction {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWidgetAction {
    return QWidgetAction{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWidgetAction {
//  type Target = QWidgetActionBASE;
//
//  fn deref(&self) -> &QWidgetActionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWidgetActionBASE> for QWidgetAction {
//  fn as_ref(& self) -> & QWidgetActionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qwidgetaction.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QWidgetAction {
  pub fn metaObject_0<RetType, T: QWidgetAction_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QWidgetAction_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QWidgetAction) -> RetType;
}
impl<'a> /*trait*/ QWidgetAction_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QWidgetAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QWidgetAction10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidgetaction.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QWidgetAction(QObject *)

/*
Constructs an action with parent.
*/
// QWidgetAction(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QWidgetAction {
  pub fn QWidgetAction_0<T: QWidgetAction_QWidgetAction_0>(value: T) -> QWidgetAction {
    let rsthis = value.QWidgetAction_0();
    return rsthis;
    // return 1;
  }
}

pub trait QWidgetAction_QWidgetAction_0 {
  fn QWidgetAction_0(self) -> QWidgetAction;
}
// QWidgetAction(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWidgetAction_QWidgetAction_0 for (usize) {
  fn QWidgetAction_0(self) -> QWidgetAction {
    // unsafe{_ZN13QWidgetActionC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QWidgetActionC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWidgetAction{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidgetaction.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QWidgetAction()

/*

*/
pub fn DeleteQWidgetAction(this :*mut QWidgetAction) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QWidgetActionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qwidgetaction.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultWidget(QWidget *)

/*
Sets widget to be the default widget. The ownership is transferred to QWidgetAction. Unless createWidget() is reimplemented by a subclass to return a new widget the default widget is used when a container widget requests a widget through requestWidget().

See also defaultWidget().
*/
impl /*struct*/ QWidgetAction {
  pub fn setDefaultWidget_0<RetType, T: QWidgetAction_setDefaultWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultWidget_0(self);
    // return 1;
  }
}
pub trait QWidgetAction_setDefaultWidget_0<RetType> {
  fn setDefaultWidget_0(self , rsthis: & QWidgetAction) -> RetType;
}
impl<'a> /*trait*/ QWidgetAction_setDefaultWidget_0<(/*void*/)> for (usize) {
  fn setDefaultWidget_0(self , rsthis: & QWidgetAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QWidgetAction16setDefaultWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidgetaction.h:63
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * defaultWidget() const

/*
Returns the default widget.

See also setDefaultWidget().
*/
impl /*struct*/ QWidgetAction {
  pub fn defaultWidget_0<RetType, T: QWidgetAction_defaultWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultWidget_0(self);
    // return 1;
  }
}
pub trait QWidgetAction_defaultWidget_0<RetType> {
  fn defaultWidget_0(self , rsthis: & QWidgetAction) -> RetType;
}
impl<'a> /*trait*/ QWidgetAction_defaultWidget_0<usize> for () {
  fn defaultWidget_0(self , rsthis: & QWidgetAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QWidgetAction13defaultWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidgetaction.h:65
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * requestWidget(QWidget *)

/*
Returns a widget that represents the action, with the given parent.

Container widgets that support actions can call this function to request a widget as visual representation of the action.

See also releaseWidget(), createWidget(), and defaultWidget().
*/
impl /*struct*/ QWidgetAction {
  pub fn requestWidget_0<RetType, T: QWidgetAction_requestWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.requestWidget_0(self);
    // return 1;
  }
}
pub trait QWidgetAction_requestWidget_0<RetType> {
  fn requestWidget_0(self , rsthis: & QWidgetAction) -> RetType;
}
impl<'a> /*trait*/ QWidgetAction_requestWidget_0<usize> for (usize) {
  fn requestWidget_0(self , rsthis: & QWidgetAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QWidgetAction13requestWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidgetaction.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void releaseWidget(QWidget *)

/*
Releases the specified widget.

Container widgets that support actions call this function when a widget action is removed.

See also requestWidget(), deleteWidget(), and defaultWidget().
*/
impl /*struct*/ QWidgetAction {
  pub fn releaseWidget_0<RetType, T: QWidgetAction_releaseWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.releaseWidget_0(self);
    // return 1;
  }
}
pub trait QWidgetAction_releaseWidget_0<RetType> {
  fn releaseWidget_0(self , rsthis: & QWidgetAction) -> RetType;
}
impl<'a> /*trait*/ QWidgetAction_releaseWidget_0<(/*void*/)> for (usize) {
  fn releaseWidget_0(self , rsthis: & QWidgetAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QWidgetAction13releaseWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwidgetaction.h:69
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QWidgetAction {
  pub fn event_0<RetType, T: QWidgetAction_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QWidgetAction_event_0<RetType> {
  fn event_0(self , rsthis: & QWidgetAction) -> RetType;
}
impl<'a> /*trait*/ QWidgetAction_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QWidgetAction) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QWidgetAction5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidgetaction.h:70
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().
*/
impl /*struct*/ QWidgetAction {
  pub fn eventFilter_0<RetType, T: QWidgetAction_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QWidgetAction_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QWidgetAction) -> RetType;
}
impl<'a> /*trait*/ QWidgetAction_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QWidgetAction) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QWidgetAction11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidgetaction.h:71
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QWidget * createWidget(QWidget *)

/*
This function is called whenever the action is added to a container widget that supports custom widgets. If you don't want a custom widget to be used as representation of the action in the specified parent widget then 0 should be returned.

See also deleteWidget().
*/
impl /*struct*/ QWidgetAction {
  pub fn createWidget_0<RetType, T: QWidgetAction_createWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createWidget_0(self);
    // return 1;
  }
}
pub trait QWidgetAction_createWidget_0<RetType> {
  fn createWidget_0(self , rsthis: & QWidgetAction) -> RetType;
}
impl<'a> /*trait*/ QWidgetAction_createWidget_0<usize> for (usize) {
  fn createWidget_0(self , rsthis: & QWidgetAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QWidgetAction12createWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwidgetaction.h:72
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void deleteWidget(QWidget *)

/*
This function is called whenever the action is removed from a container widget that displays the action using a custom widget previously created using createWidget(). The default implementation hides the widget and schedules it for deletion using QObject::deleteLater().

See also createWidget().
*/
impl /*struct*/ QWidgetAction {
  pub fn deleteWidget_0<RetType, T: QWidgetAction_deleteWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.deleteWidget_0(self);
    // return 1;
  }
}
pub trait QWidgetAction_deleteWidget_0<RetType> {
  fn deleteWidget_0(self , rsthis: & QWidgetAction) -> RetType;
}
impl<'a> /*trait*/ QWidgetAction_deleteWidget_0<(/*void*/)> for (usize) {
  fn deleteWidget_0(self , rsthis: & QWidgetAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QWidgetAction12deleteWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
