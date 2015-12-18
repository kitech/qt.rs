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
  pub fn addStack<RetType, T: QUndoGroup_addStack<RetType>>(&mut self, value: T) -> RetType {
    return value.addStack(self);
    // return 1;
  }
}

pub trait QUndoGroup_addStack<RetType> {
  fn addStack(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  void QUndoGroup::addStack(QUndoStack * stack);
impl<'a> /*trait*/ QUndoGroup_addStack<()> for (&'a mut QUndoStack) {
  fn addStack(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup8addStackEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup8addStackEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn undo<RetType, T: QUndoGroup_undo<RetType>>(&mut self, value: T) -> RetType {
    return value.undo(self);
    // return 1;
  }
}

pub trait QUndoGroup_undo<RetType> {
  fn undo(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  void QUndoGroup::undo();
impl<'a> /*trait*/ QUndoGroup_undo<()> for () {
  fn undo(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup4undoEv()};
     unsafe {_ZN10QUndoGroup4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn stacks<RetType, T: QUndoGroup_stacks<RetType>>(&mut self, value: T) -> RetType {
    return value.stacks(self);
    // return 1;
  }
}

pub trait QUndoGroup_stacks<RetType> {
  fn stacks(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  QList<QUndoStack *> QUndoGroup::stacks();
impl<'a> /*trait*/ QUndoGroup_stacks<()> for () {
  fn stacks(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup6stacksEv()};
     unsafe {_ZNK10QUndoGroup6stacksEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn canRedoChanged<RetType, T: QUndoGroup_canRedoChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.canRedoChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_canRedoChanged<RetType> {
  fn canRedoChanged(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  void QUndoGroup::canRedoChanged(bool canRedo);
impl<'a> /*trait*/ QUndoGroup_canRedoChanged<()> for (i8) {
  fn canRedoChanged(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup14canRedoChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoGroup14canRedoChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn cleanChanged<RetType, T: QUndoGroup_cleanChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.cleanChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_cleanChanged<RetType> {
  fn cleanChanged(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  void QUndoGroup::cleanChanged(bool clean);
impl<'a> /*trait*/ QUndoGroup_cleanChanged<()> for (i8) {
  fn cleanChanged(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup12cleanChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoGroup12cleanChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn canUndoChanged<RetType, T: QUndoGroup_canUndoChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.canUndoChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_canUndoChanged<RetType> {
  fn canUndoChanged(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  void QUndoGroup::canUndoChanged(bool canUndo);
impl<'a> /*trait*/ QUndoGroup_canUndoChanged<()> for (i8) {
  fn canUndoChanged(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup14canUndoChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QUndoGroup14canUndoChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn redo<RetType, T: QUndoGroup_redo<RetType>>(&mut self, value: T) -> RetType {
    return value.redo(self);
    // return 1;
  }
}

pub trait QUndoGroup_redo<RetType> {
  fn redo(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  void QUndoGroup::redo();
impl<'a> /*trait*/ QUndoGroup_redo<()> for () {
  fn redo(self, rsthis: &mut QUndoGroup) -> () {
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
  pub fn setActiveStack<RetType, T: QUndoGroup_setActiveStack<RetType>>(&mut self, value: T) -> RetType {
    return value.setActiveStack(self);
    // return 1;
  }
}

pub trait QUndoGroup_setActiveStack<RetType> {
  fn setActiveStack(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  void QUndoGroup::setActiveStack(QUndoStack * stack);
impl<'a> /*trait*/ QUndoGroup_setActiveStack<()> for (&'a mut QUndoStack) {
  fn setActiveStack(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup14setActiveStackEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup14setActiveStackEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn createRedoAction<RetType, T: QUndoGroup_createRedoAction<RetType>>(&mut self, value: T) -> RetType {
    return value.createRedoAction(self);
    // return 1;
  }
}

pub trait QUndoGroup_createRedoAction<RetType> {
  fn createRedoAction(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  QAction * QUndoGroup::createRedoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoGroup_createRedoAction<QAction> for (&'a mut QObject, &'a  QString) {
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
  pub fn metaObject<RetType, T: QUndoGroup_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QUndoGroup_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  const QMetaObject * QUndoGroup::metaObject();
impl<'a> /*trait*/ QUndoGroup_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup10metaObjectEv()};
     unsafe {_ZNK10QUndoGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn canRedo<RetType, T: QUndoGroup_canRedo<RetType>>(&mut self, value: T) -> RetType {
    return value.canRedo(self);
    // return 1;
  }
}

pub trait QUndoGroup_canRedo<RetType> {
  fn canRedo(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  bool QUndoGroup::canRedo();
impl<'a> /*trait*/ QUndoGroup_canRedo<i8> for () {
  fn canRedo(self, rsthis: &mut QUndoGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup7canRedoEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup7canRedoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn redoText<RetType, T: QUndoGroup_redoText<RetType>>(&mut self, value: T) -> RetType {
    return value.redoText(self);
    // return 1;
  }
}

pub trait QUndoGroup_redoText<RetType> {
  fn redoText(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  QString QUndoGroup::redoText();
impl<'a> /*trait*/ QUndoGroup_redoText<QString> for () {
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
  pub fn activeStack<RetType, T: QUndoGroup_activeStack<RetType>>(&mut self, value: T) -> RetType {
    return value.activeStack(self);
    // return 1;
  }
}

pub trait QUndoGroup_activeStack<RetType> {
  fn activeStack(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  QUndoStack * QUndoGroup::activeStack();
impl<'a> /*trait*/ QUndoGroup_activeStack<QUndoStack> for () {
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
  pub fn undoText<RetType, T: QUndoGroup_undoText<RetType>>(&mut self, value: T) -> RetType {
    return value.undoText(self);
    // return 1;
  }
}

pub trait QUndoGroup_undoText<RetType> {
  fn undoText(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  QString QUndoGroup::undoText();
impl<'a> /*trait*/ QUndoGroup_undoText<QString> for () {
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
  pub fn indexChanged<RetType, T: QUndoGroup_indexChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.indexChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_indexChanged<RetType> {
  fn indexChanged(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  void QUndoGroup::indexChanged(int idx);
impl<'a> /*trait*/ QUndoGroup_indexChanged<()> for (i32) {
  fn indexChanged(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup12indexChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QUndoGroup12indexChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn canUndo<RetType, T: QUndoGroup_canUndo<RetType>>(&mut self, value: T) -> RetType {
    return value.canUndo(self);
    // return 1;
  }
}

pub trait QUndoGroup_canUndo<RetType> {
  fn canUndo(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  bool QUndoGroup::canUndo();
impl<'a> /*trait*/ QUndoGroup_canUndo<i8> for () {
  fn canUndo(self, rsthis: &mut QUndoGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup7canUndoEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup7canUndoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn FreeQUndoGroup<RetType, T: QUndoGroup_FreeQUndoGroup<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQUndoGroup(self);
    // return 1;
  }
}

pub trait QUndoGroup_FreeQUndoGroup<RetType> {
  fn FreeQUndoGroup(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  void QUndoGroup::FreeQUndoGroup();
impl<'a> /*trait*/ QUndoGroup_FreeQUndoGroup<()> for () {
  fn FreeQUndoGroup(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroupD0Ev()};
     unsafe {_ZN10QUndoGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn activeStackChanged<RetType, T: QUndoGroup_activeStackChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.activeStackChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_activeStackChanged<RetType> {
  fn activeStackChanged(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  void QUndoGroup::activeStackChanged(QUndoStack * stack);
impl<'a> /*trait*/ QUndoGroup_activeStackChanged<()> for (&'a mut QUndoStack) {
  fn activeStackChanged(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup18activeStackChangedEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup18activeStackChangedEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn undoTextChanged<RetType, T: QUndoGroup_undoTextChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.undoTextChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_undoTextChanged<RetType> {
  fn undoTextChanged(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  void QUndoGroup::undoTextChanged(const QString & undoText);
impl<'a> /*trait*/ QUndoGroup_undoTextChanged<()> for (&'a  QString) {
  fn undoTextChanged(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup15undoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup15undoTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn isClean<RetType, T: QUndoGroup_isClean<RetType>>(&mut self, value: T) -> RetType {
    return value.isClean(self);
    // return 1;
  }
}

pub trait QUndoGroup_isClean<RetType> {
  fn isClean(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  bool QUndoGroup::isClean();
impl<'a> /*trait*/ QUndoGroup_isClean<i8> for () {
  fn isClean(self, rsthis: &mut QUndoGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup7isCleanEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup7isCleanEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn redoTextChanged<RetType, T: QUndoGroup_redoTextChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.redoTextChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_redoTextChanged<RetType> {
  fn redoTextChanged(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  void QUndoGroup::redoTextChanged(const QString & redoText);
impl<'a> /*trait*/ QUndoGroup_redoTextChanged<()> for (&'a  QString) {
  fn redoTextChanged(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup15redoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup15redoTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoGroup {
  pub fn createUndoAction<RetType, T: QUndoGroup_createUndoAction<RetType>>(&mut self, value: T) -> RetType {
    return value.createUndoAction(self);
    // return 1;
  }
}

pub trait QUndoGroup_createUndoAction<RetType> {
  fn createUndoAction(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  QAction * QUndoGroup::createUndoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoGroup_createUndoAction<QAction> for (&'a mut QObject, &'a  QString) {
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
  pub fn removeStack<RetType, T: QUndoGroup_removeStack<RetType>>(&mut self, value: T) -> RetType {
    return value.removeStack(self);
    // return 1;
  }
}

pub trait QUndoGroup_removeStack<RetType> {
  fn removeStack(self, rsthis: &mut QUndoGroup) -> RetType;
}

// proto:  void QUndoGroup::removeStack(QUndoStack * stack);
impl<'a> /*trait*/ QUndoGroup_removeStack<()> for (&'a mut QUndoStack) {
  fn removeStack(self, rsthis: &mut QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup11removeStackEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup11removeStackEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

