// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qstring::QString;
use super::qicon::QIcon;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QToolBox::removeItem(int index);
  fn _ZN8QToolBox10removeItemEi(arg0: c_int) -> i32;
  // proto: int QToolBox::insertItem(int index, QWidget * widget, const QString & text);
  fn _ZN8QToolBox10insertItemEiP7QWidgetRK7QString(arg0: c_int, arg1: *mut c_void, arg2: *const c_void) -> i32;
  // proto: QString QToolBox::itemText(int index);
  fn _ZNK8QToolBox8itemTextEi(arg0: c_int) -> i32;
  // proto: int QToolBox::indexOf(QWidget * widget);
  fn _ZNK8QToolBox7indexOfEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: QString QToolBox::itemToolTip(int index);
  fn _ZNK8QToolBox11itemToolTipEi(arg0: c_int) -> i32;
  // proto: void QToolBox::NewQToolBox(const QToolBox & );
  fn _ZN8QToolBoxC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QToolBox::setCurrentWidget(QWidget * widget);
  fn _ZN8QToolBox16setCurrentWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QToolBox::setCurrentIndex(int index);
  fn _ZN8QToolBox15setCurrentIndexEi(arg0: c_int) -> i32;
  // proto: void QToolBox::setItemIcon(int index, const QIcon & icon);
  fn _ZN8QToolBox11setItemIconEiRK5QIcon(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QToolBox::setItemText(int index, const QString & text);
  fn _ZN8QToolBox11setItemTextEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: int QToolBox::count();
  fn _ZNK8QToolBox5countEv() -> i32;
  // proto: const QMetaObject * QToolBox::metaObject();
  fn _ZNK8QToolBox10metaObjectEv() -> i32;
  // proto: QWidget * QToolBox::widget(int index);
  fn _ZNK8QToolBox6widgetEi(arg0: c_int) -> i32;
  // proto: void QToolBox::setItemToolTip(int index, const QString & toolTip);
  fn _ZN8QToolBox14setItemToolTipEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: int QToolBox::currentIndex();
  fn _ZNK8QToolBox12currentIndexEv() -> i32;
  // proto: QWidget * QToolBox::currentWidget();
  fn _ZNK8QToolBox13currentWidgetEv() -> i32;
  // proto: int QToolBox::addItem(QWidget * widget, const QString & text);
  fn _ZN8QToolBox7addItemEP7QWidgetRK7QString(arg0: *mut c_void, arg1: *const c_void) -> i32;
  // proto: bool QToolBox::isItemEnabled(int index);
  fn _ZNK8QToolBox13isItemEnabledEi(arg0: c_int) -> i32;
  // proto: void QToolBox::setItemEnabled(int index, bool enabled);
  fn _ZN8QToolBox14setItemEnabledEib(arg0: c_int, arg1: int8_t) -> i32;
  // proto: void QToolBox::currentChanged(int index);
  fn _ZN8QToolBox14currentChangedEi(arg0: c_int) -> i32;
  // proto: QIcon QToolBox::itemIcon(int index);
  fn _ZNK8QToolBox8itemIconEi(arg0: c_int) -> i32;
  // proto: void QToolBox::FreeQToolBox();
  fn _ZN8QToolBoxD0Ev() -> i32;
  // proto: int QToolBox::addItem(QWidget * widget, const QIcon & icon, const QString & text);
  fn _ZN8QToolBox7addItemEP7QWidgetRK5QIconRK7QString(arg0: *mut c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: int QToolBox::insertItem(int index, QWidget * widget, const QIcon & icon, const QString & text);
  fn _ZN8QToolBox10insertItemEiP7QWidgetRK5QIconRK7QString(arg0: c_int, arg1: *mut c_void, arg2: *const c_void, arg3: *const c_void) -> i32;
}

// body block begin
// class sizeof(QToolBox)=1
pub struct QToolBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QToolBox {
  pub fn removeItem<T: QToolBox_removeItem>(&mut self, value: T) -> i32 {
    value.removeItem(self);
    return 1;
  }
}

pub trait QToolBox_removeItem {
  fn removeItem(self, this: &mut QToolBox) -> i32;
}

// proto: void QToolBox::removeItem(int index);
impl<'a> /*trait*/ QToolBox_removeItem for (i32) {
  fn removeItem(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox10removeItemEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QToolBox10removeItemEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn insertItem<T: QToolBox_insertItem>(&mut self, value: T) -> i32 {
    value.insertItem(self);
    return 1;
  }
}

pub trait QToolBox_insertItem {
  fn insertItem(self, this: &mut QToolBox) -> i32;
}

// proto: int QToolBox::insertItem(int index, QWidget * widget, const QString & text);
impl<'a> /*trait*/ QToolBox_insertItem for (i32, &'a mut QWidget, &'a  QString) {
  fn insertItem(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox10insertItemEiP7QWidgetRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QToolBox10insertItemEiP7QWidgetRK7QString(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn itemText<T: QToolBox_itemText>(&mut self, value: T) -> i32 {
    value.itemText(self);
    return 1;
  }
}

pub trait QToolBox_itemText {
  fn itemText(self, this: &mut QToolBox) -> i32;
}

// proto: QString QToolBox::itemText(int index);
impl<'a> /*trait*/ QToolBox_itemText for (i32) {
  fn itemText(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox8itemTextEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK8QToolBox8itemTextEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn indexOf<T: QToolBox_indexOf>(&mut self, value: T) -> i32 {
    value.indexOf(self);
    return 1;
  }
}

pub trait QToolBox_indexOf {
  fn indexOf(self, this: &mut QToolBox) -> i32;
}

// proto: int QToolBox::indexOf(QWidget * widget);
impl<'a> /*trait*/ QToolBox_indexOf for (&'a mut QWidget) {
  fn indexOf(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK8QToolBox7indexOfEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn itemToolTip<T: QToolBox_itemToolTip>(&mut self, value: T) -> i32 {
    value.itemToolTip(self);
    return 1;
  }
}

pub trait QToolBox_itemToolTip {
  fn itemToolTip(self, this: &mut QToolBox) -> i32;
}

// proto: QString QToolBox::itemToolTip(int index);
impl<'a> /*trait*/ QToolBox_itemToolTip for (i32) {
  fn itemToolTip(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox11itemToolTipEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK8QToolBox11itemToolTipEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn NewQToolBox<T: QToolBox_NewQToolBox>(value: T) -> QToolBox {
    let rsthis = value.NewQToolBox();
    return rsthis;
    // return 1;
  }
}

pub trait QToolBox_NewQToolBox {
  fn NewQToolBox(self) -> QToolBox;
}

// proto: void QToolBox::NewQToolBox(const QToolBox & );
impl<'a> /*trait*/ QToolBox_NewQToolBox for (&'a  QToolBox) {
  fn NewQToolBox(self) -> QToolBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QToolBoxC1ERKS_(qthis, arg0)};
    let rsthis = QToolBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn setCurrentWidget<T: QToolBox_setCurrentWidget>(&mut self, value: T) -> i32 {
    value.setCurrentWidget(self);
    return 1;
  }
}

pub trait QToolBox_setCurrentWidget {
  fn setCurrentWidget(self, this: &mut QToolBox) -> i32;
}

// proto: void QToolBox::setCurrentWidget(QWidget * widget);
impl<'a> /*trait*/ QToolBox_setCurrentWidget for (&'a mut QWidget) {
  fn setCurrentWidget(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox16setCurrentWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QToolBox16setCurrentWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn setCurrentIndex<T: QToolBox_setCurrentIndex>(&mut self, value: T) -> i32 {
    value.setCurrentIndex(self);
    return 1;
  }
}

pub trait QToolBox_setCurrentIndex {
  fn setCurrentIndex(self, this: &mut QToolBox) -> i32;
}

// proto: void QToolBox::setCurrentIndex(int index);
impl<'a> /*trait*/ QToolBox_setCurrentIndex for (i32) {
  fn setCurrentIndex(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox15setCurrentIndexEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QToolBox15setCurrentIndexEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn setItemIcon<T: QToolBox_setItemIcon>(&mut self, value: T) -> i32 {
    value.setItemIcon(self);
    return 1;
  }
}

pub trait QToolBox_setItemIcon {
  fn setItemIcon(self, this: &mut QToolBox) -> i32;
}

// proto: void QToolBox::setItemIcon(int index, const QIcon & icon);
impl<'a> /*trait*/ QToolBox_setItemIcon for (i32, &'a  QIcon) {
  fn setItemIcon(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox11setItemIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QToolBox11setItemIconEiRK5QIcon(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn setItemText<T: QToolBox_setItemText>(&mut self, value: T) -> i32 {
    value.setItemText(self);
    return 1;
  }
}

pub trait QToolBox_setItemText {
  fn setItemText(self, this: &mut QToolBox) -> i32;
}

// proto: void QToolBox::setItemText(int index, const QString & text);
impl<'a> /*trait*/ QToolBox_setItemText for (i32, &'a  QString) {
  fn setItemText(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox11setItemTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QToolBox11setItemTextEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn count<T: QToolBox_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QToolBox_count {
  fn count(self, this: &mut QToolBox) -> i32;
}

// proto: int QToolBox::count();
impl<'a> /*trait*/ QToolBox_count for () {
  fn count(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox5countEv()};
    unsafe {_ZNK8QToolBox5countEv()};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn metaObject<T: QToolBox_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QToolBox_metaObject {
  fn metaObject(self, this: &mut QToolBox) -> i32;
}

// proto: const QMetaObject * QToolBox::metaObject();
impl<'a> /*trait*/ QToolBox_metaObject for () {
  fn metaObject(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox10metaObjectEv()};
    unsafe {_ZNK8QToolBox10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn widget<T: QToolBox_widget>(&mut self, value: T) -> i32 {
    value.widget(self);
    return 1;
  }
}

pub trait QToolBox_widget {
  fn widget(self, this: &mut QToolBox) -> i32;
}

// proto: QWidget * QToolBox::widget(int index);
impl<'a> /*trait*/ QToolBox_widget for (i32) {
  fn widget(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox6widgetEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK8QToolBox6widgetEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn setItemToolTip<T: QToolBox_setItemToolTip>(&mut self, value: T) -> i32 {
    value.setItemToolTip(self);
    return 1;
  }
}

pub trait QToolBox_setItemToolTip {
  fn setItemToolTip(self, this: &mut QToolBox) -> i32;
}

// proto: void QToolBox::setItemToolTip(int index, const QString & toolTip);
impl<'a> /*trait*/ QToolBox_setItemToolTip for (i32, &'a  QString) {
  fn setItemToolTip(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox14setItemToolTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QToolBox14setItemToolTipEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn currentIndex<T: QToolBox_currentIndex>(&mut self, value: T) -> i32 {
    value.currentIndex(self);
    return 1;
  }
}

pub trait QToolBox_currentIndex {
  fn currentIndex(self, this: &mut QToolBox) -> i32;
}

// proto: int QToolBox::currentIndex();
impl<'a> /*trait*/ QToolBox_currentIndex for () {
  fn currentIndex(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox12currentIndexEv()};
    unsafe {_ZNK8QToolBox12currentIndexEv()};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn currentWidget<T: QToolBox_currentWidget>(&mut self, value: T) -> i32 {
    value.currentWidget(self);
    return 1;
  }
}

pub trait QToolBox_currentWidget {
  fn currentWidget(self, this: &mut QToolBox) -> i32;
}

// proto: QWidget * QToolBox::currentWidget();
impl<'a> /*trait*/ QToolBox_currentWidget for () {
  fn currentWidget(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox13currentWidgetEv()};
    unsafe {_ZNK8QToolBox13currentWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn addItem<T: QToolBox_addItem>(&mut self, value: T) -> i32 {
    value.addItem(self);
    return 1;
  }
}

pub trait QToolBox_addItem {
  fn addItem(self, this: &mut QToolBox) -> i32;
}

// proto: int QToolBox::addItem(QWidget * widget, const QString & text);
impl<'a> /*trait*/ QToolBox_addItem for (&'a mut QWidget, &'a  QString) {
  fn addItem(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox7addItemEP7QWidgetRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QToolBox7addItemEP7QWidgetRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn isItemEnabled<T: QToolBox_isItemEnabled>(&mut self, value: T) -> i32 {
    value.isItemEnabled(self);
    return 1;
  }
}

pub trait QToolBox_isItemEnabled {
  fn isItemEnabled(self, this: &mut QToolBox) -> i32;
}

// proto: bool QToolBox::isItemEnabled(int index);
impl<'a> /*trait*/ QToolBox_isItemEnabled for (i32) {
  fn isItemEnabled(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox13isItemEnabledEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK8QToolBox13isItemEnabledEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn setItemEnabled<T: QToolBox_setItemEnabled>(&mut self, value: T) -> i32 {
    value.setItemEnabled(self);
    return 1;
  }
}

pub trait QToolBox_setItemEnabled {
  fn setItemEnabled(self, this: &mut QToolBox) -> i32;
}

// proto: void QToolBox::setItemEnabled(int index, bool enabled);
impl<'a> /*trait*/ QToolBox_setItemEnabled for (i32, i8) {
  fn setItemEnabled(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox14setItemEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN8QToolBox14setItemEnabledEib(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn currentChanged<T: QToolBox_currentChanged>(&mut self, value: T) -> i32 {
    value.currentChanged(self);
    return 1;
  }
}

pub trait QToolBox_currentChanged {
  fn currentChanged(self, this: &mut QToolBox) -> i32;
}

// proto: void QToolBox::currentChanged(int index);
impl<'a> /*trait*/ QToolBox_currentChanged for (i32) {
  fn currentChanged(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox14currentChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QToolBox14currentChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn itemIcon<T: QToolBox_itemIcon>(&mut self, value: T) -> i32 {
    value.itemIcon(self);
    return 1;
  }
}

pub trait QToolBox_itemIcon {
  fn itemIcon(self, this: &mut QToolBox) -> i32;
}

// proto: QIcon QToolBox::itemIcon(int index);
impl<'a> /*trait*/ QToolBox_itemIcon for (i32) {
  fn itemIcon(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox8itemIconEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK8QToolBox8itemIconEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBox {
  pub fn FreeQToolBox<T: QToolBox_FreeQToolBox>(&mut self, value: T) -> i32 {
    value.FreeQToolBox(self);
    return 1;
  }
}

pub trait QToolBox_FreeQToolBox {
  fn FreeQToolBox(self, this: &mut QToolBox) -> i32;
}

// proto: void QToolBox::FreeQToolBox();
impl<'a> /*trait*/ QToolBox_FreeQToolBox for () {
  fn FreeQToolBox(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBoxD0Ev()};
    unsafe {_ZN8QToolBoxD0Ev()};
    return 1;
  }
}

// proto: int QToolBox::addItem(QWidget * widget, const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QToolBox_addItem for (&'a mut QWidget, &'a  QIcon, &'a  QString) {
  fn addItem(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox7addItemEP7QWidgetRK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QToolBox7addItemEP7QWidgetRK5QIconRK7QString(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: int QToolBox::insertItem(int index, QWidget * widget, const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QToolBox_insertItem for (i32, &'a mut QWidget, &'a  QIcon, &'a  QString) {
  fn insertItem(self, this: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox10insertItemEiP7QWidgetRK5QIconRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN8QToolBox10insertItemEiP7QWidgetRK5QIconRK7QString(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

