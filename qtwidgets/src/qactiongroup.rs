

// mod ::widgets::QActionGroup
// package qtwidgets
// /usr/include/qt/QtWidgets/qactiongroup.h
// #include <qactiongroup.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 62
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



/*

*/
#[derive(Default)] // class sizeof(QActionGroup)=16
pub struct QActionGroup {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QActionGroup_ITF interface {
//    qtcore.QObject_ITF
//    QActionGroup_PTR() *QActionGroup
//}
//func (ptr *QActionGroup) QActionGroup_PTR() *QActionGroup { return ptr }

impl /*struct*/ QActionGroup {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QActionGroup {
    return QActionGroup{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QActionGroup {
//  type Target = QActionGroupBASE;
//
//  fn deref(&self) -> &QActionGroupBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QActionGroupBASE> for QActionGroup {
//  fn as_ref(& self) -> & QActionGroupBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qactiongroup.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QActionGroup {
  pub fn metaObject_0<RetType, T: QActionGroup_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QActionGroup_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QActionGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QActionGroup10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QActionGroup(QObject *)

/*
Constructs an action group for the parent object.

The action group is exclusive by default. Call setExclusive(false) to make the action group non-exclusive.
*/
// QActionGroup(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QActionGroup {
  pub fn QActionGroup_0<T: QActionGroup_QActionGroup_0>(value: T) -> QActionGroup {
    let rsthis = value.QActionGroup_0();
    return rsthis;
    // return 1;
  }
}

pub trait QActionGroup_QActionGroup_0 {
  fn QActionGroup_0(self) -> QActionGroup;
}
// QActionGroup(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QActionGroup_QActionGroup_0 for (usize) {
  fn QActionGroup_0(self) -> QActionGroup {
    // unsafe{_ZN12QActionGroupC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QActionGroupC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QActionGroup{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QActionGroup()

/*

*/
pub fn DeleteQActionGroup(this :*mut QActionGroup) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QActionGroupD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qactiongroup.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(QAction *)

/*
Adds the action to this group, and returns it.

Normally an action is added to a group by creating it with the group as its parent, so this function is not usually used.

See also QAction::setActionGroup().
*/
impl /*struct*/ QActionGroup {
  pub fn addAction_0<RetType, T: QActionGroup_addAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_0(self);
    // return 1;
  }
}
pub trait QActionGroup_addAction_0<RetType> {
  fn addAction_0(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_addAction_0<usize> for (usize) {
  fn addAction_0(self , rsthis: & QActionGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QActionGroup9addActionEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:67
// index:1
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(const QString &)

/*
Adds the action to this group, and returns it.

Normally an action is added to a group by creating it with the group as its parent, so this function is not usually used.

See also QAction::setActionGroup().
*/
impl /*struct*/ QActionGroup {
  pub fn addAction_1<RetType, T: QActionGroup_addAction_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_1(self);
    // return 1;
  }
}
pub trait QActionGroup_addAction_1<RetType> {
  fn addAction_1(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_addAction_1<usize> for (usize) {
  fn addAction_1(self , rsthis: & QActionGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QActionGroup9addActionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:68
// index:2
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(const QIcon &, const QString &)

/*
Adds the action to this group, and returns it.

Normally an action is added to a group by creating it with the group as its parent, so this function is not usually used.

See also QAction::setActionGroup().
*/
impl /*struct*/ QActionGroup {
  pub fn addAction_2<RetType, T: QActionGroup_addAction_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_2(self);
    // return 1;
  }
}
pub trait QActionGroup_addAction_2<RetType> {
  fn addAction_2(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_addAction_2<usize> for (usize,usize) {
  fn addAction_2(self , rsthis: & QActionGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QActionGroup9addActionERK5QIconRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeAction(QAction *)

/*
Removes the action from this group. The action will have no parent as a result.

See also QAction::setActionGroup().
*/
impl /*struct*/ QActionGroup {
  pub fn removeAction_0<RetType, T: QActionGroup_removeAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeAction_0(self);
    // return 1;
  }
}
pub trait QActionGroup_removeAction_0<RetType> {
  fn removeAction_0(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_removeAction_0<(/*void*/)> for (usize) {
  fn removeAction_0(self , rsthis: & QActionGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QActionGroup12removeActionEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:72
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * checkedAction() const

/*
Returns the currently checked action in the group, or 0 if none are checked.
*/
impl /*struct*/ QActionGroup {
  pub fn checkedAction_0<RetType, T: QActionGroup_checkedAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkedAction_0(self);
    // return 1;
  }
}
pub trait QActionGroup_checkedAction_0<RetType> {
  fn checkedAction_0(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_checkedAction_0<usize> for () {
  fn checkedAction_0(self , rsthis: & QActionGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QActionGroup13checkedActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:73
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isExclusive() const

/*

*/
impl /*struct*/ QActionGroup {
  pub fn isExclusive_0<RetType, T: QActionGroup_isExclusive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isExclusive_0(self);
    // return 1;
  }
}
pub trait QActionGroup_isExclusive_0<RetType> {
  fn isExclusive_0(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_isExclusive_0<bool> for () {
  fn isExclusive_0(self , rsthis: & QActionGroup) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QActionGroup11isExclusiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:74
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEnabled() const

/*

*/
impl /*struct*/ QActionGroup {
  pub fn isEnabled_0<RetType, T: QActionGroup_isEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEnabled_0(self);
    // return 1;
  }
}
pub trait QActionGroup_isEnabled_0<RetType> {
  fn isEnabled_0(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_isEnabled_0<bool> for () {
  fn isEnabled_0(self , rsthis: & QActionGroup) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QActionGroup9isEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:75
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isVisible() const

/*

*/
impl /*struct*/ QActionGroup {
  pub fn isVisible_0<RetType, T: QActionGroup_isVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isVisible_0(self);
    // return 1;
  }
}
pub trait QActionGroup_isVisible_0<RetType> {
  fn isVisible_0(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_isVisible_0<bool> for () {
  fn isVisible_0(self , rsthis: & QActionGroup) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QActionGroup9isVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEnabled(bool)

/*

*/
impl /*struct*/ QActionGroup {
  pub fn setEnabled_0<RetType, T: QActionGroup_setEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEnabled_0(self);
    // return 1;
  }
}
pub trait QActionGroup_setEnabled_0<RetType> {
  fn setEnabled_0(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_setEnabled_0<(/*void*/)> for (bool) {
  fn setEnabled_0(self , rsthis: & QActionGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QActionGroup10setEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:80
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setDisabled(bool)

/*
This is a convenience function for the enabled property, that is useful for signals--slots connections. If b is true the action group is disabled; otherwise it is enabled.
*/
impl /*struct*/ QActionGroup {
  pub fn setDisabled_0<RetType, T: QActionGroup_setDisabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDisabled_0(self);
    // return 1;
  }
}
pub trait QActionGroup_setDisabled_0<RetType> {
  fn setDisabled_0(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_setDisabled_0<(/*void*/)> for (bool) {
  fn setDisabled_0(self , rsthis: & QActionGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QActionGroup11setDisabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*

*/
impl /*struct*/ QActionGroup {
  pub fn setVisible_0<RetType, T: QActionGroup_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QActionGroup_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QActionGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QActionGroup10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setExclusive(bool)

/*

*/
impl /*struct*/ QActionGroup {
  pub fn setExclusive_0<RetType, T: QActionGroup_setExclusive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExclusive_0(self);
    // return 1;
  }
}
pub trait QActionGroup_setExclusive_0<RetType> {
  fn setExclusive_0(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_setExclusive_0<(/*void*/)> for (bool) {
  fn setExclusive_0(self , rsthis: & QActionGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QActionGroup12setExclusiveEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void triggered(QAction *)

/*
This signal is emitted when the given action in the action group is activated by the user; for example, when the user clicks a menu option, toolbar button, or presses an action's shortcut key combination.

Connect to this signal for command actions.

See also QAction::activate().
*/
impl /*struct*/ QActionGroup {
  pub fn triggered_0<RetType, T: QActionGroup_triggered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.triggered_0(self);
    // return 1;
  }
}
pub trait QActionGroup_triggered_0<RetType> {
  fn triggered_0(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_triggered_0<(/*void*/)> for (usize) {
  fn triggered_0(self , rsthis: & QActionGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QActionGroup9triggeredEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qactiongroup.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void hovered(QAction *)

/*
This signal is emitted when the given action in the action group is highlighted by the user; for example, when the user pauses with the cursor over a menu option, toolbar button, or presses an action's shortcut key combination.

See also QAction::activate().
*/
impl /*struct*/ QActionGroup {
  pub fn hovered_0<RetType, T: QActionGroup_hovered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hovered_0(self);
    // return 1;
  }
}
pub trait QActionGroup_hovered_0<RetType> {
  fn hovered_0(self , rsthis: & QActionGroup) -> RetType;
}
impl<'a> /*trait*/ QActionGroup_hovered_0<(/*void*/)> for (usize) {
  fn hovered_0(self , rsthis: & QActionGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QActionGroup7hoveredEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
