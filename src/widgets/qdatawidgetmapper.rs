// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtWidgets/qdatawidgetmapper.h
// dst-file: /src/widgets/qdatawidgetmapper.rs
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
use super::super::core::qobject::*; // 771
use std::ops::Deref;
use super::qwidget::*; // 773
use super::super::core::qbytearray::*; // 771
use super::super::core::qabstractitemmodel::*; // 771
use super::qabstractitemdelegate::*; // 773
use super::super::core::qobjectdefs::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDataWidgetMapper_Class_Size() -> c_int;
  // proto:  int QDataWidgetMapper::currentIndex();
  fn C_ZNK17QDataWidgetMapper12currentIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section, const QByteArray & propertyName);
  fn C_ZN17QDataWidgetMapper10addMappingEP7QWidgetiRK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  QModelIndex QDataWidgetMapper::rootIndex();
  fn C_ZNK17QDataWidgetMapper9rootIndexEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDataWidgetMapper::setCurrentIndex(int index);
  fn C_ZN17QDataWidgetMapper15setCurrentIndexEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QDataWidgetMapper::setModel(QAbstractItemModel * model);
  fn C_ZN17QDataWidgetMapper8setModelEP18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QDataWidgetMapper::mappedWidgetAt(int section);
  fn C_ZNK17QDataWidgetMapper14mappedWidgetAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QDataWidgetMapper::removeMapping(QWidget * widget);
  fn C_ZN17QDataWidgetMapper13removeMappingEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDataWidgetMapper::toFirst();
  fn C_ZN17QDataWidgetMapper7toFirstEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDataWidgetMapper::toPrevious();
  fn C_ZN17QDataWidgetMapper10toPreviousEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDataWidgetMapper::setRootIndex(const QModelIndex & index);
  fn C_ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDataWidgetMapper::revert();
  fn C_ZN17QDataWidgetMapper6revertEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDataWidgetMapper::clearMapping();
  fn C_ZN17QDataWidgetMapper12clearMappingEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QDataWidgetMapper::mappedSection(QWidget * widget);
  fn C_ZNK17QDataWidgetMapper13mappedSectionEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QByteArray QDataWidgetMapper::mappedPropertyName(QWidget * widget);
  fn C_ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QDataWidgetMapper::setItemDelegate(QAbstractItemDelegate * delegate);
  fn C_ZN17QDataWidgetMapper15setItemDelegateEP21QAbstractItemDelegate(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDataWidgetMapper::setCurrentModelIndex(const QModelIndex & index);
  fn C_ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDataWidgetMapper::~QDataWidgetMapper();
  fn C_ZN17QDataWidgetMapperD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section);
  fn C_ZN17QDataWidgetMapper10addMappingEP7QWidgeti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  void QDataWidgetMapper::QDataWidgetMapper(QObject * parent);
  fn C_ZN17QDataWidgetMapperC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  const QMetaObject * QDataWidgetMapper::metaObject();
  fn C_ZNK17QDataWidgetMapper10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDataWidgetMapper::toLast();
  fn C_ZN17QDataWidgetMapper6toLastEv(qthis: u64 /* *mut c_void*/);
  // proto:  QAbstractItemModel * QDataWidgetMapper::model();
  fn C_ZNK17QDataWidgetMapper5modelEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAbstractItemDelegate * QDataWidgetMapper::itemDelegate();
  fn C_ZNK17QDataWidgetMapper12itemDelegateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QDataWidgetMapper::submit();
  fn C_ZN17QDataWidgetMapper6submitEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QDataWidgetMapper::toNext();
  fn C_ZN17QDataWidgetMapper6toNextEv(qthis: u64 /* *mut c_void*/);
  fn QDataWidgetMapper_SlotProxy_connect__ZN17QDataWidgetMapper19currentIndexChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QDataWidgetMapper)=1
#[derive(Default)]
pub struct QDataWidgetMapper {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _currentIndexChanged: QDataWidgetMapper_currentIndexChanged_signal,
}

impl /*struct*/ QDataWidgetMapper {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDataWidgetMapper {
    return QDataWidgetMapper{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDataWidgetMapper {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QDataWidgetMapper {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  int QDataWidgetMapper::currentIndex();
impl /*struct*/ QDataWidgetMapper {
  pub fn currentIndex<RetType, T: QDataWidgetMapper_currentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_currentIndex<RetType> {
  fn currentIndex(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  int QDataWidgetMapper::currentIndex();
impl<'a> /*trait*/ QDataWidgetMapper_currentIndex<i32> for () {
  fn currentIndex(self , rsthis: & QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper12currentIndexEv()};
    let mut ret = unsafe {C_ZNK17QDataWidgetMapper12currentIndexEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section, const QByteArray & propertyName);
impl /*struct*/ QDataWidgetMapper {
  pub fn addMapping<RetType, T: QDataWidgetMapper_addMapping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addMapping(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_addMapping<RetType> {
  fn addMapping(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section, const QByteArray & propertyName);
impl<'a> /*trait*/ QDataWidgetMapper_addMapping<()> for (&'a QWidget, i32, &'a QByteArray) {
  fn addMapping(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper10addMappingEP7QWidgetiRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN17QDataWidgetMapper10addMappingEP7QWidgetiRK10QByteArray(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QModelIndex QDataWidgetMapper::rootIndex();
impl /*struct*/ QDataWidgetMapper {
  pub fn rootIndex<RetType, T: QDataWidgetMapper_rootIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_rootIndex<RetType> {
  fn rootIndex(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  QModelIndex QDataWidgetMapper::rootIndex();
impl<'a> /*trait*/ QDataWidgetMapper_rootIndex<QModelIndex> for () {
  fn rootIndex(self , rsthis: & QDataWidgetMapper) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper9rootIndexEv()};
    let mut ret = unsafe {C_ZNK17QDataWidgetMapper9rootIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::setCurrentIndex(int index);
impl /*struct*/ QDataWidgetMapper {
  pub fn setCurrentIndex<RetType, T: QDataWidgetMapper_setCurrentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_setCurrentIndex<RetType> {
  fn setCurrentIndex(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::setCurrentIndex(int index);
impl<'a> /*trait*/ QDataWidgetMapper_setCurrentIndex<()> for (i32) {
  fn setCurrentIndex(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN17QDataWidgetMapper15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::setModel(QAbstractItemModel * model);
impl /*struct*/ QDataWidgetMapper {
  pub fn setModel<RetType, T: QDataWidgetMapper_setModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModel(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_setModel<RetType> {
  fn setModel(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::setModel(QAbstractItemModel * model);
impl<'a> /*trait*/ QDataWidgetMapper_setModel<()> for (&'a QAbstractItemModel) {
  fn setModel(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper8setModelEP18QAbstractItemModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QDataWidgetMapper8setModelEP18QAbstractItemModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QDataWidgetMapper::mappedWidgetAt(int section);
impl /*struct*/ QDataWidgetMapper {
  pub fn mappedWidgetAt<RetType, T: QDataWidgetMapper_mappedWidgetAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mappedWidgetAt(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_mappedWidgetAt<RetType> {
  fn mappedWidgetAt(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  QWidget * QDataWidgetMapper::mappedWidgetAt(int section);
impl<'a> /*trait*/ QDataWidgetMapper_mappedWidgetAt<QWidget> for (i32) {
  fn mappedWidgetAt(self , rsthis: & QDataWidgetMapper) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper14mappedWidgetAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK17QDataWidgetMapper14mappedWidgetAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::removeMapping(QWidget * widget);
impl /*struct*/ QDataWidgetMapper {
  pub fn removeMapping<RetType, T: QDataWidgetMapper_removeMapping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeMapping(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_removeMapping<RetType> {
  fn removeMapping(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::removeMapping(QWidget * widget);
impl<'a> /*trait*/ QDataWidgetMapper_removeMapping<()> for (&'a QWidget) {
  fn removeMapping(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper13removeMappingEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QDataWidgetMapper13removeMappingEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::toFirst();
impl /*struct*/ QDataWidgetMapper {
  pub fn toFirst<RetType, T: QDataWidgetMapper_toFirst<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toFirst(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_toFirst<RetType> {
  fn toFirst(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::toFirst();
impl<'a> /*trait*/ QDataWidgetMapper_toFirst<()> for () {
  fn toFirst(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper7toFirstEv()};
     unsafe {C_ZN17QDataWidgetMapper7toFirstEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::toPrevious();
impl /*struct*/ QDataWidgetMapper {
  pub fn toPrevious<RetType, T: QDataWidgetMapper_toPrevious<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPrevious(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_toPrevious<RetType> {
  fn toPrevious(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::toPrevious();
impl<'a> /*trait*/ QDataWidgetMapper_toPrevious<()> for () {
  fn toPrevious(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper10toPreviousEv()};
     unsafe {C_ZN17QDataWidgetMapper10toPreviousEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::setRootIndex(const QModelIndex & index);
impl /*struct*/ QDataWidgetMapper {
  pub fn setRootIndex<RetType, T: QDataWidgetMapper_setRootIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRootIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_setRootIndex<RetType> {
  fn setRootIndex(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QDataWidgetMapper_setRootIndex<()> for (&'a QModelIndex) {
  fn setRootIndex(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::revert();
impl /*struct*/ QDataWidgetMapper {
  pub fn revert<RetType, T: QDataWidgetMapper_revert<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.revert(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_revert<RetType> {
  fn revert(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::revert();
impl<'a> /*trait*/ QDataWidgetMapper_revert<()> for () {
  fn revert(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6revertEv()};
     unsafe {C_ZN17QDataWidgetMapper6revertEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::clearMapping();
impl /*struct*/ QDataWidgetMapper {
  pub fn clearMapping<RetType, T: QDataWidgetMapper_clearMapping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearMapping(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_clearMapping<RetType> {
  fn clearMapping(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::clearMapping();
impl<'a> /*trait*/ QDataWidgetMapper_clearMapping<()> for () {
  fn clearMapping(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper12clearMappingEv()};
     unsafe {C_ZN17QDataWidgetMapper12clearMappingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QDataWidgetMapper::mappedSection(QWidget * widget);
impl /*struct*/ QDataWidgetMapper {
  pub fn mappedSection<RetType, T: QDataWidgetMapper_mappedSection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mappedSection(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_mappedSection<RetType> {
  fn mappedSection(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  int QDataWidgetMapper::mappedSection(QWidget * widget);
impl<'a> /*trait*/ QDataWidgetMapper_mappedSection<i32> for (&'a QWidget) {
  fn mappedSection(self , rsthis: & QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper13mappedSectionEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK17QDataWidgetMapper13mappedSectionEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QByteArray QDataWidgetMapper::mappedPropertyName(QWidget * widget);
impl /*struct*/ QDataWidgetMapper {
  pub fn mappedPropertyName<RetType, T: QDataWidgetMapper_mappedPropertyName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mappedPropertyName(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_mappedPropertyName<RetType> {
  fn mappedPropertyName(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  QByteArray QDataWidgetMapper::mappedPropertyName(QWidget * widget);
impl<'a> /*trait*/ QDataWidgetMapper_mappedPropertyName<QByteArray> for (&'a QWidget) {
  fn mappedPropertyName(self , rsthis: & QDataWidgetMapper) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::setItemDelegate(QAbstractItemDelegate * delegate);
impl /*struct*/ QDataWidgetMapper {
  pub fn setItemDelegate<RetType, T: QDataWidgetMapper_setItemDelegate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemDelegate(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_setItemDelegate<RetType> {
  fn setItemDelegate(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::setItemDelegate(QAbstractItemDelegate * delegate);
impl<'a> /*trait*/ QDataWidgetMapper_setItemDelegate<()> for (&'a QAbstractItemDelegate) {
  fn setItemDelegate(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper15setItemDelegateEP21QAbstractItemDelegate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QDataWidgetMapper15setItemDelegateEP21QAbstractItemDelegate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::setCurrentModelIndex(const QModelIndex & index);
impl /*struct*/ QDataWidgetMapper {
  pub fn setCurrentModelIndex<RetType, T: QDataWidgetMapper_setCurrentModelIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentModelIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_setCurrentModelIndex<RetType> {
  fn setCurrentModelIndex(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::setCurrentModelIndex(const QModelIndex & index);
impl<'a> /*trait*/ QDataWidgetMapper_setCurrentModelIndex<()> for (&'a QModelIndex) {
  fn setCurrentModelIndex(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::~QDataWidgetMapper();
impl /*struct*/ QDataWidgetMapper {
  pub fn free<RetType, T: QDataWidgetMapper_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_free<RetType> {
  fn free(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::~QDataWidgetMapper();
impl<'a> /*trait*/ QDataWidgetMapper_free<()> for () {
  fn free(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapperD2Ev()};
     unsafe {C_ZN17QDataWidgetMapperD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section);
impl<'a> /*trait*/ QDataWidgetMapper_addMapping<()> for (&'a QWidget, i32) {
  fn addMapping(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper10addMappingEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN17QDataWidgetMapper10addMappingEP7QWidgeti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::QDataWidgetMapper(QObject * parent);
impl /*struct*/ QDataWidgetMapper {
  pub fn new<T: QDataWidgetMapper_new>(value: T) -> QDataWidgetMapper {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDataWidgetMapper_new {
  fn new(self) -> QDataWidgetMapper;
}

  // proto:  void QDataWidgetMapper::QDataWidgetMapper(QObject * parent);
impl<'a> /*trait*/ QDataWidgetMapper_new for (&'a QObject) {
  fn new(self) -> QDataWidgetMapper {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapperC2EP7QObject()};
    let ctysz: c_int = unsafe{QDataWidgetMapper_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QDataWidgetMapperC2EP7QObject(arg0)};
    let rsthis = QDataWidgetMapper{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QDataWidgetMapper::metaObject();
impl /*struct*/ QDataWidgetMapper {
  pub fn metaObject<RetType, T: QDataWidgetMapper_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_metaObject<RetType> {
  fn metaObject(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  const QMetaObject * QDataWidgetMapper::metaObject();
impl<'a> /*trait*/ QDataWidgetMapper_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QDataWidgetMapper) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper10metaObjectEv()};
    let mut ret = unsafe {C_ZNK17QDataWidgetMapper10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::toLast();
impl /*struct*/ QDataWidgetMapper {
  pub fn toLast<RetType, T: QDataWidgetMapper_toLast<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLast(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_toLast<RetType> {
  fn toLast(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::toLast();
impl<'a> /*trait*/ QDataWidgetMapper_toLast<()> for () {
  fn toLast(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6toLastEv()};
     unsafe {C_ZN17QDataWidgetMapper6toLastEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAbstractItemModel * QDataWidgetMapper::model();
impl /*struct*/ QDataWidgetMapper {
  pub fn model<RetType, T: QDataWidgetMapper_model<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_model<RetType> {
  fn model(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  QAbstractItemModel * QDataWidgetMapper::model();
impl<'a> /*trait*/ QDataWidgetMapper_model<QAbstractItemModel> for () {
  fn model(self , rsthis: & QDataWidgetMapper) -> QAbstractItemModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper5modelEv()};
    let mut ret = unsafe {C_ZNK17QDataWidgetMapper5modelEv(rsthis.qclsinst)};
    let mut ret1 = QAbstractItemModel::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAbstractItemDelegate * QDataWidgetMapper::itemDelegate();
impl /*struct*/ QDataWidgetMapper {
  pub fn itemDelegate<RetType, T: QDataWidgetMapper_itemDelegate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemDelegate(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_itemDelegate<RetType> {
  fn itemDelegate(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  QAbstractItemDelegate * QDataWidgetMapper::itemDelegate();
impl<'a> /*trait*/ QDataWidgetMapper_itemDelegate<QAbstractItemDelegate> for () {
  fn itemDelegate(self , rsthis: & QDataWidgetMapper) -> QAbstractItemDelegate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper12itemDelegateEv()};
    let mut ret = unsafe {C_ZNK17QDataWidgetMapper12itemDelegateEv(rsthis.qclsinst)};
    let mut ret1 = QAbstractItemDelegate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDataWidgetMapper::submit();
impl /*struct*/ QDataWidgetMapper {
  pub fn submit<RetType, T: QDataWidgetMapper_submit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.submit(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_submit<RetType> {
  fn submit(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  bool QDataWidgetMapper::submit();
impl<'a> /*trait*/ QDataWidgetMapper_submit<i8> for () {
  fn submit(self , rsthis: & QDataWidgetMapper) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6submitEv()};
    let mut ret = unsafe {C_ZN17QDataWidgetMapper6submitEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::toNext();
impl /*struct*/ QDataWidgetMapper {
  pub fn toNext<RetType, T: QDataWidgetMapper_toNext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toNext(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_toNext<RetType> {
  fn toNext(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::toNext();
impl<'a> /*trait*/ QDataWidgetMapper_toNext<()> for () {
  fn toNext(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6toNextEv()};
     unsafe {C_ZN17QDataWidgetMapper6toNextEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QDataWidgetMapper_currentIndexChanged
pub struct QDataWidgetMapper_currentIndexChanged_signal{poi:u64}
impl /* struct */ QDataWidgetMapper {
  pub fn currentIndexChanged(&self) -> QDataWidgetMapper_currentIndexChanged_signal {
     return QDataWidgetMapper_currentIndexChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDataWidgetMapper_currentIndexChanged_signal {
  pub fn connect<T: QDataWidgetMapper_currentIndexChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDataWidgetMapper_currentIndexChanged_signal_connect {
  fn connect(self, sigthis: QDataWidgetMapper_currentIndexChanged_signal);
}

// currentIndexChanged(int)
extern fn QDataWidgetMapper_currentIndexChanged_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QDataWidgetMapper_currentIndexChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDataWidgetMapper_currentIndexChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QDataWidgetMapper_currentIndexChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDataWidgetMapper_currentIndexChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDataWidgetMapper_SlotProxy_connect__ZN17QDataWidgetMapper19currentIndexChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDataWidgetMapper_currentIndexChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QDataWidgetMapper_currentIndexChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDataWidgetMapper_currentIndexChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDataWidgetMapper_SlotProxy_connect__ZN17QDataWidgetMapper19currentIndexChangedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

