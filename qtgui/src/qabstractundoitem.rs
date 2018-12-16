

// mod ::gui::QAbstractUndoItem
// package qtgui
// /usr/include/qt/QtGui/qtextdocument.h
// #include <qtextdocument.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 26
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QAbstractUndoItem)=8
pub struct QAbstractUndoItem {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractUndoItem_ITF interface {
//    QAbstractUndoItem_PTR() *QAbstractUndoItem
//}
//func (ptr *QAbstractUndoItem) QAbstractUndoItem_PTR() *QAbstractUndoItem { return ptr }

impl /*struct*/ QAbstractUndoItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractUndoItem {
    return QAbstractUndoItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractUndoItem {
//  type Target = QAbstractUndoItemBASE;
//
//  fn deref(&self) -> &QAbstractUndoItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractUndoItemBASE> for QAbstractUndoItem {
//  fn as_ref(& self) -> & QAbstractUndoItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextdocument.h:86
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractUndoItem()

/*

*/
pub fn DeleteQAbstractUndoItem(this :*mut QAbstractUndoItem) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QAbstractUndoItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtextdocument.h:87
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void undo()

/*
Undoes the last editing operation on the document if undo is available. The provided cursor is positioned at the end of the location where the edition operation was undone.

See the Qt Undo Framework documentation for details.

This function was introduced in  Qt 4.2.

See also undoAvailable() and isUndoRedoEnabled().
*/
impl /*struct*/ QAbstractUndoItem {
  pub fn undo_0<RetType, T: QAbstractUndoItem_undo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undo_0(self);
    // return 1;
  }
}
pub trait QAbstractUndoItem_undo_0<RetType> {
  fn undo_0(self , rsthis: & QAbstractUndoItem) -> RetType;
}
impl<'a> /*trait*/ QAbstractUndoItem_undo_0<(/*void*/)> for () {
  fn undo_0(self , rsthis: & QAbstractUndoItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractUndoItem4undoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:88
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void redo()

/*
Redoes the last editing operation on the document if redo is available.

The provided cursor is positioned at the end of the location where the edition operation was redone.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QAbstractUndoItem {
  pub fn redo_0<RetType, T: QAbstractUndoItem_redo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redo_0(self);
    // return 1;
  }
}
pub trait QAbstractUndoItem_redo_0<RetType> {
  fn redo_0(self , rsthis: & QAbstractUndoItem) -> RetType;
}
impl<'a> /*trait*/ QAbstractUndoItem_redo_0<(/*void*/)> for () {
  fn redo_0(self , rsthis: & QAbstractUndoItem) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QAbstractUndoItem4redoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
