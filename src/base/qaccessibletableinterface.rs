// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qaccessibleinterface::QAccessibleInterface;
use super::qaccessibletablemodelchangeevent::QAccessibleTableModelChangeEvent;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QAccessibleTableInterface::unselectColumn(int column);
  fn _ZN25QAccessibleTableInterface14unselectColumnEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  QString QAccessibleTableInterface::columnDescription(int column);
  fn _ZNK25QAccessibleTableInterface17columnDescriptionEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QAccessibleTableInterface::selectedCellCount();
  fn _ZNK25QAccessibleTableInterface17selectedCellCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QList<QAccessibleInterface *> QAccessibleTableInterface::selectedCells();
  fn _ZNK25QAccessibleTableInterface13selectedCellsEv(qthis: *mut c_void) ;
  // proto:  bool QAccessibleTableInterface::selectRow(int row);
  fn _ZN25QAccessibleTableInterface9selectRowEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  int QAccessibleTableInterface::selectedRowCount();
  fn _ZNK25QAccessibleTableInterface16selectedRowCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTableInterface::FreeQAccessibleTableInterface();
  fn _ZN25QAccessibleTableInterfaceD0Ev(qthis: *mut c_void) ;
  // proto:  QList<int> QAccessibleTableInterface::selectedColumns();
  fn _ZNK25QAccessibleTableInterface15selectedColumnsEv(qthis: *mut c_void) ;
  // proto:  QAccessibleInterface * QAccessibleTableInterface::cellAt(int row, int column);
  fn _ZNK25QAccessibleTableInterface6cellAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QList<int> QAccessibleTableInterface::selectedRows();
  fn _ZNK25QAccessibleTableInterface12selectedRowsEv(qthis: *mut c_void) ;
  // proto:  void QAccessibleTableInterface::modelChange(QAccessibleTableModelChangeEvent * event);
  fn _ZN25QAccessibleTableInterface11modelChangeEP32QAccessibleTableModelChangeEvent(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QAccessibleTableInterface::columnCount();
  fn _ZNK25QAccessibleTableInterface11columnCountEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QAccessibleTableInterface::selectColumn(int column);
  fn _ZN25QAccessibleTableInterface12selectColumnEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  bool QAccessibleTableInterface::unselectRow(int row);
  fn _ZN25QAccessibleTableInterface11unselectRowEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  int QAccessibleTableInterface::rowCount();
  fn _ZNK25QAccessibleTableInterface8rowCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QAccessibleTableInterface::rowDescription(int row);
  fn _ZNK25QAccessibleTableInterface14rowDescriptionEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleTableInterface::summary();
  fn _ZNK25QAccessibleTableInterface7summaryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QAccessibleTableInterface::isColumnSelected(int column);
  fn _ZNK25QAccessibleTableInterface16isColumnSelectedEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  QAccessibleInterface * QAccessibleTableInterface::caption();
  fn _ZNK25QAccessibleTableInterface7captionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QAccessibleTableInterface::isRowSelected(int row);
  fn _ZNK25QAccessibleTableInterface13isRowSelectedEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  int QAccessibleTableInterface::selectedColumnCount();
  fn _ZNK25QAccessibleTableInterface19selectedColumnCountEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QAccessibleTableInterface)=8
pub struct QAccessibleTableInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn unselectColumn<T: QAccessibleTableInterface_unselectColumn>(&mut self, value: T) -> i8 {
    return value.unselectColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_unselectColumn {
  fn unselectColumn(self, rsthis: &mut QAccessibleTableInterface) -> i8;
}

// proto:  bool QAccessibleTableInterface::unselectColumn(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_unselectColumn for (i32) {
  fn unselectColumn(self, rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface14unselectColumnEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QAccessibleTableInterface14unselectColumnEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn columnDescription<T: QAccessibleTableInterface_columnDescription>(&mut self, value: T) -> QString {
    return value.columnDescription(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_columnDescription {
  fn columnDescription(self, rsthis: &mut QAccessibleTableInterface) -> QString;
}

// proto:  QString QAccessibleTableInterface::columnDescription(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_columnDescription for (i32) {
  fn columnDescription(self, rsthis: &mut QAccessibleTableInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface17columnDescriptionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface17columnDescriptionEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedCellCount<T: QAccessibleTableInterface_selectedCellCount>(&mut self, value: T) -> i32 {
    return value.selectedCellCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedCellCount {
  fn selectedCellCount(self, rsthis: &mut QAccessibleTableInterface) -> i32;
}

// proto:  int QAccessibleTableInterface::selectedCellCount();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedCellCount for () {
  fn selectedCellCount(self, rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface17selectedCellCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface17selectedCellCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedCells<T: QAccessibleTableInterface_selectedCells>(&mut self, value: T)  {
     value.selectedCells(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedCells {
  fn selectedCells(self, rsthis: &mut QAccessibleTableInterface) ;
}

// proto:  QList<QAccessibleInterface *> QAccessibleTableInterface::selectedCells();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedCells for () {
  fn selectedCells(self, rsthis: &mut QAccessibleTableInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface13selectedCellsEv()};
     unsafe {_ZNK25QAccessibleTableInterface13selectedCellsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectRow<T: QAccessibleTableInterface_selectRow>(&mut self, value: T) -> i8 {
    return value.selectRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectRow {
  fn selectRow(self, rsthis: &mut QAccessibleTableInterface) -> i8;
}

// proto:  bool QAccessibleTableInterface::selectRow(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_selectRow for (i32) {
  fn selectRow(self, rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface9selectRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QAccessibleTableInterface9selectRowEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedRowCount<T: QAccessibleTableInterface_selectedRowCount>(&mut self, value: T) -> i32 {
    return value.selectedRowCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedRowCount {
  fn selectedRowCount(self, rsthis: &mut QAccessibleTableInterface) -> i32;
}

// proto:  int QAccessibleTableInterface::selectedRowCount();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedRowCount for () {
  fn selectedRowCount(self, rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface16selectedRowCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface16selectedRowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn FreeQAccessibleTableInterface<T: QAccessibleTableInterface_FreeQAccessibleTableInterface>(&mut self, value: T)  {
     value.FreeQAccessibleTableInterface(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_FreeQAccessibleTableInterface {
  fn FreeQAccessibleTableInterface(self, rsthis: &mut QAccessibleTableInterface) ;
}

// proto:  void QAccessibleTableInterface::FreeQAccessibleTableInterface();
impl<'a> /*trait*/ QAccessibleTableInterface_FreeQAccessibleTableInterface for () {
  fn FreeQAccessibleTableInterface(self, rsthis: &mut QAccessibleTableInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterfaceD0Ev()};
     unsafe {_ZN25QAccessibleTableInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedColumns<T: QAccessibleTableInterface_selectedColumns>(&mut self, value: T)  {
     value.selectedColumns(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedColumns {
  fn selectedColumns(self, rsthis: &mut QAccessibleTableInterface) ;
}

// proto:  QList<int> QAccessibleTableInterface::selectedColumns();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedColumns for () {
  fn selectedColumns(self, rsthis: &mut QAccessibleTableInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface15selectedColumnsEv()};
     unsafe {_ZNK25QAccessibleTableInterface15selectedColumnsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn cellAt<T: QAccessibleTableInterface_cellAt>(&mut self, value: T) -> QAccessibleInterface {
    return value.cellAt(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_cellAt {
  fn cellAt(self, rsthis: &mut QAccessibleTableInterface) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleTableInterface::cellAt(int row, int column);
impl<'a> /*trait*/ QAccessibleTableInterface_cellAt for (i32, i32) {
  fn cellAt(self, rsthis: &mut QAccessibleTableInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface6cellAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface6cellAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedRows<T: QAccessibleTableInterface_selectedRows>(&mut self, value: T)  {
     value.selectedRows(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedRows {
  fn selectedRows(self, rsthis: &mut QAccessibleTableInterface) ;
}

// proto:  QList<int> QAccessibleTableInterface::selectedRows();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedRows for () {
  fn selectedRows(self, rsthis: &mut QAccessibleTableInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface12selectedRowsEv()};
     unsafe {_ZNK25QAccessibleTableInterface12selectedRowsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn modelChange<T: QAccessibleTableInterface_modelChange>(&mut self, value: T)  {
     value.modelChange(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_modelChange {
  fn modelChange(self, rsthis: &mut QAccessibleTableInterface) ;
}

// proto:  void QAccessibleTableInterface::modelChange(QAccessibleTableModelChangeEvent * event);
impl<'a> /*trait*/ QAccessibleTableInterface_modelChange for (&'a mut QAccessibleTableModelChangeEvent) {
  fn modelChange(self, rsthis: &mut QAccessibleTableInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface11modelChangeEP32QAccessibleTableModelChangeEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QAccessibleTableInterface11modelChangeEP32QAccessibleTableModelChangeEvent(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn columnCount<T: QAccessibleTableInterface_columnCount>(&mut self, value: T) -> i32 {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_columnCount {
  fn columnCount(self, rsthis: &mut QAccessibleTableInterface) -> i32;
}

// proto:  int QAccessibleTableInterface::columnCount();
impl<'a> /*trait*/ QAccessibleTableInterface_columnCount for () {
  fn columnCount(self, rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface11columnCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectColumn<T: QAccessibleTableInterface_selectColumn>(&mut self, value: T) -> i8 {
    return value.selectColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectColumn {
  fn selectColumn(self, rsthis: &mut QAccessibleTableInterface) -> i8;
}

// proto:  bool QAccessibleTableInterface::selectColumn(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_selectColumn for (i32) {
  fn selectColumn(self, rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface12selectColumnEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QAccessibleTableInterface12selectColumnEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn unselectRow<T: QAccessibleTableInterface_unselectRow>(&mut self, value: T) -> i8 {
    return value.unselectRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_unselectRow {
  fn unselectRow(self, rsthis: &mut QAccessibleTableInterface) -> i8;
}

// proto:  bool QAccessibleTableInterface::unselectRow(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_unselectRow for (i32) {
  fn unselectRow(self, rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface11unselectRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QAccessibleTableInterface11unselectRowEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn rowCount<T: QAccessibleTableInterface_rowCount>(&mut self, value: T) -> i32 {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_rowCount {
  fn rowCount(self, rsthis: &mut QAccessibleTableInterface) -> i32;
}

// proto:  int QAccessibleTableInterface::rowCount();
impl<'a> /*trait*/ QAccessibleTableInterface_rowCount for () {
  fn rowCount(self, rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface8rowCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn rowDescription<T: QAccessibleTableInterface_rowDescription>(&mut self, value: T) -> QString {
    return value.rowDescription(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_rowDescription {
  fn rowDescription(self, rsthis: &mut QAccessibleTableInterface) -> QString;
}

// proto:  QString QAccessibleTableInterface::rowDescription(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_rowDescription for (i32) {
  fn rowDescription(self, rsthis: &mut QAccessibleTableInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface14rowDescriptionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface14rowDescriptionEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn summary<T: QAccessibleTableInterface_summary>(&mut self, value: T) -> QAccessibleInterface {
    return value.summary(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_summary {
  fn summary(self, rsthis: &mut QAccessibleTableInterface) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleTableInterface::summary();
impl<'a> /*trait*/ QAccessibleTableInterface_summary for () {
  fn summary(self, rsthis: &mut QAccessibleTableInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface7summaryEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface7summaryEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn isColumnSelected<T: QAccessibleTableInterface_isColumnSelected>(&mut self, value: T) -> i8 {
    return value.isColumnSelected(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_isColumnSelected {
  fn isColumnSelected(self, rsthis: &mut QAccessibleTableInterface) -> i8;
}

// proto:  bool QAccessibleTableInterface::isColumnSelected(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_isColumnSelected for (i32) {
  fn isColumnSelected(self, rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface16isColumnSelectedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface16isColumnSelectedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn caption<T: QAccessibleTableInterface_caption>(&mut self, value: T) -> QAccessibleInterface {
    return value.caption(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_caption {
  fn caption(self, rsthis: &mut QAccessibleTableInterface) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleTableInterface::caption();
impl<'a> /*trait*/ QAccessibleTableInterface_caption for () {
  fn caption(self, rsthis: &mut QAccessibleTableInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface7captionEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface7captionEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn isRowSelected<T: QAccessibleTableInterface_isRowSelected>(&mut self, value: T) -> i8 {
    return value.isRowSelected(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_isRowSelected {
  fn isRowSelected(self, rsthis: &mut QAccessibleTableInterface) -> i8;
}

// proto:  bool QAccessibleTableInterface::isRowSelected(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_isRowSelected for (i32) {
  fn isRowSelected(self, rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface13isRowSelectedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface13isRowSelectedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedColumnCount<T: QAccessibleTableInterface_selectedColumnCount>(&mut self, value: T) -> i32 {
    return value.selectedColumnCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedColumnCount {
  fn selectedColumnCount(self, rsthis: &mut QAccessibleTableInterface) -> i32;
}

// proto:  int QAccessibleTableInterface::selectedColumnCount();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedColumnCount for () {
  fn selectedColumnCount(self, rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface19selectedColumnCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface19selectedColumnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

