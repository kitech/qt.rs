// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
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
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
  // proto:  QList<QAbstractButton *> QButtonGroup::buttons();
  fn _ZNK12QButtonGroup7buttonsEv(qthis: *mut c_void);
  // proto:  void QButtonGroup::~QButtonGroup();
  fn _ZN12QButtonGroupD0Ev(qthis: *mut c_void);
  // proto:  void QButtonGroup::buttonPressed(int );
  fn _ZN12QButtonGroup13buttonPressedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QButtonGroup::QButtonGroup(const QButtonGroup & );
  fn _ZN12QButtonGroupC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QButtonGroup::buttonReleased(int );
  fn _ZN12QButtonGroup14buttonReleasedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  const QMetaObject * QButtonGroup::metaObject();
  fn _ZNK12QButtonGroup10metaObjectEv(qthis: *mut c_void);
  // proto:  void QButtonGroup::buttonToggled(int , bool );
  fn _ZN12QButtonGroup13buttonToggledEib(qthis: *mut c_void, arg0: c_int, arg1: c_char);
  // proto:  void QButtonGroup::QButtonGroup(QObject * parent);
  fn _ZN12QButtonGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QAbstractButton * QButtonGroup::button(int id);
  fn _ZNK12QButtonGroup6buttonEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QButtonGroup::checkedId();
  fn _ZNK12QButtonGroup9checkedIdEv(qthis: *mut c_void) -> c_int;
  // proto:  QAbstractButton * QButtonGroup::checkedButton();
  fn _ZNK12QButtonGroup13checkedButtonEv(qthis: *mut c_void);
  // proto:  void QButtonGroup::setExclusive(bool );
  fn _ZN12QButtonGroup12setExclusiveEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QButtonGroup::exclusive();
  fn _ZNK12QButtonGroup9exclusiveEv(qthis: *mut c_void) -> c_char;
  // proto:  void QButtonGroup::buttonClicked(int );
  fn _ZN12QButtonGroup13buttonClickedEi(qthis: *mut c_void, arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QButtonGroup)=1
pub struct QButtonGroup {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QButtonGroup {
  pub fn inheritFrom(qthis: *mut c_void) -> QButtonGroup {
    return QButtonGroup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
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
  pub fn Free<RetType, T: QButtonGroup_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QButtonGroup_Free<RetType> {
  fn Free(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  void QButtonGroup::~QButtonGroup();
impl<'a> /*trait*/ QButtonGroup_Free<()> for () {
  fn Free(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroupD0Ev()};
     unsafe {_ZN12QButtonGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QButtonGroup::buttonPressed(int );
impl /*struct*/ QButtonGroup {
  pub fn buttonPressed<RetType, T: QButtonGroup_buttonPressed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.buttonPressed(self);
    // return 1;
  }
}

pub trait QButtonGroup_buttonPressed<RetType> {
  fn buttonPressed(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  void QButtonGroup::buttonPressed(int );
impl<'a> /*trait*/ QButtonGroup_buttonPressed<()> for (i32) {
  fn buttonPressed(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup13buttonPressedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QButtonGroup13buttonPressedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QButtonGroup::QButtonGroup(const QButtonGroup & );
impl /*struct*/ QButtonGroup {
  pub fn New<T: QButtonGroup_New>(value: T) -> QButtonGroup {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QButtonGroup_New {
  fn New(self) -> QButtonGroup;
}

  // proto:  void QButtonGroup::QButtonGroup(const QButtonGroup & );
impl<'a> /*trait*/ QButtonGroup_New for (&'a QButtonGroup) {
  fn New(self) -> QButtonGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QButtonGroupC1ERKS_(qthis, arg0)};
    let rsthis = QButtonGroup{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QButtonGroup::buttonReleased(int );
impl /*struct*/ QButtonGroup {
  pub fn buttonReleased<RetType, T: QButtonGroup_buttonReleased<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.buttonReleased(self);
    // return 1;
  }
}

pub trait QButtonGroup_buttonReleased<RetType> {
  fn buttonReleased(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  void QButtonGroup::buttonReleased(int );
impl<'a> /*trait*/ QButtonGroup_buttonReleased<()> for (i32) {
  fn buttonReleased(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup14buttonReleasedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QButtonGroup14buttonReleasedEi(rsthis.qclsinst, arg0)};
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

  // proto:  void QButtonGroup::buttonToggled(int , bool );
impl /*struct*/ QButtonGroup {
  pub fn buttonToggled<RetType, T: QButtonGroup_buttonToggled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.buttonToggled(self);
    // return 1;
  }
}

pub trait QButtonGroup_buttonToggled<RetType> {
  fn buttonToggled(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  void QButtonGroup::buttonToggled(int , bool );
impl<'a> /*trait*/ QButtonGroup_buttonToggled<()> for (i32, i8) {
  fn buttonToggled(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup13buttonToggledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {_ZN12QButtonGroup13buttonToggledEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QButtonGroup::QButtonGroup(QObject * parent);
impl<'a> /*trait*/ QButtonGroup_New for (&'a QObject) {
  fn New(self) -> QButtonGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroupC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QButtonGroupC1EP7QObject(qthis, arg0)};
    let rsthis = QButtonGroup{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
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

  // proto:  void QButtonGroup::buttonClicked(int );
impl /*struct*/ QButtonGroup {
  pub fn buttonClicked<RetType, T: QButtonGroup_buttonClicked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.buttonClicked(self);
    // return 1;
  }
}

pub trait QButtonGroup_buttonClicked<RetType> {
  fn buttonClicked(self , rsthis: & QButtonGroup) -> RetType;
}

  // proto:  void QButtonGroup::buttonClicked(int );
impl<'a> /*trait*/ QButtonGroup_buttonClicked<()> for (i32) {
  fn buttonClicked(self , rsthis: & QButtonGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup13buttonClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QButtonGroup13buttonClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

