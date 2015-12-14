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
  fn _ZNK18QStandardItemModel9mimeTypesEv(qthis: *mut c_void) -> *mut c_void;
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

impl /*struct*/ QStandardItemModel {
  pub fn clear<T: QStandardItemModel_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QStandardItemModel_clear {
  fn clear(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  void QStandardItemModel::clear();
impl<'a> /*trait*/ QStandardItemModel_clear for () {
  fn clear(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel5clearEv()};
     unsafe {_ZN18QStandardItemModel5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn item<T: QStandardItemModel_item>(&mut self, value: T) -> QStandardItem {
    return value.item(self);
    // return 1;
  }
}

pub trait QStandardItemModel_item {
  fn item(self, rsthis: &mut QStandardItemModel) -> QStandardItem;
}

// proto:  QStandardItem * QStandardItemModel::item(int row, int column);
impl<'a> /*trait*/ QStandardItemModel_item for (i32, i32) {
  fn item(self, rsthis: &mut QStandardItemModel) -> QStandardItem {
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

impl /*struct*/ QStandardItemModel {
  pub fn setItem<T: QStandardItemModel_setItem>(&mut self, value: T)  {
     value.setItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setItem {
  fn setItem(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  void QStandardItemModel::setItem(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setItem for (i32, &'a mut QStandardItem) {
  fn setItem(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel7setItemEiP13QStandardItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel7setItemEiP13QStandardItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn index<T: QStandardItemModel_index>(&mut self, value: T) -> QModelIndex {
    return value.index(self);
    // return 1;
  }
}

pub trait QStandardItemModel_index {
  fn index(self, rsthis: &mut QStandardItemModel) -> QModelIndex;
}

// proto:  QModelIndex QStandardItemModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_index for (i32, i32, &'a  QModelIndex) {
  fn index(self, rsthis: &mut QStandardItemModel) -> QModelIndex {
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

impl /*struct*/ QStandardItemModel {
  pub fn setData<T: QStandardItemModel_setData>(&mut self, value: T) -> i8 {
    return value.setData(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setData {
  fn setData(self, rsthis: &mut QStandardItemModel) -> i8;
}

// proto:  bool QStandardItemModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QStandardItemModel_setData for (&'a  QModelIndex, &'a  QVariant, i32) {
  fn setData(self, rsthis: &mut QStandardItemModel) -> i8 {
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

impl /*struct*/ QStandardItemModel {
  pub fn columnCount<T: QStandardItemModel_columnCount>(&mut self, value: T) -> i32 {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QStandardItemModel_columnCount {
  fn columnCount(self, rsthis: &mut QStandardItemModel) -> i32;
}

// proto:  int QStandardItemModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_columnCount for (&'a  QModelIndex) {
  fn columnCount(self, rsthis: &mut QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn takeItem<T: QStandardItemModel_takeItem>(&mut self, value: T) -> QStandardItem {
    return value.takeItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeItem {
  fn takeItem(self, rsthis: &mut QStandardItemModel) -> QStandardItem;
}

// proto:  QStandardItem * QStandardItemModel::takeItem(int row, int column);
impl<'a> /*trait*/ QStandardItemModel_takeItem for (i32, i32) {
  fn takeItem(self, rsthis: &mut QStandardItemModel) -> QStandardItem {
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

impl /*struct*/ QStandardItemModel {
  pub fn setRowCount<T: QStandardItemModel_setRowCount>(&mut self, value: T)  {
     value.setRowCount(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setRowCount {
  fn setRowCount(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  void QStandardItemModel::setRowCount(int rows);
impl<'a> /*trait*/ QStandardItemModel_setRowCount for (i32) {
  fn setRowCount(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel11setRowCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QStandardItemModel11setRowCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn itemFromIndex<T: QStandardItemModel_itemFromIndex>(&mut self, value: T) -> QStandardItem {
    return value.itemFromIndex(self);
    // return 1;
  }
}

pub trait QStandardItemModel_itemFromIndex {
  fn itemFromIndex(self, rsthis: &mut QStandardItemModel) -> QStandardItem;
}

// proto:  QStandardItem * QStandardItemModel::itemFromIndex(const QModelIndex & index);
impl<'a> /*trait*/ QStandardItemModel_itemFromIndex for (&'a  QModelIndex) {
  fn itemFromIndex(self, rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel13itemFromIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel13itemFromIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn insertColumn<T: QStandardItemModel_insertColumn>(&mut self, value: T) -> i8 {
    return value.insertColumn(self);
    // return 1;
  }
}

pub trait QStandardItemModel_insertColumn {
  fn insertColumn(self, rsthis: &mut QStandardItemModel) -> i8;
}

// proto:  bool QStandardItemModel::insertColumn(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_insertColumn for (i32, &'a  QModelIndex) {
  fn insertColumn(self, rsthis: &mut QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel12insertColumnEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QStandardItemModel12insertColumnEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn setVerticalHeaderItem<T: QStandardItemModel_setVerticalHeaderItem>(&mut self, value: T)  {
     value.setVerticalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setVerticalHeaderItem {
  fn setVerticalHeaderItem(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  void QStandardItemModel::setVerticalHeaderItem(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setVerticalHeaderItem for (i32, &'a mut QStandardItem) {
  fn setVerticalHeaderItem(self, rsthis: &mut QStandardItemModel)  {
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

impl /*struct*/ QStandardItemModel {
  pub fn takeColumn<T: QStandardItemModel_takeColumn>(&mut self, value: T)  {
     value.takeColumn(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeColumn {
  fn takeColumn(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  QList<QStandardItem *> QStandardItemModel::takeColumn(int column);
impl<'a> /*trait*/ QStandardItemModel_takeColumn for (i32) {
  fn takeColumn(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel10takeColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QStandardItemModel10takeColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn takeVerticalHeaderItem<T: QStandardItemModel_takeVerticalHeaderItem>(&mut self, value: T) -> QStandardItem {
    return value.takeVerticalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeVerticalHeaderItem {
  fn takeVerticalHeaderItem(self, rsthis: &mut QStandardItemModel) -> QStandardItem;
}

// proto:  QStandardItem * QStandardItemModel::takeVerticalHeaderItem(int row);
impl<'a> /*trait*/ QStandardItemModel_takeVerticalHeaderItem for (i32) {
  fn takeVerticalHeaderItem(self, rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel22takeVerticalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN18QStandardItemModel22takeVerticalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn insertColumns<T: QStandardItemModel_insertColumns>(&mut self, value: T) -> i8 {
    return value.insertColumns(self);
    // return 1;
  }
}

pub trait QStandardItemModel_insertColumns {
  fn insertColumns(self, rsthis: &mut QStandardItemModel) -> i8;
}

// proto:  bool QStandardItemModel::insertColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_insertColumns for (i32, i32, &'a  QModelIndex) {
  fn insertColumns(self, rsthis: &mut QStandardItemModel) -> i8 {
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

impl /*struct*/ QStandardItemModel {
  pub fn metaObject<T: QStandardItemModel_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QStandardItemModel_metaObject {
  fn metaObject(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  const QMetaObject * QStandardItemModel::metaObject();
impl<'a> /*trait*/ QStandardItemModel_metaObject for () {
  fn metaObject(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel10metaObjectEv()};
     unsafe {_ZNK18QStandardItemModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn insertRows<T: QStandardItemModel_insertRows>(&mut self, value: T) -> i8 {
    return value.insertRows(self);
    // return 1;
  }
}

pub trait QStandardItemModel_insertRows {
  fn insertRows(self, rsthis: &mut QStandardItemModel) -> i8;
}

// proto:  bool QStandardItemModel::insertRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_insertRows for (i32, i32, &'a  QModelIndex) {
  fn insertRows(self, rsthis: &mut QStandardItemModel) -> i8 {
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

impl /*struct*/ QStandardItemModel {
  pub fn invisibleRootItem<T: QStandardItemModel_invisibleRootItem>(&mut self, value: T) -> QStandardItem {
    return value.invisibleRootItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_invisibleRootItem {
  fn invisibleRootItem(self, rsthis: &mut QStandardItemModel) -> QStandardItem;
}

// proto:  QStandardItem * QStandardItemModel::invisibleRootItem();
impl<'a> /*trait*/ QStandardItemModel_invisibleRootItem for () {
  fn invisibleRootItem(self, rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel17invisibleRootItemEv()};
    let mut ret = unsafe {_ZNK18QStandardItemModel17invisibleRootItemEv(rsthis.qclsinst)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn setItemPrototype<T: QStandardItemModel_setItemPrototype>(&mut self, value: T)  {
     value.setItemPrototype(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setItemPrototype {
  fn setItemPrototype(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  void QStandardItemModel::setItemPrototype(const QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setItemPrototype for (&'a  QStandardItem) {
  fn setItemPrototype(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel16setItemPrototypeEPK13QStandardItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel16setItemPrototypeEPK13QStandardItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn setHorizontalHeaderLabels<T: QStandardItemModel_setHorizontalHeaderLabels>(&mut self, value: T)  {
     value.setHorizontalHeaderLabels(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setHorizontalHeaderLabels {
  fn setHorizontalHeaderLabels(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  void QStandardItemModel::setHorizontalHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QStandardItemModel_setHorizontalHeaderLabels for (&'a  QStringList) {
  fn setHorizontalHeaderLabels(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel25setHorizontalHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel25setHorizontalHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn parent<T: QStandardItemModel_parent>(&mut self, value: T) -> QModelIndex {
    return value.parent(self);
    // return 1;
  }
}

pub trait QStandardItemModel_parent {
  fn parent(self, rsthis: &mut QStandardItemModel) -> QModelIndex;
}

// proto:  QModelIndex QStandardItemModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QStandardItemModel_parent for (&'a  QModelIndex) {
  fn parent(self, rsthis: &mut QStandardItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn removeColumns<T: QStandardItemModel_removeColumns>(&mut self, value: T) -> i8 {
    return value.removeColumns(self);
    // return 1;
  }
}

pub trait QStandardItemModel_removeColumns {
  fn removeColumns(self, rsthis: &mut QStandardItemModel) -> i8;
}

// proto:  bool QStandardItemModel::removeColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_removeColumns for (i32, i32, &'a  QModelIndex) {
  fn removeColumns(self, rsthis: &mut QStandardItemModel) -> i8 {
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

impl /*struct*/ QStandardItemModel {
  pub fn sibling<T: QStandardItemModel_sibling>(&mut self, value: T) -> QModelIndex {
    return value.sibling(self);
    // return 1;
  }
}

pub trait QStandardItemModel_sibling {
  fn sibling(self, rsthis: &mut QStandardItemModel) -> QModelIndex;
}

// proto:  QModelIndex QStandardItemModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QStandardItemModel_sibling for (i32, i32, &'a  QModelIndex) {
  fn sibling(self, rsthis: &mut QStandardItemModel) -> QModelIndex {
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

impl /*struct*/ QStandardItemModel {
  pub fn sortRole<T: QStandardItemModel_sortRole>(&mut self, value: T) -> i32 {
    return value.sortRole(self);
    // return 1;
  }
}

pub trait QStandardItemModel_sortRole {
  fn sortRole(self, rsthis: &mut QStandardItemModel) -> i32;
}

// proto:  int QStandardItemModel::sortRole();
impl<'a> /*trait*/ QStandardItemModel_sortRole for () {
  fn sortRole(self, rsthis: &mut QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel8sortRoleEv()};
    let mut ret = unsafe {_ZNK18QStandardItemModel8sortRoleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn takeHorizontalHeaderItem<T: QStandardItemModel_takeHorizontalHeaderItem>(&mut self, value: T) -> QStandardItem {
    return value.takeHorizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeHorizontalHeaderItem {
  fn takeHorizontalHeaderItem(self, rsthis: &mut QStandardItemModel) -> QStandardItem;
}

// proto:  QStandardItem * QStandardItemModel::takeHorizontalHeaderItem(int column);
impl<'a> /*trait*/ QStandardItemModel_takeHorizontalHeaderItem for (i32) {
  fn takeHorizontalHeaderItem(self, rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel24takeHorizontalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN18QStandardItemModel24takeHorizontalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn indexFromItem<T: QStandardItemModel_indexFromItem>(&mut self, value: T) -> QModelIndex {
    return value.indexFromItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_indexFromItem {
  fn indexFromItem(self, rsthis: &mut QStandardItemModel) -> QModelIndex;
}

// proto:  QModelIndex QStandardItemModel::indexFromItem(const QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_indexFromItem for (&'a  QStandardItem) {
  fn indexFromItem(self, rsthis: &mut QStandardItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel13indexFromItemEPK13QStandardItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel13indexFromItemEPK13QStandardItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn itemPrototype<T: QStandardItemModel_itemPrototype>(&mut self, value: T) -> QStandardItem {
    return value.itemPrototype(self);
    // return 1;
  }
}

pub trait QStandardItemModel_itemPrototype {
  fn itemPrototype(self, rsthis: &mut QStandardItemModel) -> QStandardItem;
}

// proto:  const QStandardItem * QStandardItemModel::itemPrototype();
impl<'a> /*trait*/ QStandardItemModel_itemPrototype for () {
  fn itemPrototype(self, rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel13itemPrototypeEv()};
    let mut ret = unsafe {_ZNK18QStandardItemModel13itemPrototypeEv(rsthis.qclsinst)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn setHorizontalHeaderItem<T: QStandardItemModel_setHorizontalHeaderItem>(&mut self, value: T)  {
     value.setHorizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setHorizontalHeaderItem {
  fn setHorizontalHeaderItem(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  void QStandardItemModel::setHorizontalHeaderItem(int column, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setHorizontalHeaderItem for (i32, &'a mut QStandardItem) {
  fn setHorizontalHeaderItem(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel23setHorizontalHeaderItemEiP13QStandardItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel23setHorizontalHeaderItemEiP13QStandardItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn horizontalHeaderItem<T: QStandardItemModel_horizontalHeaderItem>(&mut self, value: T) -> QStandardItem {
    return value.horizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_horizontalHeaderItem {
  fn horizontalHeaderItem(self, rsthis: &mut QStandardItemModel) -> QStandardItem;
}

// proto:  QStandardItem * QStandardItemModel::horizontalHeaderItem(int column);
impl<'a> /*trait*/ QStandardItemModel_horizontalHeaderItem for (i32) {
  fn horizontalHeaderItem(self, rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel20horizontalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK18QStandardItemModel20horizontalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn appendRow<T: QStandardItemModel_appendRow>(&mut self, value: T)  {
     value.appendRow(self);
    // return 1;
  }
}

pub trait QStandardItemModel_appendRow {
  fn appendRow(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  void QStandardItemModel::appendRow(QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_appendRow for (&'a mut QStandardItem) {
  fn appendRow(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel9appendRowEP13QStandardItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel9appendRowEP13QStandardItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn itemData<T: QStandardItemModel_itemData>(&mut self, value: T)  {
     value.itemData(self);
    // return 1;
  }
}

pub trait QStandardItemModel_itemData {
  fn itemData(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  QMap<int, QVariant> QStandardItemModel::itemData(const QModelIndex & index);
impl<'a> /*trait*/ QStandardItemModel_itemData for (&'a  QModelIndex) {
  fn itemData(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel8itemDataERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK18QStandardItemModel8itemDataERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn setSortRole<T: QStandardItemModel_setSortRole>(&mut self, value: T)  {
     value.setSortRole(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setSortRole {
  fn setSortRole(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  void QStandardItemModel::setSortRole(int role);
impl<'a> /*trait*/ QStandardItemModel_setSortRole for (i32) {
  fn setSortRole(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel11setSortRoleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QStandardItemModel11setSortRoleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn itemChanged<T: QStandardItemModel_itemChanged>(&mut self, value: T)  {
     value.itemChanged(self);
    // return 1;
  }
}

pub trait QStandardItemModel_itemChanged {
  fn itemChanged(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  void QStandardItemModel::itemChanged(QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_itemChanged for (&'a mut QStandardItem) {
  fn itemChanged(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel11itemChangedEP13QStandardItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel11itemChangedEP13QStandardItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn hasChildren<T: QStandardItemModel_hasChildren>(&mut self, value: T) -> i8 {
    return value.hasChildren(self);
    // return 1;
  }
}

pub trait QStandardItemModel_hasChildren {
  fn hasChildren(self, rsthis: &mut QStandardItemModel) -> i8;
}

// proto:  bool QStandardItemModel::hasChildren(const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_hasChildren for (&'a  QModelIndex) {
  fn hasChildren(self, rsthis: &mut QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel11hasChildrenERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn FreeQStandardItemModel<T: QStandardItemModel_FreeQStandardItemModel>(&mut self, value: T)  {
     value.FreeQStandardItemModel(self);
    // return 1;
  }
}

pub trait QStandardItemModel_FreeQStandardItemModel {
  fn FreeQStandardItemModel(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  void QStandardItemModel::FreeQStandardItemModel();
impl<'a> /*trait*/ QStandardItemModel_FreeQStandardItemModel for () {
  fn FreeQStandardItemModel(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModelD0Ev()};
     unsafe {_ZN18QStandardItemModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn data<T: QStandardItemModel_data>(&mut self, value: T) -> QVariant {
    return value.data(self);
    // return 1;
  }
}

pub trait QStandardItemModel_data {
  fn data(self, rsthis: &mut QStandardItemModel) -> QVariant;
}

// proto:  QVariant QStandardItemModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QStandardItemModel_data for (&'a  QModelIndex, i32) {
  fn data(self, rsthis: &mut QStandardItemModel) -> QVariant {
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

impl /*struct*/ QStandardItemModel {
  pub fn takeRow<T: QStandardItemModel_takeRow>(&mut self, value: T)  {
     value.takeRow(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeRow {
  fn takeRow(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  QList<QStandardItem *> QStandardItemModel::takeRow(int row);
impl<'a> /*trait*/ QStandardItemModel_takeRow for (i32) {
  fn takeRow(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel7takeRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QStandardItemModel7takeRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn setColumnCount<T: QStandardItemModel_setColumnCount>(&mut self, value: T)  {
     value.setColumnCount(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setColumnCount {
  fn setColumnCount(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  void QStandardItemModel::setColumnCount(int columns);
impl<'a> /*trait*/ QStandardItemModel_setColumnCount for (i32) {
  fn setColumnCount(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel14setColumnCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QStandardItemModel14setColumnCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn verticalHeaderItem<T: QStandardItemModel_verticalHeaderItem>(&mut self, value: T) -> QStandardItem {
    return value.verticalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_verticalHeaderItem {
  fn verticalHeaderItem(self, rsthis: &mut QStandardItemModel) -> QStandardItem;
}

// proto:  QStandardItem * QStandardItemModel::verticalHeaderItem(int row);
impl<'a> /*trait*/ QStandardItemModel_verticalHeaderItem for (i32) {
  fn verticalHeaderItem(self, rsthis: &mut QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel18verticalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK18QStandardItemModel18verticalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn removeRows<T: QStandardItemModel_removeRows>(&mut self, value: T) -> i8 {
    return value.removeRows(self);
    // return 1;
  }
}

pub trait QStandardItemModel_removeRows {
  fn removeRows(self, rsthis: &mut QStandardItemModel) -> i8;
}

// proto:  bool QStandardItemModel::removeRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_removeRows for (i32, i32, &'a  QModelIndex) {
  fn removeRows(self, rsthis: &mut QStandardItemModel) -> i8 {
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

impl /*struct*/ QStandardItemModel {
  pub fn setVerticalHeaderLabels<T: QStandardItemModel_setVerticalHeaderLabels>(&mut self, value: T)  {
     value.setVerticalHeaderLabels(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setVerticalHeaderLabels {
  fn setVerticalHeaderLabels(self, rsthis: &mut QStandardItemModel) ;
}

// proto:  void QStandardItemModel::setVerticalHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QStandardItemModel_setVerticalHeaderLabels for (&'a  QStringList) {
  fn setVerticalHeaderLabels(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel23setVerticalHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel23setVerticalHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QStandardItemModel::setItem(int row, int column, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setItem for (i32, i32, &'a mut QStandardItem) {
  fn setItem(self, rsthis: &mut QStandardItemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel7setItemEiiP13QStandardItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN18QStandardItemModel7setItemEiiP13QStandardItem(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn mimeTypes<T: QStandardItemModel_mimeTypes>(&mut self, value: T) -> QStringList {
    return value.mimeTypes(self);
    // return 1;
  }
}

pub trait QStandardItemModel_mimeTypes {
  fn mimeTypes(self, rsthis: &mut QStandardItemModel) -> QStringList;
}

// proto:  QStringList QStandardItemModel::mimeTypes();
impl<'a> /*trait*/ QStandardItemModel_mimeTypes for () {
  fn mimeTypes(self, rsthis: &mut QStandardItemModel) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel9mimeTypesEv()};
    let mut ret = unsafe {_ZNK18QStandardItemModel9mimeTypesEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItemModel {
  pub fn rowCount<T: QStandardItemModel_rowCount>(&mut self, value: T) -> i32 {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QStandardItemModel_rowCount {
  fn rowCount(self, rsthis: &mut QStandardItemModel) -> i32;
}

// proto:  int QStandardItemModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_rowCount for (&'a  QModelIndex) {
  fn rowCount(self, rsthis: &mut QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QStandardItemModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

