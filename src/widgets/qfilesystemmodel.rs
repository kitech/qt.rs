// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtWidgets/qfilesystemmodel.h
// dst-file: /src/widgets/qfilesystemmodel.rs
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
use super::super::core::qabstractitemmodel::QModelIndex; // 771
use super::super::core::qstring::QString; // 771
use super::super::gui::qicon::QIcon; // 771
use super::qfileiconprovider::QFileIconProvider; // 773
use super::super::core::qstringlist::QStringList; // 771
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qdir::QDir; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qfileinfo::QFileInfo; // 771
use super::super::core::qmimedata::QMimeData; // 771
use super::super::core::qdatetime::QDateTime; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFileSystemModel_Class_Size() -> c_int;
  // proto:  QString QFileSystemModel::fileName(const QModelIndex & index);
  fn demth_ZNK16QFileSystemModel8fileNameERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFileSystemModel::hasChildren(const QModelIndex & parent);
  fn _ZNK16QFileSystemModel11hasChildrenERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QStringList QFileSystemModel::mimeTypes();
  fn _ZNK16QFileSystemModel9mimeTypesEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QFileSystemModel::~QFileSystemModel();
  fn _ZN16QFileSystemModelD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QModelIndex QFileSystemModel::index(const QString & path, int column);
  fn _ZNK16QFileSystemModel5indexERK7QStringi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  void QFileSystemModel::setNameFilterDisables(bool enable);
  fn _ZN16QFileSystemModel21setNameFilterDisablesEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QIcon QFileSystemModel::fileIcon(const QModelIndex & index);
  fn demth_ZNK16QFileSystemModel8fileIconERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFileSystemModel::resolveSymlinks();
  fn _ZNK16QFileSystemModel15resolveSymlinksEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QFileSystemModel::filePath(const QModelIndex & index);
  fn _ZNK16QFileSystemModel8filePathERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QFileSystemModel::parent(const QModelIndex & child);
  fn _ZNK16QFileSystemModel6parentERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFileSystemModel::nameFilterDisables();
  fn _ZNK16QFileSystemModel18nameFilterDisablesEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QFileSystemModel::metaObject();
  fn _ZNK16QFileSystemModel10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QFileSystemModel::fetchMore(const QModelIndex & parent);
  fn _ZN16QFileSystemModel9fetchMoreERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QFileSystemModel::QFileSystemModel(const QFileSystemModel & );
  fn dector_ZN16QFileSystemModelC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QFileSystemModelC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qint64 QFileSystemModel::size(const QModelIndex & index);
  fn _ZNK16QFileSystemModel4sizeERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_longlong;
  // proto:  QFileIconProvider * QFileSystemModel::iconProvider();
  fn _ZNK16QFileSystemModel12iconProviderEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFileSystemModel::setNameFilters(const QStringList & filters);
  fn _ZN16QFileSystemModel14setNameFiltersERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QVariant QFileSystemModel::data(const QModelIndex & index, int role);
  fn _ZNK16QFileSystemModel4dataERK11QModelIndexi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QDir QFileSystemModel::rootDirectory();
  fn _ZNK16QFileSystemModel13rootDirectoryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QModelIndex QFileSystemModel::mkdir(const QModelIndex & parent, const QString & name);
  fn _ZN16QFileSystemModel5mkdirERK11QModelIndexRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  int QFileSystemModel::rowCount(const QModelIndex & parent);
  fn _ZNK16QFileSystemModel8rowCountERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  bool QFileSystemModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn _ZN16QFileSystemModel7setDataERK11QModelIndexRK8QVarianti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> c_char;
  // proto:  int QFileSystemModel::columnCount(const QModelIndex & parent);
  fn _ZNK16QFileSystemModel11columnCountERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QModelIndex QFileSystemModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK16QFileSystemModel5indexEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  bool QFileSystemModel::canFetchMore(const QModelIndex & parent);
  fn _ZNK16QFileSystemModel12canFetchMoreERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QFileSystemModel::remove(const QModelIndex & index);
  fn _ZN16QFileSystemModel6removeERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QFileSystemModel::rootPathChanged(const QString & newPath);
  fn _ZN16QFileSystemModel15rootPathChangedERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QFileSystemModel::setRootPath(const QString & path);
  fn _ZN16QFileSystemModel11setRootPathERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFileSystemModel::directoryLoaded(const QString & path);
  fn _ZN16QFileSystemModel15directoryLoadedERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QFileSystemModel::setResolveSymlinks(bool enable);
  fn _ZN16QFileSystemModel18setResolveSymlinksEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QFileSystemModel::setReadOnly(bool enable);
  fn _ZN16QFileSystemModel11setReadOnlyEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QStringList QFileSystemModel::nameFilters();
  fn _ZNK16QFileSystemModel11nameFiltersEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QFileSystemModel::QFileSystemModel(QObject * parent);
  fn dector_ZN16QFileSystemModelC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN16QFileSystemModelC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QFileInfo QFileSystemModel::fileInfo(const QModelIndex & index);
  fn demth_ZNK16QFileSystemModel8fileInfoERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QVariant QFileSystemModel::myComputer(int role);
  fn _ZNK16QFileSystemModel10myComputerEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QFileSystemModel::isReadOnly();
  fn _ZNK16QFileSystemModel10isReadOnlyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QFileSystemModel::type(const QModelIndex & index);
  fn _ZNK16QFileSystemModel4typeERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QFileSystemModel::rootPath();
  fn _ZNK16QFileSystemModel8rootPathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QDateTime QFileSystemModel::lastModified(const QModelIndex & index);
  fn _ZNK16QFileSystemModel12lastModifiedERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFileSystemModel::fileRenamed(const QString & path, const QString & oldName, const QString & newName);
  fn _ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  bool QFileSystemModel::isDir(const QModelIndex & index);
  fn _ZNK16QFileSystemModel5isDirERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QFileSystemModel::rmdir(const QModelIndex & index);
  fn _ZN16QFileSystemModel5rmdirERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QFileSystemModel::setIconProvider(QFileIconProvider * provider);
  fn _ZN16QFileSystemModel15setIconProviderEP17QFileIconProvider(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QFileSystemModel_SlotProxy_connect__ZN16QFileSystemModel15directoryLoadedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFileSystemModel_SlotProxy_connect_box__ZN16QFileSystemModel15directoryLoadedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFileSystemModel_SlotProxy_connect__ZN16QFileSystemModel15rootPathChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFileSystemModel_SlotProxy_connect_box__ZN16QFileSystemModel15rootPathChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFileSystemModel_SlotProxy_connect__ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QFileSystemModel_SlotProxy_connect_box__ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFileSystemModel)=1
#[derive(Default)]
pub struct QFileSystemModel {
  qbase: QAbstractItemModel,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _rootPathChanged_1: QFileSystemModel_rootPathChanged_signal,
  pub _directoryLoaded_1: QFileSystemModel_directoryLoaded_signal,
  pub _fileRenamed_1: QFileSystemModel_fileRenamed_signal,
}

impl /*struct*/ QFileSystemModel {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFileSystemModel {
    return QFileSystemModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFileSystemModel {
  type Target = QAbstractItemModel;

  fn deref(&self) -> &QAbstractItemModel {
    return & self.qbase;
  }
}
impl AsRef<QAbstractItemModel> for QFileSystemModel {
  fn as_ref(& self) -> & QAbstractItemModel {
    return & self.qbase;
  }
}
  // proto:  QString QFileSystemModel::fileName(const QModelIndex & index);
impl /*struct*/ QFileSystemModel {
  pub fn fileName<RetType, T: QFileSystemModel_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QFileSystemModel_fileName<RetType> {
  fn fileName(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QString QFileSystemModel::fileName(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_fileName<QString> for (&'a QModelIndex) {
  fn fileName(self , rsthis: & QFileSystemModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8fileNameERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZNK16QFileSystemModel8fileNameERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFileSystemModel::hasChildren(const QModelIndex & parent);
impl /*struct*/ QFileSystemModel {
  pub fn hasChildren<RetType, T: QFileSystemModel_hasChildren<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasChildren(self);
    // return 1;
  }
}

pub trait QFileSystemModel_hasChildren<RetType> {
  fn hasChildren(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  bool QFileSystemModel::hasChildren(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_hasChildren<i8> for (&'a QModelIndex) {
  fn hasChildren(self , rsthis: & QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel11hasChildrenERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QStringList QFileSystemModel::mimeTypes();
impl /*struct*/ QFileSystemModel {
  pub fn mimeTypes<RetType, T: QFileSystemModel_mimeTypes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes(self);
    // return 1;
  }
}

pub trait QFileSystemModel_mimeTypes<RetType> {
  fn mimeTypes(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QStringList QFileSystemModel::mimeTypes();
impl<'a> /*trait*/ QFileSystemModel_mimeTypes<()> for () {
  fn mimeTypes(self , rsthis: & QFileSystemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel9mimeTypesEv()};
     unsafe {_ZNK16QFileSystemModel9mimeTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFileSystemModel::~QFileSystemModel();
impl /*struct*/ QFileSystemModel {
  pub fn Free<RetType, T: QFileSystemModel_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QFileSystemModel_Free<RetType> {
  fn Free(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  void QFileSystemModel::~QFileSystemModel();
impl<'a> /*trait*/ QFileSystemModel_Free<()> for () {
  fn Free(self , rsthis: & QFileSystemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModelD0Ev()};
     unsafe {_ZN16QFileSystemModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndex QFileSystemModel::index(const QString & path, int column);
impl /*struct*/ QFileSystemModel {
  pub fn index<RetType, T: QFileSystemModel_index<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QFileSystemModel_index<RetType> {
  fn index(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QModelIndex QFileSystemModel::index(const QString & path, int column);
impl<'a> /*trait*/ QFileSystemModel_index<QModelIndex> for (&'a QString, i32) {
  fn index(self , rsthis: & QFileSystemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel5indexERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK16QFileSystemModel5indexERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileSystemModel::setNameFilterDisables(bool enable);
impl /*struct*/ QFileSystemModel {
  pub fn setNameFilterDisables<RetType, T: QFileSystemModel_setNameFilterDisables<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNameFilterDisables(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setNameFilterDisables<RetType> {
  fn setNameFilterDisables(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  void QFileSystemModel::setNameFilterDisables(bool enable);
impl<'a> /*trait*/ QFileSystemModel_setNameFilterDisables<()> for (i8) {
  fn setNameFilterDisables(self , rsthis: & QFileSystemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel21setNameFilterDisablesEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN16QFileSystemModel21setNameFilterDisablesEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIcon QFileSystemModel::fileIcon(const QModelIndex & index);
impl /*struct*/ QFileSystemModel {
  pub fn fileIcon<RetType, T: QFileSystemModel_fileIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileIcon(self);
    // return 1;
  }
}

pub trait QFileSystemModel_fileIcon<RetType> {
  fn fileIcon(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QIcon QFileSystemModel::fileIcon(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_fileIcon<QIcon> for (&'a QModelIndex) {
  fn fileIcon(self , rsthis: & QFileSystemModel) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8fileIconERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZNK16QFileSystemModel8fileIconERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFileSystemModel::resolveSymlinks();
impl /*struct*/ QFileSystemModel {
  pub fn resolveSymlinks<RetType, T: QFileSystemModel_resolveSymlinks<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resolveSymlinks(self);
    // return 1;
  }
}

pub trait QFileSystemModel_resolveSymlinks<RetType> {
  fn resolveSymlinks(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  bool QFileSystemModel::resolveSymlinks();
impl<'a> /*trait*/ QFileSystemModel_resolveSymlinks<i8> for () {
  fn resolveSymlinks(self , rsthis: & QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel15resolveSymlinksEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel15resolveSymlinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QFileSystemModel::filePath(const QModelIndex & index);
impl /*struct*/ QFileSystemModel {
  pub fn filePath<RetType, T: QFileSystemModel_filePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.filePath(self);
    // return 1;
  }
}

pub trait QFileSystemModel_filePath<RetType> {
  fn filePath(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QString QFileSystemModel::filePath(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_filePath<QString> for (&'a QModelIndex) {
  fn filePath(self , rsthis: & QFileSystemModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8filePathERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel8filePathERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QFileSystemModel::parent(const QModelIndex & child);
impl /*struct*/ QFileSystemModel {
  pub fn parent<RetType, T: QFileSystemModel_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QFileSystemModel_parent<RetType> {
  fn parent(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QModelIndex QFileSystemModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QFileSystemModel_parent<QModelIndex> for (&'a QModelIndex) {
  fn parent(self , rsthis: & QFileSystemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFileSystemModel::nameFilterDisables();
impl /*struct*/ QFileSystemModel {
  pub fn nameFilterDisables<RetType, T: QFileSystemModel_nameFilterDisables<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nameFilterDisables(self);
    // return 1;
  }
}

pub trait QFileSystemModel_nameFilterDisables<RetType> {
  fn nameFilterDisables(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  bool QFileSystemModel::nameFilterDisables();
impl<'a> /*trait*/ QFileSystemModel_nameFilterDisables<i8> for () {
  fn nameFilterDisables(self , rsthis: & QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel18nameFilterDisablesEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel18nameFilterDisablesEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QFileSystemModel::metaObject();
impl /*struct*/ QFileSystemModel {
  pub fn metaObject<RetType, T: QFileSystemModel_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFileSystemModel_metaObject<RetType> {
  fn metaObject(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  const QMetaObject * QFileSystemModel::metaObject();
impl<'a> /*trait*/ QFileSystemModel_metaObject<()> for () {
  fn metaObject(self , rsthis: & QFileSystemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel10metaObjectEv()};
     unsafe {_ZNK16QFileSystemModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFileSystemModel::fetchMore(const QModelIndex & parent);
impl /*struct*/ QFileSystemModel {
  pub fn fetchMore<RetType, T: QFileSystemModel_fetchMore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fetchMore(self);
    // return 1;
  }
}

pub trait QFileSystemModel_fetchMore<RetType> {
  fn fetchMore(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  void QFileSystemModel::fetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_fetchMore<()> for (&'a QModelIndex) {
  fn fetchMore(self , rsthis: & QFileSystemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel9fetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QFileSystemModel9fetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFileSystemModel::QFileSystemModel(const QFileSystemModel & );
impl /*struct*/ QFileSystemModel {
  pub fn New<T: QFileSystemModel_New>(value: T) -> QFileSystemModel {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QFileSystemModel_New {
  fn New(self) -> QFileSystemModel;
}

  // proto:  void QFileSystemModel::QFileSystemModel(const QFileSystemModel & );
impl<'a> /*trait*/ QFileSystemModel_New for (&'a QFileSystemModel) {
  fn New(self) -> QFileSystemModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModelC1ERKS_()};
    let ctysz: c_int = unsafe{QFileSystemModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QFileSystemModelC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN16QFileSystemModelC1ERKS_(arg0)} as u64;
    let rsthis = QFileSystemModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qint64 QFileSystemModel::size(const QModelIndex & index);
impl /*struct*/ QFileSystemModel {
  pub fn size<RetType, T: QFileSystemModel_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QFileSystemModel_size<RetType> {
  fn size(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  qint64 QFileSystemModel::size(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_size<i64> for (&'a QModelIndex) {
  fn size(self , rsthis: & QFileSystemModel) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel4sizeERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel4sizeERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QFileIconProvider * QFileSystemModel::iconProvider();
impl /*struct*/ QFileSystemModel {
  pub fn iconProvider<RetType, T: QFileSystemModel_iconProvider<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconProvider(self);
    // return 1;
  }
}

pub trait QFileSystemModel_iconProvider<RetType> {
  fn iconProvider(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QFileIconProvider * QFileSystemModel::iconProvider();
impl<'a> /*trait*/ QFileSystemModel_iconProvider<QFileIconProvider> for () {
  fn iconProvider(self , rsthis: & QFileSystemModel) -> QFileIconProvider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel12iconProviderEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel12iconProviderEv(rsthis.qclsinst)};
    let mut ret1 = QFileIconProvider::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileSystemModel::setNameFilters(const QStringList & filters);
impl /*struct*/ QFileSystemModel {
  pub fn setNameFilters<RetType, T: QFileSystemModel_setNameFilters<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNameFilters(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setNameFilters<RetType> {
  fn setNameFilters(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  void QFileSystemModel::setNameFilters(const QStringList & filters);
impl<'a> /*trait*/ QFileSystemModel_setNameFilters<()> for (&'a QStringList) {
  fn setNameFilters(self , rsthis: & QFileSystemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel14setNameFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QFileSystemModel14setNameFiltersERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVariant QFileSystemModel::data(const QModelIndex & index, int role);
impl /*struct*/ QFileSystemModel {
  pub fn data<RetType, T: QFileSystemModel_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QFileSystemModel_data<RetType> {
  fn data(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QVariant QFileSystemModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QFileSystemModel_data<QVariant> for (&'a QModelIndex, i32) {
  fn data(self , rsthis: & QFileSystemModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK16QFileSystemModel4dataERK11QModelIndexi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QDir QFileSystemModel::rootDirectory();
impl /*struct*/ QFileSystemModel {
  pub fn rootDirectory<RetType, T: QFileSystemModel_rootDirectory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootDirectory(self);
    // return 1;
  }
}

pub trait QFileSystemModel_rootDirectory<RetType> {
  fn rootDirectory(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QDir QFileSystemModel::rootDirectory();
impl<'a> /*trait*/ QFileSystemModel_rootDirectory<QDir> for () {
  fn rootDirectory(self , rsthis: & QFileSystemModel) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel13rootDirectoryEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel13rootDirectoryEv(rsthis.qclsinst)};
    let mut ret1 = QDir::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QFileSystemModel::mkdir(const QModelIndex & parent, const QString & name);
impl /*struct*/ QFileSystemModel {
  pub fn mkdir<RetType, T: QFileSystemModel_mkdir<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mkdir(self);
    // return 1;
  }
}

pub trait QFileSystemModel_mkdir<RetType> {
  fn mkdir(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QModelIndex QFileSystemModel::mkdir(const QModelIndex & parent, const QString & name);
impl<'a> /*trait*/ QFileSystemModel_mkdir<QModelIndex> for (&'a QModelIndex, &'a QString) {
  fn mkdir(self , rsthis: & QFileSystemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel5mkdirERK11QModelIndexRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QFileSystemModel5mkdirERK11QModelIndexRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QFileSystemModel::rowCount(const QModelIndex & parent);
impl /*struct*/ QFileSystemModel {
  pub fn rowCount<RetType, T: QFileSystemModel_rowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QFileSystemModel_rowCount<RetType> {
  fn rowCount(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  int QFileSystemModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_rowCount<i32> for (&'a QModelIndex) {
  fn rowCount(self , rsthis: & QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QFileSystemModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl /*struct*/ QFileSystemModel {
  pub fn setData<RetType, T: QFileSystemModel_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setData<RetType> {
  fn setData(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  bool QFileSystemModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QFileSystemModel_setData<i8> for (&'a QModelIndex, &'a QVariant, i32) {
  fn setData(self , rsthis: & QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN16QFileSystemModel7setDataERK11QModelIndexRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QFileSystemModel::columnCount(const QModelIndex & parent);
impl /*struct*/ QFileSystemModel {
  pub fn columnCount<RetType, T: QFileSystemModel_columnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QFileSystemModel_columnCount<RetType> {
  fn columnCount(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  int QFileSystemModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_columnCount<i32> for (&'a QModelIndex) {
  fn columnCount(self , rsthis: & QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QModelIndex QFileSystemModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_index<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn index(self , rsthis: & QFileSystemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFileSystemModel::canFetchMore(const QModelIndex & parent);
impl /*struct*/ QFileSystemModel {
  pub fn canFetchMore<RetType, T: QFileSystemModel_canFetchMore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canFetchMore(self);
    // return 1;
  }
}

pub trait QFileSystemModel_canFetchMore<RetType> {
  fn canFetchMore(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  bool QFileSystemModel::canFetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_canFetchMore<i8> for (&'a QModelIndex) {
  fn canFetchMore(self , rsthis: & QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel12canFetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel12canFetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFileSystemModel::remove(const QModelIndex & index);
impl /*struct*/ QFileSystemModel {
  pub fn remove<RetType, T: QFileSystemModel_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QFileSystemModel_remove<RetType> {
  fn remove(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  bool QFileSystemModel::remove(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_remove<i8> for (&'a QModelIndex) {
  fn remove(self , rsthis: & QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel6removeERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QFileSystemModel6removeERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFileSystemModel::rootPathChanged(const QString & newPath);
impl /*struct*/ QFileSystemModel {
  pub fn rootPathChanged<RetType, T: QFileSystemModel_rootPathChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootPathChanged(self);
    // return 1;
  }
}

pub trait QFileSystemModel_rootPathChanged<RetType> {
  fn rootPathChanged(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  void QFileSystemModel::rootPathChanged(const QString & newPath);
impl<'a> /*trait*/ QFileSystemModel_rootPathChanged<()> for (&'a QString) {
  fn rootPathChanged(self , rsthis: & QFileSystemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel15rootPathChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QFileSystemModel15rootPathChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QFileSystemModel::setRootPath(const QString & path);
impl /*struct*/ QFileSystemModel {
  pub fn setRootPath<RetType, T: QFileSystemModel_setRootPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRootPath(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setRootPath<RetType> {
  fn setRootPath(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QModelIndex QFileSystemModel::setRootPath(const QString & path);
impl<'a> /*trait*/ QFileSystemModel_setRootPath<QModelIndex> for (&'a QString) {
  fn setRootPath(self , rsthis: & QFileSystemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel11setRootPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QFileSystemModel11setRootPathERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileSystemModel::directoryLoaded(const QString & path);
impl /*struct*/ QFileSystemModel {
  pub fn directoryLoaded<RetType, T: QFileSystemModel_directoryLoaded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.directoryLoaded(self);
    // return 1;
  }
}

pub trait QFileSystemModel_directoryLoaded<RetType> {
  fn directoryLoaded(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  void QFileSystemModel::directoryLoaded(const QString & path);
impl<'a> /*trait*/ QFileSystemModel_directoryLoaded<()> for (&'a QString) {
  fn directoryLoaded(self , rsthis: & QFileSystemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel15directoryLoadedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QFileSystemModel15directoryLoadedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFileSystemModel::setResolveSymlinks(bool enable);
impl /*struct*/ QFileSystemModel {
  pub fn setResolveSymlinks<RetType, T: QFileSystemModel_setResolveSymlinks<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setResolveSymlinks(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setResolveSymlinks<RetType> {
  fn setResolveSymlinks(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  void QFileSystemModel::setResolveSymlinks(bool enable);
impl<'a> /*trait*/ QFileSystemModel_setResolveSymlinks<()> for (i8) {
  fn setResolveSymlinks(self , rsthis: & QFileSystemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel18setResolveSymlinksEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN16QFileSystemModel18setResolveSymlinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFileSystemModel::setReadOnly(bool enable);
impl /*struct*/ QFileSystemModel {
  pub fn setReadOnly<RetType, T: QFileSystemModel_setReadOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setReadOnly<RetType> {
  fn setReadOnly(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  void QFileSystemModel::setReadOnly(bool enable);
impl<'a> /*trait*/ QFileSystemModel_setReadOnly<()> for (i8) {
  fn setReadOnly(self , rsthis: & QFileSystemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel11setReadOnlyEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN16QFileSystemModel11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QFileSystemModel::nameFilters();
impl /*struct*/ QFileSystemModel {
  pub fn nameFilters<RetType, T: QFileSystemModel_nameFilters<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nameFilters(self);
    // return 1;
  }
}

pub trait QFileSystemModel_nameFilters<RetType> {
  fn nameFilters(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QStringList QFileSystemModel::nameFilters();
impl<'a> /*trait*/ QFileSystemModel_nameFilters<()> for () {
  fn nameFilters(self , rsthis: & QFileSystemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel11nameFiltersEv()};
     unsafe {_ZNK16QFileSystemModel11nameFiltersEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFileSystemModel::QFileSystemModel(QObject * parent);
impl<'a> /*trait*/ QFileSystemModel_New for (&'a QObject) {
  fn New(self) -> QFileSystemModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModelC1EP7QObject()};
    let ctysz: c_int = unsafe{QFileSystemModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN16QFileSystemModelC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN16QFileSystemModelC1EP7QObject(arg0)} as u64;
    let rsthis = QFileSystemModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QFileInfo QFileSystemModel::fileInfo(const QModelIndex & index);
impl /*struct*/ QFileSystemModel {
  pub fn fileInfo<RetType, T: QFileSystemModel_fileInfo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileInfo(self);
    // return 1;
  }
}

pub trait QFileSystemModel_fileInfo<RetType> {
  fn fileInfo(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QFileInfo QFileSystemModel::fileInfo(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_fileInfo<QFileInfo> for (&'a QModelIndex) {
  fn fileInfo(self , rsthis: & QFileSystemModel) -> QFileInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8fileInfoERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZNK16QFileSystemModel8fileInfoERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QFileInfo::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QFileSystemModel::myComputer(int role);
impl /*struct*/ QFileSystemModel {
  pub fn myComputer<RetType, T: QFileSystemModel_myComputer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.myComputer(self);
    // return 1;
  }
}

pub trait QFileSystemModel_myComputer<RetType> {
  fn myComputer(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QVariant QFileSystemModel::myComputer(int role);
impl<'a> /*trait*/ QFileSystemModel_myComputer<QVariant> for (i32) {
  fn myComputer(self , rsthis: & QFileSystemModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel10myComputerEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK16QFileSystemModel10myComputerEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFileSystemModel::isReadOnly();
impl /*struct*/ QFileSystemModel {
  pub fn isReadOnly<RetType, T: QFileSystemModel_isReadOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly(self);
    // return 1;
  }
}

pub trait QFileSystemModel_isReadOnly<RetType> {
  fn isReadOnly(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  bool QFileSystemModel::isReadOnly();
impl<'a> /*trait*/ QFileSystemModel_isReadOnly<i8> for () {
  fn isReadOnly(self , rsthis: & QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QFileSystemModel::type(const QModelIndex & index);
impl /*struct*/ QFileSystemModel {
  pub fn type_<RetType, T: QFileSystemModel_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QFileSystemModel_type_<RetType> {
  fn type_(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QString QFileSystemModel::type(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_type_<QString> for (&'a QModelIndex) {
  fn type_(self , rsthis: & QFileSystemModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel4typeERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel4typeERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QFileSystemModel::rootPath();
impl /*struct*/ QFileSystemModel {
  pub fn rootPath<RetType, T: QFileSystemModel_rootPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootPath(self);
    // return 1;
  }
}

pub trait QFileSystemModel_rootPath<RetType> {
  fn rootPath(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QString QFileSystemModel::rootPath();
impl<'a> /*trait*/ QFileSystemModel_rootPath<QString> for () {
  fn rootPath(self , rsthis: & QFileSystemModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8rootPathEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel8rootPathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QDateTime QFileSystemModel::lastModified(const QModelIndex & index);
impl /*struct*/ QFileSystemModel {
  pub fn lastModified<RetType, T: QFileSystemModel_lastModified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastModified(self);
    // return 1;
  }
}

pub trait QFileSystemModel_lastModified<RetType> {
  fn lastModified(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  QDateTime QFileSystemModel::lastModified(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_lastModified<QDateTime> for (&'a QModelIndex) {
  fn lastModified(self , rsthis: & QFileSystemModel) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel12lastModifiedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel12lastModifiedERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileSystemModel::fileRenamed(const QString & path, const QString & oldName, const QString & newName);
impl /*struct*/ QFileSystemModel {
  pub fn fileRenamed<RetType, T: QFileSystemModel_fileRenamed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileRenamed(self);
    // return 1;
  }
}

pub trait QFileSystemModel_fileRenamed<RetType> {
  fn fileRenamed(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  void QFileSystemModel::fileRenamed(const QString & path, const QString & oldName, const QString & newName);
impl<'a> /*trait*/ QFileSystemModel_fileRenamed<()> for (&'a QString, &'a QString, &'a QString) {
  fn fileRenamed(self , rsthis: & QFileSystemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QFileSystemModel::isDir(const QModelIndex & index);
impl /*struct*/ QFileSystemModel {
  pub fn isDir<RetType, T: QFileSystemModel_isDir<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDir(self);
    // return 1;
  }
}

pub trait QFileSystemModel_isDir<RetType> {
  fn isDir(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  bool QFileSystemModel::isDir(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_isDir<i8> for (&'a QModelIndex) {
  fn isDir(self , rsthis: & QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel5isDirERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel5isDirERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFileSystemModel::rmdir(const QModelIndex & index);
impl /*struct*/ QFileSystemModel {
  pub fn rmdir<RetType, T: QFileSystemModel_rmdir<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rmdir(self);
    // return 1;
  }
}

pub trait QFileSystemModel_rmdir<RetType> {
  fn rmdir(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  bool QFileSystemModel::rmdir(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_rmdir<i8> for (&'a QModelIndex) {
  fn rmdir(self , rsthis: & QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel5rmdirERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QFileSystemModel5rmdirERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFileSystemModel::setIconProvider(QFileIconProvider * provider);
impl /*struct*/ QFileSystemModel {
  pub fn setIconProvider<RetType, T: QFileSystemModel_setIconProvider<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIconProvider(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setIconProvider<RetType> {
  fn setIconProvider(self , rsthis: & QFileSystemModel) -> RetType;
}

  // proto:  void QFileSystemModel::setIconProvider(QFileIconProvider * provider);
impl<'a> /*trait*/ QFileSystemModel_setIconProvider<()> for (&'a QFileIconProvider) {
  fn setIconProvider(self , rsthis: & QFileSystemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel15setIconProviderEP17QFileIconProvider()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QFileSystemModel15setIconProviderEP17QFileIconProvider(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QFileSystemModel_rootPathChanged
pub struct QFileSystemModel_rootPathChanged_signal{poi:u64}
impl /* struct */ QFileSystemModel {
  pub fn rootPathChanged_1(&self) -> QFileSystemModel_rootPathChanged_signal {
     return QFileSystemModel_rootPathChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFileSystemModel_rootPathChanged_signal {
  pub fn connect<T: QFileSystemModel_rootPathChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFileSystemModel_rootPathChanged_signal_connect {
  fn connect(self, sigthis: QFileSystemModel_rootPathChanged_signal);
}

#[derive(Default)] // for QFileSystemModel_directoryLoaded
pub struct QFileSystemModel_directoryLoaded_signal{poi:u64}
impl /* struct */ QFileSystemModel {
  pub fn directoryLoaded_1(&self) -> QFileSystemModel_directoryLoaded_signal {
     return QFileSystemModel_directoryLoaded_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFileSystemModel_directoryLoaded_signal {
  pub fn connect<T: QFileSystemModel_directoryLoaded_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFileSystemModel_directoryLoaded_signal_connect {
  fn connect(self, sigthis: QFileSystemModel_directoryLoaded_signal);
}

#[derive(Default)] // for QFileSystemModel_fileRenamed
pub struct QFileSystemModel_fileRenamed_signal{poi:u64}
impl /* struct */ QFileSystemModel {
  pub fn fileRenamed_1(&self) -> QFileSystemModel_fileRenamed_signal {
     return QFileSystemModel_fileRenamed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFileSystemModel_fileRenamed_signal {
  pub fn connect<T: QFileSystemModel_fileRenamed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFileSystemModel_fileRenamed_signal_connect {
  fn connect(self, sigthis: QFileSystemModel_fileRenamed_signal);
}

// directoryLoaded(const class QString &)
extern fn QFileSystemModel_directoryLoaded_signal_connect_cb_0(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
extern fn QFileSystemModel_directoryLoaded_signal_connect_cb_box_0(rsfptr_raw:*mut c_void, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QFileSystemModel_directoryLoaded_signal_connect for fn(QString) {
  fn connect(self, sigthis: QFileSystemModel_directoryLoaded_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFileSystemModel_directoryLoaded_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFileSystemModel_SlotProxy_connect__ZN16QFileSystemModel15directoryLoadedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFileSystemModel_directoryLoaded_signal_connect for Box<fn(QString)> {
  fn connect(self, sigthis: QFileSystemModel_directoryLoaded_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFileSystemModel_directoryLoaded_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QFileSystemModel_SlotProxy_connect__ZN16QFileSystemModel15directoryLoadedERK7QString(arg0, arg1, arg2)};
  }
}
// rootPathChanged(const class QString &)
extern fn QFileSystemModel_rootPathChanged_signal_connect_cb_1(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
extern fn QFileSystemModel_rootPathChanged_signal_connect_cb_box_1(rsfptr_raw:*mut c_void, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QFileSystemModel_rootPathChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QFileSystemModel_rootPathChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFileSystemModel_rootPathChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFileSystemModel_SlotProxy_connect__ZN16QFileSystemModel15rootPathChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFileSystemModel_rootPathChanged_signal_connect for Box<fn(QString)> {
  fn connect(self, sigthis: QFileSystemModel_rootPathChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFileSystemModel_rootPathChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QFileSystemModel_SlotProxy_connect__ZN16QFileSystemModel15rootPathChangedERK7QString(arg0, arg1, arg2)};
  }
}
// fileRenamed(const class QString &, const class QString &, const class QString &)
extern fn QFileSystemModel_fileRenamed_signal_connect_cb_2(rsfptr:fn(QString, QString, QString), arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
extern fn QFileSystemModel_fileRenamed_signal_connect_cb_box_2(rsfptr_raw:*mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QFileSystemModel_fileRenamed_signal_connect for fn(QString, QString, QString) {
  fn connect(self, sigthis: QFileSystemModel_fileRenamed_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFileSystemModel_fileRenamed_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QFileSystemModel_SlotProxy_connect__ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_(arg0, arg1, arg2)};
  }
}
impl /* trait */ QFileSystemModel_fileRenamed_signal_connect for Box<fn(QString, QString, QString)> {
  fn connect(self, sigthis: QFileSystemModel_fileRenamed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QFileSystemModel_fileRenamed_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QFileSystemModel_SlotProxy_connect__ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_(arg0, arg1, arg2)};
  }
}
// <= body block end

