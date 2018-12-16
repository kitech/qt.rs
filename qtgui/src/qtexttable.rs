

// mod ::gui::QTextTable
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
// extern C begin: 19
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
#[derive(Default)] // class sizeof(QTextTable)=16
pub struct QTextTable {
  qbase: QTextFrame,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextTable_ITF interface {
//    QTextFrame_ITF
//    QTextTable_PTR() *QTextTable
//}
//func (ptr *QTextTable) QTextTable_PTR() *QTextTable { return ptr }

impl /*struct*/ QTextTable {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextTable {
    return QTextTable{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextTable {
//  type Target = QTextTableBASE;
//
//  fn deref(&self) -> &QTextTableBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextTableBASE> for QTextTable {
//  fn as_ref(& self) -> & QTextTableBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtexttable.h:100
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTextTable {
  pub fn metaObject_0<RetType, T: QTextTable_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTextTable_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTextTable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextTable10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextTable(QTextDocument *)

/*

*/
// QTextTable(QTextDocument *) ctx.fn_proto_cpp
impl /*struct*/ QTextTable {
  pub fn QTextTable_0<T: QTextTable_QTextTable_0>(value: T) -> QTextTable {
    let rsthis = value.QTextTable_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTable_QTextTable_0 {
  fn QTextTable_0(self) -> QTextTable;
}
// QTextTable(QTextDocument *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextTable_QTextTable_0 for (usize) {
  fn QTextTable_0(self) -> QTextTable {
    // unsafe{_ZN10QTextTableC2EP13QTextDocument()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QTextTableC2EP13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextTable{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:103
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTextTable()

/*

*/
pub fn DeleteQTextTable(this :*mut QTextTable) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QTextTableD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtexttable.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resize(int, int)

/*
Resizes the table to contain the required number of rows and columns.

See also insertRows(), insertColumns(), removeRows(), and removeColumns().
*/
impl /*struct*/ QTextTable {
  pub fn resize_0<RetType, T: QTextTable_resize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_0(self);
    // return 1;
  }
}
pub trait QTextTable_resize_0<RetType> {
  fn resize_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_resize_0<(/*void*/)> for (i32,i32) {
  fn resize_0(self , rsthis: & QTextTable) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextTable6resizeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertRows(int, int)

/*
Inserts a number of rows before the row with the specified index.

See also resize(), insertColumns(), removeRows(), removeColumns(), appendRows(), and appendColumns().
*/
impl /*struct*/ QTextTable {
  pub fn insertRows_0<RetType, T: QTextTable_insertRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertRows_0(self);
    // return 1;
  }
}
pub trait QTextTable_insertRows_0<RetType> {
  fn insertRows_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_insertRows_0<(/*void*/)> for (i32,i32) {
  fn insertRows_0(self , rsthis: & QTextTable) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextTable10insertRowsEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertColumns(int, int)

/*
Inserts a number of columns before the column with the specified index.

See also insertRows(), resize(), removeRows(), removeColumns(), appendRows(), and appendColumns().
*/
impl /*struct*/ QTextTable {
  pub fn insertColumns_0<RetType, T: QTextTable_insertColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertColumns_0(self);
    // return 1;
  }
}
pub trait QTextTable_insertColumns_0<RetType> {
  fn insertColumns_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_insertColumns_0<(/*void*/)> for (i32,i32) {
  fn insertColumns_0(self , rsthis: & QTextTable) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextTable13insertColumnsEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void appendRows(int)

/*
Appends count rows at the bottom of the table.

This function was introduced in  Qt 4.5.

See also insertColumns(), insertRows(), resize(), removeRows(), removeColumns(), and appendColumns().
*/
impl /*struct*/ QTextTable {
  pub fn appendRows_0<RetType, T: QTextTable_appendRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.appendRows_0(self);
    // return 1;
  }
}
pub trait QTextTable_appendRows_0<RetType> {
  fn appendRows_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_appendRows_0<(/*void*/)> for (i32) {
  fn appendRows_0(self , rsthis: & QTextTable) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextTable10appendRowsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void appendColumns(int)

/*
Appends count columns at the right side of the table.

This function was introduced in  Qt 4.5.

See also insertColumns(), insertRows(), resize(), removeRows(), removeColumns(), and appendRows().
*/
impl /*struct*/ QTextTable {
  pub fn appendColumns_0<RetType, T: QTextTable_appendColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.appendColumns_0(self);
    // return 1;
  }
}
pub trait QTextTable_appendColumns_0<RetType> {
  fn appendColumns_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_appendColumns_0<(/*void*/)> for (i32) {
  fn appendColumns_0(self , rsthis: & QTextTable) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextTable13appendColumnsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeRows(int, int)

/*
Removes a number of rows starting with the row at the specified index.

See also insertRows(), insertColumns(), resize(), removeColumns(), appendRows(), and appendColumns().
*/
impl /*struct*/ QTextTable {
  pub fn removeRows_0<RetType, T: QTextTable_removeRows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRows_0(self);
    // return 1;
  }
}
pub trait QTextTable_removeRows_0<RetType> {
  fn removeRows_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_removeRows_0<(/*void*/)> for (i32,i32) {
  fn removeRows_0(self , rsthis: & QTextTable) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextTable10removeRowsEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeColumns(int, int)

/*
Removes a number of columns starting with the column at the specified index.

See also insertRows(), insertColumns(), removeRows(), resize(), appendRows(), and appendColumns().
*/
impl /*struct*/ QTextTable {
  pub fn removeColumns_0<RetType, T: QTextTable_removeColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeColumns_0(self);
    // return 1;
  }
}
pub trait QTextTable_removeColumns_0<RetType> {
  fn removeColumns_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_removeColumns_0<(/*void*/)> for (i32,i32) {
  fn removeColumns_0(self , rsthis: & QTextTable) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextTable13removeColumnsEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void mergeCells(int, int, int, int)

/*
Merges the cell at the specified row and column with the adjacent cells into one cell. The new cell will span numRows rows and numCols columns. This method does nothing if numRows or numCols is less than the current number of rows or columns spanned by the cell.

This function was introduced in  Qt 4.1.

See also splitCell().
*/
impl /*struct*/ QTextTable {
  pub fn mergeCells_0<RetType, T: QTextTable_mergeCells_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mergeCells_0(self);
    // return 1;
  }
}
pub trait QTextTable_mergeCells_0<RetType> {
  fn mergeCells_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_mergeCells_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn mergeCells_0(self , rsthis: & QTextTable) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextTable10mergeCellsEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:114
// index:1
// Public Visibility=Default Availability=Available
// [-2] void mergeCells(const QTextCursor &)

/*
Merges the cell at the specified row and column with the adjacent cells into one cell. The new cell will span numRows rows and numCols columns. This method does nothing if numRows or numCols is less than the current number of rows or columns spanned by the cell.

This function was introduced in  Qt 4.1.

See also splitCell().
*/
impl /*struct*/ QTextTable {
  pub fn mergeCells_1<RetType, T: QTextTable_mergeCells_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mergeCells_1(self);
    // return 1;
  }
}
pub trait QTextTable_mergeCells_1<RetType> {
  fn mergeCells_1(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_mergeCells_1<(/*void*/)> for (usize) {
  fn mergeCells_1(self , rsthis: & QTextTable) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextTable10mergeCellsERK11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void splitCell(int, int, int, int)

/*
Splits the specified cell at row and column into an array of multiple cells with dimensions specified by numRows and numCols.

Note: It is only possible to split cells that span multiple rows or columns, such as rows that have been merged using mergeCells().

This function was introduced in  Qt 4.1.

See also mergeCells().
*/
impl /*struct*/ QTextTable {
  pub fn splitCell_0<RetType, T: QTextTable_splitCell_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.splitCell_0(self);
    // return 1;
  }
}
pub trait QTextTable_splitCell_0<RetType> {
  fn splitCell_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_splitCell_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn splitCell_0(self , rsthis: & QTextTable) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextTable9splitCellEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:117
// index:0
// Public Visibility=Default Availability=Available
// [4] int rows() const

/*
Returns the number of rows in the table.

See also columns().
*/
impl /*struct*/ QTextTable {
  pub fn rows_0<RetType, T: QTextTable_rows_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rows_0(self);
    // return 1;
  }
}
pub trait QTextTable_rows_0<RetType> {
  fn rows_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_rows_0<i32> for () {
  fn rows_0(self , rsthis: & QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextTable4rowsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:118
// index:0
// Public Visibility=Default Availability=Available
// [4] int columns() const

/*
Returns the number of columns in the table.

See also rows().
*/
impl /*struct*/ QTextTable {
  pub fn columns_0<RetType, T: QTextTable_columns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columns_0(self);
    // return 1;
  }
}
pub trait QTextTable_columns_0<RetType> {
  fn columns_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_columns_0<i32> for () {
  fn columns_0(self , rsthis: & QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextTable7columnsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:120
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextTableCell cellAt(int, int) const

/*
Returns the table cell at the given row and column in the table.

See also columns() and rows().
*/
impl /*struct*/ QTextTable {
  pub fn cellAt_0<RetType, T: QTextTable_cellAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellAt_0(self);
    // return 1;
  }
}
pub trait QTextTable_cellAt_0<RetType> {
  fn cellAt_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_cellAt_0<usize> for (i32,i32) {
  fn cellAt_0(self , rsthis: & QTextTable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextTable6cellAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:121
// index:1
// Public Visibility=Default Availability=Available
// [16] QTextTableCell cellAt(int) const

/*
Returns the table cell at the given row and column in the table.

See also columns() and rows().
*/
impl /*struct*/ QTextTable {
  pub fn cellAt_1<RetType, T: QTextTable_cellAt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellAt_1(self);
    // return 1;
  }
}
pub trait QTextTable_cellAt_1<RetType> {
  fn cellAt_1(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_cellAt_1<usize> for (i32) {
  fn cellAt_1(self , rsthis: & QTextTable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextTable6cellAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:122
// index:2
// Public Visibility=Default Availability=Available
// [16] QTextTableCell cellAt(const QTextCursor &) const

/*
Returns the table cell at the given row and column in the table.

See also columns() and rows().
*/
impl /*struct*/ QTextTable {
  pub fn cellAt_2<RetType, T: QTextTable_cellAt_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cellAt_2(self);
    // return 1;
  }
}
pub trait QTextTable_cellAt_2<RetType> {
  fn cellAt_2(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_cellAt_2<usize> for (usize) {
  fn cellAt_2(self , rsthis: & QTextTable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextTable6cellAtERK11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:124
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCursor rowStart(const QTextCursor &) const

/*
Returns a cursor pointing to the start of the row that contains the given cursor.

See also rowEnd().
*/
impl /*struct*/ QTextTable {
  pub fn rowStart_0<RetType, T: QTextTable_rowStart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowStart_0(self);
    // return 1;
  }
}
pub trait QTextTable_rowStart_0<RetType> {
  fn rowStart_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_rowStart_0<usize> for (usize) {
  fn rowStart_0(self , rsthis: & QTextTable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextTable8rowStartERK11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:125
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCursor rowEnd(const QTextCursor &) const

/*
Returns a cursor pointing to the end of the row that contains the given cursor.

See also rowStart().
*/
impl /*struct*/ QTextTable {
  pub fn rowEnd_0<RetType, T: QTextTable_rowEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rowEnd_0(self);
    // return 1;
  }
}
pub trait QTextTable_rowEnd_0<RetType> {
  fn rowEnd_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_rowEnd_0<usize> for (usize) {
  fn rowEnd_0(self , rsthis: & QTextTable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextTable6rowEndERK11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFormat(const QTextTableFormat &)

/*
Sets the table's format.

See also format().
*/
impl /*struct*/ QTextTable {
  pub fn setFormat_0<RetType, T: QTextTable_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QTextTable_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_setFormat_0<(/*void*/)> for (usize) {
  fn setFormat_0(self , rsthis: & QTextTable) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextTable9setFormatERK16QTextTableFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtexttable.h:128
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QTextTableFormat format() const

/*
Returns the table's format.

See also setFormat().
*/
impl /*struct*/ QTextTable {
  pub fn format_0<RetType, T: QTextTable_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QTextTable_format_0<RetType> {
  fn format_0(self , rsthis: & QTextTable) -> RetType;
}
impl<'a> /*trait*/ QTextTable_format_0<usize> for () {
  fn format_0(self , rsthis: & QTextTable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextTable6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
