// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextcursor::QTextCursor;
use super::qtexttableformat::QTextTableFormat;
use super::qtextdocument::QTextDocument;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QTextTableCell QTextTable::cellAt(int row, int col);
  fn _ZNK10QTextTable6cellAtEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: int QTextTable::rows();
  fn _ZNK10QTextTable4rowsEv() -> i32;
  // proto: void QTextTable::removeRows(int pos, int num);
  fn _ZN10QTextTable10removeRowsEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: int QTextTable::columns();
  fn _ZNK10QTextTable7columnsEv() -> i32;
  // proto: void QTextTable::appendRows(int count);
  fn _ZN10QTextTable10appendRowsEi(arg0: c_int) -> i32;
  // proto: void QTextTable::resize(int rows, int cols);
  fn _ZN10QTextTable6resizeEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QTextTableCell QTextTable::cellAt(const QTextCursor & c);
  fn _ZNK10QTextTable6cellAtERK11QTextCursor(arg0: *const c_void) -> i32;
  // proto: void QTextTable::NewQTextTable(const QTextTable & );
  fn _ZN10QTextTableC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QTextTable::setFormat(const QTextTableFormat & format);
  fn _ZN10QTextTable9setFormatERK16QTextTableFormat(arg0: *const c_void) -> i32;
  // proto: void QTextTable::insertColumns(int pos, int num);
  fn _ZN10QTextTable13insertColumnsEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QTextTable::splitCell(int row, int col, int numRows, int numCols);
  fn _ZN10QTextTable9splitCellEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QTextTable::mergeCells(int row, int col, int numRows, int numCols);
  fn _ZN10QTextTable10mergeCellsEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QTextTable::insertRows(int pos, int num);
  fn _ZN10QTextTable10insertRowsEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QTextTable::FreeQTextTable();
  fn _ZN10QTextTableD0Ev() -> i32;
  // proto: void QTextTable::NewQTextTable(QTextDocument * doc);
  fn _ZN10QTextTableC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QTextTableCell QTextTable::cellAt(int position);
  fn _ZNK10QTextTable6cellAtEi(arg0: c_int) -> i32;
  // proto: QTextCursor QTextTable::rowStart(const QTextCursor & c);
  fn _ZNK10QTextTable8rowStartERK11QTextCursor(arg0: *const c_void) -> i32;
  // proto: QTextTableFormat QTextTable::format();
  fn _ZNK10QTextTable6formatEv() -> i32;
  // proto: QTextCursor QTextTable::rowEnd(const QTextCursor & c);
  fn _ZNK10QTextTable6rowEndERK11QTextCursor(arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QTextTable::metaObject();
  fn _ZNK10QTextTable10metaObjectEv() -> i32;
  // proto: void QTextTable::removeColumns(int pos, int num);
  fn _ZN10QTextTable13removeColumnsEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QTextTable::appendColumns(int count);
  fn _ZN10QTextTable13appendColumnsEi(arg0: c_int) -> i32;
  // proto: void QTextTable::mergeCells(const QTextCursor & cursor);
  fn _ZN10QTextTable10mergeCellsERK11QTextCursor(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QTextTable)=1
pub struct QTextTable {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextTable {
  pub fn cellAt<T: QTextTable_cellAt>(&mut self, value: T) -> i32 {
    value.cellAt(self);
    return 1;
  }
}

pub trait QTextTable_cellAt {
  fn cellAt(self, this: &mut QTextTable) -> i32;
}

// proto: QTextTableCell QTextTable::cellAt(int row, int col);
impl<'a> /*trait*/ QTextTable_cellAt for (i32, i32) {
  fn cellAt(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6cellAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QTextTable6cellAtEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn rows<T: QTextTable_rows>(&mut self, value: T) -> i32 {
    value.rows(self);
    return 1;
  }
}

pub trait QTextTable_rows {
  fn rows(self, this: &mut QTextTable) -> i32;
}

// proto: int QTextTable::rows();
impl<'a> /*trait*/ QTextTable_rows for () {
  fn rows(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable4rowsEv()};
    unsafe {_ZNK10QTextTable4rowsEv()};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn removeRows<T: QTextTable_removeRows>(&mut self, value: T) -> i32 {
    value.removeRows(self);
    return 1;
  }
}

pub trait QTextTable_removeRows {
  fn removeRows(self, this: &mut QTextTable) -> i32;
}

// proto: void QTextTable::removeRows(int pos, int num);
impl<'a> /*trait*/ QTextTable_removeRows for (i32, i32) {
  fn removeRows(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10removeRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QTextTable10removeRowsEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn columns<T: QTextTable_columns>(&mut self, value: T) -> i32 {
    value.columns(self);
    return 1;
  }
}

pub trait QTextTable_columns {
  fn columns(self, this: &mut QTextTable) -> i32;
}

// proto: int QTextTable::columns();
impl<'a> /*trait*/ QTextTable_columns for () {
  fn columns(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable7columnsEv()};
    unsafe {_ZNK10QTextTable7columnsEv()};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn appendRows<T: QTextTable_appendRows>(&mut self, value: T) -> i32 {
    value.appendRows(self);
    return 1;
  }
}

pub trait QTextTable_appendRows {
  fn appendRows(self, this: &mut QTextTable) -> i32;
}

// proto: void QTextTable::appendRows(int count);
impl<'a> /*trait*/ QTextTable_appendRows for (i32) {
  fn appendRows(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10appendRowsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QTextTable10appendRowsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn resize<T: QTextTable_resize>(&mut self, value: T) -> i32 {
    value.resize(self);
    return 1;
  }
}

pub trait QTextTable_resize {
  fn resize(self, this: &mut QTextTable) -> i32;
}

// proto: void QTextTable::resize(int rows, int cols);
impl<'a> /*trait*/ QTextTable_resize for (i32, i32) {
  fn resize(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable6resizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QTextTable6resizeEii(arg0, arg1)};
    return 1;
  }
}

// proto: QTextTableCell QTextTable::cellAt(const QTextCursor & c);
impl<'a> /*trait*/ QTextTable_cellAt for (&'a  QTextCursor) {
  fn cellAt(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6cellAtERK11QTextCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTextTable6cellAtERK11QTextCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn NewQTextTable<T: QTextTable_NewQTextTable>(value: T) -> QTextTable {
    let rsthis = value.NewQTextTable();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTable_NewQTextTable {
  fn NewQTextTable(self) -> QTextTable;
}

// proto: void QTextTable::NewQTextTable(const QTextTable & );
impl<'a> /*trait*/ QTextTable_NewQTextTable for (&'a  QTextTable) {
  fn NewQTextTable(self) -> QTextTable {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTableC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QTextTableC1ERKS_(qthis, arg0)};
    let rsthis = QTextTable{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn setFormat<T: QTextTable_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QTextTable_setFormat {
  fn setFormat(self, this: &mut QTextTable) -> i32;
}

// proto: void QTextTable::setFormat(const QTextTableFormat & format);
impl<'a> /*trait*/ QTextTable_setFormat for (&'a  QTextTableFormat) {
  fn setFormat(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable9setFormatERK16QTextTableFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QTextTable9setFormatERK16QTextTableFormat(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn insertColumns<T: QTextTable_insertColumns>(&mut self, value: T) -> i32 {
    value.insertColumns(self);
    return 1;
  }
}

pub trait QTextTable_insertColumns {
  fn insertColumns(self, this: &mut QTextTable) -> i32;
}

// proto: void QTextTable::insertColumns(int pos, int num);
impl<'a> /*trait*/ QTextTable_insertColumns for (i32, i32) {
  fn insertColumns(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable13insertColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QTextTable13insertColumnsEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn splitCell<T: QTextTable_splitCell>(&mut self, value: T) -> i32 {
    value.splitCell(self);
    return 1;
  }
}

pub trait QTextTable_splitCell {
  fn splitCell(self, this: &mut QTextTable) -> i32;
}

// proto: void QTextTable::splitCell(int row, int col, int numRows, int numCols);
impl<'a> /*trait*/ QTextTable_splitCell for (i32, i32, i32, i32) {
  fn splitCell(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable9splitCellEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN10QTextTable9splitCellEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn mergeCells<T: QTextTable_mergeCells>(&mut self, value: T) -> i32 {
    value.mergeCells(self);
    return 1;
  }
}

pub trait QTextTable_mergeCells {
  fn mergeCells(self, this: &mut QTextTable) -> i32;
}

// proto: void QTextTable::mergeCells(int row, int col, int numRows, int numCols);
impl<'a> /*trait*/ QTextTable_mergeCells for (i32, i32, i32, i32) {
  fn mergeCells(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10mergeCellsEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN10QTextTable10mergeCellsEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn insertRows<T: QTextTable_insertRows>(&mut self, value: T) -> i32 {
    value.insertRows(self);
    return 1;
  }
}

pub trait QTextTable_insertRows {
  fn insertRows(self, this: &mut QTextTable) -> i32;
}

// proto: void QTextTable::insertRows(int pos, int num);
impl<'a> /*trait*/ QTextTable_insertRows for (i32, i32) {
  fn insertRows(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10insertRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QTextTable10insertRowsEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn FreeQTextTable<T: QTextTable_FreeQTextTable>(&mut self, value: T) -> i32 {
    value.FreeQTextTable(self);
    return 1;
  }
}

pub trait QTextTable_FreeQTextTable {
  fn FreeQTextTable(self, this: &mut QTextTable) -> i32;
}

// proto: void QTextTable::FreeQTextTable();
impl<'a> /*trait*/ QTextTable_FreeQTextTable for () {
  fn FreeQTextTable(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTableD0Ev()};
    unsafe {_ZN10QTextTableD0Ev()};
    return 1;
  }
}

// proto: void QTextTable::NewQTextTable(QTextDocument * doc);
impl<'a> /*trait*/ QTextTable_NewQTextTable for (&'a mut QTextDocument) {
  fn NewQTextTable(self) -> QTextTable {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTableC1EP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTextTableC1EP13QTextDocument(qthis, arg0)};
    let rsthis = QTextTable{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QTextTableCell QTextTable::cellAt(int position);
impl<'a> /*trait*/ QTextTable_cellAt for (i32) {
  fn cellAt(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6cellAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QTextTable6cellAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn rowStart<T: QTextTable_rowStart>(&mut self, value: T) -> i32 {
    value.rowStart(self);
    return 1;
  }
}

pub trait QTextTable_rowStart {
  fn rowStart(self, this: &mut QTextTable) -> i32;
}

// proto: QTextCursor QTextTable::rowStart(const QTextCursor & c);
impl<'a> /*trait*/ QTextTable_rowStart for (&'a  QTextCursor) {
  fn rowStart(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable8rowStartERK11QTextCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTextTable8rowStartERK11QTextCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn format<T: QTextTable_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QTextTable_format {
  fn format(self, this: &mut QTextTable) -> i32;
}

// proto: QTextTableFormat QTextTable::format();
impl<'a> /*trait*/ QTextTable_format for () {
  fn format(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6formatEv()};
    unsafe {_ZNK10QTextTable6formatEv()};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn rowEnd<T: QTextTable_rowEnd>(&mut self, value: T) -> i32 {
    value.rowEnd(self);
    return 1;
  }
}

pub trait QTextTable_rowEnd {
  fn rowEnd(self, this: &mut QTextTable) -> i32;
}

// proto: QTextCursor QTextTable::rowEnd(const QTextCursor & c);
impl<'a> /*trait*/ QTextTable_rowEnd for (&'a  QTextCursor) {
  fn rowEnd(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6rowEndERK11QTextCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QTextTable6rowEndERK11QTextCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn metaObject<T: QTextTable_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTextTable_metaObject {
  fn metaObject(self, this: &mut QTextTable) -> i32;
}

// proto: const QMetaObject * QTextTable::metaObject();
impl<'a> /*trait*/ QTextTable_metaObject for () {
  fn metaObject(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable10metaObjectEv()};
    unsafe {_ZNK10QTextTable10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn removeColumns<T: QTextTable_removeColumns>(&mut self, value: T) -> i32 {
    value.removeColumns(self);
    return 1;
  }
}

pub trait QTextTable_removeColumns {
  fn removeColumns(self, this: &mut QTextTable) -> i32;
}

// proto: void QTextTable::removeColumns(int pos, int num);
impl<'a> /*trait*/ QTextTable_removeColumns for (i32, i32) {
  fn removeColumns(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable13removeColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QTextTable13removeColumnsEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn appendColumns<T: QTextTable_appendColumns>(&mut self, value: T) -> i32 {
    value.appendColumns(self);
    return 1;
  }
}

pub trait QTextTable_appendColumns {
  fn appendColumns(self, this: &mut QTextTable) -> i32;
}

// proto: void QTextTable::appendColumns(int count);
impl<'a> /*trait*/ QTextTable_appendColumns for (i32) {
  fn appendColumns(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable13appendColumnsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QTextTable13appendColumnsEi(arg0)};
    return 1;
  }
}

// proto: void QTextTable::mergeCells(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextTable_mergeCells for (&'a  QTextCursor) {
  fn mergeCells(self, this: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10mergeCellsERK11QTextCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QTextTable10mergeCellsERK11QTextCursor(arg0)};
    return 1;
  }
}

