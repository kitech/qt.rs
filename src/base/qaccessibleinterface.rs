// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QAccessibleImageInterface * QAccessibleInterface::imageInterface();
  fn _ZN20QAccessibleInterface14imageInterfaceEv() -> i32;
  // proto: QAccessibleTableInterface * QAccessibleInterface::tableInterface();
  fn _ZN20QAccessibleInterface14tableInterfaceEv() -> i32;
  // proto: QAccessibleEditableTextInterface * QAccessibleInterface::editableTextInterface();
  fn _ZN20QAccessibleInterface21editableTextInterfaceEv() -> i32;
  // proto: QAccessibleValueInterface * QAccessibleInterface::valueInterface();
  fn _ZN20QAccessibleInterface14valueInterfaceEv() -> i32;
  // proto: QRect QAccessibleInterface::rect();
  fn _ZNK20QAccessibleInterface4rectEv() -> i32;
  // proto: QObject * QAccessibleInterface::object();
  fn _ZNK20QAccessibleInterface6objectEv() -> i32;
  // proto: QAccessibleActionInterface * QAccessibleInterface::actionInterface();
  fn _ZN20QAccessibleInterface15actionInterfaceEv() -> i32;
  // proto: QAccessibleInterface * QAccessibleInterface::parent();
  fn _ZNK20QAccessibleInterface6parentEv() -> i32;
  // proto: QAccessibleInterface * QAccessibleInterface::childAt(int x, int y);
  fn _ZNK20QAccessibleInterface7childAtEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: int QAccessibleInterface::childCount();
  fn _ZNK20QAccessibleInterface10childCountEv() -> i32;
  // proto: QAccessibleTableCellInterface * QAccessibleInterface::tableCellInterface();
  fn _ZN20QAccessibleInterface18tableCellInterfaceEv() -> i32;
  // proto: int QAccessibleInterface::indexOfChild(const QAccessibleInterface * );
  fn _ZNK20QAccessibleInterface12indexOfChildEPKS_(arg0: *const c_void) -> i32;
  // proto: QColor QAccessibleInterface::foregroundColor();
  fn _ZNK20QAccessibleInterface15foregroundColorEv() -> i32;
  // proto: bool QAccessibleInterface::isValid();
  fn _ZNK20QAccessibleInterface7isValidEv() -> i32;
  // proto: QWindow * QAccessibleInterface::window();
  fn _ZNK20QAccessibleInterface6windowEv() -> i32;
  // proto: void QAccessibleInterface::virtual_hook(int id, void * data);
  fn _ZN20QAccessibleInterface12virtual_hookEiPv(arg0: c_int, arg1: *mut uint8_t) -> i32;
  // proto: QAccessibleInterface * QAccessibleInterface::focusChild();
  fn _ZNK20QAccessibleInterface10focusChildEv() -> i32;
  // proto: QAccessibleInterface * QAccessibleInterface::child(int index);
  fn _ZNK20QAccessibleInterface5childEi(arg0: c_int) -> i32;
  // proto: QAccessibleTextInterface * QAccessibleInterface::textInterface();
  fn _ZN20QAccessibleInterface13textInterfaceEv() -> i32;
  // proto: QColor QAccessibleInterface::backgroundColor();
  fn _ZNK20QAccessibleInterface15backgroundColorEv() -> i32;
  // proto: void QAccessibleInterface::FreeQAccessibleInterface();
  fn _ZN20QAccessibleInterfaceD0Ev() -> i32;
}

// body block begin
// class sizeof(QAccessibleInterface)=8
pub struct QAccessibleInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleInterface {
  pub fn imageInterface<T: QAccessibleInterface_imageInterface>(&mut self, value: T) -> i32 {
    value.imageInterface(self);
    return 1;
  }
}

pub trait QAccessibleInterface_imageInterface {
  fn imageInterface(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QAccessibleImageInterface * QAccessibleInterface::imageInterface();
impl<'a> /*trait*/ QAccessibleInterface_imageInterface for () {
  fn imageInterface(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface14imageInterfaceEv()};
    unsafe {_ZN20QAccessibleInterface14imageInterfaceEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn tableInterface<T: QAccessibleInterface_tableInterface>(&mut self, value: T) -> i32 {
    value.tableInterface(self);
    return 1;
  }
}

pub trait QAccessibleInterface_tableInterface {
  fn tableInterface(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QAccessibleTableInterface * QAccessibleInterface::tableInterface();
impl<'a> /*trait*/ QAccessibleInterface_tableInterface for () {
  fn tableInterface(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface14tableInterfaceEv()};
    unsafe {_ZN20QAccessibleInterface14tableInterfaceEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn editableTextInterface<T: QAccessibleInterface_editableTextInterface>(&mut self, value: T) -> i32 {
    value.editableTextInterface(self);
    return 1;
  }
}

pub trait QAccessibleInterface_editableTextInterface {
  fn editableTextInterface(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QAccessibleEditableTextInterface * QAccessibleInterface::editableTextInterface();
impl<'a> /*trait*/ QAccessibleInterface_editableTextInterface for () {
  fn editableTextInterface(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface21editableTextInterfaceEv()};
    unsafe {_ZN20QAccessibleInterface21editableTextInterfaceEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn valueInterface<T: QAccessibleInterface_valueInterface>(&mut self, value: T) -> i32 {
    value.valueInterface(self);
    return 1;
  }
}

pub trait QAccessibleInterface_valueInterface {
  fn valueInterface(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QAccessibleValueInterface * QAccessibleInterface::valueInterface();
impl<'a> /*trait*/ QAccessibleInterface_valueInterface for () {
  fn valueInterface(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface14valueInterfaceEv()};
    unsafe {_ZN20QAccessibleInterface14valueInterfaceEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn rect<T: QAccessibleInterface_rect>(&mut self, value: T) -> i32 {
    value.rect(self);
    return 1;
  }
}

pub trait QAccessibleInterface_rect {
  fn rect(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QRect QAccessibleInterface::rect();
impl<'a> /*trait*/ QAccessibleInterface_rect for () {
  fn rect(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface4rectEv()};
    unsafe {_ZNK20QAccessibleInterface4rectEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn object<T: QAccessibleInterface_object>(&mut self, value: T) -> i32 {
    value.object(self);
    return 1;
  }
}

pub trait QAccessibleInterface_object {
  fn object(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QObject * QAccessibleInterface::object();
impl<'a> /*trait*/ QAccessibleInterface_object for () {
  fn object(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface6objectEv()};
    unsafe {_ZNK20QAccessibleInterface6objectEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn actionInterface<T: QAccessibleInterface_actionInterface>(&mut self, value: T) -> i32 {
    value.actionInterface(self);
    return 1;
  }
}

pub trait QAccessibleInterface_actionInterface {
  fn actionInterface(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QAccessibleActionInterface * QAccessibleInterface::actionInterface();
impl<'a> /*trait*/ QAccessibleInterface_actionInterface for () {
  fn actionInterface(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface15actionInterfaceEv()};
    unsafe {_ZN20QAccessibleInterface15actionInterfaceEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn parent<T: QAccessibleInterface_parent>(&mut self, value: T) -> i32 {
    value.parent(self);
    return 1;
  }
}

pub trait QAccessibleInterface_parent {
  fn parent(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QAccessibleInterface * QAccessibleInterface::parent();
impl<'a> /*trait*/ QAccessibleInterface_parent for () {
  fn parent(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface6parentEv()};
    unsafe {_ZNK20QAccessibleInterface6parentEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn childAt<T: QAccessibleInterface_childAt>(&mut self, value: T) -> i32 {
    value.childAt(self);
    return 1;
  }
}

pub trait QAccessibleInterface_childAt {
  fn childAt(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QAccessibleInterface * QAccessibleInterface::childAt(int x, int y);
impl<'a> /*trait*/ QAccessibleInterface_childAt for (i32, i32) {
  fn childAt(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface7childAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK20QAccessibleInterface7childAtEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn childCount<T: QAccessibleInterface_childCount>(&mut self, value: T) -> i32 {
    value.childCount(self);
    return 1;
  }
}

pub trait QAccessibleInterface_childCount {
  fn childCount(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: int QAccessibleInterface::childCount();
impl<'a> /*trait*/ QAccessibleInterface_childCount for () {
  fn childCount(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface10childCountEv()};
    unsafe {_ZNK20QAccessibleInterface10childCountEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn tableCellInterface<T: QAccessibleInterface_tableCellInterface>(&mut self, value: T) -> i32 {
    value.tableCellInterface(self);
    return 1;
  }
}

pub trait QAccessibleInterface_tableCellInterface {
  fn tableCellInterface(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QAccessibleTableCellInterface * QAccessibleInterface::tableCellInterface();
impl<'a> /*trait*/ QAccessibleInterface_tableCellInterface for () {
  fn tableCellInterface(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface18tableCellInterfaceEv()};
    unsafe {_ZN20QAccessibleInterface18tableCellInterfaceEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn indexOfChild<T: QAccessibleInterface_indexOfChild>(&mut self, value: T) -> i32 {
    value.indexOfChild(self);
    return 1;
  }
}

pub trait QAccessibleInterface_indexOfChild {
  fn indexOfChild(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: int QAccessibleInterface::indexOfChild(const QAccessibleInterface * );
impl<'a> /*trait*/ QAccessibleInterface_indexOfChild for (&'a  QAccessibleInterface) {
  fn indexOfChild(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface12indexOfChildEPKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK20QAccessibleInterface12indexOfChildEPKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn foregroundColor<T: QAccessibleInterface_foregroundColor>(&mut self, value: T) -> i32 {
    value.foregroundColor(self);
    return 1;
  }
}

pub trait QAccessibleInterface_foregroundColor {
  fn foregroundColor(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QColor QAccessibleInterface::foregroundColor();
impl<'a> /*trait*/ QAccessibleInterface_foregroundColor for () {
  fn foregroundColor(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface15foregroundColorEv()};
    unsafe {_ZNK20QAccessibleInterface15foregroundColorEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn isValid<T: QAccessibleInterface_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QAccessibleInterface_isValid {
  fn isValid(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: bool QAccessibleInterface::isValid();
impl<'a> /*trait*/ QAccessibleInterface_isValid for () {
  fn isValid(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface7isValidEv()};
    unsafe {_ZNK20QAccessibleInterface7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn window<T: QAccessibleInterface_window>(&mut self, value: T) -> i32 {
    value.window(self);
    return 1;
  }
}

pub trait QAccessibleInterface_window {
  fn window(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QWindow * QAccessibleInterface::window();
impl<'a> /*trait*/ QAccessibleInterface_window for () {
  fn window(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface6windowEv()};
    unsafe {_ZNK20QAccessibleInterface6windowEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn virtual_hook<T: QAccessibleInterface_virtual_hook>(&mut self, value: T) -> i32 {
    value.virtual_hook(self);
    return 1;
  }
}

pub trait QAccessibleInterface_virtual_hook {
  fn virtual_hook(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: void QAccessibleInterface::virtual_hook(int id, void * data);
impl<'a> /*trait*/ QAccessibleInterface_virtual_hook for (i32, &'a mut u8) {
  fn virtual_hook(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface12virtual_hookEiPv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut uint8_t;
    unsafe {_ZN20QAccessibleInterface12virtual_hookEiPv(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn focusChild<T: QAccessibleInterface_focusChild>(&mut self, value: T) -> i32 {
    value.focusChild(self);
    return 1;
  }
}

pub trait QAccessibleInterface_focusChild {
  fn focusChild(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QAccessibleInterface * QAccessibleInterface::focusChild();
impl<'a> /*trait*/ QAccessibleInterface_focusChild for () {
  fn focusChild(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface10focusChildEv()};
    unsafe {_ZNK20QAccessibleInterface10focusChildEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn child<T: QAccessibleInterface_child>(&mut self, value: T) -> i32 {
    value.child(self);
    return 1;
  }
}

pub trait QAccessibleInterface_child {
  fn child(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QAccessibleInterface * QAccessibleInterface::child(int index);
impl<'a> /*trait*/ QAccessibleInterface_child for (i32) {
  fn child(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface5childEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK20QAccessibleInterface5childEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn textInterface<T: QAccessibleInterface_textInterface>(&mut self, value: T) -> i32 {
    value.textInterface(self);
    return 1;
  }
}

pub trait QAccessibleInterface_textInterface {
  fn textInterface(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QAccessibleTextInterface * QAccessibleInterface::textInterface();
impl<'a> /*trait*/ QAccessibleInterface_textInterface for () {
  fn textInterface(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface13textInterfaceEv()};
    unsafe {_ZN20QAccessibleInterface13textInterfaceEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn backgroundColor<T: QAccessibleInterface_backgroundColor>(&mut self, value: T) -> i32 {
    value.backgroundColor(self);
    return 1;
  }
}

pub trait QAccessibleInterface_backgroundColor {
  fn backgroundColor(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: QColor QAccessibleInterface::backgroundColor();
impl<'a> /*trait*/ QAccessibleInterface_backgroundColor for () {
  fn backgroundColor(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface15backgroundColorEv()};
    unsafe {_ZNK20QAccessibleInterface15backgroundColorEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn FreeQAccessibleInterface<T: QAccessibleInterface_FreeQAccessibleInterface>(&mut self, value: T) -> i32 {
    value.FreeQAccessibleInterface(self);
    return 1;
  }
}

pub trait QAccessibleInterface_FreeQAccessibleInterface {
  fn FreeQAccessibleInterface(self, this: &mut QAccessibleInterface) -> i32;
}

// proto: void QAccessibleInterface::FreeQAccessibleInterface();
impl<'a> /*trait*/ QAccessibleInterface_FreeQAccessibleInterface for () {
  fn FreeQAccessibleInterface(self, this: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterfaceD0Ev()};
    unsafe {_ZN20QAccessibleInterfaceD0Ev()};
    return 1;
  }
}

