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
  // proto:  void QTextTableFormat::QTextTableFormat();
  fn _ZN16QTextTableFormatC1Ev(qthis: *mut c_void);
  // proto:  bool QTextTableFormat::isValid();
  fn _ZNK16QTextTableFormat7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  int QTextTableFormat::headerRowCount();
  fn _ZNK16QTextTableFormat14headerRowCountEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTextTableFormat::columns();
  fn _ZNK16QTextTableFormat7columnsEv(qthis: *mut c_void) -> c_int;
  // proto:  QVector<QTextLength> QTextTableFormat::columnWidthConstraints();
  fn _ZNK16QTextTableFormat22columnWidthConstraintsEv(qthis: *mut c_void);
  // proto:  void QTextTableFormat::setCellPadding(qreal padding);
  fn _ZN16QTextTableFormat14setCellPaddingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextTableFormat::cellPadding();
  fn _ZNK16QTextTableFormat11cellPaddingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextTableFormat::setCellSpacing(qreal spacing);
  fn _ZN16QTextTableFormat14setCellSpacingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QTextTableFormat::setColumns(int columns);
  fn _ZN16QTextTableFormat10setColumnsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTextTableFormat::QTextTableFormat(const QTextFormat & fmt);
  fn _ZN16QTextTableFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextTableFormat::clearColumnWidthConstraints();
  fn _ZN16QTextTableFormat27clearColumnWidthConstraintsEv(qthis: *mut c_void);
  // proto:  void QTextTableFormat::setHeaderRowCount(int count);
  fn _ZN16QTextTableFormat17setHeaderRowCountEi(qthis: *mut c_void, arg0: c_int);
  // proto:  qreal QTextTableFormat::cellSpacing();
  fn _ZNK16QTextTableFormat11cellSpacingEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QTextTableFormat)=1
pub struct QTextTableFormat {
  pub qclsinst: *mut c_void,
}

  // proto:  void QTextTableFormat::QTextTableFormat();
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

  // proto:  void QTextTableFormat::QTextTableFormat();
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

  // proto:  bool QTextTableFormat::isValid();
impl /*struct*/ QTextTableFormat {
  pub fn isValid<RetType, T: QTextTableFormat_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextTableFormat_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  bool QTextTableFormat::isValid();
impl<'a> /*trait*/ QTextTableFormat_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTextTableFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat7isValidEv()};
    let mut ret = unsafe {_ZNK16QTextTableFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTextTableFormat::headerRowCount();
impl /*struct*/ QTextTableFormat {
  pub fn headerRowCount<RetType, T: QTextTableFormat_headerRowCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.headerRowCount(self);
    // return 1;
  }
}

pub trait QTextTableFormat_headerRowCount<RetType> {
  fn headerRowCount(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  int QTextTableFormat::headerRowCount();
impl<'a> /*trait*/ QTextTableFormat_headerRowCount<i32> for () {
  fn headerRowCount(self , rsthis: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat14headerRowCountEv()};
    let mut ret = unsafe {_ZNK16QTextTableFormat14headerRowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTextTableFormat::columns();
impl /*struct*/ QTextTableFormat {
  pub fn columns<RetType, T: QTextTableFormat_columns<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columns(self);
    // return 1;
  }
}

pub trait QTextTableFormat_columns<RetType> {
  fn columns(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  int QTextTableFormat::columns();
impl<'a> /*trait*/ QTextTableFormat_columns<i32> for () {
  fn columns(self , rsthis: &mut QTextTableFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat7columnsEv()};
    let mut ret = unsafe {_ZNK16QTextTableFormat7columnsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QVector<QTextLength> QTextTableFormat::columnWidthConstraints();
impl /*struct*/ QTextTableFormat {
  pub fn columnWidthConstraints<RetType, T: QTextTableFormat_columnWidthConstraints<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnWidthConstraints(self);
    // return 1;
  }
}

pub trait QTextTableFormat_columnWidthConstraints<RetType> {
  fn columnWidthConstraints(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  QVector<QTextLength> QTextTableFormat::columnWidthConstraints();
impl<'a> /*trait*/ QTextTableFormat_columnWidthConstraints<()> for () {
  fn columnWidthConstraints(self , rsthis: &mut QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat22columnWidthConstraintsEv()};
     unsafe {_ZNK16QTextTableFormat22columnWidthConstraintsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextTableFormat::setCellPadding(qreal padding);
impl /*struct*/ QTextTableFormat {
  pub fn setCellPadding<RetType, T: QTextTableFormat_setCellPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCellPadding(self);
    // return 1;
  }
}

pub trait QTextTableFormat_setCellPadding<RetType> {
  fn setCellPadding(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  void QTextTableFormat::setCellPadding(qreal padding);
impl<'a> /*trait*/ QTextTableFormat_setCellPadding<()> for (f64) {
  fn setCellPadding(self , rsthis: &mut QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat14setCellPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextTableFormat14setCellPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextTableFormat::cellPadding();
impl /*struct*/ QTextTableFormat {
  pub fn cellPadding<RetType, T: QTextTableFormat_cellPadding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cellPadding(self);
    // return 1;
  }
}

pub trait QTextTableFormat_cellPadding<RetType> {
  fn cellPadding(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  qreal QTextTableFormat::cellPadding();
impl<'a> /*trait*/ QTextTableFormat_cellPadding<f64> for () {
  fn cellPadding(self , rsthis: &mut QTextTableFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat11cellPaddingEv()};
    let mut ret = unsafe {_ZNK16QTextTableFormat11cellPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextTableFormat::setCellSpacing(qreal spacing);
impl /*struct*/ QTextTableFormat {
  pub fn setCellSpacing<RetType, T: QTextTableFormat_setCellSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCellSpacing(self);
    // return 1;
  }
}

pub trait QTextTableFormat_setCellSpacing<RetType> {
  fn setCellSpacing(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  void QTextTableFormat::setCellSpacing(qreal spacing);
impl<'a> /*trait*/ QTextTableFormat_setCellSpacing<()> for (f64) {
  fn setCellSpacing(self , rsthis: &mut QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat14setCellSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextTableFormat14setCellSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextTableFormat::setColumns(int columns);
impl /*struct*/ QTextTableFormat {
  pub fn setColumns<RetType, T: QTextTableFormat_setColumns<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColumns(self);
    // return 1;
  }
}

pub trait QTextTableFormat_setColumns<RetType> {
  fn setColumns(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  void QTextTableFormat::setColumns(int columns);
impl<'a> /*trait*/ QTextTableFormat_setColumns<()> for (i32) {
  fn setColumns(self , rsthis: &mut QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat10setColumnsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QTextTableFormat10setColumnsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextTableFormat::QTextTableFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextTableFormat_NewQTextTableFormat for (QTextFormat) {
  fn NewQTextTableFormat(self) -> QTextTableFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QTextTableFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextTableFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextTableFormat::clearColumnWidthConstraints();
impl /*struct*/ QTextTableFormat {
  pub fn clearColumnWidthConstraints<RetType, T: QTextTableFormat_clearColumnWidthConstraints<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearColumnWidthConstraints(self);
    // return 1;
  }
}

pub trait QTextTableFormat_clearColumnWidthConstraints<RetType> {
  fn clearColumnWidthConstraints(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  void QTextTableFormat::clearColumnWidthConstraints();
impl<'a> /*trait*/ QTextTableFormat_clearColumnWidthConstraints<()> for () {
  fn clearColumnWidthConstraints(self , rsthis: &mut QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat27clearColumnWidthConstraintsEv()};
     unsafe {_ZN16QTextTableFormat27clearColumnWidthConstraintsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextTableFormat::setHeaderRowCount(int count);
impl /*struct*/ QTextTableFormat {
  pub fn setHeaderRowCount<RetType, T: QTextTableFormat_setHeaderRowCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeaderRowCount(self);
    // return 1;
  }
}

pub trait QTextTableFormat_setHeaderRowCount<RetType> {
  fn setHeaderRowCount(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  void QTextTableFormat::setHeaderRowCount(int count);
impl<'a> /*trait*/ QTextTableFormat_setHeaderRowCount<()> for (i32) {
  fn setHeaderRowCount(self , rsthis: &mut QTextTableFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextTableFormat17setHeaderRowCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QTextTableFormat17setHeaderRowCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextTableFormat::cellSpacing();
impl /*struct*/ QTextTableFormat {
  pub fn cellSpacing<RetType, T: QTextTableFormat_cellSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cellSpacing(self);
    // return 1;
  }
}

pub trait QTextTableFormat_cellSpacing<RetType> {
  fn cellSpacing(self , rsthis: &mut QTextTableFormat) -> RetType;
}

  // proto:  qreal QTextTableFormat::cellSpacing();
impl<'a> /*trait*/ QTextTableFormat_cellSpacing<f64> for () {
  fn cellSpacing(self , rsthis: &mut QTextTableFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextTableFormat11cellSpacingEv()};
    let mut ret = unsafe {_ZNK16QTextTableFormat11cellSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

