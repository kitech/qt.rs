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

// proto:  QAccessibleImageInterface * QAccessibleInterface::imageInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn imageInterface<RetType, T: QAccessibleInterface_imageInterface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.imageInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_imageInterface<RetType> {
  fn imageInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QAccessibleImageInterface * QAccessibleInterface::imageInterface();
impl<'a> /*trait*/ QAccessibleInterface_imageInterface<QAccessibleImageInterface> for () {
  fn imageInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleImageInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface14imageInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface14imageInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleImageInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QAccessibleTableInterface * QAccessibleInterface::tableInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn tableInterface<RetType, T: QAccessibleInterface_tableInterface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.tableInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_tableInterface<RetType> {
  fn tableInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QAccessibleTableInterface * QAccessibleInterface::tableInterface();
impl<'a> /*trait*/ QAccessibleInterface_tableInterface<QAccessibleTableInterface> for () {
  fn tableInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleTableInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface14tableInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface14tableInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleTableInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QAccessibleEditableTextInterface * QAccessibleInterface::editableTextInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn editableTextInterface<RetType, T: QAccessibleInterface_editableTextInterface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.editableTextInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_editableTextInterface<RetType> {
  fn editableTextInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QAccessibleEditableTextInterface * QAccessibleInterface::editableTextInterface();
impl<'a> /*trait*/ QAccessibleInterface_editableTextInterface<QAccessibleEditableTextInterface> for () {
  fn editableTextInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleEditableTextInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface21editableTextInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface21editableTextInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleEditableTextInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QAccessibleValueInterface * QAccessibleInterface::valueInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn valueInterface<RetType, T: QAccessibleInterface_valueInterface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.valueInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_valueInterface<RetType> {
  fn valueInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QAccessibleValueInterface * QAccessibleInterface::valueInterface();
impl<'a> /*trait*/ QAccessibleInterface_valueInterface<QAccessibleValueInterface> for () {
  fn valueInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleValueInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface14valueInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface14valueInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleValueInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QRect QAccessibleInterface::rect();
impl /*struct*/ QAccessibleInterface {
  pub fn rect<RetType, T: QAccessibleInterface_rect<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_rect<RetType> {
  fn rect(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QRect QAccessibleInterface::rect();
impl<'a> /*trait*/ QAccessibleInterface_rect<QRect> for () {
  fn rect(self , rsthis: &mut QAccessibleInterface) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface4rectEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QObject * QAccessibleInterface::object();
impl /*struct*/ QAccessibleInterface {
  pub fn object<RetType, T: QAccessibleInterface_object<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.object(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_object<RetType> {
  fn object(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QObject * QAccessibleInterface::object();
impl<'a> /*trait*/ QAccessibleInterface_object<QObject> for () {
  fn object(self , rsthis: &mut QAccessibleInterface) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface6objectEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface6objectEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QAccessibleActionInterface * QAccessibleInterface::actionInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn actionInterface<RetType, T: QAccessibleInterface_actionInterface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.actionInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_actionInterface<RetType> {
  fn actionInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QAccessibleActionInterface * QAccessibleInterface::actionInterface();
impl<'a> /*trait*/ QAccessibleInterface_actionInterface<QAccessibleActionInterface> for () {
  fn actionInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleActionInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface15actionInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface15actionInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleActionInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QAccessibleInterface * QAccessibleInterface::parent();
impl /*struct*/ QAccessibleInterface {
  pub fn parent<RetType, T: QAccessibleInterface_parent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_parent<RetType> {
  fn parent(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QAccessibleInterface * QAccessibleInterface::parent();
impl<'a> /*trait*/ QAccessibleInterface_parent<QAccessibleInterface> for () {
  fn parent(self , rsthis: &mut QAccessibleInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface6parentEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface6parentEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QAccessibleInterface * QAccessibleInterface::childAt(int x, int y);
impl /*struct*/ QAccessibleInterface {
  pub fn childAt<RetType, T: QAccessibleInterface_childAt<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.childAt(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_childAt<RetType> {
  fn childAt(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QAccessibleInterface * QAccessibleInterface::childAt(int x, int y);
impl<'a> /*trait*/ QAccessibleInterface_childAt<QAccessibleInterface> for (i32, i32) {
  fn childAt(self , rsthis: &mut QAccessibleInterface) -> QAccessibleInterface {
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

// proto:  int QAccessibleInterface::childCount();
impl /*struct*/ QAccessibleInterface {
  pub fn childCount<RetType, T: QAccessibleInterface_childCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.childCount(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_childCount<RetType> {
  fn childCount(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  int QAccessibleInterface::childCount();
impl<'a> /*trait*/ QAccessibleInterface_childCount<i32> for () {
  fn childCount(self , rsthis: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface10childCountEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface10childCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QAccessibleTableCellInterface * QAccessibleInterface::tableCellInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn tableCellInterface<RetType, T: QAccessibleInterface_tableCellInterface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.tableCellInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_tableCellInterface<RetType> {
  fn tableCellInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QAccessibleTableCellInterface * QAccessibleInterface::tableCellInterface();
impl<'a> /*trait*/ QAccessibleInterface_tableCellInterface<QAccessibleTableCellInterface> for () {
  fn tableCellInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleTableCellInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface18tableCellInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface18tableCellInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleTableCellInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QAccessibleInterface::indexOfChild(const QAccessibleInterface * );
impl /*struct*/ QAccessibleInterface {
  pub fn indexOfChild<RetType, T: QAccessibleInterface_indexOfChild<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.indexOfChild(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_indexOfChild<RetType> {
  fn indexOfChild(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  int QAccessibleInterface::indexOfChild(const QAccessibleInterface * );
impl<'a> /*trait*/ QAccessibleInterface_indexOfChild<i32> for (&'a  QAccessibleInterface) {
  fn indexOfChild(self , rsthis: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface12indexOfChildEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QAccessibleInterface12indexOfChildEPKS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QColor QAccessibleInterface::foregroundColor();
impl /*struct*/ QAccessibleInterface {
  pub fn foregroundColor<RetType, T: QAccessibleInterface_foregroundColor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.foregroundColor(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_foregroundColor<RetType> {
  fn foregroundColor(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QColor QAccessibleInterface::foregroundColor();
impl<'a> /*trait*/ QAccessibleInterface_foregroundColor<QColor> for () {
  fn foregroundColor(self , rsthis: &mut QAccessibleInterface) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface15foregroundColorEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface15foregroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QAccessibleInterface::isValid();
impl /*struct*/ QAccessibleInterface {
  pub fn isValid<RetType, T: QAccessibleInterface_isValid<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_isValid<RetType> {
  fn isValid(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  bool QAccessibleInterface::isValid();
impl<'a> /*trait*/ QAccessibleInterface_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QAccessibleInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface7isValidEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QWindow * QAccessibleInterface::window();
impl /*struct*/ QAccessibleInterface {
  pub fn window<RetType, T: QAccessibleInterface_window<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_window<RetType> {
  fn window(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QWindow * QAccessibleInterface::window();
impl<'a> /*trait*/ QAccessibleInterface_window<QWindow> for () {
  fn window(self , rsthis: &mut QAccessibleInterface) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface6windowEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QAccessibleInterface::virtual_hook(int id, void * data);
impl /*struct*/ QAccessibleInterface {
  pub fn virtual_hook<RetType, T: QAccessibleInterface_virtual_hook<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.virtual_hook(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_virtual_hook<RetType> {
  fn virtual_hook(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  void QAccessibleInterface::virtual_hook(int id, void * data);
impl<'a> /*trait*/ QAccessibleInterface_virtual_hook<()> for (i32, &'a mut u8) {
  fn virtual_hook(self , rsthis: &mut QAccessibleInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface12virtual_hookEiPv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut uint8_t;
     unsafe {_ZN20QAccessibleInterface12virtual_hookEiPv(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QAccessibleInterface * QAccessibleInterface::focusChild();
impl /*struct*/ QAccessibleInterface {
  pub fn focusChild<RetType, T: QAccessibleInterface_focusChild<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.focusChild(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_focusChild<RetType> {
  fn focusChild(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QAccessibleInterface * QAccessibleInterface::focusChild();
impl<'a> /*trait*/ QAccessibleInterface_focusChild<QAccessibleInterface> for () {
  fn focusChild(self , rsthis: &mut QAccessibleInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface10focusChildEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface10focusChildEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QAccessibleInterface * QAccessibleInterface::child(int index);
impl /*struct*/ QAccessibleInterface {
  pub fn child<RetType, T: QAccessibleInterface_child<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.child(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_child<RetType> {
  fn child(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QAccessibleInterface * QAccessibleInterface::child(int index);
impl<'a> /*trait*/ QAccessibleInterface_child<QAccessibleInterface> for (i32) {
  fn child(self , rsthis: &mut QAccessibleInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface5childEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK20QAccessibleInterface5childEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QAccessibleTextInterface * QAccessibleInterface::textInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn textInterface<RetType, T: QAccessibleInterface_textInterface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.textInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_textInterface<RetType> {
  fn textInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QAccessibleTextInterface * QAccessibleInterface::textInterface();
impl<'a> /*trait*/ QAccessibleInterface_textInterface<QAccessibleTextInterface> for () {
  fn textInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleTextInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface13textInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface13textInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleTextInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QColor QAccessibleInterface::backgroundColor();
impl /*struct*/ QAccessibleInterface {
  pub fn backgroundColor<RetType, T: QAccessibleInterface_backgroundColor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.backgroundColor(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_backgroundColor<RetType> {
  fn backgroundColor(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  QColor QAccessibleInterface::backgroundColor();
impl<'a> /*trait*/ QAccessibleInterface_backgroundColor<QColor> for () {
  fn backgroundColor(self , rsthis: &mut QAccessibleInterface) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface15backgroundColorEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QAccessibleInterface::FreeQAccessibleInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn FreeQAccessibleInterface<RetType, T: QAccessibleInterface_FreeQAccessibleInterface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_FreeQAccessibleInterface<RetType> {
  fn FreeQAccessibleInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

// proto:  void QAccessibleInterface::FreeQAccessibleInterface();
impl<'a> /*trait*/ QAccessibleInterface_FreeQAccessibleInterface<()> for () {
  fn FreeQAccessibleInterface(self , rsthis: &mut QAccessibleInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterfaceD0Ev()};
     unsafe {_ZN20QAccessibleInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

