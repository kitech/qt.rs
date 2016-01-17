// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtWidgets/qbuttongroup.h
// dst-file: /src/widgets/qbuttongroup.rs
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
use super::qabstractbutton::QAbstractButton; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QButtonGroup_Class_Size() -> c_int;
  // proto:  void QButtonGroup::addButton(QAbstractButton * , int id);
  fn _ZN12QButtonGroup9addButtonEP15QAbstractButtoni(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  QList<QAbstractButton *> QButtonGroup::buttons();
  fn _ZNK12QButtonGroup7buttonsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QButtonGroup::~QButtonGroup();
  fn _ZN12QButtonGroupD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QButtonGroup::id(QAbstractButton * button);
  fn _ZNK12QButtonGroup2idEP15QAbstractButton(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QButtonGroup::removeButton(QAbstractButton * );
  fn _ZN12QButtonGroup12removeButtonEP15QAbstractButton(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QButtonGroup::QButtonGroup(const QButtonGroup & );
  fn _ZN12QButtonGroupC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QButtonGroup::metaObject();
  fn _ZNK12QButtonGroup10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QButtonGroup::QButtonGroup(QObject * parent);
  fn _ZN12QButtonGroupC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QAbstractButton * QButtonGroup::button(int id);
  fn _ZNK12QButtonGroup6buttonEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QButtonGroup::checkedId();
  fn _ZNK12QButtonGroup9checkedIdEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QAbstractButton * QButtonGroup::checkedButton();
  fn _ZNK12QButtonGroup13checkedButtonEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QButtonGroup::setExclusive(bool );
  fn _ZN12QButtonGroup12setExclusiveEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QButtonGroup::setId(QAbstractButton * button, int id);
  fn _ZN12QButtonGroup5setIdEP15QAbstractButtoni(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  bool QButtonGroup::exclusive();
  fn _ZNK12QButtonGroup9exclusiveEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QButtonGroup_SlotProxy_connect__ZN12QButtonGroup14buttonReleasedEP15QAbstractButton(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonPressedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonToggledEib(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonPressedEP15QAbstractButton(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QButtonGroup_SlotProxy_connect__ZN12QButtonGroup14buttonReleasedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonToggledEP15QAbstractButtonb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonClickedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonClickedEP15QAbstractButton(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QButtonGroup)=1
#[derive(Default)]
pub struct QButtonGroup {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _buttonToggled: QButtonGroup_buttonToggled_signal,
  pub _buttonClicked: QButtonGroup_buttonClicked_signal,
  pub _buttonReleased: QButtonGroup_buttonReleased_signal,
  pub _buttonPressed: QButtonGroup_buttonPressed_signal,
}

impl /*struct*/ QButtonGroup {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QButtonGroup {
    return QButtonGroup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QButtonGroup {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QButtonGroup {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QButtonGroup::addButton(QAbstractButton * , int id);
impl /*struct*/ QButtonGroup {
  pub fn addButton<RetType, T: QButtonGroup_addButton<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addButton(self);
    // return 1;
  }
}

pub trait QButtonGroup_addButton<RetType> {
  fn addButton(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  void QButtonGroup::addButton(QAbstractButton * , int id);
impl<'a> /*trait*/ QButtonGroup_addButton<()> for (&'a QAbstractButton, i32) {
  fn addButton(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup9addButtonEP15QAbstractButtoni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QButtonGroup9addButtonEP15QAbstractButtoni(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QList<QAbstractButton *> QButtonGroup::buttons();
impl /*struct*/ QButtonGroup {
  pub fn buttons<RetType, T: QButtonGroup_buttons<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.buttons(self);
    // return 1;
  }
}

pub trait QButtonGroup_buttons<RetType> {
  fn buttons(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  QList<QAbstractButton *> QButtonGroup::buttons();
impl<'a> /*trait*/ QButtonGroup_buttons<()> for () {
  fn buttons(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup7buttonsEv()};
     unsafe {_ZNK12QButtonGroup7buttonsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QButtonGroup::~QButtonGroup();
impl /*struct*/ QButtonGroup {
  pub fn free<RetType, T: QButtonGroup_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QButtonGroup_free<RetType> {
  fn free(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  void QButtonGroup::~QButtonGroup();
impl<'a> /*trait*/ QButtonGroup_free<()> for () {
  fn free(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroupD2Ev()};
     unsafe {_ZN12QButtonGroupD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QButtonGroup::id(QAbstractButton * button);
impl /*struct*/ QButtonGroup {
  pub fn id<RetType, T: QButtonGroup_id<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.id(self);
    // return 1;
  }
}

pub trait QButtonGroup_id<RetType> {
  fn id(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  int QButtonGroup::id(QAbstractButton * button);
impl<'a> /*trait*/ QButtonGroup_id<i32> for (&'a QAbstractButton) {
  fn id(self , rsthis: & QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup2idEP15QAbstractButton()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QButtonGroup2idEP15QAbstractButton(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QButtonGroup::removeButton(QAbstractButton * );
impl /*struct*/ QButtonGroup {
  pub fn removeButton<RetType, T: QButtonGroup_removeButton<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeButton(self);
    // return 1;
  }
}

pub trait QButtonGroup_removeButton<RetType> {
  fn removeButton(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  void QButtonGroup::removeButton(QAbstractButton * );
impl<'a> /*trait*/ QButtonGroup_removeButton<()> for (&'a QAbstractButton) {
  fn removeButton(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup12removeButtonEP15QAbstractButton()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QButtonGroup12removeButtonEP15QAbstractButton(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QButtonGroup::QButtonGroup(const QButtonGroup & );
impl /*struct*/ QButtonGroup {
  pub fn new<T: QButtonGroup_new>(value: T) -> QButtonGroup {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QButtonGroup_new {
  fn new(self) -> QButtonGroup;
}

  // proto:  void QButtonGroup::QButtonGroup(const QButtonGroup & );
impl<'a> /*trait*/ QButtonGroup_new for (&'a QButtonGroup) {
  fn new(self) -> QButtonGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroupC2ERKS_()};
    let ctysz: c_int = unsafe{QButtonGroup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QButtonGroupC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QButtonGroup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QButtonGroup::metaObject();
impl /*struct*/ QButtonGroup {
  pub fn metaObject<RetType, T: QButtonGroup_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QButtonGroup_metaObject<RetType> {
  fn metaObject(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  const QMetaObject * QButtonGroup::metaObject();
impl<'a> /*trait*/ QButtonGroup_metaObject<()> for () {
  fn metaObject(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup10metaObjectEv()};
     unsafe {_ZNK12QButtonGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QButtonGroup::QButtonGroup(QObject * parent);
impl<'a> /*trait*/ QButtonGroup_new for (&'a QObject) {
  fn new(self) -> QButtonGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroupC2EP7QObject()};
    let ctysz: c_int = unsafe{QButtonGroup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QButtonGroupC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QButtonGroup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QAbstractButton * QButtonGroup::button(int id);
impl /*struct*/ QButtonGroup {
  pub fn button<RetType, T: QButtonGroup_button<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.button(self);
    // return 1;
  }
}

pub trait QButtonGroup_button<RetType> {
  fn button(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  QAbstractButton * QButtonGroup::button(int id);
impl<'a> /*trait*/ QButtonGroup_button<()> for (i32) {
  fn button(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup6buttonEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK12QButtonGroup6buttonEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QButtonGroup::checkedId();
impl /*struct*/ QButtonGroup {
  pub fn checkedId<RetType, T: QButtonGroup_checkedId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.checkedId(self);
    // return 1;
  }
}

pub trait QButtonGroup_checkedId<RetType> {
  fn checkedId(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  int QButtonGroup::checkedId();
impl<'a> /*trait*/ QButtonGroup_checkedId<i32> for () {
  fn checkedId(self , rsthis: & QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup9checkedIdEv()};
    let mut ret = unsafe {_ZNK12QButtonGroup9checkedIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QAbstractButton * QButtonGroup::checkedButton();
impl /*struct*/ QButtonGroup {
  pub fn checkedButton<RetType, T: QButtonGroup_checkedButton<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.checkedButton(self);
    // return 1;
  }
}

pub trait QButtonGroup_checkedButton<RetType> {
  fn checkedButton(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  QAbstractButton * QButtonGroup::checkedButton();
impl<'a> /*trait*/ QButtonGroup_checkedButton<()> for () {
  fn checkedButton(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup13checkedButtonEv()};
     unsafe {_ZNK12QButtonGroup13checkedButtonEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QButtonGroup::setExclusive(bool );
impl /*struct*/ QButtonGroup {
  pub fn setExclusive<RetType, T: QButtonGroup_setExclusive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExclusive(self);
    // return 1;
  }
}

pub trait QButtonGroup_setExclusive<RetType> {
  fn setExclusive(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  void QButtonGroup::setExclusive(bool );
impl<'a> /*trait*/ QButtonGroup_setExclusive<()> for (i8) {
  fn setExclusive(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup12setExclusiveEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QButtonGroup12setExclusiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QButtonGroup::setId(QAbstractButton * button, int id);
impl /*struct*/ QButtonGroup {
  pub fn setId<RetType, T: QButtonGroup_setId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setId(self);
    // return 1;
  }
}

pub trait QButtonGroup_setId<RetType> {
  fn setId(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  void QButtonGroup::setId(QAbstractButton * button, int id);
impl<'a> /*trait*/ QButtonGroup_setId<()> for (&'a QAbstractButton, i32) {
  fn setId(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup5setIdEP15QAbstractButtoni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QButtonGroup5setIdEP15QAbstractButtoni(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QButtonGroup::exclusive();
impl /*struct*/ QButtonGroup {
  pub fn exclusive<RetType, T: QButtonGroup_exclusive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exclusive(self);
    // return 1;
  }
}

pub trait QButtonGroup_exclusive<RetType> {
  fn exclusive(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  bool QButtonGroup::exclusive();
impl<'a> /*trait*/ QButtonGroup_exclusive<i8> for () {
  fn exclusive(self , rsthis: & QButtonGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup9exclusiveEv()};
    let mut ret = unsafe {_ZNK12QButtonGroup9exclusiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

#[derive(Default)] // for QButtonGroup_buttonToggled
pub struct QButtonGroup_buttonToggled_signal{poi:u64}
impl /* struct */ QButtonGroup {
  pub fn buttonToggled(&self) -> QButtonGroup_buttonToggled_signal {
     return QButtonGroup_buttonToggled_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QButtonGroup_buttonToggled_signal {
  pub fn connect<T: QButtonGroup_buttonToggled_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QButtonGroup_buttonToggled_signal_connect {
  fn connect(self, sigthis: QButtonGroup_buttonToggled_signal);
}

#[derive(Default)] // for QButtonGroup_buttonClicked
pub struct QButtonGroup_buttonClicked_signal{poi:u64}
impl /* struct */ QButtonGroup {
  pub fn buttonClicked(&self) -> QButtonGroup_buttonClicked_signal {
     return QButtonGroup_buttonClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QButtonGroup_buttonClicked_signal {
  pub fn connect<T: QButtonGroup_buttonClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QButtonGroup_buttonClicked_signal_connect {
  fn connect(self, sigthis: QButtonGroup_buttonClicked_signal);
}

#[derive(Default)] // for QButtonGroup_buttonReleased
pub struct QButtonGroup_buttonReleased_signal{poi:u64}
impl /* struct */ QButtonGroup {
  pub fn buttonReleased(&self) -> QButtonGroup_buttonReleased_signal {
     return QButtonGroup_buttonReleased_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QButtonGroup_buttonReleased_signal {
  pub fn connect<T: QButtonGroup_buttonReleased_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QButtonGroup_buttonReleased_signal_connect {
  fn connect(self, sigthis: QButtonGroup_buttonReleased_signal);
}

#[derive(Default)] // for QButtonGroup_buttonPressed
pub struct QButtonGroup_buttonPressed_signal{poi:u64}
impl /* struct */ QButtonGroup {
  pub fn buttonPressed(&self) -> QButtonGroup_buttonPressed_signal {
     return QButtonGroup_buttonPressed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QButtonGroup_buttonPressed_signal {
  pub fn connect<T: QButtonGroup_buttonPressed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QButtonGroup_buttonPressed_signal_connect {
  fn connect(self, sigthis: QButtonGroup_buttonPressed_signal);
}

// buttonReleased(class QAbstractButton *)
extern fn QButtonGroup_buttonReleased_signal_connect_cb_0(rsfptr:fn(QAbstractButton), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QAbstractButton::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QButtonGroup_buttonReleased_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QAbstractButton)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QAbstractButton::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QButtonGroup_buttonReleased_signal_connect for fn(QAbstractButton) {
  fn connect(self, sigthis: QButtonGroup_buttonReleased_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonReleased_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup14buttonReleasedEP15QAbstractButton(arg0, arg1, arg2)};
  }
}
impl /* trait */ QButtonGroup_buttonReleased_signal_connect for Box<Fn(QAbstractButton)> {
  fn connect(self, sigthis: QButtonGroup_buttonReleased_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonReleased_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup14buttonReleasedEP15QAbstractButton(arg0, arg1, arg2)};
  }
}
// buttonPressed(int)
extern fn QButtonGroup_buttonPressed_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QButtonGroup_buttonPressed_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QButtonGroup_buttonPressed_signal_connect for fn(i32) {
  fn connect(self, sigthis: QButtonGroup_buttonPressed_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonPressed_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonPressedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QButtonGroup_buttonPressed_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QButtonGroup_buttonPressed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonPressed_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonPressedEi(arg0, arg1, arg2)};
  }
}
// buttonToggled(int, _Bool)
extern fn QButtonGroup_buttonToggled_signal_connect_cb_2(rsfptr:fn(i32, i8), arg0: c_int, arg1: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i8;
  rsfptr(rsarg0,rsarg1);
}
extern fn QButtonGroup_buttonToggled_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(i32, i8)>, arg0: c_int, arg1: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i8;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QButtonGroup_buttonToggled_signal_connect for fn(i32, i8) {
  fn connect(self, sigthis: QButtonGroup_buttonToggled_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonToggled_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonToggledEib(arg0, arg1, arg2)};
  }
}
impl /* trait */ QButtonGroup_buttonToggled_signal_connect for Box<Fn(i32, i8)> {
  fn connect(self, sigthis: QButtonGroup_buttonToggled_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonToggled_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonToggledEib(arg0, arg1, arg2)};
  }
}
// buttonPressed(class QAbstractButton *)
extern fn QButtonGroup_buttonPressed_signal_connect_cb_3(rsfptr:fn(QAbstractButton), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QAbstractButton::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QButtonGroup_buttonPressed_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QAbstractButton)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QAbstractButton::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QButtonGroup_buttonPressed_signal_connect for fn(QAbstractButton) {
  fn connect(self, sigthis: QButtonGroup_buttonPressed_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonPressed_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonPressedEP15QAbstractButton(arg0, arg1, arg2)};
  }
}
impl /* trait */ QButtonGroup_buttonPressed_signal_connect for Box<Fn(QAbstractButton)> {
  fn connect(self, sigthis: QButtonGroup_buttonPressed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonPressed_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonPressedEP15QAbstractButton(arg0, arg1, arg2)};
  }
}
// buttonReleased(int)
extern fn QButtonGroup_buttonReleased_signal_connect_cb_4(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QButtonGroup_buttonReleased_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QButtonGroup_buttonReleased_signal_connect for fn(i32) {
  fn connect(self, sigthis: QButtonGroup_buttonReleased_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonReleased_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup14buttonReleasedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QButtonGroup_buttonReleased_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QButtonGroup_buttonReleased_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonReleased_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup14buttonReleasedEi(arg0, arg1, arg2)};
  }
}
// buttonToggled(class QAbstractButton *, _Bool)
extern fn QButtonGroup_buttonToggled_signal_connect_cb_5(rsfptr:fn(QAbstractButton, i8), arg0: *mut c_void, arg1: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QAbstractButton::inheritFrom(arg0 as u64);
  let rsarg1 = arg1 as i8;
  rsfptr(rsarg0,rsarg1);
}
extern fn QButtonGroup_buttonToggled_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(QAbstractButton, i8)>, arg0: *mut c_void, arg1: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QAbstractButton::inheritFrom(arg0 as u64);
  let rsarg1 = arg1 as i8;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QButtonGroup_buttonToggled_signal_connect for fn(QAbstractButton, i8) {
  fn connect(self, sigthis: QButtonGroup_buttonToggled_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonToggled_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonToggledEP15QAbstractButtonb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QButtonGroup_buttonToggled_signal_connect for Box<Fn(QAbstractButton, i8)> {
  fn connect(self, sigthis: QButtonGroup_buttonToggled_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonToggled_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonToggledEP15QAbstractButtonb(arg0, arg1, arg2)};
  }
}
// buttonClicked(int)
extern fn QButtonGroup_buttonClicked_signal_connect_cb_6(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QButtonGroup_buttonClicked_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QButtonGroup_buttonClicked_signal_connect for fn(i32) {
  fn connect(self, sigthis: QButtonGroup_buttonClicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonClicked_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonClickedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QButtonGroup_buttonClicked_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QButtonGroup_buttonClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonClicked_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonClickedEi(arg0, arg1, arg2)};
  }
}
// buttonClicked(class QAbstractButton *)
extern fn QButtonGroup_buttonClicked_signal_connect_cb_7(rsfptr:fn(QAbstractButton), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QAbstractButton::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QButtonGroup_buttonClicked_signal_connect_cb_box_7(rsfptr_raw:*mut Box<Fn(QAbstractButton)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QAbstractButton::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QButtonGroup_buttonClicked_signal_connect for fn(QAbstractButton) {
  fn connect(self, sigthis: QButtonGroup_buttonClicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonClicked_signal_connect_cb_7 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonClickedEP15QAbstractButton(arg0, arg1, arg2)};
  }
}
impl /* trait */ QButtonGroup_buttonClicked_signal_connect for Box<Fn(QAbstractButton)> {
  fn connect(self, sigthis: QButtonGroup_buttonClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QButtonGroup_buttonClicked_signal_connect_cb_box_7 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QButtonGroup_SlotProxy_connect__ZN12QButtonGroup13buttonClickedEP15QAbstractButton(arg0, arg1, arg2)};
  }
}
// <= body block end

