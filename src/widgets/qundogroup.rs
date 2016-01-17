// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtWidgets/qundogroup.h
// dst-file: /src/widgets/qundogroup.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::qundostack::QUndoStack; // 773
use super::super::core::qstring::QString; // 771
use super::qaction::QAction; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QUndoGroup_Class_Size() -> c_int;
  // proto:  void QUndoGroup::addStack(QUndoStack * stack);
  fn _ZN10QUndoGroup8addStackEP10QUndoStack(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QUndoGroup::undo();
  fn _ZN10QUndoGroup4undoEv(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QUndoStack *> QUndoGroup::stacks();
  fn _ZNK10QUndoGroup6stacksEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QUndoGroup::redo();
  fn _ZN10QUndoGroup4redoEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QUndoGroup::QUndoGroup(QObject * parent);
  fn _ZN10QUndoGroupC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QUndoGroup::setActiveStack(QUndoStack * stack);
  fn _ZN10QUndoGroup14setActiveStackEP10QUndoStack(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QAction * QUndoGroup::createRedoAction(QObject * parent, const QString & prefix);
  fn _ZNK10QUndoGroup16createRedoActionEP7QObjectRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QUndoGroup::QUndoGroup(const QUndoGroup & );
  fn _ZN10QUndoGroupC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QUndoGroup::metaObject();
  fn _ZNK10QUndoGroup10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QUndoGroup::canRedo();
  fn _ZNK10QUndoGroup7canRedoEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QUndoGroup::redoText();
  fn _ZNK10QUndoGroup8redoTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QUndoStack * QUndoGroup::activeStack();
  fn _ZNK10QUndoGroup11activeStackEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QUndoGroup::undoText();
  fn _ZNK10QUndoGroup8undoTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QUndoGroup::canUndo();
  fn _ZNK10QUndoGroup7canUndoEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QUndoGroup::~QUndoGroup();
  fn _ZN10QUndoGroupD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QUndoGroup::isClean();
  fn _ZNK10QUndoGroup7isCleanEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QAction * QUndoGroup::createUndoAction(QObject * parent, const QString & prefix);
  fn _ZNK10QUndoGroup16createUndoActionEP7QObjectRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QUndoGroup::removeStack(QUndoStack * stack);
  fn _ZN10QUndoGroup11removeStackEP10QUndoStack(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QUndoGroup_SlotProxy_connect__ZN10QUndoGroup14canRedoChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QUndoGroup_SlotProxy_connect__ZN10QUndoGroup18activeStackChangedEP10QUndoStack(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QUndoGroup_SlotProxy_connect__ZN10QUndoGroup12cleanChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QUndoGroup_SlotProxy_connect__ZN10QUndoGroup15undoTextChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QUndoGroup_SlotProxy_connect__ZN10QUndoGroup14canUndoChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QUndoGroup_SlotProxy_connect__ZN10QUndoGroup12indexChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QUndoGroup_SlotProxy_connect__ZN10QUndoGroup15redoTextChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QUndoGroup)=1
#[derive(Default)]
pub struct QUndoGroup {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _redoTextChanged: QUndoGroup_redoTextChanged_signal,
  pub _cleanChanged: QUndoGroup_cleanChanged_signal,
  pub _canUndoChanged: QUndoGroup_canUndoChanged_signal,
  pub _indexChanged: QUndoGroup_indexChanged_signal,
  pub _activeStackChanged: QUndoGroup_activeStackChanged_signal,
  pub _canRedoChanged: QUndoGroup_canRedoChanged_signal,
  pub _undoTextChanged: QUndoGroup_undoTextChanged_signal,
}

impl /*struct*/ QUndoGroup {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QUndoGroup {
    return QUndoGroup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QUndoGroup {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QUndoGroup {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QUndoGroup::addStack(QUndoStack * stack);
impl /*struct*/ QUndoGroup {
  pub fn addStack<RetType, T: QUndoGroup_addStack<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addStack(self);
    // return 1;
  }
}

pub trait QUndoGroup_addStack<RetType> {
  fn addStack(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::addStack(QUndoStack * stack);
impl<'a> /*trait*/ QUndoGroup_addStack<()> for (&'a QUndoStack) {
  fn addStack(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup8addStackEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup8addStackEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QUndoGroup::undo();
impl /*struct*/ QUndoGroup {
  pub fn undo<RetType, T: QUndoGroup_undo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undo(self);
    // return 1;
  }
}

pub trait QUndoGroup_undo<RetType> {
  fn undo(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::undo();
impl<'a> /*trait*/ QUndoGroup_undo<()> for () {
  fn undo(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup4undoEv()};
     unsafe {_ZN10QUndoGroup4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QUndoStack *> QUndoGroup::stacks();
impl /*struct*/ QUndoGroup {
  pub fn stacks<RetType, T: QUndoGroup_stacks<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stacks(self);
    // return 1;
  }
}

pub trait QUndoGroup_stacks<RetType> {
  fn stacks(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  QList<QUndoStack *> QUndoGroup::stacks();
impl<'a> /*trait*/ QUndoGroup_stacks<()> for () {
  fn stacks(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup6stacksEv()};
     unsafe {_ZNK10QUndoGroup6stacksEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUndoGroup::redo();
impl /*struct*/ QUndoGroup {
  pub fn redo<RetType, T: QUndoGroup_redo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redo(self);
    // return 1;
  }
}

pub trait QUndoGroup_redo<RetType> {
  fn redo(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::redo();
impl<'a> /*trait*/ QUndoGroup_redo<()> for () {
  fn redo(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup4redoEv()};
     unsafe {_ZN10QUndoGroup4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUndoGroup::QUndoGroup(QObject * parent);
impl /*struct*/ QUndoGroup {
  pub fn new<T: QUndoGroup_new>(value: T) -> QUndoGroup {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoGroup_new {
  fn new(self) -> QUndoGroup;
}

  // proto:  void QUndoGroup::QUndoGroup(QObject * parent);
impl<'a> /*trait*/ QUndoGroup_new for (&'a QObject) {
  fn new(self) -> QUndoGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroupC2EP7QObject()};
    let ctysz: c_int = unsafe{QUndoGroup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QUndoGroupC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QUndoGroup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QUndoGroup::setActiveStack(QUndoStack * stack);
impl /*struct*/ QUndoGroup {
  pub fn setActiveStack<RetType, T: QUndoGroup_setActiveStack<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setActiveStack(self);
    // return 1;
  }
}

pub trait QUndoGroup_setActiveStack<RetType> {
  fn setActiveStack(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::setActiveStack(QUndoStack * stack);
impl<'a> /*trait*/ QUndoGroup_setActiveStack<()> for (&'a QUndoStack) {
  fn setActiveStack(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup14setActiveStackEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup14setActiveStackEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAction * QUndoGroup::createRedoAction(QObject * parent, const QString & prefix);
impl /*struct*/ QUndoGroup {
  pub fn createRedoAction<RetType, T: QUndoGroup_createRedoAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createRedoAction(self);
    // return 1;
  }
}

pub trait QUndoGroup_createRedoAction<RetType> {
  fn createRedoAction(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  QAction * QUndoGroup::createRedoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoGroup_createRedoAction<QAction> for (&'a QObject, &'a QString) {
  fn createRedoAction(self , rsthis: & QUndoGroup) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup16createRedoActionEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QUndoGroup16createRedoActionEP7QObjectRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QUndoGroup::QUndoGroup(const QUndoGroup & );
impl<'a> /*trait*/ QUndoGroup_new for (&'a QUndoGroup) {
  fn new(self) -> QUndoGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroupC2ERKS_()};
    let ctysz: c_int = unsafe{QUndoGroup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QUndoGroupC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QUndoGroup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QUndoGroup::metaObject();
impl /*struct*/ QUndoGroup {
  pub fn metaObject<RetType, T: QUndoGroup_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QUndoGroup_metaObject<RetType> {
  fn metaObject(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  const QMetaObject * QUndoGroup::metaObject();
impl<'a> /*trait*/ QUndoGroup_metaObject<()> for () {
  fn metaObject(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup10metaObjectEv()};
     unsafe {_ZNK10QUndoGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QUndoGroup::canRedo();
impl /*struct*/ QUndoGroup {
  pub fn canRedo<RetType, T: QUndoGroup_canRedo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canRedo(self);
    // return 1;
  }
}

pub trait QUndoGroup_canRedo<RetType> {
  fn canRedo(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  bool QUndoGroup::canRedo();
impl<'a> /*trait*/ QUndoGroup_canRedo<i8> for () {
  fn canRedo(self , rsthis: & QUndoGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup7canRedoEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup7canRedoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QUndoGroup::redoText();
impl /*struct*/ QUndoGroup {
  pub fn redoText<RetType, T: QUndoGroup_redoText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redoText(self);
    // return 1;
  }
}

pub trait QUndoGroup_redoText<RetType> {
  fn redoText(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  QString QUndoGroup::redoText();
impl<'a> /*trait*/ QUndoGroup_redoText<QString> for () {
  fn redoText(self , rsthis: & QUndoGroup) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup8redoTextEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup8redoTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QUndoStack * QUndoGroup::activeStack();
impl /*struct*/ QUndoGroup {
  pub fn activeStack<RetType, T: QUndoGroup_activeStack<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeStack(self);
    // return 1;
  }
}

pub trait QUndoGroup_activeStack<RetType> {
  fn activeStack(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  QUndoStack * QUndoGroup::activeStack();
impl<'a> /*trait*/ QUndoGroup_activeStack<QUndoStack> for () {
  fn activeStack(self , rsthis: & QUndoGroup) -> QUndoStack {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup11activeStackEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup11activeStackEv(rsthis.qclsinst)};
    let mut ret1 = QUndoStack::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QUndoGroup::undoText();
impl /*struct*/ QUndoGroup {
  pub fn undoText<RetType, T: QUndoGroup_undoText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undoText(self);
    // return 1;
  }
}

pub trait QUndoGroup_undoText<RetType> {
  fn undoText(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  QString QUndoGroup::undoText();
impl<'a> /*trait*/ QUndoGroup_undoText<QString> for () {
  fn undoText(self , rsthis: & QUndoGroup) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup8undoTextEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup8undoTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QUndoGroup::canUndo();
impl /*struct*/ QUndoGroup {
  pub fn canUndo<RetType, T: QUndoGroup_canUndo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canUndo(self);
    // return 1;
  }
}

pub trait QUndoGroup_canUndo<RetType> {
  fn canUndo(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  bool QUndoGroup::canUndo();
impl<'a> /*trait*/ QUndoGroup_canUndo<i8> for () {
  fn canUndo(self , rsthis: & QUndoGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup7canUndoEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup7canUndoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QUndoGroup::~QUndoGroup();
impl /*struct*/ QUndoGroup {
  pub fn free<RetType, T: QUndoGroup_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QUndoGroup_free<RetType> {
  fn free(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::~QUndoGroup();
impl<'a> /*trait*/ QUndoGroup_free<()> for () {
  fn free(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroupD2Ev()};
     unsafe {_ZN10QUndoGroupD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QUndoGroup::isClean();
impl /*struct*/ QUndoGroup {
  pub fn isClean<RetType, T: QUndoGroup_isClean<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isClean(self);
    // return 1;
  }
}

pub trait QUndoGroup_isClean<RetType> {
  fn isClean(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  bool QUndoGroup::isClean();
impl<'a> /*trait*/ QUndoGroup_isClean<i8> for () {
  fn isClean(self , rsthis: & QUndoGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup7isCleanEv()};
    let mut ret = unsafe {_ZNK10QUndoGroup7isCleanEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QAction * QUndoGroup::createUndoAction(QObject * parent, const QString & prefix);
impl /*struct*/ QUndoGroup {
  pub fn createUndoAction<RetType, T: QUndoGroup_createUndoAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createUndoAction(self);
    // return 1;
  }
}

pub trait QUndoGroup_createUndoAction<RetType> {
  fn createUndoAction(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  QAction * QUndoGroup::createUndoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoGroup_createUndoAction<QAction> for (&'a QObject, &'a QString) {
  fn createUndoAction(self , rsthis: & QUndoGroup) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoGroup16createUndoActionEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QUndoGroup16createUndoActionEP7QObjectRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QUndoGroup::removeStack(QUndoStack * stack);
impl /*struct*/ QUndoGroup {
  pub fn removeStack<RetType, T: QUndoGroup_removeStack<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeStack(self);
    // return 1;
  }
}

pub trait QUndoGroup_removeStack<RetType> {
  fn removeStack(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::removeStack(QUndoStack * stack);
impl<'a> /*trait*/ QUndoGroup_removeStack<()> for (&'a QUndoStack) {
  fn removeStack(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup11removeStackEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup11removeStackEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QUndoGroup_redoTextChanged
pub struct QUndoGroup_redoTextChanged_signal{poi:u64}
impl /* struct */ QUndoGroup {
  pub fn redoTextChanged(&self) -> QUndoGroup_redoTextChanged_signal {
     return QUndoGroup_redoTextChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QUndoGroup_redoTextChanged_signal {
  pub fn connect<T: QUndoGroup_redoTextChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QUndoGroup_redoTextChanged_signal_connect {
  fn connect(self, sigthis: QUndoGroup_redoTextChanged_signal);
}

#[derive(Default)] // for QUndoGroup_cleanChanged
pub struct QUndoGroup_cleanChanged_signal{poi:u64}
impl /* struct */ QUndoGroup {
  pub fn cleanChanged(&self) -> QUndoGroup_cleanChanged_signal {
     return QUndoGroup_cleanChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QUndoGroup_cleanChanged_signal {
  pub fn connect<T: QUndoGroup_cleanChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QUndoGroup_cleanChanged_signal_connect {
  fn connect(self, sigthis: QUndoGroup_cleanChanged_signal);
}

#[derive(Default)] // for QUndoGroup_canUndoChanged
pub struct QUndoGroup_canUndoChanged_signal{poi:u64}
impl /* struct */ QUndoGroup {
  pub fn canUndoChanged(&self) -> QUndoGroup_canUndoChanged_signal {
     return QUndoGroup_canUndoChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QUndoGroup_canUndoChanged_signal {
  pub fn connect<T: QUndoGroup_canUndoChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QUndoGroup_canUndoChanged_signal_connect {
  fn connect(self, sigthis: QUndoGroup_canUndoChanged_signal);
}

#[derive(Default)] // for QUndoGroup_indexChanged
pub struct QUndoGroup_indexChanged_signal{poi:u64}
impl /* struct */ QUndoGroup {
  pub fn indexChanged(&self) -> QUndoGroup_indexChanged_signal {
     return QUndoGroup_indexChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QUndoGroup_indexChanged_signal {
  pub fn connect<T: QUndoGroup_indexChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QUndoGroup_indexChanged_signal_connect {
  fn connect(self, sigthis: QUndoGroup_indexChanged_signal);
}

#[derive(Default)] // for QUndoGroup_activeStackChanged
pub struct QUndoGroup_activeStackChanged_signal{poi:u64}
impl /* struct */ QUndoGroup {
  pub fn activeStackChanged(&self) -> QUndoGroup_activeStackChanged_signal {
     return QUndoGroup_activeStackChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QUndoGroup_activeStackChanged_signal {
  pub fn connect<T: QUndoGroup_activeStackChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QUndoGroup_activeStackChanged_signal_connect {
  fn connect(self, sigthis: QUndoGroup_activeStackChanged_signal);
}

#[derive(Default)] // for QUndoGroup_canRedoChanged
pub struct QUndoGroup_canRedoChanged_signal{poi:u64}
impl /* struct */ QUndoGroup {
  pub fn canRedoChanged(&self) -> QUndoGroup_canRedoChanged_signal {
     return QUndoGroup_canRedoChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QUndoGroup_canRedoChanged_signal {
  pub fn connect<T: QUndoGroup_canRedoChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QUndoGroup_canRedoChanged_signal_connect {
  fn connect(self, sigthis: QUndoGroup_canRedoChanged_signal);
}

#[derive(Default)] // for QUndoGroup_undoTextChanged
pub struct QUndoGroup_undoTextChanged_signal{poi:u64}
impl /* struct */ QUndoGroup {
  pub fn undoTextChanged(&self) -> QUndoGroup_undoTextChanged_signal {
     return QUndoGroup_undoTextChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QUndoGroup_undoTextChanged_signal {
  pub fn connect<T: QUndoGroup_undoTextChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QUndoGroup_undoTextChanged_signal_connect {
  fn connect(self, sigthis: QUndoGroup_undoTextChanged_signal);
}

// canRedoChanged(_Bool)
extern fn QUndoGroup_canRedoChanged_signal_connect_cb_0(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QUndoGroup_canRedoChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QUndoGroup_canRedoChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QUndoGroup_canRedoChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_canRedoChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup14canRedoChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QUndoGroup_canRedoChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QUndoGroup_canRedoChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_canRedoChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup14canRedoChangedEb(arg0, arg1, arg2)};
  }
}
// activeStackChanged(class QUndoStack *)
extern fn QUndoGroup_activeStackChanged_signal_connect_cb_1(rsfptr:fn(QUndoStack), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QUndoStack::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QUndoGroup_activeStackChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QUndoStack)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QUndoStack::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QUndoGroup_activeStackChanged_signal_connect for fn(QUndoStack) {
  fn connect(self, sigthis: QUndoGroup_activeStackChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_activeStackChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup18activeStackChangedEP10QUndoStack(arg0, arg1, arg2)};
  }
}
impl /* trait */ QUndoGroup_activeStackChanged_signal_connect for Box<Fn(QUndoStack)> {
  fn connect(self, sigthis: QUndoGroup_activeStackChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_activeStackChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup18activeStackChangedEP10QUndoStack(arg0, arg1, arg2)};
  }
}
// cleanChanged(_Bool)
extern fn QUndoGroup_cleanChanged_signal_connect_cb_2(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QUndoGroup_cleanChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QUndoGroup_cleanChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QUndoGroup_cleanChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_cleanChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup12cleanChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QUndoGroup_cleanChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QUndoGroup_cleanChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_cleanChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup12cleanChangedEb(arg0, arg1, arg2)};
  }
}
// undoTextChanged(const class QString &)
extern fn QUndoGroup_undoTextChanged_signal_connect_cb_3(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QUndoGroup_undoTextChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QUndoGroup_undoTextChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QUndoGroup_undoTextChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_undoTextChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup15undoTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QUndoGroup_undoTextChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QUndoGroup_undoTextChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_undoTextChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup15undoTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
// canUndoChanged(_Bool)
extern fn QUndoGroup_canUndoChanged_signal_connect_cb_4(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QUndoGroup_canUndoChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QUndoGroup_canUndoChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QUndoGroup_canUndoChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_canUndoChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup14canUndoChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QUndoGroup_canUndoChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QUndoGroup_canUndoChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_canUndoChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup14canUndoChangedEb(arg0, arg1, arg2)};
  }
}
// indexChanged(int)
extern fn QUndoGroup_indexChanged_signal_connect_cb_5(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QUndoGroup_indexChanged_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QUndoGroup_indexChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QUndoGroup_indexChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_indexChanged_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup12indexChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QUndoGroup_indexChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QUndoGroup_indexChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_indexChanged_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup12indexChangedEi(arg0, arg1, arg2)};
  }
}
// redoTextChanged(const class QString &)
extern fn QUndoGroup_redoTextChanged_signal_connect_cb_6(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QUndoGroup_redoTextChanged_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QUndoGroup_redoTextChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QUndoGroup_redoTextChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_redoTextChanged_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup15redoTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QUndoGroup_redoTextChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QUndoGroup_redoTextChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QUndoGroup_redoTextChanged_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QUndoGroup_SlotProxy_connect__ZN10QUndoGroup15redoTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
// <= body block end

