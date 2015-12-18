// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qundocommand::QUndoCommand;
use super::qobject::QObject;
use super::qaction::QAction;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QUndoStack::undoLimit();
  fn _ZNK10QUndoStack9undoLimitEv(qthis: *mut c_void) -> c_int;
  // proto:  void QUndoStack::undoTextChanged(const QString & undoText);
  fn _ZN10QUndoStack15undoTextChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUndoStack::redoTextChanged(const QString & redoText);
  fn _ZN10QUndoStack15redoTextChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QUndoCommand * QUndoStack::command(int index);
  fn _ZNK10QUndoStack7commandEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QUndoStack::canRedo();
  fn _ZNK10QUndoStack7canRedoEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QUndoStack::metaObject();
  fn _ZNK10QUndoStack10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QString QUndoStack::redoText();
  fn _ZNK10QUndoStack8redoTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAction * QUndoStack::createUndoAction(QObject * parent, const QString & prefix);
  fn _ZNK10QUndoStack16createUndoActionEP7QObjectRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  int QUndoStack::count();
  fn _ZNK10QUndoStack5countEv(qthis: *mut c_void) -> c_int;
  // proto:  QAction * QUndoStack::createRedoAction(QObject * parent, const QString & prefix);
  fn _ZNK10QUndoStack16createRedoActionEP7QObjectRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  int QUndoStack::index();
  fn _ZNK10QUndoStack5indexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QUndoStack::clear();
  fn _ZN10QUndoStack5clearEv(qthis: *mut c_void) ;
  // proto:  void QUndoStack::undo();
  fn _ZN10QUndoStack4undoEv(qthis: *mut c_void) ;
  // proto:  void QUndoStack::canRedoChanged(bool canRedo);
  fn _ZN10QUndoStack14canRedoChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QUndoStack::canUndo();
  fn _ZNK10QUndoStack7canUndoEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QUndoStack::isActive();
  fn _ZNK10QUndoStack8isActiveEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QUndoStack::cleanChanged(bool clean);
  fn _ZN10QUndoStack12cleanChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QUndoStack::FreeQUndoStack();
  fn _ZN10QUndoStackD0Ev(qthis: *mut c_void) ;
  // proto:  void QUndoStack::NewQUndoStack(QObject * parent);
  fn _ZN10QUndoStackC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUndoStack::indexChanged(int idx);
  fn _ZN10QUndoStack12indexChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QUndoStack::canUndoChanged(bool canUndo);
  fn _ZN10QUndoStack14canUndoChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QUndoStack::isClean();
  fn _ZNK10QUndoStack7isCleanEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QUndoStack::redo();
  fn _ZN10QUndoStack4redoEv(qthis: *mut c_void) ;
  // proto:  void QUndoStack::beginMacro(const QString & text);
  fn _ZN10QUndoStack10beginMacroERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUndoStack::setActive(bool active);
  fn _ZN10QUndoStack9setActiveEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QUndoStack::undoText();
  fn _ZNK10QUndoStack8undoTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QUndoStack::cleanIndex();
  fn _ZNK10QUndoStack10cleanIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QUndoStack::setIndex(int idx);
  fn _ZN10QUndoStack8setIndexEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QUndoStack::endMacro();
  fn _ZN10QUndoStack8endMacroEv(qthis: *mut c_void) ;
  // proto:  void QUndoStack::setUndoLimit(int limit);
  fn _ZN10QUndoStack12setUndoLimitEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QUndoStack::setClean();
  fn _ZN10QUndoStack8setCleanEv(qthis: *mut c_void) ;
  // proto:  void QUndoStack::NewQUndoStack(const QUndoStack & );
  fn _ZN10QUndoStackC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QUndoStack::text(int idx);
  fn _ZNK10QUndoStack4textEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QUndoStack::push(QUndoCommand * cmd);
  fn _ZN10QUndoStack4pushEP12QUndoCommand(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QUndoStack)=1
pub struct QUndoStack {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QUndoStack {
  pub fn undoLimit<RetType, T: QUndoStack_undoLimit<RetType>>(&mut self, value: T) -> RetType {
    return value.undoLimit(self);
    // return 1;
  }
}

pub trait QUndoStack_undoLimit<RetType> {
  fn undoLimit(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  int QUndoStack::undoLimit();
impl<'a> /*trait*/ QUndoStack_undoLimit<i32> for () {
  fn undoLimit(self, rsthis: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack9undoLimitEv()};
    let mut ret = unsafe {_ZNK10QUndoStack9undoLimitEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn undoTextChanged<RetType, T: QUndoStack_undoTextChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.undoTextChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_undoTextChanged<RetType> {
  fn undoTextChanged(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::undoTextChanged(const QString & undoText);
impl<'a> /*trait*/ QUndoStack_undoTextChanged<()> for (&'a  QString) {
  fn undoTextChanged(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack15undoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoStack15undoTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn redoTextChanged<RetType, T: QUndoStack_redoTextChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.redoTextChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_redoTextChanged<RetType> {
  fn redoTextChanged(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::redoTextChanged(const QString & redoText);
impl<'a> /*trait*/ QUndoStack_redoTextChanged<()> for (&'a  QString) {
  fn redoTextChanged(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack15redoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoStack15redoTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn command<RetType, T: QUndoStack_command<RetType>>(&mut self, value: T) -> RetType {
    return value.command(self);
    // return 1;
  }
}

pub trait QUndoStack_command<RetType> {
  fn command(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  const QUndoCommand * QUndoStack::command(int index);
impl<'a> /*trait*/ QUndoStack_command<QUndoCommand> for (i32) {
  fn command(self, rsthis: &mut QUndoStack) -> QUndoCommand {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7commandEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QUndoStack7commandEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QUndoCommand{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn canRedo<RetType, T: QUndoStack_canRedo<RetType>>(&mut self, value: T) -> RetType {
    return value.canRedo(self);
    // return 1;
  }
}

pub trait QUndoStack_canRedo<RetType> {
  fn canRedo(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  bool QUndoStack::canRedo();
impl<'a> /*trait*/ QUndoStack_canRedo<i8> for () {
  fn canRedo(self, rsthis: &mut QUndoStack) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7canRedoEv()};
    let mut ret = unsafe {_ZNK10QUndoStack7canRedoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn metaObject<RetType, T: QUndoStack_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QUndoStack_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  const QMetaObject * QUndoStack::metaObject();
impl<'a> /*trait*/ QUndoStack_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack10metaObjectEv()};
     unsafe {_ZNK10QUndoStack10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn redoText<RetType, T: QUndoStack_redoText<RetType>>(&mut self, value: T) -> RetType {
    return value.redoText(self);
    // return 1;
  }
}

pub trait QUndoStack_redoText<RetType> {
  fn redoText(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  QString QUndoStack::redoText();
impl<'a> /*trait*/ QUndoStack_redoText<QString> for () {
  fn redoText(self, rsthis: &mut QUndoStack) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack8redoTextEv()};
    let mut ret = unsafe {_ZNK10QUndoStack8redoTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn createUndoAction<RetType, T: QUndoStack_createUndoAction<RetType>>(&mut self, value: T) -> RetType {
    return value.createUndoAction(self);
    // return 1;
  }
}

pub trait QUndoStack_createUndoAction<RetType> {
  fn createUndoAction(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  QAction * QUndoStack::createUndoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoStack_createUndoAction<QAction> for (&'a mut QObject, &'a  QString) {
  fn createUndoAction(self, rsthis: &mut QUndoStack) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack16createUndoActionEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QUndoStack16createUndoActionEP7QObjectRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn count<RetType, T: QUndoStack_count<RetType>>(&mut self, value: T) -> RetType {
    return value.count(self);
    // return 1;
  }
}

pub trait QUndoStack_count<RetType> {
  fn count(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  int QUndoStack::count();
impl<'a> /*trait*/ QUndoStack_count<i32> for () {
  fn count(self, rsthis: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack5countEv()};
    let mut ret = unsafe {_ZNK10QUndoStack5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn createRedoAction<RetType, T: QUndoStack_createRedoAction<RetType>>(&mut self, value: T) -> RetType {
    return value.createRedoAction(self);
    // return 1;
  }
}

pub trait QUndoStack_createRedoAction<RetType> {
  fn createRedoAction(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  QAction * QUndoStack::createRedoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoStack_createRedoAction<QAction> for (&'a mut QObject, &'a  QString) {
  fn createRedoAction(self, rsthis: &mut QUndoStack) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack16createRedoActionEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QUndoStack16createRedoActionEP7QObjectRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn index<RetType, T: QUndoStack_index<RetType>>(&mut self, value: T) -> RetType {
    return value.index(self);
    // return 1;
  }
}

pub trait QUndoStack_index<RetType> {
  fn index(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  int QUndoStack::index();
impl<'a> /*trait*/ QUndoStack_index<i32> for () {
  fn index(self, rsthis: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack5indexEv()};
    let mut ret = unsafe {_ZNK10QUndoStack5indexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn clear<RetType, T: QUndoStack_clear<RetType>>(&mut self, value: T) -> RetType {
    return value.clear(self);
    // return 1;
  }
}

pub trait QUndoStack_clear<RetType> {
  fn clear(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::clear();
impl<'a> /*trait*/ QUndoStack_clear<()> for () {
  fn clear(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack5clearEv()};
     unsafe {_ZN10QUndoStack5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn undo<RetType, T: QUndoStack_undo<RetType>>(&mut self, value: T) -> RetType {
    return value.undo(self);
    // return 1;
  }
}

pub trait QUndoStack_undo<RetType> {
  fn undo(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::undo();
impl<'a> /*trait*/ QUndoStack_undo<()> for () {
  fn undo(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack4undoEv()};
     unsafe {_ZN10QUndoStack4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn canRedoChanged<RetType, T: QUndoStack_canRedoChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.canRedoChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_canRedoChanged<RetType> {
  fn canRedoChanged(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::canRedoChanged(bool canRedo);
impl<'a> /*trait*/ QUndoStack_canRedoChanged<()> for (i8) {
  fn canRedoChanged(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack14canRedoChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoStack14canRedoChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn canUndo<RetType, T: QUndoStack_canUndo<RetType>>(&mut self, value: T) -> RetType {
    return value.canUndo(self);
    // return 1;
  }
}

pub trait QUndoStack_canUndo<RetType> {
  fn canUndo(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  bool QUndoStack::canUndo();
impl<'a> /*trait*/ QUndoStack_canUndo<i8> for () {
  fn canUndo(self, rsthis: &mut QUndoStack) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7canUndoEv()};
    let mut ret = unsafe {_ZNK10QUndoStack7canUndoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn isActive<RetType, T: QUndoStack_isActive<RetType>>(&mut self, value: T) -> RetType {
    return value.isActive(self);
    // return 1;
  }
}

pub trait QUndoStack_isActive<RetType> {
  fn isActive(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  bool QUndoStack::isActive();
impl<'a> /*trait*/ QUndoStack_isActive<i8> for () {
  fn isActive(self, rsthis: &mut QUndoStack) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack8isActiveEv()};
    let mut ret = unsafe {_ZNK10QUndoStack8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn cleanChanged<RetType, T: QUndoStack_cleanChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.cleanChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_cleanChanged<RetType> {
  fn cleanChanged(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::cleanChanged(bool clean);
impl<'a> /*trait*/ QUndoStack_cleanChanged<()> for (i8) {
  fn cleanChanged(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack12cleanChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoStack12cleanChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn FreeQUndoStack<RetType, T: QUndoStack_FreeQUndoStack<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQUndoStack(self);
    // return 1;
  }
}

pub trait QUndoStack_FreeQUndoStack<RetType> {
  fn FreeQUndoStack(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::FreeQUndoStack();
impl<'a> /*trait*/ QUndoStack_FreeQUndoStack<()> for () {
  fn FreeQUndoStack(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStackD0Ev()};
     unsafe {_ZN10QUndoStackD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn NewQUndoStack<T: QUndoStack_NewQUndoStack>(value: T) -> QUndoStack {
    let rsthis = value.NewQUndoStack();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoStack_NewQUndoStack {
  fn NewQUndoStack(self) -> QUndoStack;
}

// proto: void QUndoStack::NewQUndoStack(QObject * parent);
impl<'a> /*trait*/ QUndoStack_NewQUndoStack for (&'a mut QObject) {
  fn NewQUndoStack(self) -> QUndoStack {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStackC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QUndoStackC1EP7QObject(qthis, arg0)};
    let rsthis = QUndoStack{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn indexChanged<RetType, T: QUndoStack_indexChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.indexChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_indexChanged<RetType> {
  fn indexChanged(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::indexChanged(int idx);
impl<'a> /*trait*/ QUndoStack_indexChanged<()> for (i32) {
  fn indexChanged(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack12indexChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QUndoStack12indexChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn canUndoChanged<RetType, T: QUndoStack_canUndoChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.canUndoChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_canUndoChanged<RetType> {
  fn canUndoChanged(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::canUndoChanged(bool canUndo);
impl<'a> /*trait*/ QUndoStack_canUndoChanged<()> for (i8) {
  fn canUndoChanged(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack14canUndoChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoStack14canUndoChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn isClean<RetType, T: QUndoStack_isClean<RetType>>(&mut self, value: T) -> RetType {
    return value.isClean(self);
    // return 1;
  }
}

pub trait QUndoStack_isClean<RetType> {
  fn isClean(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  bool QUndoStack::isClean();
impl<'a> /*trait*/ QUndoStack_isClean<i8> for () {
  fn isClean(self, rsthis: &mut QUndoStack) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7isCleanEv()};
    let mut ret = unsafe {_ZNK10QUndoStack7isCleanEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn redo<RetType, T: QUndoStack_redo<RetType>>(&mut self, value: T) -> RetType {
    return value.redo(self);
    // return 1;
  }
}

pub trait QUndoStack_redo<RetType> {
  fn redo(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::redo();
impl<'a> /*trait*/ QUndoStack_redo<()> for () {
  fn redo(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack4redoEv()};
     unsafe {_ZN10QUndoStack4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn beginMacro<RetType, T: QUndoStack_beginMacro<RetType>>(&mut self, value: T) -> RetType {
    return value.beginMacro(self);
    // return 1;
  }
}

pub trait QUndoStack_beginMacro<RetType> {
  fn beginMacro(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::beginMacro(const QString & text);
impl<'a> /*trait*/ QUndoStack_beginMacro<()> for (&'a  QString) {
  fn beginMacro(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack10beginMacroERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoStack10beginMacroERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn setActive<RetType, T: QUndoStack_setActive<RetType>>(&mut self, value: T) -> RetType {
    return value.setActive(self);
    // return 1;
  }
}

pub trait QUndoStack_setActive<RetType> {
  fn setActive(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::setActive(bool active);
impl<'a> /*trait*/ QUndoStack_setActive<()> for (i8) {
  fn setActive(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack9setActiveEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoStack9setActiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn undoText<RetType, T: QUndoStack_undoText<RetType>>(&mut self, value: T) -> RetType {
    return value.undoText(self);
    // return 1;
  }
}

pub trait QUndoStack_undoText<RetType> {
  fn undoText(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  QString QUndoStack::undoText();
impl<'a> /*trait*/ QUndoStack_undoText<QString> for () {
  fn undoText(self, rsthis: &mut QUndoStack) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack8undoTextEv()};
    let mut ret = unsafe {_ZNK10QUndoStack8undoTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn cleanIndex<RetType, T: QUndoStack_cleanIndex<RetType>>(&mut self, value: T) -> RetType {
    return value.cleanIndex(self);
    // return 1;
  }
}

pub trait QUndoStack_cleanIndex<RetType> {
  fn cleanIndex(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  int QUndoStack::cleanIndex();
impl<'a> /*trait*/ QUndoStack_cleanIndex<i32> for () {
  fn cleanIndex(self, rsthis: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack10cleanIndexEv()};
    let mut ret = unsafe {_ZNK10QUndoStack10cleanIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn setIndex<RetType, T: QUndoStack_setIndex<RetType>>(&mut self, value: T) -> RetType {
    return value.setIndex(self);
    // return 1;
  }
}

pub trait QUndoStack_setIndex<RetType> {
  fn setIndex(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::setIndex(int idx);
impl<'a> /*trait*/ QUndoStack_setIndex<()> for (i32) {
  fn setIndex(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack8setIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QUndoStack8setIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn endMacro<RetType, T: QUndoStack_endMacro<RetType>>(&mut self, value: T) -> RetType {
    return value.endMacro(self);
    // return 1;
  }
}

pub trait QUndoStack_endMacro<RetType> {
  fn endMacro(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::endMacro();
impl<'a> /*trait*/ QUndoStack_endMacro<()> for () {
  fn endMacro(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack8endMacroEv()};
     unsafe {_ZN10QUndoStack8endMacroEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn setUndoLimit<RetType, T: QUndoStack_setUndoLimit<RetType>>(&mut self, value: T) -> RetType {
    return value.setUndoLimit(self);
    // return 1;
  }
}

pub trait QUndoStack_setUndoLimit<RetType> {
  fn setUndoLimit(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::setUndoLimit(int limit);
impl<'a> /*trait*/ QUndoStack_setUndoLimit<()> for (i32) {
  fn setUndoLimit(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack12setUndoLimitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QUndoStack12setUndoLimitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn setClean<RetType, T: QUndoStack_setClean<RetType>>(&mut self, value: T) -> RetType {
    return value.setClean(self);
    // return 1;
  }
}

pub trait QUndoStack_setClean<RetType> {
  fn setClean(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::setClean();
impl<'a> /*trait*/ QUndoStack_setClean<()> for () {
  fn setClean(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack8setCleanEv()};
     unsafe {_ZN10QUndoStack8setCleanEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QUndoStack::NewQUndoStack(const QUndoStack & );
impl<'a> /*trait*/ QUndoStack_NewQUndoStack for (&'a  QUndoStack) {
  fn NewQUndoStack(self) -> QUndoStack {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStackC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QUndoStackC1ERKS_(qthis, arg0)};
    let rsthis = QUndoStack{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn text<RetType, T: QUndoStack_text<RetType>>(&mut self, value: T) -> RetType {
    return value.text(self);
    // return 1;
  }
}

pub trait QUndoStack_text<RetType> {
  fn text(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  QString QUndoStack::text(int idx);
impl<'a> /*trait*/ QUndoStack_text<QString> for (i32) {
  fn text(self, rsthis: &mut QUndoStack) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack4textEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QUndoStack4textEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn push<RetType, T: QUndoStack_push<RetType>>(&mut self, value: T) -> RetType {
    return value.push(self);
    // return 1;
  }
}

pub trait QUndoStack_push<RetType> {
  fn push(self, rsthis: &mut QUndoStack) -> RetType;
}

// proto:  void QUndoStack::push(QUndoCommand * cmd);
impl<'a> /*trait*/ QUndoStack_push<()> for (&'a mut QUndoCommand) {
  fn push(self, rsthis: &mut QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack4pushEP12QUndoCommand()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoStack4pushEP12QUndoCommand(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

