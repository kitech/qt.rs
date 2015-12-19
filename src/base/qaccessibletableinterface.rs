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

// proto:  bool QAccessibleTableInterface::unselectColumn(int column);
impl /*struct*/ QAccessibleTableInterface {
  pub fn unselectColumn<RetType, T: QAccessibleTableInterface_unselectColumn<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.unselectColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_unselectColumn<RetType> {
  fn unselectColumn(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  bool QAccessibleTableInterface::unselectColumn(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_unselectColumn<i8> for (i32) {
  fn unselectColumn(self , rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface14unselectColumnEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QAccessibleTableInterface14unselectColumnEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QString QAccessibleTableInterface::columnDescription(int column);
impl /*struct*/ QAccessibleTableInterface {
  pub fn columnDescription<RetType, T: QAccessibleTableInterface_columnDescription<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.columnDescription(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_columnDescription<RetType> {
  fn columnDescription(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  QString QAccessibleTableInterface::columnDescription(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_columnDescription<QString> for (i32) {
  fn columnDescription(self , rsthis: &mut QAccessibleTableInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface17columnDescriptionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface17columnDescriptionEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QAccessibleTableInterface::selectedCellCount();
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedCellCount<RetType, T: QAccessibleTableInterface_selectedCellCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.selectedCellCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedCellCount<RetType> {
  fn selectedCellCount(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  int QAccessibleTableInterface::selectedCellCount();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedCellCount<i32> for () {
  fn selectedCellCount(self , rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface17selectedCellCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface17selectedCellCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QList<QAccessibleInterface *> QAccessibleTableInterface::selectedCells();
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedCells<RetType, T: QAccessibleTableInterface_selectedCells<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.selectedCells(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedCells<RetType> {
  fn selectedCells(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  QList<QAccessibleInterface *> QAccessibleTableInterface::selectedCells();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedCells<()> for () {
  fn selectedCells(self , rsthis: &mut QAccessibleTableInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface13selectedCellsEv()};
     unsafe {_ZNK25QAccessibleTableInterface13selectedCellsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QAccessibleTableInterface::selectRow(int row);
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectRow<RetType, T: QAccessibleTableInterface_selectRow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.selectRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectRow<RetType> {
  fn selectRow(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  bool QAccessibleTableInterface::selectRow(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_selectRow<i8> for (i32) {
  fn selectRow(self , rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface9selectRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QAccessibleTableInterface9selectRowEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  int QAccessibleTableInterface::selectedRowCount();
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedRowCount<RetType, T: QAccessibleTableInterface_selectedRowCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.selectedRowCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedRowCount<RetType> {
  fn selectedRowCount(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  int QAccessibleTableInterface::selectedRowCount();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedRowCount<i32> for () {
  fn selectedRowCount(self , rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface16selectedRowCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface16selectedRowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QAccessibleTableInterface::FreeQAccessibleTableInterface();
impl /*struct*/ QAccessibleTableInterface {
  pub fn FreeQAccessibleTableInterface<RetType, T: QAccessibleTableInterface_FreeQAccessibleTableInterface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleTableInterface(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_FreeQAccessibleTableInterface<RetType> {
  fn FreeQAccessibleTableInterface(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  void QAccessibleTableInterface::FreeQAccessibleTableInterface();
impl<'a> /*trait*/ QAccessibleTableInterface_FreeQAccessibleTableInterface<()> for () {
  fn FreeQAccessibleTableInterface(self , rsthis: &mut QAccessibleTableInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterfaceD0Ev()};
     unsafe {_ZN25QAccessibleTableInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QList<int> QAccessibleTableInterface::selectedColumns();
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedColumns<RetType, T: QAccessibleTableInterface_selectedColumns<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.selectedColumns(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedColumns<RetType> {
  fn selectedColumns(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  QList<int> QAccessibleTableInterface::selectedColumns();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedColumns<()> for () {
  fn selectedColumns(self , rsthis: &mut QAccessibleTableInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface15selectedColumnsEv()};
     unsafe {_ZNK25QAccessibleTableInterface15selectedColumnsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QAccessibleInterface * QAccessibleTableInterface::cellAt(int row, int column);
impl /*struct*/ QAccessibleTableInterface {
  pub fn cellAt<RetType, T: QAccessibleTableInterface_cellAt<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cellAt(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_cellAt<RetType> {
  fn cellAt(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  QAccessibleInterface * QAccessibleTableInterface::cellAt(int row, int column);
impl<'a> /*trait*/ QAccessibleTableInterface_cellAt<QAccessibleInterface> for (i32, i32) {
  fn cellAt(self , rsthis: &mut QAccessibleTableInterface) -> QAccessibleInterface {
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

// proto:  QList<int> QAccessibleTableInterface::selectedRows();
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedRows<RetType, T: QAccessibleTableInterface_selectedRows<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.selectedRows(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedRows<RetType> {
  fn selectedRows(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  QList<int> QAccessibleTableInterface::selectedRows();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedRows<()> for () {
  fn selectedRows(self , rsthis: &mut QAccessibleTableInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface12selectedRowsEv()};
     unsafe {_ZNK25QAccessibleTableInterface12selectedRowsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QAccessibleTableInterface::modelChange(QAccessibleTableModelChangeEvent * event);
impl /*struct*/ QAccessibleTableInterface {
  pub fn modelChange<RetType, T: QAccessibleTableInterface_modelChange<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.modelChange(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_modelChange<RetType> {
  fn modelChange(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  void QAccessibleTableInterface::modelChange(QAccessibleTableModelChangeEvent * event);
impl<'a> /*trait*/ QAccessibleTableInterface_modelChange<()> for (&'a mut QAccessibleTableModelChangeEvent) {
  fn modelChange(self , rsthis: &mut QAccessibleTableInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface11modelChangeEP32QAccessibleTableModelChangeEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QAccessibleTableInterface11modelChangeEP32QAccessibleTableModelChangeEvent(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QAccessibleTableInterface::columnCount();
impl /*struct*/ QAccessibleTableInterface {
  pub fn columnCount<RetType, T: QAccessibleTableInterface_columnCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_columnCount<RetType> {
  fn columnCount(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  int QAccessibleTableInterface::columnCount();
impl<'a> /*trait*/ QAccessibleTableInterface_columnCount<i32> for () {
  fn columnCount(self , rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface11columnCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  bool QAccessibleTableInterface::selectColumn(int column);
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectColumn<RetType, T: QAccessibleTableInterface_selectColumn<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.selectColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectColumn<RetType> {
  fn selectColumn(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  bool QAccessibleTableInterface::selectColumn(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_selectColumn<i8> for (i32) {
  fn selectColumn(self , rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface12selectColumnEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QAccessibleTableInterface12selectColumnEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QAccessibleTableInterface::unselectRow(int row);
impl /*struct*/ QAccessibleTableInterface {
  pub fn unselectRow<RetType, T: QAccessibleTableInterface_unselectRow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.unselectRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_unselectRow<RetType> {
  fn unselectRow(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  bool QAccessibleTableInterface::unselectRow(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_unselectRow<i8> for (i32) {
  fn unselectRow(self , rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface11unselectRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QAccessibleTableInterface11unselectRowEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  int QAccessibleTableInterface::rowCount();
impl /*struct*/ QAccessibleTableInterface {
  pub fn rowCount<RetType, T: QAccessibleTableInterface_rowCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_rowCount<RetType> {
  fn rowCount(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  int QAccessibleTableInterface::rowCount();
impl<'a> /*trait*/ QAccessibleTableInterface_rowCount<i32> for () {
  fn rowCount(self , rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface8rowCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QString QAccessibleTableInterface::rowDescription(int row);
impl /*struct*/ QAccessibleTableInterface {
  pub fn rowDescription<RetType, T: QAccessibleTableInterface_rowDescription<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rowDescription(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_rowDescription<RetType> {
  fn rowDescription(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  QString QAccessibleTableInterface::rowDescription(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_rowDescription<QString> for (i32) {
  fn rowDescription(self , rsthis: &mut QAccessibleTableInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface14rowDescriptionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface14rowDescriptionEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QAccessibleInterface * QAccessibleTableInterface::summary();
impl /*struct*/ QAccessibleTableInterface {
  pub fn summary<RetType, T: QAccessibleTableInterface_summary<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.summary(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_summary<RetType> {
  fn summary(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  QAccessibleInterface * QAccessibleTableInterface::summary();
impl<'a> /*trait*/ QAccessibleTableInterface_summary<QAccessibleInterface> for () {
  fn summary(self , rsthis: &mut QAccessibleTableInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface7summaryEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface7summaryEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QAccessibleTableInterface::isColumnSelected(int column);
impl /*struct*/ QAccessibleTableInterface {
  pub fn isColumnSelected<RetType, T: QAccessibleTableInterface_isColumnSelected<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isColumnSelected(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_isColumnSelected<RetType> {
  fn isColumnSelected(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  bool QAccessibleTableInterface::isColumnSelected(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_isColumnSelected<i8> for (i32) {
  fn isColumnSelected(self , rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface16isColumnSelectedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface16isColumnSelectedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QAccessibleInterface * QAccessibleTableInterface::caption();
impl /*struct*/ QAccessibleTableInterface {
  pub fn caption<RetType, T: QAccessibleTableInterface_caption<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.caption(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_caption<RetType> {
  fn caption(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  QAccessibleInterface * QAccessibleTableInterface::caption();
impl<'a> /*trait*/ QAccessibleTableInterface_caption<QAccessibleInterface> for () {
  fn caption(self , rsthis: &mut QAccessibleTableInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface7captionEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface7captionEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QAccessibleTableInterface::isRowSelected(int row);
impl /*struct*/ QAccessibleTableInterface {
  pub fn isRowSelected<RetType, T: QAccessibleTableInterface_isRowSelected<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isRowSelected(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_isRowSelected<RetType> {
  fn isRowSelected(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  bool QAccessibleTableInterface::isRowSelected(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_isRowSelected<i8> for (i32) {
  fn isRowSelected(self , rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface13isRowSelectedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface13isRowSelectedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  int QAccessibleTableInterface::selectedColumnCount();
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedColumnCount<RetType, T: QAccessibleTableInterface_selectedColumnCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.selectedColumnCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedColumnCount<RetType> {
  fn selectedColumnCount(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

// proto:  int QAccessibleTableInterface::selectedColumnCount();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedColumnCount<i32> for () {
  fn selectedColumnCount(self , rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface19selectedColumnCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface19selectedColumnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

