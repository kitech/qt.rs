

// mod ::gui::QTextCursor
// package qtgui
// /usr/include/qt/QtGui/qtextcursor.h
// #include <qtextcursor.h>
// #include <QtGui>

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
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QTextCursor)=8
pub struct QTextCursor {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextCursor_ITF interface {
//    QTextCursor_PTR() *QTextCursor
//}
//func (ptr *QTextCursor) QTextCursor_PTR() *QTextCursor { return ptr }

impl /*struct*/ QTextCursor {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextCursor {
    return QTextCursor{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextCursor {
//  type Target = QTextCursorBASE;
//
//  fn deref(&self) -> &QTextCursorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextCursorBASE> for QTextCursor {
//  fn as_ref(& self) -> & QTextCursorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextcursor.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextCursor()

/*
Constructs a null cursor.
*/
// QTextCursor() ctx.fn_proto_cpp
impl /*struct*/ QTextCursor {
  pub fn QTextCursor_0<T: QTextCursor_QTextCursor_0>(value: T) -> QTextCursor {
    let rsthis = value.QTextCursor_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCursor_QTextCursor_0 {
  fn QTextCursor_0(self) -> QTextCursor;
}
// QTextCursor() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextCursor_QTextCursor_0 for () {
  fn QTextCursor_0(self) -> QTextCursor {
    // unsafe{_ZN11QTextCursorC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextCursorC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextCursor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:70
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTextCursor(QTextDocument *)

/*
Constructs a null cursor.
*/
// QTextCursor(QTextDocument *) ctx.fn_proto_cpp
impl /*struct*/ QTextCursor {
  pub fn QTextCursor_1<T: QTextCursor_QTextCursor_1>(value: T) -> QTextCursor {
    let rsthis = value.QTextCursor_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCursor_QTextCursor_1 {
  fn QTextCursor_1(self) -> QTextCursor;
}
// QTextCursor(QTextDocument *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextCursor_QTextCursor_1 for (usize) {
  fn QTextCursor_1(self) -> QTextCursor {
    // unsafe{_ZN11QTextCursorC2EP13QTextDocument()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextCursorC2EP13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextCursor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:73
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QTextCursor(QTextFrame *)

/*
Constructs a null cursor.
*/
// QTextCursor(QTextFrame *) ctx.fn_proto_cpp
impl /*struct*/ QTextCursor {
  pub fn QTextCursor_2<T: QTextCursor_QTextCursor_2>(value: T) -> QTextCursor {
    let rsthis = value.QTextCursor_2();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCursor_QTextCursor_2 {
  fn QTextCursor_2(self) -> QTextCursor;
}
// QTextCursor(QTextFrame *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextCursor_QTextCursor_2 for (usize) {
  fn QTextCursor_2(self) -> QTextCursor {
    // unsafe{_ZN11QTextCursorC2EP10QTextFrame()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextCursorC2EP10QTextFrame", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextCursor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:74
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QTextCursor(const QTextBlock &)

/*
Constructs a null cursor.
*/
// QTextCursor(const QTextBlock &) ctx.fn_proto_cpp
impl /*struct*/ QTextCursor {
  pub fn QTextCursor_3<T: QTextCursor_QTextCursor_3>(value: T) -> QTextCursor {
    let rsthis = value.QTextCursor_3();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCursor_QTextCursor_3 {
  fn QTextCursor_3(self) -> QTextCursor;
}
// QTextCursor(const QTextBlock &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextCursor_QTextCursor_3 for (usize) {
  fn QTextCursor_3(self) -> QTextCursor {
    // unsafe{_ZN11QTextCursorC2ERK10QTextBlock()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextCursorC2ERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextCursor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:77
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QTextCursor & operator=(QTextCursor &&)

/*

*/
impl /*struct*/ QTextCursor {
  pub fn operator_equal_0<RetType, T: QTextCursor_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QTextCursor_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextCursoraSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:79
// index:1
// Public Visibility=Default Availability=Available
// [8] QTextCursor & operator=(const QTextCursor &)

/*

*/
impl /*struct*/ QTextCursor {
  pub fn operator_equal_1<RetType, T: QTextCursor_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QTextCursor_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextCursoraSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTextCursor()

/*

*/
pub fn DeleteQTextCursor(this :*mut QTextCursor) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QTextCursorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtextcursor.h:82
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QTextCursor &)

/*
Swaps this text cursor instance with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QTextCursor {
  pub fn swap_0<RetType, T: QTextCursor_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QTextCursor_swap_0<RetType> {
  fn swap_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:84
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the cursor is null; otherwise returns false. A null cursor is created by the default constructor.
*/
impl /*struct*/ QTextCursor {
  pub fn isNull_0<RetType, T: QTextCursor_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QTextCursor_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPosition(int, QTextCursor::MoveMode)

/*
Moves the cursor to the absolute position in the document specified by pos using a MoveMode specified by m. The cursor is positioned between characters.

See also position(), movePosition(), and anchor().
*/
impl /*struct*/ QTextCursor {
  pub fn setPosition_0<RetType, T: QTextCursor_setPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPosition_0(self);
    // return 1;
  }
}
pub trait QTextCursor_setPosition_0<RetType> {
  fn setPosition_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_setPosition_0<(/*void*/)> for (i32,i32) {
  fn setPosition_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor11setPositionEiNS_8MoveModeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:92
// index:0
// Public Visibility=Default Availability=Available
// [4] int position() const

/*
Returns the absolute position of the cursor within the document. The cursor is positioned between characters.

See also setPosition(), movePosition(), anchor(), and positionInBlock().
*/
impl /*struct*/ QTextCursor {
  pub fn position_0<RetType, T: QTextCursor_position_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.position_0(self);
    // return 1;
  }
}
pub trait QTextCursor_position_0<RetType> {
  fn position_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_position_0<i32> for () {
  fn position_0(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor8positionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:93
// index:0
// Public Visibility=Default Availability=Available
// [4] int positionInBlock() const

/*
Returns the relative position of the cursor within the block. The cursor is positioned between characters.

This is equivalent to position() - block().position().

This function was introduced in  Qt 4.7.

See also position().
*/
impl /*struct*/ QTextCursor {
  pub fn positionInBlock_0<RetType, T: QTextCursor_positionInBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.positionInBlock_0(self);
    // return 1;
  }
}
pub trait QTextCursor_positionInBlock_0<RetType> {
  fn positionInBlock_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_positionInBlock_0<i32> for () {
  fn positionInBlock_0(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor15positionInBlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:95
// index:0
// Public Visibility=Default Availability=Available
// [4] int anchor() const

/*
Returns the anchor position; this is the same as position() unless there is a selection in which case position() marks one end of the selection and anchor() marks the other end. Just like the cursor position, the anchor position is between characters.

See also position(), setPosition(), movePosition(), selectionStart(), and selectionEnd().
*/
impl /*struct*/ QTextCursor {
  pub fn anchor_0<RetType, T: QTextCursor_anchor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.anchor_0(self);
    // return 1;
  }
}
pub trait QTextCursor_anchor_0<RetType> {
  fn anchor_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_anchor_0<i32> for () {
  fn anchor_0(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor6anchorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertText(const QString &)

/*
Inserts text at the current position, using the current character format.

If there is a selection, the selection is deleted and replaced by text, for example:


  cursor.clearSelection();
  cursor.movePosition(QTextCursor::NextWord, QTextCursor::KeepAnchor);
  cursor.insertText("Hello World");



This clears any existing selection, selects the word at the cursor (i.e. from position() forward), and replaces the selection with the phrase "Hello World".

Any ASCII linefeed characters (\n) in the inserted text are transformed into unicode block separators, corresponding to insertBlock() calls.

See also charFormat() and hasSelection().
*/
impl /*struct*/ QTextCursor {
  pub fn insertText_0<RetType, T: QTextCursor_insertText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertText_0(self);
    // return 1;
  }
}
pub trait QTextCursor_insertText_0<RetType> {
  fn insertText_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertText_0<(/*void*/)> for (usize) {
  fn insertText_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor10insertTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:98
// index:1
// Public Visibility=Default Availability=Available
// [-2] void insertText(const QString &, const QTextCharFormat &)

/*
Inserts text at the current position, using the current character format.

If there is a selection, the selection is deleted and replaced by text, for example:


  cursor.clearSelection();
  cursor.movePosition(QTextCursor::NextWord, QTextCursor::KeepAnchor);
  cursor.insertText("Hello World");



This clears any existing selection, selects the word at the cursor (i.e. from position() forward), and replaces the selection with the phrase "Hello World".

Any ASCII linefeed characters (\n) in the inserted text are transformed into unicode block separators, corresponding to insertBlock() calls.

See also charFormat() and hasSelection().
*/
impl /*struct*/ QTextCursor {
  pub fn insertText_1<RetType, T: QTextCursor_insertText_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertText_1(self);
    // return 1;
  }
}
pub trait QTextCursor_insertText_1<RetType> {
  fn insertText_1(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertText_1<(/*void*/)> for (usize,usize) {
  fn insertText_1(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor10insertTextERK7QStringRK15QTextCharFormat", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:131
// index:0
// Public Visibility=Default Availability=Available
// [1] bool movePosition(QTextCursor::MoveOperation, QTextCursor::MoveMode, int)

/*
Moves the cursor by performing the given operation n times, using the specified mode, and returns true if all operations were completed successfully; otherwise returns false.

For example, if this function is repeatedly used to seek to the end of the next word, it will eventually fail when the end of the document is reached.

By default, the move operation is performed once (n = 1).

If mode is KeepAnchor, the cursor selects the text it moves over. This is the same effect that the user achieves when they hold down the Shift key and move the cursor with the cursor keys.

See also setVisualNavigation().
*/
impl /*struct*/ QTextCursor {
  pub fn movePosition_0<RetType, T: QTextCursor_movePosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.movePosition_0(self);
    // return 1;
  }
}
pub trait QTextCursor_movePosition_0<RetType> {
  fn movePosition_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_movePosition_0<bool> for (i32,i32,i32) {
  fn movePosition_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextCursor12movePositionENS_13MoveOperationENS_8MoveModeEi", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:133
// index:0
// Public Visibility=Default Availability=Available
// [1] bool visualNavigation() const

/*
Returns true if the cursor does visual navigation; otherwise returns false.

Visual navigation means skipping over hidden text paragraphs. The default is false.

This function was introduced in  Qt 4.4.

See also setVisualNavigation() and movePosition().
*/
impl /*struct*/ QTextCursor {
  pub fn visualNavigation_0<RetType, T: QTextCursor_visualNavigation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visualNavigation_0(self);
    // return 1;
  }
}
pub trait QTextCursor_visualNavigation_0<RetType> {
  fn visualNavigation_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_visualNavigation_0<bool> for () {
  fn visualNavigation_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor16visualNavigationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVisualNavigation(bool)

/*
Sets visual navigation to b.

Visual navigation means skipping over hidden text paragraphs. The default is false.

This function was introduced in  Qt 4.4.

See also visualNavigation() and movePosition().
*/
impl /*struct*/ QTextCursor {
  pub fn setVisualNavigation_0<RetType, T: QTextCursor_setVisualNavigation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisualNavigation_0(self);
    // return 1;
  }
}
pub trait QTextCursor_setVisualNavigation_0<RetType> {
  fn setVisualNavigation_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_setVisualNavigation_0<(/*void*/)> for (bool) {
  fn setVisualNavigation_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor19setVisualNavigationEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:136
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVerticalMovementX(int)

/*
Sets the visual x position for vertical cursor movements to x.

The vertical movement x position is cleared automatically when the cursor moves horizontally, and kept unchanged when the cursor moves vertically. The mechanism allows the cursor to move up and down on a visually straight line with proportional fonts, and to gently "jump" over short lines.

A value of -1 indicates no predefined x position. It will then be set automatically the next time the cursor moves up or down.

This function was introduced in  Qt 4.7.

See also verticalMovementX().
*/
impl /*struct*/ QTextCursor {
  pub fn setVerticalMovementX_0<RetType, T: QTextCursor_setVerticalMovementX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalMovementX_0(self);
    // return 1;
  }
}
pub trait QTextCursor_setVerticalMovementX_0<RetType> {
  fn setVerticalMovementX_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_setVerticalMovementX_0<(/*void*/)> for (i32) {
  fn setVerticalMovementX_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor20setVerticalMovementXEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:137
// index:0
// Public Visibility=Default Availability=Available
// [4] int verticalMovementX() const

/*
Returns the visual x position for vertical cursor movements.

A value of -1 indicates no predefined x position. It will then be set automatically the next time the cursor moves up or down.

This function was introduced in  Qt 4.7.

See also setVerticalMovementX().
*/
impl /*struct*/ QTextCursor {
  pub fn verticalMovementX_0<RetType, T: QTextCursor_verticalMovementX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalMovementX_0(self);
    // return 1;
  }
}
pub trait QTextCursor_verticalMovementX_0<RetType> {
  fn verticalMovementX_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_verticalMovementX_0<i32> for () {
  fn verticalMovementX_0(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor17verticalMovementXEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setKeepPositionOnInsert(bool)

/*
Defines whether the cursor should keep its current position when text gets inserted at the current position of the cursor.

If b is true, the cursor keeps its current position when text gets inserted at the positing of the cursor. If b is false, the cursor moves along with the inserted text.

The default is false.

Note that a cursor always moves when text is inserted before the current position of the cursor, and it always keeps its position when text is inserted after the current position of the cursor.

This function was introduced in  Qt 4.7.

See also keepPositionOnInsert().
*/
impl /*struct*/ QTextCursor {
  pub fn setKeepPositionOnInsert_0<RetType, T: QTextCursor_setKeepPositionOnInsert_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKeepPositionOnInsert_0(self);
    // return 1;
  }
}
pub trait QTextCursor_setKeepPositionOnInsert_0<RetType> {
  fn setKeepPositionOnInsert_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_setKeepPositionOnInsert_0<(/*void*/)> for (bool) {
  fn setKeepPositionOnInsert_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor23setKeepPositionOnInsertEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:140
// index:0
// Public Visibility=Default Availability=Available
// [1] bool keepPositionOnInsert() const

/*
Returns whether the cursor should keep its current position when text gets inserted at the position of the cursor.

The default is false;

This function was introduced in  Qt 4.7.

See also setKeepPositionOnInsert().
*/
impl /*struct*/ QTextCursor {
  pub fn keepPositionOnInsert_0<RetType, T: QTextCursor_keepPositionOnInsert_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keepPositionOnInsert_0(self);
    // return 1;
  }
}
pub trait QTextCursor_keepPositionOnInsert_0<RetType> {
  fn keepPositionOnInsert_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_keepPositionOnInsert_0<bool> for () {
  fn keepPositionOnInsert_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor20keepPositionOnInsertEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:142
// index:0
// Public Visibility=Default Availability=Available
// [-2] void deleteChar()

/*
If there is no selected text, deletes the character at the current cursor position; otherwise deletes the selected text.

See also deletePreviousChar(), hasSelection(), and clearSelection().
*/
impl /*struct*/ QTextCursor {
  pub fn deleteChar_0<RetType, T: QTextCursor_deleteChar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.deleteChar_0(self);
    // return 1;
  }
}
pub trait QTextCursor_deleteChar_0<RetType> {
  fn deleteChar_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_deleteChar_0<(/*void*/)> for () {
  fn deleteChar_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextCursor10deleteCharEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void deletePreviousChar()

/*
If there is no selected text, deletes the character before the current cursor position; otherwise deletes the selected text.

See also deleteChar(), hasSelection(), and clearSelection().
*/
impl /*struct*/ QTextCursor {
  pub fn deletePreviousChar_0<RetType, T: QTextCursor_deletePreviousChar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.deletePreviousChar_0(self);
    // return 1;
  }
}
pub trait QTextCursor_deletePreviousChar_0<RetType> {
  fn deletePreviousChar_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_deletePreviousChar_0<(/*void*/)> for () {
  fn deletePreviousChar_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextCursor18deletePreviousCharEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:151
// index:0
// Public Visibility=Default Availability=Available
// [-2] void select(QTextCursor::SelectionType)

/*
Selects text in the document according to the given selection.
*/
impl /*struct*/ QTextCursor {
  pub fn select__0<RetType, T: QTextCursor_select__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.select__0(self);
    // return 1;
  }
}
pub trait QTextCursor_select__0<RetType> {
  fn select__0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_select__0<(/*void*/)> for (i32) {
  fn select__0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor6selectENS_13SelectionTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:153
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasSelection() const

/*
Returns true if the cursor contains a selection; otherwise returns false.
*/
impl /*struct*/ QTextCursor {
  pub fn hasSelection_0<RetType, T: QTextCursor_hasSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasSelection_0(self);
    // return 1;
  }
}
pub trait QTextCursor_hasSelection_0<RetType> {
  fn hasSelection_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_hasSelection_0<bool> for () {
  fn hasSelection_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor12hasSelectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:154
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasComplexSelection() const

/*
Returns true if the cursor contains a selection that is not simply a range from selectionStart() to selectionEnd(); otherwise returns false.

Complex selections are ones that span at least two cells in a table; their extent is specified by selectedTableCells().
*/
impl /*struct*/ QTextCursor {
  pub fn hasComplexSelection_0<RetType, T: QTextCursor_hasComplexSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasComplexSelection_0(self);
    // return 1;
  }
}
pub trait QTextCursor_hasComplexSelection_0<RetType> {
  fn hasComplexSelection_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_hasComplexSelection_0<bool> for () {
  fn hasComplexSelection_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor19hasComplexSelectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeSelectedText()

/*
If there is a selection, its content is deleted; otherwise does nothing.

See also hasSelection().
*/
impl /*struct*/ QTextCursor {
  pub fn removeSelectedText_0<RetType, T: QTextCursor_removeSelectedText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeSelectedText_0(self);
    // return 1;
  }
}
pub trait QTextCursor_removeSelectedText_0<RetType> {
  fn removeSelectedText_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_removeSelectedText_0<(/*void*/)> for () {
  fn removeSelectedText_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextCursor18removeSelectedTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearSelection()

/*
Clears the current selection by setting the anchor to the cursor position.

Note that it does not delete the text of the selection.

See also removeSelectedText() and hasSelection().
*/
impl /*struct*/ QTextCursor {
  pub fn clearSelection_0<RetType, T: QTextCursor_clearSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearSelection_0(self);
    // return 1;
  }
}
pub trait QTextCursor_clearSelection_0<RetType> {
  fn clearSelection_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_clearSelection_0<(/*void*/)> for () {
  fn clearSelection_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextCursor14clearSelectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:157
// index:0
// Public Visibility=Default Availability=Available
// [4] int selectionStart() const

/*
Returns the start of the selection or position() if the cursor doesn't have a selection.

See also selectionEnd(), position(), and anchor().
*/
impl /*struct*/ QTextCursor {
  pub fn selectionStart_0<RetType, T: QTextCursor_selectionStart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionStart_0(self);
    // return 1;
  }
}
pub trait QTextCursor_selectionStart_0<RetType> {
  fn selectionStart_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_selectionStart_0<i32> for () {
  fn selectionStart_0(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor14selectionStartEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:158
// index:0
// Public Visibility=Default Availability=Available
// [4] int selectionEnd() const

/*
Returns the end of the selection or position() if the cursor doesn't have a selection.

See also selectionStart(), position(), and anchor().
*/
impl /*struct*/ QTextCursor {
  pub fn selectionEnd_0<RetType, T: QTextCursor_selectionEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionEnd_0(self);
    // return 1;
  }
}
pub trait QTextCursor_selectionEnd_0<RetType> {
  fn selectionEnd_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_selectionEnd_0<i32> for () {
  fn selectionEnd_0(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor12selectionEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:160
// index:0
// Public Visibility=Default Availability=Available
// [8] QString selectedText() const

/*
Returns the current selection's text (which may be empty). This only returns the text, with no rich text formatting information. If you want a document fragment (i.e. formatted rich text) use selection() instead.

Note: If the selection obtained from an editor spans a line break, the text will contain a Unicode U+2029 paragraph separator character instead of a newline \n character. Use QString::replace() to replace these characters with newlines.
*/
impl /*struct*/ QTextCursor {
  pub fn selectedText_0<RetType, T: QTextCursor_selectedText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedText_0(self);
    // return 1;
  }
}
pub trait QTextCursor_selectedText_0<RetType> {
  fn selectedText_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_selectedText_0<usize> for () {
  fn selectedText_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor12selectedTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:161
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextDocumentFragment selection() const

/*
Returns the current selection (which may be empty) with all its formatting information. If you just want the selected text (i.e. plain text) use selectedText() instead.

Note: Unlike QTextDocumentFragment::toPlainText(), selectedText() may include special unicode characters such as QChar::ParagraphSeparator.

See also QTextDocumentFragment::toPlainText().
*/
impl /*struct*/ QTextCursor {
  pub fn selection_0<RetType, T: QTextCursor_selection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selection_0(self);
    // return 1;
  }
}
pub trait QTextCursor_selection_0<RetType> {
  fn selection_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_selection_0<usize> for () {
  fn selection_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor9selectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:162
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectedTableCells(int *, int *, int *, int *) const

/*
If the selection spans over table cells, firstRow is populated with the number of the first row in the selection, firstColumn with the number of the first column in the selection, and numRows and numColumns with the number of rows and columns in the selection. If the selection does not span any table cells the results are harmless but undefined.
*/
impl /*struct*/ QTextCursor {
  pub fn selectedTableCells_0<RetType, T: QTextCursor_selectedTableCells_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedTableCells_0(self);
    // return 1;
  }
}
pub trait QTextCursor_selectedTableCells_0<RetType> {
  fn selectedTableCells_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_selectedTableCells_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn selectedTableCells_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QTextCursor18selectedTableCellsEPiS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:164
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlock block() const

/*
Returns the block that contains the cursor.
*/
impl /*struct*/ QTextCursor {
  pub fn block_0<RetType, T: QTextCursor_block_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.block_0(self);
    // return 1;
  }
}
pub trait QTextCursor_block_0<RetType> {
  fn block_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_block_0<usize> for () {
  fn block_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor5blockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:166
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextCharFormat charFormat() const

/*
Returns the format of the character immediately before the cursor position(). If the cursor is positioned at the beginning of a text block that is not empty then the format of the character immediately after the cursor is returned.

See also setCharFormat(), insertText(), and blockFormat().
*/
impl /*struct*/ QTextCursor {
  pub fn charFormat_0<RetType, T: QTextCursor_charFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.charFormat_0(self);
    // return 1;
  }
}
pub trait QTextCursor_charFormat_0<RetType> {
  fn charFormat_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_charFormat_0<usize> for () {
  fn charFormat_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor10charFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:167
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCharFormat(const QTextCharFormat &)

/*
Sets the cursor's current character format to the given format. If the cursor has a selection, the given format is applied to the current selection.

See also charFormat(), hasSelection(), and mergeCharFormat().
*/
impl /*struct*/ QTextCursor {
  pub fn setCharFormat_0<RetType, T: QTextCursor_setCharFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCharFormat_0(self);
    // return 1;
  }
}
pub trait QTextCursor_setCharFormat_0<RetType> {
  fn setCharFormat_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_setCharFormat_0<(/*void*/)> for (usize) {
  fn setCharFormat_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor13setCharFormatERK15QTextCharFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:168
// index:0
// Public Visibility=Default Availability=Available
// [-2] void mergeCharFormat(const QTextCharFormat &)

/*
Merges the cursor's current character format with the properties described by format modifier. If the cursor has a selection, this function applies all the properties set in modifier to all the character formats that are part of the selection.

See also hasSelection() and setCharFormat().
*/
impl /*struct*/ QTextCursor {
  pub fn mergeCharFormat_0<RetType, T: QTextCursor_mergeCharFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mergeCharFormat_0(self);
    // return 1;
  }
}
pub trait QTextCursor_mergeCharFormat_0<RetType> {
  fn mergeCharFormat_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_mergeCharFormat_0<(/*void*/)> for (usize) {
  fn mergeCharFormat_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor15mergeCharFormatERK15QTextCharFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:170
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlockFormat blockFormat() const

/*
Returns the block format of the block the cursor is in.

See also setBlockFormat() and charFormat().
*/
impl /*struct*/ QTextCursor {
  pub fn blockFormat_0<RetType, T: QTextCursor_blockFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockFormat_0(self);
    // return 1;
  }
}
pub trait QTextCursor_blockFormat_0<RetType> {
  fn blockFormat_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_blockFormat_0<usize> for () {
  fn blockFormat_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor11blockFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:171
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBlockFormat(const QTextBlockFormat &)

/*
Sets the block format of the current block (or all blocks that are contained in the selection) to format.

See also blockFormat() and mergeBlockFormat().
*/
impl /*struct*/ QTextCursor {
  pub fn setBlockFormat_0<RetType, T: QTextCursor_setBlockFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBlockFormat_0(self);
    // return 1;
  }
}
pub trait QTextCursor_setBlockFormat_0<RetType> {
  fn setBlockFormat_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_setBlockFormat_0<(/*void*/)> for (usize) {
  fn setBlockFormat_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor14setBlockFormatERK16QTextBlockFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:172
// index:0
// Public Visibility=Default Availability=Available
// [-2] void mergeBlockFormat(const QTextBlockFormat &)

/*
Modifies the block format of the current block (or all blocks that are contained in the selection) with the block format specified by modifier.

See also setBlockFormat() and blockFormat().
*/
impl /*struct*/ QTextCursor {
  pub fn mergeBlockFormat_0<RetType, T: QTextCursor_mergeBlockFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mergeBlockFormat_0(self);
    // return 1;
  }
}
pub trait QTextCursor_mergeBlockFormat_0<RetType> {
  fn mergeBlockFormat_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_mergeBlockFormat_0<(/*void*/)> for (usize) {
  fn mergeBlockFormat_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor16mergeBlockFormatERK16QTextBlockFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:174
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextCharFormat blockCharFormat() const

/*
Returns the block character format of the block the cursor is in.

The block char format is the format used when inserting text at the beginning of an empty block.

See also setBlockCharFormat().
*/
impl /*struct*/ QTextCursor {
  pub fn blockCharFormat_0<RetType, T: QTextCursor_blockCharFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockCharFormat_0(self);
    // return 1;
  }
}
pub trait QTextCursor_blockCharFormat_0<RetType> {
  fn blockCharFormat_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_blockCharFormat_0<usize> for () {
  fn blockCharFormat_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor15blockCharFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:175
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBlockCharFormat(const QTextCharFormat &)

/*
Sets the block char format of the current block (or all blocks that are contained in the selection) to format.

See also blockCharFormat().
*/
impl /*struct*/ QTextCursor {
  pub fn setBlockCharFormat_0<RetType, T: QTextCursor_setBlockCharFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBlockCharFormat_0(self);
    // return 1;
  }
}
pub trait QTextCursor_setBlockCharFormat_0<RetType> {
  fn setBlockCharFormat_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_setBlockCharFormat_0<(/*void*/)> for (usize) {
  fn setBlockCharFormat_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor18setBlockCharFormatERK15QTextCharFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:176
// index:0
// Public Visibility=Default Availability=Available
// [-2] void mergeBlockCharFormat(const QTextCharFormat &)

/*
Modifies the block char format of the current block (or all blocks that are contained in the selection) with the block format specified by modifier.

See also setBlockCharFormat().
*/
impl /*struct*/ QTextCursor {
  pub fn mergeBlockCharFormat_0<RetType, T: QTextCursor_mergeBlockCharFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mergeBlockCharFormat_0(self);
    // return 1;
  }
}
pub trait QTextCursor_mergeBlockCharFormat_0<RetType> {
  fn mergeBlockCharFormat_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_mergeBlockCharFormat_0<(/*void*/)> for (usize) {
  fn mergeBlockCharFormat_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor20mergeBlockCharFormatERK15QTextCharFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:178
// index:0
// Public Visibility=Default Availability=Available
// [1] bool atBlockStart() const

/*
Returns true if the cursor is at the start of a block; otherwise returns false.

See also atBlockEnd() and atStart().
*/
impl /*struct*/ QTextCursor {
  pub fn atBlockStart_0<RetType, T: QTextCursor_atBlockStart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.atBlockStart_0(self);
    // return 1;
  }
}
pub trait QTextCursor_atBlockStart_0<RetType> {
  fn atBlockStart_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_atBlockStart_0<bool> for () {
  fn atBlockStart_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor12atBlockStartEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:179
// index:0
// Public Visibility=Default Availability=Available
// [1] bool atBlockEnd() const

/*
Returns true if the cursor is at the end of a block; otherwise returns false.

See also atBlockStart() and atEnd().
*/
impl /*struct*/ QTextCursor {
  pub fn atBlockEnd_0<RetType, T: QTextCursor_atBlockEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.atBlockEnd_0(self);
    // return 1;
  }
}
pub trait QTextCursor_atBlockEnd_0<RetType> {
  fn atBlockEnd_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_atBlockEnd_0<bool> for () {
  fn atBlockEnd_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor10atBlockEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:180
// index:0
// Public Visibility=Default Availability=Available
// [1] bool atStart() const

/*
Returns true if the cursor is at the start of the document; otherwise returns false.

See also atBlockStart() and atEnd().
*/
impl /*struct*/ QTextCursor {
  pub fn atStart_0<RetType, T: QTextCursor_atStart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.atStart_0(self);
    // return 1;
  }
}
pub trait QTextCursor_atStart_0<RetType> {
  fn atStart_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_atStart_0<bool> for () {
  fn atStart_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor7atStartEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:181
// index:0
// Public Visibility=Default Availability=Available
// [1] bool atEnd() const

/*
Returns true if the cursor is at the end of the document; otherwise returns false.

This function was introduced in  Qt 4.6.

See also atStart() and atBlockEnd().
*/
impl /*struct*/ QTextCursor {
  pub fn atEnd_0<RetType, T: QTextCursor_atEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.atEnd_0(self);
    // return 1;
  }
}
pub trait QTextCursor_atEnd_0<RetType> {
  fn atEnd_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_atEnd_0<bool> for () {
  fn atEnd_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor5atEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:183
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertBlock()

/*
Inserts a new empty block at the cursor position() with the current blockFormat() and charFormat().

See also setBlockFormat().
*/
impl /*struct*/ QTextCursor {
  pub fn insertBlock_0<RetType, T: QTextCursor_insertBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertBlock_0(self);
    // return 1;
  }
}
pub trait QTextCursor_insertBlock_0<RetType> {
  fn insertBlock_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertBlock_0<(/*void*/)> for () {
  fn insertBlock_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextCursor11insertBlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:184
// index:1
// Public Visibility=Default Availability=Available
// [-2] void insertBlock(const QTextBlockFormat &)

/*
Inserts a new empty block at the cursor position() with the current blockFormat() and charFormat().

See also setBlockFormat().
*/
impl /*struct*/ QTextCursor {
  pub fn insertBlock_1<RetType, T: QTextCursor_insertBlock_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertBlock_1(self);
    // return 1;
  }
}
pub trait QTextCursor_insertBlock_1<RetType> {
  fn insertBlock_1(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertBlock_1<(/*void*/)> for (usize) {
  fn insertBlock_1(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor11insertBlockERK16QTextBlockFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:185
// index:2
// Public Visibility=Default Availability=Available
// [-2] void insertBlock(const QTextBlockFormat &, const QTextCharFormat &)

/*
Inserts a new empty block at the cursor position() with the current blockFormat() and charFormat().

See also setBlockFormat().
*/
impl /*struct*/ QTextCursor {
  pub fn insertBlock_2<RetType, T: QTextCursor_insertBlock_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertBlock_2(self);
    // return 1;
  }
}
pub trait QTextCursor_insertBlock_2<RetType> {
  fn insertBlock_2(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertBlock_2<(/*void*/)> for (usize,usize) {
  fn insertBlock_2(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor11insertBlockERK16QTextBlockFormatRK15QTextCharFormat", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:187
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextList * insertList(const QTextListFormat &)

/*
Inserts a new block at the current position and makes it the first list item of a newly created list with the given format. Returns the created list.

See also currentList(), createList(), and insertBlock().
*/
impl /*struct*/ QTextCursor {
  pub fn insertList_0<RetType, T: QTextCursor_insertList_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertList_0(self);
    // return 1;
  }
}
pub trait QTextCursor_insertList_0<RetType> {
  fn insertList_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertList_0<usize> for (usize) {
  fn insertList_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextCursor10insertListERK15QTextListFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:188
// index:1
// Public Visibility=Default Availability=Available
// [8] QTextList * insertList(QTextListFormat::Style)

/*
Inserts a new block at the current position and makes it the first list item of a newly created list with the given format. Returns the created list.

See also currentList(), createList(), and insertBlock().
*/
impl /*struct*/ QTextCursor {
  pub fn insertList_1<RetType, T: QTextCursor_insertList_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertList_1(self);
    // return 1;
  }
}
pub trait QTextCursor_insertList_1<RetType> {
  fn insertList_1(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertList_1<usize> for (i32) {
  fn insertList_1(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextCursor10insertListEN15QTextListFormat5StyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:190
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextList * createList(const QTextListFormat &)

/*
Creates and returns a new list with the given format, and makes the current paragraph the cursor is in the first list item.

See also insertList() and currentList().
*/
impl /*struct*/ QTextCursor {
  pub fn createList_0<RetType, T: QTextCursor_createList_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createList_0(self);
    // return 1;
  }
}
pub trait QTextCursor_createList_0<RetType> {
  fn createList_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_createList_0<usize> for (usize) {
  fn createList_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextCursor10createListERK15QTextListFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:191
// index:1
// Public Visibility=Default Availability=Available
// [8] QTextList * createList(QTextListFormat::Style)

/*
Creates and returns a new list with the given format, and makes the current paragraph the cursor is in the first list item.

See also insertList() and currentList().
*/
impl /*struct*/ QTextCursor {
  pub fn createList_1<RetType, T: QTextCursor_createList_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createList_1(self);
    // return 1;
  }
}
pub trait QTextCursor_createList_1<RetType> {
  fn createList_1(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_createList_1<usize> for (i32) {
  fn createList_1(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextCursor10createListEN15QTextListFormat5StyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:192
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextList * currentList() const

/*
Returns the current list if the cursor position() is inside a block that is part of a list; otherwise returns 0.

See also insertList() and createList().
*/
impl /*struct*/ QTextCursor {
  pub fn currentList_0<RetType, T: QTextCursor_currentList_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentList_0(self);
    // return 1;
  }
}
pub trait QTextCursor_currentList_0<RetType> {
  fn currentList_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_currentList_0<usize> for () {
  fn currentList_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor11currentListEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:194
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextTable * insertTable(int, int, const QTextTableFormat &)

/*
Creates a new table with the given number of rows and columns in the specified format, inserts it at the current cursor position() in the document, and returns the table object. The cursor is moved to the beginning of the first cell.

There must be at least one row and one column in the table.

See also currentTable().
*/
impl /*struct*/ QTextCursor {
  pub fn insertTable_0<RetType, T: QTextCursor_insertTable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertTable_0(self);
    // return 1;
  }
}
pub trait QTextCursor_insertTable_0<RetType> {
  fn insertTable_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertTable_0<usize> for (i32,i32,usize) {
  fn insertTable_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextCursor11insertTableEiiRK16QTextTableFormat", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:195
// index:1
// Public Visibility=Default Availability=Available
// [8] QTextTable * insertTable(int, int)

/*
Creates a new table with the given number of rows and columns in the specified format, inserts it at the current cursor position() in the document, and returns the table object. The cursor is moved to the beginning of the first cell.

There must be at least one row and one column in the table.

See also currentTable().
*/
impl /*struct*/ QTextCursor {
  pub fn insertTable_1<RetType, T: QTextCursor_insertTable_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertTable_1(self);
    // return 1;
  }
}
pub trait QTextCursor_insertTable_1<RetType> {
  fn insertTable_1(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertTable_1<usize> for (i32,i32) {
  fn insertTable_1(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextCursor11insertTableEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:196
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextTable * currentTable() const

/*
Returns a pointer to the current table if the cursor position() is inside a block that is part of a table; otherwise returns 0.

See also insertTable().
*/
impl /*struct*/ QTextCursor {
  pub fn currentTable_0<RetType, T: QTextCursor_currentTable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentTable_0(self);
    // return 1;
  }
}
pub trait QTextCursor_currentTable_0<RetType> {
  fn currentTable_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_currentTable_0<usize> for () {
  fn currentTable_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor12currentTableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:198
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextFrame * insertFrame(const QTextFrameFormat &)

/*
Inserts a frame with the given format at the current cursor position(), moves the cursor position() inside the frame, and returns the frame.

If the cursor holds a selection, the whole selection is moved inside the frame.

See also hasSelection().
*/
impl /*struct*/ QTextCursor {
  pub fn insertFrame_0<RetType, T: QTextCursor_insertFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertFrame_0(self);
    // return 1;
  }
}
pub trait QTextCursor_insertFrame_0<RetType> {
  fn insertFrame_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertFrame_0<usize> for (usize) {
  fn insertFrame_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextCursor11insertFrameERK16QTextFrameFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:199
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextFrame * currentFrame() const

/*
Returns a pointer to the current frame. Returns 0 if the cursor is invalid.

See also insertFrame().
*/
impl /*struct*/ QTextCursor {
  pub fn currentFrame_0<RetType, T: QTextCursor_currentFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentFrame_0(self);
    // return 1;
  }
}
pub trait QTextCursor_currentFrame_0<RetType> {
  fn currentFrame_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_currentFrame_0<usize> for () {
  fn currentFrame_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor12currentFrameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:201
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertFragment(const QTextDocumentFragment &)

/*
Inserts the text fragment at the current position().
*/
impl /*struct*/ QTextCursor {
  pub fn insertFragment_0<RetType, T: QTextCursor_insertFragment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertFragment_0(self);
    // return 1;
  }
}
pub trait QTextCursor_insertFragment_0<RetType> {
  fn insertFragment_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertFragment_0<(/*void*/)> for (usize) {
  fn insertFragment_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor14insertFragmentERK21QTextDocumentFragment", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:204
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertHtml(const QString &)

/*
Inserts the text html at the current position(). The text is interpreted as HTML.

Note: When using this function with a style sheet, the style sheet will only apply to the current block in the document. In order to apply a style sheet throughout a document, use QTextDocument::setDefaultStyleSheet() instead.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QTextCursor {
  pub fn insertHtml_0<RetType, T: QTextCursor_insertHtml_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertHtml_0(self);
    // return 1;
  }
}
pub trait QTextCursor_insertHtml_0<RetType> {
  fn insertHtml_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertHtml_0<(/*void*/)> for (usize) {
  fn insertHtml_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor10insertHtmlERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:207
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertImage(const QTextImageFormat &, QTextFrameFormat::Position)

/*
Inserts the image defined by format at the current position().
*/
impl /*struct*/ QTextCursor {
  pub fn insertImage_0<RetType, T: QTextCursor_insertImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertImage_0(self);
    // return 1;
  }
}
pub trait QTextCursor_insertImage_0<RetType> {
  fn insertImage_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertImage_0<(/*void*/)> for (usize,i32) {
  fn insertImage_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor11insertImageERK16QTextImageFormatN16QTextFrameFormat8PositionE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:208
// index:1
// Public Visibility=Default Availability=Available
// [-2] void insertImage(const QTextImageFormat &)

/*
Inserts the image defined by format at the current position().
*/
impl /*struct*/ QTextCursor {
  pub fn insertImage_1<RetType, T: QTextCursor_insertImage_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertImage_1(self);
    // return 1;
  }
}
pub trait QTextCursor_insertImage_1<RetType> {
  fn insertImage_1(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertImage_1<(/*void*/)> for (usize) {
  fn insertImage_1(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor11insertImageERK16QTextImageFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:209
// index:2
// Public Visibility=Default Availability=Available
// [-2] void insertImage(const QString &)

/*
Inserts the image defined by format at the current position().
*/
impl /*struct*/ QTextCursor {
  pub fn insertImage_2<RetType, T: QTextCursor_insertImage_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertImage_2(self);
    // return 1;
  }
}
pub trait QTextCursor_insertImage_2<RetType> {
  fn insertImage_2(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertImage_2<(/*void*/)> for (usize) {
  fn insertImage_2(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor11insertImageERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:210
// index:3
// Public Visibility=Default Availability=Available
// [-2] void insertImage(const QImage &, const QString &)

/*
Inserts the image defined by format at the current position().
*/
impl /*struct*/ QTextCursor {
  pub fn insertImage_3<RetType, T: QTextCursor_insertImage_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertImage_3(self);
    // return 1;
  }
}
pub trait QTextCursor_insertImage_3<RetType> {
  fn insertImage_3(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_insertImage_3<(/*void*/)> for (usize,usize) {
  fn insertImage_3(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextCursor11insertImageERK6QImageRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:212
// index:0
// Public Visibility=Default Availability=Available
// [-2] void beginEditBlock()

/*
Indicates the start of a block of editing operations on the document that should appear as a single operation from an undo/redo point of view.

For example:


  QTextCursor cursor(textDocument);
  cursor.beginEditBlock();
  cursor.insertText("Hello");
  cursor.insertText("World");
  cursor.endEditBlock();

  textDocument->undo();



The call to undo() will cause both insertions to be undone, causing both "World" and "Hello" to be removed.

It is possible to nest calls to beginEditBlock and endEditBlock. The top-most pair will determine the scope of the undo/redo operation.

See also endEditBlock().
*/
impl /*struct*/ QTextCursor {
  pub fn beginEditBlock_0<RetType, T: QTextCursor_beginEditBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginEditBlock_0(self);
    // return 1;
  }
}
pub trait QTextCursor_beginEditBlock_0<RetType> {
  fn beginEditBlock_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_beginEditBlock_0<(/*void*/)> for () {
  fn beginEditBlock_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextCursor14beginEditBlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:213
// index:0
// Public Visibility=Default Availability=Available
// [-2] void joinPreviousEditBlock()

/*
Like beginEditBlock() indicates the start of a block of editing operations that should appear as a single operation for undo/redo. However unlike beginEditBlock() it does not start a new block but reverses the previous call to endEditBlock() and therefore makes following operations part of the previous edit block created.

For example:


  QTextCursor cursor(textDocument);
  cursor.beginEditBlock();
  cursor.insertText("Hello");
  cursor.insertText("World");
  cursor.endEditBlock();

  ...

  cursor.joinPreviousEditBlock();
  cursor.insertText("Hey");
  cursor.endEditBlock();

  textDocument->undo();



The call to undo() will cause all three insertions to be undone.

See also beginEditBlock() and endEditBlock().
*/
impl /*struct*/ QTextCursor {
  pub fn joinPreviousEditBlock_0<RetType, T: QTextCursor_joinPreviousEditBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.joinPreviousEditBlock_0(self);
    // return 1;
  }
}
pub trait QTextCursor_joinPreviousEditBlock_0<RetType> {
  fn joinPreviousEditBlock_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_joinPreviousEditBlock_0<(/*void*/)> for () {
  fn joinPreviousEditBlock_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextCursor21joinPreviousEditBlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:214
// index:0
// Public Visibility=Default Availability=Available
// [-2] void endEditBlock()

/*
Indicates the end of a block of editing operations on the document that should appear as a single operation from an undo/redo point of view.

See also beginEditBlock().
*/
impl /*struct*/ QTextCursor {
  pub fn endEditBlock_0<RetType, T: QTextCursor_endEditBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endEditBlock_0(self);
    // return 1;
  }
}
pub trait QTextCursor_endEditBlock_0<RetType> {
  fn endEditBlock_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_endEditBlock_0<(/*void*/)> for () {
  fn endEditBlock_0(self , rsthis: & QTextCursor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextCursor12endEditBlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:216
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QTextCursor &) const

/*

*/
impl /*struct*/ QTextCursor {
  pub fn operator_not_equal_0<RetType, T: QTextCursor_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QTextCursor_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursorneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:217
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator<(const QTextCursor &) const

/*

*/
impl /*struct*/ QTextCursor {
  pub fn operator_less_than_0<RetType, T: QTextCursor_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QTextCursor_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursorltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:218
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator<=(const QTextCursor &) const

/*

*/
impl /*struct*/ QTextCursor {
  pub fn operator_less_than_equal_0<RetType, T: QTextCursor_operator_less_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_0(self);
    // return 1;
  }
}
pub trait QTextCursor_operator_less_than_equal_0<RetType> {
  fn operator_less_than_equal_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_operator_less_than_equal_0<bool> for (usize) {
  fn operator_less_than_equal_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursorleERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:219
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QTextCursor &) const

/*

*/
impl /*struct*/ QTextCursor {
  pub fn operator_equal_equal_0<RetType, T: QTextCursor_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QTextCursor_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursoreqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:220
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator>=(const QTextCursor &) const

/*

*/
impl /*struct*/ QTextCursor {
  pub fn operator_greater_than_equal_0<RetType, T: QTextCursor_operator_greater_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_0(self);
    // return 1;
  }
}
pub trait QTextCursor_operator_greater_than_equal_0<RetType> {
  fn operator_greater_than_equal_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_operator_greater_than_equal_0<bool> for (usize) {
  fn operator_greater_than_equal_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursorgeERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:221
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator>(const QTextCursor &) const

/*

*/
impl /*struct*/ QTextCursor {
  pub fn operator_greater_than_0<RetType, T: QTextCursor_operator_greater_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_0(self);
    // return 1;
  }
}
pub trait QTextCursor_operator_greater_than_0<RetType> {
  fn operator_greater_than_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_operator_greater_than_0<bool> for (usize) {
  fn operator_greater_than_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursorgtERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:223
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCopyOf(const QTextCursor &) const

/*
Returns true if this cursor and other are copies of each other, i.e. one of them was created as a copy of the other and neither has moved since. This is much stricter than equality.

See also operator=() and operator==().
*/
impl /*struct*/ QTextCursor {
  pub fn isCopyOf_0<RetType, T: QTextCursor_isCopyOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCopyOf_0(self);
    // return 1;
  }
}
pub trait QTextCursor_isCopyOf_0<RetType> {
  fn isCopyOf_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_isCopyOf_0<bool> for (usize) {
  fn isCopyOf_0(self , rsthis: & QTextCursor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor8isCopyOfERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:225
// index:0
// Public Visibility=Default Availability=Available
// [4] int blockNumber() const

/*
Returns the number of the block the cursor is in, or 0 if the cursor is invalid.

Note that this function only makes sense in documents without complex objects such as tables or frames.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QTextCursor {
  pub fn blockNumber_0<RetType, T: QTextCursor_blockNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockNumber_0(self);
    // return 1;
  }
}
pub trait QTextCursor_blockNumber_0<RetType> {
  fn blockNumber_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_blockNumber_0<i32> for () {
  fn blockNumber_0(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor11blockNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:226
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnNumber() const

/*
Returns the position of the cursor within its containing line.

Note that this is the column number relative to a wrapped line, not relative to the block (i.e. the paragraph).

You probably want to call positionInBlock() instead.

This function was introduced in  Qt 4.2.

See also positionInBlock().
*/
impl /*struct*/ QTextCursor {
  pub fn columnNumber_0<RetType, T: QTextCursor_columnNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnNumber_0(self);
    // return 1;
  }
}
pub trait QTextCursor_columnNumber_0<RetType> {
  fn columnNumber_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_columnNumber_0<i32> for () {
  fn columnNumber_0(self , rsthis: & QTextCursor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor12columnNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextcursor.h:228
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextDocument * document() const

/*
Returns the document this cursor is associated with.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QTextCursor {
  pub fn document_0<RetType, T: QTextCursor_document_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.document_0(self);
    // return 1;
  }
}
pub trait QTextCursor_document_0<RetType> {
  fn document_0(self , rsthis: & QTextCursor) -> RetType;
}
impl<'a> /*trait*/ QTextCursor_document_0<usize> for () {
  fn document_0(self , rsthis: & QTextCursor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextCursor8documentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*


If the anchor() is kept where it is and the position() is moved, the text in between will be selected.

*/
pub type QTextCursor__MoveMode = i32;
// Moves the anchor to the same position as the cursor itself.
pub const QTextCursor__MoveAnchor :QTextCursor__MoveMode = 0;
// Keeps the anchor where it is.
pub const QTextCursor__KeepAnchor :QTextCursor__MoveMode = 1;
pub fn QTextCursor_MoveModeItemName(val: i32) ->String {
  match val {
     QTextCursor__MoveAnchor => // 0
     {return String::from("MoveAnchor");}
     QTextCursor__KeepAnchor => // 1
     {return String::from("KeepAnchor");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextCursor_MoveModeItemName_s(val: i32) ->String {
  //var nilthis *QTextCursor
  //return nilthis.MoveModeItemName(val);
  return QTextCursor_MoveModeItemName(val);
}


/*


See also movePosition().

*/
pub type QTextCursor__MoveOperation = i32;
// Keep the cursor where it is
pub const QTextCursor__NoMove :QTextCursor__MoveOperation = 0;
// Move to the start of the document.
pub const QTextCursor__Start :QTextCursor__MoveOperation = 1;
// Move up one line.
pub const QTextCursor__Up :QTextCursor__MoveOperation = 2;
// Move to the start of the current line.
pub const QTextCursor__StartOfLine :QTextCursor__MoveOperation = 3;
// Move to the start of the current block.
pub const QTextCursor__StartOfBlock :QTextCursor__MoveOperation = 4;
// Move to the start of the current word.
pub const QTextCursor__StartOfWord :QTextCursor__MoveOperation = 5;
// Move to the start of the previous block.
pub const QTextCursor__PreviousBlock :QTextCursor__MoveOperation = 6;
// Move to the previous character.
pub const QTextCursor__PreviousCharacter :QTextCursor__MoveOperation = 7;
// Move to the beginning of the previous word.
pub const QTextCursor__PreviousWord :QTextCursor__MoveOperation = 8;
// Move left one character.
pub const QTextCursor__Left :QTextCursor__MoveOperation = 9;
// 
pub const QTextCursor__WordLeft :QTextCursor__MoveOperation = 10;
// 
pub const QTextCursor__End :QTextCursor__MoveOperation = 11;
// 
pub const QTextCursor__Down :QTextCursor__MoveOperation = 12;
// 
pub const QTextCursor__EndOfLine :QTextCursor__MoveOperation = 13;
// 
pub const QTextCursor__EndOfWord :QTextCursor__MoveOperation = 14;
// 
pub const QTextCursor__EndOfBlock :QTextCursor__MoveOperation = 15;
// 
pub const QTextCursor__NextBlock :QTextCursor__MoveOperation = 16;
// 
pub const QTextCursor__NextCharacter :QTextCursor__MoveOperation = 17;
// 
pub const QTextCursor__NextWord :QTextCursor__MoveOperation = 18;
// 
pub const QTextCursor__Right :QTextCursor__MoveOperation = 19;
// 
pub const QTextCursor__WordRight :QTextCursor__MoveOperation = 20;
// 
pub const QTextCursor__NextCell :QTextCursor__MoveOperation = 21;
// 
pub const QTextCursor__PreviousCell :QTextCursor__MoveOperation = 22;
// 
pub const QTextCursor__NextRow :QTextCursor__MoveOperation = 23;
// 
pub const QTextCursor__PreviousRow :QTextCursor__MoveOperation = 24;
pub fn QTextCursor_MoveOperationItemName(val: i32) ->String {
  match val {
     QTextCursor__NoMove => // 0
     {return String::from("NoMove");}
     QTextCursor__Start => // 1
     {return String::from("Start");}
     QTextCursor__Up => // 2
     {return String::from("Up");}
     QTextCursor__StartOfLine => // 3
     {return String::from("StartOfLine");}
     QTextCursor__StartOfBlock => // 4
     {return String::from("StartOfBlock");}
     QTextCursor__StartOfWord => // 5
     {return String::from("StartOfWord");}
     QTextCursor__PreviousBlock => // 6
     {return String::from("PreviousBlock");}
     QTextCursor__PreviousCharacter => // 7
     {return String::from("PreviousCharacter");}
     QTextCursor__PreviousWord => // 8
     {return String::from("PreviousWord");}
     QTextCursor__Left => // 9
     {return String::from("Left");}
     QTextCursor__WordLeft => // 10
     {return String::from("WordLeft");}
     QTextCursor__End => // 11
     {return String::from("End");}
     QTextCursor__Down => // 12
     {return String::from("Down");}
     QTextCursor__EndOfLine => // 13
     {return String::from("EndOfLine");}
     QTextCursor__EndOfWord => // 14
     {return String::from("EndOfWord");}
     QTextCursor__EndOfBlock => // 15
     {return String::from("EndOfBlock");}
     QTextCursor__NextBlock => // 16
     {return String::from("NextBlock");}
     QTextCursor__NextCharacter => // 17
     {return String::from("NextCharacter");}
     QTextCursor__NextWord => // 18
     {return String::from("NextWord");}
     QTextCursor__Right => // 19
     {return String::from("Right");}
     QTextCursor__WordRight => // 20
     {return String::from("WordRight");}
     QTextCursor__NextCell => // 21
     {return String::from("NextCell");}
     QTextCursor__PreviousCell => // 22
     {return String::from("PreviousCell");}
     QTextCursor__NextRow => // 23
     {return String::from("NextRow");}
     QTextCursor__PreviousRow => // 24
     {return String::from("PreviousRow");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextCursor_MoveOperationItemName_s(val: i32) ->String {
  //var nilthis *QTextCursor
  //return nilthis.MoveOperationItemName(val);
  return QTextCursor_MoveOperationItemName(val);
}


/*
This enum describes the types of selection that can be applied with the select() function.


*/
pub type QTextCursor__SelectionType = i32;
// Selects the word under the cursor. If the cursor is not positioned within a string of selectable characters, no text is selected.
pub const QTextCursor__WordUnderCursor :QTextCursor__SelectionType = 0;
// Selects the line of text under the cursor.
pub const QTextCursor__LineUnderCursor :QTextCursor__SelectionType = 1;
// Selects the block of text under the cursor.
pub const QTextCursor__BlockUnderCursor :QTextCursor__SelectionType = 2;
// Selects the entire document.
pub const QTextCursor__Document :QTextCursor__SelectionType = 3;
pub fn QTextCursor_SelectionTypeItemName(val: i32) ->String {
  match val {
     QTextCursor__WordUnderCursor => // 0
     {return String::from("WordUnderCursor");}
     QTextCursor__LineUnderCursor => // 1
     {return String::from("LineUnderCursor");}
     QTextCursor__BlockUnderCursor => // 2
     {return String::from("BlockUnderCursor");}
     QTextCursor__Document => // 3
     {return String::from("Document");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextCursor_SelectionTypeItemName_s(val: i32) ->String {
  //var nilthis *QTextCursor
  //return nilthis.SelectionTypeItemName(val);
  return QTextCursor_SelectionTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
