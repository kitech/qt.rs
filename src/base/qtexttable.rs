// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtexttablecell::QTextTableCell;
use super::qtextcursor::QTextCursor;
use super::qtexttableformat::QTextTableFormat;
use super::qtextdocument::QTextDocument;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QTextTableCell QTextTable::cellAt(int row, int col);
  fn _ZNK10QTextTable6cellAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  int QTextTable::rows();
  fn _ZNK10QTextTable4rowsEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextTable::removeRows(int pos, int num);
  fn _ZN10QTextTable10removeRowsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  int QTextTable::columns();
  fn _ZNK10QTextTable7columnsEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextTable::appendRows(int count);
  fn _ZN10QTextTable10appendRowsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextTable::resize(int rows, int cols);
  fn _ZN10QTextTable6resizeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QTextTableCell QTextTable::cellAt(const QTextCursor & c);
  fn _ZNK10QTextTable6cellAtERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextTable::NewQTextTable(const QTextTable & );
  fn _ZN10QTextTableC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextTable::setFormat(const QTextTableFormat & format);
  fn _ZN10QTextTable9setFormatERK16QTextTableFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextTable::insertColumns(int pos, int num);
  fn _ZN10QTextTable13insertColumnsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTextTable::splitCell(int row, int col, int numRows, int numCols);
  fn _ZN10QTextTable9splitCellEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QTextTable::mergeCells(int row, int col, int numRows, int numCols);
  fn _ZN10QTextTable10mergeCellsEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QTextTable::insertRows(int pos, int num);
  fn _ZN10QTextTable10insertRowsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTextTable::FreeQTextTable();
  fn _ZN10QTextTableD0Ev(qthis: *mut c_void) ;
  // proto:  void QTextTable::NewQTextTable(QTextDocument * doc);
  fn _ZN10QTextTableC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextTableCell QTextTable::cellAt(int position);
  fn _ZNK10QTextTable6cellAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QTextCursor QTextTable::rowStart(const QTextCursor & c);
  fn _ZNK10QTextTable8rowStartERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QTextTableFormat QTextTable::format();
  fn _ZNK10QTextTable6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextCursor QTextTable::rowEnd(const QTextCursor & c);
  fn _ZNK10QTextTable6rowEndERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QTextTable::metaObject();
  fn _ZNK10QTextTable10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTextTable::removeColumns(int pos, int num);
  fn _ZN10QTextTable13removeColumnsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTextTable::appendColumns(int count);
  fn _ZN10QTextTable13appendColumnsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextTable::mergeCells(const QTextCursor & cursor);
  fn _ZN10QTextTable10mergeCellsERK11QTextCursor(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTextTable)=1
pub struct QTextTable {
  pub qclsinst: *mut c_void,
}

// proto:  QTextTableCell QTextTable::cellAt(int row, int col);
impl /*struct*/ QTextTable {
  pub fn cellAt<RetType, T: QTextTable_cellAt<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cellAt(self);
    // return 1;
  }
}

pub trait QTextTable_cellAt<RetType> {
  fn cellAt(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  QTextTableCell QTextTable::cellAt(int row, int col);
impl<'a> /*trait*/ QTextTable_cellAt<QTextTableCell> for (i32, i32) {
  fn cellAt(self , rsthis: &mut QTextTable) -> QTextTableCell {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6cellAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QTextTable6cellAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTextTableCell{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QTextTable::rows();
impl /*struct*/ QTextTable {
  pub fn rows<RetType, T: QTextTable_rows<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rows(self);
    // return 1;
  }
}

pub trait QTextTable_rows<RetType> {
  fn rows(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  int QTextTable::rows();
impl<'a> /*trait*/ QTextTable_rows<i32> for () {
  fn rows(self , rsthis: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable4rowsEv()};
    let mut ret = unsafe {_ZNK10QTextTable4rowsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTextTable::removeRows(int pos, int num);
impl /*struct*/ QTextTable {
  pub fn removeRows<RetType, T: QTextTable_removeRows<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.removeRows(self);
    // return 1;
  }
}

pub trait QTextTable_removeRows<RetType> {
  fn removeRows(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  void QTextTable::removeRows(int pos, int num);
impl<'a> /*trait*/ QTextTable_removeRows<()> for (i32, i32) {
  fn removeRows(self , rsthis: &mut QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10removeRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QTextTable10removeRowsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  int QTextTable::columns();
impl /*struct*/ QTextTable {
  pub fn columns<RetType, T: QTextTable_columns<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.columns(self);
    // return 1;
  }
}

pub trait QTextTable_columns<RetType> {
  fn columns(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  int QTextTable::columns();
impl<'a> /*trait*/ QTextTable_columns<i32> for () {
  fn columns(self , rsthis: &mut QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable7columnsEv()};
    let mut ret = unsafe {_ZNK10QTextTable7columnsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTextTable::appendRows(int count);
impl /*struct*/ QTextTable {
  pub fn appendRows<RetType, T: QTextTable_appendRows<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.appendRows(self);
    // return 1;
  }
}

pub trait QTextTable_appendRows<RetType> {
  fn appendRows(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  void QTextTable::appendRows(int count);
impl<'a> /*trait*/ QTextTable_appendRows<()> for (i32) {
  fn appendRows(self , rsthis: &mut QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10appendRowsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTextTable10appendRowsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextTable::resize(int rows, int cols);
impl /*struct*/ QTextTable {
  pub fn resize<RetType, T: QTextTable_resize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QTextTable_resize<RetType> {
  fn resize(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  void QTextTable::resize(int rows, int cols);
impl<'a> /*trait*/ QTextTable_resize<()> for (i32, i32) {
  fn resize(self , rsthis: &mut QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable6resizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QTextTable6resizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QTextTableCell QTextTable::cellAt(const QTextCursor & c);
impl<'a> /*trait*/ QTextTable_cellAt<QTextTableCell> for (&'a  QTextCursor) {
  fn cellAt(self , rsthis: &mut QTextTable) -> QTextTableCell {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6cellAtERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTextTable6cellAtERK11QTextCursor(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextTableCell{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTextTableC1ERKS_(qthis, arg0)};
    let rsthis = QTextTable{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QTextTable::setFormat(const QTextTableFormat & format);
impl /*struct*/ QTextTable {
  pub fn setFormat<RetType, T: QTextTable_setFormat<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QTextTable_setFormat<RetType> {
  fn setFormat(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  void QTextTable::setFormat(const QTextTableFormat & format);
impl<'a> /*trait*/ QTextTable_setFormat<()> for (&'a  QTextTableFormat) {
  fn setFormat(self , rsthis: &mut QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable9setFormatERK16QTextTableFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTextTable9setFormatERK16QTextTableFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextTable::insertColumns(int pos, int num);
impl /*struct*/ QTextTable {
  pub fn insertColumns<RetType, T: QTextTable_insertColumns<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.insertColumns(self);
    // return 1;
  }
}

pub trait QTextTable_insertColumns<RetType> {
  fn insertColumns(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  void QTextTable::insertColumns(int pos, int num);
impl<'a> /*trait*/ QTextTable_insertColumns<()> for (i32, i32) {
  fn insertColumns(self , rsthis: &mut QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable13insertColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QTextTable13insertColumnsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QTextTable::splitCell(int row, int col, int numRows, int numCols);
impl /*struct*/ QTextTable {
  pub fn splitCell<RetType, T: QTextTable_splitCell<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.splitCell(self);
    // return 1;
  }
}

pub trait QTextTable_splitCell<RetType> {
  fn splitCell(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  void QTextTable::splitCell(int row, int col, int numRows, int numCols);
impl<'a> /*trait*/ QTextTable_splitCell<()> for (i32, i32, i32, i32) {
  fn splitCell(self , rsthis: &mut QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable9splitCellEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN10QTextTable9splitCellEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QTextTable::mergeCells(int row, int col, int numRows, int numCols);
impl /*struct*/ QTextTable {
  pub fn mergeCells<RetType, T: QTextTable_mergeCells<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.mergeCells(self);
    // return 1;
  }
}

pub trait QTextTable_mergeCells<RetType> {
  fn mergeCells(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  void QTextTable::mergeCells(int row, int col, int numRows, int numCols);
impl<'a> /*trait*/ QTextTable_mergeCells<()> for (i32, i32, i32, i32) {
  fn mergeCells(self , rsthis: &mut QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10mergeCellsEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN10QTextTable10mergeCellsEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QTextTable::insertRows(int pos, int num);
impl /*struct*/ QTextTable {
  pub fn insertRows<RetType, T: QTextTable_insertRows<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.insertRows(self);
    // return 1;
  }
}

pub trait QTextTable_insertRows<RetType> {
  fn insertRows(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  void QTextTable::insertRows(int pos, int num);
impl<'a> /*trait*/ QTextTable_insertRows<()> for (i32, i32) {
  fn insertRows(self , rsthis: &mut QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10insertRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QTextTable10insertRowsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QTextTable::FreeQTextTable();
impl /*struct*/ QTextTable {
  pub fn FreeQTextTable<RetType, T: QTextTable_FreeQTextTable<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQTextTable(self);
    // return 1;
  }
}

pub trait QTextTable_FreeQTextTable<RetType> {
  fn FreeQTextTable(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  void QTextTable::FreeQTextTable();
impl<'a> /*trait*/ QTextTable_FreeQTextTable<()> for () {
  fn FreeQTextTable(self , rsthis: &mut QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTableD0Ev()};
     unsafe {_ZN10QTextTableD0Ev(rsthis.qclsinst)};
    // return 1;
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

// proto:  QTextTableCell QTextTable::cellAt(int position);
impl<'a> /*trait*/ QTextTable_cellAt<QTextTableCell> for (i32) {
  fn cellAt(self , rsthis: &mut QTextTable) -> QTextTableCell {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6cellAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTextTable6cellAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextTableCell{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QTextCursor QTextTable::rowStart(const QTextCursor & c);
impl /*struct*/ QTextTable {
  pub fn rowStart<RetType, T: QTextTable_rowStart<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rowStart(self);
    // return 1;
  }
}

pub trait QTextTable_rowStart<RetType> {
  fn rowStart(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  QTextCursor QTextTable::rowStart(const QTextCursor & c);
impl<'a> /*trait*/ QTextTable_rowStart<QTextCursor> for (&'a  QTextCursor) {
  fn rowStart(self , rsthis: &mut QTextTable) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable8rowStartERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTextTable8rowStartERK11QTextCursor(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QTextTableFormat QTextTable::format();
impl /*struct*/ QTextTable {
  pub fn format<RetType, T: QTextTable_format<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QTextTable_format<RetType> {
  fn format(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  QTextTableFormat QTextTable::format();
impl<'a> /*trait*/ QTextTable_format<QTextTableFormat> for () {
  fn format(self , rsthis: &mut QTextTable) -> QTextTableFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6formatEv()};
    let mut ret = unsafe {_ZNK10QTextTable6formatEv(rsthis.qclsinst)};
    let mut ret1 = QTextTableFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QTextCursor QTextTable::rowEnd(const QTextCursor & c);
impl /*struct*/ QTextTable {
  pub fn rowEnd<RetType, T: QTextTable_rowEnd<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rowEnd(self);
    // return 1;
  }
}

pub trait QTextTable_rowEnd<RetType> {
  fn rowEnd(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  QTextCursor QTextTable::rowEnd(const QTextCursor & c);
impl<'a> /*trait*/ QTextTable_rowEnd<QTextCursor> for (&'a  QTextCursor) {
  fn rowEnd(self , rsthis: &mut QTextTable) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6rowEndERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTextTable6rowEndERK11QTextCursor(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QMetaObject * QTextTable::metaObject();
impl /*struct*/ QTextTable {
  pub fn metaObject<RetType, T: QTextTable_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTextTable_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  const QMetaObject * QTextTable::metaObject();
impl<'a> /*trait*/ QTextTable_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable10metaObjectEv()};
     unsafe {_ZNK10QTextTable10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QTextTable::removeColumns(int pos, int num);
impl /*struct*/ QTextTable {
  pub fn removeColumns<RetType, T: QTextTable_removeColumns<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.removeColumns(self);
    // return 1;
  }
}

pub trait QTextTable_removeColumns<RetType> {
  fn removeColumns(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  void QTextTable::removeColumns(int pos, int num);
impl<'a> /*trait*/ QTextTable_removeColumns<()> for (i32, i32) {
  fn removeColumns(self , rsthis: &mut QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable13removeColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QTextTable13removeColumnsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QTextTable::appendColumns(int count);
impl /*struct*/ QTextTable {
  pub fn appendColumns<RetType, T: QTextTable_appendColumns<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.appendColumns(self);
    // return 1;
  }
}

pub trait QTextTable_appendColumns<RetType> {
  fn appendColumns(self , rsthis: &mut QTextTable) -> RetType;
}

// proto:  void QTextTable::appendColumns(int count);
impl<'a> /*trait*/ QTextTable_appendColumns<()> for (i32) {
  fn appendColumns(self , rsthis: &mut QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable13appendColumnsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTextTable13appendColumnsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextTable::mergeCells(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextTable_mergeCells<()> for (&'a  QTextCursor) {
  fn mergeCells(self , rsthis: &mut QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10mergeCellsERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTextTable10mergeCellsERK11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

