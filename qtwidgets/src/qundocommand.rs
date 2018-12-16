

// mod ::widgets::QUndoCommand
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
// extern C begin: 23
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
#[derive(Default)] // class sizeof(QUndoCommand)=16
pub struct QUndoCommand {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QUndoCommand_ITF interface {
//    QUndoCommand_PTR() *QUndoCommand
//}
//func (ptr *QUndoCommand) QUndoCommand_PTR() *QUndoCommand { return ptr }

impl /*struct*/ QUndoCommand {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QUndoCommand {
    return QUndoCommand{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QUndoCommand {
//  type Target = QUndoCommandBASE;
//
//  fn deref(&self) -> &QUndoCommandBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QUndoCommandBASE> for QUndoCommand {
//  fn as_ref(& self) -> & QUndoCommandBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qundostack.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QUndoCommand(QUndoCommand *)

/*

*/
// QUndoCommand(QUndoCommand *) ctx.fn_proto_cpp
impl /*struct*/ QUndoCommand {
  pub fn QUndoCommand_0<T: QUndoCommand_QUndoCommand_0>(value: T) -> QUndoCommand {
    let rsthis = value.QUndoCommand_0();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoCommand_QUndoCommand_0 {
  fn QUndoCommand_0(self) -> QUndoCommand;
}
// QUndoCommand(QUndoCommand *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUndoCommand_QUndoCommand_0 for (usize) {
  fn QUndoCommand_0(self) -> QUndoCommand {
    // unsafe{_ZN12QUndoCommandC2EPS_()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QUndoCommandC2EPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUndoCommand{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:61
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QUndoCommand(const QString &, QUndoCommand *)

/*

*/
// QUndoCommand(const QString &, QUndoCommand *) ctx.fn_proto_cpp
impl /*struct*/ QUndoCommand {
  pub fn QUndoCommand_1<T: QUndoCommand_QUndoCommand_1>(value: T) -> QUndoCommand {
    let rsthis = value.QUndoCommand_1();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoCommand_QUndoCommand_1 {
  fn QUndoCommand_1(self) -> QUndoCommand;
}
// QUndoCommand(const QString &, QUndoCommand *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUndoCommand_QUndoCommand_1 for (usize,usize) {
  fn QUndoCommand_1(self) -> QUndoCommand {
    // unsafe{_ZN12QUndoCommandC2ERK7QStringPS_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QUndoCommandC2ERK7QStringPS_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUndoCommand{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QUndoCommand()

/*

*/
pub fn DeleteQUndoCommand(this :*mut QUndoCommand) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QUndoCommandD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qundostack.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void undo()

/*
Undoes the command below the current command by calling QUndoCommand::undo(). Decrements the current command index.

If the stack is empty, or if the bottom command on the stack has already been undone, this function does nothing.

After the command is undone, if QUndoCommand::isObsolete() returns true, then the command will be deleted from the stack. Additionally, if the clean index is greater than or equal to the current command index, then the clean index is reset.

See also redo() and index().
*/
impl /*struct*/ QUndoCommand {
  pub fn undo_0<RetType, T: QUndoCommand_undo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undo_0(self);
    // return 1;
  }
}
pub trait QUndoCommand_undo_0<RetType> {
  fn undo_0(self , rsthis: & QUndoCommand) -> RetType;
}
impl<'a> /*trait*/ QUndoCommand_undo_0<(/*void*/)> for () {
  fn undo_0(self , rsthis: & QUndoCommand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QUndoCommand4undoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void redo()

/*
Redoes the current command by calling QUndoCommand::redo(). Increments the current command index.

If the stack is empty, or if the top command on the stack has already been redone, this function does nothing.

If QUndoCommand::isObsolete() returns true for the current command, then the command will be deleted from the stack. Additionally, if the clean index is greater than or equal to the current command index, then the clean index is reset.

See also undo() and index().
*/
impl /*struct*/ QUndoCommand {
  pub fn redo_0<RetType, T: QUndoCommand_redo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redo_0(self);
    // return 1;
  }
}
pub trait QUndoCommand_redo_0<RetType> {
  fn redo_0(self , rsthis: & QUndoCommand) -> RetType;
}
impl<'a> /*trait*/ QUndoCommand_redo_0<(/*void*/)> for () {
  fn redo_0(self , rsthis: & QUndoCommand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QUndoCommand4redoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:67
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*
Returns the text of the command at index idx.

See also beginMacro().
*/
impl /*struct*/ QUndoCommand {
  pub fn text_0<RetType, T: QUndoCommand_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QUndoCommand_text_0<RetType> {
  fn text_0(self , rsthis: & QUndoCommand) -> RetType;
}
impl<'a> /*trait*/ QUndoCommand_text_0<usize> for () {
  fn text_0(self , rsthis: & QUndoCommand) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QUndoCommand4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:68
// index:0
// Public Visibility=Default Availability=Available
// [8] QString actionText() const

/*

*/
impl /*struct*/ QUndoCommand {
  pub fn actionText_0<RetType, T: QUndoCommand_actionText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionText_0(self);
    // return 1;
  }
}
pub trait QUndoCommand_actionText_0<RetType> {
  fn actionText_0(self , rsthis: & QUndoCommand) -> RetType;
}
impl<'a> /*trait*/ QUndoCommand_actionText_0<usize> for () {
  fn actionText_0(self , rsthis: & QUndoCommand) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QUndoCommand10actionTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*

*/
impl /*struct*/ QUndoCommand {
  pub fn setText_0<RetType, T: QUndoCommand_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QUndoCommand_setText_0<RetType> {
  fn setText_0(self , rsthis: & QUndoCommand) -> RetType;
}
impl<'a> /*trait*/ QUndoCommand_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QUndoCommand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QUndoCommand7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:71
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isObsolete() const

/*

*/
impl /*struct*/ QUndoCommand {
  pub fn isObsolete_0<RetType, T: QUndoCommand_isObsolete_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isObsolete_0(self);
    // return 1;
  }
}
pub trait QUndoCommand_isObsolete_0<RetType> {
  fn isObsolete_0(self , rsthis: & QUndoCommand) -> RetType;
}
impl<'a> /*trait*/ QUndoCommand_isObsolete_0<bool> for () {
  fn isObsolete_0(self , rsthis: & QUndoCommand) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QUndoCommand10isObsoleteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setObsolete(bool)

/*

*/
impl /*struct*/ QUndoCommand {
  pub fn setObsolete_0<RetType, T: QUndoCommand_setObsolete_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setObsolete_0(self);
    // return 1;
  }
}
pub trait QUndoCommand_setObsolete_0<RetType> {
  fn setObsolete_0(self , rsthis: & QUndoCommand) -> RetType;
}
impl<'a> /*trait*/ QUndoCommand_setObsolete_0<(/*void*/)> for (bool) {
  fn setObsolete_0(self , rsthis: & QUndoCommand) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QUndoCommand11setObsoleteEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int id() const

/*

*/
impl /*struct*/ QUndoCommand {
  pub fn id_0<RetType, T: QUndoCommand_id_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.id_0(self);
    // return 1;
  }
}
pub trait QUndoCommand_id_0<RetType> {
  fn id_0(self , rsthis: & QUndoCommand) -> RetType;
}
impl<'a> /*trait*/ QUndoCommand_id_0<i32> for () {
  fn id_0(self , rsthis: & QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QUndoCommand2idEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:75
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool mergeWith(const QUndoCommand *)

/*

*/
impl /*struct*/ QUndoCommand {
  pub fn mergeWith_0<RetType, T: QUndoCommand_mergeWith_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mergeWith_0(self);
    // return 1;
  }
}
pub trait QUndoCommand_mergeWith_0<RetType> {
  fn mergeWith_0(self , rsthis: & QUndoCommand) -> RetType;
}
impl<'a> /*trait*/ QUndoCommand_mergeWith_0<bool> for (usize) {
  fn mergeWith_0(self , rsthis: & QUndoCommand) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QUndoCommand9mergeWithEPKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:77
// index:0
// Public Visibility=Default Availability=Available
// [4] int childCount() const

/*

*/
impl /*struct*/ QUndoCommand {
  pub fn childCount_0<RetType, T: QUndoCommand_childCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childCount_0(self);
    // return 1;
  }
}
pub trait QUndoCommand_childCount_0<RetType> {
  fn childCount_0(self , rsthis: & QUndoCommand) -> RetType;
}
impl<'a> /*trait*/ QUndoCommand_childCount_0<i32> for () {
  fn childCount_0(self , rsthis: & QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QUndoCommand10childCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qundostack.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] const QUndoCommand * child(int) const

/*

*/
impl /*struct*/ QUndoCommand {
  pub fn child_0<RetType, T: QUndoCommand_child_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.child_0(self);
    // return 1;
  }
}
pub trait QUndoCommand_child_0<RetType> {
  fn child_0(self , rsthis: & QUndoCommand) -> RetType;
}
impl<'a> /*trait*/ QUndoCommand_child_0<usize> for (i32) {
  fn child_0(self , rsthis: & QUndoCommand) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QUndoCommand5childEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
