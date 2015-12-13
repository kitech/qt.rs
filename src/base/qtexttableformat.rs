// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextformat::QTextFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTextTableFormat::NewQTextTableFormat();
  fn _ZN16QTextTableFormatC1Ev(qthis: *mut c_void) -> i32;
  // proto: bool QTextTableFormat::isValid();
  fn _ZNK16QTextTableFormat7isValidEv() -> i32;
  // proto: int QTextTableFormat::headerRowCount();
  fn _ZNK16QTextTableFormat14headerRowCountEv() -> i32;
  // proto: int QTextTableFormat::columns();
  fn _ZNK16QTextTableFormat7columnsEv() -> i32;
  // proto: QVector<QTextLength> QTextTableFormat::columnWidthConstraints();
  fn _ZNK16QTextTableFormat22columnWidthConstraintsEv() -> i32;
  // proto: void QTextTableFormat::setCellPadding(qreal padding);
  fn _ZN16QTextTableFormat14setCellPaddingEd(arg0: c_double) -> i32;
  // proto: double QTextTableFormat::cellPadding();
  fn _ZNK16QTextTableFormat11cellPaddingEv() -> i32;
  // proto: void QTextTableFormat::setCellSpacing(qreal spacing);
  fn _ZN16QTextTableFormat14setCellSpacingEd(arg0: c_double) -> i32;
  // proto: void QTextTableFormat::setColumns(int columns);
  fn _ZN16QTextTableFormat10setColumnsEi(arg0: c_int) -> i32;
  // proto: void QTextTableFormat::NewQTextTableFormat(const QTextFormat & fmt);
  fn _ZN16QTextTableFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QTextTableFormat::clearColumnWidthConstraints();
  fn _ZN16QTextTableFormat27clearColumnWidthConstraintsEv() -> i32;
  // proto: void QTextTableFormat::setHeaderRowCount(int count);
  fn _ZN16QTextTableFormat17setHeaderRowCountEi(arg0: c_int) -> i32;
  // proto: double QTextTableFormat::cellSpacing();
  fn _ZNK16QTextTableFormat11cellSpacingEv() -> i32;
}

// body block begin
// class sizeof(QTextTableFormat)=1
pub struct QTextTableFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextTableFormat {
  pub fn NewQTextTableFormat<T: QTextTableFormat_NewQTextTableFormat>(value: T) -> QTextTableFormat {
    let rsthis = value.NewQTextTableFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableFormat_NewQTextTableFormat {
  fn NewQTextTableFormat(self) -> QTextTableFormat;
}

// proto: void QTextTableFormat::NewQTextTableFormat();
impl<'a> /*trait*/ QTextTableFormat_NewQTextTableFormat for () {
  fn NewQTextTableFormat(self) -> QTextTableFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormatC1Ev()};
    unsafe {_ZN16QTextTableFormatC1Ev(qthis)};
    let rsthis = QTextTableFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextTableFormat {
  pub fn isValid<T: QTextTableFormat_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextTableFormat_isValid {
  fn isValid(self, this: &mut QTextTableFormat) -> i32;
}

// proto: bool QTextTableFormat::isValid();
impl<'a> /*trait*/ QTextTableFormat_isValid for () {
  fn isValid(self, this: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat7isValidEv()};
    unsafe {_ZNK16QTextTableFormat7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableFormat {
  pub fn headerRowCount<T: QTextTableFormat_headerRowCount>(&mut self, value: T) -> i32 {
    value.headerRowCount(self);
    return 1;
  }
}

pub trait QTextTableFormat_headerRowCount {
  fn headerRowCount(self, this: &mut QTextTableFormat) -> i32;
}

// proto: int QTextTableFormat::headerRowCount();
impl<'a> /*trait*/ QTextTableFormat_headerRowCount for () {
  fn headerRowCount(self, this: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat14headerRowCountEv()};
    unsafe {_ZNK16QTextTableFormat14headerRowCountEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableFormat {
  pub fn columns<T: QTextTableFormat_columns>(&mut self, value: T) -> i32 {
    value.columns(self);
    return 1;
  }
}

pub trait QTextTableFormat_columns {
  fn columns(self, this: &mut QTextTableFormat) -> i32;
}

// proto: int QTextTableFormat::columns();
impl<'a> /*trait*/ QTextTableFormat_columns for () {
  fn columns(self, this: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat7columnsEv()};
    unsafe {_ZNK16QTextTableFormat7columnsEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableFormat {
  pub fn columnWidthConstraints<T: QTextTableFormat_columnWidthConstraints>(&mut self, value: T) -> i32 {
    value.columnWidthConstraints(self);
    return 1;
  }
}

pub trait QTextTableFormat_columnWidthConstraints {
  fn columnWidthConstraints(self, this: &mut QTextTableFormat) -> i32;
}

// proto: QVector<QTextLength> QTextTableFormat::columnWidthConstraints();
impl<'a> /*trait*/ QTextTableFormat_columnWidthConstraints for () {
  fn columnWidthConstraints(self, this: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat22columnWidthConstraintsEv()};
    unsafe {_ZNK16QTextTableFormat22columnWidthConstraintsEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableFormat {
  pub fn setCellPadding<T: QTextTableFormat_setCellPadding>(&mut self, value: T) -> i32 {
    value.setCellPadding(self);
    return 1;
  }
}

pub trait QTextTableFormat_setCellPadding {
  fn setCellPadding(self, this: &mut QTextTableFormat) -> i32;
}

// proto: void QTextTableFormat::setCellPadding(qreal padding);
impl<'a> /*trait*/ QTextTableFormat_setCellPadding for (f64) {
  fn setCellPadding(self, this: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat14setCellPaddingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextTableFormat14setCellPaddingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTableFormat {
  pub fn cellPadding<T: QTextTableFormat_cellPadding>(&mut self, value: T) -> i32 {
    value.cellPadding(self);
    return 1;
  }
}

pub trait QTextTableFormat_cellPadding {
  fn cellPadding(self, this: &mut QTextTableFormat) -> i32;
}

// proto: double QTextTableFormat::cellPadding();
impl<'a> /*trait*/ QTextTableFormat_cellPadding for () {
  fn cellPadding(self, this: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat11cellPaddingEv()};
    unsafe {_ZNK16QTextTableFormat11cellPaddingEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableFormat {
  pub fn setCellSpacing<T: QTextTableFormat_setCellSpacing>(&mut self, value: T) -> i32 {
    value.setCellSpacing(self);
    return 1;
  }
}

pub trait QTextTableFormat_setCellSpacing {
  fn setCellSpacing(self, this: &mut QTextTableFormat) -> i32;
}

// proto: void QTextTableFormat::setCellSpacing(qreal spacing);
impl<'a> /*trait*/ QTextTableFormat_setCellSpacing for (f64) {
  fn setCellSpacing(self, this: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat14setCellSpacingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextTableFormat14setCellSpacingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTableFormat {
  pub fn setColumns<T: QTextTableFormat_setColumns>(&mut self, value: T) -> i32 {
    value.setColumns(self);
    return 1;
  }
}

pub trait QTextTableFormat_setColumns {
  fn setColumns(self, this: &mut QTextTableFormat) -> i32;
}

// proto: void QTextTableFormat::setColumns(int columns);
impl<'a> /*trait*/ QTextTableFormat_setColumns for (i32) {
  fn setColumns(self, this: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat10setColumnsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN16QTextTableFormat10setColumnsEi(arg0)};
    return 1;
  }
}

// proto: void QTextTableFormat::NewQTextTableFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextTableFormat_NewQTextTableFormat for (&'a  QTextFormat) {
  fn NewQTextTableFormat(self) -> QTextTableFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTextTableFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextTableFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextTableFormat {
  pub fn clearColumnWidthConstraints<T: QTextTableFormat_clearColumnWidthConstraints>(&mut self, value: T) -> i32 {
    value.clearColumnWidthConstraints(self);
    return 1;
  }
}

pub trait QTextTableFormat_clearColumnWidthConstraints {
  fn clearColumnWidthConstraints(self, this: &mut QTextTableFormat) -> i32;
}

// proto: void QTextTableFormat::clearColumnWidthConstraints();
impl<'a> /*trait*/ QTextTableFormat_clearColumnWidthConstraints for () {
  fn clearColumnWidthConstraints(self, this: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat27clearColumnWidthConstraintsEv()};
    unsafe {_ZN16QTextTableFormat27clearColumnWidthConstraintsEv()};
    return 1;
  }
}

impl /*struct*/ QTextTableFormat {
  pub fn setHeaderRowCount<T: QTextTableFormat_setHeaderRowCount>(&mut self, value: T) -> i32 {
    value.setHeaderRowCount(self);
    return 1;
  }
}

pub trait QTextTableFormat_setHeaderRowCount {
  fn setHeaderRowCount(self, this: &mut QTextTableFormat) -> i32;
}

// proto: void QTextTableFormat::setHeaderRowCount(int count);
impl<'a> /*trait*/ QTextTableFormat_setHeaderRowCount for (i32) {
  fn setHeaderRowCount(self, this: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat17setHeaderRowCountEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN16QTextTableFormat17setHeaderRowCountEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextTableFormat {
  pub fn cellSpacing<T: QTextTableFormat_cellSpacing>(&mut self, value: T) -> i32 {
    value.cellSpacing(self);
    return 1;
  }
}

pub trait QTextTableFormat_cellSpacing {
  fn cellSpacing(self, this: &mut QTextTableFormat) -> i32;
}

// proto: double QTextTableFormat::cellSpacing();
impl<'a> /*trait*/ QTextTableFormat_cellSpacing for () {
  fn cellSpacing(self, this: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat11cellSpacingEv()};
    unsafe {_ZNK16QTextTableFormat11cellSpacingEv()};
    return 1;
  }
}

