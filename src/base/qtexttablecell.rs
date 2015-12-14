// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextcharformat::QTextCharFormat;
use super::qtexttable::QTextTable;
use super::qtextcursor::QTextCursor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTextTableCell::NewQTextTableCell(const QTextTableCell & o);
  fn _ZN14QTextTableCellC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextTableCell::setFormat(const QTextCharFormat & format);
  fn _ZN14QTextTableCell9setFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTextTableCell::lastPosition();
  fn _ZNK14QTextTableCell12lastPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextTableCell::FreeQTextTableCell();
  fn _ZN14QTextTableCellD0Ev(qthis: *mut c_void) ;
  // proto:  int QTextTableCell::rowSpan();
  fn _ZNK14QTextTableCell7rowSpanEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextTableCell::firstPosition();
  fn _ZNK14QTextTableCell13firstPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextTableCell::NewQTextTableCell(const QTextTable * t, int f);
  fn _ZN14QTextTableCellC1EPK10QTextTablei(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  int QTextTableCell::tableCellFormatIndex();
  fn _ZNK14QTextTableCell20tableCellFormatIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextTableCell::columnSpan();
  fn _ZNK14QTextTableCell10columnSpanEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextCharFormat QTextTableCell::format();
  fn _ZNK14QTextTableCell6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextTableCell::row();
  fn _ZNK14QTextTableCell3rowEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextTableCell::isValid();
  fn _ZNK14QTextTableCell7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QTextCursor QTextTableCell::lastCursorPosition();
  fn _ZNK14QTextTableCell18lastCursorPositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextTableCell::column();
  fn _ZNK14QTextTableCell6columnEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextCursor QTextTableCell::firstCursorPosition();
  fn _ZNK14QTextTableCell19firstCursorPositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextTableCell::NewQTextTableCell();
  fn _ZN14QTextTableCellC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QTextTableCell)=16
pub struct QTextTableCell {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextTableCell {
  pub fn NewQTextTableCell<T: QTextTableCell_NewQTextTableCell>(value: T) -> QTextTableCell {
    let rsthis = value.NewQTextTableCell();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableCell_NewQTextTableCell {
  fn NewQTextTableCell(self) -> QTextTableCell;
}

// proto: void QTextTableCell::NewQTextTableCell(const QTextTableCell & o);
impl<'a> /*trait*/ QTextTableCell_NewQTextTableCell for (&'a  QTextTableCell) {
  fn NewQTextTableCell(self) -> QTextTableCell {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCellC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QTextTableCellC1ERKS_(qthis, arg0)};
    let rsthis = QTextTableCell{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn setFormat<T: QTextTableCell_setFormat>(&mut self, value: T)  {
     value.setFormat(self);
    // return 1;
  }
}

pub trait QTextTableCell_setFormat {
  fn setFormat(self, rsthis: &mut QTextTableCell) ;
}

// proto:  void QTextTableCell::setFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextTableCell_setFormat for (&'a  QTextCharFormat) {
  fn setFormat(self, rsthis: &mut QTextTableCell)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCell9setFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QTextTableCell9setFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn lastPosition<T: QTextTableCell_lastPosition>(&mut self, value: T) -> i32 {
    return value.lastPosition(self);
    // return 1;
  }
}

pub trait QTextTableCell_lastPosition {
  fn lastPosition(self, rsthis: &mut QTextTableCell) -> i32;
}

// proto:  int QTextTableCell::lastPosition();
impl<'a> /*trait*/ QTextTableCell_lastPosition for () {
  fn lastPosition(self, rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell12lastPositionEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell12lastPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn FreeQTextTableCell<T: QTextTableCell_FreeQTextTableCell>(&mut self, value: T)  {
     value.FreeQTextTableCell(self);
    // return 1;
  }
}

pub trait QTextTableCell_FreeQTextTableCell {
  fn FreeQTextTableCell(self, rsthis: &mut QTextTableCell) ;
}

// proto:  void QTextTableCell::FreeQTextTableCell();
impl<'a> /*trait*/ QTextTableCell_FreeQTextTableCell for () {
  fn FreeQTextTableCell(self, rsthis: &mut QTextTableCell)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCellD0Ev()};
     unsafe {_ZN14QTextTableCellD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn rowSpan<T: QTextTableCell_rowSpan>(&mut self, value: T) -> i32 {
    return value.rowSpan(self);
    // return 1;
  }
}

pub trait QTextTableCell_rowSpan {
  fn rowSpan(self, rsthis: &mut QTextTableCell) -> i32;
}

// proto:  int QTextTableCell::rowSpan();
impl<'a> /*trait*/ QTextTableCell_rowSpan for () {
  fn rowSpan(self, rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell7rowSpanEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell7rowSpanEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn firstPosition<T: QTextTableCell_firstPosition>(&mut self, value: T) -> i32 {
    return value.firstPosition(self);
    // return 1;
  }
}

pub trait QTextTableCell_firstPosition {
  fn firstPosition(self, rsthis: &mut QTextTableCell) -> i32;
}

// proto:  int QTextTableCell::firstPosition();
impl<'a> /*trait*/ QTextTableCell_firstPosition for () {
  fn firstPosition(self, rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell13firstPositionEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell13firstPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QTextTableCell::NewQTextTableCell(const QTextTable * t, int f);
impl<'a> /*trait*/ QTextTableCell_NewQTextTableCell for (&'a  QTextTable, i32) {
  fn NewQTextTableCell(self) -> QTextTableCell {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCellC1EPK10QTextTablei()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN14QTextTableCellC1EPK10QTextTablei(qthis, arg0, arg1)};
    let rsthis = QTextTableCell{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn tableCellFormatIndex<T: QTextTableCell_tableCellFormatIndex>(&mut self, value: T) -> i32 {
    return value.tableCellFormatIndex(self);
    // return 1;
  }
}

pub trait QTextTableCell_tableCellFormatIndex {
  fn tableCellFormatIndex(self, rsthis: &mut QTextTableCell) -> i32;
}

// proto:  int QTextTableCell::tableCellFormatIndex();
impl<'a> /*trait*/ QTextTableCell_tableCellFormatIndex for () {
  fn tableCellFormatIndex(self, rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell20tableCellFormatIndexEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell20tableCellFormatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn columnSpan<T: QTextTableCell_columnSpan>(&mut self, value: T) -> i32 {
    return value.columnSpan(self);
    // return 1;
  }
}

pub trait QTextTableCell_columnSpan {
  fn columnSpan(self, rsthis: &mut QTextTableCell) -> i32;
}

// proto:  int QTextTableCell::columnSpan();
impl<'a> /*trait*/ QTextTableCell_columnSpan for () {
  fn columnSpan(self, rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell10columnSpanEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell10columnSpanEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn format<T: QTextTableCell_format>(&mut self, value: T) -> QTextCharFormat {
    return value.format(self);
    // return 1;
  }
}

pub trait QTextTableCell_format {
  fn format(self, rsthis: &mut QTextTableCell) -> QTextCharFormat;
}

// proto:  QTextCharFormat QTextTableCell::format();
impl<'a> /*trait*/ QTextTableCell_format for () {
  fn format(self, rsthis: &mut QTextTableCell) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell6formatEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell6formatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn row<T: QTextTableCell_row>(&mut self, value: T) -> i32 {
    return value.row(self);
    // return 1;
  }
}

pub trait QTextTableCell_row {
  fn row(self, rsthis: &mut QTextTableCell) -> i32;
}

// proto:  int QTextTableCell::row();
impl<'a> /*trait*/ QTextTableCell_row for () {
  fn row(self, rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell3rowEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell3rowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn isValid<T: QTextTableCell_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextTableCell_isValid {
  fn isValid(self, rsthis: &mut QTextTableCell) -> i8;
}

// proto:  bool QTextTableCell::isValid();
impl<'a> /*trait*/ QTextTableCell_isValid for () {
  fn isValid(self, rsthis: &mut QTextTableCell) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell7isValidEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn lastCursorPosition<T: QTextTableCell_lastCursorPosition>(&mut self, value: T) -> QTextCursor {
    return value.lastCursorPosition(self);
    // return 1;
  }
}

pub trait QTextTableCell_lastCursorPosition {
  fn lastCursorPosition(self, rsthis: &mut QTextTableCell) -> QTextCursor;
}

// proto:  QTextCursor QTextTableCell::lastCursorPosition();
impl<'a> /*trait*/ QTextTableCell_lastCursorPosition for () {
  fn lastCursorPosition(self, rsthis: &mut QTextTableCell) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell18lastCursorPositionEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell18lastCursorPositionEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn column<T: QTextTableCell_column>(&mut self, value: T) -> i32 {
    return value.column(self);
    // return 1;
  }
}

pub trait QTextTableCell_column {
  fn column(self, rsthis: &mut QTextTableCell) -> i32;
}

// proto:  int QTextTableCell::column();
impl<'a> /*trait*/ QTextTableCell_column for () {
  fn column(self, rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell6columnEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell6columnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn firstCursorPosition<T: QTextTableCell_firstCursorPosition>(&mut self, value: T) -> QTextCursor {
    return value.firstCursorPosition(self);
    // return 1;
  }
}

pub trait QTextTableCell_firstCursorPosition {
  fn firstCursorPosition(self, rsthis: &mut QTextTableCell) -> QTextCursor;
}

// proto:  QTextCursor QTextTableCell::firstCursorPosition();
impl<'a> /*trait*/ QTextTableCell_firstCursorPosition for () {
  fn firstCursorPosition(self, rsthis: &mut QTextTableCell) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell19firstCursorPositionEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell19firstCursorPositionEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTextTableCell::NewQTextTableCell();
impl<'a> /*trait*/ QTextTableCell_NewQTextTableCell for () {
  fn NewQTextTableCell(self) -> QTextTableCell {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCellC1Ev()};
    unsafe {_ZN14QTextTableCellC1Ev(qthis)};
    let rsthis = QTextTableCell{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

