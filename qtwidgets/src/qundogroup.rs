

// mod ::widgets::QUndoGroup
// package qtwidgets
// /usr/include/qt/QtWidgets/qundogroup.h
// #include <qundogroup.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 66
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
#[derive(Default)] // class sizeof(QUndoGroup)=16
pub struct QUndoGroup {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QUndoGroup_ITF interface {
//    qtcore.QObject_ITF
//    QUndoGroup_PTR() *QUndoGroup
//}
//func (ptr *QUndoGroup) QUndoGroup_PTR() *QUndoGroup { return ptr }

impl /*struct*/ QUndoGroup {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QUndoGroup {
    return QUndoGroup{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QUndoGroup {
//  type Target = QUndoGroupBASE;
//
//  fn deref(&self) -> &QUndoGroupBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QUndoGroupBASE> for QUndoGroup {
//  fn as_ref(& self) -> & QUndoGroupBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qundogroup.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QUndoGroup {
  pub fn metaObject_0<RetType, T: QUndoGroup_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QUndoGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoGroup10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QUndoGroup(QObject *)

/*
Creates an empty QUndoGroup object with parent parent.

See also addStack().
*/
// QUndoGroup(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QUndoGroup {
  pub fn QUndoGroup_0<T: QUndoGroup_QUndoGroup_0>(value: T) -> QUndoGroup {
    let rsthis = value.QUndoGroup_0();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoGroup_QUndoGroup_0 {
  fn QUndoGroup_0(self) -> QUndoGroup;
}
// QUndoGroup(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUndoGroup_QUndoGroup_0 for (usize) {
  fn QUndoGroup_0(self) -> QUndoGroup {
    // unsafe{_ZN10QUndoGroupC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QUndoGroupC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUndoGroup{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QUndoGroup()

/*

*/
pub fn DeleteQUndoGroup(this :*mut QUndoGroup) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QUndoGroupD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qundogroup.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addStack(QUndoStack *)

/*
Adds stack to this group. The group does not take ownership of the stack. Another way of adding a stack to a group is by specifying the group as the stack's parent QObject in QUndoStack::QUndoStack(). In this case, the stack is deleted when the group is deleted, in the usual manner of QObjects.

See also removeStack(), stacks(), and QUndoStack::QUndoStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn addStack_0<RetType, T: QUndoGroup_addStack_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addStack_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_addStack_0<RetType> {
  fn addStack_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_addStack_0<(/*void*/)> for (usize) {
  fn addStack_0(self , rsthis: & QUndoGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoGroup8addStackEP10QUndoStack", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeStack(QUndoStack *)

/*
Removes stack from this group. If the stack was the active stack in the group, the active stack becomes 0.

See also addStack(), stacks(), and QUndoStack::~QUndoStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn removeStack_0<RetType, T: QUndoGroup_removeStack_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeStack_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_removeStack_0<RetType> {
  fn removeStack_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_removeStack_0<(/*void*/)> for (usize) {
  fn removeStack_0(self , rsthis: & QUndoGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoGroup11removeStackEP10QUndoStack", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:67
// index:0
// Public Visibility=Default Availability=Available
// [8] QUndoStack * activeStack() const

/*
Returns the active stack of this group.

If none of the stacks are active, or if the group is empty, this function returns 0.

See also setActiveStack() and QUndoStack::setActive().
*/
impl /*struct*/ QUndoGroup {
  pub fn activeStack_0<RetType, T: QUndoGroup_activeStack_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activeStack_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_activeStack_0<RetType> {
  fn activeStack_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_activeStack_0<usize> for () {
  fn activeStack_0(self , rsthis: & QUndoGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoGroup11activeStackEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:70
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * createUndoAction(QObject *, const QString &) const

/*
Creates an undo QAction object with parent parent.

Triggering this action will cause a call to QUndoStack::undo() on the active stack. The text of this action will always be the text of the command which will be undone in the next call to undo(), prefixed by prefix. If there is no command available for undo, if the group is empty or if none of the stacks are active, this action will be disabled.

If prefix is empty, the default template "Undo %1" is used instead of prefix. Before Qt 4.8, the prefix "Undo" was used by default.

See also createRedoAction(), canUndo(), and QUndoCommand::text().
*/
impl /*struct*/ QUndoGroup {
  pub fn createUndoAction_0<RetType, T: QUndoGroup_createUndoAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createUndoAction_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_createUndoAction_0<RetType> {
  fn createUndoAction_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_createUndoAction_0<usize> for (usize,usize) {
  fn createUndoAction_0(self , rsthis: & QUndoGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoGroup16createUndoActionEP7QObjectRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:72
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * createRedoAction(QObject *, const QString &) const

/*
Creates an redo QAction object with parent parent.

Triggering this action will cause a call to QUndoStack::redo() on the active stack. The text of this action will always be the text of the command which will be redone in the next call to redo(), prefixed by prefix. If there is no command available for redo, if the group is empty or if none of the stacks are active, this action will be disabled.

If prefix is empty, the default template "Redo %1" is used instead of prefix. Before Qt 4.8, the prefix "Redo" was used by default.

See also createUndoAction(), canRedo(), and QUndoCommand::text().
*/
impl /*struct*/ QUndoGroup {
  pub fn createRedoAction_0<RetType, T: QUndoGroup_createRedoAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createRedoAction_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_createRedoAction_0<RetType> {
  fn createRedoAction_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_createRedoAction_0<usize> for (usize,usize) {
  fn createRedoAction_0(self , rsthis: & QUndoGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoGroup16createRedoActionEP7QObjectRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:75
// index:0
// Public Visibility=Default Availability=Available
// [1] bool canUndo() const

/*
Returns the value of the active stack's QUndoStack::canUndo().

If none of the stacks are active, or if the group is empty, this function returns false.

See also canRedo() and setActiveStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn canUndo_0<RetType, T: QUndoGroup_canUndo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canUndo_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_canUndo_0<RetType> {
  fn canUndo_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_canUndo_0<bool> for () {
  fn canUndo_0(self , rsthis: & QUndoGroup) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoGroup7canUndoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:76
// index:0
// Public Visibility=Default Availability=Available
// [1] bool canRedo() const

/*
Returns the value of the active stack's QUndoStack::canRedo().

If none of the stacks are active, or if the group is empty, this function returns false.

See also canUndo() and setActiveStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn canRedo_0<RetType, T: QUndoGroup_canRedo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canRedo_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_canRedo_0<RetType> {
  fn canRedo_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_canRedo_0<bool> for () {
  fn canRedo_0(self , rsthis: & QUndoGroup) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoGroup7canRedoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:77
// index:0
// Public Visibility=Default Availability=Available
// [8] QString undoText() const

/*
Returns the value of the active stack's QUndoStack::undoText().

If none of the stacks are active, or if the group is empty, this function returns an empty string.

See also redoText() and setActiveStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn undoText_0<RetType, T: QUndoGroup_undoText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undoText_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_undoText_0<RetType> {
  fn undoText_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_undoText_0<usize> for () {
  fn undoText_0(self , rsthis: & QUndoGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoGroup8undoTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QString redoText() const

/*
Returns the value of the active stack's QUndoStack::redoText().

If none of the stacks are active, or if the group is empty, this function returns an empty string.

See also undoText() and setActiveStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn redoText_0<RetType, T: QUndoGroup_redoText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redoText_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_redoText_0<RetType> {
  fn redoText_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_redoText_0<usize> for () {
  fn redoText_0(self , rsthis: & QUndoGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoGroup8redoTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:79
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isClean() const

/*
Returns the value of the active stack's QUndoStack::isClean().

If none of the stacks are active, or if the group is empty, this function returns true.

See also setActiveStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn isClean_0<RetType, T: QUndoGroup_isClean_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isClean_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_isClean_0<RetType> {
  fn isClean_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_isClean_0<bool> for () {
  fn isClean_0(self , rsthis: & QUndoGroup) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoGroup7isCleanEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void undo()

/*
Calls QUndoStack::undo() on the active stack.

If none of the stacks are active, or if the group is empty, this function does nothing.

See also redo(), canUndo(), and setActiveStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn undo_0<RetType, T: QUndoGroup_undo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undo_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_undo_0<RetType> {
  fn undo_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_undo_0<(/*void*/)> for () {
  fn undo_0(self , rsthis: & QUndoGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QUndoGroup4undoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void redo()

/*
Calls QUndoStack::redo() on the active stack.

If none of the stacks are active, or if the group is empty, this function does nothing.

See also undo(), canRedo(), and setActiveStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn redo_0<RetType, T: QUndoGroup_redo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redo_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_redo_0<RetType> {
  fn redo_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_redo_0<(/*void*/)> for () {
  fn redo_0(self , rsthis: & QUndoGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QUndoGroup4redoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setActiveStack(QUndoStack *)

/*
Sets the active stack of this group to stack.

If the stack is not a member of this group, this function does nothing.

Synonymous with calling QUndoStack::setActive() on stack.

The actions returned by createUndoAction() and createRedoAction() will now behave in the same way as those returned by stack's QUndoStack::createUndoAction() and QUndoStack::createRedoAction().

See also QUndoStack::setActive() and activeStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn setActiveStack_0<RetType, T: QUndoGroup_setActiveStack_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setActiveStack_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_setActiveStack_0<RetType> {
  fn setActiveStack_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_setActiveStack_0<(/*void*/)> for (usize) {
  fn setActiveStack_0(self , rsthis: & QUndoGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoGroup14setActiveStackEP10QUndoStack", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activeStackChanged(QUndoStack *)

/*
This signal is emitted whenever the active stack of the group changes. This can happen when setActiveStack() or QUndoStack::setActive() is called, or when the active stack is removed form the group. stack is the new active stack. If no stack is active, stack is 0.

See also setActiveStack() and QUndoStack::setActive().
*/
impl /*struct*/ QUndoGroup {
  pub fn activeStackChanged_0<RetType, T: QUndoGroup_activeStackChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activeStackChanged_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_activeStackChanged_0<RetType> {
  fn activeStackChanged_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_activeStackChanged_0<(/*void*/)> for (usize) {
  fn activeStackChanged_0(self , rsthis: & QUndoGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoGroup18activeStackChangedEP10QUndoStack", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void indexChanged(int)

/*
This signal is emitted whenever the active stack emits QUndoStack::indexChanged() or the active stack changes.

idx is the new current index, or 0 if the active stack is 0.

See also QUndoStack::indexChanged() and setActiveStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn indexChanged_0<RetType, T: QUndoGroup_indexChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexChanged_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_indexChanged_0<RetType> {
  fn indexChanged_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_indexChanged_0<(/*void*/)> for (i32) {
  fn indexChanged_0(self , rsthis: & QUndoGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoGroup12indexChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cleanChanged(bool)

/*
This signal is emitted whenever the active stack emits QUndoStack::cleanChanged() or the active stack changes.

clean is the new state, or true if the active stack is 0.

See also QUndoStack::cleanChanged() and setActiveStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn cleanChanged_0<RetType, T: QUndoGroup_cleanChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cleanChanged_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_cleanChanged_0<RetType> {
  fn cleanChanged_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_cleanChanged_0<(/*void*/)> for (bool) {
  fn cleanChanged_0(self , rsthis: & QUndoGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoGroup12cleanChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void canUndoChanged(bool)

/*
This signal is emitted whenever the active stack emits QUndoStack::canUndoChanged() or the active stack changes.

canUndo is the new state, or false if the active stack is 0.

See also QUndoStack::canUndoChanged() and setActiveStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn canUndoChanged_0<RetType, T: QUndoGroup_canUndoChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canUndoChanged_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_canUndoChanged_0<RetType> {
  fn canUndoChanged_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_canUndoChanged_0<(/*void*/)> for (bool) {
  fn canUndoChanged_0(self , rsthis: & QUndoGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoGroup14canUndoChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void canRedoChanged(bool)

/*
This signal is emitted whenever the active stack emits QUndoStack::canRedoChanged() or the active stack changes.

canRedo is the new state, or false if the active stack is 0.

See also QUndoStack::canRedoChanged() and setActiveStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn canRedoChanged_0<RetType, T: QUndoGroup_canRedoChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canRedoChanged_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_canRedoChanged_0<RetType> {
  fn canRedoChanged_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_canRedoChanged_0<(/*void*/)> for (bool) {
  fn canRedoChanged_0(self , rsthis: & QUndoGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoGroup14canRedoChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void undoTextChanged(const QString &)

/*
This signal is emitted whenever the active stack emits QUndoStack::undoTextChanged() or the active stack changes.

undoText is the new state, or an empty string if the active stack is 0.

See also QUndoStack::undoTextChanged() and setActiveStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn undoTextChanged_0<RetType, T: QUndoGroup_undoTextChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undoTextChanged_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_undoTextChanged_0<RetType> {
  fn undoTextChanged_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_undoTextChanged_0<(/*void*/)> for (usize) {
  fn undoTextChanged_0(self , rsthis: & QUndoGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoGroup15undoTextChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundogroup.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void redoTextChanged(const QString &)

/*
This signal is emitted whenever the active stack emits QUndoStack::redoTextChanged() or the active stack changes.

redoText is the new state, or an empty string if the active stack is 0.

See also QUndoStack::redoTextChanged() and setActiveStack().
*/
impl /*struct*/ QUndoGroup {
  pub fn redoTextChanged_0<RetType, T: QUndoGroup_redoTextChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redoTextChanged_0(self);
    // return 1;
  }
}
pub trait QUndoGroup_redoTextChanged_0<RetType> {
  fn redoTextChanged_0(self , rsthis: & QUndoGroup) -> RetType;
}
impl<'a> /*trait*/ QUndoGroup_redoTextChanged_0<(/*void*/)> for (usize) {
  fn redoTextChanged_0(self , rsthis: & QUndoGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoGroup15redoTextChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
