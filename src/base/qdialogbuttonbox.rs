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
  pub fn buttons<RetType, T: QDialogButtonBox_buttons<RetType>>(&mut self, value: T) -> RetType {
    return value.buttons(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_buttons<RetType> {
  fn buttons(self, rsthis: &mut QDialogButtonBox) -> RetType;
}

// proto:  QList<QAbstractButton *> QDialogButtonBox::buttons();
impl<'a> /*trait*/ QDialogButtonBox_buttons<()> for () {
  fn buttons(self, rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox7buttonsEv()};
     unsafe {_ZNK16QDialogButtonBox7buttonsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn setCenterButtons<RetType, T: QDialogButtonBox_setCenterButtons<RetType>>(&mut self, value: T) -> RetType {
    return value.setCenterButtons(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_setCenterButtons<RetType> {
  fn setCenterButtons(self, rsthis: &mut QDialogButtonBox) -> RetType;
}

// proto:  void QDialogButtonBox::setCenterButtons(bool center);
impl<'a> /*trait*/ QDialogButtonBox_setCenterButtons<()> for (i8) {
  fn setCenterButtons(self, rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox16setCenterButtonsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN16QDialogButtonBox16setCenterButtonsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn centerButtons<RetType, T: QDialogButtonBox_centerButtons<RetType>>(&mut self, value: T) -> RetType {
    return value.centerButtons(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_centerButtons<RetType> {
  fn centerButtons(self, rsthis: &mut QDialogButtonBox) -> RetType;
}

// proto:  bool QDialogButtonBox::centerButtons();
impl<'a> /*trait*/ QDialogButtonBox_centerButtons<i8> for () {
  fn centerButtons(self, rsthis: &mut QDialogButtonBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox13centerButtonsEv()};
    let mut ret = unsafe {_ZNK16QDialogButtonBox13centerButtonsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn metaObject<RetType, T: QDialogButtonBox_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QDialogButtonBox) -> RetType;
}

// proto:  const QMetaObject * QDialogButtonBox::metaObject();
impl<'a> /*trait*/ QDialogButtonBox_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox10metaObjectEv()};
     unsafe {_ZNK16QDialogButtonBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn FreeQDialogButtonBox<RetType, T: QDialogButtonBox_FreeQDialogButtonBox<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQDialogButtonBox(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_FreeQDialogButtonBox<RetType> {
  fn FreeQDialogButtonBox(self, rsthis: &mut QDialogButtonBox) -> RetType;
}

// proto:  void QDialogButtonBox::FreeQDialogButtonBox();
impl<'a> /*trait*/ QDialogButtonBox_FreeQDialogButtonBox<()> for () {
  fn FreeQDialogButtonBox(self, rsthis: &mut QDialogButtonBox) -> () {
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
  pub fn accepted<RetType, T: QDialogButtonBox_accepted<RetType>>(&mut self, value: T) -> RetType {
    return value.accepted(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_accepted<RetType> {
  fn accepted(self, rsthis: &mut QDialogButtonBox) -> RetType;
}

// proto:  void QDialogButtonBox::accepted();
impl<'a> /*trait*/ QDialogButtonBox_accepted<()> for () {
  fn accepted(self, rsthis: &mut QDialogButtonBox) -> () {
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
  pub fn clear<RetType, T: QDialogButtonBox_clear<RetType>>(&mut self, value: T) -> RetType {
    return value.clear(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_clear<RetType> {
  fn clear(self, rsthis: &mut QDialogButtonBox) -> RetType;
}

// proto:  void QDialogButtonBox::clear();
impl<'a> /*trait*/ QDialogButtonBox_clear<()> for () {
  fn clear(self, rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox5clearEv()};
     unsafe {_ZN16QDialogButtonBox5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn rejected<RetType, T: QDialogButtonBox_rejected<RetType>>(&mut self, value: T) -> RetType {
    return value.rejected(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_rejected<RetType> {
  fn rejected(self, rsthis: &mut QDialogButtonBox) -> RetType;
}

// proto:  void QDialogButtonBox::rejected();
impl<'a> /*trait*/ QDialogButtonBox_rejected<()> for () {
  fn rejected(self, rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox8rejectedEv()};
     unsafe {_ZN16QDialogButtonBox8rejectedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDialogButtonBox {
  pub fn helpRequested<RetType, T: QDialogButtonBox_helpRequested<RetType>>(&mut self, value: T) -> RetType {
    return value.helpRequested(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_helpRequested<RetType> {
  fn helpRequested(self, rsthis: &mut QDialogButtonBox) -> RetType;
}

// proto:  void QDialogButtonBox::helpRequested();
impl<'a> /*trait*/ QDialogButtonBox_helpRequested<()> for () {
  fn helpRequested(self, rsthis: &mut QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox13helpRequestedEv()};
     unsafe {_ZN16QDialogButtonBox13helpRequestedEv(rsthis.qclsinst)};
    // return 1;
  }
}

