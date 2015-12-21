// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtWidgets/qdialogbuttonbox.h
// dst-file: /src/widgets/qdialogbuttonbox.rs
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
use super::super::core::qstring::QString; // 771
use super::qwidget::QWidget; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QList<QAbstractButton *> QDialogButtonBox::buttons();
  fn _ZNK16QDialogButtonBox7buttonsEv(qthis: *mut c_void);
  // proto:  void QDialogButtonBox::setCenterButtons(bool center);
  fn _ZN16QDialogButtonBox16setCenterButtonsEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QDialogButtonBox::centerButtons();
  fn _ZNK16QDialogButtonBox13centerButtonsEv(qthis: *mut c_void) -> c_char;
  // proto:  const QMetaObject * QDialogButtonBox::metaObject();
  fn _ZNK16QDialogButtonBox10metaObjectEv(qthis: *mut c_void);
  // proto:  void QDialogButtonBox::~QDialogButtonBox();
  fn _ZN16QDialogButtonBoxD0Ev(qthis: *mut c_void);
  // proto:  void QDialogButtonBox::QDialogButtonBox(QWidget * parent);
  fn _ZN16QDialogButtonBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDialogButtonBox::accepted();
  fn _ZN16QDialogButtonBox8acceptedEv(qthis: *mut c_void);
  // proto:  void QDialogButtonBox::QDialogButtonBox(const QDialogButtonBox & );
  fn _ZN16QDialogButtonBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDialogButtonBox::clear();
  fn _ZN16QDialogButtonBox5clearEv(qthis: *mut c_void);
  // proto:  void QDialogButtonBox::rejected();
  fn _ZN16QDialogButtonBox8rejectedEv(qthis: *mut c_void);
  // proto:  void QDialogButtonBox::helpRequested();
  fn _ZN16QDialogButtonBox13helpRequestedEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QDialogButtonBox)=1
pub struct QDialogButtonBox {
  pub qclsinst: *mut c_void,
}

  // proto:  QList<QAbstractButton *> QDialogButtonBox::buttons();
impl /*struct*/ QDialogButtonBox {
  pub fn buttons<RetType, T: QDialogButtonBox_buttons<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.buttons(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_buttons<RetType> {
  fn buttons(self , rsthis: &mut QDialogButtonBox) -> RetType;
}

  // proto:  QList<QAbstractButton *> QDialogButtonBox::buttons();
impl<'a> /*trait*/ QDialogButtonBox_buttons<()> for () {
  fn buttons(self , rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox7buttonsEv()};
     unsafe {_ZNK16QDialogButtonBox7buttonsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::setCenterButtons(bool center);
impl /*struct*/ QDialogButtonBox {
  pub fn setCenterButtons<RetType, T: QDialogButtonBox_setCenterButtons<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCenterButtons(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_setCenterButtons<RetType> {
  fn setCenterButtons(self , rsthis: &mut QDialogButtonBox) -> RetType;
}

  // proto:  void QDialogButtonBox::setCenterButtons(bool center);
impl<'a> /*trait*/ QDialogButtonBox_setCenterButtons<()> for (i8) {
  fn setCenterButtons(self , rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox16setCenterButtonsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN16QDialogButtonBox16setCenterButtonsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QDialogButtonBox::centerButtons();
impl /*struct*/ QDialogButtonBox {
  pub fn centerButtons<RetType, T: QDialogButtonBox_centerButtons<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.centerButtons(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_centerButtons<RetType> {
  fn centerButtons(self , rsthis: &mut QDialogButtonBox) -> RetType;
}

  // proto:  bool QDialogButtonBox::centerButtons();
impl<'a> /*trait*/ QDialogButtonBox_centerButtons<i8> for () {
  fn centerButtons(self , rsthis: &mut QDialogButtonBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox13centerButtonsEv()};
    let mut ret = unsafe {_ZNK16QDialogButtonBox13centerButtonsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QDialogButtonBox::metaObject();
impl /*struct*/ QDialogButtonBox {
  pub fn metaObject<RetType, T: QDialogButtonBox_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QDialogButtonBox) -> RetType;
}

  // proto:  const QMetaObject * QDialogButtonBox::metaObject();
impl<'a> /*trait*/ QDialogButtonBox_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox10metaObjectEv()};
     unsafe {_ZNK16QDialogButtonBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::~QDialogButtonBox();
impl /*struct*/ QDialogButtonBox {
  pub fn FreeQDialogButtonBox<RetType, T: QDialogButtonBox_FreeQDialogButtonBox<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQDialogButtonBox(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_FreeQDialogButtonBox<RetType> {
  fn FreeQDialogButtonBox(self , rsthis: &mut QDialogButtonBox) -> RetType;
}

  // proto:  void QDialogButtonBox::~QDialogButtonBox();
impl<'a> /*trait*/ QDialogButtonBox_FreeQDialogButtonBox<()> for () {
  fn FreeQDialogButtonBox(self , rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBoxD0Ev()};
     unsafe {_ZN16QDialogButtonBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::QDialogButtonBox(QWidget * parent);
impl /*struct*/ QDialogButtonBox {
  pub fn NewQDialogButtonBox<T: QDialogButtonBox_NewQDialogButtonBox>(value: T) -> QDialogButtonBox {
    let rsthis = value.NewQDialogButtonBox();
    return rsthis;
    // return 1;
  }
}

pub trait QDialogButtonBox_NewQDialogButtonBox {
  fn NewQDialogButtonBox(self) -> QDialogButtonBox;
}

  // proto:  void QDialogButtonBox::QDialogButtonBox(QWidget * parent);
impl<'a> /*trait*/ QDialogButtonBox_NewQDialogButtonBox for (QWidget) {
  fn NewQDialogButtonBox(self) -> QDialogButtonBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBoxC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QDialogButtonBoxC1EP7QWidget(qthis, arg0)};
    let rsthis = QDialogButtonBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::accepted();
impl /*struct*/ QDialogButtonBox {
  pub fn accepted<RetType, T: QDialogButtonBox_accepted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.accepted(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_accepted<RetType> {
  fn accepted(self , rsthis: &mut QDialogButtonBox) -> RetType;
}

  // proto:  void QDialogButtonBox::accepted();
impl<'a> /*trait*/ QDialogButtonBox_accepted<()> for () {
  fn accepted(self , rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox8acceptedEv()};
     unsafe {_ZN16QDialogButtonBox8acceptedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::QDialogButtonBox(const QDialogButtonBox & );
impl<'a> /*trait*/ QDialogButtonBox_NewQDialogButtonBox for (QDialogButtonBox) {
  fn NewQDialogButtonBox(self) -> QDialogButtonBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QDialogButtonBoxC1ERKS_(qthis, arg0)};
    let rsthis = QDialogButtonBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::clear();
impl /*struct*/ QDialogButtonBox {
  pub fn clear<RetType, T: QDialogButtonBox_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_clear<RetType> {
  fn clear(self , rsthis: &mut QDialogButtonBox) -> RetType;
}

  // proto:  void QDialogButtonBox::clear();
impl<'a> /*trait*/ QDialogButtonBox_clear<()> for () {
  fn clear(self , rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox5clearEv()};
     unsafe {_ZN16QDialogButtonBox5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::rejected();
impl /*struct*/ QDialogButtonBox {
  pub fn rejected<RetType, T: QDialogButtonBox_rejected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rejected(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_rejected<RetType> {
  fn rejected(self , rsthis: &mut QDialogButtonBox) -> RetType;
}

  // proto:  void QDialogButtonBox::rejected();
impl<'a> /*trait*/ QDialogButtonBox_rejected<()> for () {
  fn rejected(self , rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox8rejectedEv()};
     unsafe {_ZN16QDialogButtonBox8rejectedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::helpRequested();
impl /*struct*/ QDialogButtonBox {
  pub fn helpRequested<RetType, T: QDialogButtonBox_helpRequested<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.helpRequested(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_helpRequested<RetType> {
  fn helpRequested(self , rsthis: &mut QDialogButtonBox) -> RetType;
}

  // proto:  void QDialogButtonBox::helpRequested();
impl<'a> /*trait*/ QDialogButtonBox_helpRequested<()> for () {
  fn helpRequested(self , rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox13helpRequestedEv()};
     unsafe {_ZN16QDialogButtonBox13helpRequestedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

