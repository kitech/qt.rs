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
  // proto:  int QUndoCommand::id();
  fn _ZNK12QUndoCommand2idEv(qthis: *mut c_void) -> c_int;
  // proto:  void QUndoCommand::redo();
  fn _ZN12QUndoCommand4redoEv(qthis: *mut c_void) ;
  // proto:  void QUndoCommand::NewQUndoCommand(const QUndoCommand & );
  fn _ZN12QUndoCommandC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUndoCommand::NewQUndoCommand(QUndoCommand * parent);
  fn _ZN12QUndoCommandC1EPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUndoCommand::undo();
  fn _ZN12QUndoCommand4undoEv(qthis: *mut c_void) ;
  // proto:  void QUndoCommand::NewQUndoCommand(const QString & text, QUndoCommand * parent);
  fn _ZN12QUndoCommandC1ERK7QStringPS_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QUndoCommand::mergeWith(const QUndoCommand * other);
  fn _ZN12QUndoCommand9mergeWithEPKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QString QUndoCommand::text();
  fn _ZNK12QUndoCommand4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QUndoCommand::childCount();
  fn _ZNK12QUndoCommand10childCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QUndoCommand::actionText();
  fn _ZNK12QUndoCommand10actionTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QUndoCommand::FreeQUndoCommand();
  fn _ZN12QUndoCommandD0Ev(qthis: *mut c_void) ;
  // proto:  const QUndoCommand * QUndoCommand::child(int index);
  fn _ZNK12QUndoCommand5childEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QUndoCommand::setText(const QString & text);
  fn _ZN12QUndoCommand7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QUndoCommand)=16
pub struct QUndoCommand {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QUndoCommand {
  pub fn id<T: QUndoCommand_id>(&mut self, value: T) -> i32 {
    return value.id(self);
    // return 1;
  }
}

pub trait QUndoCommand_id {
  fn id(self, rsthis: &mut QUndoCommand) -> i32;
}

// proto:  int QUndoCommand::id();
impl<'a> /*trait*/ QUndoCommand_id for () {
  fn id(self, rsthis: &mut QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand2idEv()};
    let mut ret = unsafe {_ZNK12QUndoCommand2idEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn redo<T: QUndoCommand_redo>(&mut self, value: T)  {
     value.redo(self);
    // return 1;
  }
}

pub trait QUndoCommand_redo {
  fn redo(self, rsthis: &mut QUndoCommand) ;
}

// proto:  void QUndoCommand::redo();
impl<'a> /*trait*/ QUndoCommand_redo for () {
  fn redo(self, rsthis: &mut QUndoCommand)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommand4redoEv()};
     unsafe {_ZN12QUndoCommand4redoEv(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
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
  pub fn undo<T: QUndoCommand_undo>(&mut self, value: T)  {
     value.undo(self);
    // return 1;
  }
}

pub trait QUndoCommand_undo {
  fn undo(self, rsthis: &mut QUndoCommand) ;
}

// proto:  void QUndoCommand::undo();
impl<'a> /*trait*/ QUndoCommand_undo for () {
  fn undo(self, rsthis: &mut QUndoCommand)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommand4undoEv()};
     unsafe {_ZN12QUndoCommand4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QUndoCommand::NewQUndoCommand(const QString & text, QUndoCommand * parent);
impl<'a> /*trait*/ QUndoCommand_NewQUndoCommand for (&'a  QString, &'a mut QUndoCommand) {
  fn NewQUndoCommand(self) -> QUndoCommand {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommandC1ERK7QStringPS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QUndoCommandC1ERK7QStringPS_(qthis, arg0, arg1)};
    let rsthis = QUndoCommand{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn mergeWith<T: QUndoCommand_mergeWith>(&mut self, value: T) -> i8 {
    return value.mergeWith(self);
    // return 1;
  }
}

pub trait QUndoCommand_mergeWith {
  fn mergeWith(self, rsthis: &mut QUndoCommand) -> i8;
}

// proto:  bool QUndoCommand::mergeWith(const QUndoCommand * other);
impl<'a> /*trait*/ QUndoCommand_mergeWith for (&'a  QUndoCommand) {
  fn mergeWith(self, rsthis: &mut QUndoCommand) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommand9mergeWithEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QUndoCommand9mergeWithEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn text<T: QUndoCommand_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QUndoCommand_text {
  fn text(self, rsthis: &mut QUndoCommand) -> QString;
}

// proto:  QString QUndoCommand::text();
impl<'a> /*trait*/ QUndoCommand_text for () {
  fn text(self, rsthis: &mut QUndoCommand) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand4textEv()};
    let mut ret = unsafe {_ZNK12QUndoCommand4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn childCount<T: QUndoCommand_childCount>(&mut self, value: T) -> i32 {
    return value.childCount(self);
    // return 1;
  }
}

pub trait QUndoCommand_childCount {
  fn childCount(self, rsthis: &mut QUndoCommand) -> i32;
}

// proto:  int QUndoCommand::childCount();
impl<'a> /*trait*/ QUndoCommand_childCount for () {
  fn childCount(self, rsthis: &mut QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand10childCountEv()};
    let mut ret = unsafe {_ZNK12QUndoCommand10childCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn actionText<T: QUndoCommand_actionText>(&mut self, value: T) -> QString {
    return value.actionText(self);
    // return 1;
  }
}

pub trait QUndoCommand_actionText {
  fn actionText(self, rsthis: &mut QUndoCommand) -> QString;
}

// proto:  QString QUndoCommand::actionText();
impl<'a> /*trait*/ QUndoCommand_actionText for () {
  fn actionText(self, rsthis: &mut QUndoCommand) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand10actionTextEv()};
    let mut ret = unsafe {_ZNK12QUndoCommand10actionTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn FreeQUndoCommand<T: QUndoCommand_FreeQUndoCommand>(&mut self, value: T)  {
     value.FreeQUndoCommand(self);
    // return 1;
  }
}

pub trait QUndoCommand_FreeQUndoCommand {
  fn FreeQUndoCommand(self, rsthis: &mut QUndoCommand) ;
}

// proto:  void QUndoCommand::FreeQUndoCommand();
impl<'a> /*trait*/ QUndoCommand_FreeQUndoCommand for () {
  fn FreeQUndoCommand(self, rsthis: &mut QUndoCommand)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommandD0Ev()};
     unsafe {_ZN12QUndoCommandD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn child<T: QUndoCommand_child>(&mut self, value: T) -> QUndoCommand {
    return value.child(self);
    // return 1;
  }
}

pub trait QUndoCommand_child {
  fn child(self, rsthis: &mut QUndoCommand) -> QUndoCommand;
}

// proto:  const QUndoCommand * QUndoCommand::child(int index);
impl<'a> /*trait*/ QUndoCommand_child for (i32) {
  fn child(self, rsthis: &mut QUndoCommand) -> QUndoCommand {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand5childEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QUndoCommand5childEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QUndoCommand{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn setText<T: QUndoCommand_setText>(&mut self, value: T)  {
     value.setText(self);
    // return 1;
  }
}

pub trait QUndoCommand_setText {
  fn setText(self, rsthis: &mut QUndoCommand) ;
}

// proto:  void QUndoCommand::setText(const QString & text);
impl<'a> /*trait*/ QUndoCommand_setText for (&'a  QString) {
  fn setText(self, rsthis: &mut QUndoCommand)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommand7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QUndoCommand7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

