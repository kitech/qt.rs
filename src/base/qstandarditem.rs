// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstandarditemmodel::QStandardItemModel;
use super::qstring::QString;
use super::qbrush::QBrush;
use super::qicon::QIcon;
use super::qmodelindex::QModelIndex;
use super::qvariant::QVariant;
use super::qdatastream::QDataStream;
use super::qfont::QFont;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QStandardItem::setChild(int row, QStandardItem * item);
  fn _ZN13QStandardItem8setChildEiPS_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QStandardItemModel * QStandardItem::model();
  fn _ZNK13QStandardItem5modelEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItem::insertColumns(int column, int count);
  fn _ZN13QStandardItem13insertColumnsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QStandardItem::setSelectable(bool selectable);
  fn _ZN13QStandardItem13setSelectableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QStandardItem::column();
  fn _ZNK13QStandardItem6columnEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QStandardItem::whatsThis();
  fn _ZNK13QStandardItem9whatsThisEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QList<QStandardItem *> QStandardItem::takeColumn(int column);
  fn _ZN13QStandardItem10takeColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QStandardItem::setForeground(const QBrush & brush);
  fn _ZN13QStandardItem13setForegroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QStandardItem::isEditable();
  fn _ZNK13QStandardItem10isEditableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QIcon QStandardItem::icon();
  fn _ZNK13QStandardItem4iconEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItem::setWhatsThis(const QString & whatsThis);
  fn _ZN13QStandardItem12setWhatsThisERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStandardItem * QStandardItem::takeChild(int row, int column);
  fn _ZN13QStandardItem9takeChildEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  int QStandardItem::type_();
  fn _ZNK13QStandardItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  QList<QStandardItem *> QStandardItem::takeRow(int row);
  fn _ZN13QStandardItem7takeRowEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QStandardItem::row();
  fn _ZNK13QStandardItem3rowEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QStandardItem::isCheckable();
  fn _ZNK13QStandardItem11isCheckableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QStandardItem::text();
  fn _ZNK13QStandardItem4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItem::insertRows(int row, int count);
  fn _ZN13QStandardItem10insertRowsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  bool QStandardItem::isDropEnabled();
  fn _ZNK13QStandardItem13isDropEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QStandardItem::hasChildren();
  fn _ZNK13QStandardItem11hasChildrenEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QStandardItem::statusTip();
  fn _ZNK13QStandardItem9statusTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItem::setStatusTip(const QString & statusTip);
  fn _ZN13QStandardItem12setStatusTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItem::appendRow(QStandardItem * item);
  fn _ZN13QStandardItem9appendRowEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItem::setChild(int row, int column, QStandardItem * item);
  fn _ZN13QStandardItem8setChildEiiPS_(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  QModelIndex QStandardItem::index();
  fn _ZNK13QStandardItem5indexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItem::setIcon(const QIcon & icon);
  fn _ZN13QStandardItem7setIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItem::setToolTip(const QString & toolTip);
  fn _ZN13QStandardItem10setToolTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItem::setData(const QVariant & value, int role);
  fn _ZN13QStandardItem7setDataERK8QVarianti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  QBrush QStandardItem::background();
  fn _ZNK13QStandardItem10backgroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QStandardItem::data(int role);
  fn _ZNK13QStandardItem4dataEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QStandardItem::NewQStandardItem(const QStandardItem & other);
  fn _ZN13QStandardItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStandardItem * QStandardItem::child(int row, int column);
  fn _ZNK13QStandardItem5childEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  bool QStandardItem::isSelectable();
  fn _ZNK13QStandardItem12isSelectableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QStandardItem::toolTip();
  fn _ZNK13QStandardItem7toolTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItem::setRowCount(int rows);
  fn _ZN13QStandardItem11setRowCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QStandardItem::NewQStandardItem(const QString & text);
  fn _ZN13QStandardItemC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItem::write(QDataStream & out);
  fn _ZNK13QStandardItem5writeER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QStandardItem::isDragEnabled();
  fn _ZNK13QStandardItem13isDragEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QStandardItem::setAccessibleText(const QString & accessibleText);
  fn _ZN13QStandardItem17setAccessibleTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QStandardItem::rowCount();
  fn _ZNK13QStandardItem8rowCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QStandardItem::removeColumn(int column);
  fn _ZN13QStandardItem12removeColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QStandardItem::removeRow(int row);
  fn _ZN13QStandardItem9removeRowEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QStandardItem::columnCount();
  fn _ZNK13QStandardItem11columnCountEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QStandardItem::isTristate();
  fn _ZNK13QStandardItem10isTristateEv(qthis: *mut c_void) -> int8_t;
  // proto:  QStandardItem * QStandardItem::parent();
  fn _ZNK13QStandardItem6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItem::NewQStandardItem(const QIcon & icon, const QString & text);
  fn _ZN13QStandardItemC1ERK5QIconRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QStandardItem::setFont(const QFont & font);
  fn _ZN13QStandardItem7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItem::removeColumns(int column, int count);
  fn _ZN13QStandardItem13removeColumnsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QStandardItem::FreeQStandardItem();
  fn _ZN13QStandardItemD0Ev(qthis: *mut c_void) ;
  // proto:  void QStandardItem::NewQStandardItem();
  fn _ZN13QStandardItemC1Ev(qthis: *mut c_void) ;
  // proto:  QFont QStandardItem::font();
  fn _ZNK13QStandardItem4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItem::setEditable(bool editable);
  fn _ZN13QStandardItem11setEditableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QStandardItem::setText(const QString & text);
  fn _ZN13QStandardItem7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItem::NewQStandardItem(int rows, int columns);
  fn _ZN13QStandardItemC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  bool QStandardItem::isEnabled();
  fn _ZNK13QStandardItem9isEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QStandardItem::setDropEnabled(bool dropEnabled);
  fn _ZN13QStandardItem14setDropEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QStandardItem::setColumnCount(int columns);
  fn _ZN13QStandardItem14setColumnCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QString QStandardItem::accessibleText();
  fn _ZNK13QStandardItem14accessibleTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItem::read(QDataStream & in);
  fn _ZN13QStandardItem4readER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItem::setCheckable(bool checkable);
  fn _ZN13QStandardItem12setCheckableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QStandardItem::setDragEnabled(bool dragEnabled);
  fn _ZN13QStandardItem14setDragEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QBrush QStandardItem::foreground();
  fn _ZNK13QStandardItem10foregroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QStandardItem * QStandardItem::clone();
  fn _ZNK13QStandardItem5cloneEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItem::removeRows(int row, int count);
  fn _ZN13QStandardItem10removeRowsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QSize QStandardItem::sizeHint();
  fn _ZNK13QStandardItem8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItem::setEnabled(bool enabled);
  fn _ZN13QStandardItem10setEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QStandardItem::setBackground(const QBrush & brush);
  fn _ZN13QStandardItem13setBackgroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItem::setAccessibleDescription(const QString & accessibleDescription);
  fn _ZN13QStandardItem24setAccessibleDescriptionERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStandardItem::setSizeHint(const QSize & sizeHint);
  fn _ZN13QStandardItem11setSizeHintERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QStandardItem::accessibleDescription();
  fn _ZNK13QStandardItem21accessibleDescriptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItem::setTristate(bool tristate);
  fn _ZN13QStandardItem11setTristateEb(qthis: *mut c_void, arg0: int8_t) ;
}

// body block begin
// class sizeof(QStandardItem)=1
pub struct QStandardItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStandardItem {
  pub fn setChild<T: QStandardItem_setChild>(&mut self, value: T)  {
     value.setChild(self);
    // return 1;
  }
}

pub trait QStandardItem_setChild {
  fn setChild(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setChild(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_setChild for (i32, &'a mut QStandardItem) {
  fn setChild(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem8setChildEiPS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem8setChildEiPS_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn model<T: QStandardItem_model>(&mut self, value: T) -> QStandardItemModel {
    return value.model(self);
    // return 1;
  }
}

pub trait QStandardItem_model {
  fn model(self, rsthis: &mut QStandardItem) -> QStandardItemModel;
}

// proto:  QStandardItemModel * QStandardItem::model();
impl<'a> /*trait*/ QStandardItem_model for () {
  fn model(self, rsthis: &mut QStandardItem) -> QStandardItemModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5modelEv()};
    let mut ret = unsafe {_ZNK13QStandardItem5modelEv(rsthis.qclsinst)};
    let mut ret1 = QStandardItemModel{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn insertColumns<T: QStandardItem_insertColumns>(&mut self, value: T)  {
     value.insertColumns(self);
    // return 1;
  }
}

pub trait QStandardItem_insertColumns {
  fn insertColumns(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::insertColumns(int column, int count);
impl<'a> /*trait*/ QStandardItem_insertColumns for (i32, i32) {
  fn insertColumns(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13insertColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QStandardItem13insertColumnsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setSelectable<T: QStandardItem_setSelectable>(&mut self, value: T)  {
     value.setSelectable(self);
    // return 1;
  }
}

pub trait QStandardItem_setSelectable {
  fn setSelectable(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setSelectable(bool selectable);
impl<'a> /*trait*/ QStandardItem_setSelectable for (i8) {
  fn setSelectable(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13setSelectableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem13setSelectableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn column<T: QStandardItem_column>(&mut self, value: T) -> i32 {
    return value.column(self);
    // return 1;
  }
}

pub trait QStandardItem_column {
  fn column(self, rsthis: &mut QStandardItem) -> i32;
}

// proto:  int QStandardItem::column();
impl<'a> /*trait*/ QStandardItem_column for () {
  fn column(self, rsthis: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem6columnEv()};
    let mut ret = unsafe {_ZNK13QStandardItem6columnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn whatsThis<T: QStandardItem_whatsThis>(&mut self, value: T) -> QString {
    return value.whatsThis(self);
    // return 1;
  }
}

pub trait QStandardItem_whatsThis {
  fn whatsThis(self, rsthis: &mut QStandardItem) -> QString;
}

// proto:  QString QStandardItem::whatsThis();
impl<'a> /*trait*/ QStandardItem_whatsThis for () {
  fn whatsThis(self, rsthis: &mut QStandardItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem9whatsThisEv()};
    let mut ret = unsafe {_ZNK13QStandardItem9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn takeColumn<T: QStandardItem_takeColumn>(&mut self, value: T)  {
     value.takeColumn(self);
    // return 1;
  }
}

pub trait QStandardItem_takeColumn {
  fn takeColumn(self, rsthis: &mut QStandardItem) ;
}

// proto:  QList<QStandardItem *> QStandardItem::takeColumn(int column);
impl<'a> /*trait*/ QStandardItem_takeColumn for (i32) {
  fn takeColumn(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10takeColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QStandardItem10takeColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setForeground<T: QStandardItem_setForeground>(&mut self, value: T)  {
     value.setForeground(self);
    // return 1;
  }
}

pub trait QStandardItem_setForeground {
  fn setForeground(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QStandardItem_setForeground for (&'a  QBrush) {
  fn setForeground(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem13setForegroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isEditable<T: QStandardItem_isEditable>(&mut self, value: T) -> i8 {
    return value.isEditable(self);
    // return 1;
  }
}

pub trait QStandardItem_isEditable {
  fn isEditable(self, rsthis: &mut QStandardItem) -> i8;
}

// proto:  bool QStandardItem::isEditable();
impl<'a> /*trait*/ QStandardItem_isEditable for () {
  fn isEditable(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10isEditableEv()};
    let mut ret = unsafe {_ZNK13QStandardItem10isEditableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn icon<T: QStandardItem_icon>(&mut self, value: T) -> QIcon {
    return value.icon(self);
    // return 1;
  }
}

pub trait QStandardItem_icon {
  fn icon(self, rsthis: &mut QStandardItem) -> QIcon;
}

// proto:  QIcon QStandardItem::icon();
impl<'a> /*trait*/ QStandardItem_icon for () {
  fn icon(self, rsthis: &mut QStandardItem) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4iconEv()};
    let mut ret = unsafe {_ZNK13QStandardItem4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setWhatsThis<T: QStandardItem_setWhatsThis>(&mut self, value: T)  {
     value.setWhatsThis(self);
    // return 1;
  }
}

pub trait QStandardItem_setWhatsThis {
  fn setWhatsThis(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setWhatsThis(const QString & whatsThis);
impl<'a> /*trait*/ QStandardItem_setWhatsThis for (&'a  QString) {
  fn setWhatsThis(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn takeChild<T: QStandardItem_takeChild>(&mut self, value: T) -> QStandardItem {
    return value.takeChild(self);
    // return 1;
  }
}

pub trait QStandardItem_takeChild {
  fn takeChild(self, rsthis: &mut QStandardItem) -> QStandardItem;
}

// proto:  QStandardItem * QStandardItem::takeChild(int row, int column);
impl<'a> /*trait*/ QStandardItem_takeChild for (i32, i32) {
  fn takeChild(self, rsthis: &mut QStandardItem) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem9takeChildEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN13QStandardItem9takeChildEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn type_<T: QStandardItem_type_>(&mut self, value: T) -> i32 {
    return value.type_(self);
    // return 1;
  }
}

pub trait QStandardItem_type_ {
  fn type_(self, rsthis: &mut QStandardItem) -> i32;
}

// proto:  int QStandardItem::type_();
impl<'a> /*trait*/ QStandardItem_type_ for () {
  fn type_(self, rsthis: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4typeEv()};
    let mut ret = unsafe {_ZNK13QStandardItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn takeRow<T: QStandardItem_takeRow>(&mut self, value: T)  {
     value.takeRow(self);
    // return 1;
  }
}

pub trait QStandardItem_takeRow {
  fn takeRow(self, rsthis: &mut QStandardItem) ;
}

// proto:  QList<QStandardItem *> QStandardItem::takeRow(int row);
impl<'a> /*trait*/ QStandardItem_takeRow for (i32) {
  fn takeRow(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7takeRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QStandardItem7takeRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn row<T: QStandardItem_row>(&mut self, value: T) -> i32 {
    return value.row(self);
    // return 1;
  }
}

pub trait QStandardItem_row {
  fn row(self, rsthis: &mut QStandardItem) -> i32;
}

// proto:  int QStandardItem::row();
impl<'a> /*trait*/ QStandardItem_row for () {
  fn row(self, rsthis: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem3rowEv()};
    let mut ret = unsafe {_ZNK13QStandardItem3rowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isCheckable<T: QStandardItem_isCheckable>(&mut self, value: T) -> i8 {
    return value.isCheckable(self);
    // return 1;
  }
}

pub trait QStandardItem_isCheckable {
  fn isCheckable(self, rsthis: &mut QStandardItem) -> i8;
}

// proto:  bool QStandardItem::isCheckable();
impl<'a> /*trait*/ QStandardItem_isCheckable for () {
  fn isCheckable(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem11isCheckableEv()};
    let mut ret = unsafe {_ZNK13QStandardItem11isCheckableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn text<T: QStandardItem_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QStandardItem_text {
  fn text(self, rsthis: &mut QStandardItem) -> QString;
}

// proto:  QString QStandardItem::text();
impl<'a> /*trait*/ QStandardItem_text for () {
  fn text(self, rsthis: &mut QStandardItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4textEv()};
    let mut ret = unsafe {_ZNK13QStandardItem4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn insertRows<T: QStandardItem_insertRows>(&mut self, value: T)  {
     value.insertRows(self);
    // return 1;
  }
}

pub trait QStandardItem_insertRows {
  fn insertRows(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::insertRows(int row, int count);
impl<'a> /*trait*/ QStandardItem_insertRows for (i32, i32) {
  fn insertRows(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10insertRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QStandardItem10insertRowsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isDropEnabled<T: QStandardItem_isDropEnabled>(&mut self, value: T) -> i8 {
    return value.isDropEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_isDropEnabled {
  fn isDropEnabled(self, rsthis: &mut QStandardItem) -> i8;
}

// proto:  bool QStandardItem::isDropEnabled();
impl<'a> /*trait*/ QStandardItem_isDropEnabled for () {
  fn isDropEnabled(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem13isDropEnabledEv()};
    let mut ret = unsafe {_ZNK13QStandardItem13isDropEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn hasChildren<T: QStandardItem_hasChildren>(&mut self, value: T) -> i8 {
    return value.hasChildren(self);
    // return 1;
  }
}

pub trait QStandardItem_hasChildren {
  fn hasChildren(self, rsthis: &mut QStandardItem) -> i8;
}

// proto:  bool QStandardItem::hasChildren();
impl<'a> /*trait*/ QStandardItem_hasChildren for () {
  fn hasChildren(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem11hasChildrenEv()};
    let mut ret = unsafe {_ZNK13QStandardItem11hasChildrenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn statusTip<T: QStandardItem_statusTip>(&mut self, value: T) -> QString {
    return value.statusTip(self);
    // return 1;
  }
}

pub trait QStandardItem_statusTip {
  fn statusTip(self, rsthis: &mut QStandardItem) -> QString;
}

// proto:  QString QStandardItem::statusTip();
impl<'a> /*trait*/ QStandardItem_statusTip for () {
  fn statusTip(self, rsthis: &mut QStandardItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem9statusTipEv()};
    let mut ret = unsafe {_ZNK13QStandardItem9statusTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setStatusTip<T: QStandardItem_setStatusTip>(&mut self, value: T)  {
     value.setStatusTip(self);
    // return 1;
  }
}

pub trait QStandardItem_setStatusTip {
  fn setStatusTip(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QStandardItem_setStatusTip for (&'a  QString) {
  fn setStatusTip(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem12setStatusTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn appendRow<T: QStandardItem_appendRow>(&mut self, value: T)  {
     value.appendRow(self);
    // return 1;
  }
}

pub trait QStandardItem_appendRow {
  fn appendRow(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::appendRow(QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_appendRow for (&'a mut QStandardItem) {
  fn appendRow(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem9appendRowEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem9appendRowEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QStandardItem::setChild(int row, int column, QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_setChild for (i32, i32, &'a mut QStandardItem) {
  fn setChild(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem8setChildEiiPS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem8setChildEiiPS_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn index<T: QStandardItem_index>(&mut self, value: T) -> QModelIndex {
    return value.index(self);
    // return 1;
  }
}

pub trait QStandardItem_index {
  fn index(self, rsthis: &mut QStandardItem) -> QModelIndex;
}

// proto:  QModelIndex QStandardItem::index();
impl<'a> /*trait*/ QStandardItem_index for () {
  fn index(self, rsthis: &mut QStandardItem) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5indexEv()};
    let mut ret = unsafe {_ZNK13QStandardItem5indexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setIcon<T: QStandardItem_setIcon>(&mut self, value: T)  {
     value.setIcon(self);
    // return 1;
  }
}

pub trait QStandardItem_setIcon {
  fn setIcon(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QStandardItem_setIcon for (&'a  QIcon) {
  fn setIcon(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setToolTip<T: QStandardItem_setToolTip>(&mut self, value: T)  {
     value.setToolTip(self);
    // return 1;
  }
}

pub trait QStandardItem_setToolTip {
  fn setToolTip(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QStandardItem_setToolTip for (&'a  QString) {
  fn setToolTip(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setData<T: QStandardItem_setData>(&mut self, value: T)  {
     value.setData(self);
    // return 1;
  }
}

pub trait QStandardItem_setData {
  fn setData(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setData(const QVariant & value, int role);
impl<'a> /*trait*/ QStandardItem_setData for (&'a  QVariant, i32) {
  fn setData(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setDataERK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QStandardItem7setDataERK8QVarianti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn background<T: QStandardItem_background>(&mut self, value: T) -> QBrush {
    return value.background(self);
    // return 1;
  }
}

pub trait QStandardItem_background {
  fn background(self, rsthis: &mut QStandardItem) -> QBrush;
}

// proto:  QBrush QStandardItem::background();
impl<'a> /*trait*/ QStandardItem_background for () {
  fn background(self, rsthis: &mut QStandardItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10backgroundEv()};
    let mut ret = unsafe {_ZNK13QStandardItem10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn data<T: QStandardItem_data>(&mut self, value: T) -> QVariant {
    return value.data(self);
    // return 1;
  }
}

pub trait QStandardItem_data {
  fn data(self, rsthis: &mut QStandardItem) -> QVariant;
}

// proto:  QVariant QStandardItem::data(int role);
impl<'a> /*trait*/ QStandardItem_data for (i32) {
  fn data(self, rsthis: &mut QStandardItem) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QStandardItem4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn NewQStandardItem<T: QStandardItem_NewQStandardItem>(value: T) -> QStandardItem {
    let rsthis = value.NewQStandardItem();
    return rsthis;
    // return 1;
  }
}

pub trait QStandardItem_NewQStandardItem {
  fn NewQStandardItem(self) -> QStandardItem;
}

// proto: void QStandardItem::NewQStandardItem(const QStandardItem & other);
impl<'a> /*trait*/ QStandardItem_NewQStandardItem for (&'a  QStandardItem) {
  fn NewQStandardItem(self) -> QStandardItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QStandardItemC1ERKS_(qthis, arg0)};
    let rsthis = QStandardItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn child<T: QStandardItem_child>(&mut self, value: T) -> QStandardItem {
    return value.child(self);
    // return 1;
  }
}

pub trait QStandardItem_child {
  fn child(self, rsthis: &mut QStandardItem) -> QStandardItem;
}

// proto:  QStandardItem * QStandardItem::child(int row, int column);
impl<'a> /*trait*/ QStandardItem_child for (i32, i32) {
  fn child(self, rsthis: &mut QStandardItem) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5childEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK13QStandardItem5childEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isSelectable<T: QStandardItem_isSelectable>(&mut self, value: T) -> i8 {
    return value.isSelectable(self);
    // return 1;
  }
}

pub trait QStandardItem_isSelectable {
  fn isSelectable(self, rsthis: &mut QStandardItem) -> i8;
}

// proto:  bool QStandardItem::isSelectable();
impl<'a> /*trait*/ QStandardItem_isSelectable for () {
  fn isSelectable(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem12isSelectableEv()};
    let mut ret = unsafe {_ZNK13QStandardItem12isSelectableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn toolTip<T: QStandardItem_toolTip>(&mut self, value: T) -> QString {
    return value.toolTip(self);
    // return 1;
  }
}

pub trait QStandardItem_toolTip {
  fn toolTip(self, rsthis: &mut QStandardItem) -> QString;
}

// proto:  QString QStandardItem::toolTip();
impl<'a> /*trait*/ QStandardItem_toolTip for () {
  fn toolTip(self, rsthis: &mut QStandardItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem7toolTipEv()};
    let mut ret = unsafe {_ZNK13QStandardItem7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setRowCount<T: QStandardItem_setRowCount>(&mut self, value: T)  {
     value.setRowCount(self);
    // return 1;
  }
}

pub trait QStandardItem_setRowCount {
  fn setRowCount(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setRowCount(int rows);
impl<'a> /*trait*/ QStandardItem_setRowCount for (i32) {
  fn setRowCount(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setRowCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QStandardItem11setRowCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QStandardItem::NewQStandardItem(const QString & text);
impl<'a> /*trait*/ QStandardItem_NewQStandardItem for (&'a  QString) {
  fn NewQStandardItem(self) -> QStandardItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QStandardItemC1ERK7QString(qthis, arg0)};
    let rsthis = QStandardItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn write<T: QStandardItem_write>(&mut self, value: T)  {
     value.write(self);
    // return 1;
  }
}

pub trait QStandardItem_write {
  fn write(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::write(QDataStream & out);
impl<'a> /*trait*/ QStandardItem_write for (&'a mut QDataStream) {
  fn write(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK13QStandardItem5writeER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isDragEnabled<T: QStandardItem_isDragEnabled>(&mut self, value: T) -> i8 {
    return value.isDragEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_isDragEnabled {
  fn isDragEnabled(self, rsthis: &mut QStandardItem) -> i8;
}

// proto:  bool QStandardItem::isDragEnabled();
impl<'a> /*trait*/ QStandardItem_isDragEnabled for () {
  fn isDragEnabled(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem13isDragEnabledEv()};
    let mut ret = unsafe {_ZNK13QStandardItem13isDragEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setAccessibleText<T: QStandardItem_setAccessibleText>(&mut self, value: T)  {
     value.setAccessibleText(self);
    // return 1;
  }
}

pub trait QStandardItem_setAccessibleText {
  fn setAccessibleText(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setAccessibleText(const QString & accessibleText);
impl<'a> /*trait*/ QStandardItem_setAccessibleText for (&'a  QString) {
  fn setAccessibleText(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem17setAccessibleTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem17setAccessibleTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn rowCount<T: QStandardItem_rowCount>(&mut self, value: T) -> i32 {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QStandardItem_rowCount {
  fn rowCount(self, rsthis: &mut QStandardItem) -> i32;
}

// proto:  int QStandardItem::rowCount();
impl<'a> /*trait*/ QStandardItem_rowCount for () {
  fn rowCount(self, rsthis: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem8rowCountEv()};
    let mut ret = unsafe {_ZNK13QStandardItem8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn removeColumn<T: QStandardItem_removeColumn>(&mut self, value: T)  {
     value.removeColumn(self);
    // return 1;
  }
}

pub trait QStandardItem_removeColumn {
  fn removeColumn(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::removeColumn(int column);
impl<'a> /*trait*/ QStandardItem_removeColumn for (i32) {
  fn removeColumn(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12removeColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QStandardItem12removeColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn removeRow<T: QStandardItem_removeRow>(&mut self, value: T)  {
     value.removeRow(self);
    // return 1;
  }
}

pub trait QStandardItem_removeRow {
  fn removeRow(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::removeRow(int row);
impl<'a> /*trait*/ QStandardItem_removeRow for (i32) {
  fn removeRow(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem9removeRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QStandardItem9removeRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn columnCount<T: QStandardItem_columnCount>(&mut self, value: T) -> i32 {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QStandardItem_columnCount {
  fn columnCount(self, rsthis: &mut QStandardItem) -> i32;
}

// proto:  int QStandardItem::columnCount();
impl<'a> /*trait*/ QStandardItem_columnCount for () {
  fn columnCount(self, rsthis: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem11columnCountEv()};
    let mut ret = unsafe {_ZNK13QStandardItem11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isTristate<T: QStandardItem_isTristate>(&mut self, value: T) -> i8 {
    return value.isTristate(self);
    // return 1;
  }
}

pub trait QStandardItem_isTristate {
  fn isTristate(self, rsthis: &mut QStandardItem) -> i8;
}

// proto:  bool QStandardItem::isTristate();
impl<'a> /*trait*/ QStandardItem_isTristate for () {
  fn isTristate(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10isTristateEv()};
    let mut ret = unsafe {_ZNK13QStandardItem10isTristateEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn parent<T: QStandardItem_parent>(&mut self, value: T) -> QStandardItem {
    return value.parent(self);
    // return 1;
  }
}

pub trait QStandardItem_parent {
  fn parent(self, rsthis: &mut QStandardItem) -> QStandardItem;
}

// proto:  QStandardItem * QStandardItem::parent();
impl<'a> /*trait*/ QStandardItem_parent for () {
  fn parent(self, rsthis: &mut QStandardItem) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem6parentEv()};
    let mut ret = unsafe {_ZNK13QStandardItem6parentEv(rsthis.qclsinst)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QStandardItem::NewQStandardItem(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QStandardItem_NewQStandardItem for (&'a  QIcon, &'a  QString) {
  fn NewQStandardItem(self) -> QStandardItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemC1ERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QStandardItemC1ERK5QIconRK7QString(qthis, arg0, arg1)};
    let rsthis = QStandardItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setFont<T: QStandardItem_setFont>(&mut self, value: T)  {
     value.setFont(self);
    // return 1;
  }
}

pub trait QStandardItem_setFont {
  fn setFont(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setFont(const QFont & font);
impl<'a> /*trait*/ QStandardItem_setFont for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn removeColumns<T: QStandardItem_removeColumns>(&mut self, value: T)  {
     value.removeColumns(self);
    // return 1;
  }
}

pub trait QStandardItem_removeColumns {
  fn removeColumns(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::removeColumns(int column, int count);
impl<'a> /*trait*/ QStandardItem_removeColumns for (i32, i32) {
  fn removeColumns(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13removeColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QStandardItem13removeColumnsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn FreeQStandardItem<T: QStandardItem_FreeQStandardItem>(&mut self, value: T)  {
     value.FreeQStandardItem(self);
    // return 1;
  }
}

pub trait QStandardItem_FreeQStandardItem {
  fn FreeQStandardItem(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::FreeQStandardItem();
impl<'a> /*trait*/ QStandardItem_FreeQStandardItem for () {
  fn FreeQStandardItem(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemD0Ev()};
     unsafe {_ZN13QStandardItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QStandardItem::NewQStandardItem();
impl<'a> /*trait*/ QStandardItem_NewQStandardItem for () {
  fn NewQStandardItem(self) -> QStandardItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemC1Ev()};
    unsafe {_ZN13QStandardItemC1Ev(qthis)};
    let rsthis = QStandardItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn font<T: QStandardItem_font>(&mut self, value: T) -> QFont {
    return value.font(self);
    // return 1;
  }
}

pub trait QStandardItem_font {
  fn font(self, rsthis: &mut QStandardItem) -> QFont;
}

// proto:  QFont QStandardItem::font();
impl<'a> /*trait*/ QStandardItem_font for () {
  fn font(self, rsthis: &mut QStandardItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4fontEv()};
    let mut ret = unsafe {_ZNK13QStandardItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setEditable<T: QStandardItem_setEditable>(&mut self, value: T)  {
     value.setEditable(self);
    // return 1;
  }
}

pub trait QStandardItem_setEditable {
  fn setEditable(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setEditable(bool editable);
impl<'a> /*trait*/ QStandardItem_setEditable for (i8) {
  fn setEditable(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setEditableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem11setEditableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setText<T: QStandardItem_setText>(&mut self, value: T)  {
     value.setText(self);
    // return 1;
  }
}

pub trait QStandardItem_setText {
  fn setText(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setText(const QString & text);
impl<'a> /*trait*/ QStandardItem_setText for (&'a  QString) {
  fn setText(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QStandardItem::NewQStandardItem(int rows, int columns);
impl<'a> /*trait*/ QStandardItem_NewQStandardItem for (i32, i32) {
  fn NewQStandardItem(self) -> QStandardItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN13QStandardItemC1Eii(qthis, arg0, arg1)};
    let rsthis = QStandardItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isEnabled<T: QStandardItem_isEnabled>(&mut self, value: T) -> i8 {
    return value.isEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_isEnabled {
  fn isEnabled(self, rsthis: &mut QStandardItem) -> i8;
}

// proto:  bool QStandardItem::isEnabled();
impl<'a> /*trait*/ QStandardItem_isEnabled for () {
  fn isEnabled(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem9isEnabledEv()};
    let mut ret = unsafe {_ZNK13QStandardItem9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setDropEnabled<T: QStandardItem_setDropEnabled>(&mut self, value: T)  {
     value.setDropEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_setDropEnabled {
  fn setDropEnabled(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setDropEnabled(bool dropEnabled);
impl<'a> /*trait*/ QStandardItem_setDropEnabled for (i8) {
  fn setDropEnabled(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem14setDropEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem14setDropEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setColumnCount<T: QStandardItem_setColumnCount>(&mut self, value: T)  {
     value.setColumnCount(self);
    // return 1;
  }
}

pub trait QStandardItem_setColumnCount {
  fn setColumnCount(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setColumnCount(int columns);
impl<'a> /*trait*/ QStandardItem_setColumnCount for (i32) {
  fn setColumnCount(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem14setColumnCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QStandardItem14setColumnCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn accessibleText<T: QStandardItem_accessibleText>(&mut self, value: T) -> QString {
    return value.accessibleText(self);
    // return 1;
  }
}

pub trait QStandardItem_accessibleText {
  fn accessibleText(self, rsthis: &mut QStandardItem) -> QString;
}

// proto:  QString QStandardItem::accessibleText();
impl<'a> /*trait*/ QStandardItem_accessibleText for () {
  fn accessibleText(self, rsthis: &mut QStandardItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem14accessibleTextEv()};
    let mut ret = unsafe {_ZNK13QStandardItem14accessibleTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn read<T: QStandardItem_read>(&mut self, value: T)  {
     value.read(self);
    // return 1;
  }
}

pub trait QStandardItem_read {
  fn read(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::read(QDataStream & in);
impl<'a> /*trait*/ QStandardItem_read for (&'a mut QDataStream) {
  fn read(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem4readER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setCheckable<T: QStandardItem_setCheckable>(&mut self, value: T)  {
     value.setCheckable(self);
    // return 1;
  }
}

pub trait QStandardItem_setCheckable {
  fn setCheckable(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setCheckable(bool checkable);
impl<'a> /*trait*/ QStandardItem_setCheckable for (i8) {
  fn setCheckable(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12setCheckableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem12setCheckableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setDragEnabled<T: QStandardItem_setDragEnabled>(&mut self, value: T)  {
     value.setDragEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_setDragEnabled {
  fn setDragEnabled(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setDragEnabled(bool dragEnabled);
impl<'a> /*trait*/ QStandardItem_setDragEnabled for (i8) {
  fn setDragEnabled(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem14setDragEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem14setDragEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn foreground<T: QStandardItem_foreground>(&mut self, value: T) -> QBrush {
    return value.foreground(self);
    // return 1;
  }
}

pub trait QStandardItem_foreground {
  fn foreground(self, rsthis: &mut QStandardItem) -> QBrush;
}

// proto:  QBrush QStandardItem::foreground();
impl<'a> /*trait*/ QStandardItem_foreground for () {
  fn foreground(self, rsthis: &mut QStandardItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10foregroundEv()};
    let mut ret = unsafe {_ZNK13QStandardItem10foregroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn clone<T: QStandardItem_clone>(&mut self, value: T) -> QStandardItem {
    return value.clone(self);
    // return 1;
  }
}

pub trait QStandardItem_clone {
  fn clone(self, rsthis: &mut QStandardItem) -> QStandardItem;
}

// proto:  QStandardItem * QStandardItem::clone();
impl<'a> /*trait*/ QStandardItem_clone for () {
  fn clone(self, rsthis: &mut QStandardItem) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5cloneEv()};
    let mut ret = unsafe {_ZNK13QStandardItem5cloneEv(rsthis.qclsinst)};
    let mut ret1 = QStandardItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn removeRows<T: QStandardItem_removeRows>(&mut self, value: T)  {
     value.removeRows(self);
    // return 1;
  }
}

pub trait QStandardItem_removeRows {
  fn removeRows(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::removeRows(int row, int count);
impl<'a> /*trait*/ QStandardItem_removeRows for (i32, i32) {
  fn removeRows(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10removeRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QStandardItem10removeRowsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn sizeHint<T: QStandardItem_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QStandardItem_sizeHint {
  fn sizeHint(self, rsthis: &mut QStandardItem) -> QSize;
}

// proto:  QSize QStandardItem::sizeHint();
impl<'a> /*trait*/ QStandardItem_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QStandardItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK13QStandardItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setEnabled<T: QStandardItem_setEnabled>(&mut self, value: T)  {
     value.setEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_setEnabled {
  fn setEnabled(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setEnabled(bool enabled);
impl<'a> /*trait*/ QStandardItem_setEnabled for (i8) {
  fn setEnabled(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10setEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setBackground<T: QStandardItem_setBackground>(&mut self, value: T)  {
     value.setBackground(self);
    // return 1;
  }
}

pub trait QStandardItem_setBackground {
  fn setBackground(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QStandardItem_setBackground for (&'a  QBrush) {
  fn setBackground(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setAccessibleDescription<T: QStandardItem_setAccessibleDescription>(&mut self, value: T)  {
     value.setAccessibleDescription(self);
    // return 1;
  }
}

pub trait QStandardItem_setAccessibleDescription {
  fn setAccessibleDescription(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setAccessibleDescription(const QString & accessibleDescription);
impl<'a> /*trait*/ QStandardItem_setAccessibleDescription for (&'a  QString) {
  fn setAccessibleDescription(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem24setAccessibleDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem24setAccessibleDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setSizeHint<T: QStandardItem_setSizeHint>(&mut self, value: T)  {
     value.setSizeHint(self);
    // return 1;
  }
}

pub trait QStandardItem_setSizeHint {
  fn setSizeHint(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setSizeHint(const QSize & sizeHint);
impl<'a> /*trait*/ QStandardItem_setSizeHint for (&'a  QSize) {
  fn setSizeHint(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setSizeHintERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem11setSizeHintERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn accessibleDescription<T: QStandardItem_accessibleDescription>(&mut self, value: T) -> QString {
    return value.accessibleDescription(self);
    // return 1;
  }
}

pub trait QStandardItem_accessibleDescription {
  fn accessibleDescription(self, rsthis: &mut QStandardItem) -> QString;
}

// proto:  QString QStandardItem::accessibleDescription();
impl<'a> /*trait*/ QStandardItem_accessibleDescription for () {
  fn accessibleDescription(self, rsthis: &mut QStandardItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem21accessibleDescriptionEv()};
    let mut ret = unsafe {_ZNK13QStandardItem21accessibleDescriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setTristate<T: QStandardItem_setTristate>(&mut self, value: T)  {
     value.setTristate(self);
    // return 1;
  }
}

pub trait QStandardItem_setTristate {
  fn setTristate(self, rsthis: &mut QStandardItem) ;
}

// proto:  void QStandardItem::setTristate(bool tristate);
impl<'a> /*trait*/ QStandardItem_setTristate for (i8) {
  fn setTristate(self, rsthis: &mut QStandardItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setTristateEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem11setTristateEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

