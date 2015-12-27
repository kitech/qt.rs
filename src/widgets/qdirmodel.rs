// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
// src-file: /QtWidgets/qdirmodel.h
// dst-file: /src/widgets/qdirmodel.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::super::core::qabstractitemmodel::QAbstractItemModel; // 771
use std::ops::Deref;
use super::super::core::qobject::QObject; // 771
use super::qfileiconprovider::QFileIconProvider; // 773
use super::super::core::qabstractitemmodel::QModelIndex; // 771
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qstringlist::QStringList; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qmimedata::QMimeData; // 771
use super::super::gui::qicon::QIcon; // 771
use super::super::core::qfileinfo::QFileInfo; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDirModel_Class_Size() -> c_int;
  // proto:  QFileIconProvider * QDirModel::iconProvider();
  fn _ZNK9QDirModel12iconProviderEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QModelIndex QDirModel::parent(const QModelIndex & child);
  fn _ZNK9QDirModel6parentERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QVariant QDirModel::data(const QModelIndex & index, int role);
  fn _ZNK9QDirModel4dataERK11QModelIndexi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QStringList QDirModel::nameFilters();
  fn _ZNK9QDirModel11nameFiltersEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QDirModel::isReadOnly();
  fn _ZNK9QDirModel10isReadOnlyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QDirModel::columnCount(const QModelIndex & parent);
  fn _ZNK9QDirModel11columnCountERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QStringList QDirModel::mimeTypes();
  fn _ZNK9QDirModel9mimeTypesEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDirModel::~QDirModel();
  fn _ZN9QDirModelD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QDirModel::remove(const QModelIndex & index);
  fn _ZN9QDirModel6removeERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QString QDirModel::fileName(const QModelIndex & index);
  fn _ZNK9QDirModel8fileNameERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QDirModel::metaObject();
  fn _ZNK9QDirModel10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QDirModel::resolveSymlinks();
  fn _ZNK9QDirModel15resolveSymlinksEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QDirModel::refresh(const QModelIndex & parent);
  fn _ZN9QDirModel7refreshERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDirModel::setNameFilters(const QStringList & filters);
  fn _ZN9QDirModel14setNameFiltersERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDirModel::setIconProvider(QFileIconProvider * provider);
  fn _ZN9QDirModel15setIconProviderEP17QFileIconProvider(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QDirModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK9QDirModel5indexEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QDirModel::QDirModel(QObject * parent);
  fn dector_ZN9QDirModelC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QDirModelC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QDirModel::index(const QString & path, int column);
  fn _ZNK9QDirModel5indexERK7QStringi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  bool QDirModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn _ZN9QDirModel7setDataERK11QModelIndexRK8QVarianti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> c_char;
  // proto:  void QDirModel::setLazyChildCount(bool enable);
  fn _ZN9QDirModel17setLazyChildCountEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QIcon QDirModel::fileIcon(const QModelIndex & index);
  fn _ZNK9QDirModel8fileIconERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QDirModel::hasChildren(const QModelIndex & index);
  fn _ZNK9QDirModel11hasChildrenERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QDirModel::isDir(const QModelIndex & index);
  fn _ZNK9QDirModel5isDirERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QModelIndex QDirModel::mkdir(const QModelIndex & parent, const QString & name);
  fn _ZN9QDirModel5mkdirERK11QModelIndexRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QDirModel::rmdir(const QModelIndex & index);
  fn _ZN9QDirModel5rmdirERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QString QDirModel::filePath(const QModelIndex & index);
  fn _ZNK9QDirModel8filePathERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QDirModel::rowCount(const QModelIndex & parent);
  fn _ZNK9QDirModel8rowCountERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QDirModel::setReadOnly(bool enable);
  fn _ZN9QDirModel11setReadOnlyEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QDirModel::QDirModel(const QDirModel & );
  fn dector_ZN9QDirModelC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QDirModelC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDirModel::setResolveSymlinks(bool enable);
  fn _ZN9QDirModel18setResolveSymlinksEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QDirModel::lazyChildCount();
  fn _ZNK9QDirModel14lazyChildCountEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QFileInfo QDirModel::fileInfo(const QModelIndex & index);
  fn _ZNK9QDirModel8fileInfoERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QDirModel)=1
#[derive(Default)]
pub struct QDirModel {
  qbase: QAbstractItemModel,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QDirModel {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDirModel {
    return QDirModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDirModel {
  type Target = QAbstractItemModel;

  fn deref(&self) -> &QAbstractItemModel {
    return & self.qbase;
  }
}
impl AsRef<QAbstractItemModel> for QDirModel {
  fn as_ref(& self) -> & QAbstractItemModel {
    return & self.qbase;
  }
}
  // proto:  QFileIconProvider * QDirModel::iconProvider();
impl /*struct*/ QDirModel {
  pub fn iconProvider<RetType, T: QDirModel_iconProvider<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconProvider(self);
    // return 1;
  }
}

pub trait QDirModel_iconProvider<RetType> {
  fn iconProvider(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  QFileIconProvider * QDirModel::iconProvider();
impl<'a> /*trait*/ QDirModel_iconProvider<QFileIconProvider> for () {
  fn iconProvider(self , rsthis: & QDirModel) -> QFileIconProvider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel12iconProviderEv()};
    let mut ret = unsafe {_ZNK9QDirModel12iconProviderEv(rsthis.qclsinst)};
    let mut ret1 = QFileIconProvider::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QDirModel::parent(const QModelIndex & child);
impl /*struct*/ QDirModel {
  pub fn parent<RetType, T: QDirModel_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QDirModel_parent<RetType> {
  fn parent(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  QModelIndex QDirModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QDirModel_parent<QModelIndex> for (&'a QModelIndex) {
  fn parent(self , rsthis: & QDirModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QDirModel::data(const QModelIndex & index, int role);
impl /*struct*/ QDirModel {
  pub fn data<RetType, T: QDirModel_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QDirModel_data<RetType> {
  fn data(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  QVariant QDirModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QDirModel_data<QVariant> for (&'a QModelIndex, i32) {
  fn data(self , rsthis: & QDirModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK9QDirModel4dataERK11QModelIndexi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QDirModel::nameFilters();
impl /*struct*/ QDirModel {
  pub fn nameFilters<RetType, T: QDirModel_nameFilters<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nameFilters(self);
    // return 1;
  }
}

pub trait QDirModel_nameFilters<RetType> {
  fn nameFilters(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  QStringList QDirModel::nameFilters();
impl<'a> /*trait*/ QDirModel_nameFilters<()> for () {
  fn nameFilters(self , rsthis: & QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel11nameFiltersEv()};
     unsafe {_ZNK9QDirModel11nameFiltersEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QDirModel::isReadOnly();
impl /*struct*/ QDirModel {
  pub fn isReadOnly<RetType, T: QDirModel_isReadOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly(self);
    // return 1;
  }
}

pub trait QDirModel_isReadOnly<RetType> {
  fn isReadOnly(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  bool QDirModel::isReadOnly();
impl<'a> /*trait*/ QDirModel_isReadOnly<i8> for () {
  fn isReadOnly(self , rsthis: & QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK9QDirModel10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QDirModel::columnCount(const QModelIndex & parent);
impl /*struct*/ QDirModel {
  pub fn columnCount<RetType, T: QDirModel_columnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QDirModel_columnCount<RetType> {
  fn columnCount(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  int QDirModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QDirModel_columnCount<i32> for (&'a QModelIndex) {
  fn columnCount(self , rsthis: & QDirModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QStringList QDirModel::mimeTypes();
impl /*struct*/ QDirModel {
  pub fn mimeTypes<RetType, T: QDirModel_mimeTypes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes(self);
    // return 1;
  }
}

pub trait QDirModel_mimeTypes<RetType> {
  fn mimeTypes(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  QStringList QDirModel::mimeTypes();
impl<'a> /*trait*/ QDirModel_mimeTypes<()> for () {
  fn mimeTypes(self , rsthis: & QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel9mimeTypesEv()};
     unsafe {_ZNK9QDirModel9mimeTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDirModel::~QDirModel();
impl /*struct*/ QDirModel {
  pub fn Free<RetType, T: QDirModel_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDirModel_Free<RetType> {
  fn Free(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  void QDirModel::~QDirModel();
impl<'a> /*trait*/ QDirModel_Free<()> for () {
  fn Free(self , rsthis: & QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModelD0Ev()};
     unsafe {_ZN9QDirModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QDirModel::remove(const QModelIndex & index);
impl /*struct*/ QDirModel {
  pub fn remove<RetType, T: QDirModel_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QDirModel_remove<RetType> {
  fn remove(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  bool QDirModel::remove(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_remove<i8> for (&'a QModelIndex) {
  fn remove(self , rsthis: & QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel6removeERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDirModel6removeERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QDirModel::fileName(const QModelIndex & index);
impl /*struct*/ QDirModel {
  pub fn fileName<RetType, T: QDirModel_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QDirModel_fileName<RetType> {
  fn fileName(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  QString QDirModel::fileName(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_fileName<QString> for (&'a QModelIndex) {
  fn fileName(self , rsthis: & QDirModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8fileNameERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8fileNameERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QDirModel::metaObject();
impl /*struct*/ QDirModel {
  pub fn metaObject<RetType, T: QDirModel_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDirModel_metaObject<RetType> {
  fn metaObject(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  const QMetaObject * QDirModel::metaObject();
impl<'a> /*trait*/ QDirModel_metaObject<()> for () {
  fn metaObject(self , rsthis: & QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel10metaObjectEv()};
     unsafe {_ZNK9QDirModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QDirModel::resolveSymlinks();
impl /*struct*/ QDirModel {
  pub fn resolveSymlinks<RetType, T: QDirModel_resolveSymlinks<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resolveSymlinks(self);
    // return 1;
  }
}

pub trait QDirModel_resolveSymlinks<RetType> {
  fn resolveSymlinks(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  bool QDirModel::resolveSymlinks();
impl<'a> /*trait*/ QDirModel_resolveSymlinks<i8> for () {
  fn resolveSymlinks(self , rsthis: & QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel15resolveSymlinksEv()};
    let mut ret = unsafe {_ZNK9QDirModel15resolveSymlinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QDirModel::refresh(const QModelIndex & parent);
impl /*struct*/ QDirModel {
  pub fn refresh<RetType, T: QDirModel_refresh<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.refresh(self);
    // return 1;
  }
}

pub trait QDirModel_refresh<RetType> {
  fn refresh(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  void QDirModel::refresh(const QModelIndex & parent);
impl<'a> /*trait*/ QDirModel_refresh<()> for (&'a QModelIndex) {
  fn refresh(self , rsthis: & QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel7refreshERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDirModel7refreshERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDirModel::setNameFilters(const QStringList & filters);
impl /*struct*/ QDirModel {
  pub fn setNameFilters<RetType, T: QDirModel_setNameFilters<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNameFilters(self);
    // return 1;
  }
}

pub trait QDirModel_setNameFilters<RetType> {
  fn setNameFilters(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  void QDirModel::setNameFilters(const QStringList & filters);
impl<'a> /*trait*/ QDirModel_setNameFilters<()> for (&'a QStringList) {
  fn setNameFilters(self , rsthis: & QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel14setNameFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDirModel14setNameFiltersERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDirModel::setIconProvider(QFileIconProvider * provider);
impl /*struct*/ QDirModel {
  pub fn setIconProvider<RetType, T: QDirModel_setIconProvider<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIconProvider(self);
    // return 1;
  }
}

pub trait QDirModel_setIconProvider<RetType> {
  fn setIconProvider(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  void QDirModel::setIconProvider(QFileIconProvider * provider);
impl<'a> /*trait*/ QDirModel_setIconProvider<()> for (&'a QFileIconProvider) {
  fn setIconProvider(self , rsthis: & QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel15setIconProviderEP17QFileIconProvider()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDirModel15setIconProviderEP17QFileIconProvider(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QDirModel::index(int row, int column, const QModelIndex & parent);
impl /*struct*/ QDirModel {
  pub fn index<RetType, T: QDirModel_index<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QDirModel_index<RetType> {
  fn index(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  QModelIndex QDirModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QDirModel_index<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn index(self , rsthis: & QDirModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDirModel::QDirModel(QObject * parent);
impl /*struct*/ QDirModel {
  pub fn New<T: QDirModel_New>(value: T) -> QDirModel {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDirModel_New {
  fn New(self) -> QDirModel;
}

  // proto:  void QDirModel::QDirModel(QObject * parent);
impl<'a> /*trait*/ QDirModel_New for (&'a QObject) {
  fn New(self) -> QDirModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModelC1EP7QObject()};
    let ctysz: c_int = unsafe{QDirModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QDirModelC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QDirModelC1EP7QObject(arg0)} as u64;
    let rsthis = QDirModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QModelIndex QDirModel::index(const QString & path, int column);
impl<'a> /*trait*/ QDirModel_index<QModelIndex> for (&'a QString, i32) {
  fn index(self , rsthis: & QDirModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel5indexERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK9QDirModel5indexERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDirModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl /*struct*/ QDirModel {
  pub fn setData<RetType, T: QDirModel_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QDirModel_setData<RetType> {
  fn setData(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  bool QDirModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QDirModel_setData<i8> for (&'a QModelIndex, &'a QVariant, i32) {
  fn setData(self , rsthis: & QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN9QDirModel7setDataERK11QModelIndexRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QDirModel::setLazyChildCount(bool enable);
impl /*struct*/ QDirModel {
  pub fn setLazyChildCount<RetType, T: QDirModel_setLazyChildCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLazyChildCount(self);
    // return 1;
  }
}

pub trait QDirModel_setLazyChildCount<RetType> {
  fn setLazyChildCount(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  void QDirModel::setLazyChildCount(bool enable);
impl<'a> /*trait*/ QDirModel_setLazyChildCount<()> for (i8) {
  fn setLazyChildCount(self , rsthis: & QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel17setLazyChildCountEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QDirModel17setLazyChildCountEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIcon QDirModel::fileIcon(const QModelIndex & index);
impl /*struct*/ QDirModel {
  pub fn fileIcon<RetType, T: QDirModel_fileIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileIcon(self);
    // return 1;
  }
}

pub trait QDirModel_fileIcon<RetType> {
  fn fileIcon(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  QIcon QDirModel::fileIcon(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_fileIcon<QIcon> for (&'a QModelIndex) {
  fn fileIcon(self , rsthis: & QDirModel) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8fileIconERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8fileIconERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDirModel::hasChildren(const QModelIndex & index);
impl /*struct*/ QDirModel {
  pub fn hasChildren<RetType, T: QDirModel_hasChildren<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasChildren(self);
    // return 1;
  }
}

pub trait QDirModel_hasChildren<RetType> {
  fn hasChildren(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  bool QDirModel::hasChildren(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_hasChildren<i8> for (&'a QModelIndex) {
  fn hasChildren(self , rsthis: & QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel11hasChildrenERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QDirModel::isDir(const QModelIndex & index);
impl /*struct*/ QDirModel {
  pub fn isDir<RetType, T: QDirModel_isDir<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDir(self);
    // return 1;
  }
}

pub trait QDirModel_isDir<RetType> {
  fn isDir(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  bool QDirModel::isDir(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_isDir<i8> for (&'a QModelIndex) {
  fn isDir(self , rsthis: & QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel5isDirERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel5isDirERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QModelIndex QDirModel::mkdir(const QModelIndex & parent, const QString & name);
impl /*struct*/ QDirModel {
  pub fn mkdir<RetType, T: QDirModel_mkdir<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mkdir(self);
    // return 1;
  }
}

pub trait QDirModel_mkdir<RetType> {
  fn mkdir(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  QModelIndex QDirModel::mkdir(const QModelIndex & parent, const QString & name);
impl<'a> /*trait*/ QDirModel_mkdir<QModelIndex> for (&'a QModelIndex, &'a QString) {
  fn mkdir(self , rsthis: & QDirModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel5mkdirERK11QModelIndexRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDirModel5mkdirERK11QModelIndexRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDirModel::rmdir(const QModelIndex & index);
impl /*struct*/ QDirModel {
  pub fn rmdir<RetType, T: QDirModel_rmdir<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rmdir(self);
    // return 1;
  }
}

pub trait QDirModel_rmdir<RetType> {
  fn rmdir(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  bool QDirModel::rmdir(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_rmdir<i8> for (&'a QModelIndex) {
  fn rmdir(self , rsthis: & QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel5rmdirERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDirModel5rmdirERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QDirModel::filePath(const QModelIndex & index);
impl /*struct*/ QDirModel {
  pub fn filePath<RetType, T: QDirModel_filePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.filePath(self);
    // return 1;
  }
}

pub trait QDirModel_filePath<RetType> {
  fn filePath(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  QString QDirModel::filePath(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_filePath<QString> for (&'a QModelIndex) {
  fn filePath(self , rsthis: & QDirModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8filePathERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8filePathERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QDirModel::rowCount(const QModelIndex & parent);
impl /*struct*/ QDirModel {
  pub fn rowCount<RetType, T: QDirModel_rowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QDirModel_rowCount<RetType> {
  fn rowCount(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  int QDirModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QDirModel_rowCount<i32> for (&'a QModelIndex) {
  fn rowCount(self , rsthis: & QDirModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QDirModel::setReadOnly(bool enable);
impl /*struct*/ QDirModel {
  pub fn setReadOnly<RetType, T: QDirModel_setReadOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly(self);
    // return 1;
  }
}

pub trait QDirModel_setReadOnly<RetType> {
  fn setReadOnly(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  void QDirModel::setReadOnly(bool enable);
impl<'a> /*trait*/ QDirModel_setReadOnly<()> for (i8) {
  fn setReadOnly(self , rsthis: & QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel11setReadOnlyEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QDirModel11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDirModel::QDirModel(const QDirModel & );
impl<'a> /*trait*/ QDirModel_New for (&'a QDirModel) {
  fn New(self) -> QDirModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModelC1ERKS_()};
    let ctysz: c_int = unsafe{QDirModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QDirModelC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QDirModelC1ERKS_(arg0)} as u64;
    let rsthis = QDirModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDirModel::setResolveSymlinks(bool enable);
impl /*struct*/ QDirModel {
  pub fn setResolveSymlinks<RetType, T: QDirModel_setResolveSymlinks<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setResolveSymlinks(self);
    // return 1;
  }
}

pub trait QDirModel_setResolveSymlinks<RetType> {
  fn setResolveSymlinks(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  void QDirModel::setResolveSymlinks(bool enable);
impl<'a> /*trait*/ QDirModel_setResolveSymlinks<()> for (i8) {
  fn setResolveSymlinks(self , rsthis: & QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel18setResolveSymlinksEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QDirModel18setResolveSymlinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QDirModel::lazyChildCount();
impl /*struct*/ QDirModel {
  pub fn lazyChildCount<RetType, T: QDirModel_lazyChildCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lazyChildCount(self);
    // return 1;
  }
}

pub trait QDirModel_lazyChildCount<RetType> {
  fn lazyChildCount(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  bool QDirModel::lazyChildCount();
impl<'a> /*trait*/ QDirModel_lazyChildCount<i8> for () {
  fn lazyChildCount(self , rsthis: & QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel14lazyChildCountEv()};
    let mut ret = unsafe {_ZNK9QDirModel14lazyChildCountEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QFileInfo QDirModel::fileInfo(const QModelIndex & index);
impl /*struct*/ QDirModel {
  pub fn fileInfo<RetType, T: QDirModel_fileInfo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileInfo(self);
    // return 1;
  }
}

pub trait QDirModel_fileInfo<RetType> {
  fn fileInfo(self , rsthis: & QDirModel) -> RetType;
}

  // proto:  QFileInfo QDirModel::fileInfo(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_fileInfo<QFileInfo> for (&'a QModelIndex) {
  fn fileInfo(self , rsthis: & QDirModel) -> QFileInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8fileInfoERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8fileInfoERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QFileInfo::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

