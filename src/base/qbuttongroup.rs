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
  // proto:  QList<QAbstractButton *> QButtonGroup::buttons();
  fn _ZNK12QButtonGroup7buttonsEv(qthis: *mut c_void) ;
  // proto:  void QButtonGroup::FreeQButtonGroup();
  fn _ZN12QButtonGroupD0Ev(qthis: *mut c_void) ;
  // proto:  void QButtonGroup::buttonPressed(int );
  fn _ZN12QButtonGroup13buttonPressedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QButtonGroup::NewQButtonGroup(const QButtonGroup & );
  fn _ZN12QButtonGroupC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QButtonGroup::buttonReleased(int );
  fn _ZN12QButtonGroup14buttonReleasedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  const QMetaObject * QButtonGroup::metaObject();
  fn _ZNK12QButtonGroup10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QButtonGroup::buttonToggled(int , bool );
  fn _ZN12QButtonGroup13buttonToggledEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  void QButtonGroup::NewQButtonGroup(QObject * parent);
  fn _ZN12QButtonGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAbstractButton * QButtonGroup::button(int id);
  fn _ZNK12QButtonGroup6buttonEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QButtonGroup::checkedId();
  fn _ZNK12QButtonGroup9checkedIdEv(qthis: *mut c_void) -> c_int;
  // proto:  QAbstractButton * QButtonGroup::checkedButton();
  fn _ZNK12QButtonGroup13checkedButtonEv(qthis: *mut c_void) ;
  // proto:  void QButtonGroup::setExclusive(bool );
  fn _ZN12QButtonGroup12setExclusiveEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QButtonGroup::exclusive();
  fn _ZNK12QButtonGroup9exclusiveEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QButtonGroup::buttonClicked(int );
  fn _ZN12QButtonGroup13buttonClickedEi(qthis: *mut c_void, arg0: c_int) ;
}

// body block begin
// class sizeof(QButtonGroup)=1
pub struct QButtonGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QButtonGroup {
  pub fn buttons<T: QButtonGroup_buttons>(&mut self, value: T)  {
     value.buttons(self);
    // return 1;
  }
}

pub trait QButtonGroup_buttons {
  fn buttons(self, rsthis: &mut QButtonGroup) ;
}

// proto:  QList<QAbstractButton *> QButtonGroup::buttons();
impl<'a> /*trait*/ QButtonGroup_buttons for () {
  fn buttons(self, rsthis: &mut QButtonGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup7buttonsEv()};
     unsafe {_ZNK12QButtonGroup7buttonsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn FreeQButtonGroup<T: QButtonGroup_FreeQButtonGroup>(&mut self, value: T)  {
     value.FreeQButtonGroup(self);
    // return 1;
  }
}

pub trait QButtonGroup_FreeQButtonGroup {
  fn FreeQButtonGroup(self, rsthis: &mut QButtonGroup) ;
}

// proto:  void QButtonGroup::FreeQButtonGroup();
impl<'a> /*trait*/ QButtonGroup_FreeQButtonGroup for () {
  fn FreeQButtonGroup(self, rsthis: &mut QButtonGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroupD0Ev()};
     unsafe {_ZN12QButtonGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn buttonPressed<T: QButtonGroup_buttonPressed>(&mut self, value: T)  {
     value.buttonPressed(self);
    // return 1;
  }
}

pub trait QButtonGroup_buttonPressed {
  fn buttonPressed(self, rsthis: &mut QButtonGroup) ;
}

// proto:  void QButtonGroup::buttonPressed(int );
impl<'a> /*trait*/ QButtonGroup_buttonPressed for (i32) {
  fn buttonPressed(self, rsthis: &mut QButtonGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup13buttonPressedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QButtonGroup13buttonPressedEi(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QButtonGroupC1ERKS_(qthis, arg0)};
    let rsthis = QButtonGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn buttonReleased<T: QButtonGroup_buttonReleased>(&mut self, value: T)  {
     value.buttonReleased(self);
    // return 1;
  }
}

pub trait QButtonGroup_buttonReleased {
  fn buttonReleased(self, rsthis: &mut QButtonGroup) ;
}

// proto:  void QButtonGroup::buttonReleased(int );
impl<'a> /*trait*/ QButtonGroup_buttonReleased for (i32) {
  fn buttonReleased(self, rsthis: &mut QButtonGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup14buttonReleasedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QButtonGroup14buttonReleasedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn metaObject<T: QButtonGroup_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QButtonGroup_metaObject {
  fn metaObject(self, rsthis: &mut QButtonGroup) ;
}

// proto:  const QMetaObject * QButtonGroup::metaObject();
impl<'a> /*trait*/ QButtonGroup_metaObject for () {
  fn metaObject(self, rsthis: &mut QButtonGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup10metaObjectEv()};
     unsafe {_ZNK12QButtonGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn buttonToggled<T: QButtonGroup_buttonToggled>(&mut self, value: T)  {
     value.buttonToggled(self);
    // return 1;
  }
}

pub trait QButtonGroup_buttonToggled {
  fn buttonToggled(self, rsthis: &mut QButtonGroup) ;
}

// proto:  void QButtonGroup::buttonToggled(int , bool );
impl<'a> /*trait*/ QButtonGroup_buttonToggled for (i32, i8) {
  fn buttonToggled(self, rsthis: &mut QButtonGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup13buttonToggledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN12QButtonGroup13buttonToggledEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
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
  pub fn button<T: QButtonGroup_button>(&mut self, value: T)  {
     value.button(self);
    // return 1;
  }
}

pub trait QButtonGroup_button {
  fn button(self, rsthis: &mut QButtonGroup) ;
}

// proto:  QAbstractButton * QButtonGroup::button(int id);
impl<'a> /*trait*/ QButtonGroup_button for (i32) {
  fn button(self, rsthis: &mut QButtonGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup6buttonEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK12QButtonGroup6buttonEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn checkedId<T: QButtonGroup_checkedId>(&mut self, value: T) -> i32 {
    return value.checkedId(self);
    // return 1;
  }
}

pub trait QButtonGroup_checkedId {
  fn checkedId(self, rsthis: &mut QButtonGroup) -> i32;
}

// proto:  int QButtonGroup::checkedId();
impl<'a> /*trait*/ QButtonGroup_checkedId for () {
  fn checkedId(self, rsthis: &mut QButtonGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup9checkedIdEv()};
    let mut ret = unsafe {_ZNK12QButtonGroup9checkedIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn checkedButton<T: QButtonGroup_checkedButton>(&mut self, value: T)  {
     value.checkedButton(self);
    // return 1;
  }
}

pub trait QButtonGroup_checkedButton {
  fn checkedButton(self, rsthis: &mut QButtonGroup) ;
}

// proto:  QAbstractButton * QButtonGroup::checkedButton();
impl<'a> /*trait*/ QButtonGroup_checkedButton for () {
  fn checkedButton(self, rsthis: &mut QButtonGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup13checkedButtonEv()};
     unsafe {_ZNK12QButtonGroup13checkedButtonEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn setExclusive<T: QButtonGroup_setExclusive>(&mut self, value: T)  {
     value.setExclusive(self);
    // return 1;
  }
}

pub trait QButtonGroup_setExclusive {
  fn setExclusive(self, rsthis: &mut QButtonGroup) ;
}

// proto:  void QButtonGroup::setExclusive(bool );
impl<'a> /*trait*/ QButtonGroup_setExclusive for (i8) {
  fn setExclusive(self, rsthis: &mut QButtonGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup12setExclusiveEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QButtonGroup12setExclusiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn exclusive<T: QButtonGroup_exclusive>(&mut self, value: T) -> i8 {
    return value.exclusive(self);
    // return 1;
  }
}

pub trait QButtonGroup_exclusive {
  fn exclusive(self, rsthis: &mut QButtonGroup) -> i8;
}

// proto:  bool QButtonGroup::exclusive();
impl<'a> /*trait*/ QButtonGroup_exclusive for () {
  fn exclusive(self, rsthis: &mut QButtonGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QButtonGroup9exclusiveEv()};
    let mut ret = unsafe {_ZNK12QButtonGroup9exclusiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QButtonGroup {
  pub fn buttonClicked<T: QButtonGroup_buttonClicked>(&mut self, value: T)  {
     value.buttonClicked(self);
    // return 1;
  }
}

pub trait QButtonGroup_buttonClicked {
  fn buttonClicked(self, rsthis: &mut QButtonGroup) ;
}

// proto:  void QButtonGroup::buttonClicked(int );
impl<'a> /*trait*/ QButtonGroup_buttonClicked for (i32) {
  fn buttonClicked(self, rsthis: &mut QButtonGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QButtonGroup13buttonClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QButtonGroup13buttonClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

