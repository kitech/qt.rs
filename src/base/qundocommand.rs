// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QUndoCommand::id();
  fn _ZNK12QUndoCommand2idEv() -> i32;
  // proto: void QUndoCommand::redo();
  fn _ZN12QUndoCommand4redoEv() -> i32;
  // proto: void QUndoCommand::NewQUndoCommand(const QUndoCommand & );
  fn _ZN12QUndoCommandC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QUndoCommand::NewQUndoCommand(QUndoCommand * parent);
  fn _ZN12QUndoCommandC1EPS_(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QUndoCommand::undo();
  fn _ZN12QUndoCommand4undoEv() -> i32;
  // proto: void QUndoCommand::NewQUndoCommand(const QString & text, QUndoCommand * parent);
  fn _ZN12QUndoCommandC1ERK7QStringPS_(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: bool QUndoCommand::mergeWith(const QUndoCommand * other);
  fn _ZN12QUndoCommand9mergeWithEPKS_(arg0: *const c_void) -> i32;
  // proto: QString QUndoCommand::text();
  fn _ZNK12QUndoCommand4textEv() -> i32;
  // proto: int QUndoCommand::childCount();
  fn _ZNK12QUndoCommand10childCountEv() -> i32;
  // proto: QString QUndoCommand::actionText();
  fn _ZNK12QUndoCommand10actionTextEv() -> i32;
  // proto: void QUndoCommand::FreeQUndoCommand();
  fn _ZN12QUndoCommandD0Ev() -> i32;
  // proto: const QUndoCommand * QUndoCommand::child(int index);
  fn _ZNK12QUndoCommand5childEi(arg0: c_int) -> i32;
  // proto: void QUndoCommand::setText(const QString & text);
  fn _ZN12QUndoCommand7setTextERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QUndoCommand)=16
pub struct QUndoCommand {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QUndoCommand {
  pub fn id<T: QUndoCommand_id>(&mut self, value: T) -> i32 {
    value.id(self);
    return 1;
  }
}

pub trait QUndoCommand_id {
  fn id(self, this: &mut QUndoCommand) -> i32;
}

// proto: int QUndoCommand::id();
impl<'a> /*trait*/ QUndoCommand_id for () {
  fn id(self, this: &mut QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand2idEv()};
    unsafe {_ZNK12QUndoCommand2idEv()};
    return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn redo<T: QUndoCommand_redo>(&mut self, value: T) -> i32 {
    value.redo(self);
    return 1;
  }
}

pub trait QUndoCommand_redo {
  fn redo(self, this: &mut QUndoCommand) -> i32;
}

// proto: void QUndoCommand::redo();
impl<'a> /*trait*/ QUndoCommand_redo for () {
  fn redo(self, this: &mut QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommand4redoEv()};
    unsafe {_ZN12QUndoCommand4redoEv()};
    return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn NewQUndoCommand<T: QUndoCommand_NewQUndoCommand>(value: T) -> QUndoCommand {
    let rsthis = value.NewQUndoCommand();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoCommand_NewQUndoCommand {
  fn NewQUndoCommand(self) -> QUndoCommand;
}

// proto: void QUndoCommand::NewQUndoCommand(const QUndoCommand & );
impl<'a> /*trait*/ QUndoCommand_NewQUndoCommand for (&'a  QUndoCommand) {
  fn NewQUndoCommand(self) -> QUndoCommand {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommandC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QUndoCommandC1ERKS_(qthis, arg0)};
    let rsthis = QUndoCommand{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QUndoCommand::NewQUndoCommand(QUndoCommand * parent);
impl<'a> /*trait*/ QUndoCommand_NewQUndoCommand for (&'a mut QUndoCommand) {
  fn NewQUndoCommand(self) -> QUndoCommand {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommandC1EPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QUndoCommandC1EPS_(qthis, arg0)};
    let rsthis = QUndoCommand{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn undo<T: QUndoCommand_undo>(&mut self, value: T) -> i32 {
    value.undo(self);
    return 1;
  }
}

pub trait QUndoCommand_undo {
  fn undo(self, this: &mut QUndoCommand) -> i32;
}

// proto: void QUndoCommand::undo();
impl<'a> /*trait*/ QUndoCommand_undo for () {
  fn undo(self, this: &mut QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommand4undoEv()};
    unsafe {_ZN12QUndoCommand4undoEv()};
    return 1;
  }
}

// proto: void QUndoCommand::NewQUndoCommand(const QString & text, QUndoCommand * parent);
impl<'a> /*trait*/ QUndoCommand_NewQUndoCommand for (&'a  QString, &'a mut QUndoCommand) {
  fn NewQUndoCommand(self) -> QUndoCommand {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommandC1ERK7QStringPS_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QUndoCommandC1ERK7QStringPS_(qthis, arg0, arg1)};
    let rsthis = QUndoCommand{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn mergeWith<T: QUndoCommand_mergeWith>(&mut self, value: T) -> i32 {
    value.mergeWith(self);
    return 1;
  }
}

pub trait QUndoCommand_mergeWith {
  fn mergeWith(self, this: &mut QUndoCommand) -> i32;
}

// proto: bool QUndoCommand::mergeWith(const QUndoCommand * other);
impl<'a> /*trait*/ QUndoCommand_mergeWith for (&'a  QUndoCommand) {
  fn mergeWith(self, this: &mut QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommand9mergeWithEPKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QUndoCommand9mergeWithEPKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn text<T: QUndoCommand_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QUndoCommand_text {
  fn text(self, this: &mut QUndoCommand) -> i32;
}

// proto: QString QUndoCommand::text();
impl<'a> /*trait*/ QUndoCommand_text for () {
  fn text(self, this: &mut QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand4textEv()};
    unsafe {_ZNK12QUndoCommand4textEv()};
    return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn childCount<T: QUndoCommand_childCount>(&mut self, value: T) -> i32 {
    value.childCount(self);
    return 1;
  }
}

pub trait QUndoCommand_childCount {
  fn childCount(self, this: &mut QUndoCommand) -> i32;
}

// proto: int QUndoCommand::childCount();
impl<'a> /*trait*/ QUndoCommand_childCount for () {
  fn childCount(self, this: &mut QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand10childCountEv()};
    unsafe {_ZNK12QUndoCommand10childCountEv()};
    return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn actionText<T: QUndoCommand_actionText>(&mut self, value: T) -> i32 {
    value.actionText(self);
    return 1;
  }
}

pub trait QUndoCommand_actionText {
  fn actionText(self, this: &mut QUndoCommand) -> i32;
}

// proto: QString QUndoCommand::actionText();
impl<'a> /*trait*/ QUndoCommand_actionText for () {
  fn actionText(self, this: &mut QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand10actionTextEv()};
    unsafe {_ZNK12QUndoCommand10actionTextEv()};
    return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn FreeQUndoCommand<T: QUndoCommand_FreeQUndoCommand>(&mut self, value: T) -> i32 {
    value.FreeQUndoCommand(self);
    return 1;
  }
}

pub trait QUndoCommand_FreeQUndoCommand {
  fn FreeQUndoCommand(self, this: &mut QUndoCommand) -> i32;
}

// proto: void QUndoCommand::FreeQUndoCommand();
impl<'a> /*trait*/ QUndoCommand_FreeQUndoCommand for () {
  fn FreeQUndoCommand(self, this: &mut QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommandD0Ev()};
    unsafe {_ZN12QUndoCommandD0Ev()};
    return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn child<T: QUndoCommand_child>(&mut self, value: T) -> i32 {
    value.child(self);
    return 1;
  }
}

pub trait QUndoCommand_child {
  fn child(self, this: &mut QUndoCommand) -> i32;
}

// proto: const QUndoCommand * QUndoCommand::child(int index);
impl<'a> /*trait*/ QUndoCommand_child for (i32) {
  fn child(self, this: &mut QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand5childEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK12QUndoCommand5childEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn setText<T: QUndoCommand_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QUndoCommand_setText {
  fn setText(self, this: &mut QUndoCommand) -> i32;
}

// proto: void QUndoCommand::setText(const QString & text);
impl<'a> /*trait*/ QUndoCommand_setText for (&'a  QString) {
  fn setText(self, this: &mut QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommand7setTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QUndoCommand7setTextERK7QString(arg0)};
    return 1;
  }
}

