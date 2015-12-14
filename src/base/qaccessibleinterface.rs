// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaccessibleimageinterface::QAccessibleImageInterface;
use super::qaccessibletableinterface::QAccessibleTableInterface;
use super::qaccessibleeditabletextinterface::QAccessibleEditableTextInterface;
use super::qaccessiblevalueinterface::QAccessibleValueInterface;
use super::qrect::QRect;
use super::qobject::QObject;
use super::qaccessibleactioninterface::QAccessibleActionInterface;
use super::qaccessibletablecellinterface::QAccessibleTableCellInterface;
use super::qcolor::QColor;
use super::qwindow::QWindow;
use super::qaccessibletextinterface::QAccessibleTextInterface;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QAccessibleImageInterface * QAccessibleInterface::imageInterface();
  fn _ZN20QAccessibleInterface14imageInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleTableInterface * QAccessibleInterface::tableInterface();
  fn _ZN20QAccessibleInterface14tableInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleEditableTextInterface * QAccessibleInterface::editableTextInterface();
  fn _ZN20QAccessibleInterface21editableTextInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleValueInterface * QAccessibleInterface::valueInterface();
  fn _ZN20QAccessibleInterface14valueInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QAccessibleInterface::rect();
  fn _ZNK20QAccessibleInterface4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QObject * QAccessibleInterface::object();
  fn _ZNK20QAccessibleInterface6objectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleActionInterface * QAccessibleInterface::actionInterface();
  fn _ZN20QAccessibleInterface15actionInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleInterface::parent();
  fn _ZNK20QAccessibleInterface6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleInterface::childAt(int x, int y);
  fn _ZNK20QAccessibleInterface7childAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  int QAccessibleInterface::childCount();
  fn _ZNK20QAccessibleInterface10childCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QAccessibleTableCellInterface * QAccessibleInterface::tableCellInterface();
  fn _ZN20QAccessibleInterface18tableCellInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QAccessibleInterface::indexOfChild(const QAccessibleInterface * );
  fn _ZNK20QAccessibleInterface12indexOfChildEPKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QColor QAccessibleInterface::foregroundColor();
  fn _ZNK20QAccessibleInterface15foregroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QAccessibleInterface::isValid();
  fn _ZNK20QAccessibleInterface7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QWindow * QAccessibleInterface::window();
  fn _ZNK20QAccessibleInterface6windowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleInterface::virtual_hook(int id, void * data);
  fn _ZN20QAccessibleInterface12virtual_hookEiPv(qthis: *mut c_void, arg0: c_int, arg1: *mut uint8_t) ;
  // proto:  QAccessibleInterface * QAccessibleInterface::focusChild();
  fn _ZNK20QAccessibleInterface10focusChildEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleInterface::child(int index);
  fn _ZNK20QAccessibleInterface5childEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QAccessibleTextInterface * QAccessibleInterface::textInterface();
  fn _ZN20QAccessibleInterface13textInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QColor QAccessibleInterface::backgroundColor();
  fn _ZNK20QAccessibleInterface15backgroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleInterface::FreeQAccessibleInterface();
  fn _ZN20QAccessibleInterfaceD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QAccessibleInterface)=8
pub struct QAccessibleInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleInterface {
  pub fn imageInterface<T: QAccessibleInterface_imageInterface>(&mut self, value: T) -> QAccessibleImageInterface {
    return value.imageInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_imageInterface {
  fn imageInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleImageInterface;
}

// proto:  QAccessibleImageInterface * QAccessibleInterface::imageInterface();
impl<'a> /*trait*/ QAccessibleInterface_imageInterface for () {
  fn imageInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleImageInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface14imageInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface14imageInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleImageInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn tableInterface<T: QAccessibleInterface_tableInterface>(&mut self, value: T) -> QAccessibleTableInterface {
    return value.tableInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_tableInterface {
  fn tableInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleTableInterface;
}

// proto:  QAccessibleTableInterface * QAccessibleInterface::tableInterface();
impl<'a> /*trait*/ QAccessibleInterface_tableInterface for () {
  fn tableInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleTableInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface14tableInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface14tableInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleTableInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn editableTextInterface<T: QAccessibleInterface_editableTextInterface>(&mut self, value: T) -> QAccessibleEditableTextInterface {
    return value.editableTextInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_editableTextInterface {
  fn editableTextInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleEditableTextInterface;
}

// proto:  QAccessibleEditableTextInterface * QAccessibleInterface::editableTextInterface();
impl<'a> /*trait*/ QAccessibleInterface_editableTextInterface for () {
  fn editableTextInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleEditableTextInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface21editableTextInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface21editableTextInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleEditableTextInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn valueInterface<T: QAccessibleInterface_valueInterface>(&mut self, value: T) -> QAccessibleValueInterface {
    return value.valueInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_valueInterface {
  fn valueInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleValueInterface;
}

// proto:  QAccessibleValueInterface * QAccessibleInterface::valueInterface();
impl<'a> /*trait*/ QAccessibleInterface_valueInterface for () {
  fn valueInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleValueInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface14valueInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface14valueInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleValueInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn rect<T: QAccessibleInterface_rect>(&mut self, value: T) -> QRect {
    return value.rect(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_rect {
  fn rect(self, rsthis: &mut QAccessibleInterface) -> QRect;
}

// proto:  QRect QAccessibleInterface::rect();
impl<'a> /*trait*/ QAccessibleInterface_rect for () {
  fn rect(self, rsthis: &mut QAccessibleInterface) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface4rectEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn object<T: QAccessibleInterface_object>(&mut self, value: T) -> QObject {
    return value.object(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_object {
  fn object(self, rsthis: &mut QAccessibleInterface) -> QObject;
}

// proto:  QObject * QAccessibleInterface::object();
impl<'a> /*trait*/ QAccessibleInterface_object for () {
  fn object(self, rsthis: &mut QAccessibleInterface) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface6objectEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface6objectEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn actionInterface<T: QAccessibleInterface_actionInterface>(&mut self, value: T) -> QAccessibleActionInterface {
    return value.actionInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_actionInterface {
  fn actionInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleActionInterface;
}

// proto:  QAccessibleActionInterface * QAccessibleInterface::actionInterface();
impl<'a> /*trait*/ QAccessibleInterface_actionInterface for () {
  fn actionInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleActionInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface15actionInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface15actionInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleActionInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn parent<T: QAccessibleInterface_parent>(&mut self, value: T) -> QAccessibleInterface {
    return value.parent(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_parent {
  fn parent(self, rsthis: &mut QAccessibleInterface) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleInterface::parent();
impl<'a> /*trait*/ QAccessibleInterface_parent for () {
  fn parent(self, rsthis: &mut QAccessibleInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface6parentEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface6parentEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn childAt<T: QAccessibleInterface_childAt>(&mut self, value: T) -> QAccessibleInterface {
    return value.childAt(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_childAt {
  fn childAt(self, rsthis: &mut QAccessibleInterface) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleInterface::childAt(int x, int y);
impl<'a> /*trait*/ QAccessibleInterface_childAt for (i32, i32) {
  fn childAt(self, rsthis: &mut QAccessibleInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface7childAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK20QAccessibleInterface7childAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn childCount<T: QAccessibleInterface_childCount>(&mut self, value: T) -> i32 {
    return value.childCount(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_childCount {
  fn childCount(self, rsthis: &mut QAccessibleInterface) -> i32;
}

// proto:  int QAccessibleInterface::childCount();
impl<'a> /*trait*/ QAccessibleInterface_childCount for () {
  fn childCount(self, rsthis: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface10childCountEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface10childCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn tableCellInterface<T: QAccessibleInterface_tableCellInterface>(&mut self, value: T) -> QAccessibleTableCellInterface {
    return value.tableCellInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_tableCellInterface {
  fn tableCellInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleTableCellInterface;
}

// proto:  QAccessibleTableCellInterface * QAccessibleInterface::tableCellInterface();
impl<'a> /*trait*/ QAccessibleInterface_tableCellInterface for () {
  fn tableCellInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleTableCellInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface18tableCellInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface18tableCellInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleTableCellInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn indexOfChild<T: QAccessibleInterface_indexOfChild>(&mut self, value: T) -> i32 {
    return value.indexOfChild(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_indexOfChild {
  fn indexOfChild(self, rsthis: &mut QAccessibleInterface) -> i32;
}

// proto:  int QAccessibleInterface::indexOfChild(const QAccessibleInterface * );
impl<'a> /*trait*/ QAccessibleInterface_indexOfChild for (&'a  QAccessibleInterface) {
  fn indexOfChild(self, rsthis: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface12indexOfChildEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QAccessibleInterface12indexOfChildEPKS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn foregroundColor<T: QAccessibleInterface_foregroundColor>(&mut self, value: T) -> QColor {
    return value.foregroundColor(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_foregroundColor {
  fn foregroundColor(self, rsthis: &mut QAccessibleInterface) -> QColor;
}

// proto:  QColor QAccessibleInterface::foregroundColor();
impl<'a> /*trait*/ QAccessibleInterface_foregroundColor for () {
  fn foregroundColor(self, rsthis: &mut QAccessibleInterface) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface15foregroundColorEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface15foregroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn isValid<T: QAccessibleInterface_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_isValid {
  fn isValid(self, rsthis: &mut QAccessibleInterface) -> i8;
}

// proto:  bool QAccessibleInterface::isValid();
impl<'a> /*trait*/ QAccessibleInterface_isValid for () {
  fn isValid(self, rsthis: &mut QAccessibleInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface7isValidEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn window<T: QAccessibleInterface_window>(&mut self, value: T) -> QWindow {
    return value.window(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_window {
  fn window(self, rsthis: &mut QAccessibleInterface) -> QWindow;
}

// proto:  QWindow * QAccessibleInterface::window();
impl<'a> /*trait*/ QAccessibleInterface_window for () {
  fn window(self, rsthis: &mut QAccessibleInterface) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface6windowEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn virtual_hook<T: QAccessibleInterface_virtual_hook>(&mut self, value: T)  {
     value.virtual_hook(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_virtual_hook {
  fn virtual_hook(self, rsthis: &mut QAccessibleInterface) ;
}

// proto:  void QAccessibleInterface::virtual_hook(int id, void * data);
impl<'a> /*trait*/ QAccessibleInterface_virtual_hook for (i32, &'a mut u8) {
  fn virtual_hook(self, rsthis: &mut QAccessibleInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface12virtual_hookEiPv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut uint8_t;
     unsafe {_ZN20QAccessibleInterface12virtual_hookEiPv(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn focusChild<T: QAccessibleInterface_focusChild>(&mut self, value: T) -> QAccessibleInterface {
    return value.focusChild(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_focusChild {
  fn focusChild(self, rsthis: &mut QAccessibleInterface) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleInterface::focusChild();
impl<'a> /*trait*/ QAccessibleInterface_focusChild for () {
  fn focusChild(self, rsthis: &mut QAccessibleInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface10focusChildEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface10focusChildEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn child<T: QAccessibleInterface_child>(&mut self, value: T) -> QAccessibleInterface {
    return value.child(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_child {
  fn child(self, rsthis: &mut QAccessibleInterface) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleInterface::child(int index);
impl<'a> /*trait*/ QAccessibleInterface_child for (i32) {
  fn child(self, rsthis: &mut QAccessibleInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface5childEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK20QAccessibleInterface5childEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn textInterface<T: QAccessibleInterface_textInterface>(&mut self, value: T) -> QAccessibleTextInterface {
    return value.textInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_textInterface {
  fn textInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleTextInterface;
}

// proto:  QAccessibleTextInterface * QAccessibleInterface::textInterface();
impl<'a> /*trait*/ QAccessibleInterface_textInterface for () {
  fn textInterface(self, rsthis: &mut QAccessibleInterface) -> QAccessibleTextInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface13textInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface13textInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleTextInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn backgroundColor<T: QAccessibleInterface_backgroundColor>(&mut self, value: T) -> QColor {
    return value.backgroundColor(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_backgroundColor {
  fn backgroundColor(self, rsthis: &mut QAccessibleInterface) -> QColor;
}

// proto:  QColor QAccessibleInterface::backgroundColor();
impl<'a> /*trait*/ QAccessibleInterface_backgroundColor for () {
  fn backgroundColor(self, rsthis: &mut QAccessibleInterface) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface15backgroundColorEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn FreeQAccessibleInterface<T: QAccessibleInterface_FreeQAccessibleInterface>(&mut self, value: T)  {
     value.FreeQAccessibleInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_FreeQAccessibleInterface {
  fn FreeQAccessibleInterface(self, rsthis: &mut QAccessibleInterface) ;
}

// proto:  void QAccessibleInterface::FreeQAccessibleInterface();
impl<'a> /*trait*/ QAccessibleInterface_FreeQAccessibleInterface for () {
  fn FreeQAccessibleInterface(self, rsthis: &mut QAccessibleInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterfaceD0Ev()};
     unsafe {_ZN20QAccessibleInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

