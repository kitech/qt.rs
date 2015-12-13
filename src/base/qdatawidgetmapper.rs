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
  // proto: void QDataWidgetMapper::NewQDataWidgetMapper(const QDataWidgetMapper & );
  fn _ZN17QDataWidgetMapperC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QDataWidgetMapper::currentIndex();
  fn _ZNK17QDataWidgetMapper12currentIndexEv() -> i32;
  // proto: void QDataWidgetMapper::addMapping(QWidget * widget, int section, const QByteArray & propertyName);
  fn _ZN17QDataWidgetMapper10addMappingEP7QWidgetiRK10QByteArray(arg0: *mut c_void, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: QModelIndex QDataWidgetMapper::rootIndex();
  fn _ZNK17QDataWidgetMapper9rootIndexEv() -> i32;
  // proto: void QDataWidgetMapper::setCurrentIndex(int index);
  fn _ZN17QDataWidgetMapper15setCurrentIndexEi(arg0: c_int) -> i32;
  // proto: QWidget * QDataWidgetMapper::mappedWidgetAt(int section);
  fn _ZNK17QDataWidgetMapper14mappedWidgetAtEi(arg0: c_int) -> i32;
  // proto: void QDataWidgetMapper::removeMapping(QWidget * widget);
  fn _ZN17QDataWidgetMapper13removeMappingEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QDataWidgetMapper::toFirst();
  fn _ZN17QDataWidgetMapper7toFirstEv() -> i32;
  // proto: void QDataWidgetMapper::currentIndexChanged(int index);
  fn _ZN17QDataWidgetMapper19currentIndexChangedEi(arg0: c_int) -> i32;
  // proto: void QDataWidgetMapper::toPrevious();
  fn _ZN17QDataWidgetMapper10toPreviousEv() -> i32;
  // proto: void QDataWidgetMapper::setRootIndex(const QModelIndex & index);
  fn _ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QDataWidgetMapper::revert();
  fn _ZN17QDataWidgetMapper6revertEv() -> i32;
  // proto: void QDataWidgetMapper::clearMapping();
  fn _ZN17QDataWidgetMapper12clearMappingEv() -> i32;
  // proto: int QDataWidgetMapper::mappedSection(QWidget * widget);
  fn _ZNK17QDataWidgetMapper13mappedSectionEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: QByteArray QDataWidgetMapper::mappedPropertyName(QWidget * widget);
  fn _ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QDataWidgetMapper::setCurrentModelIndex(const QModelIndex & index);
  fn _ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QDataWidgetMapper::FreeQDataWidgetMapper();
  fn _ZN17QDataWidgetMapperD0Ev() -> i32;
  // proto: void QDataWidgetMapper::addMapping(QWidget * widget, int section);
  fn _ZN17QDataWidgetMapper10addMappingEP7QWidgeti(arg0: *mut c_void, arg1: c_int) -> i32;
  // proto: void QDataWidgetMapper::NewQDataWidgetMapper(QObject * parent);
  fn _ZN17QDataWidgetMapperC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QDataWidgetMapper::metaObject();
  fn _ZNK17QDataWidgetMapper10metaObjectEv() -> i32;
  // proto: void QDataWidgetMapper::toLast();
  fn _ZN17QDataWidgetMapper6toLastEv() -> i32;
  // proto: QAbstractItemModel * QDataWidgetMapper::model();
  fn _ZNK17QDataWidgetMapper5modelEv() -> i32;
  // proto: QAbstractItemDelegate * QDataWidgetMapper::itemDelegate();
  fn _ZNK17QDataWidgetMapper12itemDelegateEv() -> i32;
  // proto: bool QDataWidgetMapper::submit();
  fn _ZN17QDataWidgetMapper6submitEv() -> i32;
  // proto: void QDataWidgetMapper::toNext();
  fn _ZN17QDataWidgetMapper6toNextEv() -> i32;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QDataWidgetMapperC1ERKS_(qthis, arg0)};
    let rsthis = QDataWidgetMapper{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn currentIndex<T: QDataWidgetMapper_currentIndex>(&mut self, value: T) -> i32 {
    value.currentIndex(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_currentIndex {
  fn currentIndex(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: int QDataWidgetMapper::currentIndex();
impl<'a> /*trait*/ QDataWidgetMapper_currentIndex for () {
  fn currentIndex(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper12currentIndexEv()};
    unsafe {_ZNK17QDataWidgetMapper12currentIndexEv()};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn addMapping<T: QDataWidgetMapper_addMapping>(&mut self, value: T) -> i32 {
    value.addMapping(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_addMapping {
  fn addMapping(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: void QDataWidgetMapper::addMapping(QWidget * widget, int section, const QByteArray & propertyName);
impl<'a> /*trait*/ QDataWidgetMapper_addMapping for (&'a mut QWidget, i32, &'a  QByteArray) {
  fn addMapping(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper10addMappingEP7QWidgetiRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN17QDataWidgetMapper10addMappingEP7QWidgetiRK10QByteArray(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn rootIndex<T: QDataWidgetMapper_rootIndex>(&mut self, value: T) -> i32 {
    value.rootIndex(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_rootIndex {
  fn rootIndex(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: QModelIndex QDataWidgetMapper::rootIndex();
impl<'a> /*trait*/ QDataWidgetMapper_rootIndex for () {
  fn rootIndex(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper9rootIndexEv()};
    unsafe {_ZNK17QDataWidgetMapper9rootIndexEv()};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn setCurrentIndex<T: QDataWidgetMapper_setCurrentIndex>(&mut self, value: T) -> i32 {
    value.setCurrentIndex(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_setCurrentIndex {
  fn setCurrentIndex(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: void QDataWidgetMapper::setCurrentIndex(int index);
impl<'a> /*trait*/ QDataWidgetMapper_setCurrentIndex for (i32) {
  fn setCurrentIndex(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper15setCurrentIndexEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN17QDataWidgetMapper15setCurrentIndexEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn mappedWidgetAt<T: QDataWidgetMapper_mappedWidgetAt>(&mut self, value: T) -> i32 {
    value.mappedWidgetAt(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_mappedWidgetAt {
  fn mappedWidgetAt(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: QWidget * QDataWidgetMapper::mappedWidgetAt(int section);
impl<'a> /*trait*/ QDataWidgetMapper_mappedWidgetAt for (i32) {
  fn mappedWidgetAt(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper14mappedWidgetAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK17QDataWidgetMapper14mappedWidgetAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn removeMapping<T: QDataWidgetMapper_removeMapping>(&mut self, value: T) -> i32 {
    value.removeMapping(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_removeMapping {
  fn removeMapping(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: void QDataWidgetMapper::removeMapping(QWidget * widget);
impl<'a> /*trait*/ QDataWidgetMapper_removeMapping for (&'a mut QWidget) {
  fn removeMapping(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper13removeMappingEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QDataWidgetMapper13removeMappingEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn toFirst<T: QDataWidgetMapper_toFirst>(&mut self, value: T) -> i32 {
    value.toFirst(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_toFirst {
  fn toFirst(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: void QDataWidgetMapper::toFirst();
impl<'a> /*trait*/ QDataWidgetMapper_toFirst for () {
  fn toFirst(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper7toFirstEv()};
    unsafe {_ZN17QDataWidgetMapper7toFirstEv()};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn currentIndexChanged<T: QDataWidgetMapper_currentIndexChanged>(&mut self, value: T) -> i32 {
    value.currentIndexChanged(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_currentIndexChanged {
  fn currentIndexChanged(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: void QDataWidgetMapper::currentIndexChanged(int index);
impl<'a> /*trait*/ QDataWidgetMapper_currentIndexChanged for (i32) {
  fn currentIndexChanged(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper19currentIndexChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN17QDataWidgetMapper19currentIndexChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn toPrevious<T: QDataWidgetMapper_toPrevious>(&mut self, value: T) -> i32 {
    value.toPrevious(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_toPrevious {
  fn toPrevious(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: void QDataWidgetMapper::toPrevious();
impl<'a> /*trait*/ QDataWidgetMapper_toPrevious for () {
  fn toPrevious(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper10toPreviousEv()};
    unsafe {_ZN17QDataWidgetMapper10toPreviousEv()};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn setRootIndex<T: QDataWidgetMapper_setRootIndex>(&mut self, value: T) -> i32 {
    value.setRootIndex(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_setRootIndex {
  fn setRootIndex(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: void QDataWidgetMapper::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QDataWidgetMapper_setRootIndex for (&'a  QModelIndex) {
  fn setRootIndex(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QDataWidgetMapper12setRootIndexERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn revert<T: QDataWidgetMapper_revert>(&mut self, value: T) -> i32 {
    value.revert(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_revert {
  fn revert(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: void QDataWidgetMapper::revert();
impl<'a> /*trait*/ QDataWidgetMapper_revert for () {
  fn revert(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6revertEv()};
    unsafe {_ZN17QDataWidgetMapper6revertEv()};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn clearMapping<T: QDataWidgetMapper_clearMapping>(&mut self, value: T) -> i32 {
    value.clearMapping(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_clearMapping {
  fn clearMapping(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: void QDataWidgetMapper::clearMapping();
impl<'a> /*trait*/ QDataWidgetMapper_clearMapping for () {
  fn clearMapping(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper12clearMappingEv()};
    unsafe {_ZN17QDataWidgetMapper12clearMappingEv()};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn mappedSection<T: QDataWidgetMapper_mappedSection>(&mut self, value: T) -> i32 {
    value.mappedSection(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_mappedSection {
  fn mappedSection(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: int QDataWidgetMapper::mappedSection(QWidget * widget);
impl<'a> /*trait*/ QDataWidgetMapper_mappedSection for (&'a mut QWidget) {
  fn mappedSection(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper13mappedSectionEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK17QDataWidgetMapper13mappedSectionEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn mappedPropertyName<T: QDataWidgetMapper_mappedPropertyName>(&mut self, value: T) -> i32 {
    value.mappedPropertyName(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_mappedPropertyName {
  fn mappedPropertyName(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: QByteArray QDataWidgetMapper::mappedPropertyName(QWidget * widget);
impl<'a> /*trait*/ QDataWidgetMapper_mappedPropertyName for (&'a mut QWidget) {
  fn mappedPropertyName(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK17QDataWidgetMapper18mappedPropertyNameEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn setCurrentModelIndex<T: QDataWidgetMapper_setCurrentModelIndex>(&mut self, value: T) -> i32 {
    value.setCurrentModelIndex(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_setCurrentModelIndex {
  fn setCurrentModelIndex(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: void QDataWidgetMapper::setCurrentModelIndex(const QModelIndex & index);
impl<'a> /*trait*/ QDataWidgetMapper_setCurrentModelIndex for (&'a  QModelIndex) {
  fn setCurrentModelIndex(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QDataWidgetMapper20setCurrentModelIndexERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn FreeQDataWidgetMapper<T: QDataWidgetMapper_FreeQDataWidgetMapper>(&mut self, value: T) -> i32 {
    value.FreeQDataWidgetMapper(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_FreeQDataWidgetMapper {
  fn FreeQDataWidgetMapper(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: void QDataWidgetMapper::FreeQDataWidgetMapper();
impl<'a> /*trait*/ QDataWidgetMapper_FreeQDataWidgetMapper for () {
  fn FreeQDataWidgetMapper(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapperD0Ev()};
    unsafe {_ZN17QDataWidgetMapperD0Ev()};
    return 1;
  }
}

// proto: void QDataWidgetMapper::addMapping(QWidget * widget, int section);
impl<'a> /*trait*/ QDataWidgetMapper_addMapping for (&'a mut QWidget, i32) {
  fn addMapping(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper10addMappingEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN17QDataWidgetMapper10addMappingEP7QWidgeti(arg0, arg1)};
    return 1;
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
  pub fn metaObject<T: QDataWidgetMapper_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_metaObject {
  fn metaObject(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: const QMetaObject * QDataWidgetMapper::metaObject();
impl<'a> /*trait*/ QDataWidgetMapper_metaObject for () {
  fn metaObject(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper10metaObjectEv()};
    unsafe {_ZNK17QDataWidgetMapper10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn toLast<T: QDataWidgetMapper_toLast>(&mut self, value: T) -> i32 {
    value.toLast(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_toLast {
  fn toLast(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: void QDataWidgetMapper::toLast();
impl<'a> /*trait*/ QDataWidgetMapper_toLast for () {
  fn toLast(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6toLastEv()};
    unsafe {_ZN17QDataWidgetMapper6toLastEv()};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn model<T: QDataWidgetMapper_model>(&mut self, value: T) -> i32 {
    value.model(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_model {
  fn model(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: QAbstractItemModel * QDataWidgetMapper::model();
impl<'a> /*trait*/ QDataWidgetMapper_model for () {
  fn model(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper5modelEv()};
    unsafe {_ZNK17QDataWidgetMapper5modelEv()};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn itemDelegate<T: QDataWidgetMapper_itemDelegate>(&mut self, value: T) -> i32 {
    value.itemDelegate(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_itemDelegate {
  fn itemDelegate(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: QAbstractItemDelegate * QDataWidgetMapper::itemDelegate();
impl<'a> /*trait*/ QDataWidgetMapper_itemDelegate for () {
  fn itemDelegate(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QDataWidgetMapper12itemDelegateEv()};
    unsafe {_ZNK17QDataWidgetMapper12itemDelegateEv()};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn submit<T: QDataWidgetMapper_submit>(&mut self, value: T) -> i32 {
    value.submit(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_submit {
  fn submit(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: bool QDataWidgetMapper::submit();
impl<'a> /*trait*/ QDataWidgetMapper_submit for () {
  fn submit(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6submitEv()};
    unsafe {_ZN17QDataWidgetMapper6submitEv()};
    return 1;
  }
}

impl /*struct*/ QDataWidgetMapper {
  pub fn toNext<T: QDataWidgetMapper_toNext>(&mut self, value: T) -> i32 {
    value.toNext(self);
    return 1;
  }
}

pub trait QDataWidgetMapper_toNext {
  fn toNext(self, this: &mut QDataWidgetMapper) -> i32;
}

// proto: void QDataWidgetMapper::toNext();
impl<'a> /*trait*/ QDataWidgetMapper_toNext for () {
  fn toNext(self, this: &mut QDataWidgetMapper) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QDataWidgetMapper6toNextEv()};
    unsafe {_ZN17QDataWidgetMapper6toNextEv()};
    return 1;
  }
}

