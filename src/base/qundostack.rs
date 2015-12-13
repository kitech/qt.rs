// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qobject::QObject;
use super::qundocommand::QUndoCommand;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QUndoStack::undoLimit();
  fn _ZNK10QUndoStack9undoLimitEv() -> i32;
  // proto: void QUndoStack::undoTextChanged(const QString & undoText);
  fn _ZN10QUndoStack15undoTextChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: void QUndoStack::redoTextChanged(const QString & redoText);
  fn _ZN10QUndoStack15redoTextChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: const QUndoCommand * QUndoStack::command(int index);
  fn _ZNK10QUndoStack7commandEi(arg0: c_int) -> i32;
  // proto: bool QUndoStack::canRedo();
  fn _ZNK10QUndoStack7canRedoEv() -> i32;
  // proto: const QMetaObject * QUndoStack::metaObject();
  fn _ZNK10QUndoStack10metaObjectEv() -> i32;
  // proto: QString QUndoStack::redoText();
  fn _ZNK10QUndoStack8redoTextEv() -> i32;
  // proto: QAction * QUndoStack::createUndoAction(QObject * parent, const QString & prefix);
  fn _ZNK10QUndoStack16createUndoActionEP7QObjectRK7QString(arg0: *mut c_void, arg1: *const c_void) -> i32;
  // proto: int QUndoStack::count();
  fn _ZNK10QUndoStack5countEv() -> i32;
  // proto: QAction * QUndoStack::createRedoAction(QObject * parent, const QString & prefix);
  fn _ZNK10QUndoStack16createRedoActionEP7QObjectRK7QString(arg0: *mut c_void, arg1: *const c_void) -> i32;
  // proto: int QUndoStack::index();
  fn _ZNK10QUndoStack5indexEv() -> i32;
  // proto: void QUndoStack::clear();
  fn _ZN10QUndoStack5clearEv() -> i32;
  // proto: void QUndoStack::undo();
  fn _ZN10QUndoStack4undoEv() -> i32;
  // proto: void QUndoStack::canRedoChanged(bool canRedo);
  fn _ZN10QUndoStack14canRedoChangedEb(arg0: int8_t) -> i32;
  // proto: bool QUndoStack::canUndo();
  fn _ZNK10QUndoStack7canUndoEv() -> i32;
  // proto: bool QUndoStack::isActive();
  fn _ZNK10QUndoStack8isActiveEv() -> i32;
  // proto: void QUndoStack::cleanChanged(bool clean);
  fn _ZN10QUndoStack12cleanChangedEb(arg0: int8_t) -> i32;
  // proto: void QUndoStack::FreeQUndoStack();
  fn _ZN10QUndoStackD0Ev() -> i32;
  // proto: void QUndoStack::NewQUndoStack(QObject * parent);
  fn _ZN10QUndoStackC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QUndoStack::indexChanged(int idx);
  fn _ZN10QUndoStack12indexChangedEi(arg0: c_int) -> i32;
  // proto: void QUndoStack::canUndoChanged(bool canUndo);
  fn _ZN10QUndoStack14canUndoChangedEb(arg0: int8_t) -> i32;
  // proto: bool QUndoStack::isClean();
  fn _ZNK10QUndoStack7isCleanEv() -> i32;
  // proto: void QUndoStack::redo();
  fn _ZN10QUndoStack4redoEv() -> i32;
  // proto: void QUndoStack::beginMacro(const QString & text);
  fn _ZN10QUndoStack10beginMacroERK7QString(arg0: *const c_void) -> i32;
  // proto: void QUndoStack::setActive(bool active);
  fn _ZN10QUndoStack9setActiveEb(arg0: int8_t) -> i32;
  // proto: QString QUndoStack::undoText();
  fn _ZNK10QUndoStack8undoTextEv() -> i32;
  // proto: int QUndoStack::cleanIndex();
  fn _ZNK10QUndoStack10cleanIndexEv() -> i32;
  // proto: void QUndoStack::setIndex(int idx);
  fn _ZN10QUndoStack8setIndexEi(arg0: c_int) -> i32;
  // proto: void QUndoStack::endMacro();
  fn _ZN10QUndoStack8endMacroEv() -> i32;
  // proto: void QUndoStack::setUndoLimit(int limit);
  fn _ZN10QUndoStack12setUndoLimitEi(arg0: c_int) -> i32;
  // proto: void QUndoStack::setClean();
  fn _ZN10QUndoStack8setCleanEv() -> i32;
  // proto: void QUndoStack::NewQUndoStack(const QUndoStack & );
  fn _ZN10QUndoStackC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QString QUndoStack::text(int idx);
  fn _ZNK10QUndoStack4textEi(arg0: c_int) -> i32;
  // proto: void QUndoStack::push(QUndoCommand * cmd);
  fn _ZN10QUndoStack4pushEP12QUndoCommand(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QUndoStack)=1
pub struct QUndoStack {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QUndoStack {
  pub fn undoLimit<T: QUndoStack_undoLimit>(&mut self, value: T) -> i32 {
    value.undoLimit(self);
    return 1;
  }
}

pub trait QUndoStack_undoLimit {
  fn undoLimit(self, this: &mut QUndoStack) -> i32;
}

// proto: int QUndoStack::undoLimit();
impl<'a> /*trait*/ QUndoStack_undoLimit for () {
  fn undoLimit(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack9undoLimitEv()};
    unsafe {_ZNK10QUndoStack9undoLimitEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn undoTextChanged<T: QUndoStack_undoTextChanged>(&mut self, value: T) -> i32 {
    value.undoTextChanged(self);
    return 1;
  }
}

pub trait QUndoStack_undoTextChanged {
  fn undoTextChanged(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::undoTextChanged(const QString & undoText);
impl<'a> /*trait*/ QUndoStack_undoTextChanged for (&'a  QString) {
  fn undoTextChanged(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack15undoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QUndoStack15undoTextChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn redoTextChanged<T: QUndoStack_redoTextChanged>(&mut self, value: T) -> i32 {
    value.redoTextChanged(self);
    return 1;
  }
}

pub trait QUndoStack_redoTextChanged {
  fn redoTextChanged(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::redoTextChanged(const QString & redoText);
impl<'a> /*trait*/ QUndoStack_redoTextChanged for (&'a  QString) {
  fn redoTextChanged(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack15redoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QUndoStack15redoTextChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn command<T: QUndoStack_command>(&mut self, value: T) -> i32 {
    value.command(self);
    return 1;
  }
}

pub trait QUndoStack_command {
  fn command(self, this: &mut QUndoStack) -> i32;
}

// proto: const QUndoCommand * QUndoStack::command(int index);
impl<'a> /*trait*/ QUndoStack_command for (i32) {
  fn command(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7commandEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QUndoStack7commandEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn canRedo<T: QUndoStack_canRedo>(&mut self, value: T) -> i32 {
    value.canRedo(self);
    return 1;
  }
}

pub trait QUndoStack_canRedo {
  fn canRedo(self, this: &mut QUndoStack) -> i32;
}

// proto: bool QUndoStack::canRedo();
impl<'a> /*trait*/ QUndoStack_canRedo for () {
  fn canRedo(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7canRedoEv()};
    unsafe {_ZNK10QUndoStack7canRedoEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn metaObject<T: QUndoStack_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QUndoStack_metaObject {
  fn metaObject(self, this: &mut QUndoStack) -> i32;
}

// proto: const QMetaObject * QUndoStack::metaObject();
impl<'a> /*trait*/ QUndoStack_metaObject for () {
  fn metaObject(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack10metaObjectEv()};
    unsafe {_ZNK10QUndoStack10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn redoText<T: QUndoStack_redoText>(&mut self, value: T) -> i32 {
    value.redoText(self);
    return 1;
  }
}

pub trait QUndoStack_redoText {
  fn redoText(self, this: &mut QUndoStack) -> i32;
}

// proto: QString QUndoStack::redoText();
impl<'a> /*trait*/ QUndoStack_redoText for () {
  fn redoText(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack8redoTextEv()};
    unsafe {_ZNK10QUndoStack8redoTextEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn createUndoAction<T: QUndoStack_createUndoAction>(&mut self, value: T) -> i32 {
    value.createUndoAction(self);
    return 1;
  }
}

pub trait QUndoStack_createUndoAction {
  fn createUndoAction(self, this: &mut QUndoStack) -> i32;
}

// proto: QAction * QUndoStack::createUndoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoStack_createUndoAction for (&'a mut QObject, &'a  QString) {
  fn createUndoAction(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack16createUndoActionEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK10QUndoStack16createUndoActionEP7QObjectRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn count<T: QUndoStack_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QUndoStack_count {
  fn count(self, this: &mut QUndoStack) -> i32;
}

// proto: int QUndoStack::count();
impl<'a> /*trait*/ QUndoStack_count for () {
  fn count(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack5countEv()};
    unsafe {_ZNK10QUndoStack5countEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn createRedoAction<T: QUndoStack_createRedoAction>(&mut self, value: T) -> i32 {
    value.createRedoAction(self);
    return 1;
  }
}

pub trait QUndoStack_createRedoAction {
  fn createRedoAction(self, this: &mut QUndoStack) -> i32;
}

// proto: QAction * QUndoStack::createRedoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoStack_createRedoAction for (&'a mut QObject, &'a  QString) {
  fn createRedoAction(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack16createRedoActionEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK10QUndoStack16createRedoActionEP7QObjectRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn index<T: QUndoStack_index>(&mut self, value: T) -> i32 {
    value.index(self);
    return 1;
  }
}

pub trait QUndoStack_index {
  fn index(self, this: &mut QUndoStack) -> i32;
}

// proto: int QUndoStack::index();
impl<'a> /*trait*/ QUndoStack_index for () {
  fn index(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack5indexEv()};
    unsafe {_ZNK10QUndoStack5indexEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn clear<T: QUndoStack_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QUndoStack_clear {
  fn clear(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::clear();
impl<'a> /*trait*/ QUndoStack_clear for () {
  fn clear(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack5clearEv()};
    unsafe {_ZN10QUndoStack5clearEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn undo<T: QUndoStack_undo>(&mut self, value: T) -> i32 {
    value.undo(self);
    return 1;
  }
}

pub trait QUndoStack_undo {
  fn undo(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::undo();
impl<'a> /*trait*/ QUndoStack_undo for () {
  fn undo(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack4undoEv()};
    unsafe {_ZN10QUndoStack4undoEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn canRedoChanged<T: QUndoStack_canRedoChanged>(&mut self, value: T) -> i32 {
    value.canRedoChanged(self);
    return 1;
  }
}

pub trait QUndoStack_canRedoChanged {
  fn canRedoChanged(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::canRedoChanged(bool canRedo);
impl<'a> /*trait*/ QUndoStack_canRedoChanged for (i8) {
  fn canRedoChanged(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack14canRedoChangedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QUndoStack14canRedoChangedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn canUndo<T: QUndoStack_canUndo>(&mut self, value: T) -> i32 {
    value.canUndo(self);
    return 1;
  }
}

pub trait QUndoStack_canUndo {
  fn canUndo(self, this: &mut QUndoStack) -> i32;
}

// proto: bool QUndoStack::canUndo();
impl<'a> /*trait*/ QUndoStack_canUndo for () {
  fn canUndo(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7canUndoEv()};
    unsafe {_ZNK10QUndoStack7canUndoEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn isActive<T: QUndoStack_isActive>(&mut self, value: T) -> i32 {
    value.isActive(self);
    return 1;
  }
}

pub trait QUndoStack_isActive {
  fn isActive(self, this: &mut QUndoStack) -> i32;
}

// proto: bool QUndoStack::isActive();
impl<'a> /*trait*/ QUndoStack_isActive for () {
  fn isActive(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack8isActiveEv()};
    unsafe {_ZNK10QUndoStack8isActiveEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn cleanChanged<T: QUndoStack_cleanChanged>(&mut self, value: T) -> i32 {
    value.cleanChanged(self);
    return 1;
  }
}

pub trait QUndoStack_cleanChanged {
  fn cleanChanged(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::cleanChanged(bool clean);
impl<'a> /*trait*/ QUndoStack_cleanChanged for (i8) {
  fn cleanChanged(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack12cleanChangedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QUndoStack12cleanChangedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn FreeQUndoStack<T: QUndoStack_FreeQUndoStack>(&mut self, value: T) -> i32 {
    value.FreeQUndoStack(self);
    return 1;
  }
}

pub trait QUndoStack_FreeQUndoStack {
  fn FreeQUndoStack(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::FreeQUndoStack();
impl<'a> /*trait*/ QUndoStack_FreeQUndoStack for () {
  fn FreeQUndoStack(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStackD0Ev()};
    unsafe {_ZN10QUndoStackD0Ev()};
    return 1;
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
  pub fn indexChanged<T: QUndoStack_indexChanged>(&mut self, value: T) -> i32 {
    value.indexChanged(self);
    return 1;
  }
}

pub trait QUndoStack_indexChanged {
  fn indexChanged(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::indexChanged(int idx);
impl<'a> /*trait*/ QUndoStack_indexChanged for (i32) {
  fn indexChanged(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack12indexChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QUndoStack12indexChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn canUndoChanged<T: QUndoStack_canUndoChanged>(&mut self, value: T) -> i32 {
    value.canUndoChanged(self);
    return 1;
  }
}

pub trait QUndoStack_canUndoChanged {
  fn canUndoChanged(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::canUndoChanged(bool canUndo);
impl<'a> /*trait*/ QUndoStack_canUndoChanged for (i8) {
  fn canUndoChanged(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack14canUndoChangedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QUndoStack14canUndoChangedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn isClean<T: QUndoStack_isClean>(&mut self, value: T) -> i32 {
    value.isClean(self);
    return 1;
  }
}

pub trait QUndoStack_isClean {
  fn isClean(self, this: &mut QUndoStack) -> i32;
}

// proto: bool QUndoStack::isClean();
impl<'a> /*trait*/ QUndoStack_isClean for () {
  fn isClean(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7isCleanEv()};
    unsafe {_ZNK10QUndoStack7isCleanEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn redo<T: QUndoStack_redo>(&mut self, value: T) -> i32 {
    value.redo(self);
    return 1;
  }
}

pub trait QUndoStack_redo {
  fn redo(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::redo();
impl<'a> /*trait*/ QUndoStack_redo for () {
  fn redo(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack4redoEv()};
    unsafe {_ZN10QUndoStack4redoEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn beginMacro<T: QUndoStack_beginMacro>(&mut self, value: T) -> i32 {
    value.beginMacro(self);
    return 1;
  }
}

pub trait QUndoStack_beginMacro {
  fn beginMacro(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::beginMacro(const QString & text);
impl<'a> /*trait*/ QUndoStack_beginMacro for (&'a  QString) {
  fn beginMacro(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack10beginMacroERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QUndoStack10beginMacroERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn setActive<T: QUndoStack_setActive>(&mut self, value: T) -> i32 {
    value.setActive(self);
    return 1;
  }
}

pub trait QUndoStack_setActive {
  fn setActive(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::setActive(bool active);
impl<'a> /*trait*/ QUndoStack_setActive for (i8) {
  fn setActive(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack9setActiveEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QUndoStack9setActiveEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn undoText<T: QUndoStack_undoText>(&mut self, value: T) -> i32 {
    value.undoText(self);
    return 1;
  }
}

pub trait QUndoStack_undoText {
  fn undoText(self, this: &mut QUndoStack) -> i32;
}

// proto: QString QUndoStack::undoText();
impl<'a> /*trait*/ QUndoStack_undoText for () {
  fn undoText(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack8undoTextEv()};
    unsafe {_ZNK10QUndoStack8undoTextEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn cleanIndex<T: QUndoStack_cleanIndex>(&mut self, value: T) -> i32 {
    value.cleanIndex(self);
    return 1;
  }
}

pub trait QUndoStack_cleanIndex {
  fn cleanIndex(self, this: &mut QUndoStack) -> i32;
}

// proto: int QUndoStack::cleanIndex();
impl<'a> /*trait*/ QUndoStack_cleanIndex for () {
  fn cleanIndex(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack10cleanIndexEv()};
    unsafe {_ZNK10QUndoStack10cleanIndexEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn setIndex<T: QUndoStack_setIndex>(&mut self, value: T) -> i32 {
    value.setIndex(self);
    return 1;
  }
}

pub trait QUndoStack_setIndex {
  fn setIndex(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::setIndex(int idx);
impl<'a> /*trait*/ QUndoStack_setIndex for (i32) {
  fn setIndex(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack8setIndexEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QUndoStack8setIndexEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn endMacro<T: QUndoStack_endMacro>(&mut self, value: T) -> i32 {
    value.endMacro(self);
    return 1;
  }
}

pub trait QUndoStack_endMacro {
  fn endMacro(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::endMacro();
impl<'a> /*trait*/ QUndoStack_endMacro for () {
  fn endMacro(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack8endMacroEv()};
    unsafe {_ZN10QUndoStack8endMacroEv()};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn setUndoLimit<T: QUndoStack_setUndoLimit>(&mut self, value: T) -> i32 {
    value.setUndoLimit(self);
    return 1;
  }
}

pub trait QUndoStack_setUndoLimit {
  fn setUndoLimit(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::setUndoLimit(int limit);
impl<'a> /*trait*/ QUndoStack_setUndoLimit for (i32) {
  fn setUndoLimit(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack12setUndoLimitEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QUndoStack12setUndoLimitEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn setClean<T: QUndoStack_setClean>(&mut self, value: T) -> i32 {
    value.setClean(self);
    return 1;
  }
}

pub trait QUndoStack_setClean {
  fn setClean(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::setClean();
impl<'a> /*trait*/ QUndoStack_setClean for () {
  fn setClean(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack8setCleanEv()};
    unsafe {_ZN10QUndoStack8setCleanEv()};
    return 1;
  }
}

// proto: void QUndoStack::NewQUndoStack(const QUndoStack & );
impl<'a> /*trait*/ QUndoStack_NewQUndoStack for (&'a  QUndoStack) {
  fn NewQUndoStack(self) -> QUndoStack {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStackC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QUndoStackC1ERKS_(qthis, arg0)};
    let rsthis = QUndoStack{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn text<T: QUndoStack_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QUndoStack_text {
  fn text(self, this: &mut QUndoStack) -> i32;
}

// proto: QString QUndoStack::text(int idx);
impl<'a> /*trait*/ QUndoStack_text for (i32) {
  fn text(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack4textEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QUndoStack4textEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoStack {
  pub fn push<T: QUndoStack_push>(&mut self, value: T) -> i32 {
    value.push(self);
    return 1;
  }
}

pub trait QUndoStack_push {
  fn push(self, this: &mut QUndoStack) -> i32;
}

// proto: void QUndoStack::push(QUndoCommand * cmd);
impl<'a> /*trait*/ QUndoStack_push for (&'a mut QUndoCommand) {
  fn push(self, this: &mut QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack4pushEP12QUndoCommand()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QUndoStack4pushEP12QUndoCommand(arg0)};
    return 1;
  }
}

