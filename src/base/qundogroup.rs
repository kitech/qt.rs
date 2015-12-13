// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qundostack::QUndoStack;
use super::qobject::QObject;
use super::qstring::QString;
use super::qaction::QAction;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QUndoGroup::addStack(QUndoStack * stack);
  fn _ZN10QUndoGroup8addStackEP10QUndoStack(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUndoGroup::undo();
  fn _ZN10QUndoGroup4undoEv(qthis: *mut c_void) ;
  // proto:  QList<QUndoStack *> QUndoGroup::stacks();
  fn _ZNK10QUndoGroup6stacksEv(qthis: *mut c_void) ;
  // proto:  void QUndoGroup::canRedoChanged(bool canRedo);
  fn _ZN10QUndoGroup14canRedoChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QUndoGroup::cleanChanged(bool clean);
  fn _ZN10QUndoGroup12cleanChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QUndoGroup::canUndoChanged(bool canUndo);
  fn _ZN10QUndoGroup14canUndoChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QUndoGroup::redo();
  fn _ZN10QUndoGroup4redoEv(qthis: *mut c_void) ;
  // proto:  void QUndoGroup::NewQUndoGroup(QObject * parent);
  fn _ZN10QUndoGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUndoGroup::setActiveStack(QUndoStack * stack);
  fn _ZN10QUndoGroup14setActiveStackEP10QUndoStack(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAction * QUndoGroup::createRedoAction(QObject * parent, const QString & prefix);
  fn _ZNK10QUndoGroup16createRedoActionEP7QObjectRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QUndoGroup::NewQUndoGroup(const QUndoGroup & );
  fn _ZN10QUndoGroupC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QUndoGroup::metaObject();
  fn _ZNK10QUndoGroup10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QUndoGroup::canRedo();
  fn _ZNK10QUndoGroup7canRedoEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QUndoGroup::redoText();
  fn _ZNK10QUndoGroup8redoTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QUndoStack * QUndoGroup::activeStack();
  fn _ZNK10QUndoGroup11activeStackEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QUndoGroup::undoText();
  fn _ZNK10QUndoGroup8undoTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QUndoGroup::indexChanged(int idx);
  fn _ZN10QUndoGroup12indexChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QUndoGroup::canUndo();
  fn _ZNK10QUndoGroup7canUndoEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QUndoGroup::FreeQUndoGroup();
  fn _ZN10QUndoGroupD0Ev(qthis: *mut c_void) ;
  // proto:  void QUndoGroup::activeStackChanged(QUndoStack * stack);
  fn _ZN10QUndoGroup18activeStackChangedEP10QUndoStack(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUndoGroup::undoTextChanged(const QString & undoText);
  fn _ZN10QUndoGroup15undoTextChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QUndoGroup::isClean();
  fn _ZNK10QUndoGroup7isCleanEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QUndoGroup::redoTextChanged(const QString & redoText);
  fn _ZN10QUndoGroup15redoTextChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAction * QUndoGroup::createUndoAction(QObject * parent, const QString & prefix);
  fn _ZNK10QUndoGroup16createUndoActionEP7QObjectRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QUndoGroup::removeStack(QUndoStack * stack);
  fn _ZN10QUndoGroup11removeStackEP10QUndoStack(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QUndoGroup)=1
pub struct QUndoGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QUndoGroup {
  pub fn addStack<T: QUndoGroup_addStack>(&mut self, value: T)  {
     value.addStack(self);
    // return 1;
  }
}

pub trait QUndoGroup_addStack {
  fn addStack(self, rsthis: &mut QUndoGroup) ;
}

// proto:  void QUndoGroup::addStack(QUndoStack * stack);
impl<'a> /*trait*/ QUndoGroup_addStack for (&'a mut QUndoStack) {
  fn addStack(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup8addStackEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup8addStackEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn undo<T: QUndoGroup_undo>(&mut self, value: T)  {
     value.undo(self);
    // return 1;
  }
}

pub trait QUndoGroup_undo {
  fn undo(self, rsthis: &mut QUndoGroup) ;
}

// proto:  void QUndoGroup::undo();
impl<'a> /*trait*/ QUndoGroup_undo for () {
  fn undo(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup4undoEv()};
     unsafe {_ZN10QUndoGroup4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn stacks<T: QUndoGroup_stacks>(&mut self, value: T)  {
     value.stacks(self);
    // return 1;
  }
}

pub trait QUndoGroup_stacks {
  fn stacks(self, rsthis: &mut QUndoGroup) ;
}

// proto:  QList<QUndoStack *> QUndoGroup::stacks();
impl<'a> /*trait*/ QUndoGroup_stacks for () {
  fn stacks(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup6stacksEv()};
     unsafe {_ZNK10QUndoGroup6stacksEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn canRedoChanged<T: QUndoGroup_canRedoChanged>(&mut self, value: T)  {
     value.canRedoChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_canRedoChanged {
  fn canRedoChanged(self, rsthis: &mut QUndoGroup) ;
}

// proto:  void QUndoGroup::canRedoChanged(bool canRedo);
impl<'a> /*trait*/ QUndoGroup_canRedoChanged for (i8) {
  fn canRedoChanged(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup14canRedoChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoGroup14canRedoChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn cleanChanged<T: QUndoGroup_cleanChanged>(&mut self, value: T)  {
     value.cleanChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_cleanChanged {
  fn cleanChanged(self, rsthis: &mut QUndoGroup) ;
}

// proto:  void QUndoGroup::cleanChanged(bool clean);
impl<'a> /*trait*/ QUndoGroup_cleanChanged for (i8) {
  fn cleanChanged(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup12cleanChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoGroup12cleanChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn canUndoChanged<T: QUndoGroup_canUndoChanged>(&mut self, value: T)  {
     value.canUndoChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_canUndoChanged {
  fn canUndoChanged(self, rsthis: &mut QUndoGroup) ;
}

// proto:  void QUndoGroup::canUndoChanged(bool canUndo);
impl<'a> /*trait*/ QUndoGroup_canUndoChanged for (i8) {
  fn canUndoChanged(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup14canUndoChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoGroup14canUndoChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn redo<T: QUndoGroup_redo>(&mut self, value: T)  {
     value.redo(self);
    // return 1;
  }
}

pub trait QUndoGroup_redo {
  fn redo(self, rsthis: &mut QUndoGroup) ;
}

// proto:  void QUndoGroup::redo();
impl<'a> /*trait*/ QUndoGroup_redo for () {
  fn redo(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup4redoEv()};
     unsafe {_ZN10QUndoGroup4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn NewQUndoGroup<T: QUndoGroup_NewQUndoGroup>(value: T) -> QUndoGroup {
    let rsthis = value.NewQUndoGroup();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoGroup_NewQUndoGroup {
  fn NewQUndoGroup(self) -> QUndoGroup;
}

// proto: void QUndoGroup::NewQUndoGroup(QObject * parent);
impl<'a> /*trait*/ QUndoGroup_NewQUndoGroup for (&'a mut QObject) {
  fn NewQUndoGroup(self) -> QUndoGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroupC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QUndoGroupC1EP7QObject(qthis, arg0)};
    let rsthis = QUndoGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn setActiveStack<T: QUndoGroup_setActiveStack>(&mut self, value: T)  {
     value.setActiveStack(self);
    // return 1;
  }
}

pub trait QUndoGroup_setActiveStack {
  fn setActiveStack(self, rsthis: &mut QUndoGroup) ;
}

// proto:  void QUndoGroup::setActiveStack(QUndoStack * stack);
impl<'a> /*trait*/ QUndoGroup_setActiveStack for (&'a mut QUndoStack) {
  fn setActiveStack(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup14setActiveStackEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup14setActiveStackEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn createRedoAction<T: QUndoGroup_createRedoAction>(&mut self, value: T) -> QAction {
    return value.createRedoAction(self);
    // return 1;
  }
}

pub trait QUndoGroup_createRedoAction {
  fn createRedoAction(self, rsthis: &mut QUndoGroup) -> QAction;
}

// proto:  QAction * QUndoGroup::createRedoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoGroup_createRedoAction for (&'a mut QObject, &'a  QString) {
  fn createRedoAction(self, rsthis: &mut QUndoGroup) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup16createRedoActionEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QUndoGroup16createRedoActionEP7QObjectRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QUndoGroup::NewQUndoGroup(const QUndoGroup & );
impl<'a> /*trait*/ QUndoGroup_NewQUndoGroup for (&'a  QUndoGroup) {
  fn NewQUndoGroup(self) -> QUndoGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QUndoGroupC1ERKS_(qthis, arg0)};
    let rsthis = QUndoGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn metaObject<T: QUndoGroup_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QUndoGroup_metaObject {
  fn metaObject(self, rsthis: &mut QUndoGroup) ;
}

// proto:  const QMetaObject * QUndoGroup::metaObject();
impl<'a> /*trait*/ QUndoGroup_metaObject for () {
  fn metaObject(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup10metaObjectEv()};
     unsafe {_ZNK10QUndoGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn canRedo<T: QUndoGroup_canRedo>(&mut self, value: T) -> i8 {
    return value.canRedo(self);
    // return 1;
  }
}

pub trait QUndoGroup_canRedo {
  fn canRedo(self, rsthis: &mut QUndoGroup) -> i8;
}

// proto:  bool QUndoGroup::canRedo();
impl<'a> /*trait*/ QUndoGroup_canRedo for () {
  fn canRedo(self, rsthis: &mut QUndoGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup7canRedoEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup7canRedoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn redoText<T: QUndoGroup_redoText>(&mut self, value: T) -> QString {
    return value.redoText(self);
    // return 1;
  }
}

pub trait QUndoGroup_redoText {
  fn redoText(self, rsthis: &mut QUndoGroup) -> QString;
}

// proto:  QString QUndoGroup::redoText();
impl<'a> /*trait*/ QUndoGroup_redoText for () {
  fn redoText(self, rsthis: &mut QUndoGroup) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup8redoTextEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup8redoTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn activeStack<T: QUndoGroup_activeStack>(&mut self, value: T) -> QUndoStack {
    return value.activeStack(self);
    // return 1;
  }
}

pub trait QUndoGroup_activeStack {
  fn activeStack(self, rsthis: &mut QUndoGroup) -> QUndoStack;
}

// proto:  QUndoStack * QUndoGroup::activeStack();
impl<'a> /*trait*/ QUndoGroup_activeStack for () {
  fn activeStack(self, rsthis: &mut QUndoGroup) -> QUndoStack {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup11activeStackEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup11activeStackEv(rsthis.qclsinst)};
    let mut ret1 = QUndoStack{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn undoText<T: QUndoGroup_undoText>(&mut self, value: T) -> QString {
    return value.undoText(self);
    // return 1;
  }
}

pub trait QUndoGroup_undoText {
  fn undoText(self, rsthis: &mut QUndoGroup) -> QString;
}

// proto:  QString QUndoGroup::undoText();
impl<'a> /*trait*/ QUndoGroup_undoText for () {
  fn undoText(self, rsthis: &mut QUndoGroup) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup8undoTextEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup8undoTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn indexChanged<T: QUndoGroup_indexChanged>(&mut self, value: T)  {
     value.indexChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_indexChanged {
  fn indexChanged(self, rsthis: &mut QUndoGroup) ;
}

// proto:  void QUndoGroup::indexChanged(int idx);
impl<'a> /*trait*/ QUndoGroup_indexChanged for (i32) {
  fn indexChanged(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup12indexChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QUndoGroup12indexChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn canUndo<T: QUndoGroup_canUndo>(&mut self, value: T) -> i8 {
    return value.canUndo(self);
    // return 1;
  }
}

pub trait QUndoGroup_canUndo {
  fn canUndo(self, rsthis: &mut QUndoGroup) -> i8;
}

// proto:  bool QUndoGroup::canUndo();
impl<'a> /*trait*/ QUndoGroup_canUndo for () {
  fn canUndo(self, rsthis: &mut QUndoGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup7canUndoEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup7canUndoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn FreeQUndoGroup<T: QUndoGroup_FreeQUndoGroup>(&mut self, value: T)  {
     value.FreeQUndoGroup(self);
    // return 1;
  }
}

pub trait QUndoGroup_FreeQUndoGroup {
  fn FreeQUndoGroup(self, rsthis: &mut QUndoGroup) ;
}

// proto:  void QUndoGroup::FreeQUndoGroup();
impl<'a> /*trait*/ QUndoGroup_FreeQUndoGroup for () {
  fn FreeQUndoGroup(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroupD0Ev()};
     unsafe {_ZN10QUndoGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn activeStackChanged<T: QUndoGroup_activeStackChanged>(&mut self, value: T)  {
     value.activeStackChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_activeStackChanged {
  fn activeStackChanged(self, rsthis: &mut QUndoGroup) ;
}

// proto:  void QUndoGroup::activeStackChanged(QUndoStack * stack);
impl<'a> /*trait*/ QUndoGroup_activeStackChanged for (&'a mut QUndoStack) {
  fn activeStackChanged(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup18activeStackChangedEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup18activeStackChangedEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn undoTextChanged<T: QUndoGroup_undoTextChanged>(&mut self, value: T)  {
     value.undoTextChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_undoTextChanged {
  fn undoTextChanged(self, rsthis: &mut QUndoGroup) ;
}

// proto:  void QUndoGroup::undoTextChanged(const QString & undoText);
impl<'a> /*trait*/ QUndoGroup_undoTextChanged for (&'a  QString) {
  fn undoTextChanged(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup15undoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup15undoTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn isClean<T: QUndoGroup_isClean>(&mut self, value: T) -> i8 {
    return value.isClean(self);
    // return 1;
  }
}

pub trait QUndoGroup_isClean {
  fn isClean(self, rsthis: &mut QUndoGroup) -> i8;
}

// proto:  bool QUndoGroup::isClean();
impl<'a> /*trait*/ QUndoGroup_isClean for () {
  fn isClean(self, rsthis: &mut QUndoGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup7isCleanEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup7isCleanEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn redoTextChanged<T: QUndoGroup_redoTextChanged>(&mut self, value: T)  {
     value.redoTextChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_redoTextChanged {
  fn redoTextChanged(self, rsthis: &mut QUndoGroup) ;
}

// proto:  void QUndoGroup::redoTextChanged(const QString & redoText);
impl<'a> /*trait*/ QUndoGroup_redoTextChanged for (&'a  QString) {
  fn redoTextChanged(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup15redoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup15redoTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn createUndoAction<T: QUndoGroup_createUndoAction>(&mut self, value: T) -> QAction {
    return value.createUndoAction(self);
    // return 1;
  }
}

pub trait QUndoGroup_createUndoAction {
  fn createUndoAction(self, rsthis: &mut QUndoGroup) -> QAction;
}

// proto:  QAction * QUndoGroup::createUndoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoGroup_createUndoAction for (&'a mut QObject, &'a  QString) {
  fn createUndoAction(self, rsthis: &mut QUndoGroup) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup16createUndoActionEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QUndoGroup16createUndoActionEP7QObjectRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn removeStack<T: QUndoGroup_removeStack>(&mut self, value: T)  {
     value.removeStack(self);
    // return 1;
  }
}

pub trait QUndoGroup_removeStack {
  fn removeStack(self, rsthis: &mut QUndoGroup) ;
}

// proto:  void QUndoGroup::removeStack(QUndoStack * stack);
impl<'a> /*trait*/ QUndoGroup_removeStack for (&'a mut QUndoStack) {
  fn removeStack(self, rsthis: &mut QUndoGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup11removeStackEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup11removeStackEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

