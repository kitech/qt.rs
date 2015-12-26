// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
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
  fn _ZN10QUndoGroup8addStackEP10QUndoStack(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QUndoGroup::undo();
  fn _ZN10QUndoGroup4undoEv(qthis: *mut c_void);
  // proto:  QList<QUndoStack *> QUndoGroup::stacks();
  fn _ZNK10QUndoGroup6stacksEv(qthis: *mut c_void);
  // proto:  void QUndoGroup::canRedoChanged(bool canRedo);
  fn _ZN10QUndoGroup14canRedoChangedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QUndoGroup::cleanChanged(bool clean);
  fn _ZN10QUndoGroup12cleanChangedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QUndoGroup::canUndoChanged(bool canUndo);
  fn _ZN10QUndoGroup14canUndoChangedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QUndoGroup::redo();
  fn _ZN10QUndoGroup4redoEv(qthis: *mut c_void);
  // proto:  void QUndoGroup::QUndoGroup(QObject * parent);
  fn dector_ZN10QUndoGroupC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QUndoGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QUndoGroup::setActiveStack(QUndoStack * stack);
  fn _ZN10QUndoGroup14setActiveStackEP10QUndoStack(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QAction * QUndoGroup::createRedoAction(QObject * parent, const QString & prefix);
  fn _ZNK10QUndoGroup16createRedoActionEP7QObjectRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QUndoGroup::QUndoGroup(const QUndoGroup & );
  fn dector_ZN10QUndoGroupC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QUndoGroupC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QUndoGroup::metaObject();
  fn _ZNK10QUndoGroup10metaObjectEv(qthis: *mut c_void);
  // proto:  bool QUndoGroup::canRedo();
  fn _ZNK10QUndoGroup7canRedoEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QUndoGroup::redoText();
  fn _ZNK10QUndoGroup8redoTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QUndoStack * QUndoGroup::activeStack();
  fn _ZNK10QUndoGroup11activeStackEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QUndoGroup::undoText();
  fn _ZNK10QUndoGroup8undoTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QUndoGroup::indexChanged(int idx);
  fn _ZN10QUndoGroup12indexChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QUndoGroup::canUndo();
  fn _ZNK10QUndoGroup7canUndoEv(qthis: *mut c_void) -> c_char;
  // proto:  void QUndoGroup::~QUndoGroup();
  fn _ZN10QUndoGroupD0Ev(qthis: *mut c_void);
  // proto:  void QUndoGroup::activeStackChanged(QUndoStack * stack);
  fn _ZN10QUndoGroup18activeStackChangedEP10QUndoStack(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QUndoGroup::undoTextChanged(const QString & undoText);
  fn _ZN10QUndoGroup15undoTextChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QUndoGroup::isClean();
  fn _ZNK10QUndoGroup7isCleanEv(qthis: *mut c_void) -> c_char;
  // proto:  void QUndoGroup::redoTextChanged(const QString & redoText);
  fn _ZN10QUndoGroup15redoTextChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QAction * QUndoGroup::createUndoAction(QObject * parent, const QString & prefix);
  fn _ZNK10QUndoGroup16createUndoActionEP7QObjectRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QUndoGroup::removeStack(QUndoStack * stack);
  fn _ZN10QUndoGroup11removeStackEP10QUndoStack(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QUndoGroup)=1
pub struct QUndoGroup {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QUndoGroup {
  pub fn inheritFrom(qthis: *mut c_void) -> QUndoGroup {
    return QUndoGroup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
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

  // proto:  void QUndoGroup::canRedoChanged(bool canRedo);
impl /*struct*/ QUndoGroup {
  pub fn canRedoChanged<RetType, T: QUndoGroup_canRedoChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canRedoChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_canRedoChanged<RetType> {
  fn canRedoChanged(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::canRedoChanged(bool canRedo);
impl<'a> /*trait*/ QUndoGroup_canRedoChanged<()> for (i8) {
  fn canRedoChanged(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup14canRedoChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QUndoGroup14canRedoChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QUndoGroup::cleanChanged(bool clean);
impl /*struct*/ QUndoGroup {
  pub fn cleanChanged<RetType, T: QUndoGroup_cleanChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cleanChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_cleanChanged<RetType> {
  fn cleanChanged(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::cleanChanged(bool clean);
impl<'a> /*trait*/ QUndoGroup_cleanChanged<()> for (i8) {
  fn cleanChanged(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup12cleanChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QUndoGroup12cleanChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QUndoGroup::canUndoChanged(bool canUndo);
impl /*struct*/ QUndoGroup {
  pub fn canUndoChanged<RetType, T: QUndoGroup_canUndoChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canUndoChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_canUndoChanged<RetType> {
  fn canUndoChanged(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::canUndoChanged(bool canUndo);
impl<'a> /*trait*/ QUndoGroup_canUndoChanged<()> for (i8) {
  fn canUndoChanged(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup14canUndoChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QUndoGroup14canUndoChangedEb(rsthis.qclsinst, arg0)};
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
  pub fn New<T: QUndoGroup_New>(value: T) -> QUndoGroup {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoGroup_New {
  fn New(self) -> QUndoGroup;
}

  // proto:  void QUndoGroup::QUndoGroup(QObject * parent);
impl<'a> /*trait*/ QUndoGroup_New for (&'a QObject) {
  fn New(self) -> QUndoGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroupC1EP7QObject()};
    let ctysz: c_int = unsafe{QUndoGroup_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QUndoGroupC1EP7QObject(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN10QUndoGroupC1EP7QObject(arg0)};
    let rsthis = QUndoGroup{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
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
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QUndoGroup::QUndoGroup(const QUndoGroup & );
impl<'a> /*trait*/ QUndoGroup_New for (&'a QUndoGroup) {
  fn New(self) -> QUndoGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroupC1ERKS_()};
    let ctysz: c_int = unsafe{QUndoGroup_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QUndoGroupC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN10QUndoGroupC1ERKS_(arg0)};
    let rsthis = QUndoGroup{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
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
    let mut ret1 = QString::inheritFrom(ret);
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
    let mut ret1 = QUndoStack::inheritFrom(ret);
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
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QUndoGroup::indexChanged(int idx);
impl /*struct*/ QUndoGroup {
  pub fn indexChanged<RetType, T: QUndoGroup_indexChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_indexChanged<RetType> {
  fn indexChanged(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::indexChanged(int idx);
impl<'a> /*trait*/ QUndoGroup_indexChanged<()> for (i32) {
  fn indexChanged(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup12indexChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QUndoGroup12indexChangedEi(rsthis.qclsinst, arg0)};
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
  pub fn Free<RetType, T: QUndoGroup_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QUndoGroup_Free<RetType> {
  fn Free(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::~QUndoGroup();
impl<'a> /*trait*/ QUndoGroup_Free<()> for () {
  fn Free(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroupD0Ev()};
     unsafe {_ZN10QUndoGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUndoGroup::activeStackChanged(QUndoStack * stack);
impl /*struct*/ QUndoGroup {
  pub fn activeStackChanged<RetType, T: QUndoGroup_activeStackChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeStackChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_activeStackChanged<RetType> {
  fn activeStackChanged(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::activeStackChanged(QUndoStack * stack);
impl<'a> /*trait*/ QUndoGroup_activeStackChanged<()> for (&'a QUndoStack) {
  fn activeStackChanged(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup18activeStackChangedEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup18activeStackChangedEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QUndoGroup::undoTextChanged(const QString & undoText);
impl /*struct*/ QUndoGroup {
  pub fn undoTextChanged<RetType, T: QUndoGroup_undoTextChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undoTextChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_undoTextChanged<RetType> {
  fn undoTextChanged(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::undoTextChanged(const QString & undoText);
impl<'a> /*trait*/ QUndoGroup_undoTextChanged<()> for (&'a QString) {
  fn undoTextChanged(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup15undoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup15undoTextChangedERK7QString(rsthis.qclsinst, arg0)};
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

  // proto:  void QUndoGroup::redoTextChanged(const QString & redoText);
impl /*struct*/ QUndoGroup {
  pub fn redoTextChanged<RetType, T: QUndoGroup_redoTextChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redoTextChanged(self);
    // return 1;
  }
}

pub trait QUndoGroup_redoTextChanged<RetType> {
  fn redoTextChanged(self , rsthis: & QUndoGroup) -> RetType;
}

  // proto:  void QUndoGroup::redoTextChanged(const QString & redoText);
impl<'a> /*trait*/ QUndoGroup_redoTextChanged<()> for (&'a QString) {
  fn redoTextChanged(self , rsthis: & QUndoGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoGroup15redoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoGroup15redoTextChangedERK7QString(rsthis.qclsinst, arg0)};
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
    let mut ret1 = QAction::inheritFrom(ret);
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

// <= body block end

