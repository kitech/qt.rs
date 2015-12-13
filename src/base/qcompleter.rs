// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qrect::QRect;
use super::qmodelindex::QModelIndex;
use super::qstringlist::QStringList;
use super::qstring::QString;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QCompleter::NewQCompleter(QObject * parent);
  fn _ZN10QCompleterC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QCompleter::metaObject();
  fn _ZNK10QCompleter10metaObjectEv() -> i32;
  // proto: QAbstractItemView * QCompleter::popup();
  fn _ZNK10QCompleter5popupEv() -> i32;
  // proto: void QCompleter::complete(const QRect & rect);
  fn _ZN10QCompleter8completeERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QCompleter::activated(const QModelIndex & index);
  fn _ZN10QCompleter9activatedERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QCompleter::setCompletionRole(int role);
  fn _ZN10QCompleter17setCompletionRoleEi(arg0: c_int) -> i32;
  // proto: int QCompleter::completionCount();
  fn _ZNK10QCompleter15completionCountEv() -> i32;
  // proto: void QCompleter::NewQCompleter(const QStringList & completions, QObject * parent);
  fn _ZN10QCompleterC1ERK11QStringListP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: QModelIndex QCompleter::currentIndex();
  fn _ZNK10QCompleter12currentIndexEv() -> i32;
  // proto: QString QCompleter::pathFromIndex(const QModelIndex & index);
  fn _ZNK10QCompleter13pathFromIndexERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QCompleter::highlighted(const QModelIndex & index);
  fn _ZN10QCompleter11highlightedERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QCompleter::setMaxVisibleItems(int maxItems);
  fn _ZN10QCompleter18setMaxVisibleItemsEi(arg0: c_int) -> i32;
  // proto: int QCompleter::completionColumn();
  fn _ZNK10QCompleter16completionColumnEv() -> i32;
  // proto: int QCompleter::maxVisibleItems();
  fn _ZNK10QCompleter15maxVisibleItemsEv() -> i32;
  // proto: void QCompleter::FreeQCompleter();
  fn _ZN10QCompleterD0Ev() -> i32;
  // proto: void QCompleter::setWrapAround(bool wrap);
  fn _ZN10QCompleter13setWrapAroundEb(arg0: int8_t) -> i32;
  // proto: QStringList QCompleter::splitPath(const QString & path);
  fn _ZNK10QCompleter9splitPathERK7QString(arg0: *const c_void) -> i32;
  // proto: QAbstractItemModel * QCompleter::model();
  fn _ZNK10QCompleter5modelEv() -> i32;
  // proto: QString QCompleter::currentCompletion();
  fn _ZNK10QCompleter17currentCompletionEv() -> i32;
  // proto: void QCompleter::setCompletionColumn(int column);
  fn _ZN10QCompleter19setCompletionColumnEi(arg0: c_int) -> i32;
  // proto: void QCompleter::highlighted(const QString & text);
  fn _ZN10QCompleter11highlightedERK7QString(arg0: *const c_void) -> i32;
  // proto: void QCompleter::setCompletionPrefix(const QString & prefix);
  fn _ZN10QCompleter19setCompletionPrefixERK7QString(arg0: *const c_void) -> i32;
  // proto: QAbstractItemModel * QCompleter::completionModel();
  fn _ZNK10QCompleter15completionModelEv() -> i32;
  // proto: bool QCompleter::setCurrentRow(int row);
  fn _ZN10QCompleter13setCurrentRowEi(arg0: c_int) -> i32;
  // proto: int QCompleter::currentRow();
  fn _ZNK10QCompleter10currentRowEv() -> i32;
  // proto: void QCompleter::activated(const QString & text);
  fn _ZN10QCompleter9activatedERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QCompleter::wrapAround();
  fn _ZNK10QCompleter10wrapAroundEv() -> i32;
  // proto: void QCompleter::NewQCompleter(const QCompleter & );
  fn _ZN10QCompleterC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QWidget * QCompleter::widget();
  fn _ZNK10QCompleter6widgetEv() -> i32;
  // proto: int QCompleter::completionRole();
  fn _ZNK10QCompleter14completionRoleEv() -> i32;
  // proto: QString QCompleter::completionPrefix();
  fn _ZNK10QCompleter16completionPrefixEv() -> i32;
  // proto: void QCompleter::setWidget(QWidget * widget);
  fn _ZN10QCompleter9setWidgetEP7QWidget(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QCompleter)=1
pub struct QCompleter {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCompleter {
  pub fn NewQCompleter<T: QCompleter_NewQCompleter>(value: T) -> QCompleter {
    let rsthis = value.NewQCompleter();
    return rsthis;
    // return 1;
  }
}

pub trait QCompleter_NewQCompleter {
  fn NewQCompleter(self) -> QCompleter;
}

// proto: void QCompleter::NewQCompleter(QObject * parent);
impl<'a> /*trait*/ QCompleter_NewQCompleter for (&'a mut QObject) {
  fn NewQCompleter(self) -> QCompleter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleterC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QCompleterC1EP7QObject(qthis, arg0)};
    let rsthis = QCompleter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn metaObject<T: QCompleter_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QCompleter_metaObject {
  fn metaObject(self, this: &mut QCompleter) -> i32;
}

// proto: const QMetaObject * QCompleter::metaObject();
impl<'a> /*trait*/ QCompleter_metaObject for () {
  fn metaObject(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter10metaObjectEv()};
    unsafe {_ZNK10QCompleter10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn popup<T: QCompleter_popup>(&mut self, value: T) -> i32 {
    value.popup(self);
    return 1;
  }
}

pub trait QCompleter_popup {
  fn popup(self, this: &mut QCompleter) -> i32;
}

// proto: QAbstractItemView * QCompleter::popup();
impl<'a> /*trait*/ QCompleter_popup for () {
  fn popup(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter5popupEv()};
    unsafe {_ZNK10QCompleter5popupEv()};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn complete<T: QCompleter_complete>(&mut self, value: T) -> i32 {
    value.complete(self);
    return 1;
  }
}

pub trait QCompleter_complete {
  fn complete(self, this: &mut QCompleter) -> i32;
}

// proto: void QCompleter::complete(const QRect & rect);
impl<'a> /*trait*/ QCompleter_complete for (&'a  QRect) {
  fn complete(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter8completeERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QCompleter8completeERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn activated<T: QCompleter_activated>(&mut self, value: T) -> i32 {
    value.activated(self);
    return 1;
  }
}

pub trait QCompleter_activated {
  fn activated(self, this: &mut QCompleter) -> i32;
}

// proto: void QCompleter::activated(const QModelIndex & index);
impl<'a> /*trait*/ QCompleter_activated for (&'a  QModelIndex) {
  fn activated(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter9activatedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QCompleter9activatedERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setCompletionRole<T: QCompleter_setCompletionRole>(&mut self, value: T) -> i32 {
    value.setCompletionRole(self);
    return 1;
  }
}

pub trait QCompleter_setCompletionRole {
  fn setCompletionRole(self, this: &mut QCompleter) -> i32;
}

// proto: void QCompleter::setCompletionRole(int role);
impl<'a> /*trait*/ QCompleter_setCompletionRole for (i32) {
  fn setCompletionRole(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter17setCompletionRoleEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QCompleter17setCompletionRoleEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn completionCount<T: QCompleter_completionCount>(&mut self, value: T) -> i32 {
    value.completionCount(self);
    return 1;
  }
}

pub trait QCompleter_completionCount {
  fn completionCount(self, this: &mut QCompleter) -> i32;
}

// proto: int QCompleter::completionCount();
impl<'a> /*trait*/ QCompleter_completionCount for () {
  fn completionCount(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter15completionCountEv()};
    unsafe {_ZNK10QCompleter15completionCountEv()};
    return 1;
  }
}

// proto: void QCompleter::NewQCompleter(const QStringList & completions, QObject * parent);
impl<'a> /*trait*/ QCompleter_NewQCompleter for (&'a  QStringList, &'a mut QObject) {
  fn NewQCompleter(self) -> QCompleter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleterC1ERK11QStringListP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QCompleterC1ERK11QStringListP7QObject(qthis, arg0, arg1)};
    let rsthis = QCompleter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn currentIndex<T: QCompleter_currentIndex>(&mut self, value: T) -> i32 {
    value.currentIndex(self);
    return 1;
  }
}

pub trait QCompleter_currentIndex {
  fn currentIndex(self, this: &mut QCompleter) -> i32;
}

// proto: QModelIndex QCompleter::currentIndex();
impl<'a> /*trait*/ QCompleter_currentIndex for () {
  fn currentIndex(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter12currentIndexEv()};
    unsafe {_ZNK10QCompleter12currentIndexEv()};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn pathFromIndex<T: QCompleter_pathFromIndex>(&mut self, value: T) -> i32 {
    value.pathFromIndex(self);
    return 1;
  }
}

pub trait QCompleter_pathFromIndex {
  fn pathFromIndex(self, this: &mut QCompleter) -> i32;
}

// proto: QString QCompleter::pathFromIndex(const QModelIndex & index);
impl<'a> /*trait*/ QCompleter_pathFromIndex for (&'a  QModelIndex) {
  fn pathFromIndex(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter13pathFromIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QCompleter13pathFromIndexERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn highlighted<T: QCompleter_highlighted>(&mut self, value: T) -> i32 {
    value.highlighted(self);
    return 1;
  }
}

pub trait QCompleter_highlighted {
  fn highlighted(self, this: &mut QCompleter) -> i32;
}

// proto: void QCompleter::highlighted(const QModelIndex & index);
impl<'a> /*trait*/ QCompleter_highlighted for (&'a  QModelIndex) {
  fn highlighted(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter11highlightedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QCompleter11highlightedERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setMaxVisibleItems<T: QCompleter_setMaxVisibleItems>(&mut self, value: T) -> i32 {
    value.setMaxVisibleItems(self);
    return 1;
  }
}

pub trait QCompleter_setMaxVisibleItems {
  fn setMaxVisibleItems(self, this: &mut QCompleter) -> i32;
}

// proto: void QCompleter::setMaxVisibleItems(int maxItems);
impl<'a> /*trait*/ QCompleter_setMaxVisibleItems for (i32) {
  fn setMaxVisibleItems(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter18setMaxVisibleItemsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QCompleter18setMaxVisibleItemsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn completionColumn<T: QCompleter_completionColumn>(&mut self, value: T) -> i32 {
    value.completionColumn(self);
    return 1;
  }
}

pub trait QCompleter_completionColumn {
  fn completionColumn(self, this: &mut QCompleter) -> i32;
}

// proto: int QCompleter::completionColumn();
impl<'a> /*trait*/ QCompleter_completionColumn for () {
  fn completionColumn(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter16completionColumnEv()};
    unsafe {_ZNK10QCompleter16completionColumnEv()};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn maxVisibleItems<T: QCompleter_maxVisibleItems>(&mut self, value: T) -> i32 {
    value.maxVisibleItems(self);
    return 1;
  }
}

pub trait QCompleter_maxVisibleItems {
  fn maxVisibleItems(self, this: &mut QCompleter) -> i32;
}

// proto: int QCompleter::maxVisibleItems();
impl<'a> /*trait*/ QCompleter_maxVisibleItems for () {
  fn maxVisibleItems(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter15maxVisibleItemsEv()};
    unsafe {_ZNK10QCompleter15maxVisibleItemsEv()};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn FreeQCompleter<T: QCompleter_FreeQCompleter>(&mut self, value: T) -> i32 {
    value.FreeQCompleter(self);
    return 1;
  }
}

pub trait QCompleter_FreeQCompleter {
  fn FreeQCompleter(self, this: &mut QCompleter) -> i32;
}

// proto: void QCompleter::FreeQCompleter();
impl<'a> /*trait*/ QCompleter_FreeQCompleter for () {
  fn FreeQCompleter(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleterD0Ev()};
    unsafe {_ZN10QCompleterD0Ev()};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setWrapAround<T: QCompleter_setWrapAround>(&mut self, value: T) -> i32 {
    value.setWrapAround(self);
    return 1;
  }
}

pub trait QCompleter_setWrapAround {
  fn setWrapAround(self, this: &mut QCompleter) -> i32;
}

// proto: void QCompleter::setWrapAround(bool wrap);
impl<'a> /*trait*/ QCompleter_setWrapAround for (i8) {
  fn setWrapAround(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter13setWrapAroundEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QCompleter13setWrapAroundEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn splitPath<T: QCompleter_splitPath>(&mut self, value: T) -> i32 {
    value.splitPath(self);
    return 1;
  }
}

pub trait QCompleter_splitPath {
  fn splitPath(self, this: &mut QCompleter) -> i32;
}

// proto: QStringList QCompleter::splitPath(const QString & path);
impl<'a> /*trait*/ QCompleter_splitPath for (&'a  QString) {
  fn splitPath(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter9splitPathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QCompleter9splitPathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn model<T: QCompleter_model>(&mut self, value: T) -> i32 {
    value.model(self);
    return 1;
  }
}

pub trait QCompleter_model {
  fn model(self, this: &mut QCompleter) -> i32;
}

// proto: QAbstractItemModel * QCompleter::model();
impl<'a> /*trait*/ QCompleter_model for () {
  fn model(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter5modelEv()};
    unsafe {_ZNK10QCompleter5modelEv()};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn currentCompletion<T: QCompleter_currentCompletion>(&mut self, value: T) -> i32 {
    value.currentCompletion(self);
    return 1;
  }
}

pub trait QCompleter_currentCompletion {
  fn currentCompletion(self, this: &mut QCompleter) -> i32;
}

// proto: QString QCompleter::currentCompletion();
impl<'a> /*trait*/ QCompleter_currentCompletion for () {
  fn currentCompletion(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter17currentCompletionEv()};
    unsafe {_ZNK10QCompleter17currentCompletionEv()};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setCompletionColumn<T: QCompleter_setCompletionColumn>(&mut self, value: T) -> i32 {
    value.setCompletionColumn(self);
    return 1;
  }
}

pub trait QCompleter_setCompletionColumn {
  fn setCompletionColumn(self, this: &mut QCompleter) -> i32;
}

// proto: void QCompleter::setCompletionColumn(int column);
impl<'a> /*trait*/ QCompleter_setCompletionColumn for (i32) {
  fn setCompletionColumn(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter19setCompletionColumnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QCompleter19setCompletionColumnEi(arg0)};
    return 1;
  }
}

// proto: void QCompleter::highlighted(const QString & text);
impl<'a> /*trait*/ QCompleter_highlighted for (&'a  QString) {
  fn highlighted(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter11highlightedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QCompleter11highlightedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setCompletionPrefix<T: QCompleter_setCompletionPrefix>(&mut self, value: T) -> i32 {
    value.setCompletionPrefix(self);
    return 1;
  }
}

pub trait QCompleter_setCompletionPrefix {
  fn setCompletionPrefix(self, this: &mut QCompleter) -> i32;
}

// proto: void QCompleter::setCompletionPrefix(const QString & prefix);
impl<'a> /*trait*/ QCompleter_setCompletionPrefix for (&'a  QString) {
  fn setCompletionPrefix(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter19setCompletionPrefixERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QCompleter19setCompletionPrefixERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn completionModel<T: QCompleter_completionModel>(&mut self, value: T) -> i32 {
    value.completionModel(self);
    return 1;
  }
}

pub trait QCompleter_completionModel {
  fn completionModel(self, this: &mut QCompleter) -> i32;
}

// proto: QAbstractItemModel * QCompleter::completionModel();
impl<'a> /*trait*/ QCompleter_completionModel for () {
  fn completionModel(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter15completionModelEv()};
    unsafe {_ZNK10QCompleter15completionModelEv()};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setCurrentRow<T: QCompleter_setCurrentRow>(&mut self, value: T) -> i32 {
    value.setCurrentRow(self);
    return 1;
  }
}

pub trait QCompleter_setCurrentRow {
  fn setCurrentRow(self, this: &mut QCompleter) -> i32;
}

// proto: bool QCompleter::setCurrentRow(int row);
impl<'a> /*trait*/ QCompleter_setCurrentRow for (i32) {
  fn setCurrentRow(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter13setCurrentRowEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QCompleter13setCurrentRowEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn currentRow<T: QCompleter_currentRow>(&mut self, value: T) -> i32 {
    value.currentRow(self);
    return 1;
  }
}

pub trait QCompleter_currentRow {
  fn currentRow(self, this: &mut QCompleter) -> i32;
}

// proto: int QCompleter::currentRow();
impl<'a> /*trait*/ QCompleter_currentRow for () {
  fn currentRow(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter10currentRowEv()};
    unsafe {_ZNK10QCompleter10currentRowEv()};
    return 1;
  }
}

// proto: void QCompleter::activated(const QString & text);
impl<'a> /*trait*/ QCompleter_activated for (&'a  QString) {
  fn activated(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter9activatedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QCompleter9activatedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn wrapAround<T: QCompleter_wrapAround>(&mut self, value: T) -> i32 {
    value.wrapAround(self);
    return 1;
  }
}

pub trait QCompleter_wrapAround {
  fn wrapAround(self, this: &mut QCompleter) -> i32;
}

// proto: bool QCompleter::wrapAround();
impl<'a> /*trait*/ QCompleter_wrapAround for () {
  fn wrapAround(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter10wrapAroundEv()};
    unsafe {_ZNK10QCompleter10wrapAroundEv()};
    return 1;
  }
}

// proto: void QCompleter::NewQCompleter(const QCompleter & );
impl<'a> /*trait*/ QCompleter_NewQCompleter for (&'a  QCompleter) {
  fn NewQCompleter(self) -> QCompleter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleterC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QCompleterC1ERKS_(qthis, arg0)};
    let rsthis = QCompleter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn widget<T: QCompleter_widget>(&mut self, value: T) -> i32 {
    value.widget(self);
    return 1;
  }
}

pub trait QCompleter_widget {
  fn widget(self, this: &mut QCompleter) -> i32;
}

// proto: QWidget * QCompleter::widget();
impl<'a> /*trait*/ QCompleter_widget for () {
  fn widget(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter6widgetEv()};
    unsafe {_ZNK10QCompleter6widgetEv()};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn completionRole<T: QCompleter_completionRole>(&mut self, value: T) -> i32 {
    value.completionRole(self);
    return 1;
  }
}

pub trait QCompleter_completionRole {
  fn completionRole(self, this: &mut QCompleter) -> i32;
}

// proto: int QCompleter::completionRole();
impl<'a> /*trait*/ QCompleter_completionRole for () {
  fn completionRole(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter14completionRoleEv()};
    unsafe {_ZNK10QCompleter14completionRoleEv()};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn completionPrefix<T: QCompleter_completionPrefix>(&mut self, value: T) -> i32 {
    value.completionPrefix(self);
    return 1;
  }
}

pub trait QCompleter_completionPrefix {
  fn completionPrefix(self, this: &mut QCompleter) -> i32;
}

// proto: QString QCompleter::completionPrefix();
impl<'a> /*trait*/ QCompleter_completionPrefix for () {
  fn completionPrefix(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter16completionPrefixEv()};
    unsafe {_ZNK10QCompleter16completionPrefixEv()};
    return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setWidget<T: QCompleter_setWidget>(&mut self, value: T) -> i32 {
    value.setWidget(self);
    return 1;
  }
}

pub trait QCompleter_setWidget {
  fn setWidget(self, this: &mut QCompleter) -> i32;
}

// proto: void QCompleter::setWidget(QWidget * widget);
impl<'a> /*trait*/ QCompleter_setWidget for (&'a mut QWidget) {
  fn setWidget(self, this: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QCompleter9setWidgetEP7QWidget(arg0)};
    return 1;
  }
}

