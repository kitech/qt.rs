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
  pub fn undoLimit<T: QUndoStack_undoLimit>(&mut self, value: T) -> i32 {
    return value.undoLimit(self);
    // return 1;
  }
}

pub trait QUndoStack_undoLimit {
  fn undoLimit(self, rsthis: &mut QUndoStack) -> i32;
}

// proto:  int QUndoStack::undoLimit();
impl<'a> /*trait*/ QUndoStack_undoLimit for () {
  fn undoLimit(self, rsthis: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack9undoLimitEv()};
    let mut ret = unsafe {_ZNK10QUndoStack9undoLimitEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn undoTextChanged<T: QUndoStack_undoTextChanged>(&mut self, value: T)  {
     value.undoTextChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_undoTextChanged {
  fn undoTextChanged(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::undoTextChanged(const QString & undoText);
impl<'a> /*trait*/ QUndoStack_undoTextChanged for (&'a  QString) {
  fn undoTextChanged(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack15undoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoStack15undoTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn redoTextChanged<T: QUndoStack_redoTextChanged>(&mut self, value: T)  {
     value.redoTextChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_redoTextChanged {
  fn redoTextChanged(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::redoTextChanged(const QString & redoText);
impl<'a> /*trait*/ QUndoStack_redoTextChanged for (&'a  QString) {
  fn redoTextChanged(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack15redoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoStack15redoTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn command<T: QUndoStack_command>(&mut self, value: T) -> QUndoCommand {
    return value.command(self);
    // return 1;
  }
}

pub trait QUndoStack_command {
  fn command(self, rsthis: &mut QUndoStack) -> QUndoCommand;
}

// proto:  const QUndoCommand * QUndoStack::command(int index);
impl<'a> /*trait*/ QUndoStack_command for (i32) {
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
  pub fn canRedo<T: QUndoStack_canRedo>(&mut self, value: T) -> i8 {
    return value.canRedo(self);
    // return 1;
  }
}

pub trait QUndoStack_canRedo {
  fn canRedo(self, rsthis: &mut QUndoStack) -> i8;
}

// proto:  bool QUndoStack::canRedo();
impl<'a> /*trait*/ QUndoStack_canRedo for () {
  fn canRedo(self, rsthis: &mut QUndoStack) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7canRedoEv()};
    let mut ret = unsafe {_ZNK10QUndoStack7canRedoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn metaObject<T: QUndoStack_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QUndoStack_metaObject {
  fn metaObject(self, rsthis: &mut QUndoStack) ;
}

// proto:  const QMetaObject * QUndoStack::metaObject();
impl<'a> /*trait*/ QUndoStack_metaObject for () {
  fn metaObject(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack10metaObjectEv()};
     unsafe {_ZNK10QUndoStack10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn redoText<T: QUndoStack_redoText>(&mut self, value: T) -> QString {
    return value.redoText(self);
    // return 1;
  }
}

pub trait QUndoStack_redoText {
  fn redoText(self, rsthis: &mut QUndoStack) -> QString;
}

// proto:  QString QUndoStack::redoText();
impl<'a> /*trait*/ QUndoStack_redoText for () {
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
  pub fn createUndoAction<T: QUndoStack_createUndoAction>(&mut self, value: T) -> QAction {
    return value.createUndoAction(self);
    // return 1;
  }
}

pub trait QUndoStack_createUndoAction {
  fn createUndoAction(self, rsthis: &mut QUndoStack) -> QAction;
}

// proto:  QAction * QUndoStack::createUndoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoStack_createUndoAction for (&'a mut QObject, &'a  QString) {
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
  pub fn count<T: QUndoStack_count>(&mut self, value: T) -> i32 {
    return value.count(self);
    // return 1;
  }
}

pub trait QUndoStack_count {
  fn count(self, rsthis: &mut QUndoStack) -> i32;
}

// proto:  int QUndoStack::count();
impl<'a> /*trait*/ QUndoStack_count for () {
  fn count(self, rsthis: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack5countEv()};
    let mut ret = unsafe {_ZNK10QUndoStack5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn createRedoAction<T: QUndoStack_createRedoAction>(&mut self, value: T) -> QAction {
    return value.createRedoAction(self);
    // return 1;
  }
}

pub trait QUndoStack_createRedoAction {
  fn createRedoAction(self, rsthis: &mut QUndoStack) -> QAction;
}

// proto:  QAction * QUndoStack::createRedoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoStack_createRedoAction for (&'a mut QObject, &'a  QString) {
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
  pub fn index<T: QUndoStack_index>(&mut self, value: T) -> i32 {
    return value.index(self);
    // return 1;
  }
}

pub trait QUndoStack_index {
  fn index(self, rsthis: &mut QUndoStack) -> i32;
}

// proto:  int QUndoStack::index();
impl<'a> /*trait*/ QUndoStack_index for () {
  fn index(self, rsthis: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack5indexEv()};
    let mut ret = unsafe {_ZNK10QUndoStack5indexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn clear<T: QUndoStack_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QUndoStack_clear {
  fn clear(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::clear();
impl<'a> /*trait*/ QUndoStack_clear for () {
  fn clear(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack5clearEv()};
     unsafe {_ZN10QUndoStack5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn undo<T: QUndoStack_undo>(&mut self, value: T)  {
     value.undo(self);
    // return 1;
  }
}

pub trait QUndoStack_undo {
  fn undo(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::undo();
impl<'a> /*trait*/ QUndoStack_undo for () {
  fn undo(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack4undoEv()};
     unsafe {_ZN10QUndoStack4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn canRedoChanged<T: QUndoStack_canRedoChanged>(&mut self, value: T)  {
     value.canRedoChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_canRedoChanged {
  fn canRedoChanged(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::canRedoChanged(bool canRedo);
impl<'a> /*trait*/ QUndoStack_canRedoChanged for (i8) {
  fn canRedoChanged(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack14canRedoChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoStack14canRedoChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn canUndo<T: QUndoStack_canUndo>(&mut self, value: T) -> i8 {
    return value.canUndo(self);
    // return 1;
  }
}

pub trait QUndoStack_canUndo {
  fn canUndo(self, rsthis: &mut QUndoStack) -> i8;
}

// proto:  bool QUndoStack::canUndo();
impl<'a> /*trait*/ QUndoStack_canUndo for () {
  fn canUndo(self, rsthis: &mut QUndoStack) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7canUndoEv()};
    let mut ret = unsafe {_ZNK10QUndoStack7canUndoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn isActive<T: QUndoStack_isActive>(&mut self, value: T) -> i8 {
    return value.isActive(self);
    // return 1;
  }
}

pub trait QUndoStack_isActive {
  fn isActive(self, rsthis: &mut QUndoStack) -> i8;
}

// proto:  bool QUndoStack::isActive();
impl<'a> /*trait*/ QUndoStack_isActive for () {
  fn isActive(self, rsthis: &mut QUndoStack) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack8isActiveEv()};
    let mut ret = unsafe {_ZNK10QUndoStack8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn cleanChanged<T: QUndoStack_cleanChanged>(&mut self, value: T)  {
     value.cleanChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_cleanChanged {
  fn cleanChanged(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::cleanChanged(bool clean);
impl<'a> /*trait*/ QUndoStack_cleanChanged for (i8) {
  fn cleanChanged(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack12cleanChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoStack12cleanChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn FreeQUndoStack<T: QUndoStack_FreeQUndoStack>(&mut self, value: T)  {
     value.FreeQUndoStack(self);
    // return 1;
  }
}

pub trait QUndoStack_FreeQUndoStack {
  fn FreeQUndoStack(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::FreeQUndoStack();
impl<'a> /*trait*/ QUndoStack_FreeQUndoStack for () {
  fn FreeQUndoStack(self, rsthis: &mut QUndoStack)  {
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
  pub fn indexChanged<T: QUndoStack_indexChanged>(&mut self, value: T)  {
     value.indexChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_indexChanged {
  fn indexChanged(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::indexChanged(int idx);
impl<'a> /*trait*/ QUndoStack_indexChanged for (i32) {
  fn indexChanged(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack12indexChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QUndoStack12indexChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn canUndoChanged<T: QUndoStack_canUndoChanged>(&mut self, value: T)  {
     value.canUndoChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_canUndoChanged {
  fn canUndoChanged(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::canUndoChanged(bool canUndo);
impl<'a> /*trait*/ QUndoStack_canUndoChanged for (i8) {
  fn canUndoChanged(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack14canUndoChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoStack14canUndoChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn isClean<T: QUndoStack_isClean>(&mut self, value: T) -> i8 {
    return value.isClean(self);
    // return 1;
  }
}

pub trait QUndoStack_isClean {
  fn isClean(self, rsthis: &mut QUndoStack) -> i8;
}

// proto:  bool QUndoStack::isClean();
impl<'a> /*trait*/ QUndoStack_isClean for () {
  fn isClean(self, rsthis: &mut QUndoStack) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7isCleanEv()};
    let mut ret = unsafe {_ZNK10QUndoStack7isCleanEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn redo<T: QUndoStack_redo>(&mut self, value: T)  {
     value.redo(self);
    // return 1;
  }
}

pub trait QUndoStack_redo {
  fn redo(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::redo();
impl<'a> /*trait*/ QUndoStack_redo for () {
  fn redo(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack4redoEv()};
     unsafe {_ZN10QUndoStack4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn beginMacro<T: QUndoStack_beginMacro>(&mut self, value: T)  {
     value.beginMacro(self);
    // return 1;
  }
}

pub trait QUndoStack_beginMacro {
  fn beginMacro(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::beginMacro(const QString & text);
impl<'a> /*trait*/ QUndoStack_beginMacro for (&'a  QString) {
  fn beginMacro(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack10beginMacroERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoStack10beginMacroERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn setActive<T: QUndoStack_setActive>(&mut self, value: T)  {
     value.setActive(self);
    // return 1;
  }
}

pub trait QUndoStack_setActive {
  fn setActive(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::setActive(bool active);
impl<'a> /*trait*/ QUndoStack_setActive for (i8) {
  fn setActive(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack9setActiveEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoStack9setActiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn undoText<T: QUndoStack_undoText>(&mut self, value: T) -> QString {
    return value.undoText(self);
    // return 1;
  }
}

pub trait QUndoStack_undoText {
  fn undoText(self, rsthis: &mut QUndoStack) -> QString;
}

// proto:  QString QUndoStack::undoText();
impl<'a> /*trait*/ QUndoStack_undoText for () {
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
  pub fn cleanIndex<T: QUndoStack_cleanIndex>(&mut self, value: T) -> i32 {
    return value.cleanIndex(self);
    // return 1;
  }
}

pub trait QUndoStack_cleanIndex {
  fn cleanIndex(self, rsthis: &mut QUndoStack) -> i32;
}

// proto:  int QUndoStack::cleanIndex();
impl<'a> /*trait*/ QUndoStack_cleanIndex for () {
  fn cleanIndex(self, rsthis: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack10cleanIndexEv()};
    let mut ret = unsafe {_ZNK10QUndoStack10cleanIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn setIndex<T: QUndoStack_setIndex>(&mut self, value: T)  {
     value.setIndex(self);
    // return 1;
  }
}

pub trait QUndoStack_setIndex {
  fn setIndex(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::setIndex(int idx);
impl<'a> /*trait*/ QUndoStack_setIndex for (i32) {
  fn setIndex(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack8setIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QUndoStack8setIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn endMacro<T: QUndoStack_endMacro>(&mut self, value: T)  {
     value.endMacro(self);
    // return 1;
  }
}

pub trait QUndoStack_endMacro {
  fn endMacro(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::endMacro();
impl<'a> /*trait*/ QUndoStack_endMacro for () {
  fn endMacro(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack8endMacroEv()};
     unsafe {_ZN10QUndoStack8endMacroEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn setUndoLimit<T: QUndoStack_setUndoLimit>(&mut self, value: T)  {
     value.setUndoLimit(self);
    // return 1;
  }
}

pub trait QUndoStack_setUndoLimit {
  fn setUndoLimit(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::setUndoLimit(int limit);
impl<'a> /*trait*/ QUndoStack_setUndoLimit for (i32) {
  fn setUndoLimit(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack12setUndoLimitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QUndoStack12setUndoLimitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn setClean<T: QUndoStack_setClean>(&mut self, value: T)  {
     value.setClean(self);
    // return 1;
  }
}

pub trait QUndoStack_setClean {
  fn setClean(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::setClean();
impl<'a> /*trait*/ QUndoStack_setClean for () {
  fn setClean(self, rsthis: &mut QUndoStack)  {
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
  pub fn text<T: QUndoStack_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QUndoStack_text {
  fn text(self, rsthis: &mut QUndoStack) -> QString;
}

// proto:  QString QUndoStack::text(int idx);
impl<'a> /*trait*/ QUndoStack_text for (i32) {
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
  pub fn push<T: QUndoStack_push>(&mut self, value: T)  {
     value.push(self);
    // return 1;
  }
}

pub trait QUndoStack_push {
  fn push(self, rsthis: &mut QUndoStack) ;
}

// proto:  void QUndoStack::push(QUndoCommand * cmd);
impl<'a> /*trait*/ QUndoStack_push for (&'a mut QUndoCommand) {
  fn push(self, rsthis: &mut QUndoStack)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack4pushEP12QUndoCommand()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoStack4pushEP12QUndoCommand(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

