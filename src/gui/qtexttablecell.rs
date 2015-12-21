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
  // proto:  void QTextTableCell::QTextTableCell(const QTextTableCell & o);
  fn _ZN14QTextTableCellC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextTableCell::setFormat(const QTextCharFormat & format);
  fn _ZN14QTextTableCell9setFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QTextTableCell::lastPosition();
  fn _ZNK14QTextTableCell12lastPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextTableCell::~QTextTableCell();
  fn _ZN14QTextTableCellD0Ev(qthis: *mut c_void);
  // proto:  int QTextTableCell::rowSpan();
  fn _ZNK14QTextTableCell7rowSpanEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextTableCell::firstPosition();
  fn _ZNK14QTextTableCell13firstPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextTableCell::QTextTableCell(const QTextTable * t, int f);
  fn _ZN14QTextTableCellC1EPK10QTextTablei(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  int QTextTableCell::tableCellFormatIndex();
  fn _ZNK14QTextTableCell20tableCellFormatIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextTableCell::columnSpan();
  fn _ZNK14QTextTableCell10columnSpanEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextCharFormat QTextTableCell::format();
  fn _ZNK14QTextTableCell6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextTableCell::row();
  fn _ZNK14QTextTableCell3rowEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextTableCell::isValid();
  fn _ZNK14QTextTableCell7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  QTextCursor QTextTableCell::lastCursorPosition();
  fn _ZNK14QTextTableCell18lastCursorPositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextTableCell::column();
  fn _ZNK14QTextTableCell6columnEv(qthis: *mut c_void) -> c_int;
  // proto:  QTextCursor QTextTableCell::firstCursorPosition();
  fn _ZNK14QTextTableCell19firstCursorPositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextTableCell::QTextTableCell();
  fn _ZN14QTextTableCellC1Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QTextTableCell)=16
pub struct QTextTableCell {
  pub qclsinst: *mut c_void,
}

  // proto:  void QTextTableCell::QTextTableCell(const QTextTableCell & o);
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

  // proto:  void QTextTableCell::QTextTableCell(const QTextTableCell & o);
impl<'a> /*trait*/ QTextTableCell_NewQTextTableCell for (QTextTableCell) {
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

  // proto:  void QTextTableCell::setFormat(const QTextCharFormat & format);
impl /*struct*/ QTextTableCell {
  pub fn setFormat<RetType, T: QTextTableCell_setFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QTextTableCell_setFormat<RetType> {
  fn setFormat(self , rsthis: &mut QTextTableCell) -> RetType;
}

  // proto:  void QTextTableCell::setFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextTableCell_setFormat<()> for (QTextCharFormat) {
  fn setFormat(self , rsthis: &mut QTextTableCell) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCell9setFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QTextTableCell9setFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextTableCell::lastPosition();
impl /*struct*/ QTextTableCell {
  pub fn lastPosition<RetType, T: QTextTableCell_lastPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lastPosition(self);
    // return 1;
  }
}

pub trait QTextTableCell_lastPosition<RetType> {
  fn lastPosition(self , rsthis: &mut QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::lastPosition();
impl<'a> /*trait*/ QTextTableCell_lastPosition<i32> for () {
  fn lastPosition(self , rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell12lastPositionEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell12lastPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextTableCell::~QTextTableCell();
impl /*struct*/ QTextTableCell {
  pub fn FreeQTextTableCell<RetType, T: QTextTableCell_FreeQTextTableCell<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTextTableCell(self);
    // return 1;
  }
}

pub trait QTextTableCell_FreeQTextTableCell<RetType> {
  fn FreeQTextTableCell(self , rsthis: &mut QTextTableCell) -> RetType;
}

  // proto:  void QTextTableCell::~QTextTableCell();
impl<'a> /*trait*/ QTextTableCell_FreeQTextTableCell<()> for () {
  fn FreeQTextTableCell(self , rsthis: &mut QTextTableCell) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCellD0Ev()};
     unsafe {_ZN14QTextTableCellD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTextTableCell::rowSpan();
impl /*struct*/ QTextTableCell {
  pub fn rowSpan<RetType, T: QTextTableCell_rowSpan<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowSpan(self);
    // return 1;
  }
}

pub trait QTextTableCell_rowSpan<RetType> {
  fn rowSpan(self , rsthis: &mut QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::rowSpan();
impl<'a> /*trait*/ QTextTableCell_rowSpan<i32> for () {
  fn rowSpan(self , rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell7rowSpanEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell7rowSpanEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTextTableCell::firstPosition();
impl /*struct*/ QTextTableCell {
  pub fn firstPosition<RetType, T: QTextTableCell_firstPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.firstPosition(self);
    // return 1;
  }
}

pub trait QTextTableCell_firstPosition<RetType> {
  fn firstPosition(self , rsthis: &mut QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::firstPosition();
impl<'a> /*trait*/ QTextTableCell_firstPosition<i32> for () {
  fn firstPosition(self , rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell13firstPositionEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell13firstPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextTableCell::QTextTableCell(const QTextTable * t, int f);
impl<'a> /*trait*/ QTextTableCell_NewQTextTableCell for (QTextTable, i32) {
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

  // proto:  int QTextTableCell::tableCellFormatIndex();
impl /*struct*/ QTextTableCell {
  pub fn tableCellFormatIndex<RetType, T: QTextTableCell_tableCellFormatIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tableCellFormatIndex(self);
    // return 1;
  }
}

pub trait QTextTableCell_tableCellFormatIndex<RetType> {
  fn tableCellFormatIndex(self , rsthis: &mut QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::tableCellFormatIndex();
impl<'a> /*trait*/ QTextTableCell_tableCellFormatIndex<i32> for () {
  fn tableCellFormatIndex(self , rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell20tableCellFormatIndexEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell20tableCellFormatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTextTableCell::columnSpan();
impl /*struct*/ QTextTableCell {
  pub fn columnSpan<RetType, T: QTextTableCell_columnSpan<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnSpan(self);
    // return 1;
  }
}

pub trait QTextTableCell_columnSpan<RetType> {
  fn columnSpan(self , rsthis: &mut QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::columnSpan();
impl<'a> /*trait*/ QTextTableCell_columnSpan<i32> for () {
  fn columnSpan(self , rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell10columnSpanEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell10columnSpanEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTextCharFormat QTextTableCell::format();
impl /*struct*/ QTextTableCell {
  pub fn format<RetType, T: QTextTableCell_format<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QTextTableCell_format<RetType> {
  fn format(self , rsthis: &mut QTextTableCell) -> RetType;
}

  // proto:  QTextCharFormat QTextTableCell::format();
impl<'a> /*trait*/ QTextTableCell_format<QTextCharFormat> for () {
  fn format(self , rsthis: &mut QTextTableCell) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell6formatEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell6formatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextTableCell::row();
impl /*struct*/ QTextTableCell {
  pub fn row<RetType, T: QTextTableCell_row<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QTextTableCell_row<RetType> {
  fn row(self , rsthis: &mut QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::row();
impl<'a> /*trait*/ QTextTableCell_row<i32> for () {
  fn row(self , rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell3rowEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell3rowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTextTableCell::isValid();
impl /*struct*/ QTextTableCell {
  pub fn isValid<RetType, T: QTextTableCell_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextTableCell_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTextTableCell) -> RetType;
}

  // proto:  bool QTextTableCell::isValid();
impl<'a> /*trait*/ QTextTableCell_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTextTableCell) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell7isValidEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTextCursor QTextTableCell::lastCursorPosition();
impl /*struct*/ QTextTableCell {
  pub fn lastCursorPosition<RetType, T: QTextTableCell_lastCursorPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lastCursorPosition(self);
    // return 1;
  }
}

pub trait QTextTableCell_lastCursorPosition<RetType> {
  fn lastCursorPosition(self , rsthis: &mut QTextTableCell) -> RetType;
}

  // proto:  QTextCursor QTextTableCell::lastCursorPosition();
impl<'a> /*trait*/ QTextTableCell_lastCursorPosition<QTextCursor> for () {
  fn lastCursorPosition(self , rsthis: &mut QTextTableCell) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell18lastCursorPositionEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell18lastCursorPositionEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextTableCell::column();
impl /*struct*/ QTextTableCell {
  pub fn column<RetType, T: QTextTableCell_column<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.column(self);
    // return 1;
  }
}

pub trait QTextTableCell_column<RetType> {
  fn column(self , rsthis: &mut QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::column();
impl<'a> /*trait*/ QTextTableCell_column<i32> for () {
  fn column(self , rsthis: &mut QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell6columnEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell6columnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTextCursor QTextTableCell::firstCursorPosition();
impl /*struct*/ QTextTableCell {
  pub fn firstCursorPosition<RetType, T: QTextTableCell_firstCursorPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.firstCursorPosition(self);
    // return 1;
  }
}

pub trait QTextTableCell_firstCursorPosition<RetType> {
  fn firstCursorPosition(self , rsthis: &mut QTextTableCell) -> RetType;
}

  // proto:  QTextCursor QTextTableCell::firstCursorPosition();
impl<'a> /*trait*/ QTextTableCell_firstCursorPosition<QTextCursor> for () {
  fn firstCursorPosition(self , rsthis: &mut QTextTableCell) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell19firstCursorPositionEv()};
    let mut ret = unsafe {_ZNK14QTextTableCell19firstCursorPositionEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextTableCell::QTextTableCell();
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

