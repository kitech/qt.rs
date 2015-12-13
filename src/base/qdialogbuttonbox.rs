// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QList<QAbstractButton *> QDialogButtonBox::buttons();
  fn _ZNK16QDialogButtonBox7buttonsEv(qthis: *mut c_void) ;
  // proto:  void QDialogButtonBox::setCenterButtons(bool center);
  fn _ZN16QDialogButtonBox16setCenterButtonsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QDialogButtonBox::centerButtons();
  fn _ZNK16QDialogButtonBox13centerButtonsEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QDialogButtonBox::metaObject();
  fn _ZNK16QDialogButtonBox10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QDialogButtonBox::FreeQDialogButtonBox();
  fn _ZN16QDialogButtonBoxD0Ev(qthis: *mut c_void) ;
  // proto:  void QDialogButtonBox::NewQDialogButtonBox(QWidget * parent);
  fn _ZN16QDialogButtonBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDialogButtonBox::accepted();
  fn _ZN16QDialogButtonBox8acceptedEv(qthis: *mut c_void) ;
  // proto:  void QDialogButtonBox::NewQDialogButtonBox(const QDialogButtonBox & );
  fn _ZN16QDialogButtonBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDialogButtonBox::clear();
  fn _ZN16QDialogButtonBox5clearEv(qthis: *mut c_void) ;
  // proto:  void QDialogButtonBox::rejected();
  fn _ZN16QDialogButtonBox8rejectedEv(qthis: *mut c_void) ;
  // proto:  void QDialogButtonBox::helpRequested();
  fn _ZN16QDialogButtonBox13helpRequestedEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QDialogButtonBox)=1
pub struct QDialogButtonBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDialogButtonBox {
  pub fn buttons<T: QDialogButtonBox_buttons>(&mut self, value: T)  {
     value.buttons(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_buttons {
  fn buttons(self, rsthis: &mut QDialogButtonBox) ;
}

// proto:  QList<QAbstractButton *> QDialogButtonBox::buttons();
impl<'a> /*trait*/ QDialogButtonBox_buttons for () {
  fn buttons(self, rsthis: &mut QDialogButtonBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox7buttonsEv()};
     unsafe {_ZNK16QDialogButtonBox7buttonsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn setCenterButtons<T: QDialogButtonBox_setCenterButtons>(&mut self, value: T)  {
     value.setCenterButtons(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_setCenterButtons {
  fn setCenterButtons(self, rsthis: &mut QDialogButtonBox) ;
}

// proto:  void QDialogButtonBox::setCenterButtons(bool center);
impl<'a> /*trait*/ QDialogButtonBox_setCenterButtons for (i8) {
  fn setCenterButtons(self, rsthis: &mut QDialogButtonBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox16setCenterButtonsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN16QDialogButtonBox16setCenterButtonsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn centerButtons<T: QDialogButtonBox_centerButtons>(&mut self, value: T) -> i8 {
    return value.centerButtons(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_centerButtons {
  fn centerButtons(self, rsthis: &mut QDialogButtonBox) -> i8;
}

// proto:  bool QDialogButtonBox::centerButtons();
impl<'a> /*trait*/ QDialogButtonBox_centerButtons for () {
  fn centerButtons(self, rsthis: &mut QDialogButtonBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox13centerButtonsEv()};
    let mut ret = unsafe {_ZNK16QDialogButtonBox13centerButtonsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn metaObject<T: QDialogButtonBox_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_metaObject {
  fn metaObject(self, rsthis: &mut QDialogButtonBox) ;
}

// proto:  const QMetaObject * QDialogButtonBox::metaObject();
impl<'a> /*trait*/ QDialogButtonBox_metaObject for () {
  fn metaObject(self, rsthis: &mut QDialogButtonBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox10metaObjectEv()};
     unsafe {_ZNK16QDialogButtonBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn FreeQDialogButtonBox<T: QDialogButtonBox_FreeQDialogButtonBox>(&mut self, value: T)  {
     value.FreeQDialogButtonBox(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_FreeQDialogButtonBox {
  fn FreeQDialogButtonBox(self, rsthis: &mut QDialogButtonBox) ;
}

// proto:  void QDialogButtonBox::FreeQDialogButtonBox();
impl<'a> /*trait*/ QDialogButtonBox_FreeQDialogButtonBox for () {
  fn FreeQDialogButtonBox(self, rsthis: &mut QDialogButtonBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBoxD0Ev()};
     unsafe {_ZN16QDialogButtonBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

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

// proto: void QDialogButtonBox::NewQDialogButtonBox(QWidget * parent);
impl<'a> /*trait*/ QDialogButtonBox_NewQDialogButtonBox for (&'a mut QWidget) {
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

impl /*struct*/ QDialogButtonBox {
  pub fn accepted<T: QDialogButtonBox_accepted>(&mut self, value: T)  {
     value.accepted(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_accepted {
  fn accepted(self, rsthis: &mut QDialogButtonBox) ;
}

// proto:  void QDialogButtonBox::accepted();
impl<'a> /*trait*/ QDialogButtonBox_accepted for () {
  fn accepted(self, rsthis: &mut QDialogButtonBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox8acceptedEv()};
     unsafe {_ZN16QDialogButtonBox8acceptedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QDialogButtonBox::NewQDialogButtonBox(const QDialogButtonBox & );
impl<'a> /*trait*/ QDialogButtonBox_NewQDialogButtonBox for (&'a  QDialogButtonBox) {
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

impl /*struct*/ QDialogButtonBox {
  pub fn clear<T: QDialogButtonBox_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_clear {
  fn clear(self, rsthis: &mut QDialogButtonBox) ;
}

// proto:  void QDialogButtonBox::clear();
impl<'a> /*trait*/ QDialogButtonBox_clear for () {
  fn clear(self, rsthis: &mut QDialogButtonBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox5clearEv()};
     unsafe {_ZN16QDialogButtonBox5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn rejected<T: QDialogButtonBox_rejected>(&mut self, value: T)  {
     value.rejected(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_rejected {
  fn rejected(self, rsthis: &mut QDialogButtonBox) ;
}

// proto:  void QDialogButtonBox::rejected();
impl<'a> /*trait*/ QDialogButtonBox_rejected for () {
  fn rejected(self, rsthis: &mut QDialogButtonBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox8rejectedEv()};
     unsafe {_ZN16QDialogButtonBox8rejectedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn helpRequested<T: QDialogButtonBox_helpRequested>(&mut self, value: T)  {
     value.helpRequested(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_helpRequested {
  fn helpRequested(self, rsthis: &mut QDialogButtonBox) ;
}

// proto:  void QDialogButtonBox::helpRequested();
impl<'a> /*trait*/ QDialogButtonBox_helpRequested for () {
  fn helpRequested(self, rsthis: &mut QDialogButtonBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox13helpRequestedEv()};
     unsafe {_ZN16QDialogButtonBox13helpRequestedEv(rsthis.qclsinst)};
    // return 1;
  }
}

