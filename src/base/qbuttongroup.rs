// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QList<QAbstractButton *> QButtonGroup::buttons();
  fn _ZNK12QButtonGroup7buttonsEv() -> i32;
  // proto: void QButtonGroup::FreeQButtonGroup();
  fn _ZN12QButtonGroupD0Ev() -> i32;
  // proto: void QButtonGroup::buttonPressed(int );
  fn _ZN12QButtonGroup13buttonPressedEi(arg0: c_int) -> i32;
  // proto: void QButtonGroup::NewQButtonGroup(const QButtonGroup & );
  fn _ZN12QButtonGroupC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QButtonGroup::buttonReleased(int );
  fn _ZN12QButtonGroup14buttonReleasedEi(arg0: c_int) -> i32;
  // proto: const QMetaObject * QButtonGroup::metaObject();
  fn _ZNK12QButtonGroup10metaObjectEv() -> i32;
  // proto: void QButtonGroup::buttonToggled(int , bool );
  fn _ZN12QButtonGroup13buttonToggledEib(arg0: c_int, arg1: int8_t) -> i32;
  // proto: void QButtonGroup::NewQButtonGroup(QObject * parent);
  fn _ZN12QButtonGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QAbstractButton * QButtonGroup::button(int id);
  fn _ZNK12QButtonGroup6buttonEi(arg0: c_int) -> i32;
  // proto: int QButtonGroup::checkedId();
  fn _ZNK12QButtonGroup9checkedIdEv() -> i32;
  // proto: QAbstractButton * QButtonGroup::checkedButton();
  fn _ZNK12QButtonGroup13checkedButtonEv() -> i32;
  // proto: void QButtonGroup::setExclusive(bool );
  fn _ZN12QButtonGroup12setExclusiveEb(arg0: int8_t) -> i32;
  // proto: bool QButtonGroup::exclusive();
  fn _ZNK12QButtonGroup9exclusiveEv() -> i32;
  // proto: void QButtonGroup::buttonClicked(int );
  fn _ZN12QButtonGroup13buttonClickedEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QButtonGroup)=1
pub struct QButtonGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QButtonGroup {
  pub fn buttons<T: QButtonGroup_buttons>(&mut self, value: T) -> i32 {
    value.buttons(self);
    return 1;
  }
}

pub trait QButtonGroup_buttons {
  fn buttons(self, this: &mut QButtonGroup) -> i32;
}

// proto: QList<QAbstractButton *> QButtonGroup::buttons();
impl<'a> /*trait*/ QButtonGroup_buttons for () {
  fn buttons(self, this: &mut QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup7buttonsEv()};
    unsafe {_ZNK12QButtonGroup7buttonsEv()};
    return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn FreeQButtonGroup<T: QButtonGroup_FreeQButtonGroup>(&mut self, value: T) -> i32 {
    value.FreeQButtonGroup(self);
    return 1;
  }
}

pub trait QButtonGroup_FreeQButtonGroup {
  fn FreeQButtonGroup(self, this: &mut QButtonGroup) -> i32;
}

// proto: void QButtonGroup::FreeQButtonGroup();
impl<'a> /*trait*/ QButtonGroup_FreeQButtonGroup for () {
  fn FreeQButtonGroup(self, this: &mut QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroupD0Ev()};
    unsafe {_ZN12QButtonGroupD0Ev()};
    return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn buttonPressed<T: QButtonGroup_buttonPressed>(&mut self, value: T) -> i32 {
    value.buttonPressed(self);
    return 1;
  }
}

pub trait QButtonGroup_buttonPressed {
  fn buttonPressed(self, this: &mut QButtonGroup) -> i32;
}

// proto: void QButtonGroup::buttonPressed(int );
impl<'a> /*trait*/ QButtonGroup_buttonPressed for (i32) {
  fn buttonPressed(self, this: &mut QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup13buttonPressedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QButtonGroup13buttonPressedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn NewQButtonGroup<T: QButtonGroup_NewQButtonGroup>(value: T) -> QButtonGroup {
    let rsthis = value.NewQButtonGroup();
    return rsthis;
    // return 1;
  }
}

pub trait QButtonGroup_NewQButtonGroup {
  fn NewQButtonGroup(self) -> QButtonGroup;
}

// proto: void QButtonGroup::NewQButtonGroup(const QButtonGroup & );
impl<'a> /*trait*/ QButtonGroup_NewQButtonGroup for (&'a  QButtonGroup) {
  fn NewQButtonGroup(self) -> QButtonGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QButtonGroupC1ERKS_(qthis, arg0)};
    let rsthis = QButtonGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn buttonReleased<T: QButtonGroup_buttonReleased>(&mut self, value: T) -> i32 {
    value.buttonReleased(self);
    return 1;
  }
}

pub trait QButtonGroup_buttonReleased {
  fn buttonReleased(self, this: &mut QButtonGroup) -> i32;
}

// proto: void QButtonGroup::buttonReleased(int );
impl<'a> /*trait*/ QButtonGroup_buttonReleased for (i32) {
  fn buttonReleased(self, this: &mut QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup14buttonReleasedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QButtonGroup14buttonReleasedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn metaObject<T: QButtonGroup_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QButtonGroup_metaObject {
  fn metaObject(self, this: &mut QButtonGroup) -> i32;
}

// proto: const QMetaObject * QButtonGroup::metaObject();
impl<'a> /*trait*/ QButtonGroup_metaObject for () {
  fn metaObject(self, this: &mut QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup10metaObjectEv()};
    unsafe {_ZNK12QButtonGroup10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn buttonToggled<T: QButtonGroup_buttonToggled>(&mut self, value: T) -> i32 {
    value.buttonToggled(self);
    return 1;
  }
}

pub trait QButtonGroup_buttonToggled {
  fn buttonToggled(self, this: &mut QButtonGroup) -> i32;
}

// proto: void QButtonGroup::buttonToggled(int , bool );
impl<'a> /*trait*/ QButtonGroup_buttonToggled for (i32, i8) {
  fn buttonToggled(self, this: &mut QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup13buttonToggledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN12QButtonGroup13buttonToggledEib(arg0, arg1)};
    return 1;
  }
}

// proto: void QButtonGroup::NewQButtonGroup(QObject * parent);
impl<'a> /*trait*/ QButtonGroup_NewQButtonGroup for (&'a mut QObject) {
  fn NewQButtonGroup(self) -> QButtonGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroupC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QButtonGroupC1EP7QObject(qthis, arg0)};
    let rsthis = QButtonGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn button<T: QButtonGroup_button>(&mut self, value: T) -> i32 {
    value.button(self);
    return 1;
  }
}

pub trait QButtonGroup_button {
  fn button(self, this: &mut QButtonGroup) -> i32;
}

// proto: QAbstractButton * QButtonGroup::button(int id);
impl<'a> /*trait*/ QButtonGroup_button for (i32) {
  fn button(self, this: &mut QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup6buttonEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK12QButtonGroup6buttonEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn checkedId<T: QButtonGroup_checkedId>(&mut self, value: T) -> i32 {
    value.checkedId(self);
    return 1;
  }
}

pub trait QButtonGroup_checkedId {
  fn checkedId(self, this: &mut QButtonGroup) -> i32;
}

// proto: int QButtonGroup::checkedId();
impl<'a> /*trait*/ QButtonGroup_checkedId for () {
  fn checkedId(self, this: &mut QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup9checkedIdEv()};
    unsafe {_ZNK12QButtonGroup9checkedIdEv()};
    return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn checkedButton<T: QButtonGroup_checkedButton>(&mut self, value: T) -> i32 {
    value.checkedButton(self);
    return 1;
  }
}

pub trait QButtonGroup_checkedButton {
  fn checkedButton(self, this: &mut QButtonGroup) -> i32;
}

// proto: QAbstractButton * QButtonGroup::checkedButton();
impl<'a> /*trait*/ QButtonGroup_checkedButton for () {
  fn checkedButton(self, this: &mut QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup13checkedButtonEv()};
    unsafe {_ZNK12QButtonGroup13checkedButtonEv()};
    return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn setExclusive<T: QButtonGroup_setExclusive>(&mut self, value: T) -> i32 {
    value.setExclusive(self);
    return 1;
  }
}

pub trait QButtonGroup_setExclusive {
  fn setExclusive(self, this: &mut QButtonGroup) -> i32;
}

// proto: void QButtonGroup::setExclusive(bool );
impl<'a> /*trait*/ QButtonGroup_setExclusive for (i8) {
  fn setExclusive(self, this: &mut QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup12setExclusiveEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QButtonGroup12setExclusiveEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn exclusive<T: QButtonGroup_exclusive>(&mut self, value: T) -> i32 {
    value.exclusive(self);
    return 1;
  }
}

pub trait QButtonGroup_exclusive {
  fn exclusive(self, this: &mut QButtonGroup) -> i32;
}

// proto: bool QButtonGroup::exclusive();
impl<'a> /*trait*/ QButtonGroup_exclusive for () {
  fn exclusive(self, this: &mut QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup9exclusiveEv()};
    unsafe {_ZNK12QButtonGroup9exclusiveEv()};
    return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn buttonClicked<T: QButtonGroup_buttonClicked>(&mut self, value: T) -> i32 {
    value.buttonClicked(self);
    return 1;
  }
}

pub trait QButtonGroup_buttonClicked {
  fn buttonClicked(self, this: &mut QButtonGroup) -> i32;
}

// proto: void QButtonGroup::buttonClicked(int );
impl<'a> /*trait*/ QButtonGroup_buttonClicked for (i32) {
  fn buttonClicked(self, this: &mut QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup13buttonClickedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QButtonGroup13buttonClickedEi(arg0)};
    return 1;
  }
}

