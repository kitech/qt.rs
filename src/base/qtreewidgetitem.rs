// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfont::QFont;
use super::qvariant::QVariant;
use super::qstring::QString;
use super::qdatastream::QDataStream;
use super::qicon::QIcon;
use super::qtreewidget::QTreeWidget;
use super::qcolor::QColor;
use super::qstringlist::QStringList;
use super::qbrush::QBrush;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTreeWidgetItem::setFirstColumnSpanned(bool span);
  fn _ZN15QTreeWidgetItem21setFirstColumnSpannedEb(arg0: int8_t) -> i32;
  // proto: int QTreeWidgetItem::indexOfChild(QTreeWidgetItem * child);
  fn _ZNK15QTreeWidgetItem12indexOfChildEPS_(arg0: *mut c_void) -> i32;
  // proto: QVariant QTreeWidgetItem::data(int column, int role);
  fn _ZNK15QTreeWidgetItem4dataEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QTreeWidgetItem * QTreeWidgetItem::parent();
  fn _ZNK15QTreeWidgetItem6parentEv() -> i32;
  // proto: void QTreeWidgetItem::setFont(int column, const QFont & font);
  fn _ZN15QTreeWidgetItem7setFontEiRK5QFont(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTreeWidgetItem::setData(int column, int role, const QVariant & value);
  fn _ZN15QTreeWidgetItem7setDataEiiRK8QVariant(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: QFont QTreeWidgetItem::font(int column);
  fn _ZNK15QTreeWidgetItem4fontEi(arg0: c_int) -> i32;
  // proto: void QTreeWidgetItem::setStatusTip(int column, const QString & statusTip);
  fn _ZN15QTreeWidgetItem12setStatusTipEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTreeWidgetItem::setExpanded(bool expand);
  fn _ZN15QTreeWidgetItem11setExpandedEb(arg0: int8_t) -> i32;
  // proto: void QTreeWidgetItem::write(QDataStream & out);
  fn _ZNK15QTreeWidgetItem5writeER11QDataStream(arg0: *mut c_void) -> i32;
  // proto: bool QTreeWidgetItem::isExpanded();
  fn _ZNK15QTreeWidgetItem10isExpandedEv() -> i32;
  // proto: QList<QTreeWidgetItem *> QTreeWidgetItem::takeChildren();
  fn _ZN15QTreeWidgetItem12takeChildrenEv() -> i32;
  // proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidgetItem * parent, int type);
  fn _ZN15QTreeWidgetItemC1EPS_i(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> i32;
  // proto: void QTreeWidgetItem::setIcon(int column, const QIcon & icon);
  fn _ZN15QTreeWidgetItem7setIconEiRK5QIcon(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidgetItem * parent, QTreeWidgetItem * after, int type);
  fn _ZN15QTreeWidgetItemC1EPS_S0_i(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> i32;
  // proto: QString QTreeWidgetItem::toolTip(int column);
  fn _ZNK15QTreeWidgetItem7toolTipEi(arg0: c_int) -> i32;
  // proto: QColor QTreeWidgetItem::backgroundColor(int column);
  fn _ZNK15QTreeWidgetItem15backgroundColorEi(arg0: c_int) -> i32;
  // proto: QString QTreeWidgetItem::text(int column);
  fn _ZNK15QTreeWidgetItem4textEi(arg0: c_int) -> i32;
  // proto: bool QTreeWidgetItem::isHidden();
  fn _ZNK15QTreeWidgetItem8isHiddenEv() -> i32;
  // proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidget * view, QTreeWidgetItem * after, int type);
  fn _ZN15QTreeWidgetItemC1EP11QTreeWidgetPS_i(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> i32;
  // proto: void QTreeWidgetItem::setTextAlignment(int column, int alignment);
  fn _ZN15QTreeWidgetItem16setTextAlignmentEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QTreeWidgetItem::insertChild(int index, QTreeWidgetItem * child);
  fn _ZN15QTreeWidgetItem11insertChildEiPS_(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: void QTreeWidgetItem::NewQTreeWidgetItem(const QTreeWidgetItem & other);
  fn _ZN15QTreeWidgetItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QTreeWidgetItem::isDisabled();
  fn _ZNK15QTreeWidgetItem10isDisabledEv() -> i32;
  // proto: void QTreeWidgetItem::setText(int column, const QString & text);
  fn _ZN15QTreeWidgetItem7setTextEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTreeWidgetItem::setTextColor(int column, const QColor & color);
  fn _ZN15QTreeWidgetItem12setTextColorEiRK6QColor(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: QSize QTreeWidgetItem::sizeHint(int column);
  fn _ZNK15QTreeWidgetItem8sizeHintEi(arg0: c_int) -> i32;
  // proto: QString QTreeWidgetItem::whatsThis(int column);
  fn _ZNK15QTreeWidgetItem9whatsThisEi(arg0: c_int) -> i32;
  // proto: void QTreeWidgetItem::setWhatsThis(int column, const QString & whatsThis);
  fn _ZN15QTreeWidgetItem12setWhatsThisEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTreeWidgetItem::NewQTreeWidgetItem(int type);
  fn _ZN15QTreeWidgetItemC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  // proto: QColor QTreeWidgetItem::textColor(int column);
  fn _ZNK15QTreeWidgetItem9textColorEi(arg0: c_int) -> i32;
  // proto: QIcon QTreeWidgetItem::icon(int column);
  fn _ZNK15QTreeWidgetItem4iconEi(arg0: c_int) -> i32;
  // proto: void QTreeWidgetItem::setToolTip(int column, const QString & toolTip);
  fn _ZN15QTreeWidgetItem10setToolTipEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidget * view, const QStringList & strings, int type);
  fn _ZN15QTreeWidgetItemC1EP11QTreeWidgetRK11QStringListi(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_void, arg2: c_int) -> i32;
  // proto: bool QTreeWidgetItem::isFirstColumnSpanned();
  fn _ZNK15QTreeWidgetItem20isFirstColumnSpannedEv() -> i32;
  // proto: int QTreeWidgetItem::textAlignment(int column);
  fn _ZNK15QTreeWidgetItem13textAlignmentEi(arg0: c_int) -> i32;
  // proto: QTreeWidgetItem * QTreeWidgetItem::child(int index);
  fn _ZNK15QTreeWidgetItem5childEi(arg0: c_int) -> i32;
  // proto: void QTreeWidgetItem::NewQTreeWidgetItem(const QStringList & strings, int type);
  fn _ZN15QTreeWidgetItemC1ERK11QStringListi(qthis: *mut c_void, arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QTreeWidgetItem::setSelected(bool select);
  fn _ZN15QTreeWidgetItem11setSelectedEb(arg0: int8_t) -> i32;
  // proto: void QTreeWidgetItem::FreeQTreeWidgetItem();
  fn _ZN15QTreeWidgetItemD0Ev() -> i32;
  // proto: void QTreeWidgetItem::setHidden(bool hide);
  fn _ZN15QTreeWidgetItem9setHiddenEb(arg0: int8_t) -> i32;
  // proto: int QTreeWidgetItem::columnCount();
  fn _ZNK15QTreeWidgetItem11columnCountEv() -> i32;
  // proto: QTreeWidgetItem * QTreeWidgetItem::takeChild(int index);
  fn _ZN15QTreeWidgetItem9takeChildEi(arg0: c_int) -> i32;
  // proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidgetItem * parent, const QStringList & strings, int type);
  fn _ZN15QTreeWidgetItemC1EPS_RK11QStringListi(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_void, arg2: c_int) -> i32;
  // proto: void QTreeWidgetItem::setDisabled(bool disabled);
  fn _ZN15QTreeWidgetItem11setDisabledEb(arg0: int8_t) -> i32;
  // proto: void QTreeWidgetItem::setBackground(int column, const QBrush & brush);
  fn _ZN15QTreeWidgetItem13setBackgroundEiRK6QBrush(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTreeWidgetItem::addChild(QTreeWidgetItem * child);
  fn _ZN15QTreeWidgetItem8addChildEPS_(arg0: *mut c_void) -> i32;
  // proto: void QTreeWidgetItem::removeChild(QTreeWidgetItem * child);
  fn _ZN15QTreeWidgetItem11removeChildEPS_(arg0: *mut c_void) -> i32;
  // proto: QTreeWidgetItem * QTreeWidgetItem::clone();
  fn _ZNK15QTreeWidgetItem5cloneEv() -> i32;
  // proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidget * view, int type);
  fn _ZN15QTreeWidgetItemC1EP11QTreeWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> i32;
  // proto: void QTreeWidgetItem::setSizeHint(int column, const QSize & size);
  fn _ZN15QTreeWidgetItem11setSizeHintEiRK5QSize(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: QBrush QTreeWidgetItem::foreground(int column);
  fn _ZNK15QTreeWidgetItem10foregroundEi(arg0: c_int) -> i32;
  // proto: int QTreeWidgetItem::childCount();
  fn _ZNK15QTreeWidgetItem10childCountEv() -> i32;
  // proto: void QTreeWidgetItem::setBackgroundColor(int column, const QColor & color);
  fn _ZN15QTreeWidgetItem18setBackgroundColorEiRK6QColor(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: QString QTreeWidgetItem::statusTip(int column);
  fn _ZNK15QTreeWidgetItem9statusTipEi(arg0: c_int) -> i32;
  // proto: QBrush QTreeWidgetItem::background(int column);
  fn _ZNK15QTreeWidgetItem10backgroundEi(arg0: c_int) -> i32;
  // proto: int QTreeWidgetItem::type_();
  fn _ZNK15QTreeWidgetItem4typeEv() -> i32;
  // proto: QTreeWidget * QTreeWidgetItem::treeWidget();
  fn _ZNK15QTreeWidgetItem10treeWidgetEv() -> i32;
  // proto: void QTreeWidgetItem::read(QDataStream & in);
  fn _ZN15QTreeWidgetItem4readER11QDataStream(arg0: *mut c_void) -> i32;
  // proto: void QTreeWidgetItem::setForeground(int column, const QBrush & brush);
  fn _ZN15QTreeWidgetItem13setForegroundEiRK6QBrush(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: bool QTreeWidgetItem::isSelected();
  fn _ZNK15QTreeWidgetItem10isSelectedEv() -> i32;
}

// body block begin
// class sizeof(QTreeWidgetItem)=1
pub struct QTreeWidgetItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setFirstColumnSpanned<T: QTreeWidgetItem_setFirstColumnSpanned>(&mut self, value: T) -> i32 {
    value.setFirstColumnSpanned(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setFirstColumnSpanned {
  fn setFirstColumnSpanned(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setFirstColumnSpanned(bool span);
impl<'a> /*trait*/ QTreeWidgetItem_setFirstColumnSpanned for (i8) {
  fn setFirstColumnSpanned(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem21setFirstColumnSpannedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QTreeWidgetItem21setFirstColumnSpannedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn indexOfChild<T: QTreeWidgetItem_indexOfChild>(&mut self, value: T) -> i32 {
    value.indexOfChild(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_indexOfChild {
  fn indexOfChild(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: int QTreeWidgetItem::indexOfChild(QTreeWidgetItem * child);
impl<'a> /*trait*/ QTreeWidgetItem_indexOfChild for (&'a mut QTreeWidgetItem) {
  fn indexOfChild(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem12indexOfChildEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK15QTreeWidgetItem12indexOfChildEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn data<T: QTreeWidgetItem_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_data {
  fn data(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QVariant QTreeWidgetItem::data(int column, int role);
impl<'a> /*trait*/ QTreeWidgetItem_data for (i32, i32) {
  fn data(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem4dataEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK15QTreeWidgetItem4dataEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn parent<T: QTreeWidgetItem_parent>(&mut self, value: T) -> i32 {
    value.parent(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_parent {
  fn parent(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QTreeWidgetItem * QTreeWidgetItem::parent();
impl<'a> /*trait*/ QTreeWidgetItem_parent for () {
  fn parent(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem6parentEv()};
    unsafe {_ZNK15QTreeWidgetItem6parentEv()};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setFont<T: QTreeWidgetItem_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setFont {
  fn setFont(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setFont(int column, const QFont & font);
impl<'a> /*trait*/ QTreeWidgetItem_setFont for (i32, &'a  QFont) {
  fn setFont(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem7setFontEiRK5QFont()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QTreeWidgetItem7setFontEiRK5QFont(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setData<T: QTreeWidgetItem_setData>(&mut self, value: T) -> i32 {
    value.setData(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setData {
  fn setData(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setData(int column, int role, const QVariant & value);
impl<'a> /*trait*/ QTreeWidgetItem_setData for (i32, i32, &'a  QVariant) {
  fn setData(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem7setDataEiiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN15QTreeWidgetItem7setDataEiiRK8QVariant(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn font<T: QTreeWidgetItem_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_font {
  fn font(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QFont QTreeWidgetItem::font(int column);
impl<'a> /*trait*/ QTreeWidgetItem_font for (i32) {
  fn font(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem4fontEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QTreeWidgetItem4fontEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setStatusTip<T: QTreeWidgetItem_setStatusTip>(&mut self, value: T) -> i32 {
    value.setStatusTip(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setStatusTip {
  fn setStatusTip(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setStatusTip(int column, const QString & statusTip);
impl<'a> /*trait*/ QTreeWidgetItem_setStatusTip for (i32, &'a  QString) {
  fn setStatusTip(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem12setStatusTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QTreeWidgetItem12setStatusTipEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setExpanded<T: QTreeWidgetItem_setExpanded>(&mut self, value: T) -> i32 {
    value.setExpanded(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setExpanded {
  fn setExpanded(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setExpanded(bool expand);
impl<'a> /*trait*/ QTreeWidgetItem_setExpanded for (i8) {
  fn setExpanded(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11setExpandedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QTreeWidgetItem11setExpandedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn write<T: QTreeWidgetItem_write>(&mut self, value: T) -> i32 {
    value.write(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_write {
  fn write(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::write(QDataStream & out);
impl<'a> /*trait*/ QTreeWidgetItem_write for (&'a mut QDataStream) {
  fn write(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK15QTreeWidgetItem5writeER11QDataStream(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn isExpanded<T: QTreeWidgetItem_isExpanded>(&mut self, value: T) -> i32 {
    value.isExpanded(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_isExpanded {
  fn isExpanded(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: bool QTreeWidgetItem::isExpanded();
impl<'a> /*trait*/ QTreeWidgetItem_isExpanded for () {
  fn isExpanded(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10isExpandedEv()};
    unsafe {_ZNK15QTreeWidgetItem10isExpandedEv()};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn takeChildren<T: QTreeWidgetItem_takeChildren>(&mut self, value: T) -> i32 {
    value.takeChildren(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_takeChildren {
  fn takeChildren(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QList<QTreeWidgetItem *> QTreeWidgetItem::takeChildren();
impl<'a> /*trait*/ QTreeWidgetItem_takeChildren for () {
  fn takeChildren(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem12takeChildrenEv()};
    unsafe {_ZN15QTreeWidgetItem12takeChildrenEv()};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn NewQTreeWidgetItem<T: QTreeWidgetItem_NewQTreeWidgetItem>(value: T) -> QTreeWidgetItem {
    let rsthis = value.NewQTreeWidgetItem();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItem_NewQTreeWidgetItem {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem;
}

// proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidgetItem * parent, int type);
impl<'a> /*trait*/ QTreeWidgetItem_NewQTreeWidgetItem for (&'a mut QTreeWidgetItem, i32) {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EPS_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN15QTreeWidgetItemC1EPS_i(qthis, arg0, arg1)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setIcon<T: QTreeWidgetItem_setIcon>(&mut self, value: T) -> i32 {
    value.setIcon(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setIcon {
  fn setIcon(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setIcon(int column, const QIcon & icon);
impl<'a> /*trait*/ QTreeWidgetItem_setIcon for (i32, &'a  QIcon) {
  fn setIcon(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem7setIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QTreeWidgetItem7setIconEiRK5QIcon(arg0, arg1)};
    return 1;
  }
}

// proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidgetItem * parent, QTreeWidgetItem * after, int type);
impl<'a> /*trait*/ QTreeWidgetItem_NewQTreeWidgetItem for (&'a mut QTreeWidgetItem, &'a mut QTreeWidgetItem, i32) {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EPS_S0_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN15QTreeWidgetItemC1EPS_S0_i(qthis, arg0, arg1, arg2)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn toolTip<T: QTreeWidgetItem_toolTip>(&mut self, value: T) -> i32 {
    value.toolTip(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_toolTip {
  fn toolTip(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QString QTreeWidgetItem::toolTip(int column);
impl<'a> /*trait*/ QTreeWidgetItem_toolTip for (i32) {
  fn toolTip(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem7toolTipEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QTreeWidgetItem7toolTipEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn backgroundColor<T: QTreeWidgetItem_backgroundColor>(&mut self, value: T) -> i32 {
    value.backgroundColor(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_backgroundColor {
  fn backgroundColor(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QColor QTreeWidgetItem::backgroundColor(int column);
impl<'a> /*trait*/ QTreeWidgetItem_backgroundColor for (i32) {
  fn backgroundColor(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem15backgroundColorEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QTreeWidgetItem15backgroundColorEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn text<T: QTreeWidgetItem_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_text {
  fn text(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QString QTreeWidgetItem::text(int column);
impl<'a> /*trait*/ QTreeWidgetItem_text for (i32) {
  fn text(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem4textEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QTreeWidgetItem4textEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn isHidden<T: QTreeWidgetItem_isHidden>(&mut self, value: T) -> i32 {
    value.isHidden(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_isHidden {
  fn isHidden(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: bool QTreeWidgetItem::isHidden();
impl<'a> /*trait*/ QTreeWidgetItem_isHidden for () {
  fn isHidden(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem8isHiddenEv()};
    unsafe {_ZNK15QTreeWidgetItem8isHiddenEv()};
    return 1;
  }
}

// proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidget * view, QTreeWidgetItem * after, int type);
impl<'a> /*trait*/ QTreeWidgetItem_NewQTreeWidgetItem for (&'a mut QTreeWidget, &'a mut QTreeWidgetItem, i32) {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EP11QTreeWidgetPS_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN15QTreeWidgetItemC1EP11QTreeWidgetPS_i(qthis, arg0, arg1, arg2)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setTextAlignment<T: QTreeWidgetItem_setTextAlignment>(&mut self, value: T) -> i32 {
    value.setTextAlignment(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setTextAlignment {
  fn setTextAlignment(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setTextAlignment(int column, int alignment);
impl<'a> /*trait*/ QTreeWidgetItem_setTextAlignment for (i32, i32) {
  fn setTextAlignment(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem16setTextAlignmentEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN15QTreeWidgetItem16setTextAlignmentEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn insertChild<T: QTreeWidgetItem_insertChild>(&mut self, value: T) -> i32 {
    value.insertChild(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_insertChild {
  fn insertChild(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::insertChild(int index, QTreeWidgetItem * child);
impl<'a> /*trait*/ QTreeWidgetItem_insertChild for (i32, &'a mut QTreeWidgetItem) {
  fn insertChild(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11insertChildEiPS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN15QTreeWidgetItem11insertChildEiPS_(arg0, arg1)};
    return 1;
  }
}

// proto: void QTreeWidgetItem::NewQTreeWidgetItem(const QTreeWidgetItem & other);
impl<'a> /*trait*/ QTreeWidgetItem_NewQTreeWidgetItem for (&'a  QTreeWidgetItem) {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTreeWidgetItemC1ERKS_(qthis, arg0)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn isDisabled<T: QTreeWidgetItem_isDisabled>(&mut self, value: T) -> i32 {
    value.isDisabled(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_isDisabled {
  fn isDisabled(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: bool QTreeWidgetItem::isDisabled();
impl<'a> /*trait*/ QTreeWidgetItem_isDisabled for () {
  fn isDisabled(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10isDisabledEv()};
    unsafe {_ZNK15QTreeWidgetItem10isDisabledEv()};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setText<T: QTreeWidgetItem_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setText {
  fn setText(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setText(int column, const QString & text);
impl<'a> /*trait*/ QTreeWidgetItem_setText for (i32, &'a  QString) {
  fn setText(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem7setTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QTreeWidgetItem7setTextEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setTextColor<T: QTreeWidgetItem_setTextColor>(&mut self, value: T) -> i32 {
    value.setTextColor(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setTextColor {
  fn setTextColor(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setTextColor(int column, const QColor & color);
impl<'a> /*trait*/ QTreeWidgetItem_setTextColor for (i32, &'a  QColor) {
  fn setTextColor(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem12setTextColorEiRK6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QTreeWidgetItem12setTextColorEiRK6QColor(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn sizeHint<T: QTreeWidgetItem_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_sizeHint {
  fn sizeHint(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QSize QTreeWidgetItem::sizeHint(int column);
impl<'a> /*trait*/ QTreeWidgetItem_sizeHint for (i32) {
  fn sizeHint(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem8sizeHintEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QTreeWidgetItem8sizeHintEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn whatsThis<T: QTreeWidgetItem_whatsThis>(&mut self, value: T) -> i32 {
    value.whatsThis(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_whatsThis {
  fn whatsThis(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QString QTreeWidgetItem::whatsThis(int column);
impl<'a> /*trait*/ QTreeWidgetItem_whatsThis for (i32) {
  fn whatsThis(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem9whatsThisEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QTreeWidgetItem9whatsThisEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setWhatsThis<T: QTreeWidgetItem_setWhatsThis>(&mut self, value: T) -> i32 {
    value.setWhatsThis(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setWhatsThis {
  fn setWhatsThis(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setWhatsThis(int column, const QString & whatsThis);
impl<'a> /*trait*/ QTreeWidgetItem_setWhatsThis for (i32, &'a  QString) {
  fn setWhatsThis(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem12setWhatsThisEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QTreeWidgetItem12setWhatsThisEiRK7QString(arg0, arg1)};
    return 1;
  }
}

// proto: void QTreeWidgetItem::NewQTreeWidgetItem(int type);
impl<'a> /*trait*/ QTreeWidgetItem_NewQTreeWidgetItem for (i32) {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QTreeWidgetItemC1Ei(qthis, arg0)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn textColor<T: QTreeWidgetItem_textColor>(&mut self, value: T) -> i32 {
    value.textColor(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_textColor {
  fn textColor(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QColor QTreeWidgetItem::textColor(int column);
impl<'a> /*trait*/ QTreeWidgetItem_textColor for (i32) {
  fn textColor(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem9textColorEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QTreeWidgetItem9textColorEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn icon<T: QTreeWidgetItem_icon>(&mut self, value: T) -> i32 {
    value.icon(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_icon {
  fn icon(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QIcon QTreeWidgetItem::icon(int column);
impl<'a> /*trait*/ QTreeWidgetItem_icon for (i32) {
  fn icon(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem4iconEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QTreeWidgetItem4iconEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setToolTip<T: QTreeWidgetItem_setToolTip>(&mut self, value: T) -> i32 {
    value.setToolTip(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setToolTip {
  fn setToolTip(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setToolTip(int column, const QString & toolTip);
impl<'a> /*trait*/ QTreeWidgetItem_setToolTip for (i32, &'a  QString) {
  fn setToolTip(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem10setToolTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QTreeWidgetItem10setToolTipEiRK7QString(arg0, arg1)};
    return 1;
  }
}

// proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidget * view, const QStringList & strings, int type);
impl<'a> /*trait*/ QTreeWidgetItem_NewQTreeWidgetItem for (&'a mut QTreeWidget, &'a  QStringList, i32) {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EP11QTreeWidgetRK11QStringListi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN15QTreeWidgetItemC1EP11QTreeWidgetRK11QStringListi(qthis, arg0, arg1, arg2)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn isFirstColumnSpanned<T: QTreeWidgetItem_isFirstColumnSpanned>(&mut self, value: T) -> i32 {
    value.isFirstColumnSpanned(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_isFirstColumnSpanned {
  fn isFirstColumnSpanned(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: bool QTreeWidgetItem::isFirstColumnSpanned();
impl<'a> /*trait*/ QTreeWidgetItem_isFirstColumnSpanned for () {
  fn isFirstColumnSpanned(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem20isFirstColumnSpannedEv()};
    unsafe {_ZNK15QTreeWidgetItem20isFirstColumnSpannedEv()};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn textAlignment<T: QTreeWidgetItem_textAlignment>(&mut self, value: T) -> i32 {
    value.textAlignment(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_textAlignment {
  fn textAlignment(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: int QTreeWidgetItem::textAlignment(int column);
impl<'a> /*trait*/ QTreeWidgetItem_textAlignment for (i32) {
  fn textAlignment(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem13textAlignmentEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QTreeWidgetItem13textAlignmentEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn child<T: QTreeWidgetItem_child>(&mut self, value: T) -> i32 {
    value.child(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_child {
  fn child(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QTreeWidgetItem * QTreeWidgetItem::child(int index);
impl<'a> /*trait*/ QTreeWidgetItem_child for (i32) {
  fn child(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem5childEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QTreeWidgetItem5childEi(arg0)};
    return 1;
  }
}

// proto: void QTreeWidgetItem::NewQTreeWidgetItem(const QStringList & strings, int type);
impl<'a> /*trait*/ QTreeWidgetItem_NewQTreeWidgetItem for (&'a  QStringList, i32) {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1ERK11QStringListi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN15QTreeWidgetItemC1ERK11QStringListi(qthis, arg0, arg1)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setSelected<T: QTreeWidgetItem_setSelected>(&mut self, value: T) -> i32 {
    value.setSelected(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setSelected {
  fn setSelected(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setSelected(bool select);
impl<'a> /*trait*/ QTreeWidgetItem_setSelected for (i8) {
  fn setSelected(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11setSelectedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QTreeWidgetItem11setSelectedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn FreeQTreeWidgetItem<T: QTreeWidgetItem_FreeQTreeWidgetItem>(&mut self, value: T) -> i32 {
    value.FreeQTreeWidgetItem(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_FreeQTreeWidgetItem {
  fn FreeQTreeWidgetItem(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::FreeQTreeWidgetItem();
impl<'a> /*trait*/ QTreeWidgetItem_FreeQTreeWidgetItem for () {
  fn FreeQTreeWidgetItem(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemD0Ev()};
    unsafe {_ZN15QTreeWidgetItemD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setHidden<T: QTreeWidgetItem_setHidden>(&mut self, value: T) -> i32 {
    value.setHidden(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setHidden {
  fn setHidden(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setHidden(bool hide);
impl<'a> /*trait*/ QTreeWidgetItem_setHidden for (i8) {
  fn setHidden(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem9setHiddenEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QTreeWidgetItem9setHiddenEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn columnCount<T: QTreeWidgetItem_columnCount>(&mut self, value: T) -> i32 {
    value.columnCount(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_columnCount {
  fn columnCount(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: int QTreeWidgetItem::columnCount();
impl<'a> /*trait*/ QTreeWidgetItem_columnCount for () {
  fn columnCount(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem11columnCountEv()};
    unsafe {_ZNK15QTreeWidgetItem11columnCountEv()};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn takeChild<T: QTreeWidgetItem_takeChild>(&mut self, value: T) -> i32 {
    value.takeChild(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_takeChild {
  fn takeChild(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QTreeWidgetItem * QTreeWidgetItem::takeChild(int index);
impl<'a> /*trait*/ QTreeWidgetItem_takeChild for (i32) {
  fn takeChild(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem9takeChildEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QTreeWidgetItem9takeChildEi(arg0)};
    return 1;
  }
}

// proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidgetItem * parent, const QStringList & strings, int type);
impl<'a> /*trait*/ QTreeWidgetItem_NewQTreeWidgetItem for (&'a mut QTreeWidgetItem, &'a  QStringList, i32) {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EPS_RK11QStringListi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN15QTreeWidgetItemC1EPS_RK11QStringListi(qthis, arg0, arg1, arg2)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setDisabled<T: QTreeWidgetItem_setDisabled>(&mut self, value: T) -> i32 {
    value.setDisabled(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setDisabled {
  fn setDisabled(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setDisabled(bool disabled);
impl<'a> /*trait*/ QTreeWidgetItem_setDisabled for (i8) {
  fn setDisabled(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11setDisabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QTreeWidgetItem11setDisabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setBackground<T: QTreeWidgetItem_setBackground>(&mut self, value: T) -> i32 {
    value.setBackground(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setBackground {
  fn setBackground(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setBackground(int column, const QBrush & brush);
impl<'a> /*trait*/ QTreeWidgetItem_setBackground for (i32, &'a  QBrush) {
  fn setBackground(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem13setBackgroundEiRK6QBrush()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QTreeWidgetItem13setBackgroundEiRK6QBrush(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn addChild<T: QTreeWidgetItem_addChild>(&mut self, value: T) -> i32 {
    value.addChild(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_addChild {
  fn addChild(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::addChild(QTreeWidgetItem * child);
impl<'a> /*trait*/ QTreeWidgetItem_addChild for (&'a mut QTreeWidgetItem) {
  fn addChild(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem8addChildEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QTreeWidgetItem8addChildEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn removeChild<T: QTreeWidgetItem_removeChild>(&mut self, value: T) -> i32 {
    value.removeChild(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_removeChild {
  fn removeChild(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::removeChild(QTreeWidgetItem * child);
impl<'a> /*trait*/ QTreeWidgetItem_removeChild for (&'a mut QTreeWidgetItem) {
  fn removeChild(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11removeChildEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QTreeWidgetItem11removeChildEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn clone<T: QTreeWidgetItem_clone>(&mut self, value: T) -> i32 {
    value.clone(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_clone {
  fn clone(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QTreeWidgetItem * QTreeWidgetItem::clone();
impl<'a> /*trait*/ QTreeWidgetItem_clone for () {
  fn clone(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem5cloneEv()};
    unsafe {_ZNK15QTreeWidgetItem5cloneEv()};
    return 1;
  }
}

// proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidget * view, int type);
impl<'a> /*trait*/ QTreeWidgetItem_NewQTreeWidgetItem for (&'a mut QTreeWidget, i32) {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EP11QTreeWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN15QTreeWidgetItemC1EP11QTreeWidgeti(qthis, arg0, arg1)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setSizeHint<T: QTreeWidgetItem_setSizeHint>(&mut self, value: T) -> i32 {
    value.setSizeHint(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setSizeHint {
  fn setSizeHint(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setSizeHint(int column, const QSize & size);
impl<'a> /*trait*/ QTreeWidgetItem_setSizeHint for (i32, &'a  QSize) {
  fn setSizeHint(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11setSizeHintEiRK5QSize()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QTreeWidgetItem11setSizeHintEiRK5QSize(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn foreground<T: QTreeWidgetItem_foreground>(&mut self, value: T) -> i32 {
    value.foreground(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_foreground {
  fn foreground(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QBrush QTreeWidgetItem::foreground(int column);
impl<'a> /*trait*/ QTreeWidgetItem_foreground for (i32) {
  fn foreground(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10foregroundEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QTreeWidgetItem10foregroundEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn childCount<T: QTreeWidgetItem_childCount>(&mut self, value: T) -> i32 {
    value.childCount(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_childCount {
  fn childCount(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: int QTreeWidgetItem::childCount();
impl<'a> /*trait*/ QTreeWidgetItem_childCount for () {
  fn childCount(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10childCountEv()};
    unsafe {_ZNK15QTreeWidgetItem10childCountEv()};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setBackgroundColor<T: QTreeWidgetItem_setBackgroundColor>(&mut self, value: T) -> i32 {
    value.setBackgroundColor(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setBackgroundColor {
  fn setBackgroundColor(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setBackgroundColor(int column, const QColor & color);
impl<'a> /*trait*/ QTreeWidgetItem_setBackgroundColor for (i32, &'a  QColor) {
  fn setBackgroundColor(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem18setBackgroundColorEiRK6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QTreeWidgetItem18setBackgroundColorEiRK6QColor(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn statusTip<T: QTreeWidgetItem_statusTip>(&mut self, value: T) -> i32 {
    value.statusTip(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_statusTip {
  fn statusTip(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QString QTreeWidgetItem::statusTip(int column);
impl<'a> /*trait*/ QTreeWidgetItem_statusTip for (i32) {
  fn statusTip(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem9statusTipEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QTreeWidgetItem9statusTipEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn background<T: QTreeWidgetItem_background>(&mut self, value: T) -> i32 {
    value.background(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_background {
  fn background(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QBrush QTreeWidgetItem::background(int column);
impl<'a> /*trait*/ QTreeWidgetItem_background for (i32) {
  fn background(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10backgroundEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QTreeWidgetItem10backgroundEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn type_<T: QTreeWidgetItem_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_type_ {
  fn type_(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: int QTreeWidgetItem::type_();
impl<'a> /*trait*/ QTreeWidgetItem_type_ for () {
  fn type_(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem4typeEv()};
    unsafe {_ZNK15QTreeWidgetItem4typeEv()};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn treeWidget<T: QTreeWidgetItem_treeWidget>(&mut self, value: T) -> i32 {
    value.treeWidget(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_treeWidget {
  fn treeWidget(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: QTreeWidget * QTreeWidgetItem::treeWidget();
impl<'a> /*trait*/ QTreeWidgetItem_treeWidget for () {
  fn treeWidget(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10treeWidgetEv()};
    unsafe {_ZNK15QTreeWidgetItem10treeWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn read<T: QTreeWidgetItem_read>(&mut self, value: T) -> i32 {
    value.read(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_read {
  fn read(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::read(QDataStream & in);
impl<'a> /*trait*/ QTreeWidgetItem_read for (&'a mut QDataStream) {
  fn read(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QTreeWidgetItem4readER11QDataStream(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setForeground<T: QTreeWidgetItem_setForeground>(&mut self, value: T) -> i32 {
    value.setForeground(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_setForeground {
  fn setForeground(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: void QTreeWidgetItem::setForeground(int column, const QBrush & brush);
impl<'a> /*trait*/ QTreeWidgetItem_setForeground for (i32, &'a  QBrush) {
  fn setForeground(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem13setForegroundEiRK6QBrush()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN15QTreeWidgetItem13setForegroundEiRK6QBrush(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn isSelected<T: QTreeWidgetItem_isSelected>(&mut self, value: T) -> i32 {
    value.isSelected(self);
    return 1;
  }
}

pub trait QTreeWidgetItem_isSelected {
  fn isSelected(self, this: &mut QTreeWidgetItem) -> i32;
}

// proto: bool QTreeWidgetItem::isSelected();
impl<'a> /*trait*/ QTreeWidgetItem_isSelected for () {
  fn isSelected(self, this: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10isSelectedEv()};
    unsafe {_ZNK15QTreeWidgetItem10isSelectedEv()};
    return 1;
  }
}

