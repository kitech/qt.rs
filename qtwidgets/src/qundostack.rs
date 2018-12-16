

// mod ::widgets::QUndoStack
// package qtwidgets
// /usr/include/qt/QtWidgets/qundostack.h
// #include <qundostack.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 14
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
#[derive(Default)] // class sizeof(QUndoStack)=16
pub struct QUndoStack {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QUndoStack_ITF interface {
//    qtcore.QObject_ITF
//    QUndoStack_PTR() *QUndoStack
//}
//func (ptr *QUndoStack) QUndoStack_PTR() *QUndoStack { return ptr }

impl /*struct*/ QUndoStack {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QUndoStack {
    return QUndoStack{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QUndoStack {
//  type Target = QUndoStackBASE;
//
//  fn deref(&self) -> &QUndoStackBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QUndoStackBASE> for QUndoStack {
//  fn as_ref(& self) -> & QUndoStackBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qundostack.h:89
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QUndoStack {
  pub fn metaObject_0<RetType, T: QUndoStack_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QUndoStack_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QUndoStack) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QUndoStack(QObject *)

/*
Constructs an empty undo stack with the parent parent. The stack will initially be in the clean state. If parent is a QUndoGroup object, the stack is automatically added to the group.

See also push().
*/
// QUndoStack(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QUndoStack {
  pub fn QUndoStack_0<T: QUndoStack_QUndoStack_0>(value: T) -> QUndoStack {
    let rsthis = value.QUndoStack_0();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoStack_QUndoStack_0 {
  fn QUndoStack_0(self) -> QUndoStack;
}
// QUndoStack(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUndoStack_QUndoStack_0 for (usize) {
  fn QUndoStack_0(self) -> QUndoStack {
    // unsafe{_ZN10QUndoStackC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QUndoStackC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUndoStack{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:96
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QUndoStack()

/*

*/
pub fn DeleteQUndoStack(this :*mut QUndoStack) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QUndoStackD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qundostack.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the command stack by deleting all commands on it, and returns the stack to the clean state.

Commands are not undone or redone; the state of the edited object remains unchanged.

This function is usually used when the contents of the document are abandoned.

See also QUndoStack().
*/
impl /*struct*/ QUndoStack {
  pub fn clear_0<RetType, T: QUndoStack_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QUndoStack_clear_0<RetType> {
  fn clear_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QUndoStack5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void push(QUndoCommand *)

/*
Pushes cmd on the stack or merges it with the most recently executed command. In either case, executes cmd by calling its redo() function.

If cmd's id is not -1, and if the id is the same as that of the most recently executed command, QUndoStack will attempt to merge the two commands by calling QUndoCommand::mergeWith() on the most recently executed command. If QUndoCommand::mergeWith() returns true, cmd is deleted.

After calling QUndoCommand::redo() and, if applicable, QUndoCommand::mergeWith(), QUndoCommand::isObsolete() will be called for cmd or the merged command. If QUndoCommand::isObsolete() returns true, then cmd or the merged command will be deleted from the stack.

In all other cases cmd is simply pushed on the stack.

If commands were undone before cmd was pushed, the current command and all commands above it are deleted. Hence cmd always ends up being the top-most on the stack.

Once a command is pushed, the stack takes ownership of it. There are no getters to return the command, since modifying it after it has been executed will almost always lead to corruption of the document's state.

See also QUndoCommand::id() and QUndoCommand::mergeWith().
*/
impl /*struct*/ QUndoStack {
  pub fn push_0<RetType, T: QUndoStack_push_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.push_0(self);
    // return 1;
  }
}
pub trait QUndoStack_push_0<RetType> {
  fn push_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_push_0<(/*void*/)> for (usize) {
  fn push_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoStack4pushEP12QUndoCommand", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:101
// index:0
// Public Visibility=Default Availability=Available
// [1] bool canUndo() const

/*
Returns true if there is a command available for undo; otherwise returns false.

This function returns false if the stack is empty, or if the bottom command on the stack has already been undone.

Synonymous with index() == 0.

See also index() and canRedo().
*/
impl /*struct*/ QUndoStack {
  pub fn canUndo_0<RetType, T: QUndoStack_canUndo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canUndo_0(self);
    // return 1;
  }
}
pub trait QUndoStack_canUndo_0<RetType> {
  fn canUndo_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_canUndo_0<bool> for () {
  fn canUndo_0(self , rsthis: & QUndoStack) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack7canUndoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:102
// index:0
// Public Visibility=Default Availability=Available
// [1] bool canRedo() const

/*
Returns true if there is a command available for redo; otherwise returns false.

This function returns false if the stack is empty or if the top command on the stack has already been redone.

Synonymous with index() == count().

See also index() and canUndo().
*/
impl /*struct*/ QUndoStack {
  pub fn canRedo_0<RetType, T: QUndoStack_canRedo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canRedo_0(self);
    // return 1;
  }
}
pub trait QUndoStack_canRedo_0<RetType> {
  fn canRedo_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_canRedo_0<bool> for () {
  fn canRedo_0(self , rsthis: & QUndoStack) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack7canRedoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] QString undoText() const

/*
Returns the text of the command which will be undone in the next call to undo().

See also QUndoCommand::actionText() and redoText().
*/
impl /*struct*/ QUndoStack {
  pub fn undoText_0<RetType, T: QUndoStack_undoText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undoText_0(self);
    // return 1;
  }
}
pub trait QUndoStack_undoText_0<RetType> {
  fn undoText_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_undoText_0<usize> for () {
  fn undoText_0(self , rsthis: & QUndoStack) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack8undoTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:104
// index:0
// Public Visibility=Default Availability=Available
// [8] QString redoText() const

/*
Returns the text of the command which will be redone in the next call to redo().

See also QUndoCommand::actionText() and undoText().
*/
impl /*struct*/ QUndoStack {
  pub fn redoText_0<RetType, T: QUndoStack_redoText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redoText_0(self);
    // return 1;
  }
}
pub trait QUndoStack_redoText_0<RetType> {
  fn redoText_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_redoText_0<usize> for () {
  fn redoText_0(self , rsthis: & QUndoStack) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack8redoTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:106
// index:0
// Public Visibility=Default Availability=Available
// [4] int count() const

/*
Returns the number of commands on the stack. Macro commands are counted as one command.

See also index(), setIndex(), and command().
*/
impl /*struct*/ QUndoStack {
  pub fn count_0<RetType, T: QUndoStack_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QUndoStack_count_0<RetType> {
  fn count_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_count_0<i32> for () {
  fn count_0(self , rsthis: & QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:107
// index:0
// Public Visibility=Default Availability=Available
// [4] int index() const

/*
Returns the index of the current command. This is the command that will be executed on the next call to redo(). It is not always the top-most command on the stack, since a number of commands may have been undone.

See also setIndex(), undo(), redo(), and count().
*/
impl /*struct*/ QUndoStack {
  pub fn index_0<RetType, T: QUndoStack_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.index_0(self);
    // return 1;
  }
}
pub trait QUndoStack_index_0<RetType> {
  fn index_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_index_0<i32> for () {
  fn index_0(self , rsthis: & QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack5indexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:108
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text(int) const

/*
Returns the text of the command at index idx.

See also beginMacro().
*/
impl /*struct*/ QUndoStack {
  pub fn text_0<RetType, T: QUndoStack_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QUndoStack_text_0<RetType> {
  fn text_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_text_0<usize> for (i32) {
  fn text_0(self , rsthis: & QUndoStack) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack4textEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:111
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * createUndoAction(QObject *, const QString &) const

/*
Creates an undo QAction object with the given parent.

Triggering this action will cause a call to undo(). The text of this action is the text of the command which will be undone in the next call to undo(), prefixed by the specified prefix. If there is no command available for undo, this action will be disabled.

If prefix is empty, the default template "Undo %1" is used instead of prefix. Before Qt 4.8, the prefix "Undo" was used by default.

See also createRedoAction(), canUndo(), and QUndoCommand::text().
*/
impl /*struct*/ QUndoStack {
  pub fn createUndoAction_0<RetType, T: QUndoStack_createUndoAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createUndoAction_0(self);
    // return 1;
  }
}
pub trait QUndoStack_createUndoAction_0<RetType> {
  fn createUndoAction_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_createUndoAction_0<usize> for (usize,usize) {
  fn createUndoAction_0(self , rsthis: & QUndoStack) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack16createUndoActionEP7QObjectRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:113
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * createRedoAction(QObject *, const QString &) const

/*
Creates an redo QAction object with the given parent.

Triggering this action will cause a call to redo(). The text of this action is the text of the command which will be redone in the next call to redo(), prefixed by the specified prefix. If there is no command available for redo, this action will be disabled.

If prefix is empty, the default template "Redo %1" is used instead of prefix. Before Qt 4.8, the prefix "Redo" was used by default.

See also createUndoAction(), canRedo(), and QUndoCommand::text().
*/
impl /*struct*/ QUndoStack {
  pub fn createRedoAction_0<RetType, T: QUndoStack_createRedoAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createRedoAction_0(self);
    // return 1;
  }
}
pub trait QUndoStack_createRedoAction_0<RetType> {
  fn createRedoAction_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_createRedoAction_0<usize> for (usize,usize) {
  fn createRedoAction_0(self , rsthis: & QUndoStack) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack16createRedoActionEP7QObjectRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:117
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isActive() const

/*

*/
impl /*struct*/ QUndoStack {
  pub fn isActive_0<RetType, T: QUndoStack_isActive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isActive_0(self);
    // return 1;
  }
}
pub trait QUndoStack_isActive_0<RetType> {
  fn isActive_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_isActive_0<bool> for () {
  fn isActive_0(self , rsthis: & QUndoStack) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack8isActiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:118
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isClean() const

/*
If the stack is in the clean state, returns true; otherwise returns false.

See also setClean() and cleanIndex().
*/
impl /*struct*/ QUndoStack {
  pub fn isClean_0<RetType, T: QUndoStack_isClean_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isClean_0(self);
    // return 1;
  }
}
pub trait QUndoStack_isClean_0<RetType> {
  fn isClean_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_isClean_0<bool> for () {
  fn isClean_0(self , rsthis: & QUndoStack) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack7isCleanEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:119
// index:0
// Public Visibility=Default Availability=Available
// [4] int cleanIndex() const

/*
Returns the clean index. This is the index at which setClean() was called.

A stack may not have a clean index. This happens if a document is saved, some commands are undone, then a new command is pushed. Since push() deletes all the undone commands before pushing the new command, the stack can't return to the clean state again. In this case, this function returns -1. The -1 may also be returned after an explicit call to resetClean().

See also isClean() and setClean().
*/
impl /*struct*/ QUndoStack {
  pub fn cleanIndex_0<RetType, T: QUndoStack_cleanIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cleanIndex_0(self);
    // return 1;
  }
}
pub trait QUndoStack_cleanIndex_0<RetType> {
  fn cleanIndex_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_cleanIndex_0<i32> for () {
  fn cleanIndex_0(self , rsthis: & QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack10cleanIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void beginMacro(const QString &)

/*
Begins composition of a macro command with the given text description.

An empty command described by the specified text is pushed on the stack. Any subsequent commands pushed on the stack will be appended to the empty command's children until endMacro() is called.

Calls to beginMacro() and endMacro() may be nested, but every call to beginMacro() must have a matching call to endMacro().

While a macro is being composed, the stack is disabled. This means that:


indexChanged() and cleanChanged() are not emitted,
canUndo() and canRedo() return false,
calling undo() or redo() has no effect,
the undo/redo actions are disabled.


The stack becomes enabled and appropriate signals are emitted when endMacro() is called for the outermost macro.


  stack.beginMacro("insert red text");
  stack.push(new InsertText(document, idx, text));
  stack.push(new SetColor(document, idx, text.length(), Qt::red));
  stack.endMacro(); // indexChanged() is emitted



This code is equivalent to:


  QUndoCommand *insertRed = new QUndoCommand(); // an empty command
  insertRed->setText("insert red text");

  new InsertText(document, idx, text, insertRed); // becomes child of insertRed
  new SetColor(document, idx, text.length(), Qt::red, insertRed);

  stack.push(insertRed);



See also endMacro().
*/
impl /*struct*/ QUndoStack {
  pub fn beginMacro_0<RetType, T: QUndoStack_beginMacro_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginMacro_0(self);
    // return 1;
  }
}
pub trait QUndoStack_beginMacro_0<RetType> {
  fn beginMacro_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_beginMacro_0<(/*void*/)> for (usize) {
  fn beginMacro_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoStack10beginMacroERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:122
// index:0
// Public Visibility=Default Availability=Available
// [-2] void endMacro()

/*
Ends composition of a macro command.

If this is the outermost macro in a set nested macros, this function emits indexChanged() once for the entire macro command.

See also beginMacro().
*/
impl /*struct*/ QUndoStack {
  pub fn endMacro_0<RetType, T: QUndoStack_endMacro_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endMacro_0(self);
    // return 1;
  }
}
pub trait QUndoStack_endMacro_0<RetType> {
  fn endMacro_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_endMacro_0<(/*void*/)> for () {
  fn endMacro_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QUndoStack8endMacroEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:124
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUndoLimit(int)

/*

*/
impl /*struct*/ QUndoStack {
  pub fn setUndoLimit_0<RetType, T: QUndoStack_setUndoLimit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUndoLimit_0(self);
    // return 1;
  }
}
pub trait QUndoStack_setUndoLimit_0<RetType> {
  fn setUndoLimit_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_setUndoLimit_0<(/*void*/)> for (i32) {
  fn setUndoLimit_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoStack12setUndoLimitEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:125
// index:0
// Public Visibility=Default Availability=Available
// [4] int undoLimit() const

/*

*/
impl /*struct*/ QUndoStack {
  pub fn undoLimit_0<RetType, T: QUndoStack_undoLimit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undoLimit_0(self);
    // return 1;
  }
}
pub trait QUndoStack_undoLimit_0<RetType> {
  fn undoLimit_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_undoLimit_0<i32> for () {
  fn undoLimit_0(self , rsthis: & QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack9undoLimitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:127
// index:0
// Public Visibility=Default Availability=Available
// [8] const QUndoCommand * command(int) const

/*
Returns a const pointer to the command at index.

This function returns a const pointer, because modifying a command, once it has been pushed onto the stack and executed, almost always causes corruption of the state of the document, if the command is later undone or redone.

This function was introduced in  Qt 4.4.

See also QUndoCommand::child().
*/
impl /*struct*/ QUndoStack {
  pub fn command_0<RetType, T: QUndoStack_command_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.command_0(self);
    // return 1;
  }
}
pub trait QUndoStack_command_0<RetType> {
  fn command_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_command_0<usize> for (i32) {
  fn command_0(self , rsthis: & QUndoStack) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QUndoStack7commandEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setClean()

/*
Marks the stack as clean and emits cleanChanged() if the stack was not already clean.

This is typically called when a document is saved, for example.

Whenever the stack returns to this state through the use of undo/redo commands, it emits the signal cleanChanged(). This signal is also emitted when the stack leaves the clean state.

See also isClean(), resetClean(), and cleanIndex().
*/
impl /*struct*/ QUndoStack {
  pub fn setClean_0<RetType, T: QUndoStack_setClean_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setClean_0(self);
    // return 1;
  }
}
pub trait QUndoStack_setClean_0<RetType> {
  fn setClean_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_setClean_0<(/*void*/)> for () {
  fn setClean_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QUndoStack8setCleanEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:131
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetClean()

/*
Leaves the clean state and emits cleanChanged() if the stack was clean. This method resets the clean index to -1.

This is typically called in the following cases, when a document has been:


created basing on some template and has not been saved, so no filename has been associated with the document yet.
restored from a backup file.
changed outside of the editor and the user did not reload it.


This function was introduced in  Qt 5.8.

See also isClean(), setClean(), and cleanIndex().
*/
impl /*struct*/ QUndoStack {
  pub fn resetClean_0<RetType, T: QUndoStack_resetClean_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetClean_0(self);
    // return 1;
  }
}
pub trait QUndoStack_resetClean_0<RetType> {
  fn resetClean_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_resetClean_0<(/*void*/)> for () {
  fn resetClean_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QUndoStack10resetCleanEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIndex(int)

/*
Repeatedly calls undo() or redo() until the current command index reaches idx. This function can be used to roll the state of the document forwards of backwards. indexChanged() is emitted only once.

See also index(), count(), undo(), and redo().
*/
impl /*struct*/ QUndoStack {
  pub fn setIndex_0<RetType, T: QUndoStack_setIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIndex_0(self);
    // return 1;
  }
}
pub trait QUndoStack_setIndex_0<RetType> {
  fn setIndex_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_setIndex_0<(/*void*/)> for (i32) {
  fn setIndex_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoStack8setIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void undo()

/*
Undoes the command below the current command by calling QUndoCommand::undo(). Decrements the current command index.

If the stack is empty, or if the bottom command on the stack has already been undone, this function does nothing.

After the command is undone, if QUndoCommand::isObsolete() returns true, then the command will be deleted from the stack. Additionally, if the clean index is greater than or equal to the current command index, then the clean index is reset.

See also redo() and index().
*/
impl /*struct*/ QUndoStack {
  pub fn undo_0<RetType, T: QUndoStack_undo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undo_0(self);
    // return 1;
  }
}
pub trait QUndoStack_undo_0<RetType> {
  fn undo_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_undo_0<(/*void*/)> for () {
  fn undo_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QUndoStack4undoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void redo()

/*
Redoes the current command by calling QUndoCommand::redo(). Increments the current command index.

If the stack is empty, or if the top command on the stack has already been redone, this function does nothing.

If QUndoCommand::isObsolete() returns true for the current command, then the command will be deleted from the stack. Additionally, if the clean index is greater than or equal to the current command index, then the clean index is reset.

See also undo() and index().
*/
impl /*struct*/ QUndoStack {
  pub fn redo_0<RetType, T: QUndoStack_redo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redo_0(self);
    // return 1;
  }
}
pub trait QUndoStack_redo_0<RetType> {
  fn redo_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_redo_0<(/*void*/)> for () {
  fn redo_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QUndoStack4redoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setActive(bool)

/*

*/
impl /*struct*/ QUndoStack {
  pub fn setActive_0<RetType, T: QUndoStack_setActive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setActive_0(self);
    // return 1;
  }
}
pub trait QUndoStack_setActive_0<RetType> {
  fn setActive_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_setActive_0<(/*void*/)> for (bool) {
  fn setActive_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoStack9setActiveEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:138
// index:0
// Public Visibility=Default Availability=Available
// [-2] void indexChanged(int)

/*
This signal is emitted whenever a command modifies the state of the document. This happens when a command is undone or redone. When a macro command is undone or redone, or setIndex() is called, this signal is emitted only once.

idx specifies the index of the current command, ie. the command which will be executed on the next call to redo().

See also index() and setIndex().
*/
impl /*struct*/ QUndoStack {
  pub fn indexChanged_0<RetType, T: QUndoStack_indexChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexChanged_0(self);
    // return 1;
  }
}
pub trait QUndoStack_indexChanged_0<RetType> {
  fn indexChanged_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_indexChanged_0<(/*void*/)> for (i32) {
  fn indexChanged_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoStack12indexChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cleanChanged(bool)

/*
This signal is emitted whenever the stack enters or leaves the clean state. If clean is true, the stack is in a clean state; otherwise this signal indicates that it has left the clean state.

See also isClean() and setClean().
*/
impl /*struct*/ QUndoStack {
  pub fn cleanChanged_0<RetType, T: QUndoStack_cleanChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cleanChanged_0(self);
    // return 1;
  }
}
pub trait QUndoStack_cleanChanged_0<RetType> {
  fn cleanChanged_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_cleanChanged_0<(/*void*/)> for (bool) {
  fn cleanChanged_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoStack12cleanChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void canUndoChanged(bool)

/*
This signal is emitted whenever the value of canUndo() changes. It is used to enable or disable the undo action returned by createUndoAction(). canUndo specifies the new value.
*/
impl /*struct*/ QUndoStack {
  pub fn canUndoChanged_0<RetType, T: QUndoStack_canUndoChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canUndoChanged_0(self);
    // return 1;
  }
}
pub trait QUndoStack_canUndoChanged_0<RetType> {
  fn canUndoChanged_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_canUndoChanged_0<(/*void*/)> for (bool) {
  fn canUndoChanged_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoStack14canUndoChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:141
// index:0
// Public Visibility=Default Availability=Available
// [-2] void canRedoChanged(bool)

/*
This signal is emitted whenever the value of canRedo() changes. It is used to enable or disable the redo action returned by createRedoAction(). canRedo specifies the new value.
*/
impl /*struct*/ QUndoStack {
  pub fn canRedoChanged_0<RetType, T: QUndoStack_canRedoChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canRedoChanged_0(self);
    // return 1;
  }
}
pub trait QUndoStack_canRedoChanged_0<RetType> {
  fn canRedoChanged_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_canRedoChanged_0<(/*void*/)> for (bool) {
  fn canRedoChanged_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoStack14canRedoChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:142
// index:0
// Public Visibility=Default Availability=Available
// [-2] void undoTextChanged(const QString &)

/*
This signal is emitted whenever the value of undoText() changes. It is used to update the text property of the undo action returned by createUndoAction(). undoText specifies the new text.
*/
impl /*struct*/ QUndoStack {
  pub fn undoTextChanged_0<RetType, T: QUndoStack_undoTextChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undoTextChanged_0(self);
    // return 1;
  }
}
pub trait QUndoStack_undoTextChanged_0<RetType> {
  fn undoTextChanged_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_undoTextChanged_0<(/*void*/)> for (usize) {
  fn undoTextChanged_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoStack15undoTextChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void redoTextChanged(const QString &)

/*
This signal is emitted whenever the value of redoText() changes. It is used to update the text property of the redo action returned by createRedoAction(). redoText specifies the new text.
*/
impl /*struct*/ QUndoStack {
  pub fn redoTextChanged_0<RetType, T: QUndoStack_redoTextChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redoTextChanged_0(self);
    // return 1;
  }
}
pub trait QUndoStack_redoTextChanged_0<RetType> {
  fn redoTextChanged_0(self , rsthis: & QUndoStack) -> RetType;
}
impl<'a> /*trait*/ QUndoStack_redoTextChanged_0<(/*void*/)> for (usize) {
  fn redoTextChanged_0(self , rsthis: & QUndoStack) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QUndoStack15redoTextChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
