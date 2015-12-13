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
  // proto:  void QDataWidgetMapper::NewQDataWidgetMapper(const QDataWidgetMapper & );
  fn _ZN17QDataWidgetMapperC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QDataWidgetMapper::currentIndex();
  fn _ZNK17QDataWidgetMapper12currentIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section, const QByteArray & propertyName);
  fn _ZN17QDataWidgetMapper10addMappingEP7QWidgetiRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) ;
  // proto:  QModelIndex QDataWidgetMapper::rootIndex();
  fn _ZNK17QDataWidgetMapper9rootIndexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDataWidgetMapper::setCurrentIndex(int index);
  fn _ZN17QDataWidgetMapper15setCurrentIndexEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QWidget * QDataWidgetMapper::mappedWidgetAt(int section);
  fn _ZNK17QDataWidgetMapper14mappedWidgetAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QDataWidgetMapper::removeMapping(QWidget * widget);
  fn _ZN17QDataWidgetMapper13removeMappingEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDataWidgetMapper::toFirst();
  fn _ZN17QDataWidgetMapper7toFirstEv(qthis: *mut c_void) ;
  // proto:  void QDataWidgetMapper::currentIndexChanged(int index);
  fn _ZN17QDataWidgetMapper19currentIndexChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QDataWidgetMapper::toPrevious();
  fn _ZN17QDataWidgetMapper10toPreviousEv(qthis: *mut c_void) ;
  // proto:  void QDataWidgetMapper::setRootIndex(const QModelIndex & index);
  fn _ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDataWidgetMapper::revert();
  fn _ZN17QDataWidgetMapper6revertEv(qthis: *mut c_void) ;
  // proto:  void QDataWidgetMapper::clearMapping();
  fn _ZN17QDataWidgetMapper12clearMappingEv(qthis: *mut c_void) ;
  // proto:  int QDataWidgetMapper::mappedSection(QWidget * widget);
  fn _ZNK17QDataWidgetMapper13mappedSectionEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QByteArray QDataWidgetMapper::mappedPropertyName(QWidget * widget);
  fn _ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QDataWidgetMapper::setCurrentModelIndex(const QModelIndex & index);
  fn _ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDataWidgetMapper::FreeQDataWidgetMapper();
  fn _ZN17QDataWidgetMapperD0Ev(qthis: *mut c_void) ;
  // proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section);
  fn _ZN17QDataWidgetMapper10addMappingEP7QWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QDataWidgetMapper::NewQDataWidgetMapper(QObject * parent);
  fn _ZN17QDataWidgetMapperC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QDataWidgetMapper::metaObject();
  fn _ZNK17QDataWidgetMapper10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QDataWidgetMapper::toLast();
  fn _ZN17QDataWidgetMapper6toLastEv(qthis: *mut c_void) ;
  // proto:  QAbstractItemModel * QDataWidgetMapper::model();
  fn _ZNK17QDataWidgetMapper5modelEv(qthis: *mut c_void) ;
  // proto:  QAbstractItemDelegate * QDataWidgetMapper::itemDelegate();
  fn _ZNK17QDataWidgetMapper12itemDelegateEv(qthis: *mut c_void) ;
  // proto:  bool QDataWidgetMapper::submit();
  fn _ZN17QDataWidgetMapper6submitEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QDataWidgetMapper::toNext();
  fn _ZN17QDataWidgetMapper6toNextEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QDataWidgetMapper)=1
pub struct QDataWidgetMapper {
  pub qclsinst: *mut c_void,
}

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

// proto: void QDataWidgetMapper::NewQDataWidgetMapper(const QDataWidgetMapper & );
impl<'a> /*trait*/ QDataWidgetMapper_NewQDataWidgetMapper for (&'a  QDataWidgetMapper) {
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

impl /*struct*/ QDataWidgetMapper {
  pub fn currentIndex<T: QDataWidgetMapper_currentIndex>(&mut self, value: T) -> i32 {
    return value.currentIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_currentIndex {
  fn currentIndex(self, rsthis: &mut QDataWidgetMapper) -> i32;
}

// proto:  int QDataWidgetMapper::currentIndex();
impl<'a> /*trait*/ QDataWidgetMapper_currentIndex for () {
  fn currentIndex(self, rsthis: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper12currentIndexEv()};
    let mut ret = unsafe {_ZNK17QDataWidgetMapper12currentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn addMapping<T: QDataWidgetMapper_addMapping>(&mut self, value: T)  {
     value.addMapping(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_addMapping {
  fn addMapping(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section, const QByteArray & propertyName);
impl<'a> /*trait*/ QDataWidgetMapper_addMapping for (&'a mut QWidget, i32, &'a  QByteArray) {
  fn addMapping(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper10addMappingEP7QWidgetiRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN17QDataWidgetMapper10addMappingEP7QWidgetiRK10QByteArray(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn rootIndex<T: QDataWidgetMapper_rootIndex>(&mut self, value: T) -> QModelIndex {
    return value.rootIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_rootIndex {
  fn rootIndex(self, rsthis: &mut QDataWidgetMapper) -> QModelIndex;
}

// proto:  QModelIndex QDataWidgetMapper::rootIndex();
impl<'a> /*trait*/ QDataWidgetMapper_rootIndex for () {
  fn rootIndex(self, rsthis: &mut QDataWidgetMapper) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper9rootIndexEv()};
    let mut ret = unsafe {_ZNK17QDataWidgetMapper9rootIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn setCurrentIndex<T: QDataWidgetMapper_setCurrentIndex>(&mut self, value: T)  {
     value.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_setCurrentIndex {
  fn setCurrentIndex(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  void QDataWidgetMapper::setCurrentIndex(int index);
impl<'a> /*trait*/ QDataWidgetMapper_setCurrentIndex for (i32) {
  fn setCurrentIndex(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN17QDataWidgetMapper15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn mappedWidgetAt<T: QDataWidgetMapper_mappedWidgetAt>(&mut self, value: T) -> QWidget {
    return value.mappedWidgetAt(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_mappedWidgetAt {
  fn mappedWidgetAt(self, rsthis: &mut QDataWidgetMapper) -> QWidget;
}

// proto:  QWidget * QDataWidgetMapper::mappedWidgetAt(int section);
impl<'a> /*trait*/ QDataWidgetMapper_mappedWidgetAt for (i32) {
  fn mappedWidgetAt(self, rsthis: &mut QDataWidgetMapper) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper14mappedWidgetAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK17QDataWidgetMapper14mappedWidgetAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn removeMapping<T: QDataWidgetMapper_removeMapping>(&mut self, value: T)  {
     value.removeMapping(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_removeMapping {
  fn removeMapping(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  void QDataWidgetMapper::removeMapping(QWidget * widget);
impl<'a> /*trait*/ QDataWidgetMapper_removeMapping for (&'a mut QWidget) {
  fn removeMapping(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper13removeMappingEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QDataWidgetMapper13removeMappingEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn toFirst<T: QDataWidgetMapper_toFirst>(&mut self, value: T)  {
     value.toFirst(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_toFirst {
  fn toFirst(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  void QDataWidgetMapper::toFirst();
impl<'a> /*trait*/ QDataWidgetMapper_toFirst for () {
  fn toFirst(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper7toFirstEv()};
     unsafe {_ZN17QDataWidgetMapper7toFirstEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn currentIndexChanged<T: QDataWidgetMapper_currentIndexChanged>(&mut self, value: T)  {
     value.currentIndexChanged(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_currentIndexChanged {
  fn currentIndexChanged(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  void QDataWidgetMapper::currentIndexChanged(int index);
impl<'a> /*trait*/ QDataWidgetMapper_currentIndexChanged for (i32) {
  fn currentIndexChanged(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper19currentIndexChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN17QDataWidgetMapper19currentIndexChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn toPrevious<T: QDataWidgetMapper_toPrevious>(&mut self, value: T)  {
     value.toPrevious(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_toPrevious {
  fn toPrevious(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  void QDataWidgetMapper::toPrevious();
impl<'a> /*trait*/ QDataWidgetMapper_toPrevious for () {
  fn toPrevious(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper10toPreviousEv()};
     unsafe {_ZN17QDataWidgetMapper10toPreviousEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn setRootIndex<T: QDataWidgetMapper_setRootIndex>(&mut self, value: T)  {
     value.setRootIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_setRootIndex {
  fn setRootIndex(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  void QDataWidgetMapper::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QDataWidgetMapper_setRootIndex for (&'a  QModelIndex) {
  fn setRootIndex(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn revert<T: QDataWidgetMapper_revert>(&mut self, value: T)  {
     value.revert(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_revert {
  fn revert(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  void QDataWidgetMapper::revert();
impl<'a> /*trait*/ QDataWidgetMapper_revert for () {
  fn revert(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6revertEv()};
     unsafe {_ZN17QDataWidgetMapper6revertEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn clearMapping<T: QDataWidgetMapper_clearMapping>(&mut self, value: T)  {
     value.clearMapping(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_clearMapping {
  fn clearMapping(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  void QDataWidgetMapper::clearMapping();
impl<'a> /*trait*/ QDataWidgetMapper_clearMapping for () {
  fn clearMapping(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper12clearMappingEv()};
     unsafe {_ZN17QDataWidgetMapper12clearMappingEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn mappedSection<T: QDataWidgetMapper_mappedSection>(&mut self, value: T) -> i32 {
    return value.mappedSection(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_mappedSection {
  fn mappedSection(self, rsthis: &mut QDataWidgetMapper) -> i32;
}

// proto:  int QDataWidgetMapper::mappedSection(QWidget * widget);
impl<'a> /*trait*/ QDataWidgetMapper_mappedSection for (&'a mut QWidget) {
  fn mappedSection(self, rsthis: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper13mappedSectionEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QDataWidgetMapper13mappedSectionEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn mappedPropertyName<T: QDataWidgetMapper_mappedPropertyName>(&mut self, value: T) -> QByteArray {
    return value.mappedPropertyName(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_mappedPropertyName {
  fn mappedPropertyName(self, rsthis: &mut QDataWidgetMapper) -> QByteArray;
}

// proto:  QByteArray QDataWidgetMapper::mappedPropertyName(QWidget * widget);
impl<'a> /*trait*/ QDataWidgetMapper_mappedPropertyName for (&'a mut QWidget) {
  fn mappedPropertyName(self, rsthis: &mut QDataWidgetMapper) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn setCurrentModelIndex<T: QDataWidgetMapper_setCurrentModelIndex>(&mut self, value: T)  {
     value.setCurrentModelIndex(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_setCurrentModelIndex {
  fn setCurrentModelIndex(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  void QDataWidgetMapper::setCurrentModelIndex(const QModelIndex & index);
impl<'a> /*trait*/ QDataWidgetMapper_setCurrentModelIndex for (&'a  QModelIndex) {
  fn setCurrentModelIndex(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn FreeQDataWidgetMapper<T: QDataWidgetMapper_FreeQDataWidgetMapper>(&mut self, value: T)  {
     value.FreeQDataWidgetMapper(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_FreeQDataWidgetMapper {
  fn FreeQDataWidgetMapper(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  void QDataWidgetMapper::FreeQDataWidgetMapper();
impl<'a> /*trait*/ QDataWidgetMapper_FreeQDataWidgetMapper for () {
  fn FreeQDataWidgetMapper(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapperD0Ev()};
     unsafe {_ZN17QDataWidgetMapperD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QDataWidgetMapper::addMapping(QWidget * widget, int section);
impl<'a> /*trait*/ QDataWidgetMapper_addMapping for (&'a mut QWidget, i32) {
  fn addMapping(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper10addMappingEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN17QDataWidgetMapper10addMappingEP7QWidgeti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto: void QDataWidgetMapper::NewQDataWidgetMapper(QObject * parent);
impl<'a> /*trait*/ QDataWidgetMapper_NewQDataWidgetMapper for (&'a mut QObject) {
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

impl /*struct*/ QDataWidgetMapper {
  pub fn metaObject<T: QDataWidgetMapper_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_metaObject {
  fn metaObject(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  const QMetaObject * QDataWidgetMapper::metaObject();
impl<'a> /*trait*/ QDataWidgetMapper_metaObject for () {
  fn metaObject(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper10metaObjectEv()};
     unsafe {_ZNK17QDataWidgetMapper10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn toLast<T: QDataWidgetMapper_toLast>(&mut self, value: T)  {
     value.toLast(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_toLast {
  fn toLast(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  void QDataWidgetMapper::toLast();
impl<'a> /*trait*/ QDataWidgetMapper_toLast for () {
  fn toLast(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6toLastEv()};
     unsafe {_ZN17QDataWidgetMapper6toLastEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn model<T: QDataWidgetMapper_model>(&mut self, value: T)  {
     value.model(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_model {
  fn model(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  QAbstractItemModel * QDataWidgetMapper::model();
impl<'a> /*trait*/ QDataWidgetMapper_model for () {
  fn model(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper5modelEv()};
     unsafe {_ZNK17QDataWidgetMapper5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn itemDelegate<T: QDataWidgetMapper_itemDelegate>(&mut self, value: T)  {
     value.itemDelegate(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_itemDelegate {
  fn itemDelegate(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  QAbstractItemDelegate * QDataWidgetMapper::itemDelegate();
impl<'a> /*trait*/ QDataWidgetMapper_itemDelegate for () {
  fn itemDelegate(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper12itemDelegateEv()};
     unsafe {_ZNK17QDataWidgetMapper12itemDelegateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn submit<T: QDataWidgetMapper_submit>(&mut self, value: T) -> i8 {
    return value.submit(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_submit {
  fn submit(self, rsthis: &mut QDataWidgetMapper) -> i8;
}

// proto:  bool QDataWidgetMapper::submit();
impl<'a> /*trait*/ QDataWidgetMapper_submit for () {
  fn submit(self, rsthis: &mut QDataWidgetMapper) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6submitEv()};
    let mut ret = unsafe {_ZN17QDataWidgetMapper6submitEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn toNext<T: QDataWidgetMapper_toNext>(&mut self, value: T)  {
     value.toNext(self);
    // return 1;
  }
}

pub trait QDataWidgetMapper_toNext {
  fn toNext(self, rsthis: &mut QDataWidgetMapper) ;
}

// proto:  void QDataWidgetMapper::toNext();
impl<'a> /*trait*/ QDataWidgetMapper_toNext for () {
  fn toNext(self, rsthis: &mut QDataWidgetMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6toNextEv()};
     unsafe {_ZN17QDataWidgetMapper6toNextEv(rsthis.qclsinst)};
    // return 1;
  }
}

