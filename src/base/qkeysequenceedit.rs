// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qkeysequence::QKeySequence;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QKeySequenceEdit::NewQKeySequenceEdit(const QKeySequenceEdit & );
  fn _ZN16QKeySequenceEditC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QKeySequenceEdit::NewQKeySequenceEdit(const QKeySequence & keySequence, QWidget * parent);
  fn _ZN16QKeySequenceEditC1ERK12QKeySequenceP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QKeySequenceEdit::clear();
  fn _ZN16QKeySequenceEdit5clearEv(qthis: *mut c_void) ;
  // proto:  void QKeySequenceEdit::setKeySequence(const QKeySequence & keySequence);
  fn _ZN16QKeySequenceEdit14setKeySequenceERK12QKeySequence(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QKeySequence QKeySequenceEdit::keySequence();
  fn _ZNK16QKeySequenceEdit11keySequenceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QKeySequenceEdit::FreeQKeySequenceEdit();
  fn _ZN16QKeySequenceEditD0Ev(qthis: *mut c_void) ;
  // proto:  void QKeySequenceEdit::keySequenceChanged(const QKeySequence & keySequence);
  fn _ZN16QKeySequenceEdit18keySequenceChangedERK12QKeySequence(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QKeySequenceEdit::editingFinished();
  fn _ZN16QKeySequenceEdit15editingFinishedEv(qthis: *mut c_void) ;
  // proto:  void QKeySequenceEdit::NewQKeySequenceEdit(QWidget * parent);
  fn _ZN16QKeySequenceEditC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QKeySequenceEdit::metaObject();
  fn _ZNK16QKeySequenceEdit10metaObjectEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QKeySequenceEdit)=1
pub struct QKeySequenceEdit {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QKeySequenceEdit {
  pub fn NewQKeySequenceEdit<T: QKeySequenceEdit_NewQKeySequenceEdit>(value: T) -> QKeySequenceEdit {
    let rsthis = value.NewQKeySequenceEdit();
    return rsthis;
    // return 1;
  }
}

pub trait QKeySequenceEdit_NewQKeySequenceEdit {
  fn NewQKeySequenceEdit(self) -> QKeySequenceEdit;
}

// proto: void QKeySequenceEdit::NewQKeySequenceEdit(const QKeySequenceEdit & );
impl<'a> /*trait*/ QKeySequenceEdit_NewQKeySequenceEdit for (&'a  QKeySequenceEdit) {
  fn NewQKeySequenceEdit(self) -> QKeySequenceEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEditC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QKeySequenceEditC1ERKS_(qthis, arg0)};
    let rsthis = QKeySequenceEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QKeySequenceEdit::NewQKeySequenceEdit(const QKeySequence & keySequence, QWidget * parent);
impl<'a> /*trait*/ QKeySequenceEdit_NewQKeySequenceEdit for (&'a  QKeySequence, &'a mut QWidget) {
  fn NewQKeySequenceEdit(self) -> QKeySequenceEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEditC1ERK12QKeySequenceP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN16QKeySequenceEditC1ERK12QKeySequenceP7QWidget(qthis, arg0, arg1)};
    let rsthis = QKeySequenceEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QKeySequenceEdit::clear();
impl /*struct*/ QKeySequenceEdit {
  pub fn clear<RetType, T: QKeySequenceEdit_clear<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_clear<RetType> {
  fn clear(self , rsthis: &mut QKeySequenceEdit) -> RetType;
}

// proto:  void QKeySequenceEdit::clear();
impl<'a> /*trait*/ QKeySequenceEdit_clear<()> for () {
  fn clear(self , rsthis: &mut QKeySequenceEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEdit5clearEv()};
     unsafe {_ZN16QKeySequenceEdit5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QKeySequenceEdit::setKeySequence(const QKeySequence & keySequence);
impl /*struct*/ QKeySequenceEdit {
  pub fn setKeySequence<RetType, T: QKeySequenceEdit_setKeySequence<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setKeySequence(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_setKeySequence<RetType> {
  fn setKeySequence(self , rsthis: &mut QKeySequenceEdit) -> RetType;
}

// proto:  void QKeySequenceEdit::setKeySequence(const QKeySequence & keySequence);
impl<'a> /*trait*/ QKeySequenceEdit_setKeySequence<()> for (&'a  QKeySequence) {
  fn setKeySequence(self , rsthis: &mut QKeySequenceEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEdit14setKeySequenceERK12QKeySequence()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QKeySequenceEdit14setKeySequenceERK12QKeySequence(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QKeySequence QKeySequenceEdit::keySequence();
impl /*struct*/ QKeySequenceEdit {
  pub fn keySequence<RetType, T: QKeySequenceEdit_keySequence<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.keySequence(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_keySequence<RetType> {
  fn keySequence(self , rsthis: &mut QKeySequenceEdit) -> RetType;
}

// proto:  QKeySequence QKeySequenceEdit::keySequence();
impl<'a> /*trait*/ QKeySequenceEdit_keySequence<QKeySequence> for () {
  fn keySequence(self , rsthis: &mut QKeySequenceEdit) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QKeySequenceEdit11keySequenceEv()};
    let mut ret = unsafe {_ZNK16QKeySequenceEdit11keySequenceEv(rsthis.qclsinst)};
    let mut ret1 = QKeySequence{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QKeySequenceEdit::FreeQKeySequenceEdit();
impl /*struct*/ QKeySequenceEdit {
  pub fn FreeQKeySequenceEdit<RetType, T: QKeySequenceEdit_FreeQKeySequenceEdit<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQKeySequenceEdit(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_FreeQKeySequenceEdit<RetType> {
  fn FreeQKeySequenceEdit(self , rsthis: &mut QKeySequenceEdit) -> RetType;
}

// proto:  void QKeySequenceEdit::FreeQKeySequenceEdit();
impl<'a> /*trait*/ QKeySequenceEdit_FreeQKeySequenceEdit<()> for () {
  fn FreeQKeySequenceEdit(self , rsthis: &mut QKeySequenceEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEditD0Ev()};
     unsafe {_ZN16QKeySequenceEditD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QKeySequenceEdit::keySequenceChanged(const QKeySequence & keySequence);
impl /*struct*/ QKeySequenceEdit {
  pub fn keySequenceChanged<RetType, T: QKeySequenceEdit_keySequenceChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.keySequenceChanged(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_keySequenceChanged<RetType> {
  fn keySequenceChanged(self , rsthis: &mut QKeySequenceEdit) -> RetType;
}

// proto:  void QKeySequenceEdit::keySequenceChanged(const QKeySequence & keySequence);
impl<'a> /*trait*/ QKeySequenceEdit_keySequenceChanged<()> for (&'a  QKeySequence) {
  fn keySequenceChanged(self , rsthis: &mut QKeySequenceEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEdit18keySequenceChangedERK12QKeySequence()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QKeySequenceEdit18keySequenceChangedERK12QKeySequence(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QKeySequenceEdit::editingFinished();
impl /*struct*/ QKeySequenceEdit {
  pub fn editingFinished<RetType, T: QKeySequenceEdit_editingFinished<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.editingFinished(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_editingFinished<RetType> {
  fn editingFinished(self , rsthis: &mut QKeySequenceEdit) -> RetType;
}

// proto:  void QKeySequenceEdit::editingFinished();
impl<'a> /*trait*/ QKeySequenceEdit_editingFinished<()> for () {
  fn editingFinished(self , rsthis: &mut QKeySequenceEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEdit15editingFinishedEv()};
     unsafe {_ZN16QKeySequenceEdit15editingFinishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QKeySequenceEdit::NewQKeySequenceEdit(QWidget * parent);
impl<'a> /*trait*/ QKeySequenceEdit_NewQKeySequenceEdit for (&'a mut QWidget) {
  fn NewQKeySequenceEdit(self) -> QKeySequenceEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEditC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QKeySequenceEditC1EP7QWidget(qthis, arg0)};
    let rsthis = QKeySequenceEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  const QMetaObject * QKeySequenceEdit::metaObject();
impl /*struct*/ QKeySequenceEdit {
  pub fn metaObject<RetType, T: QKeySequenceEdit_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QKeySequenceEdit_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QKeySequenceEdit) -> RetType;
}

// proto:  const QMetaObject * QKeySequenceEdit::metaObject();
impl<'a> /*trait*/ QKeySequenceEdit_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QKeySequenceEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QKeySequenceEdit10metaObjectEv()};
     unsafe {_ZNK16QKeySequenceEdit10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

