// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstandarditem::QStandardItem;
use super::qmodelindex::QModelIndex;
use super::qvariant::QVariant;
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QStandardItemModel::NewQStandardItemModel(int rows, int columns, QObject * parent);
  fn _ZN18QStandardItemModelC1EiiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  void QStandardItemModel::clear();
  fn _ZN18QStandardItemModel5clearEv(qthis: *mut c_void) ;
  // proto:  QStandardItem * QStandardItemModel::item(int row, int column);
  fn _ZNK18QStandardItemModel4itemEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  bool QStandardItemModel::insertRow(int row, const QModelIndex & parent);
  fn _ZN18QStandardItemModel9insertRowEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> int8_t;
  // proto:  void QStandardItemModel::setItem(int row, QStandardItem * item);
  fn _ZN18QStandardItemModel7setItemEiP13QStandardItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QModelIndex QStandardItemModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK18QStandardItemModel5indexEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  bool QStandardItemModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn _ZN18QStandardItemModel7setDataERK11QModelIndexRK8QVarianti(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> int8_t;
  // proto:  int QStandardItemModel::columnCount(const QModelIndex & parent);
  fn _ZNK18QStandardItemModel11columnCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QStandardItem * QStandardItemModel::takeItem(int row, int column);
  fn _ZN18QStandardItemModel8takeItemEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QStandardItemModel::setRowCount(int rows);
  fn _ZN18QStandardItemModel11setRowCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QStandardItem * QStandardItemModel::itemFromIndex(const QModelIndex & index);
  fn _ZNK18QStandardItemModel13itemFromIndexERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QStandardItemModel::insertColumn(int column, const QModelIndex & parent);
  fn _ZN18QStandardItemModel12insertColumnEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> int8_t;
  // proto:  void QStandardItemModel::setVerticalHeaderItem(int row, QStandardItem * item);
  fn _ZN18QStandardItemModel21setVerticalHeaderItemEiP13QStandardItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QStandardItemModel::NewQStandardItemModel(const QStandardItemModel & );
  fn _ZN18QStandardItemModelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItemModel::NewQStandardItemModel(QObject * parent);
  fn _ZN18QStandardItemModelC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QList<QStandardItem *> QStandardItemModel::takeColumn(int column);
  fn _ZN18QStandardItemModel10takeColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QStandardItem * QStandardItemModel::takeVerticalHeaderItem(int row);
  fn _ZN18QStandardItemModel22takeVerticalHeaderItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QStandardItemModel::insertColumns(int column, int count, const QModelIndex & parent);
  fn _ZN18QStandardItemModel13insertColumnsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QStandardItemModel::metaObject();
  fn _ZNK18QStandardItemModel10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QStandardItemModel::insertRows(int row, int count, const QModelIndex & parent);
  fn _ZN18QStandardItemModel10insertRowsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  void QStandardItemModel::insertRow(int row, QStandardItem * item);
  fn _ZN18QStandardItemModel9insertRowEiP13QStandardItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QStandardItem * QStandardItemModel::invisibleRootItem();
  fn _ZNK18QStandardItemModel17invisibleRootItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItemModel::setItemPrototype(const QStandardItem * item);
  fn _ZN18QStandardItemModel16setItemPrototypeEPK13QStandardItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItemModel::setHorizontalHeaderLabels(const QStringList & labels);
  fn _ZN18QStandardItemModel25setHorizontalHeaderLabelsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QModelIndex QStandardItemModel::parent(const QModelIndex & child);
  fn _ZNK18QStandardItemModel6parentERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QStandardItemModel::removeColumns(int column, int count, const QModelIndex & parent);
  fn _ZN18QStandardItemModel13removeColumnsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  QModelIndex QStandardItemModel::sibling(int row, int column, const QModelIndex & idx);
  fn _ZNK18QStandardItemModel7siblingEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  int QStandardItemModel::sortRole();
  fn _ZNK18QStandardItemModel8sortRoleEv(qthis: *mut c_void) -> c_int;
  // proto:  QStandardItem * QStandardItemModel::takeHorizontalHeaderItem(int column);
  fn _ZN18QStandardItemModel24takeHorizontalHeaderItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QModelIndex QStandardItemModel::indexFromItem(const QStandardItem * item);
  fn _ZNK18QStandardItemModel13indexFromItemEPK13QStandardItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QStandardItem * QStandardItemModel::itemPrototype();
  fn _ZNK18QStandardItemModel13itemPrototypeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItemModel::setHorizontalHeaderItem(int column, QStandardItem * item);
  fn _ZN18QStandardItemModel23setHorizontalHeaderItemEiP13QStandardItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QStandardItem * QStandardItemModel::horizontalHeaderItem(int column);
  fn _ZNK18QStandardItemModel20horizontalHeaderItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QStandardItemModel::appendRow(QStandardItem * item);
  fn _ZN18QStandardItemModel9appendRowEP13QStandardItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QMap<int, QVariant> QStandardItemModel::itemData(const QModelIndex & index);
  fn _ZNK18QStandardItemModel8itemDataERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItemModel::setSortRole(int role);
  fn _ZN18QStandardItemModel11setSortRoleEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QStandardItemModel::itemChanged(QStandardItem * item);
  fn _ZN18QStandardItemModel11itemChangedEP13QStandardItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QStandardItemModel::hasChildren(const QModelIndex & parent);
  fn _ZNK18QStandardItemModel11hasChildrenERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QStandardItemModel::FreeQStandardItemModel();
  fn _ZN18QStandardItemModelD0Ev(qthis: *mut c_void) ;
  // proto:  QVariant QStandardItemModel::data(const QModelIndex & index, int role);
  fn _ZNK18QStandardItemModel4dataERK11QModelIndexi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QList<QStandardItem *> QStandardItemModel::takeRow(int row);
  fn _ZN18QStandardItemModel7takeRowEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QStandardItemModel::setColumnCount(int columns);
  fn _ZN18QStandardItemModel14setColumnCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QStandardItem * QStandardItemModel::verticalHeaderItem(int row);
  fn _ZNK18QStandardItemModel18verticalHeaderItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QStandardItemModel::removeRows(int row, int count, const QModelIndex & parent);
  fn _ZN18QStandardItemModel10removeRowsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  void QStandardItemModel::setVerticalHeaderLabels(const QStringList & labels);
  fn _ZN18QStandardItemModel23setVerticalHeaderLabelsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItemModel::setItem(int row, int column, QStandardItem * item);
  fn _ZN18QStandardItemModel7setItemEiiP13QStandardItem(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  QStringList QStandardItemModel::mimeTypes();
  fn _ZNK18QStandardItemModel9mimeTypesEv(qthis: *mut c_void) ;
  // proto:  int QStandardItemModel::rowCount(const QModelIndex & parent);
  fn _ZNK18QStandardItemModel8rowCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QStandardItemModel)=1
pub struct QStandardItemModel {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStandardItemModel {
  pub fn NewQStandardItemModel<T: QStandardItemModel_NewQStandardItemModel>(value: T) -> QStandardItemModel {
    let rsthis = value.NewQStandardItemModel();
    return rsthis;
    // return 1;
  }
}

pub trait QStandardItemModel_NewQStandardItemModel {
  fn NewQStandardItemModel(self) -> QStandardItemModel;
}

// proto: void QStandardItemModel::NewQStandardItemModel(int rows, int columns, QObject * parent);
impl<'a> /*trait*/ QStandardItemModel_NewQStandardItemModel for (i32, i32, &'a mut QObject) {
  fn NewQStandardItemModel(self) -> QStandardItemModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModelC1EiiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN18QStandardItemModelC1EiiP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QStandardItemModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QStandardItemModel::clear();
impl /*struct*/ QStandardItemModel {
  pub fn clear<RetType, T: QStandardItemModel_clear<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QStandardItemModel_clear<RetType> {
  fn clear(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  void QStandardItemModel::clear();
impl<'a> /*trait*/ QStandardItemModel_clear<()> for () {
  fn clear(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel5clearEv()};
     unsafe {_ZN18QStandardItemModel5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QStandardItem * QStandardItemModel::item(int row, int column);
impl /*struct*/ QStandardItemModel {
  pub fn item<RetType, T: QStandardItemModel_item<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.item(self);
    // return 1;
  }
}

pub trait QStandardItemModel_item<RetType> {
  fn item(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QStandardItem * QStandardItemModel::item(int row, int column);
impl<'a> /*trait*/ QStandardItemModel_item<QStandardItem> for (i32, i32) {
  fn item(self , rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel4itemEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK18QStandardItemModel4itemEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QStandardItemModel::insertRow(int row, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn insertRow<RetType, T: QStandardItemModel_insertRow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.insertRow(self);
    // return 1;
  }
}

pub trait QStandardItemModel_insertRow<RetType> {
  fn insertRow(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  bool QStandardItemModel::insertRow(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_insertRow<i8> for (i32, &'a  QModelIndex) {
  fn insertRow(self , rsthis: &mut QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel9insertRowEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QStandardItemModel9insertRowEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QStandardItemModel::setItem(int row, QStandardItem * item);
impl /*struct*/ QStandardItemModel {
  pub fn setItem<RetType, T: QStandardItemModel_setItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setItem<RetType> {
  fn setItem(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  void QStandardItemModel::setItem(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setItem<()> for (i32, &'a mut QStandardItem) {
  fn setItem(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel7setItemEiP13QStandardItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel7setItemEiP13QStandardItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QModelIndex QStandardItemModel::index(int row, int column, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn index<RetType, T: QStandardItemModel_index<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QStandardItemModel_index<RetType> {
  fn index(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QModelIndex QStandardItemModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_index<QModelIndex> for (i32, i32, &'a  QModelIndex) {
  fn index(self , rsthis: &mut QStandardItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QStandardItemModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl /*struct*/ QStandardItemModel {
  pub fn setData<RetType, T: QStandardItemModel_setData<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setData<RetType> {
  fn setData(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  bool QStandardItemModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QStandardItemModel_setData<i8> for (&'a  QModelIndex, &'a  QVariant, i32) {
  fn setData(self , rsthis: &mut QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN18QStandardItemModel7setDataERK11QModelIndexRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

// proto:  int QStandardItemModel::columnCount(const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn columnCount<RetType, T: QStandardItemModel_columnCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QStandardItemModel_columnCount<RetType> {
  fn columnCount(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  int QStandardItemModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_columnCount<i32> for (&'a  QModelIndex) {
  fn columnCount(self , rsthis: &mut QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QStandardItem * QStandardItemModel::takeItem(int row, int column);
impl /*struct*/ QStandardItemModel {
  pub fn takeItem<RetType, T: QStandardItemModel_takeItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.takeItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeItem<RetType> {
  fn takeItem(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QStandardItem * QStandardItemModel::takeItem(int row, int column);
impl<'a> /*trait*/ QStandardItemModel_takeItem<QStandardItem> for (i32, i32) {
  fn takeItem(self , rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel8takeItemEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN18QStandardItemModel8takeItemEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QStandardItemModel::setRowCount(int rows);
impl /*struct*/ QStandardItemModel {
  pub fn setRowCount<RetType, T: QStandardItemModel_setRowCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRowCount(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setRowCount<RetType> {
  fn setRowCount(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  void QStandardItemModel::setRowCount(int rows);
impl<'a> /*trait*/ QStandardItemModel_setRowCount<()> for (i32) {
  fn setRowCount(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel11setRowCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QStandardItemModel11setRowCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QStandardItem * QStandardItemModel::itemFromIndex(const QModelIndex & index);
impl /*struct*/ QStandardItemModel {
  pub fn itemFromIndex<RetType, T: QStandardItemModel_itemFromIndex<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemFromIndex(self);
    // return 1;
  }
}

pub trait QStandardItemModel_itemFromIndex<RetType> {
  fn itemFromIndex(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QStandardItem * QStandardItemModel::itemFromIndex(const QModelIndex & index);
impl<'a> /*trait*/ QStandardItemModel_itemFromIndex<QStandardItem> for (&'a  QModelIndex) {
  fn itemFromIndex(self , rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel13itemFromIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel13itemFromIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QStandardItemModel::insertColumn(int column, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn insertColumn<RetType, T: QStandardItemModel_insertColumn<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.insertColumn(self);
    // return 1;
  }
}

pub trait QStandardItemModel_insertColumn<RetType> {
  fn insertColumn(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  bool QStandardItemModel::insertColumn(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_insertColumn<i8> for (i32, &'a  QModelIndex) {
  fn insertColumn(self , rsthis: &mut QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel12insertColumnEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QStandardItemModel12insertColumnEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QStandardItemModel::setVerticalHeaderItem(int row, QStandardItem * item);
impl /*struct*/ QStandardItemModel {
  pub fn setVerticalHeaderItem<RetType, T: QStandardItemModel_setVerticalHeaderItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setVerticalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setVerticalHeaderItem<RetType> {
  fn setVerticalHeaderItem(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  void QStandardItemModel::setVerticalHeaderItem(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setVerticalHeaderItem<()> for (i32, &'a mut QStandardItem) {
  fn setVerticalHeaderItem(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel21setVerticalHeaderItemEiP13QStandardItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel21setVerticalHeaderItemEiP13QStandardItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto: void QStandardItemModel::NewQStandardItemModel(const QStandardItemModel & );
impl<'a> /*trait*/ QStandardItemModel_NewQStandardItemModel for (&'a  QStandardItemModel) {
  fn NewQStandardItemModel(self) -> QStandardItemModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModelC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QStandardItemModelC1ERKS_(qthis, arg0)};
    let rsthis = QStandardItemModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStandardItemModel::NewQStandardItemModel(QObject * parent);
impl<'a> /*trait*/ QStandardItemModel_NewQStandardItemModel for (&'a mut QObject) {
  fn NewQStandardItemModel(self) -> QStandardItemModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModelC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QStandardItemModelC1EP7QObject(qthis, arg0)};
    let rsthis = QStandardItemModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  QList<QStandardItem *> QStandardItemModel::takeColumn(int column);
impl /*struct*/ QStandardItemModel {
  pub fn takeColumn<RetType, T: QStandardItemModel_takeColumn<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.takeColumn(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeColumn<RetType> {
  fn takeColumn(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QList<QStandardItem *> QStandardItemModel::takeColumn(int column);
impl<'a> /*trait*/ QStandardItemModel_takeColumn<()> for (i32) {
  fn takeColumn(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel10takeColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QStandardItemModel10takeColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QStandardItem * QStandardItemModel::takeVerticalHeaderItem(int row);
impl /*struct*/ QStandardItemModel {
  pub fn takeVerticalHeaderItem<RetType, T: QStandardItemModel_takeVerticalHeaderItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.takeVerticalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeVerticalHeaderItem<RetType> {
  fn takeVerticalHeaderItem(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QStandardItem * QStandardItemModel::takeVerticalHeaderItem(int row);
impl<'a> /*trait*/ QStandardItemModel_takeVerticalHeaderItem<QStandardItem> for (i32) {
  fn takeVerticalHeaderItem(self , rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel22takeVerticalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN18QStandardItemModel22takeVerticalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QStandardItemModel::insertColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn insertColumns<RetType, T: QStandardItemModel_insertColumns<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.insertColumns(self);
    // return 1;
  }
}

pub trait QStandardItemModel_insertColumns<RetType> {
  fn insertColumns(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  bool QStandardItemModel::insertColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_insertColumns<i8> for (i32, i32, &'a  QModelIndex) {
  fn insertColumns(self , rsthis: &mut QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel13insertColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QStandardItemModel13insertColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

// proto:  const QMetaObject * QStandardItemModel::metaObject();
impl /*struct*/ QStandardItemModel {
  pub fn metaObject<RetType, T: QStandardItemModel_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStandardItemModel_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  const QMetaObject * QStandardItemModel::metaObject();
impl<'a> /*trait*/ QStandardItemModel_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel10metaObjectEv()};
     unsafe {_ZNK18QStandardItemModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QStandardItemModel::insertRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn insertRows<RetType, T: QStandardItemModel_insertRows<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.insertRows(self);
    // return 1;
  }
}

pub trait QStandardItemModel_insertRows<RetType> {
  fn insertRows(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  bool QStandardItemModel::insertRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_insertRows<i8> for (i32, i32, &'a  QModelIndex) {
  fn insertRows(self , rsthis: &mut QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel10insertRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QStandardItemModel10insertRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QStandardItemModel::insertRow(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_insertRow<()> for (i32, &'a mut QStandardItem) {
  fn insertRow(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel9insertRowEiP13QStandardItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel9insertRowEiP13QStandardItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QStandardItem * QStandardItemModel::invisibleRootItem();
impl /*struct*/ QStandardItemModel {
  pub fn invisibleRootItem<RetType, T: QStandardItemModel_invisibleRootItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.invisibleRootItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_invisibleRootItem<RetType> {
  fn invisibleRootItem(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QStandardItem * QStandardItemModel::invisibleRootItem();
impl<'a> /*trait*/ QStandardItemModel_invisibleRootItem<QStandardItem> for () {
  fn invisibleRootItem(self , rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel17invisibleRootItemEv()};
    let mut ret = unsafe {_ZNK18QStandardItemModel17invisibleRootItemEv(rsthis.qclsinst)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QStandardItemModel::setItemPrototype(const QStandardItem * item);
impl /*struct*/ QStandardItemModel {
  pub fn setItemPrototype<RetType, T: QStandardItemModel_setItemPrototype<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setItemPrototype(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setItemPrototype<RetType> {
  fn setItemPrototype(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  void QStandardItemModel::setItemPrototype(const QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setItemPrototype<()> for (&'a  QStandardItem) {
  fn setItemPrototype(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel16setItemPrototypeEPK13QStandardItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel16setItemPrototypeEPK13QStandardItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QStandardItemModel::setHorizontalHeaderLabels(const QStringList & labels);
impl /*struct*/ QStandardItemModel {
  pub fn setHorizontalHeaderLabels<RetType, T: QStandardItemModel_setHorizontalHeaderLabels<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setHorizontalHeaderLabels(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setHorizontalHeaderLabels<RetType> {
  fn setHorizontalHeaderLabels(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  void QStandardItemModel::setHorizontalHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QStandardItemModel_setHorizontalHeaderLabels<()> for (&'a  QStringList) {
  fn setHorizontalHeaderLabels(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel25setHorizontalHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel25setHorizontalHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QModelIndex QStandardItemModel::parent(const QModelIndex & child);
impl /*struct*/ QStandardItemModel {
  pub fn parent<RetType, T: QStandardItemModel_parent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QStandardItemModel_parent<RetType> {
  fn parent(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QModelIndex QStandardItemModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QStandardItemModel_parent<QModelIndex> for (&'a  QModelIndex) {
  fn parent(self , rsthis: &mut QStandardItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QStandardItemModel::removeColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn removeColumns<RetType, T: QStandardItemModel_removeColumns<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.removeColumns(self);
    // return 1;
  }
}

pub trait QStandardItemModel_removeColumns<RetType> {
  fn removeColumns(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  bool QStandardItemModel::removeColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_removeColumns<i8> for (i32, i32, &'a  QModelIndex) {
  fn removeColumns(self , rsthis: &mut QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel13removeColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QStandardItemModel13removeColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QModelIndex QStandardItemModel::sibling(int row, int column, const QModelIndex & idx);
impl /*struct*/ QStandardItemModel {
  pub fn sibling<RetType, T: QStandardItemModel_sibling<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QStandardItemModel_sibling<RetType> {
  fn sibling(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QModelIndex QStandardItemModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QStandardItemModel_sibling<QModelIndex> for (i32, i32, &'a  QModelIndex) {
  fn sibling(self , rsthis: &mut QStandardItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel7siblingEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel7siblingEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QStandardItemModel::sortRole();
impl /*struct*/ QStandardItemModel {
  pub fn sortRole<RetType, T: QStandardItemModel_sortRole<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.sortRole(self);
    // return 1;
  }
}

pub trait QStandardItemModel_sortRole<RetType> {
  fn sortRole(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  int QStandardItemModel::sortRole();
impl<'a> /*trait*/ QStandardItemModel_sortRole<i32> for () {
  fn sortRole(self , rsthis: &mut QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel8sortRoleEv()};
    let mut ret = unsafe {_ZNK18QStandardItemModel8sortRoleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QStandardItem * QStandardItemModel::takeHorizontalHeaderItem(int column);
impl /*struct*/ QStandardItemModel {
  pub fn takeHorizontalHeaderItem<RetType, T: QStandardItemModel_takeHorizontalHeaderItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.takeHorizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeHorizontalHeaderItem<RetType> {
  fn takeHorizontalHeaderItem(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QStandardItem * QStandardItemModel::takeHorizontalHeaderItem(int column);
impl<'a> /*trait*/ QStandardItemModel_takeHorizontalHeaderItem<QStandardItem> for (i32) {
  fn takeHorizontalHeaderItem(self , rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel24takeHorizontalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN18QStandardItemModel24takeHorizontalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QModelIndex QStandardItemModel::indexFromItem(const QStandardItem * item);
impl /*struct*/ QStandardItemModel {
  pub fn indexFromItem<RetType, T: QStandardItemModel_indexFromItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.indexFromItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_indexFromItem<RetType> {
  fn indexFromItem(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QModelIndex QStandardItemModel::indexFromItem(const QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_indexFromItem<QModelIndex> for (&'a  QStandardItem) {
  fn indexFromItem(self , rsthis: &mut QStandardItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel13indexFromItemEPK13QStandardItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel13indexFromItemEPK13QStandardItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QStandardItem * QStandardItemModel::itemPrototype();
impl /*struct*/ QStandardItemModel {
  pub fn itemPrototype<RetType, T: QStandardItemModel_itemPrototype<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemPrototype(self);
    // return 1;
  }
}

pub trait QStandardItemModel_itemPrototype<RetType> {
  fn itemPrototype(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  const QStandardItem * QStandardItemModel::itemPrototype();
impl<'a> /*trait*/ QStandardItemModel_itemPrototype<QStandardItem> for () {
  fn itemPrototype(self , rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel13itemPrototypeEv()};
    let mut ret = unsafe {_ZNK18QStandardItemModel13itemPrototypeEv(rsthis.qclsinst)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QStandardItemModel::setHorizontalHeaderItem(int column, QStandardItem * item);
impl /*struct*/ QStandardItemModel {
  pub fn setHorizontalHeaderItem<RetType, T: QStandardItemModel_setHorizontalHeaderItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setHorizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setHorizontalHeaderItem<RetType> {
  fn setHorizontalHeaderItem(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  void QStandardItemModel::setHorizontalHeaderItem(int column, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setHorizontalHeaderItem<()> for (i32, &'a mut QStandardItem) {
  fn setHorizontalHeaderItem(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel23setHorizontalHeaderItemEiP13QStandardItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel23setHorizontalHeaderItemEiP13QStandardItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QStandardItem * QStandardItemModel::horizontalHeaderItem(int column);
impl /*struct*/ QStandardItemModel {
  pub fn horizontalHeaderItem<RetType, T: QStandardItemModel_horizontalHeaderItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.horizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_horizontalHeaderItem<RetType> {
  fn horizontalHeaderItem(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QStandardItem * QStandardItemModel::horizontalHeaderItem(int column);
impl<'a> /*trait*/ QStandardItemModel_horizontalHeaderItem<QStandardItem> for (i32) {
  fn horizontalHeaderItem(self , rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel20horizontalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK18QStandardItemModel20horizontalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QStandardItemModel::appendRow(QStandardItem * item);
impl /*struct*/ QStandardItemModel {
  pub fn appendRow<RetType, T: QStandardItemModel_appendRow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.appendRow(self);
    // return 1;
  }
}

pub trait QStandardItemModel_appendRow<RetType> {
  fn appendRow(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  void QStandardItemModel::appendRow(QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_appendRow<()> for (&'a mut QStandardItem) {
  fn appendRow(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel9appendRowEP13QStandardItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel9appendRowEP13QStandardItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QMap<int, QVariant> QStandardItemModel::itemData(const QModelIndex & index);
impl /*struct*/ QStandardItemModel {
  pub fn itemData<RetType, T: QStandardItemModel_itemData<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemData(self);
    // return 1;
  }
}

pub trait QStandardItemModel_itemData<RetType> {
  fn itemData(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QMap<int, QVariant> QStandardItemModel::itemData(const QModelIndex & index);
impl<'a> /*trait*/ QStandardItemModel_itemData<()> for (&'a  QModelIndex) {
  fn itemData(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel8itemDataERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK18QStandardItemModel8itemDataERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QStandardItemModel::setSortRole(int role);
impl /*struct*/ QStandardItemModel {
  pub fn setSortRole<RetType, T: QStandardItemModel_setSortRole<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSortRole(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setSortRole<RetType> {
  fn setSortRole(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  void QStandardItemModel::setSortRole(int role);
impl<'a> /*trait*/ QStandardItemModel_setSortRole<()> for (i32) {
  fn setSortRole(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel11setSortRoleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QStandardItemModel11setSortRoleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QStandardItemModel::itemChanged(QStandardItem * item);
impl /*struct*/ QStandardItemModel {
  pub fn itemChanged<RetType, T: QStandardItemModel_itemChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemChanged(self);
    // return 1;
  }
}

pub trait QStandardItemModel_itemChanged<RetType> {
  fn itemChanged(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  void QStandardItemModel::itemChanged(QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_itemChanged<()> for (&'a mut QStandardItem) {
  fn itemChanged(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel11itemChangedEP13QStandardItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel11itemChangedEP13QStandardItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QStandardItemModel::hasChildren(const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn hasChildren<RetType, T: QStandardItemModel_hasChildren<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hasChildren(self);
    // return 1;
  }
}

pub trait QStandardItemModel_hasChildren<RetType> {
  fn hasChildren(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  bool QStandardItemModel::hasChildren(const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_hasChildren<i8> for (&'a  QModelIndex) {
  fn hasChildren(self , rsthis: &mut QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel11hasChildrenERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QStandardItemModel::FreeQStandardItemModel();
impl /*struct*/ QStandardItemModel {
  pub fn FreeQStandardItemModel<RetType, T: QStandardItemModel_FreeQStandardItemModel<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQStandardItemModel(self);
    // return 1;
  }
}

pub trait QStandardItemModel_FreeQStandardItemModel<RetType> {
  fn FreeQStandardItemModel(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  void QStandardItemModel::FreeQStandardItemModel();
impl<'a> /*trait*/ QStandardItemModel_FreeQStandardItemModel<()> for () {
  fn FreeQStandardItemModel(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModelD0Ev()};
     unsafe {_ZN18QStandardItemModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QVariant QStandardItemModel::data(const QModelIndex & index, int role);
impl /*struct*/ QStandardItemModel {
  pub fn data<RetType, T: QStandardItemModel_data<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QStandardItemModel_data<RetType> {
  fn data(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QVariant QStandardItemModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QStandardItemModel_data<QVariant> for (&'a  QModelIndex, i32) {
  fn data(self , rsthis: &mut QStandardItemModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK18QStandardItemModel4dataERK11QModelIndexi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QList<QStandardItem *> QStandardItemModel::takeRow(int row);
impl /*struct*/ QStandardItemModel {
  pub fn takeRow<RetType, T: QStandardItemModel_takeRow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.takeRow(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeRow<RetType> {
  fn takeRow(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QList<QStandardItem *> QStandardItemModel::takeRow(int row);
impl<'a> /*trait*/ QStandardItemModel_takeRow<()> for (i32) {
  fn takeRow(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel7takeRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QStandardItemModel7takeRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QStandardItemModel::setColumnCount(int columns);
impl /*struct*/ QStandardItemModel {
  pub fn setColumnCount<RetType, T: QStandardItemModel_setColumnCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setColumnCount(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setColumnCount<RetType> {
  fn setColumnCount(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  void QStandardItemModel::setColumnCount(int columns);
impl<'a> /*trait*/ QStandardItemModel_setColumnCount<()> for (i32) {
  fn setColumnCount(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel14setColumnCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QStandardItemModel14setColumnCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QStandardItem * QStandardItemModel::verticalHeaderItem(int row);
impl /*struct*/ QStandardItemModel {
  pub fn verticalHeaderItem<RetType, T: QStandardItemModel_verticalHeaderItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.verticalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_verticalHeaderItem<RetType> {
  fn verticalHeaderItem(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QStandardItem * QStandardItemModel::verticalHeaderItem(int row);
impl<'a> /*trait*/ QStandardItemModel_verticalHeaderItem<QStandardItem> for (i32) {
  fn verticalHeaderItem(self , rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel18verticalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK18QStandardItemModel18verticalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QStandardItemModel::removeRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn removeRows<RetType, T: QStandardItemModel_removeRows<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.removeRows(self);
    // return 1;
  }
}

pub trait QStandardItemModel_removeRows<RetType> {
  fn removeRows(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  bool QStandardItemModel::removeRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_removeRows<i8> for (i32, i32, &'a  QModelIndex) {
  fn removeRows(self , rsthis: &mut QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel10removeRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QStandardItemModel10removeRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QStandardItemModel::setVerticalHeaderLabels(const QStringList & labels);
impl /*struct*/ QStandardItemModel {
  pub fn setVerticalHeaderLabels<RetType, T: QStandardItemModel_setVerticalHeaderLabels<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setVerticalHeaderLabels(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setVerticalHeaderLabels<RetType> {
  fn setVerticalHeaderLabels(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  void QStandardItemModel::setVerticalHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QStandardItemModel_setVerticalHeaderLabels<()> for (&'a  QStringList) {
  fn setVerticalHeaderLabels(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel23setVerticalHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel23setVerticalHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QStandardItemModel::setItem(int row, int column, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setItem<()> for (i32, i32, &'a mut QStandardItem) {
  fn setItem(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel7setItemEiiP13QStandardItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel7setItemEiiP13QStandardItem(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  QStringList QStandardItemModel::mimeTypes();
impl /*struct*/ QStandardItemModel {
  pub fn mimeTypes<RetType, T: QStandardItemModel_mimeTypes<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.mimeTypes(self);
    // return 1;
  }
}

pub trait QStandardItemModel_mimeTypes<RetType> {
  fn mimeTypes(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  QStringList QStandardItemModel::mimeTypes();
impl<'a> /*trait*/ QStandardItemModel_mimeTypes<()> for () {
  fn mimeTypes(self , rsthis: &mut QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel9mimeTypesEv()};
     unsafe {_ZNK18QStandardItemModel9mimeTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QStandardItemModel::rowCount(const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn rowCount<RetType, T: QStandardItemModel_rowCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QStandardItemModel_rowCount<RetType> {
  fn rowCount(self , rsthis: &mut QStandardItemModel) -> RetType;
}

// proto:  int QStandardItemModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_rowCount<i32> for (&'a  QModelIndex) {
  fn rowCount(self , rsthis: &mut QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

