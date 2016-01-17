// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
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
use super::qwidget::QWidget; // 773
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::qabstractbutton::QAbstractButton; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDialogButtonBox_Class_Size() -> c_int;
  // proto:  QList<QAbstractButton *> QDialogButtonBox::buttons();
  fn _ZNK16QDialogButtonBox7buttonsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDialogButtonBox::setCenterButtons(bool center);
  fn _ZN16QDialogButtonBox16setCenterButtonsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QDialogButtonBox::centerButtons();
  fn _ZNK16QDialogButtonBox13centerButtonsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QDialogButtonBox::metaObject();
  fn _ZNK16QDialogButtonBox10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDialogButtonBox::removeButton(QAbstractButton * button);
  fn _ZN16QDialogButtonBox12removeButtonEP15QAbstractButton(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDialogButtonBox::~QDialogButtonBox();
  fn _ZN16QDialogButtonBoxD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDialogButtonBox::QDialogButtonBox(QWidget * parent);
  fn _ZN16QDialogButtonBoxC2EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDialogButtonBox::QDialogButtonBox(const QDialogButtonBox & );
  fn _ZN16QDialogButtonBoxC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDialogButtonBox::clear();
  fn _ZN16QDialogButtonBox5clearEv(qthis: u64 /* *mut c_void*/);
  fn QDialogButtonBox_SlotProxy_connect__ZN16QDialogButtonBox8acceptedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDialogButtonBox_SlotProxy_connect__ZN16QDialogButtonBox13helpRequestedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDialogButtonBox_SlotProxy_connect__ZN16QDialogButtonBox7clickedEP15QAbstractButton(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDialogButtonBox_SlotProxy_connect__ZN16QDialogButtonBox8rejectedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QDialogButtonBox)=1
#[derive(Default)]
pub struct QDialogButtonBox {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _helpRequested: QDialogButtonBox_helpRequested_signal,
  pub _accepted: QDialogButtonBox_accepted_signal,
  pub _clicked: QDialogButtonBox_clicked_signal,
  pub _rejected: QDialogButtonBox_rejected_signal,
}

impl /*struct*/ QDialogButtonBox {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDialogButtonBox {
    return QDialogButtonBox{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDialogButtonBox {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QDialogButtonBox {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  QList<QAbstractButton *> QDialogButtonBox::buttons();
impl /*struct*/ QDialogButtonBox {
  pub fn buttons<RetType, T: QDialogButtonBox_buttons<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.buttons(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_buttons<RetType> {
  fn buttons(self , rsthis: & QDialogButtonBox) -> RetType;
}

  // proto:  QList<QAbstractButton *> QDialogButtonBox::buttons();
impl<'a> /*trait*/ QDialogButtonBox_buttons<()> for () {
  fn buttons(self , rsthis: & QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox7buttonsEv()};
     unsafe {_ZNK16QDialogButtonBox7buttonsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::setCenterButtons(bool center);
impl /*struct*/ QDialogButtonBox {
  pub fn setCenterButtons<RetType, T: QDialogButtonBox_setCenterButtons<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCenterButtons(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_setCenterButtons<RetType> {
  fn setCenterButtons(self , rsthis: & QDialogButtonBox) -> RetType;
}

  // proto:  void QDialogButtonBox::setCenterButtons(bool center);
impl<'a> /*trait*/ QDialogButtonBox_setCenterButtons<()> for (i8) {
  fn setCenterButtons(self , rsthis: & QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox16setCenterButtonsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN16QDialogButtonBox16setCenterButtonsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QDialogButtonBox::centerButtons();
impl /*struct*/ QDialogButtonBox {
  pub fn centerButtons<RetType, T: QDialogButtonBox_centerButtons<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.centerButtons(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_centerButtons<RetType> {
  fn centerButtons(self , rsthis: & QDialogButtonBox) -> RetType;
}

  // proto:  bool QDialogButtonBox::centerButtons();
impl<'a> /*trait*/ QDialogButtonBox_centerButtons<i8> for () {
  fn centerButtons(self , rsthis: & QDialogButtonBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox13centerButtonsEv()};
    let mut ret = unsafe {_ZNK16QDialogButtonBox13centerButtonsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QDialogButtonBox::metaObject();
impl /*struct*/ QDialogButtonBox {
  pub fn metaObject<RetType, T: QDialogButtonBox_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_metaObject<RetType> {
  fn metaObject(self , rsthis: & QDialogButtonBox) -> RetType;
}

  // proto:  const QMetaObject * QDialogButtonBox::metaObject();
impl<'a> /*trait*/ QDialogButtonBox_metaObject<()> for () {
  fn metaObject(self , rsthis: & QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QDialogButtonBox10metaObjectEv()};
     unsafe {_ZNK16QDialogButtonBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::removeButton(QAbstractButton * button);
impl /*struct*/ QDialogButtonBox {
  pub fn removeButton<RetType, T: QDialogButtonBox_removeButton<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeButton(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_removeButton<RetType> {
  fn removeButton(self , rsthis: & QDialogButtonBox) -> RetType;
}

  // proto:  void QDialogButtonBox::removeButton(QAbstractButton * button);
impl<'a> /*trait*/ QDialogButtonBox_removeButton<()> for (&'a QAbstractButton) {
  fn removeButton(self , rsthis: & QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox12removeButtonEP15QAbstractButton()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QDialogButtonBox12removeButtonEP15QAbstractButton(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::~QDialogButtonBox();
impl /*struct*/ QDialogButtonBox {
  pub fn free<RetType, T: QDialogButtonBox_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_free<RetType> {
  fn free(self , rsthis: & QDialogButtonBox) -> RetType;
}

  // proto:  void QDialogButtonBox::~QDialogButtonBox();
impl<'a> /*trait*/ QDialogButtonBox_free<()> for () {
  fn free(self , rsthis: & QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBoxD2Ev()};
     unsafe {_ZN16QDialogButtonBoxD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::QDialogButtonBox(QWidget * parent);
impl /*struct*/ QDialogButtonBox {
  pub fn new<T: QDialogButtonBox_new>(value: T) -> QDialogButtonBox {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDialogButtonBox_new {
  fn new(self) -> QDialogButtonBox;
}

  // proto:  void QDialogButtonBox::QDialogButtonBox(QWidget * parent);
impl<'a> /*trait*/ QDialogButtonBox_new for (&'a QWidget) {
  fn new(self) -> QDialogButtonBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBoxC2EP7QWidget()};
    let ctysz: c_int = unsafe{QDialogButtonBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QDialogButtonBoxC2EP7QWidget(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QDialogButtonBox{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::QDialogButtonBox(const QDialogButtonBox & );
impl<'a> /*trait*/ QDialogButtonBox_new for (&'a QDialogButtonBox) {
  fn new(self) -> QDialogButtonBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBoxC2ERKS_()};
    let ctysz: c_int = unsafe{QDialogButtonBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QDialogButtonBoxC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QDialogButtonBox{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDialogButtonBox::clear();
impl /*struct*/ QDialogButtonBox {
  pub fn clear<RetType, T: QDialogButtonBox_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QDialogButtonBox_clear<RetType> {
  fn clear(self , rsthis: & QDialogButtonBox) -> RetType;
}

  // proto:  void QDialogButtonBox::clear();
impl<'a> /*trait*/ QDialogButtonBox_clear<()> for () {
  fn clear(self , rsthis: & QDialogButtonBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDialogButtonBox5clearEv()};
     unsafe {_ZN16QDialogButtonBox5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QDialogButtonBox_helpRequested
pub struct QDialogButtonBox_helpRequested_signal{poi:u64}
impl /* struct */ QDialogButtonBox {
  pub fn helpRequested(&self) -> QDialogButtonBox_helpRequested_signal {
     return QDialogButtonBox_helpRequested_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDialogButtonBox_helpRequested_signal {
  pub fn connect<T: QDialogButtonBox_helpRequested_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDialogButtonBox_helpRequested_signal_connect {
  fn connect(self, sigthis: QDialogButtonBox_helpRequested_signal);
}

#[derive(Default)] // for QDialogButtonBox_accepted
pub struct QDialogButtonBox_accepted_signal{poi:u64}
impl /* struct */ QDialogButtonBox {
  pub fn accepted(&self) -> QDialogButtonBox_accepted_signal {
     return QDialogButtonBox_accepted_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDialogButtonBox_accepted_signal {
  pub fn connect<T: QDialogButtonBox_accepted_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDialogButtonBox_accepted_signal_connect {
  fn connect(self, sigthis: QDialogButtonBox_accepted_signal);
}

#[derive(Default)] // for QDialogButtonBox_clicked
pub struct QDialogButtonBox_clicked_signal{poi:u64}
impl /* struct */ QDialogButtonBox {
  pub fn clicked(&self) -> QDialogButtonBox_clicked_signal {
     return QDialogButtonBox_clicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDialogButtonBox_clicked_signal {
  pub fn connect<T: QDialogButtonBox_clicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDialogButtonBox_clicked_signal_connect {
  fn connect(self, sigthis: QDialogButtonBox_clicked_signal);
}

#[derive(Default)] // for QDialogButtonBox_rejected
pub struct QDialogButtonBox_rejected_signal{poi:u64}
impl /* struct */ QDialogButtonBox {
  pub fn rejected(&self) -> QDialogButtonBox_rejected_signal {
     return QDialogButtonBox_rejected_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDialogButtonBox_rejected_signal {
  pub fn connect<T: QDialogButtonBox_rejected_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDialogButtonBox_rejected_signal_connect {
  fn connect(self, sigthis: QDialogButtonBox_rejected_signal);
}

// accepted()
extern fn QDialogButtonBox_accepted_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QDialogButtonBox_accepted_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QDialogButtonBox_accepted_signal_connect for fn() {
  fn connect(self, sigthis: QDialogButtonBox_accepted_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialogButtonBox_accepted_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDialogButtonBox_SlotProxy_connect__ZN16QDialogButtonBox8acceptedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDialogButtonBox_accepted_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QDialogButtonBox_accepted_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialogButtonBox_accepted_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDialogButtonBox_SlotProxy_connect__ZN16QDialogButtonBox8acceptedEv(arg0, arg1, arg2)};
  }
}
// helpRequested()
extern fn QDialogButtonBox_helpRequested_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QDialogButtonBox_helpRequested_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QDialogButtonBox_helpRequested_signal_connect for fn() {
  fn connect(self, sigthis: QDialogButtonBox_helpRequested_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialogButtonBox_helpRequested_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDialogButtonBox_SlotProxy_connect__ZN16QDialogButtonBox13helpRequestedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDialogButtonBox_helpRequested_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QDialogButtonBox_helpRequested_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialogButtonBox_helpRequested_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDialogButtonBox_SlotProxy_connect__ZN16QDialogButtonBox13helpRequestedEv(arg0, arg1, arg2)};
  }
}
// clicked(class QAbstractButton *)
extern fn QDialogButtonBox_clicked_signal_connect_cb_2(rsfptr:fn(QAbstractButton), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QAbstractButton::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QDialogButtonBox_clicked_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QAbstractButton)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QAbstractButton::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDialogButtonBox_clicked_signal_connect for fn(QAbstractButton) {
  fn connect(self, sigthis: QDialogButtonBox_clicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialogButtonBox_clicked_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDialogButtonBox_SlotProxy_connect__ZN16QDialogButtonBox7clickedEP15QAbstractButton(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDialogButtonBox_clicked_signal_connect for Box<Fn(QAbstractButton)> {
  fn connect(self, sigthis: QDialogButtonBox_clicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialogButtonBox_clicked_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDialogButtonBox_SlotProxy_connect__ZN16QDialogButtonBox7clickedEP15QAbstractButton(arg0, arg1, arg2)};
  }
}
// rejected()
extern fn QDialogButtonBox_rejected_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QDialogButtonBox_rejected_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QDialogButtonBox_rejected_signal_connect for fn() {
  fn connect(self, sigthis: QDialogButtonBox_rejected_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialogButtonBox_rejected_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDialogButtonBox_SlotProxy_connect__ZN16QDialogButtonBox8rejectedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDialogButtonBox_rejected_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QDialogButtonBox_rejected_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDialogButtonBox_rejected_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDialogButtonBox_SlotProxy_connect__ZN16QDialogButtonBox8rejectedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

