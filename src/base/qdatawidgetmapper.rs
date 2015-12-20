// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qbytearray::QByteArray;
use super::qmodelindex::QModelIndex;
use super::qobject::QObject;

// ext block begin
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
}

// body block begin
// class sizeof(QDataWidgetMapper)=1
pub struct QDataWidgetMapper {
  pub qclsinst: *mut c_void,
}

  // proto:  void QDataWidgetMapper::QDataWidgetMapper(const QDataWidgetMapper & );
impl /*struct*/ QDataWidgetMapper {
  pub fn NewQDataWidgetMapper<T: QDataWidgetMapper_NewQDataWidgetMapper>(value: T) -> QDataWidgetMapper {
    let rsthis = value.NewQDataWidgetMapper();
    return rsthis;
    // return 1;
  }
}

pub trait QDataWidgetMapper_NewQDataWidgetMapper {
  fn NewQDataWidgetMapper(self) -> QDataWidgetMapper;
}

  // proto:  void QDataWidgetMapper::QDataWidgetMapper(const QDataWidgetMapper & );
impl<'a> /*trait*/ QDataWidgetMapper_NewQDataWidgetMapper for (QDataWidgetMapper) {
  fn NewQDataWidgetMapper(self) -> QDataWidgetMapper {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapperC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QDataWidgetMapperC1ERKS_(qthis, arg0)};
    let rsthis = QDataWidgetMapper{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QDataWidgetMapper::currentIndex();
impl /*struct*/ QDataWidgetMapper {
  pub fn currentIndex<RetType, T: QDataWidgetMapper_currentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_currentIndex<RetType> {
  fn currentIndex(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  int QDataWidgetMapper::currentIndex();
impl<'a> /*trait*/ QDataWidgetMapper_currentIndex<i32> for () {
  fn currentIndex(self , rsthis: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper12currentIndexEv()};
    let mut ret = unsafe {_ZNK17QDataWidgetMapper12currentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section, const QByteArray & propertyName);
impl /*struct*/ QDataWidgetMapper {
  pub fn addMapping<RetType, T: QDataWidgetMapper_addMapping<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addMapping(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_addMapping<RetType> {
  fn addMapping(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section, const QByteArray & propertyName);
impl<'a> /*trait*/ QDataWidgetMapper_addMapping<()> for (QWidget, i32, QByteArray) {
  fn addMapping(self , rsthis: &mut QDataWidgetMapper) -> () {
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
  pub fn rootIndex<RetType, T: QDataWidgetMapper_rootIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rootIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_rootIndex<RetType> {
  fn rootIndex(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  QModelIndex QDataWidgetMapper::rootIndex();
impl<'a> /*trait*/ QDataWidgetMapper_rootIndex<QModelIndex> for () {
  fn rootIndex(self , rsthis: &mut QDataWidgetMapper) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper9rootIndexEv()};
    let mut ret = unsafe {_ZNK17QDataWidgetMapper9rootIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::setCurrentIndex(int index);
impl /*struct*/ QDataWidgetMapper {
  pub fn setCurrentIndex<RetType, T: QDataWidgetMapper_setCurrentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_setCurrentIndex<RetType> {
  fn setCurrentIndex(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::setCurrentIndex(int index);
impl<'a> /*trait*/ QDataWidgetMapper_setCurrentIndex<()> for (i32) {
  fn setCurrentIndex(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN17QDataWidgetMapper15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QDataWidgetMapper::mappedWidgetAt(int section);
impl /*struct*/ QDataWidgetMapper {
  pub fn mappedWidgetAt<RetType, T: QDataWidgetMapper_mappedWidgetAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mappedWidgetAt(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_mappedWidgetAt<RetType> {
  fn mappedWidgetAt(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  QWidget * QDataWidgetMapper::mappedWidgetAt(int section);
impl<'a> /*trait*/ QDataWidgetMapper_mappedWidgetAt<QWidget> for (i32) {
  fn mappedWidgetAt(self , rsthis: &mut QDataWidgetMapper) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper14mappedWidgetAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK17QDataWidgetMapper14mappedWidgetAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::removeMapping(QWidget * widget);
impl /*struct*/ QDataWidgetMapper {
  pub fn removeMapping<RetType, T: QDataWidgetMapper_removeMapping<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeMapping(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_removeMapping<RetType> {
  fn removeMapping(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::removeMapping(QWidget * widget);
impl<'a> /*trait*/ QDataWidgetMapper_removeMapping<()> for (QWidget) {
  fn removeMapping(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper13removeMappingEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QDataWidgetMapper13removeMappingEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::toFirst();
impl /*struct*/ QDataWidgetMapper {
  pub fn toFirst<RetType, T: QDataWidgetMapper_toFirst<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toFirst(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_toFirst<RetType> {
  fn toFirst(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::toFirst();
impl<'a> /*trait*/ QDataWidgetMapper_toFirst<()> for () {
  fn toFirst(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper7toFirstEv()};
     unsafe {_ZN17QDataWidgetMapper7toFirstEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::currentIndexChanged(int index);
impl /*struct*/ QDataWidgetMapper {
  pub fn currentIndexChanged<RetType, T: QDataWidgetMapper_currentIndexChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentIndexChanged(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_currentIndexChanged<RetType> {
  fn currentIndexChanged(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::currentIndexChanged(int index);
impl<'a> /*trait*/ QDataWidgetMapper_currentIndexChanged<()> for (i32) {
  fn currentIndexChanged(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper19currentIndexChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN17QDataWidgetMapper19currentIndexChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::toPrevious();
impl /*struct*/ QDataWidgetMapper {
  pub fn toPrevious<RetType, T: QDataWidgetMapper_toPrevious<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toPrevious(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_toPrevious<RetType> {
  fn toPrevious(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::toPrevious();
impl<'a> /*trait*/ QDataWidgetMapper_toPrevious<()> for () {
  fn toPrevious(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper10toPreviousEv()};
     unsafe {_ZN17QDataWidgetMapper10toPreviousEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::setRootIndex(const QModelIndex & index);
impl /*struct*/ QDataWidgetMapper {
  pub fn setRootIndex<RetType, T: QDataWidgetMapper_setRootIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRootIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_setRootIndex<RetType> {
  fn setRootIndex(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QDataWidgetMapper_setRootIndex<()> for (QModelIndex) {
  fn setRootIndex(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::revert();
impl /*struct*/ QDataWidgetMapper {
  pub fn revert<RetType, T: QDataWidgetMapper_revert<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.revert(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_revert<RetType> {
  fn revert(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::revert();
impl<'a> /*trait*/ QDataWidgetMapper_revert<()> for () {
  fn revert(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6revertEv()};
     unsafe {_ZN17QDataWidgetMapper6revertEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::clearMapping();
impl /*struct*/ QDataWidgetMapper {
  pub fn clearMapping<RetType, T: QDataWidgetMapper_clearMapping<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearMapping(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_clearMapping<RetType> {
  fn clearMapping(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::clearMapping();
impl<'a> /*trait*/ QDataWidgetMapper_clearMapping<()> for () {
  fn clearMapping(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper12clearMappingEv()};
     unsafe {_ZN17QDataWidgetMapper12clearMappingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QDataWidgetMapper::mappedSection(QWidget * widget);
impl /*struct*/ QDataWidgetMapper {
  pub fn mappedSection<RetType, T: QDataWidgetMapper_mappedSection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mappedSection(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_mappedSection<RetType> {
  fn mappedSection(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  int QDataWidgetMapper::mappedSection(QWidget * widget);
impl<'a> /*trait*/ QDataWidgetMapper_mappedSection<i32> for (QWidget) {
  fn mappedSection(self , rsthis: &mut QDataWidgetMapper) -> i32 {
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
  pub fn mappedPropertyName<RetType, T: QDataWidgetMapper_mappedPropertyName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mappedPropertyName(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_mappedPropertyName<RetType> {
  fn mappedPropertyName(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  QByteArray QDataWidgetMapper::mappedPropertyName(QWidget * widget);
impl<'a> /*trait*/ QDataWidgetMapper_mappedPropertyName<QByteArray> for (QWidget) {
  fn mappedPropertyName(self , rsthis: &mut QDataWidgetMapper) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::setCurrentModelIndex(const QModelIndex & index);
impl /*struct*/ QDataWidgetMapper {
  pub fn setCurrentModelIndex<RetType, T: QDataWidgetMapper_setCurrentModelIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentModelIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_setCurrentModelIndex<RetType> {
  fn setCurrentModelIndex(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::setCurrentModelIndex(const QModelIndex & index);
impl<'a> /*trait*/ QDataWidgetMapper_setCurrentModelIndex<()> for (QModelIndex) {
  fn setCurrentModelIndex(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::~QDataWidgetMapper();
impl /*struct*/ QDataWidgetMapper {
  pub fn FreeQDataWidgetMapper<RetType, T: QDataWidgetMapper_FreeQDataWidgetMapper<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQDataWidgetMapper(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_FreeQDataWidgetMapper<RetType> {
  fn FreeQDataWidgetMapper(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::~QDataWidgetMapper();
impl<'a> /*trait*/ QDataWidgetMapper_FreeQDataWidgetMapper<()> for () {
  fn FreeQDataWidgetMapper(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapperD0Ev()};
     unsafe {_ZN17QDataWidgetMapperD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section);
impl<'a> /*trait*/ QDataWidgetMapper_addMapping<()> for (QWidget, i32) {
  fn addMapping(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper10addMappingEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN17QDataWidgetMapper10addMappingEP7QWidgeti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::QDataWidgetMapper(QObject * parent);
impl<'a> /*trait*/ QDataWidgetMapper_NewQDataWidgetMapper for (QObject) {
  fn NewQDataWidgetMapper(self) -> QDataWidgetMapper {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapperC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QDataWidgetMapperC1EP7QObject(qthis, arg0)};
    let rsthis = QDataWidgetMapper{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QDataWidgetMapper::metaObject();
impl /*struct*/ QDataWidgetMapper {
  pub fn metaObject<RetType, T: QDataWidgetMapper_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  const QMetaObject * QDataWidgetMapper::metaObject();
impl<'a> /*trait*/ QDataWidgetMapper_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper10metaObjectEv()};
     unsafe {_ZNK17QDataWidgetMapper10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::toLast();
impl /*struct*/ QDataWidgetMapper {
  pub fn toLast<RetType, T: QDataWidgetMapper_toLast<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toLast(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_toLast<RetType> {
  fn toLast(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::toLast();
impl<'a> /*trait*/ QDataWidgetMapper_toLast<()> for () {
  fn toLast(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6toLastEv()};
     unsafe {_ZN17QDataWidgetMapper6toLastEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAbstractItemModel * QDataWidgetMapper::model();
impl /*struct*/ QDataWidgetMapper {
  pub fn model<RetType, T: QDataWidgetMapper_model<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_model<RetType> {
  fn model(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  QAbstractItemModel * QDataWidgetMapper::model();
impl<'a> /*trait*/ QDataWidgetMapper_model<()> for () {
  fn model(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper5modelEv()};
     unsafe {_ZNK17QDataWidgetMapper5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAbstractItemDelegate * QDataWidgetMapper::itemDelegate();
impl /*struct*/ QDataWidgetMapper {
  pub fn itemDelegate<RetType, T: QDataWidgetMapper_itemDelegate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemDelegate(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_itemDelegate<RetType> {
  fn itemDelegate(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  QAbstractItemDelegate * QDataWidgetMapper::itemDelegate();
impl<'a> /*trait*/ QDataWidgetMapper_itemDelegate<()> for () {
  fn itemDelegate(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper12itemDelegateEv()};
     unsafe {_ZNK17QDataWidgetMapper12itemDelegateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QDataWidgetMapper::submit();
impl /*struct*/ QDataWidgetMapper {
  pub fn submit<RetType, T: QDataWidgetMapper_submit<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.submit(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_submit<RetType> {
  fn submit(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  bool QDataWidgetMapper::submit();
impl<'a> /*trait*/ QDataWidgetMapper_submit<i8> for () {
  fn submit(self , rsthis: &mut QDataWidgetMapper) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6submitEv()};
    let mut ret = unsafe {_ZN17QDataWidgetMapper6submitEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QDataWidgetMapper::toNext();
impl /*struct*/ QDataWidgetMapper {
  pub fn toNext<RetType, T: QDataWidgetMapper_toNext<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toNext(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_toNext<RetType> {
  fn toNext(self , rsthis: &mut QDataWidgetMapper) -> RetType;
}

  // proto:  void QDataWidgetMapper::toNext();
impl<'a> /*trait*/ QDataWidgetMapper_toNext<()> for () {
  fn toNext(self , rsthis: &mut QDataWidgetMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6toNextEv()};
     unsafe {_ZN17QDataWidgetMapper6toNextEv(rsthis.qclsinst)};
    // return 1;
  }
}

