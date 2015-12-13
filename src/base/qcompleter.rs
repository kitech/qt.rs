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
  // proto:  void QCompleter::NewQCompleter(QObject * parent);
  fn _ZN10QCompleterC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QCompleter::metaObject();
  fn _ZNK10QCompleter10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QAbstractItemView * QCompleter::popup();
  fn _ZNK10QCompleter5popupEv(qthis: *mut c_void) ;
  // proto:  void QCompleter::complete(const QRect & rect);
  fn _ZN10QCompleter8completeERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCompleter::activated(const QModelIndex & index);
  fn _ZN10QCompleter9activatedERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCompleter::setCompletionRole(int role);
  fn _ZN10QCompleter17setCompletionRoleEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QCompleter::completionCount();
  fn _ZNK10QCompleter15completionCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QCompleter::NewQCompleter(const QStringList & completions, QObject * parent);
  fn _ZN10QCompleterC1ERK11QStringListP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QModelIndex QCompleter::currentIndex();
  fn _ZNK10QCompleter12currentIndexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QCompleter::pathFromIndex(const QModelIndex & index);
  fn _ZNK10QCompleter13pathFromIndexERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QCompleter::highlighted(const QModelIndex & index);
  fn _ZN10QCompleter11highlightedERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCompleter::setMaxVisibleItems(int maxItems);
  fn _ZN10QCompleter18setMaxVisibleItemsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QCompleter::completionColumn();
  fn _ZNK10QCompleter16completionColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  int QCompleter::maxVisibleItems();
  fn _ZNK10QCompleter15maxVisibleItemsEv(qthis: *mut c_void) -> c_int;
  // proto:  void QCompleter::FreeQCompleter();
  fn _ZN10QCompleterD0Ev(qthis: *mut c_void) ;
  // proto:  void QCompleter::setWrapAround(bool wrap);
  fn _ZN10QCompleter13setWrapAroundEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QStringList QCompleter::splitPath(const QString & path);
  fn _ZNK10QCompleter9splitPathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QAbstractItemModel * QCompleter::model();
  fn _ZNK10QCompleter5modelEv(qthis: *mut c_void) ;
  // proto:  QString QCompleter::currentCompletion();
  fn _ZNK10QCompleter17currentCompletionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCompleter::setCompletionColumn(int column);
  fn _ZN10QCompleter19setCompletionColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QCompleter::highlighted(const QString & text);
  fn _ZN10QCompleter11highlightedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCompleter::setCompletionPrefix(const QString & prefix);
  fn _ZN10QCompleter19setCompletionPrefixERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAbstractItemModel * QCompleter::completionModel();
  fn _ZNK10QCompleter15completionModelEv(qthis: *mut c_void) ;
  // proto:  bool QCompleter::setCurrentRow(int row);
  fn _ZN10QCompleter13setCurrentRowEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  int QCompleter::currentRow();
  fn _ZNK10QCompleter10currentRowEv(qthis: *mut c_void) -> c_int;
  // proto:  void QCompleter::activated(const QString & text);
  fn _ZN10QCompleter9activatedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QCompleter::wrapAround();
  fn _ZNK10QCompleter10wrapAroundEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QCompleter::NewQCompleter(const QCompleter & );
  fn _ZN10QCompleterC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QCompleter::widget();
  fn _ZNK10QCompleter6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QCompleter::completionRole();
  fn _ZNK10QCompleter14completionRoleEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QCompleter::completionPrefix();
  fn _ZNK10QCompleter16completionPrefixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCompleter::setWidget(QWidget * widget);
  fn _ZN10QCompleter9setWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
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
  pub fn metaObject<T: QCompleter_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QCompleter_metaObject {
  fn metaObject(self, rsthis: &mut QCompleter) ;
}

// proto:  const QMetaObject * QCompleter::metaObject();
impl<'a> /*trait*/ QCompleter_metaObject for () {
  fn metaObject(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter10metaObjectEv()};
     unsafe {_ZNK10QCompleter10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn popup<T: QCompleter_popup>(&mut self, value: T)  {
     value.popup(self);
    // return 1;
  }
}

pub trait QCompleter_popup {
  fn popup(self, rsthis: &mut QCompleter) ;
}

// proto:  QAbstractItemView * QCompleter::popup();
impl<'a> /*trait*/ QCompleter_popup for () {
  fn popup(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter5popupEv()};
     unsafe {_ZNK10QCompleter5popupEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn complete<T: QCompleter_complete>(&mut self, value: T)  {
     value.complete(self);
    // return 1;
  }
}

pub trait QCompleter_complete {
  fn complete(self, rsthis: &mut QCompleter) ;
}

// proto:  void QCompleter::complete(const QRect & rect);
impl<'a> /*trait*/ QCompleter_complete for (&'a  QRect) {
  fn complete(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter8completeERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter8completeERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn activated<T: QCompleter_activated>(&mut self, value: T)  {
     value.activated(self);
    // return 1;
  }
}

pub trait QCompleter_activated {
  fn activated(self, rsthis: &mut QCompleter) ;
}

// proto:  void QCompleter::activated(const QModelIndex & index);
impl<'a> /*trait*/ QCompleter_activated for (&'a  QModelIndex) {
  fn activated(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter9activatedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter9activatedERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setCompletionRole<T: QCompleter_setCompletionRole>(&mut self, value: T)  {
     value.setCompletionRole(self);
    // return 1;
  }
}

pub trait QCompleter_setCompletionRole {
  fn setCompletionRole(self, rsthis: &mut QCompleter) ;
}

// proto:  void QCompleter::setCompletionRole(int role);
impl<'a> /*trait*/ QCompleter_setCompletionRole for (i32) {
  fn setCompletionRole(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter17setCompletionRoleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QCompleter17setCompletionRoleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn completionCount<T: QCompleter_completionCount>(&mut self, value: T) -> i32 {
    return value.completionCount(self);
    // return 1;
  }
}

pub trait QCompleter_completionCount {
  fn completionCount(self, rsthis: &mut QCompleter) -> i32;
}

// proto:  int QCompleter::completionCount();
impl<'a> /*trait*/ QCompleter_completionCount for () {
  fn completionCount(self, rsthis: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter15completionCountEv()};
    let mut ret = unsafe {_ZNK10QCompleter15completionCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QCompleter::NewQCompleter(const QStringList & completions, QObject * parent);
impl<'a> /*trait*/ QCompleter_NewQCompleter for (&'a  QStringList, &'a mut QObject) {
  fn NewQCompleter(self) -> QCompleter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleterC1ERK11QStringListP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QCompleterC1ERK11QStringListP7QObject(qthis, arg0, arg1)};
    let rsthis = QCompleter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn currentIndex<T: QCompleter_currentIndex>(&mut self, value: T) -> QModelIndex {
    return value.currentIndex(self);
    // return 1;
  }
}

pub trait QCompleter_currentIndex {
  fn currentIndex(self, rsthis: &mut QCompleter) -> QModelIndex;
}

// proto:  QModelIndex QCompleter::currentIndex();
impl<'a> /*trait*/ QCompleter_currentIndex for () {
  fn currentIndex(self, rsthis: &mut QCompleter) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter12currentIndexEv()};
    let mut ret = unsafe {_ZNK10QCompleter12currentIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn pathFromIndex<T: QCompleter_pathFromIndex>(&mut self, value: T) -> QString {
    return value.pathFromIndex(self);
    // return 1;
  }
}

pub trait QCompleter_pathFromIndex {
  fn pathFromIndex(self, rsthis: &mut QCompleter) -> QString;
}

// proto:  QString QCompleter::pathFromIndex(const QModelIndex & index);
impl<'a> /*trait*/ QCompleter_pathFromIndex for (&'a  QModelIndex) {
  fn pathFromIndex(self, rsthis: &mut QCompleter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter13pathFromIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QCompleter13pathFromIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn highlighted<T: QCompleter_highlighted>(&mut self, value: T)  {
     value.highlighted(self);
    // return 1;
  }
}

pub trait QCompleter_highlighted {
  fn highlighted(self, rsthis: &mut QCompleter) ;
}

// proto:  void QCompleter::highlighted(const QModelIndex & index);
impl<'a> /*trait*/ QCompleter_highlighted for (&'a  QModelIndex) {
  fn highlighted(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter11highlightedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter11highlightedERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setMaxVisibleItems<T: QCompleter_setMaxVisibleItems>(&mut self, value: T)  {
     value.setMaxVisibleItems(self);
    // return 1;
  }
}

pub trait QCompleter_setMaxVisibleItems {
  fn setMaxVisibleItems(self, rsthis: &mut QCompleter) ;
}

// proto:  void QCompleter::setMaxVisibleItems(int maxItems);
impl<'a> /*trait*/ QCompleter_setMaxVisibleItems for (i32) {
  fn setMaxVisibleItems(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter18setMaxVisibleItemsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QCompleter18setMaxVisibleItemsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn completionColumn<T: QCompleter_completionColumn>(&mut self, value: T) -> i32 {
    return value.completionColumn(self);
    // return 1;
  }
}

pub trait QCompleter_completionColumn {
  fn completionColumn(self, rsthis: &mut QCompleter) -> i32;
}

// proto:  int QCompleter::completionColumn();
impl<'a> /*trait*/ QCompleter_completionColumn for () {
  fn completionColumn(self, rsthis: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter16completionColumnEv()};
    let mut ret = unsafe {_ZNK10QCompleter16completionColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn maxVisibleItems<T: QCompleter_maxVisibleItems>(&mut self, value: T) -> i32 {
    return value.maxVisibleItems(self);
    // return 1;
  }
}

pub trait QCompleter_maxVisibleItems {
  fn maxVisibleItems(self, rsthis: &mut QCompleter) -> i32;
}

// proto:  int QCompleter::maxVisibleItems();
impl<'a> /*trait*/ QCompleter_maxVisibleItems for () {
  fn maxVisibleItems(self, rsthis: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter15maxVisibleItemsEv()};
    let mut ret = unsafe {_ZNK10QCompleter15maxVisibleItemsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn FreeQCompleter<T: QCompleter_FreeQCompleter>(&mut self, value: T)  {
     value.FreeQCompleter(self);
    // return 1;
  }
}

pub trait QCompleter_FreeQCompleter {
  fn FreeQCompleter(self, rsthis: &mut QCompleter) ;
}

// proto:  void QCompleter::FreeQCompleter();
impl<'a> /*trait*/ QCompleter_FreeQCompleter for () {
  fn FreeQCompleter(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleterD0Ev()};
     unsafe {_ZN10QCompleterD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setWrapAround<T: QCompleter_setWrapAround>(&mut self, value: T)  {
     value.setWrapAround(self);
    // return 1;
  }
}

pub trait QCompleter_setWrapAround {
  fn setWrapAround(self, rsthis: &mut QCompleter) ;
}

// proto:  void QCompleter::setWrapAround(bool wrap);
impl<'a> /*trait*/ QCompleter_setWrapAround for (i8) {
  fn setWrapAround(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter13setWrapAroundEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QCompleter13setWrapAroundEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn splitPath<T: QCompleter_splitPath>(&mut self, value: T) -> QStringList {
    return value.splitPath(self);
    // return 1;
  }
}

pub trait QCompleter_splitPath {
  fn splitPath(self, rsthis: &mut QCompleter) -> QStringList;
}

// proto:  QStringList QCompleter::splitPath(const QString & path);
impl<'a> /*trait*/ QCompleter_splitPath for (&'a  QString) {
  fn splitPath(self, rsthis: &mut QCompleter) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter9splitPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QCompleter9splitPathERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn model<T: QCompleter_model>(&mut self, value: T)  {
     value.model(self);
    // return 1;
  }
}

pub trait QCompleter_model {
  fn model(self, rsthis: &mut QCompleter) ;
}

// proto:  QAbstractItemModel * QCompleter::model();
impl<'a> /*trait*/ QCompleter_model for () {
  fn model(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter5modelEv()};
     unsafe {_ZNK10QCompleter5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn currentCompletion<T: QCompleter_currentCompletion>(&mut self, value: T) -> QString {
    return value.currentCompletion(self);
    // return 1;
  }
}

pub trait QCompleter_currentCompletion {
  fn currentCompletion(self, rsthis: &mut QCompleter) -> QString;
}

// proto:  QString QCompleter::currentCompletion();
impl<'a> /*trait*/ QCompleter_currentCompletion for () {
  fn currentCompletion(self, rsthis: &mut QCompleter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter17currentCompletionEv()};
    let mut ret = unsafe {_ZNK10QCompleter17currentCompletionEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setCompletionColumn<T: QCompleter_setCompletionColumn>(&mut self, value: T)  {
     value.setCompletionColumn(self);
    // return 1;
  }
}

pub trait QCompleter_setCompletionColumn {
  fn setCompletionColumn(self, rsthis: &mut QCompleter) ;
}

// proto:  void QCompleter::setCompletionColumn(int column);
impl<'a> /*trait*/ QCompleter_setCompletionColumn for (i32) {
  fn setCompletionColumn(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter19setCompletionColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QCompleter19setCompletionColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QCompleter::highlighted(const QString & text);
impl<'a> /*trait*/ QCompleter_highlighted for (&'a  QString) {
  fn highlighted(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter11highlightedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter11highlightedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setCompletionPrefix<T: QCompleter_setCompletionPrefix>(&mut self, value: T)  {
     value.setCompletionPrefix(self);
    // return 1;
  }
}

pub trait QCompleter_setCompletionPrefix {
  fn setCompletionPrefix(self, rsthis: &mut QCompleter) ;
}

// proto:  void QCompleter::setCompletionPrefix(const QString & prefix);
impl<'a> /*trait*/ QCompleter_setCompletionPrefix for (&'a  QString) {
  fn setCompletionPrefix(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter19setCompletionPrefixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter19setCompletionPrefixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn completionModel<T: QCompleter_completionModel>(&mut self, value: T)  {
     value.completionModel(self);
    // return 1;
  }
}

pub trait QCompleter_completionModel {
  fn completionModel(self, rsthis: &mut QCompleter) ;
}

// proto:  QAbstractItemModel * QCompleter::completionModel();
impl<'a> /*trait*/ QCompleter_completionModel for () {
  fn completionModel(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter15completionModelEv()};
     unsafe {_ZNK10QCompleter15completionModelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setCurrentRow<T: QCompleter_setCurrentRow>(&mut self, value: T) -> i8 {
    return value.setCurrentRow(self);
    // return 1;
  }
}

pub trait QCompleter_setCurrentRow {
  fn setCurrentRow(self, rsthis: &mut QCompleter) -> i8;
}

// proto:  bool QCompleter::setCurrentRow(int row);
impl<'a> /*trait*/ QCompleter_setCurrentRow for (i32) {
  fn setCurrentRow(self, rsthis: &mut QCompleter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter13setCurrentRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN10QCompleter13setCurrentRowEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn currentRow<T: QCompleter_currentRow>(&mut self, value: T) -> i32 {
    return value.currentRow(self);
    // return 1;
  }
}

pub trait QCompleter_currentRow {
  fn currentRow(self, rsthis: &mut QCompleter) -> i32;
}

// proto:  int QCompleter::currentRow();
impl<'a> /*trait*/ QCompleter_currentRow for () {
  fn currentRow(self, rsthis: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter10currentRowEv()};
    let mut ret = unsafe {_ZNK10QCompleter10currentRowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QCompleter::activated(const QString & text);
impl<'a> /*trait*/ QCompleter_activated for (&'a  QString) {
  fn activated(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter9activatedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter9activatedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn wrapAround<T: QCompleter_wrapAround>(&mut self, value: T) -> i8 {
    return value.wrapAround(self);
    // return 1;
  }
}

pub trait QCompleter_wrapAround {
  fn wrapAround(self, rsthis: &mut QCompleter) -> i8;
}

// proto:  bool QCompleter::wrapAround();
impl<'a> /*trait*/ QCompleter_wrapAround for () {
  fn wrapAround(self, rsthis: &mut QCompleter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter10wrapAroundEv()};
    let mut ret = unsafe {_ZNK10QCompleter10wrapAroundEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QCompleter::NewQCompleter(const QCompleter & );
impl<'a> /*trait*/ QCompleter_NewQCompleter for (&'a  QCompleter) {
  fn NewQCompleter(self) -> QCompleter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleterC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QCompleterC1ERKS_(qthis, arg0)};
    let rsthis = QCompleter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn widget<T: QCompleter_widget>(&mut self, value: T) -> QWidget {
    return value.widget(self);
    // return 1;
  }
}

pub trait QCompleter_widget {
  fn widget(self, rsthis: &mut QCompleter) -> QWidget;
}

// proto:  QWidget * QCompleter::widget();
impl<'a> /*trait*/ QCompleter_widget for () {
  fn widget(self, rsthis: &mut QCompleter) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter6widgetEv()};
    let mut ret = unsafe {_ZNK10QCompleter6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn completionRole<T: QCompleter_completionRole>(&mut self, value: T) -> i32 {
    return value.completionRole(self);
    // return 1;
  }
}

pub trait QCompleter_completionRole {
  fn completionRole(self, rsthis: &mut QCompleter) -> i32;
}

// proto:  int QCompleter::completionRole();
impl<'a> /*trait*/ QCompleter_completionRole for () {
  fn completionRole(self, rsthis: &mut QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter14completionRoleEv()};
    let mut ret = unsafe {_ZNK10QCompleter14completionRoleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn completionPrefix<T: QCompleter_completionPrefix>(&mut self, value: T) -> QString {
    return value.completionPrefix(self);
    // return 1;
  }
}

pub trait QCompleter_completionPrefix {
  fn completionPrefix(self, rsthis: &mut QCompleter) -> QString;
}

// proto:  QString QCompleter::completionPrefix();
impl<'a> /*trait*/ QCompleter_completionPrefix for () {
  fn completionPrefix(self, rsthis: &mut QCompleter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter16completionPrefixEv()};
    let mut ret = unsafe {_ZNK10QCompleter16completionPrefixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCompleter {
  pub fn setWidget<T: QCompleter_setWidget>(&mut self, value: T)  {
     value.setWidget(self);
    // return 1;
  }
}

pub trait QCompleter_setWidget {
  fn setWidget(self, rsthis: &mut QCompleter) ;
}

// proto:  void QCompleter::setWidget(QWidget * widget);
impl<'a> /*trait*/ QCompleter_setWidget for (&'a mut QWidget) {
  fn setWidget(self, rsthis: &mut QCompleter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

