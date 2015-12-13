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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTextTableCell::NewQTextTableCell(const QTextTableCell & o);
  fn _ZN14QTextTableCellC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QTextTableCell::setFormat(const QTextCharFormat & format);
  fn _ZN14QTextTableCell9setFormatERK15QTextCharFormat(arg0: *const c_void) -> i32;
  // proto: int QTextTableCell::lastPosition();
  fn _ZNK14QTextTableCell12lastPositionEv() -> i32;
  // proto: void QTextTableCell::FreeQTextTableCell();
  fn _ZN14QTextTableCellD0Ev() -> i32;
  // proto: int QTextTableCell::rowSpan();
  fn _ZNK14QTextTableCell7rowSpanEv() -> i32;
  // proto: int QTextTableCell::firstPosition();
  fn _ZNK14QTextTableCell13firstPositionEv() -> i32;
  // proto: void QTextTableCell::NewQTextTableCell(const QTextTable * t, int f);
  fn _ZN14QTextTableCellC1EPK10QTextTablei(qthis: *mut c_void, arg0: *const c_void, arg1: c_int) -> i32;
  // proto: int QTextTableCell::tableCellFormatIndex();
  fn _ZNK14QTextTableCell20tableCellFormatIndexEv() -> i32;
  // proto: int QTextTableCell::columnSpan();
  fn _ZNK14QTextTableCell10columnSpanEv() -> i32;
  // proto: QTextCharFormat QTextTableCell::format();
  fn _ZNK14QTextTableCell6formatEv() -> i32;
  // proto: int QTextTableCell::row();
  fn _ZNK14QTextTableCell3rowEv() -> i32;
  // proto: bool QTextTableCell::isValid();
  fn _ZNK14QTextTableCell7isValidEv() -> i32;
  // proto: QTextCursor QTextTableCell::lastCursorPosition();
  fn _ZNK14QTextTableCell18lastCursorPositionEv() -> i32;
  // proto: int QTextTableCell::column();
  fn _ZNK14QTextTableCell6columnEv() -> i32;
  // proto: QTextCursor QTextTableCell::firstCursorPosition();
  fn _ZNK14QTextTableCell19firstCursorPositionEv() -> i32;
  // proto: void QTextTableCell::NewQTextTableCell();
  fn _ZN14QTextTableCellC1Ev(qthis: *mut c_void) -> i32;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QTextTableCellC1ERKS_(qthis, arg0)};
    let rsthis = QTextTableCell{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn setFormat<T: QTextTableCell_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QTextTableCell_setFormat {
  fn setFormat(self, this: &mut QTextTableCell) -> i32;
}

// proto: void QTextTableCell::setFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextTableCell_setFormat for (&'a  QTextCharFormat) {
  fn setFormat(self, this: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCell9setFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QTextTableCell9setFormatERK15QTextCharFormat(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn lastPosition<T: QTextTableCell_lastPosition>(&mut self, value: T) -> i32 {
    value.lastPosition(self);
    return 1;
  }
}

pub trait QTextTableCell_lastPosition {
  fn lastPosition(self, this: &mut QTextTableCell) -> i32;
}

// proto: int QTextTableCell::lastPosition();
impl<'a> /*trait*/ QTextTableCell_lastPosition for () {
  fn lastPosition(self, this: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell12lastPositionEv()};
    unsafe {_ZNK14QTextTableCell12lastPositionEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn FreeQTextTableCell<T: QTextTableCell_FreeQTextTableCell>(&mut self, value: T) -> i32 {
    value.FreeQTextTableCell(self);
    return 1;
  }
}

pub trait QTextTableCell_FreeQTextTableCell {
  fn FreeQTextTableCell(self, this: &mut QTextTableCell) -> i32;
}

// proto: void QTextTableCell::FreeQTextTableCell();
impl<'a> /*trait*/ QTextTableCell_FreeQTextTableCell for () {
  fn FreeQTextTableCell(self, this: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCellD0Ev()};
    unsafe {_ZN14QTextTableCellD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn rowSpan<T: QTextTableCell_rowSpan>(&mut self, value: T) -> i32 {
    value.rowSpan(self);
    return 1;
  }
}

pub trait QTextTableCell_rowSpan {
  fn rowSpan(self, this: &mut QTextTableCell) -> i32;
}

// proto: int QTextTableCell::rowSpan();
impl<'a> /*trait*/ QTextTableCell_rowSpan for () {
  fn rowSpan(self, this: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell7rowSpanEv()};
    unsafe {_ZNK14QTextTableCell7rowSpanEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn firstPosition<T: QTextTableCell_firstPosition>(&mut self, value: T) -> i32 {
    value.firstPosition(self);
    return 1;
  }
}

pub trait QTextTableCell_firstPosition {
  fn firstPosition(self, this: &mut QTextTableCell) -> i32;
}

// proto: int QTextTableCell::firstPosition();
impl<'a> /*trait*/ QTextTableCell_firstPosition for () {
  fn firstPosition(self, this: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell13firstPositionEv()};
    unsafe {_ZNK14QTextTableCell13firstPositionEv()};
    return 1;
  }
}

// proto: void QTextTableCell::NewQTextTableCell(const QTextTable * t, int f);
impl<'a> /*trait*/ QTextTableCell_NewQTextTableCell for (&'a  QTextTable, i32) {
  fn NewQTextTableCell(self) -> QTextTableCell {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCellC1EPK10QTextTablei()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN14QTextTableCellC1EPK10QTextTablei(qthis, arg0, arg1)};
    let rsthis = QTextTableCell{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn tableCellFormatIndex<T: QTextTableCell_tableCellFormatIndex>(&mut self, value: T) -> i32 {
    value.tableCellFormatIndex(self);
    return 1;
  }
}

pub trait QTextTableCell_tableCellFormatIndex {
  fn tableCellFormatIndex(self, this: &mut QTextTableCell) -> i32;
}

// proto: int QTextTableCell::tableCellFormatIndex();
impl<'a> /*trait*/ QTextTableCell_tableCellFormatIndex for () {
  fn tableCellFormatIndex(self, this: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell20tableCellFormatIndexEv()};
    unsafe {_ZNK14QTextTableCell20tableCellFormatIndexEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn columnSpan<T: QTextTableCell_columnSpan>(&mut self, value: T) -> i32 {
    value.columnSpan(self);
    return 1;
  }
}

pub trait QTextTableCell_columnSpan {
  fn columnSpan(self, this: &mut QTextTableCell) -> i32;
}

// proto: int QTextTableCell::columnSpan();
impl<'a> /*trait*/ QTextTableCell_columnSpan for () {
  fn columnSpan(self, this: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell10columnSpanEv()};
    unsafe {_ZNK14QTextTableCell10columnSpanEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn format<T: QTextTableCell_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QTextTableCell_format {
  fn format(self, this: &mut QTextTableCell) -> i32;
}

// proto: QTextCharFormat QTextTableCell::format();
impl<'a> /*trait*/ QTextTableCell_format for () {
  fn format(self, this: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell6formatEv()};
    unsafe {_ZNK14QTextTableCell6formatEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn row<T: QTextTableCell_row>(&mut self, value: T) -> i32 {
    value.row(self);
    return 1;
  }
}

pub trait QTextTableCell_row {
  fn row(self, this: &mut QTextTableCell) -> i32;
}

// proto: int QTextTableCell::row();
impl<'a> /*trait*/ QTextTableCell_row for () {
  fn row(self, this: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell3rowEv()};
    unsafe {_ZNK14QTextTableCell3rowEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn isValid<T: QTextTableCell_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextTableCell_isValid {
  fn isValid(self, this: &mut QTextTableCell) -> i32;
}

// proto: bool QTextTableCell::isValid();
impl<'a> /*trait*/ QTextTableCell_isValid for () {
  fn isValid(self, this: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell7isValidEv()};
    unsafe {_ZNK14QTextTableCell7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn lastCursorPosition<T: QTextTableCell_lastCursorPosition>(&mut self, value: T) -> i32 {
    value.lastCursorPosition(self);
    return 1;
  }
}

pub trait QTextTableCell_lastCursorPosition {
  fn lastCursorPosition(self, this: &mut QTextTableCell) -> i32;
}

// proto: QTextCursor QTextTableCell::lastCursorPosition();
impl<'a> /*trait*/ QTextTableCell_lastCursorPosition for () {
  fn lastCursorPosition(self, this: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell18lastCursorPositionEv()};
    unsafe {_ZNK14QTextTableCell18lastCursorPositionEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn column<T: QTextTableCell_column>(&mut self, value: T) -> i32 {
    value.column(self);
    return 1;
  }
}

pub trait QTextTableCell_column {
  fn column(self, this: &mut QTextTableCell) -> i32;
}

// proto: int QTextTableCell::column();
impl<'a> /*trait*/ QTextTableCell_column for () {
  fn column(self, this: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell6columnEv()};
    unsafe {_ZNK14QTextTableCell6columnEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableCell {
  pub fn firstCursorPosition<T: QTextTableCell_firstCursorPosition>(&mut self, value: T) -> i32 {
    value.firstCursorPosition(self);
    return 1;
  }
}

pub trait QTextTableCell_firstCursorPosition {
  fn firstCursorPosition(self, this: &mut QTextTableCell) -> i32;
}

// proto: QTextCursor QTextTableCell::firstCursorPosition();
impl<'a> /*trait*/ QTextTableCell_firstCursorPosition for () {
  fn firstCursorPosition(self, this: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell19firstCursorPositionEv()};
    unsafe {_ZNK14QTextTableCell19firstCursorPositionEv()};
    return 1;
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

