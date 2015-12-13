// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qvariant::QVariant;
use super::qfont::QFont;
use super::qstring::QString;
use super::qdatastream::QDataStream;
use super::qicon::QIcon;
use super::qcolor::QColor;
use super::qtreewidget::QTreeWidget;
use super::qsize::QSize;
use super::qstringlist::QStringList;
use super::qbrush::QBrush;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTreeWidgetItem::setFirstColumnSpanned(bool span);
  fn _ZN15QTreeWidgetItem21setFirstColumnSpannedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QTreeWidgetItem::indexOfChild(QTreeWidgetItem * child);
  fn _ZNK15QTreeWidgetItem12indexOfChildEPS_(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QVariant QTreeWidgetItem::data(int column, int role);
  fn _ZNK15QTreeWidgetItem4dataEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QTreeWidgetItem * QTreeWidgetItem::parent();
  fn _ZNK15QTreeWidgetItem6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidgetItem::setFont(int column, const QFont & font);
  fn _ZN15QTreeWidgetItem7setFontEiRK5QFont(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTreeWidgetItem::setData(int column, int role, const QVariant & value);
  fn _ZN15QTreeWidgetItem7setDataEiiRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  QFont QTreeWidgetItem::font(int column);
  fn _ZNK15QTreeWidgetItem4fontEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTreeWidgetItem::setStatusTip(int column, const QString & statusTip);
  fn _ZN15QTreeWidgetItem12setStatusTipEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTreeWidgetItem::setExpanded(bool expand);
  fn _ZN15QTreeWidgetItem11setExpandedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTreeWidgetItem::write(QDataStream & out);
  fn _ZNK15QTreeWidgetItem5writeER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTreeWidgetItem::isExpanded();
  fn _ZNK15QTreeWidgetItem10isExpandedEv(qthis: *mut c_void) -> int8_t;
  // proto:  QList<QTreeWidgetItem *> QTreeWidgetItem::takeChildren();
  fn _ZN15QTreeWidgetItem12takeChildrenEv(qthis: *mut c_void) ;
  // proto:  void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidgetItem * parent, int type);
  fn _ZN15QTreeWidgetItemC1EPS_i(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QTreeWidgetItem::setIcon(int column, const QIcon & icon);
  fn _ZN15QTreeWidgetItem7setIconEiRK5QIcon(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidgetItem * parent, QTreeWidgetItem * after, int type);
  fn _ZN15QTreeWidgetItemC1EPS_S0_i(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) ;
  // proto:  QString QTreeWidgetItem::toolTip(int column);
  fn _ZNK15QTreeWidgetItem7toolTipEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QColor QTreeWidgetItem::backgroundColor(int column);
  fn _ZNK15QTreeWidgetItem15backgroundColorEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QString QTreeWidgetItem::text(int column);
  fn _ZNK15QTreeWidgetItem4textEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QTreeWidgetItem::isHidden();
  fn _ZNK15QTreeWidgetItem8isHiddenEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidget * view, QTreeWidgetItem * after, int type);
  fn _ZN15QTreeWidgetItemC1EP11QTreeWidgetPS_i(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) ;
  // proto:  void QTreeWidgetItem::setTextAlignment(int column, int alignment);
  fn _ZN15QTreeWidgetItem16setTextAlignmentEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTreeWidgetItem::insertChild(int index, QTreeWidgetItem * child);
  fn _ZN15QTreeWidgetItem11insertChildEiPS_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTreeWidgetItem::NewQTreeWidgetItem(const QTreeWidgetItem & other);
  fn _ZN15QTreeWidgetItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTreeWidgetItem::isDisabled();
  fn _ZNK15QTreeWidgetItem10isDisabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTreeWidgetItem::setText(int column, const QString & text);
  fn _ZN15QTreeWidgetItem7setTextEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTreeWidgetItem::setTextColor(int column, const QColor & color);
  fn _ZN15QTreeWidgetItem12setTextColorEiRK6QColor(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QSize QTreeWidgetItem::sizeHint(int column);
  fn _ZNK15QTreeWidgetItem8sizeHintEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QString QTreeWidgetItem::whatsThis(int column);
  fn _ZNK15QTreeWidgetItem9whatsThisEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTreeWidgetItem::setWhatsThis(int column, const QString & whatsThis);
  fn _ZN15QTreeWidgetItem12setWhatsThisEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTreeWidgetItem::NewQTreeWidgetItem(int type);
  fn _ZN15QTreeWidgetItemC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QColor QTreeWidgetItem::textColor(int column);
  fn _ZNK15QTreeWidgetItem9textColorEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QIcon QTreeWidgetItem::icon(int column);
  fn _ZNK15QTreeWidgetItem4iconEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTreeWidgetItem::setToolTip(int column, const QString & toolTip);
  fn _ZN15QTreeWidgetItem10setToolTipEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidget * view, const QStringList & strings, int type);
  fn _ZN15QTreeWidgetItemC1EP11QTreeWidgetRK11QStringListi(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) ;
  // proto:  bool QTreeWidgetItem::isFirstColumnSpanned();
  fn _ZNK15QTreeWidgetItem20isFirstColumnSpannedEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTreeWidgetItem::textAlignment(int column);
  fn _ZNK15QTreeWidgetItem13textAlignmentEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QTreeWidgetItem * QTreeWidgetItem::child(int index);
  fn _ZNK15QTreeWidgetItem5childEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTreeWidgetItem::NewQTreeWidgetItem(const QStringList & strings, int type);
  fn _ZN15QTreeWidgetItemC1ERK11QStringListi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QTreeWidgetItem::setSelected(bool select);
  fn _ZN15QTreeWidgetItem11setSelectedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTreeWidgetItem::FreeQTreeWidgetItem();
  fn _ZN15QTreeWidgetItemD0Ev(qthis: *mut c_void) ;
  // proto:  void QTreeWidgetItem::setHidden(bool hide);
  fn _ZN15QTreeWidgetItem9setHiddenEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QTreeWidgetItem::columnCount();
  fn _ZNK15QTreeWidgetItem11columnCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QTreeWidgetItem * QTreeWidgetItem::takeChild(int index);
  fn _ZN15QTreeWidgetItem9takeChildEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidgetItem * parent, const QStringList & strings, int type);
  fn _ZN15QTreeWidgetItemC1EPS_RK11QStringListi(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) ;
  // proto:  void QTreeWidgetItem::setDisabled(bool disabled);
  fn _ZN15QTreeWidgetItem11setDisabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTreeWidgetItem::setBackground(int column, const QBrush & brush);
  fn _ZN15QTreeWidgetItem13setBackgroundEiRK6QBrush(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTreeWidgetItem::addChild(QTreeWidgetItem * child);
  fn _ZN15QTreeWidgetItem8addChildEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeWidgetItem::removeChild(QTreeWidgetItem * child);
  fn _ZN15QTreeWidgetItem11removeChildEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTreeWidgetItem * QTreeWidgetItem::clone();
  fn _ZNK15QTreeWidgetItem5cloneEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidget * view, int type);
  fn _ZN15QTreeWidgetItemC1EP11QTreeWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QTreeWidgetItem::setSizeHint(int column, const QSize & size);
  fn _ZN15QTreeWidgetItem11setSizeHintEiRK5QSize(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QBrush QTreeWidgetItem::foreground(int column);
  fn _ZNK15QTreeWidgetItem10foregroundEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QTreeWidgetItem::childCount();
  fn _ZNK15QTreeWidgetItem10childCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTreeWidgetItem::setBackgroundColor(int column, const QColor & color);
  fn _ZN15QTreeWidgetItem18setBackgroundColorEiRK6QColor(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QString QTreeWidgetItem::statusTip(int column);
  fn _ZNK15QTreeWidgetItem9statusTipEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QBrush QTreeWidgetItem::background(int column);
  fn _ZNK15QTreeWidgetItem10backgroundEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QTreeWidgetItem::type_();
  fn _ZNK15QTreeWidgetItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  QTreeWidget * QTreeWidgetItem::treeWidget();
  fn _ZNK15QTreeWidgetItem10treeWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidgetItem::read(QDataStream & in);
  fn _ZN15QTreeWidgetItem4readER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeWidgetItem::setForeground(int column, const QBrush & brush);
  fn _ZN15QTreeWidgetItem13setForegroundEiRK6QBrush(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  bool QTreeWidgetItem::isSelected();
  fn _ZNK15QTreeWidgetItem10isSelectedEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QTreeWidgetItem)=1
pub struct QTreeWidgetItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setFirstColumnSpanned<T: QTreeWidgetItem_setFirstColumnSpanned>(&mut self, value: T)  {
     value.setFirstColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setFirstColumnSpanned {
  fn setFirstColumnSpanned(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setFirstColumnSpanned(bool span);
impl<'a> /*trait*/ QTreeWidgetItem_setFirstColumnSpanned for (i8) {
  fn setFirstColumnSpanned(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem21setFirstColumnSpannedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTreeWidgetItem21setFirstColumnSpannedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn indexOfChild<T: QTreeWidgetItem_indexOfChild>(&mut self, value: T) -> i32 {
    return value.indexOfChild(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_indexOfChild {
  fn indexOfChild(self, rsthis: &mut QTreeWidgetItem) -> i32;
}

// proto:  int QTreeWidgetItem::indexOfChild(QTreeWidgetItem * child);
impl<'a> /*trait*/ QTreeWidgetItem_indexOfChild for (&'a mut QTreeWidgetItem) {
  fn indexOfChild(self, rsthis: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem12indexOfChildEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem12indexOfChildEPS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn data<T: QTreeWidgetItem_data>(&mut self, value: T) -> QVariant {
    return value.data(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_data {
  fn data(self, rsthis: &mut QTreeWidgetItem) -> QVariant;
}

// proto:  QVariant QTreeWidgetItem::data(int column, int role);
impl<'a> /*trait*/ QTreeWidgetItem_data for (i32, i32) {
  fn data(self, rsthis: &mut QTreeWidgetItem) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem4dataEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem4dataEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn parent<T: QTreeWidgetItem_parent>(&mut self, value: T) -> QTreeWidgetItem {
    return value.parent(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_parent {
  fn parent(self, rsthis: &mut QTreeWidgetItem) -> QTreeWidgetItem;
}

// proto:  QTreeWidgetItem * QTreeWidgetItem::parent();
impl<'a> /*trait*/ QTreeWidgetItem_parent for () {
  fn parent(self, rsthis: &mut QTreeWidgetItem) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem6parentEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem6parentEv(rsthis.qclsinst)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setFont<T: QTreeWidgetItem_setFont>(&mut self, value: T)  {
     value.setFont(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setFont {
  fn setFont(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setFont(int column, const QFont & font);
impl<'a> /*trait*/ QTreeWidgetItem_setFont for (i32, &'a  QFont) {
  fn setFont(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem7setFontEiRK5QFont()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem7setFontEiRK5QFont(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setData<T: QTreeWidgetItem_setData>(&mut self, value: T)  {
     value.setData(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setData {
  fn setData(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setData(int column, int role, const QVariant & value);
impl<'a> /*trait*/ QTreeWidgetItem_setData for (i32, i32, &'a  QVariant) {
  fn setData(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem7setDataEiiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem7setDataEiiRK8QVariant(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn font<T: QTreeWidgetItem_font>(&mut self, value: T) -> QFont {
    return value.font(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_font {
  fn font(self, rsthis: &mut QTreeWidgetItem) -> QFont;
}

// proto:  QFont QTreeWidgetItem::font(int column);
impl<'a> /*trait*/ QTreeWidgetItem_font for (i32) {
  fn font(self, rsthis: &mut QTreeWidgetItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem4fontEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem4fontEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setStatusTip<T: QTreeWidgetItem_setStatusTip>(&mut self, value: T)  {
     value.setStatusTip(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setStatusTip {
  fn setStatusTip(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setStatusTip(int column, const QString & statusTip);
impl<'a> /*trait*/ QTreeWidgetItem_setStatusTip for (i32, &'a  QString) {
  fn setStatusTip(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem12setStatusTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem12setStatusTipEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setExpanded<T: QTreeWidgetItem_setExpanded>(&mut self, value: T)  {
     value.setExpanded(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setExpanded {
  fn setExpanded(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setExpanded(bool expand);
impl<'a> /*trait*/ QTreeWidgetItem_setExpanded for (i8) {
  fn setExpanded(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11setExpandedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTreeWidgetItem11setExpandedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn write<T: QTreeWidgetItem_write>(&mut self, value: T)  {
     value.write(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_write {
  fn write(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::write(QDataStream & out);
impl<'a> /*trait*/ QTreeWidgetItem_write for (&'a mut QDataStream) {
  fn write(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK15QTreeWidgetItem5writeER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn isExpanded<T: QTreeWidgetItem_isExpanded>(&mut self, value: T) -> i8 {
    return value.isExpanded(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_isExpanded {
  fn isExpanded(self, rsthis: &mut QTreeWidgetItem) -> i8;
}

// proto:  bool QTreeWidgetItem::isExpanded();
impl<'a> /*trait*/ QTreeWidgetItem_isExpanded for () {
  fn isExpanded(self, rsthis: &mut QTreeWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10isExpandedEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem10isExpandedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn takeChildren<T: QTreeWidgetItem_takeChildren>(&mut self, value: T)  {
     value.takeChildren(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_takeChildren {
  fn takeChildren(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  QList<QTreeWidgetItem *> QTreeWidgetItem::takeChildren();
impl<'a> /*trait*/ QTreeWidgetItem_takeChildren for () {
  fn takeChildren(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem12takeChildrenEv()};
     unsafe {_ZN15QTreeWidgetItem12takeChildrenEv(rsthis.qclsinst)};
    // return 1;
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
  pub fn setIcon<T: QTreeWidgetItem_setIcon>(&mut self, value: T)  {
     value.setIcon(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setIcon {
  fn setIcon(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setIcon(int column, const QIcon & icon);
impl<'a> /*trait*/ QTreeWidgetItem_setIcon for (i32, &'a  QIcon) {
  fn setIcon(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem7setIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem7setIconEiRK5QIcon(rsthis.qclsinst, arg0, arg1)};
    // return 1;
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
  pub fn toolTip<T: QTreeWidgetItem_toolTip>(&mut self, value: T) -> QString {
    return value.toolTip(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_toolTip {
  fn toolTip(self, rsthis: &mut QTreeWidgetItem) -> QString;
}

// proto:  QString QTreeWidgetItem::toolTip(int column);
impl<'a> /*trait*/ QTreeWidgetItem_toolTip for (i32) {
  fn toolTip(self, rsthis: &mut QTreeWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem7toolTipEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem7toolTipEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn backgroundColor<T: QTreeWidgetItem_backgroundColor>(&mut self, value: T) -> QColor {
    return value.backgroundColor(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_backgroundColor {
  fn backgroundColor(self, rsthis: &mut QTreeWidgetItem) -> QColor;
}

// proto:  QColor QTreeWidgetItem::backgroundColor(int column);
impl<'a> /*trait*/ QTreeWidgetItem_backgroundColor for (i32) {
  fn backgroundColor(self, rsthis: &mut QTreeWidgetItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem15backgroundColorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem15backgroundColorEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn text<T: QTreeWidgetItem_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_text {
  fn text(self, rsthis: &mut QTreeWidgetItem) -> QString;
}

// proto:  QString QTreeWidgetItem::text(int column);
impl<'a> /*trait*/ QTreeWidgetItem_text for (i32) {
  fn text(self, rsthis: &mut QTreeWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem4textEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem4textEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn isHidden<T: QTreeWidgetItem_isHidden>(&mut self, value: T) -> i8 {
    return value.isHidden(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_isHidden {
  fn isHidden(self, rsthis: &mut QTreeWidgetItem) -> i8;
}

// proto:  bool QTreeWidgetItem::isHidden();
impl<'a> /*trait*/ QTreeWidgetItem_isHidden for () {
  fn isHidden(self, rsthis: &mut QTreeWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem8isHiddenEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem8isHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
  pub fn setTextAlignment<T: QTreeWidgetItem_setTextAlignment>(&mut self, value: T)  {
     value.setTextAlignment(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setTextAlignment {
  fn setTextAlignment(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setTextAlignment(int column, int alignment);
impl<'a> /*trait*/ QTreeWidgetItem_setTextAlignment for (i32, i32) {
  fn setTextAlignment(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem16setTextAlignmentEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN15QTreeWidgetItem16setTextAlignmentEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn insertChild<T: QTreeWidgetItem_insertChild>(&mut self, value: T)  {
     value.insertChild(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_insertChild {
  fn insertChild(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::insertChild(int index, QTreeWidgetItem * child);
impl<'a> /*trait*/ QTreeWidgetItem_insertChild for (i32, &'a mut QTreeWidgetItem) {
  fn insertChild(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11insertChildEiPS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem11insertChildEiPS_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto: void QTreeWidgetItem::NewQTreeWidgetItem(const QTreeWidgetItem & other);
impl<'a> /*trait*/ QTreeWidgetItem_NewQTreeWidgetItem for (&'a  QTreeWidgetItem) {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QTreeWidgetItemC1ERKS_(qthis, arg0)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn isDisabled<T: QTreeWidgetItem_isDisabled>(&mut self, value: T) -> i8 {
    return value.isDisabled(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_isDisabled {
  fn isDisabled(self, rsthis: &mut QTreeWidgetItem) -> i8;
}

// proto:  bool QTreeWidgetItem::isDisabled();
impl<'a> /*trait*/ QTreeWidgetItem_isDisabled for () {
  fn isDisabled(self, rsthis: &mut QTreeWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10isDisabledEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem10isDisabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setText<T: QTreeWidgetItem_setText>(&mut self, value: T)  {
     value.setText(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setText {
  fn setText(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setText(int column, const QString & text);
impl<'a> /*trait*/ QTreeWidgetItem_setText for (i32, &'a  QString) {
  fn setText(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem7setTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem7setTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setTextColor<T: QTreeWidgetItem_setTextColor>(&mut self, value: T)  {
     value.setTextColor(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setTextColor {
  fn setTextColor(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setTextColor(int column, const QColor & color);
impl<'a> /*trait*/ QTreeWidgetItem_setTextColor for (i32, &'a  QColor) {
  fn setTextColor(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem12setTextColorEiRK6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem12setTextColorEiRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn sizeHint<T: QTreeWidgetItem_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_sizeHint {
  fn sizeHint(self, rsthis: &mut QTreeWidgetItem) -> QSize;
}

// proto:  QSize QTreeWidgetItem::sizeHint(int column);
impl<'a> /*trait*/ QTreeWidgetItem_sizeHint for (i32) {
  fn sizeHint(self, rsthis: &mut QTreeWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem8sizeHintEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem8sizeHintEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn whatsThis<T: QTreeWidgetItem_whatsThis>(&mut self, value: T) -> QString {
    return value.whatsThis(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_whatsThis {
  fn whatsThis(self, rsthis: &mut QTreeWidgetItem) -> QString;
}

// proto:  QString QTreeWidgetItem::whatsThis(int column);
impl<'a> /*trait*/ QTreeWidgetItem_whatsThis for (i32) {
  fn whatsThis(self, rsthis: &mut QTreeWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem9whatsThisEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem9whatsThisEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setWhatsThis<T: QTreeWidgetItem_setWhatsThis>(&mut self, value: T)  {
     value.setWhatsThis(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setWhatsThis {
  fn setWhatsThis(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setWhatsThis(int column, const QString & whatsThis);
impl<'a> /*trait*/ QTreeWidgetItem_setWhatsThis for (i32, &'a  QString) {
  fn setWhatsThis(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem12setWhatsThisEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem12setWhatsThisEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
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
  pub fn textColor<T: QTreeWidgetItem_textColor>(&mut self, value: T) -> QColor {
    return value.textColor(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_textColor {
  fn textColor(self, rsthis: &mut QTreeWidgetItem) -> QColor;
}

// proto:  QColor QTreeWidgetItem::textColor(int column);
impl<'a> /*trait*/ QTreeWidgetItem_textColor for (i32) {
  fn textColor(self, rsthis: &mut QTreeWidgetItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem9textColorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem9textColorEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn icon<T: QTreeWidgetItem_icon>(&mut self, value: T) -> QIcon {
    return value.icon(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_icon {
  fn icon(self, rsthis: &mut QTreeWidgetItem) -> QIcon;
}

// proto:  QIcon QTreeWidgetItem::icon(int column);
impl<'a> /*trait*/ QTreeWidgetItem_icon for (i32) {
  fn icon(self, rsthis: &mut QTreeWidgetItem) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem4iconEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem4iconEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setToolTip<T: QTreeWidgetItem_setToolTip>(&mut self, value: T)  {
     value.setToolTip(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setToolTip {
  fn setToolTip(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setToolTip(int column, const QString & toolTip);
impl<'a> /*trait*/ QTreeWidgetItem_setToolTip for (i32, &'a  QString) {
  fn setToolTip(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem10setToolTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem10setToolTipEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidget * view, const QStringList & strings, int type);
impl<'a> /*trait*/ QTreeWidgetItem_NewQTreeWidgetItem for (&'a mut QTreeWidget, &'a  QStringList, i32) {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EP11QTreeWidgetRK11QStringListi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN15QTreeWidgetItemC1EP11QTreeWidgetRK11QStringListi(qthis, arg0, arg1, arg2)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn isFirstColumnSpanned<T: QTreeWidgetItem_isFirstColumnSpanned>(&mut self, value: T) -> i8 {
    return value.isFirstColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_isFirstColumnSpanned {
  fn isFirstColumnSpanned(self, rsthis: &mut QTreeWidgetItem) -> i8;
}

// proto:  bool QTreeWidgetItem::isFirstColumnSpanned();
impl<'a> /*trait*/ QTreeWidgetItem_isFirstColumnSpanned for () {
  fn isFirstColumnSpanned(self, rsthis: &mut QTreeWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem20isFirstColumnSpannedEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem20isFirstColumnSpannedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn textAlignment<T: QTreeWidgetItem_textAlignment>(&mut self, value: T) -> i32 {
    return value.textAlignment(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_textAlignment {
  fn textAlignment(self, rsthis: &mut QTreeWidgetItem) -> i32;
}

// proto:  int QTreeWidgetItem::textAlignment(int column);
impl<'a> /*trait*/ QTreeWidgetItem_textAlignment for (i32) {
  fn textAlignment(self, rsthis: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem13textAlignmentEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem13textAlignmentEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn child<T: QTreeWidgetItem_child>(&mut self, value: T) -> QTreeWidgetItem {
    return value.child(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_child {
  fn child(self, rsthis: &mut QTreeWidgetItem) -> QTreeWidgetItem;
}

// proto:  QTreeWidgetItem * QTreeWidgetItem::child(int index);
impl<'a> /*trait*/ QTreeWidgetItem_child for (i32) {
  fn child(self, rsthis: &mut QTreeWidgetItem) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem5childEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem5childEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTreeWidgetItem::NewQTreeWidgetItem(const QStringList & strings, int type);
impl<'a> /*trait*/ QTreeWidgetItem_NewQTreeWidgetItem for (&'a  QStringList, i32) {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1ERK11QStringListi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN15QTreeWidgetItemC1ERK11QStringListi(qthis, arg0, arg1)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setSelected<T: QTreeWidgetItem_setSelected>(&mut self, value: T)  {
     value.setSelected(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setSelected {
  fn setSelected(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setSelected(bool select);
impl<'a> /*trait*/ QTreeWidgetItem_setSelected for (i8) {
  fn setSelected(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11setSelectedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTreeWidgetItem11setSelectedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn FreeQTreeWidgetItem<T: QTreeWidgetItem_FreeQTreeWidgetItem>(&mut self, value: T)  {
     value.FreeQTreeWidgetItem(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_FreeQTreeWidgetItem {
  fn FreeQTreeWidgetItem(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::FreeQTreeWidgetItem();
impl<'a> /*trait*/ QTreeWidgetItem_FreeQTreeWidgetItem for () {
  fn FreeQTreeWidgetItem(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemD0Ev()};
     unsafe {_ZN15QTreeWidgetItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setHidden<T: QTreeWidgetItem_setHidden>(&mut self, value: T)  {
     value.setHidden(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setHidden {
  fn setHidden(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setHidden(bool hide);
impl<'a> /*trait*/ QTreeWidgetItem_setHidden for (i8) {
  fn setHidden(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem9setHiddenEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTreeWidgetItem9setHiddenEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn columnCount<T: QTreeWidgetItem_columnCount>(&mut self, value: T) -> i32 {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_columnCount {
  fn columnCount(self, rsthis: &mut QTreeWidgetItem) -> i32;
}

// proto:  int QTreeWidgetItem::columnCount();
impl<'a> /*trait*/ QTreeWidgetItem_columnCount for () {
  fn columnCount(self, rsthis: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem11columnCountEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn takeChild<T: QTreeWidgetItem_takeChild>(&mut self, value: T) -> QTreeWidgetItem {
    return value.takeChild(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_takeChild {
  fn takeChild(self, rsthis: &mut QTreeWidgetItem) -> QTreeWidgetItem;
}

// proto:  QTreeWidgetItem * QTreeWidgetItem::takeChild(int index);
impl<'a> /*trait*/ QTreeWidgetItem_takeChild for (i32) {
  fn takeChild(self, rsthis: &mut QTreeWidgetItem) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem9takeChildEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN15QTreeWidgetItem9takeChildEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTreeWidgetItem::NewQTreeWidgetItem(QTreeWidgetItem * parent, const QStringList & strings, int type);
impl<'a> /*trait*/ QTreeWidgetItem_NewQTreeWidgetItem for (&'a mut QTreeWidgetItem, &'a  QStringList, i32) {
  fn NewQTreeWidgetItem(self) -> QTreeWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EPS_RK11QStringListi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN15QTreeWidgetItemC1EPS_RK11QStringListi(qthis, arg0, arg1, arg2)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setDisabled<T: QTreeWidgetItem_setDisabled>(&mut self, value: T)  {
     value.setDisabled(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setDisabled {
  fn setDisabled(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setDisabled(bool disabled);
impl<'a> /*trait*/ QTreeWidgetItem_setDisabled for (i8) {
  fn setDisabled(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11setDisabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTreeWidgetItem11setDisabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setBackground<T: QTreeWidgetItem_setBackground>(&mut self, value: T)  {
     value.setBackground(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setBackground {
  fn setBackground(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setBackground(int column, const QBrush & brush);
impl<'a> /*trait*/ QTreeWidgetItem_setBackground for (i32, &'a  QBrush) {
  fn setBackground(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem13setBackgroundEiRK6QBrush()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem13setBackgroundEiRK6QBrush(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn addChild<T: QTreeWidgetItem_addChild>(&mut self, value: T)  {
     value.addChild(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_addChild {
  fn addChild(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::addChild(QTreeWidgetItem * child);
impl<'a> /*trait*/ QTreeWidgetItem_addChild for (&'a mut QTreeWidgetItem) {
  fn addChild(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem8addChildEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem8addChildEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn removeChild<T: QTreeWidgetItem_removeChild>(&mut self, value: T)  {
     value.removeChild(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_removeChild {
  fn removeChild(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::removeChild(QTreeWidgetItem * child);
impl<'a> /*trait*/ QTreeWidgetItem_removeChild for (&'a mut QTreeWidgetItem) {
  fn removeChild(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11removeChildEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem11removeChildEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn clone<T: QTreeWidgetItem_clone>(&mut self, value: T) -> QTreeWidgetItem {
    return value.clone(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_clone {
  fn clone(self, rsthis: &mut QTreeWidgetItem) -> QTreeWidgetItem;
}

// proto:  QTreeWidgetItem * QTreeWidgetItem::clone();
impl<'a> /*trait*/ QTreeWidgetItem_clone for () {
  fn clone(self, rsthis: &mut QTreeWidgetItem) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem5cloneEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem5cloneEv(rsthis.qclsinst)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn setSizeHint<T: QTreeWidgetItem_setSizeHint>(&mut self, value: T)  {
     value.setSizeHint(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setSizeHint {
  fn setSizeHint(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setSizeHint(int column, const QSize & size);
impl<'a> /*trait*/ QTreeWidgetItem_setSizeHint for (i32, &'a  QSize) {
  fn setSizeHint(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11setSizeHintEiRK5QSize()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem11setSizeHintEiRK5QSize(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn foreground<T: QTreeWidgetItem_foreground>(&mut self, value: T) -> QBrush {
    return value.foreground(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_foreground {
  fn foreground(self, rsthis: &mut QTreeWidgetItem) -> QBrush;
}

// proto:  QBrush QTreeWidgetItem::foreground(int column);
impl<'a> /*trait*/ QTreeWidgetItem_foreground for (i32) {
  fn foreground(self, rsthis: &mut QTreeWidgetItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10foregroundEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem10foregroundEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn childCount<T: QTreeWidgetItem_childCount>(&mut self, value: T) -> i32 {
    return value.childCount(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_childCount {
  fn childCount(self, rsthis: &mut QTreeWidgetItem) -> i32;
}

// proto:  int QTreeWidgetItem::childCount();
impl<'a> /*trait*/ QTreeWidgetItem_childCount for () {
  fn childCount(self, rsthis: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10childCountEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem10childCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setBackgroundColor<T: QTreeWidgetItem_setBackgroundColor>(&mut self, value: T)  {
     value.setBackgroundColor(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setBackgroundColor {
  fn setBackgroundColor(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setBackgroundColor(int column, const QColor & color);
impl<'a> /*trait*/ QTreeWidgetItem_setBackgroundColor for (i32, &'a  QColor) {
  fn setBackgroundColor(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem18setBackgroundColorEiRK6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem18setBackgroundColorEiRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn statusTip<T: QTreeWidgetItem_statusTip>(&mut self, value: T) -> QString {
    return value.statusTip(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_statusTip {
  fn statusTip(self, rsthis: &mut QTreeWidgetItem) -> QString;
}

// proto:  QString QTreeWidgetItem::statusTip(int column);
impl<'a> /*trait*/ QTreeWidgetItem_statusTip for (i32) {
  fn statusTip(self, rsthis: &mut QTreeWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem9statusTipEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem9statusTipEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn background<T: QTreeWidgetItem_background>(&mut self, value: T) -> QBrush {
    return value.background(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_background {
  fn background(self, rsthis: &mut QTreeWidgetItem) -> QBrush;
}

// proto:  QBrush QTreeWidgetItem::background(int column);
impl<'a> /*trait*/ QTreeWidgetItem_background for (i32) {
  fn background(self, rsthis: &mut QTreeWidgetItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10backgroundEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem10backgroundEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn type_<T: QTreeWidgetItem_type_>(&mut self, value: T) -> i32 {
    return value.type_(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_type_ {
  fn type_(self, rsthis: &mut QTreeWidgetItem) -> i32;
}

// proto:  int QTreeWidgetItem::type_();
impl<'a> /*trait*/ QTreeWidgetItem_type_ for () {
  fn type_(self, rsthis: &mut QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem4typeEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn treeWidget<T: QTreeWidgetItem_treeWidget>(&mut self, value: T) -> QTreeWidget {
    return value.treeWidget(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_treeWidget {
  fn treeWidget(self, rsthis: &mut QTreeWidgetItem) -> QTreeWidget;
}

// proto:  QTreeWidget * QTreeWidgetItem::treeWidget();
impl<'a> /*trait*/ QTreeWidgetItem_treeWidget for () {
  fn treeWidget(self, rsthis: &mut QTreeWidgetItem) -> QTreeWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10treeWidgetEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem10treeWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QTreeWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn read<T: QTreeWidgetItem_read>(&mut self, value: T)  {
     value.read(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_read {
  fn read(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::read(QDataStream & in);
impl<'a> /*trait*/ QTreeWidgetItem_read for (&'a mut QDataStream) {
  fn read(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem4readER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn setForeground<T: QTreeWidgetItem_setForeground>(&mut self, value: T)  {
     value.setForeground(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setForeground {
  fn setForeground(self, rsthis: &mut QTreeWidgetItem) ;
}

// proto:  void QTreeWidgetItem::setForeground(int column, const QBrush & brush);
impl<'a> /*trait*/ QTreeWidgetItem_setForeground for (i32, &'a  QBrush) {
  fn setForeground(self, rsthis: &mut QTreeWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem13setForegroundEiRK6QBrush()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem13setForegroundEiRK6QBrush(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn isSelected<T: QTreeWidgetItem_isSelected>(&mut self, value: T) -> i8 {
    return value.isSelected(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_isSelected {
  fn isSelected(self, rsthis: &mut QTreeWidgetItem) -> i8;
}

// proto:  bool QTreeWidgetItem::isSelected();
impl<'a> /*trait*/ QTreeWidgetItem_isSelected for () {
  fn isSelected(self, rsthis: &mut QTreeWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10isSelectedEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem10isSelectedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

