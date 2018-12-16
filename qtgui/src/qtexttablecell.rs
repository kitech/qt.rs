

// mod ::gui::QTextTableCell
// package qtgui
// /usr/include/qt/QtGui/qtexttable.h
// #include <qtexttable.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 13
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
#[derive(Default)] // class sizeof(QTextTableCell)=16
pub struct QTextTableCell {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextTableCell_ITF interface {
//    QTextTableCell_PTR() *QTextTableCell
//}
//func (ptr *QTextTableCell) QTextTableCell_PTR() *QTextTableCell { return ptr }

impl /*struct*/ QTextTableCell {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextTableCell {
    return QTextTableCell{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextTableCell {
//  type Target = QTextTableCellBASE;
//
//  fn deref(&self) -> &QTextTableCellBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextTableCellBASE> for QTextTableCell {
//  fn as_ref(& self) -> & QTextTableCellBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtexttable.h:57
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QTextTableCell()

/*

*/
// QTextTableCell() ctx.fn_proto_cpp
impl /*struct*/ QTextTableCell {
  pub fn QTextTableCell_0<T: QTextTableCell_QTextTableCell_0>(value: T) -> QTextTableCell {
    let rsthis = value.QTextTableCell_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableCell_QTextTableCell_0 {
  fn QTextTableCell_0(self) -> QTextTableCell;
}
// QTextTableCell() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextTableCell_QTextTableCell_0 for () {
  fn QTextTableCell_0(self) -> QTextTableCell {
    // unsafe{_ZN14QTextTableCellC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QTextTableCellC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextTableCell{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:58
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~QTextTableCell()

/*

*/
pub fn DeleteQTextTableCell(this :*mut QTextTableCell) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QTextTableCellD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtexttable.h:60
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QTextTableCell & operator=(const QTextTableCell &)

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn operator_equal_0<RetType, T: QTextTableCell_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QTextTableCell) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QTextTableCellaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFormat(const QTextCharFormat &)

/*
Sets the table's format.

See also format().
*/
impl /*struct*/ QTextTableCell {
  pub fn setFormat_0<RetType, T: QTextTableCell_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_setFormat_0<(/*void*/)> for (usize) {
  fn setFormat_0(self , rsthis: & QTextTableCell) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QTextTableCell9setFormatERK15QTextCharFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:64
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextCharFormat format() const

/*
Returns the table's format.

See also setFormat().
*/
impl /*struct*/ QTextTableCell {
  pub fn format_0<RetType, T: QTextTableCell_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_format_0<RetType> {
  fn format_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_format_0<usize> for () {
  fn format_0(self , rsthis: & QTextTableCell) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCell6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:66
// index:0
// Public Visibility=Default Availability=Available
// [4] int row() const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn row_0<RetType, T: QTextTableCell_row_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.row_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_row_0<RetType> {
  fn row_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_row_0<i32> for () {
  fn row_0(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCell3rowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:67
// index:0
// Public Visibility=Default Availability=Available
// [4] int column() const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn column_0<RetType, T: QTextTableCell_column_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.column_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_column_0<RetType> {
  fn column_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_column_0<i32> for () {
  fn column_0(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCell6columnEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:69
// index:0
// Public Visibility=Default Availability=Available
// [4] int rowSpan() const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn rowSpan_0<RetType, T: QTextTableCell_rowSpan_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowSpan_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_rowSpan_0<RetType> {
  fn rowSpan_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_rowSpan_0<i32> for () {
  fn rowSpan_0(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCell7rowSpanEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:70
// index:0
// Public Visibility=Default Availability=Available
// [4] int columnSpan() const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn columnSpan_0<RetType, T: QTextTableCell_columnSpan_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnSpan_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_columnSpan_0<RetType> {
  fn columnSpan_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_columnSpan_0<i32> for () {
  fn columnSpan_0(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCell10columnSpanEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:72
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn isValid_0<RetType, T: QTextTableCell_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextTableCell) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCell7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCursor firstCursorPosition() const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn firstCursorPosition_0<RetType, T: QTextTableCell_firstCursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.firstCursorPosition_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_firstCursorPosition_0<RetType> {
  fn firstCursorPosition_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_firstCursorPosition_0<usize> for () {
  fn firstCursorPosition_0(self , rsthis: & QTextTableCell) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCell19firstCursorPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:75
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCursor lastCursorPosition() const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn lastCursorPosition_0<RetType, T: QTextTableCell_lastCursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastCursorPosition_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_lastCursorPosition_0<RetType> {
  fn lastCursorPosition_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_lastCursorPosition_0<usize> for () {
  fn lastCursorPosition_0(self , rsthis: & QTextTableCell) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCell18lastCursorPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] int firstPosition() const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn firstPosition_0<RetType, T: QTextTableCell_firstPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.firstPosition_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_firstPosition_0<RetType> {
  fn firstPosition_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_firstPosition_0<i32> for () {
  fn firstPosition_0(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCell13firstPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:77
// index:0
// Public Visibility=Default Availability=Available
// [4] int lastPosition() const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn lastPosition_0<RetType, T: QTextTableCell_lastPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastPosition_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_lastPosition_0<RetType> {
  fn lastPosition_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_lastPosition_0<i32> for () {
  fn lastPosition_0(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCell12lastPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:79
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QTextTableCell &) const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn operator_equal_equal_0<RetType, T: QTextTableCell_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QTextTableCell) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCelleqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QTextTableCell &) const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn operator_not_equal_0<RetType, T: QTextTableCell_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QTextTableCell) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCellneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:84
// index:0
// Public Visibility=Default Availability=Available
// [32] QTextFrame::iterator begin() const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn begin_0<RetType, T: QTextTableCell_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_begin_0<RetType> {
  fn begin_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_begin_0<i32> for () {
  fn begin_0(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCell5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:85
// index:0
// Public Visibility=Default Availability=Available
// [32] QTextFrame::iterator end() const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn end_0<RetType, T: QTextTableCell_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_end_0<RetType> {
  fn end_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_end_0<i32> for () {
  fn end_0(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCell3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:87
// index:0
// Public Visibility=Default Availability=Available
// [4] int tableCellFormatIndex() const

/*

*/
impl /*struct*/ QTextTableCell {
  pub fn tableCellFormatIndex_0<RetType, T: QTextTableCell_tableCellFormatIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tableCellFormatIndex_0(self);
    // return 1;
  }
}
pub trait QTextTableCell_tableCellFormatIndex_0<RetType> {
  fn tableCellFormatIndex_0(self , rsthis: & QTextTableCell) -> RetType;
}
impl<'a> /*trait*/ QTextTableCell_tableCellFormatIndex_0<i32> for () {
  fn tableCellFormatIndex_0(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QTextTableCell20tableCellFormatIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
