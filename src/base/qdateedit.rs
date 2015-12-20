// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qdate::QDate;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QDateEdit::QDateEdit(const QDate & date, QWidget * parent);
  fn _ZN9QDateEditC1ERK5QDateP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QDateEdit::userDateChanged(const QDate & date);
  fn _ZN9QDateEdit15userDateChangedERK5QDate(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QDateEdit::metaObject();
  fn _ZNK9QDateEdit10metaObjectEv(qthis: *mut c_void);
  // proto:  void QDateEdit::QDateEdit(QWidget * parent);
  fn _ZN9QDateEditC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDateEdit::~QDateEdit();
  fn _ZN9QDateEditD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QDateEdit)=1
pub struct QDateEdit {
  pub qclsinst: *mut c_void,
}

  // proto:  void QDateEdit::QDateEdit(const QDate & date, QWidget * parent);
impl /*struct*/ QDateEdit {
  pub fn NewQDateEdit<T: QDateEdit_NewQDateEdit>(value: T) -> QDateEdit {
    let rsthis = value.NewQDateEdit();
    return rsthis;
    // return 1;
  }
}

pub trait QDateEdit_NewQDateEdit {
  fn NewQDateEdit(self) -> QDateEdit;
}

  // proto:  void QDateEdit::QDateEdit(const QDate & date, QWidget * parent);
impl<'a> /*trait*/ QDateEdit_NewQDateEdit for (QDate, QWidget) {
  fn NewQDateEdit(self) -> QDateEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateEditC1ERK5QDateP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QDateEditC1ERK5QDateP7QWidget(qthis, arg0, arg1)};
    let rsthis = QDateEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDateEdit::userDateChanged(const QDate & date);
impl /*struct*/ QDateEdit {
  pub fn userDateChanged<RetType, T: QDateEdit_userDateChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.userDateChanged(self);
    // return 1;
  }
}

pub trait QDateEdit_userDateChanged<RetType> {
  fn userDateChanged(self , rsthis: &mut QDateEdit) -> RetType;
}

  // proto:  void QDateEdit::userDateChanged(const QDate & date);
impl<'a> /*trait*/ QDateEdit_userDateChanged<()> for (QDate) {
  fn userDateChanged(self , rsthis: &mut QDateEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateEdit15userDateChangedERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDateEdit15userDateChangedERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QDateEdit::metaObject();
impl /*struct*/ QDateEdit {
  pub fn metaObject<RetType, T: QDateEdit_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDateEdit_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QDateEdit) -> RetType;
}

  // proto:  const QMetaObject * QDateEdit::metaObject();
impl<'a> /*trait*/ QDateEdit_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QDateEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateEdit10metaObjectEv()};
     unsafe {_ZNK9QDateEdit10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDateEdit::QDateEdit(QWidget * parent);
impl<'a> /*trait*/ QDateEdit_NewQDateEdit for (QWidget) {
  fn NewQDateEdit(self) -> QDateEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateEditC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QDateEditC1EP7QWidget(qthis, arg0)};
    let rsthis = QDateEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDateEdit::~QDateEdit();
impl /*struct*/ QDateEdit {
  pub fn FreeQDateEdit<RetType, T: QDateEdit_FreeQDateEdit<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQDateEdit(self);
    // return 1;
  }
}

pub trait QDateEdit_FreeQDateEdit<RetType> {
  fn FreeQDateEdit(self , rsthis: &mut QDateEdit) -> RetType;
}

  // proto:  void QDateEdit::~QDateEdit();
impl<'a> /*trait*/ QDateEdit_FreeQDateEdit<()> for () {
  fn FreeQDateEdit(self , rsthis: &mut QDateEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateEditD0Ev()};
     unsafe {_ZN9QDateEditD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

