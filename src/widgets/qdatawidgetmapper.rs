// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::super::core::qbytearray::QByteArray; // 771
use super::super::core::qabstractitemmodel::QModelIndex; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QDataWidgetMapper::QDataWidgetMapper(const QDataWidgetMapper & );
  fn _ZN17QDataWidgetMapperC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QDataWidgetMapper::currentIndex();
  fn _ZNK17QDataWidgetMapper12currentIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section, const QByteArray & propertyName);
  fn _ZN17QDataWidgetMapper10addMappingEP7QWidgetiRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  QModelIndex QDataWidgetMapper::rootIndex();
  fn _ZNK17QDataWidgetMapper9rootIndexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDataWidgetMapper::setCurrentIndex(int index);
  fn _ZN17QDataWidgetMapper15setCurrentIndexEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QWidget * QDataWidgetMapper::mappedWidgetAt(int section);
  fn _ZNK17QDataWidgetMapper14mappedWidgetAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QDataWidgetMapper::removeMapping(QWidget * widget);
  fn _ZN17QDataWidgetMapper13removeMappingEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDataWidgetMapper::toFirst();
  fn _ZN17QDataWidgetMapper7toFirstEv(qthis: *mut c_void);
  // proto:  void QDataWidgetMapper::currentIndexChanged(int index);
  fn _ZN17QDataWidgetMapper19currentIndexChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QDataWidgetMapper::toPrevious();
  fn _ZN17QDataWidgetMapper10toPreviousEv(qthis: *mut c_void);
  // proto:  void QDataWidgetMapper::setRootIndex(const QModelIndex & index);
  fn _ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDataWidgetMapper::revert();
  fn _ZN17QDataWidgetMapper6revertEv(qthis: *mut c_void);
  // proto:  void QDataWidgetMapper::clearMapping();
  fn _ZN17QDataWidgetMapper12clearMappingEv(qthis: *mut c_void);
  // proto:  int QDataWidgetMapper::mappedSection(QWidget * widget);
  fn _ZNK17QDataWidgetMapper13mappedSectionEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QByteArray QDataWidgetMapper::mappedPropertyName(QWidget * widget);
  fn _ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QDataWidgetMapper::setCurrentModelIndex(const QModelIndex & index);
  fn _ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDataWidgetMapper::~QDataWidgetMapper();
  fn _ZN17QDataWidgetMapperD0Ev(qthis: *mut c_void);
  // proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section);
  fn _ZN17QDataWidgetMapper10addMappingEP7QWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QDataWidgetMapper::QDataWidgetMapper(QObject * parent);
  fn _ZN17QDataWidgetMapperC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QDataWidgetMapper::metaObject();
  fn _ZNK17QDataWidgetMapper10metaObjectEv(qthis: *mut c_void);
  // proto:  void QDataWidgetMapper::toLast();
  fn _ZN17QDataWidgetMapper6toLastEv(qthis: *mut c_void);
  // proto:  QAbstractItemModel * QDataWidgetMapper::model();
  fn _ZNK17QDataWidgetMapper5modelEv(qthis: *mut c_void);
  // proto:  QAbstractItemDelegate * QDataWidgetMapper::itemDelegate();
  fn _ZNK17QDataWidgetMapper12itemDelegateEv(qthis: *mut c_void);
  // proto:  bool QDataWidgetMapper::submit();
  fn _ZN17QDataWidgetMapper6submitEv(qthis: *mut c_void) -> c_char;
  // proto:  void QDataWidgetMapper::toNext();
  fn _ZN17QDataWidgetMapper6toNextEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QDataWidgetMapper)=1
pub struct QDataWidgetMapper {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDataWidgetMapper {
  pub fn inheritFrom(qthis: *mut c_void) -> QDataWidgetMapper {
    return QDataWidgetMapper{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
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
  // proto:  void QDataWidgetMapper::QDataWidgetMapper(const QDataWidgetMapper & );
impl /*struct*/ QDataWidgetMapper {
  pub fn New<T: QDataWidgetMapper_New>(value: T) -> QDataWidgetMapper {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDataWidgetMapper_New {
  fn New(self) -> QDataWidgetMapper;
}

  // proto:  void QDataWidgetMapper::QDataWidgetMapper(const QDataWidgetMapper & );
impl<'a> /*trait*/ QDataWidgetMapper_New for (&'a QDataWidgetMapper) {
  fn New(self) -> QDataWidgetMapper {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapperC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QDataWidgetMapperC1ERKS_(qthis, arg0)};
    let rsthis = QDataWidgetMapper{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
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
    let mut ret = unsafe {_ZNK17QDataWidgetMapper12currentIndexEv(rsthis.qclsinst)};
    return ret as i32;
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
     unsafe {_ZN17QDataWidgetMapper10addMappingEP7QWidgetiRK10QByteArray(rsthis.qclsinst, arg0, arg1, arg2)};
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
    let mut ret = unsafe {_ZNK17QDataWidgetMapper9rootIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex::inheritFrom(ret);
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
     unsafe {_ZN17QDataWidgetMapper15setCurrentIndexEi(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK17QDataWidgetMapper14mappedWidgetAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret);
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
     unsafe {_ZN17QDataWidgetMapper13removeMappingEP7QWidget(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN17QDataWidgetMapper7toFirstEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::currentIndexChanged(int index);
impl /*struct*/ QDataWidgetMapper {
  pub fn currentIndexChanged<RetType, T: QDataWidgetMapper_currentIndexChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentIndexChanged(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_currentIndexChanged<RetType> {
  fn currentIndexChanged(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::currentIndexChanged(int index);
impl<'a> /*trait*/ QDataWidgetMapper_currentIndexChanged<()> for (i32) {
  fn currentIndexChanged(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper19currentIndexChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN17QDataWidgetMapper19currentIndexChangedEi(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN17QDataWidgetMapper10toPreviousEv(rsthis.qclsinst)};
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
     unsafe {_ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN17QDataWidgetMapper6revertEv(rsthis.qclsinst)};
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
     unsafe {_ZN17QDataWidgetMapper12clearMappingEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK17QDataWidgetMapper13mappedSectionEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
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
    let mut ret = unsafe {_ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
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
     unsafe {_ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::~QDataWidgetMapper();
impl /*struct*/ QDataWidgetMapper {
  pub fn Free<RetType, T: QDataWidgetMapper_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_Free<RetType> {
  fn Free(self , rsthis: & QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::~QDataWidgetMapper();
impl<'a> /*trait*/ QDataWidgetMapper_Free<()> for () {
  fn Free(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapperD0Ev()};
     unsafe {_ZN17QDataWidgetMapperD0Ev(rsthis.qclsinst)};
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
     unsafe {_ZN17QDataWidgetMapper10addMappingEP7QWidgeti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::QDataWidgetMapper(QObject * parent);
impl<'a> /*trait*/ QDataWidgetMapper_New for (&'a QObject) {
  fn New(self) -> QDataWidgetMapper {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapperC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QDataWidgetMapperC1EP7QObject(qthis, arg0)};
    let rsthis = QDataWidgetMapper{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
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
impl<'a> /*trait*/ QDataWidgetMapper_metaObject<()> for () {
  fn metaObject(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper10metaObjectEv()};
     unsafe {_ZNK17QDataWidgetMapper10metaObjectEv(rsthis.qclsinst)};
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
     unsafe {_ZN17QDataWidgetMapper6toLastEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QDataWidgetMapper_model<()> for () {
  fn model(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper5modelEv()};
     unsafe {_ZNK17QDataWidgetMapper5modelEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QDataWidgetMapper_itemDelegate<()> for () {
  fn itemDelegate(self , rsthis: & QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper12itemDelegateEv()};
     unsafe {_ZNK17QDataWidgetMapper12itemDelegateEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZN17QDataWidgetMapper6submitEv(rsthis.qclsinst)};
    return ret as i8;
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
     unsafe {_ZN17QDataWidgetMapper6toNextEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

