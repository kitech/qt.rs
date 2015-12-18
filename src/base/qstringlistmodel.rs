// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstringlist::QStringList;
use super::qobject::QObject;
use super::qmodelindex::QModelIndex;
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QStringListModel::NewQStringListModel(const QStringList & strings, QObject * parent);
  fn _ZN16QStringListModelC1ERK11QStringListP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QStringListModel::insertRows(int row, int count, const QModelIndex & parent);
  fn _ZN16QStringListModel10insertRowsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  QVariant QStringListModel::data(const QModelIndex & index, int role);
  fn _ZNK16QStringListModel4dataERK11QModelIndexi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QStringList QStringListModel::stringList();
  fn _ZNK16QStringListModel10stringListEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QStringListModel::metaObject();
  fn _ZNK16QStringListModel10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QStringListModel::removeRows(int row, int count, const QModelIndex & parent);
  fn _ZN16QStringListModel10removeRowsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  void QStringListModel::NewQStringListModel(QObject * parent);
  fn _ZN16QStringListModelC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStringListModel::NewQStringListModel(const QStringListModel & );
  fn _ZN16QStringListModelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QStringListModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn _ZN16QStringListModel7setDataERK11QModelIndexRK8QVarianti(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> int8_t;
  // proto:  int QStringListModel::rowCount(const QModelIndex & parent);
  fn _ZNK16QStringListModel8rowCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QModelIndex QStringListModel::sibling(int row, int column, const QModelIndex & idx);
  fn _ZNK16QStringListModel7siblingEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QStringListModel::setStringList(const QStringList & strings);
  fn _ZN16QStringListModel13setStringListERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QStringListModel)=1
pub struct QStringListModel {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStringListModel {
  pub fn NewQStringListModel<T: QStringListModel_NewQStringListModel>(value: T) -> QStringListModel {
    let rsthis = value.NewQStringListModel();
    return rsthis;
    // return 1;
  }
}

pub trait QStringListModel_NewQStringListModel {
  fn NewQStringListModel(self) -> QStringListModel;
}

// proto: void QStringListModel::NewQStringListModel(const QStringList & strings, QObject * parent);
impl<'a> /*trait*/ QStringListModel_NewQStringListModel for (&'a  QStringList, &'a mut QObject) {
  fn NewQStringListModel(self) -> QStringListModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStringListModelC1ERK11QStringListP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN16QStringListModelC1ERK11QStringListP7QObject(qthis, arg0, arg1)};
    let rsthis = QStringListModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStringListModel {
  pub fn insertRows<RetType, T: QStringListModel_insertRows<RetType>>(&mut self, value: T) -> RetType {
    return value.insertRows(self);
    // return 1;
  }
}

pub trait QStringListModel_insertRows<RetType> {
  fn insertRows(self, rsthis: &mut QStringListModel) -> RetType;
}

// proto:  bool QStringListModel::insertRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStringListModel_insertRows<i8> for (i32, i32, &'a  QModelIndex) {
  fn insertRows(self, rsthis: &mut QStringListModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStringListModel10insertRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QStringListModel10insertRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStringListModel {
  pub fn data<RetType, T: QStringListModel_data<RetType>>(&mut self, value: T) -> RetType {
    return value.data(self);
    // return 1;
  }
}

pub trait QStringListModel_data<RetType> {
  fn data(self, rsthis: &mut QStringListModel) -> RetType;
}

// proto:  QVariant QStringListModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QStringListModel_data<QVariant> for (&'a  QModelIndex, i32) {
  fn data(self, rsthis: &mut QStringListModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QStringListModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK16QStringListModel4dataERK11QModelIndexi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStringListModel {
  pub fn stringList<RetType, T: QStringListModel_stringList<RetType>>(&mut self, value: T) -> RetType {
    return value.stringList(self);
    // return 1;
  }
}

pub trait QStringListModel_stringList<RetType> {
  fn stringList(self, rsthis: &mut QStringListModel) -> RetType;
}

// proto:  QStringList QStringListModel::stringList();
impl<'a> /*trait*/ QStringListModel_stringList<()> for () {
  fn stringList(self, rsthis: &mut QStringListModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QStringListModel10stringListEv()};
     unsafe {_ZNK16QStringListModel10stringListEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStringListModel {
  pub fn metaObject<RetType, T: QStringListModel_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QStringListModel_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QStringListModel) -> RetType;
}

// proto:  const QMetaObject * QStringListModel::metaObject();
impl<'a> /*trait*/ QStringListModel_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QStringListModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QStringListModel10metaObjectEv()};
     unsafe {_ZNK16QStringListModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStringListModel {
  pub fn removeRows<RetType, T: QStringListModel_removeRows<RetType>>(&mut self, value: T) -> RetType {
    return value.removeRows(self);
    // return 1;
  }
}

pub trait QStringListModel_removeRows<RetType> {
  fn removeRows(self, rsthis: &mut QStringListModel) -> RetType;
}

// proto:  bool QStringListModel::removeRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStringListModel_removeRows<i8> for (i32, i32, &'a  QModelIndex) {
  fn removeRows(self, rsthis: &mut QStringListModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStringListModel10removeRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QStringListModel10removeRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QStringListModel::NewQStringListModel(QObject * parent);
impl<'a> /*trait*/ QStringListModel_NewQStringListModel for (&'a mut QObject) {
  fn NewQStringListModel(self) -> QStringListModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStringListModelC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QStringListModelC1EP7QObject(qthis, arg0)};
    let rsthis = QStringListModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStringListModel::NewQStringListModel(const QStringListModel & );
impl<'a> /*trait*/ QStringListModel_NewQStringListModel for (&'a  QStringListModel) {
  fn NewQStringListModel(self) -> QStringListModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStringListModelC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QStringListModelC1ERKS_(qthis, arg0)};
    let rsthis = QStringListModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStringListModel {
  pub fn setData<RetType, T: QStringListModel_setData<RetType>>(&mut self, value: T) -> RetType {
    return value.setData(self);
    // return 1;
  }
}

pub trait QStringListModel_setData<RetType> {
  fn setData(self, rsthis: &mut QStringListModel) -> RetType;
}

// proto:  bool QStringListModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QStringListModel_setData<i8> for (&'a  QModelIndex, &'a  QVariant, i32) {
  fn setData(self, rsthis: &mut QStringListModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStringListModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN16QStringListModel7setDataERK11QModelIndexRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStringListModel {
  pub fn rowCount<RetType, T: QStringListModel_rowCount<RetType>>(&mut self, value: T) -> RetType {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QStringListModel_rowCount<RetType> {
  fn rowCount(self, rsthis: &mut QStringListModel) -> RetType;
}

// proto:  int QStringListModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QStringListModel_rowCount<i32> for (&'a  QModelIndex) {
  fn rowCount(self, rsthis: &mut QStringListModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QStringListModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QStringListModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStringListModel {
  pub fn sibling<RetType, T: QStringListModel_sibling<RetType>>(&mut self, value: T) -> RetType {
    return value.sibling(self);
    // return 1;
  }
}

pub trait QStringListModel_sibling<RetType> {
  fn sibling(self, rsthis: &mut QStringListModel) -> RetType;
}

// proto:  QModelIndex QStringListModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QStringListModel_sibling<QModelIndex> for (i32, i32, &'a  QModelIndex) {
  fn sibling(self, rsthis: &mut QStringListModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QStringListModel7siblingEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QStringListModel7siblingEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStringListModel {
  pub fn setStringList<RetType, T: QStringListModel_setStringList<RetType>>(&mut self, value: T) -> RetType {
    return value.setStringList(self);
    // return 1;
  }
}

pub trait QStringListModel_setStringList<RetType> {
  fn setStringList(self, rsthis: &mut QStringListModel) -> RetType;
}

// proto:  void QStringListModel::setStringList(const QStringList & strings);
impl<'a> /*trait*/ QStringListModel_setStringList<()> for (&'a  QStringList) {
  fn setStringList(self, rsthis: &mut QStringListModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStringListModel13setStringListERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QStringListModel13setStringListERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

