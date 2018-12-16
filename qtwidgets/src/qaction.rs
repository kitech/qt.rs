

// mod ::widgets::QAction
// package qtwidgets
// /usr/include/qt/QtWidgets/qaction.h
// #include <qaction.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 22
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
// func (this *QAction) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAction)=16
pub struct QAction {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAction_ITF interface {
//    qtcore.QObject_ITF
//    QAction_PTR() *QAction
//}
//func (ptr *QAction) QAction_PTR() *QAction { return ptr }

impl /*struct*/ QAction {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAction {
    return QAction{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAction {
//  type Target = QActionBASE;
//
//  fn deref(&self) -> &QActionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QActionBASE> for QAction {
//  fn as_ref(& self) -> & QActionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qaction.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAction {
  pub fn metaObject_0<RetType, T: QAction_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAction_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAction(QObject *)

/*
Constructs an action with parent. If parent is an action group the action will be automatically inserted into the group.

Note: The parent argument is optional since Qt 5.7.
*/
// QAction(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QAction {
  pub fn QAction_0<T: QAction_QAction_0>(value: T) -> QAction {
    let rsthis = value.QAction_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAction_QAction_0 {
  fn QAction_0(self) -> QAction;
}
// QAction(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAction_QAction_0 for (usize) {
  fn QAction_0(self) -> QAction {
    // unsafe{_ZN7QActionC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QActionC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAction{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:96
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QAction(const QString &, QObject *)

/*
Constructs an action with parent. If parent is an action group the action will be automatically inserted into the group.

Note: The parent argument is optional since Qt 5.7.
*/
// QAction(const QString &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QAction {
  pub fn QAction_1<T: QAction_QAction_1>(value: T) -> QAction {
    let rsthis = value.QAction_1();
    return rsthis;
    // return 1;
  }
}

pub trait QAction_QAction_1 {
  fn QAction_1(self) -> QAction;
}
// QAction(const QString &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAction_QAction_1 for (usize,usize) {
  fn QAction_1(self) -> QAction {
    // unsafe{_ZN7QActionC2ERK7QStringP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QActionC2ERK7QStringP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAction{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:97
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QAction(const QIcon &, const QString &, QObject *)

/*
Constructs an action with parent. If parent is an action group the action will be automatically inserted into the group.

Note: The parent argument is optional since Qt 5.7.
*/
// QAction(const QIcon &, const QString &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QAction {
  pub fn QAction_2<T: QAction_QAction_2>(value: T) -> QAction {
    let rsthis = value.QAction_2();
    return rsthis;
    // return 1;
  }
}

pub trait QAction_QAction_2 {
  fn QAction_2(self) -> QAction;
}
// QAction(const QIcon &, const QString &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAction_QAction_2 for (usize,usize,usize) {
  fn QAction_2(self) -> QAction {
    // unsafe{_ZN7QActionC2ERK5QIconRK7QStringP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QActionC2ERK5QIconRK7QStringP7QObject", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAction{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:99
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAction()

/*

*/
pub fn DeleteQAction(this :*mut QAction) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QActionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qaction.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setActionGroup(QActionGroup *)

/*
Sets this action group to group. The action will be automatically added to the group's list of actions.

Actions within the group will be mutually exclusive.

See also QActionGroup and QAction::actionGroup().
*/
impl /*struct*/ QAction {
  pub fn setActionGroup_0<RetType, T: QAction_setActionGroup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setActionGroup_0(self);
    // return 1;
  }
}
pub trait QAction_setActionGroup_0<RetType> {
  fn setActionGroup_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setActionGroup_0<(/*void*/)> for (usize) {
  fn setActionGroup_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction14setActionGroupEP12QActionGroup", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:102
// index:0
// Public Visibility=Default Availability=Available
// [8] QActionGroup * actionGroup() const

/*
Returns the action group for this action. If no action group manages this action then 0 will be returned.

See also QActionGroup and QAction::setActionGroup().
*/
impl /*struct*/ QAction {
  pub fn actionGroup_0<RetType, T: QAction_actionGroup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionGroup_0(self);
    // return 1;
  }
}
pub trait QAction_actionGroup_0<RetType> {
  fn actionGroup_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_actionGroup_0<usize> for () {
  fn actionGroup_0(self , rsthis: & QAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction11actionGroupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIcon(const QIcon &)

/*

*/
impl /*struct*/ QAction {
  pub fn setIcon_0<RetType, T: QAction_setIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIcon_0(self);
    // return 1;
  }
}
pub trait QAction_setIcon_0<RetType> {
  fn setIcon_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setIcon_0<(/*void*/)> for (usize) {
  fn setIcon_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction7setIconERK5QIcon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:104
// index:0
// Public Visibility=Default Availability=Available
// [8] QIcon icon() const

/*

*/
impl /*struct*/ QAction {
  pub fn icon_0<RetType, T: QAction_icon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.icon_0(self);
    // return 1;
  }
}
pub trait QAction_icon_0<RetType> {
  fn icon_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_icon_0<usize> for () {
  fn icon_0(self , rsthis: & QAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction4iconEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*

*/
impl /*struct*/ QAction {
  pub fn setText_0<RetType, T: QAction_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QAction_setText_0<RetType> {
  fn setText_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:107
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QAction {
  pub fn text_0<RetType, T: QAction_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QAction_text_0<RetType> {
  fn text_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_text_0<usize> for () {
  fn text_0(self , rsthis: & QAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIconText(const QString &)

/*

*/
impl /*struct*/ QAction {
  pub fn setIconText_0<RetType, T: QAction_setIconText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIconText_0(self);
    // return 1;
  }
}
pub trait QAction_setIconText_0<RetType> {
  fn setIconText_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setIconText_0<(/*void*/)> for (usize) {
  fn setIconText_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction11setIconTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:110
// index:0
// Public Visibility=Default Availability=Available
// [8] QString iconText() const

/*

*/
impl /*struct*/ QAction {
  pub fn iconText_0<RetType, T: QAction_iconText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconText_0(self);
    // return 1;
  }
}
pub trait QAction_iconText_0<RetType> {
  fn iconText_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_iconText_0<usize> for () {
  fn iconText_0(self , rsthis: & QAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction8iconTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setToolTip(const QString &)

/*

*/
impl /*struct*/ QAction {
  pub fn setToolTip_0<RetType, T: QAction_setToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolTip_0(self);
    // return 1;
  }
}
pub trait QAction_setToolTip_0<RetType> {
  fn setToolTip_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setToolTip_0<(/*void*/)> for (usize) {
  fn setToolTip_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction10setToolTipERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:113
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toolTip() const

/*

*/
impl /*struct*/ QAction {
  pub fn toolTip_0<RetType, T: QAction_toolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolTip_0(self);
    // return 1;
  }
}
pub trait QAction_toolTip_0<RetType> {
  fn toolTip_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_toolTip_0<usize> for () {
  fn toolTip_0(self , rsthis: & QAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction7toolTipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStatusTip(const QString &)

/*

*/
impl /*struct*/ QAction {
  pub fn setStatusTip_0<RetType, T: QAction_setStatusTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStatusTip_0(self);
    // return 1;
  }
}
pub trait QAction_setStatusTip_0<RetType> {
  fn setStatusTip_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setStatusTip_0<(/*void*/)> for (usize) {
  fn setStatusTip_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction12setStatusTipERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:116
// index:0
// Public Visibility=Default Availability=Available
// [8] QString statusTip() const

/*

*/
impl /*struct*/ QAction {
  pub fn statusTip_0<RetType, T: QAction_statusTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.statusTip_0(self);
    // return 1;
  }
}
pub trait QAction_statusTip_0<RetType> {
  fn statusTip_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_statusTip_0<usize> for () {
  fn statusTip_0(self , rsthis: & QAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction9statusTipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWhatsThis(const QString &)

/*

*/
impl /*struct*/ QAction {
  pub fn setWhatsThis_0<RetType, T: QAction_setWhatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis_0(self);
    // return 1;
  }
}
pub trait QAction_setWhatsThis_0<RetType> {
  fn setWhatsThis_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setWhatsThis_0<(/*void*/)> for (usize) {
  fn setWhatsThis_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction12setWhatsThisERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:119
// index:0
// Public Visibility=Default Availability=Available
// [8] QString whatsThis() const

/*

*/
impl /*struct*/ QAction {
  pub fn whatsThis_0<RetType, T: QAction_whatsThis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.whatsThis_0(self);
    // return 1;
  }
}
pub trait QAction_whatsThis_0<RetType> {
  fn whatsThis_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_whatsThis_0<usize> for () {
  fn whatsThis_0(self , rsthis: & QAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction9whatsThisEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPriority(QAction::Priority)

/*

*/
impl /*struct*/ QAction {
  pub fn setPriority_0<RetType, T: QAction_setPriority_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPriority_0(self);
    // return 1;
  }
}
pub trait QAction_setPriority_0<RetType> {
  fn setPriority_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setPriority_0<(/*void*/)> for (i32) {
  fn setPriority_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction11setPriorityENS_8PriorityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:122
// index:0
// Public Visibility=Default Availability=Available
// [4] QAction::Priority priority() const

/*

*/
impl /*struct*/ QAction {
  pub fn priority_0<RetType, T: QAction_priority_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.priority_0(self);
    // return 1;
  }
}
pub trait QAction_priority_0<RetType> {
  fn priority_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_priority_0<i32> for () {
  fn priority_0(self , rsthis: & QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction8priorityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:125
// index:0
// Public Visibility=Default Availability=Available
// [8] QMenu * menu() const

/*
Returns the menu contained by this action. Actions that contain menus can be used to create menu items with submenus, or inserted into toolbars to create buttons with popup menus.

See also setMenu() and QMenu::addAction().
*/
impl /*struct*/ QAction {
  pub fn menu_0<RetType, T: QAction_menu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.menu_0(self);
    // return 1;
  }
}
pub trait QAction_menu_0<RetType> {
  fn menu_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_menu_0<usize> for () {
  fn menu_0(self , rsthis: & QAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction4menuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMenu(QMenu *)

/*
Sets the menu contained by this action to the specified menu.

See also menu().
*/
impl /*struct*/ QAction {
  pub fn setMenu_0<RetType, T: QAction_setMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMenu_0(self);
    // return 1;
  }
}
pub trait QAction_setMenu_0<RetType> {
  fn setMenu_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setMenu_0<(/*void*/)> for (usize) {
  fn setMenu_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction7setMenuEP5QMenu", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSeparator(bool)

/*
If b is true then this action will be considered a separator.

How a separator is represented depends on the widget it is inserted into. Under most circumstances the text, submenu, and icon will be ignored for separator actions.

See also QAction::isSeparator().
*/
impl /*struct*/ QAction {
  pub fn setSeparator_0<RetType, T: QAction_setSeparator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSeparator_0(self);
    // return 1;
  }
}
pub trait QAction_setSeparator_0<RetType> {
  fn setSeparator_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setSeparator_0<(/*void*/)> for (bool) {
  fn setSeparator_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction12setSeparatorEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:130
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSeparator() const

/*
Returns true if this action is a separator action; otherwise it returns false.

See also QAction::setSeparator().
*/
impl /*struct*/ QAction {
  pub fn isSeparator_0<RetType, T: QAction_isSeparator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSeparator_0(self);
    // return 1;
  }
}
pub trait QAction_isSeparator_0<RetType> {
  fn isSeparator_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_isSeparator_0<bool> for () {
  fn isSeparator_0(self , rsthis: & QAction) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction11isSeparatorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setShortcut(const QKeySequence &)

/*

*/
impl /*struct*/ QAction {
  pub fn setShortcut_0<RetType, T: QAction_setShortcut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setShortcut_0(self);
    // return 1;
  }
}
pub trait QAction_setShortcut_0<RetType> {
  fn setShortcut_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setShortcut_0<(/*void*/)> for (usize) {
  fn setShortcut_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction11setShortcutERK12QKeySequence", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:134
// index:0
// Public Visibility=Default Availability=Available
// [8] QKeySequence shortcut() const

/*
Returns the primary shortcut.

Note: Getter function for property shortcut. 

See also setShortcuts().
*/
impl /*struct*/ QAction {
  pub fn shortcut_0<RetType, T: QAction_shortcut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shortcut_0(self);
    // return 1;
  }
}
pub trait QAction_shortcut_0<RetType> {
  fn shortcut_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_shortcut_0<usize> for () {
  fn shortcut_0(self , rsthis: & QAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction8shortcutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setShortcuts(QKeySequence::StandardKey)

/*
Sets shortcuts as the list of shortcuts that trigger the action. The first element of the list is the primary shortcut.

This function was introduced in  Qt 4.2.

See also shortcuts() and shortcut.
*/
impl /*struct*/ QAction {
  pub fn setShortcuts_0<RetType, T: QAction_setShortcuts_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setShortcuts_0(self);
    // return 1;
  }
}
pub trait QAction_setShortcuts_0<RetType> {
  fn setShortcuts_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setShortcuts_0<(/*void*/)> for (i32) {
  fn setShortcuts_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction12setShortcutsEN12QKeySequence11StandardKeyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setShortcutContext(Qt::ShortcutContext)

/*

*/
impl /*struct*/ QAction {
  pub fn setShortcutContext_0<RetType, T: QAction_setShortcutContext_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setShortcutContext_0(self);
    // return 1;
  }
}
pub trait QAction_setShortcutContext_0<RetType> {
  fn setShortcutContext_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setShortcutContext_0<(/*void*/)> for (i32) {
  fn setShortcutContext_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction18setShortcutContextEN2Qt15ShortcutContextE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:141
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ShortcutContext shortcutContext() const

/*

*/
impl /*struct*/ QAction {
  pub fn shortcutContext_0<RetType, T: QAction_shortcutContext_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shortcutContext_0(self);
    // return 1;
  }
}
pub trait QAction_shortcutContext_0<RetType> {
  fn shortcutContext_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_shortcutContext_0<i32> for () {
  fn shortcutContext_0(self , rsthis: & QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction15shortcutContextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoRepeat(bool)

/*

*/
impl /*struct*/ QAction {
  pub fn setAutoRepeat_0<RetType, T: QAction_setAutoRepeat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoRepeat_0(self);
    // return 1;
  }
}
pub trait QAction_setAutoRepeat_0<RetType> {
  fn setAutoRepeat_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setAutoRepeat_0<(/*void*/)> for (bool) {
  fn setAutoRepeat_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction13setAutoRepeatEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:144
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoRepeat() const

/*

*/
impl /*struct*/ QAction {
  pub fn autoRepeat_0<RetType, T: QAction_autoRepeat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoRepeat_0(self);
    // return 1;
  }
}
pub trait QAction_autoRepeat_0<RetType> {
  fn autoRepeat_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_autoRepeat_0<bool> for () {
  fn autoRepeat_0(self , rsthis: & QAction) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction10autoRepeatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:147
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*

*/
impl /*struct*/ QAction {
  pub fn setFont_0<RetType, T: QAction_setFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_0(self);
    // return 1;
  }
}
pub trait QAction_setFont_0<RetType> {
  fn setFont_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setFont_0<(/*void*/)> for (usize) {
  fn setFont_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:148
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont font() const

/*

*/
impl /*struct*/ QAction {
  pub fn font_0<RetType, T: QAction_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QAction_font_0<RetType> {
  fn font_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_font_0<usize> for () {
  fn font_0(self , rsthis: & QAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:150
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCheckable(bool)

/*

*/
impl /*struct*/ QAction {
  pub fn setCheckable_0<RetType, T: QAction_setCheckable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCheckable_0(self);
    // return 1;
  }
}
pub trait QAction_setCheckable_0<RetType> {
  fn setCheckable_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setCheckable_0<(/*void*/)> for (bool) {
  fn setCheckable_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction12setCheckableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:151
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCheckable() const

/*

*/
impl /*struct*/ QAction {
  pub fn isCheckable_0<RetType, T: QAction_isCheckable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCheckable_0(self);
    // return 1;
  }
}
pub trait QAction_isCheckable_0<RetType> {
  fn isCheckable_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_isCheckable_0<bool> for () {
  fn isCheckable_0(self , rsthis: & QAction) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction11isCheckableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:153
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant data() const

/*
Returns the user data as set in QAction::setData.

See also setData().
*/
impl /*struct*/ QAction {
  pub fn data_0<RetType, T: QAction_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QAction_data_0<RetType> {
  fn data_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_data_0<usize> for () {
  fn data_0(self , rsthis: & QAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:154
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setData(const QVariant &)

/*
Sets the action's internal data to the given userData.

See also data().
*/
impl /*struct*/ QAction {
  pub fn setData_0<RetType, T: QAction_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QAction_setData_0<RetType> {
  fn setData_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setData_0<(/*void*/)> for (usize) {
  fn setData_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction7setDataERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:156
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isChecked() const

/*

*/
impl /*struct*/ QAction {
  pub fn isChecked_0<RetType, T: QAction_isChecked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isChecked_0(self);
    // return 1;
  }
}
pub trait QAction_isChecked_0<RetType> {
  fn isChecked_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_isChecked_0<bool> for () {
  fn isChecked_0(self , rsthis: & QAction) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction9isCheckedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:158
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEnabled() const

/*

*/
impl /*struct*/ QAction {
  pub fn isEnabled_0<RetType, T: QAction_isEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEnabled_0(self);
    // return 1;
  }
}
pub trait QAction_isEnabled_0<RetType> {
  fn isEnabled_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_isEnabled_0<bool> for () {
  fn isEnabled_0(self , rsthis: & QAction) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction9isEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:160
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isVisible() const

/*

*/
impl /*struct*/ QAction {
  pub fn isVisible_0<RetType, T: QAction_isVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isVisible_0(self);
    // return 1;
  }
}
pub trait QAction_isVisible_0<RetType> {
  fn isVisible_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_isVisible_0<bool> for () {
  fn isVisible_0(self , rsthis: & QAction) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction9isVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:163
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activate(QAction::ActionEvent)

/*
Sends the relevant signals for ActionEvent event.

Action based widgets use this API to cause the QAction to emit signals as well as emitting their own.
*/
impl /*struct*/ QAction {
  pub fn activate_0<RetType, T: QAction_activate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activate_0(self);
    // return 1;
  }
}
pub trait QAction_activate_0<RetType> {
  fn activate_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_activate_0<(/*void*/)> for (i32) {
  fn activate_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction8activateENS_11ActionEventE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:164
// index:0
// Public Visibility=Default Availability=Available
// [1] bool showStatusText(QWidget *)

/*
Updates the relevant status bar for the widget specified by sending a QStatusTipEvent to its parent widget. Returns true if an event was sent; otherwise returns false.

If a null widget is specified, the event is sent to the action's parent.

See also statusTip.
*/
impl /*struct*/ QAction {
  pub fn showStatusText_0<RetType, T: QAction_showStatusText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showStatusText_0(self);
    // return 1;
  }
}
pub trait QAction_showStatusText_0<RetType> {
  fn showStatusText_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_showStatusText_0<bool> for (usize) {
  fn showStatusText_0(self , rsthis: & QAction) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QAction14showStatusTextEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:166
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMenuRole(QAction::MenuRole)

/*

*/
impl /*struct*/ QAction {
  pub fn setMenuRole_0<RetType, T: QAction_setMenuRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMenuRole_0(self);
    // return 1;
  }
}
pub trait QAction_setMenuRole_0<RetType> {
  fn setMenuRole_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setMenuRole_0<(/*void*/)> for (i32) {
  fn setMenuRole_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction11setMenuRoleENS_8MenuRoleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:167
// index:0
// Public Visibility=Default Availability=Available
// [4] QAction::MenuRole menuRole() const

/*

*/
impl /*struct*/ QAction {
  pub fn menuRole_0<RetType, T: QAction_menuRole_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.menuRole_0(self);
    // return 1;
  }
}
pub trait QAction_menuRole_0<RetType> {
  fn menuRole_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_menuRole_0<i32> for () {
  fn menuRole_0(self , rsthis: & QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction8menuRoleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:169
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIconVisibleInMenu(bool)

/*

*/
impl /*struct*/ QAction {
  pub fn setIconVisibleInMenu_0<RetType, T: QAction_setIconVisibleInMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIconVisibleInMenu_0(self);
    // return 1;
  }
}
pub trait QAction_setIconVisibleInMenu_0<RetType> {
  fn setIconVisibleInMenu_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setIconVisibleInMenu_0<(/*void*/)> for (bool) {
  fn setIconVisibleInMenu_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction20setIconVisibleInMenuEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:170
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isIconVisibleInMenu() const

/*

*/
impl /*struct*/ QAction {
  pub fn isIconVisibleInMenu_0<RetType, T: QAction_isIconVisibleInMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isIconVisibleInMenu_0(self);
    // return 1;
  }
}
pub trait QAction_isIconVisibleInMenu_0<RetType> {
  fn isIconVisibleInMenu_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_isIconVisibleInMenu_0<bool> for () {
  fn isIconVisibleInMenu_0(self , rsthis: & QAction) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction19isIconVisibleInMenuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:172
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setShortcutVisibleInContextMenu(bool)

/*

*/
impl /*struct*/ QAction {
  pub fn setShortcutVisibleInContextMenu_0<RetType, T: QAction_setShortcutVisibleInContextMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setShortcutVisibleInContextMenu_0(self);
    // return 1;
  }
}
pub trait QAction_setShortcutVisibleInContextMenu_0<RetType> {
  fn setShortcutVisibleInContextMenu_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setShortcutVisibleInContextMenu_0<(/*void*/)> for (bool) {
  fn setShortcutVisibleInContextMenu_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction31setShortcutVisibleInContextMenuEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:173
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isShortcutVisibleInContextMenu() const

/*

*/
impl /*struct*/ QAction {
  pub fn isShortcutVisibleInContextMenu_0<RetType, T: QAction_isShortcutVisibleInContextMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isShortcutVisibleInContextMenu_0(self);
    // return 1;
  }
}
pub trait QAction_isShortcutVisibleInContextMenu_0<RetType> {
  fn isShortcutVisibleInContextMenu_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_isShortcutVisibleInContextMenu_0<bool> for () {
  fn isShortcutVisibleInContextMenu_0(self , rsthis: & QAction) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction30isShortcutVisibleInContextMenuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:175
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * parentWidget() const

/*
Returns the parent widget.
*/
impl /*struct*/ QAction {
  pub fn parentWidget_0<RetType, T: QAction_parentWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parentWidget_0(self);
    // return 1;
  }
}
pub trait QAction_parentWidget_0<RetType> {
  fn parentWidget_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_parentWidget_0<usize> for () {
  fn parentWidget_0(self , rsthis: & QAction) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QAction12parentWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:183
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QAction {
  pub fn event_0<RetType, T: QAction_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QAction_event_0<RetType> {
  fn event_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QAction) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QAction5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:187
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void trigger()

/*
This is a convenience slot that calls activate(Trigger).
*/
impl /*struct*/ QAction {
  pub fn trigger_0<RetType, T: QAction_trigger_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.trigger_0(self);
    // return 1;
  }
}
pub trait QAction_trigger_0<RetType> {
  fn trigger_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_trigger_0<(/*void*/)> for () {
  fn trigger_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QAction7triggerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:188
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void hover()

/*
This is a convenience slot that calls activate(Hover).
*/
impl /*struct*/ QAction {
  pub fn hover_0<RetType, T: QAction_hover_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hover_0(self);
    // return 1;
  }
}
pub trait QAction_hover_0<RetType> {
  fn hover_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_hover_0<(/*void*/)> for () {
  fn hover_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QAction5hoverEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:189
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setChecked(bool)

/*

*/
impl /*struct*/ QAction {
  pub fn setChecked_0<RetType, T: QAction_setChecked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setChecked_0(self);
    // return 1;
  }
}
pub trait QAction_setChecked_0<RetType> {
  fn setChecked_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setChecked_0<(/*void*/)> for (bool) {
  fn setChecked_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction10setCheckedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toggle()

/*
This is a convenience function for the checked property. Connect to it to change the checked state to its opposite state.
*/
impl /*struct*/ QAction {
  pub fn toggle_0<RetType, T: QAction_toggle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toggle_0(self);
    // return 1;
  }
}
pub trait QAction_toggle_0<RetType> {
  fn toggle_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_toggle_0<(/*void*/)> for () {
  fn toggle_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QAction6toggleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:191
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEnabled(bool)

/*

*/
impl /*struct*/ QAction {
  pub fn setEnabled_0<RetType, T: QAction_setEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEnabled_0(self);
    // return 1;
  }
}
pub trait QAction_setEnabled_0<RetType> {
  fn setEnabled_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setEnabled_0<(/*void*/)> for (bool) {
  fn setEnabled_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction10setEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:192
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setDisabled(bool)

/*
This is a convenience function for the enabled property, that is useful for signals--slots connections. If b is true the action is disabled; otherwise it is enabled.
*/
impl /*struct*/ QAction {
  pub fn setDisabled_0<RetType, T: QAction_setDisabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDisabled_0(self);
    // return 1;
  }
}
pub trait QAction_setDisabled_0<RetType> {
  fn setDisabled_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setDisabled_0<(/*void*/)> for (bool) {
  fn setDisabled_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction11setDisabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:193
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*

*/
impl /*struct*/ QAction {
  pub fn setVisible_0<RetType, T: QAction_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QAction_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:196
// index:0
// Public Visibility=Default Availability=Available
// [-2] void changed()

/*
This signal is emitted when an action has changed. If you are only interested in actions in a given widget, you can watch for QWidget::actionEvent() sent with an QEvent::ActionChanged.

Note: Notifier signal for property autoRepeat. Notifier signal for property checkable. Notifier signal for property enabled. Notifier signal for property font. Notifier signal for property icon. Notifier signal for property iconText. Notifier signal for property iconVisibleInMenu. Notifier signal for property menuRole. Notifier signal for property shortcut. Notifier signal for property shortcutContext. Notifier signal for property shortcutVisibleInContextMenu. Notifier signal for property statusTip. Notifier signal for property text. Notifier signal for property toolTip. Notifier signal for property visible. Notifier signal for property whatsThis. 

See also QWidget::actionEvent().
*/
impl /*struct*/ QAction {
  pub fn changed_0<RetType, T: QAction_changed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changed_0(self);
    // return 1;
  }
}
pub trait QAction_changed_0<RetType> {
  fn changed_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_changed_0<(/*void*/)> for () {
  fn changed_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QAction7changedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:197
// index:0
// Public Visibility=Default Availability=Available
// [-2] void triggered(bool)

/*
This signal is emitted when an action is activated by the user; for example, when the user clicks a menu option, toolbar button, or presses an action's shortcut key combination, or when trigger() was called. Notably, it is not emitted when setChecked() or toggle() is called.

If the action is checkable, checked is true if the action is checked, or false if the action is unchecked.

See also QAction::activate(), QAction::toggled(), and checked.
*/
impl /*struct*/ QAction {
  pub fn triggered_0<RetType, T: QAction_triggered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.triggered_0(self);
    // return 1;
  }
}
pub trait QAction_triggered_0<RetType> {
  fn triggered_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_triggered_0<(/*void*/)> for (bool) {
  fn triggered_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction9triggeredEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:198
// index:0
// Public Visibility=Default Availability=Available
// [-2] void hovered()

/*
This signal is emitted when an action is highlighted by the user; for example, when the user pauses with the cursor over a menu option, toolbar button, or presses an action's shortcut key combination.

See also QAction::activate().
*/
impl /*struct*/ QAction {
  pub fn hovered_0<RetType, T: QAction_hovered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hovered_0(self);
    // return 1;
  }
}
pub trait QAction_hovered_0<RetType> {
  fn hovered_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_hovered_0<(/*void*/)> for () {
  fn hovered_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QAction7hoveredEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qaction.h:199
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toggled(bool)

/*
This signal is emitted whenever a checkable action changes its isChecked() status. This can be the result of a user interaction, or because setChecked() was called.

checked is true if the action is checked, or false if the action is unchecked.

Note: Notifier signal for property checked. 

See also QAction::activate(), QAction::triggered(), and checked.
*/
impl /*struct*/ QAction {
  pub fn toggled_0<RetType, T: QAction_toggled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toggled_0(self);
    // return 1;
  }
}
pub trait QAction_toggled_0<RetType> {
  fn toggled_0(self , rsthis: & QAction) -> RetType;
}
impl<'a> /*trait*/ QAction_toggled_0<(/*void*/)> for (bool) {
  fn toggled_0(self , rsthis: & QAction) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QAction7toggledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum describes how an action should be moved into the application menu on macOS.



Setting this value only has effect on items that are in the immediate menus of the menubar, not the submenus of those menus. For example, if you have File menu in your menubar and the File menu has a submenu, setting the MenuRole for the actions in that submenu have no effect. They will never be moved.

*/
pub type QAction__MenuRole = i32;
// This action should not be put into the application menu
pub const QAction__NoRole :QAction__MenuRole = 0;
// This action should be put in the application menu based on the action's text as described in the QMenuBar documentation.
pub const QAction__TextHeuristicRole :QAction__MenuRole = 1;
// This action should be put in the application menu with an application specific role
pub const QAction__ApplicationSpecificRole :QAction__MenuRole = 2;
// This action handles the "About Qt" menu item.
pub const QAction__AboutQtRole :QAction__MenuRole = 3;
// This action should be placed where the "About" menu item is in the application menu. The text of the menu item will be set to "About <application name>". The application name is fetched from the Info.plist file in the application's bundle (See Qt for macOS - Deployment).
pub const QAction__AboutRole :QAction__MenuRole = 4;
// This action should be placed where the "Preferences..." menu item is in the application menu.
pub const QAction__PreferencesRole :QAction__MenuRole = 5;
// This action should be placed where the Quit menu item is in the application menu.
pub const QAction__QuitRole :QAction__MenuRole = 6;
pub fn QAction_MenuRoleItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAction", val);
}
pub fn QAction_MenuRoleItemName_s(val: i32) ->String {
  //var nilthis *QAction
  //return nilthis.MenuRoleItemName(val);
  return QAction_MenuRoleItemName(val);
}


/*
This enum defines priorities for actions in user interface.



This enum was introduced or modified in  Qt 4.6.

See also priority.

*/
pub type QAction__Priority = i32;
// The action should not be prioritized in the user interface.
pub const QAction__LowPriority :QAction__Priority = 0;
// 
pub const QAction__NormalPriority :QAction__Priority = 128;
// 
pub const QAction__HighPriority :QAction__Priority = 256;
pub fn QAction_PriorityItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAction", val);
}
pub fn QAction_PriorityItemName_s(val: i32) ->String {
  //var nilthis *QAction
  //return nilthis.PriorityItemName(val);
  return QAction_PriorityItemName(val);
}


/*
This enum type is used when calling QAction::activate()


*/
pub type QAction__ActionEvent = i32;
// this will cause the QAction::triggered() signal to be emitted.
pub const QAction__Trigger :QAction__ActionEvent = 0;
// this will cause the QAction::hovered() signal to be emitted.
pub const QAction__Hover :QAction__ActionEvent = 1;
pub fn QAction_ActionEventItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAction", val);
}
pub fn QAction_ActionEventItemName_s(val: i32) ->String {
  //var nilthis *QAction
  //return nilthis.ActionEventItemName(val);
  return QAction_ActionEventItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
