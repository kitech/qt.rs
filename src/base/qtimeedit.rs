// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qtime::QTime;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTimeEdit::NewQTimeEdit(QWidget * parent);
  fn _ZN9QTimeEditC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTimeEdit::userTimeChanged(const QTime & time);
  fn _ZN9QTimeEdit15userTimeChangedERK5QTime(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QTimeEdit::metaObject();
  fn _ZNK9QTimeEdit10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTimeEdit::NewQTimeEdit(const QTime & time, QWidget * parent);
  fn _ZN9QTimeEditC1ERK5QTimeP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QTimeEdit::FreeQTimeEdit();
  fn _ZN9QTimeEditD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QTimeEdit)=1
pub struct QTimeEdit {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTimeEdit {
  pub fn NewQTimeEdit<T: QTimeEdit_NewQTimeEdit>(value: T) -> QTimeEdit {
    let rsthis = value.NewQTimeEdit();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeEdit_NewQTimeEdit {
  fn NewQTimeEdit(self) -> QTimeEdit;
}

// proto: void QTimeEdit::NewQTimeEdit(QWidget * parent);
impl<'a> /*trait*/ QTimeEdit_NewQTimeEdit for (&'a mut QWidget) {
  fn NewQTimeEdit(self) -> QTimeEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeEditC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTimeEditC1EP7QWidget(qthis, arg0)};
    let rsthis = QTimeEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QTimeEdit::userTimeChanged(const QTime & time);
impl /*struct*/ QTimeEdit {
  pub fn userTimeChanged<RetType, T: QTimeEdit_userTimeChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.userTimeChanged(self);
    // return 1;
  }
}

pub trait QTimeEdit_userTimeChanged<RetType> {
  fn userTimeChanged(self , rsthis: &mut QTimeEdit) -> RetType;
}

// proto:  void QTimeEdit::userTimeChanged(const QTime & time);
impl<'a> /*trait*/ QTimeEdit_userTimeChanged<()> for (&'a  QTime) {
  fn userTimeChanged(self , rsthis: &mut QTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeEdit15userTimeChangedERK5QTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTimeEdit15userTimeChangedERK5QTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  const QMetaObject * QTimeEdit::metaObject();
impl /*struct*/ QTimeEdit {
  pub fn metaObject<RetType, T: QTimeEdit_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTimeEdit_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QTimeEdit) -> RetType;
}

// proto:  const QMetaObject * QTimeEdit::metaObject();
impl<'a> /*trait*/ QTimeEdit_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeEdit10metaObjectEv()};
     unsafe {_ZNK9QTimeEdit10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QTimeEdit::NewQTimeEdit(const QTime & time, QWidget * parent);
impl<'a> /*trait*/ QTimeEdit_NewQTimeEdit for (&'a  QTime, &'a mut QWidget) {
  fn NewQTimeEdit(self) -> QTimeEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeEditC1ERK5QTimeP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QTimeEditC1ERK5QTimeP7QWidget(qthis, arg0, arg1)};
    let rsthis = QTimeEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QTimeEdit::FreeQTimeEdit();
impl /*struct*/ QTimeEdit {
  pub fn FreeQTimeEdit<RetType, T: QTimeEdit_FreeQTimeEdit<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQTimeEdit(self);
    // return 1;
  }
}

pub trait QTimeEdit_FreeQTimeEdit<RetType> {
  fn FreeQTimeEdit(self , rsthis: &mut QTimeEdit) -> RetType;
}

// proto:  void QTimeEdit::FreeQTimeEdit();
impl<'a> /*trait*/ QTimeEdit_FreeQTimeEdit<()> for () {
  fn FreeQTimeEdit(self , rsthis: &mut QTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeEditD0Ev()};
     unsafe {_ZN9QTimeEditD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

