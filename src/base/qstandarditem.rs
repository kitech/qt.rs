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
  pub fn setChild<RetType, T: QStandardItem_setChild<RetType>>(&mut self, value: T) -> RetType {
    return value.setChild(self);
    // return 1;
  }
}

pub trait QStandardItem_setChild<RetType> {
  fn setChild(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setChild(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_setChild<()> for (i32, &'a mut QStandardItem) {
  fn setChild(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem8setChildEiPS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem8setChildEiPS_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn model<RetType, T: QStandardItem_model<RetType>>(&mut self, value: T) -> RetType {
    return value.model(self);
    // return 1;
  }
}

pub trait QStandardItem_model<RetType> {
  fn model(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QStandardItemModel * QStandardItem::model();
impl<'a> /*trait*/ QStandardItem_model<QStandardItemModel> for () {
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
  pub fn insertColumns<RetType, T: QStandardItem_insertColumns<RetType>>(&mut self, value: T) -> RetType {
    return value.insertColumns(self);
    // return 1;
  }
}

pub trait QStandardItem_insertColumns<RetType> {
  fn insertColumns(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::insertColumns(int column, int count);
impl<'a> /*trait*/ QStandardItem_insertColumns<()> for (i32, i32) {
  fn insertColumns(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13insertColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QStandardItem13insertColumnsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setSelectable<RetType, T: QStandardItem_setSelectable<RetType>>(&mut self, value: T) -> RetType {
    return value.setSelectable(self);
    // return 1;
  }
}

pub trait QStandardItem_setSelectable<RetType> {
  fn setSelectable(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setSelectable(bool selectable);
impl<'a> /*trait*/ QStandardItem_setSelectable<()> for (i8) {
  fn setSelectable(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13setSelectableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem13setSelectableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn column<RetType, T: QStandardItem_column<RetType>>(&mut self, value: T) -> RetType {
    return value.column(self);
    // return 1;
  }
}

pub trait QStandardItem_column<RetType> {
  fn column(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  int QStandardItem::column();
impl<'a> /*trait*/ QStandardItem_column<i32> for () {
  fn column(self, rsthis: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem6columnEv()};
    let mut ret = unsafe {_ZNK13QStandardItem6columnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn whatsThis<RetType, T: QStandardItem_whatsThis<RetType>>(&mut self, value: T) -> RetType {
    return value.whatsThis(self);
    // return 1;
  }
}

pub trait QStandardItem_whatsThis<RetType> {
  fn whatsThis(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QString QStandardItem::whatsThis();
impl<'a> /*trait*/ QStandardItem_whatsThis<QString> for () {
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
  pub fn takeColumn<RetType, T: QStandardItem_takeColumn<RetType>>(&mut self, value: T) -> RetType {
    return value.takeColumn(self);
    // return 1;
  }
}

pub trait QStandardItem_takeColumn<RetType> {
  fn takeColumn(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QList<QStandardItem *> QStandardItem::takeColumn(int column);
impl<'a> /*trait*/ QStandardItem_takeColumn<()> for (i32) {
  fn takeColumn(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10takeColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QStandardItem10takeColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setForeground<RetType, T: QStandardItem_setForeground<RetType>>(&mut self, value: T) -> RetType {
    return value.setForeground(self);
    // return 1;
  }
}

pub trait QStandardItem_setForeground<RetType> {
  fn setForeground(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QStandardItem_setForeground<()> for (&'a  QBrush) {
  fn setForeground(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem13setForegroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isEditable<RetType, T: QStandardItem_isEditable<RetType>>(&mut self, value: T) -> RetType {
    return value.isEditable(self);
    // return 1;
  }
}

pub trait QStandardItem_isEditable<RetType> {
  fn isEditable(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  bool QStandardItem::isEditable();
impl<'a> /*trait*/ QStandardItem_isEditable<i8> for () {
  fn isEditable(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10isEditableEv()};
    let mut ret = unsafe {_ZNK13QStandardItem10isEditableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn icon<RetType, T: QStandardItem_icon<RetType>>(&mut self, value: T) -> RetType {
    return value.icon(self);
    // return 1;
  }
}

pub trait QStandardItem_icon<RetType> {
  fn icon(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QIcon QStandardItem::icon();
impl<'a> /*trait*/ QStandardItem_icon<QIcon> for () {
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
  pub fn setWhatsThis<RetType, T: QStandardItem_setWhatsThis<RetType>>(&mut self, value: T) -> RetType {
    return value.setWhatsThis(self);
    // return 1;
  }
}

pub trait QStandardItem_setWhatsThis<RetType> {
  fn setWhatsThis(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setWhatsThis(const QString & whatsThis);
impl<'a> /*trait*/ QStandardItem_setWhatsThis<()> for (&'a  QString) {
  fn setWhatsThis(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn takeChild<RetType, T: QStandardItem_takeChild<RetType>>(&mut self, value: T) -> RetType {
    return value.takeChild(self);
    // return 1;
  }
}

pub trait QStandardItem_takeChild<RetType> {
  fn takeChild(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QStandardItem * QStandardItem::takeChild(int row, int column);
impl<'a> /*trait*/ QStandardItem_takeChild<QStandardItem> for (i32, i32) {
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
  pub fn type_<RetType, T: QStandardItem_type_<RetType>>(&mut self, value: T) -> RetType {
    return value.type_(self);
    // return 1;
  }
}

pub trait QStandardItem_type_<RetType> {
  fn type_(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  int QStandardItem::type_();
impl<'a> /*trait*/ QStandardItem_type_<i32> for () {
  fn type_(self, rsthis: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4typeEv()};
    let mut ret = unsafe {_ZNK13QStandardItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn takeRow<RetType, T: QStandardItem_takeRow<RetType>>(&mut self, value: T) -> RetType {
    return value.takeRow(self);
    // return 1;
  }
}

pub trait QStandardItem_takeRow<RetType> {
  fn takeRow(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QList<QStandardItem *> QStandardItem::takeRow(int row);
impl<'a> /*trait*/ QStandardItem_takeRow<()> for (i32) {
  fn takeRow(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7takeRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QStandardItem7takeRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn row<RetType, T: QStandardItem_row<RetType>>(&mut self, value: T) -> RetType {
    return value.row(self);
    // return 1;
  }
}

pub trait QStandardItem_row<RetType> {
  fn row(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  int QStandardItem::row();
impl<'a> /*trait*/ QStandardItem_row<i32> for () {
  fn row(self, rsthis: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem3rowEv()};
    let mut ret = unsafe {_ZNK13QStandardItem3rowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isCheckable<RetType, T: QStandardItem_isCheckable<RetType>>(&mut self, value: T) -> RetType {
    return value.isCheckable(self);
    // return 1;
  }
}

pub trait QStandardItem_isCheckable<RetType> {
  fn isCheckable(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  bool QStandardItem::isCheckable();
impl<'a> /*trait*/ QStandardItem_isCheckable<i8> for () {
  fn isCheckable(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem11isCheckableEv()};
    let mut ret = unsafe {_ZNK13QStandardItem11isCheckableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn text<RetType, T: QStandardItem_text<RetType>>(&mut self, value: T) -> RetType {
    return value.text(self);
    // return 1;
  }
}

pub trait QStandardItem_text<RetType> {
  fn text(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QString QStandardItem::text();
impl<'a> /*trait*/ QStandardItem_text<QString> for () {
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
  pub fn insertRows<RetType, T: QStandardItem_insertRows<RetType>>(&mut self, value: T) -> RetType {
    return value.insertRows(self);
    // return 1;
  }
}

pub trait QStandardItem_insertRows<RetType> {
  fn insertRows(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::insertRows(int row, int count);
impl<'a> /*trait*/ QStandardItem_insertRows<()> for (i32, i32) {
  fn insertRows(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10insertRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QStandardItem10insertRowsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isDropEnabled<RetType, T: QStandardItem_isDropEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isDropEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_isDropEnabled<RetType> {
  fn isDropEnabled(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  bool QStandardItem::isDropEnabled();
impl<'a> /*trait*/ QStandardItem_isDropEnabled<i8> for () {
  fn isDropEnabled(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem13isDropEnabledEv()};
    let mut ret = unsafe {_ZNK13QStandardItem13isDropEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn hasChildren<RetType, T: QStandardItem_hasChildren<RetType>>(&mut self, value: T) -> RetType {
    return value.hasChildren(self);
    // return 1;
  }
}

pub trait QStandardItem_hasChildren<RetType> {
  fn hasChildren(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  bool QStandardItem::hasChildren();
impl<'a> /*trait*/ QStandardItem_hasChildren<i8> for () {
  fn hasChildren(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem11hasChildrenEv()};
    let mut ret = unsafe {_ZNK13QStandardItem11hasChildrenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn statusTip<RetType, T: QStandardItem_statusTip<RetType>>(&mut self, value: T) -> RetType {
    return value.statusTip(self);
    // return 1;
  }
}

pub trait QStandardItem_statusTip<RetType> {
  fn statusTip(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QString QStandardItem::statusTip();
impl<'a> /*trait*/ QStandardItem_statusTip<QString> for () {
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
  pub fn setStatusTip<RetType, T: QStandardItem_setStatusTip<RetType>>(&mut self, value: T) -> RetType {
    return value.setStatusTip(self);
    // return 1;
  }
}

pub trait QStandardItem_setStatusTip<RetType> {
  fn setStatusTip(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QStandardItem_setStatusTip<()> for (&'a  QString) {
  fn setStatusTip(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem12setStatusTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn appendRow<RetType, T: QStandardItem_appendRow<RetType>>(&mut self, value: T) -> RetType {
    return value.appendRow(self);
    // return 1;
  }
}

pub trait QStandardItem_appendRow<RetType> {
  fn appendRow(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::appendRow(QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_appendRow<()> for (&'a mut QStandardItem) {
  fn appendRow(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem9appendRowEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem9appendRowEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QStandardItem::setChild(int row, int column, QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_setChild<()> for (i32, i32, &'a mut QStandardItem) {
  fn setChild(self, rsthis: &mut QStandardItem) -> () {
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
  pub fn index<RetType, T: QStandardItem_index<RetType>>(&mut self, value: T) -> RetType {
    return value.index(self);
    // return 1;
  }
}

pub trait QStandardItem_index<RetType> {
  fn index(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QModelIndex QStandardItem::index();
impl<'a> /*trait*/ QStandardItem_index<QModelIndex> for () {
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
  pub fn setIcon<RetType, T: QStandardItem_setIcon<RetType>>(&mut self, value: T) -> RetType {
    return value.setIcon(self);
    // return 1;
  }
}

pub trait QStandardItem_setIcon<RetType> {
  fn setIcon(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QStandardItem_setIcon<()> for (&'a  QIcon) {
  fn setIcon(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setToolTip<RetType, T: QStandardItem_setToolTip<RetType>>(&mut self, value: T) -> RetType {
    return value.setToolTip(self);
    // return 1;
  }
}

pub trait QStandardItem_setToolTip<RetType> {
  fn setToolTip(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QStandardItem_setToolTip<()> for (&'a  QString) {
  fn setToolTip(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setData<RetType, T: QStandardItem_setData<RetType>>(&mut self, value: T) -> RetType {
    return value.setData(self);
    // return 1;
  }
}

pub trait QStandardItem_setData<RetType> {
  fn setData(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setData(const QVariant & value, int role);
impl<'a> /*trait*/ QStandardItem_setData<()> for (&'a  QVariant, i32) {
  fn setData(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setDataERK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QStandardItem7setDataERK8QVarianti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn background<RetType, T: QStandardItem_background<RetType>>(&mut self, value: T) -> RetType {
    return value.background(self);
    // return 1;
  }
}

pub trait QStandardItem_background<RetType> {
  fn background(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QBrush QStandardItem::background();
impl<'a> /*trait*/ QStandardItem_background<QBrush> for () {
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
  pub fn data<RetType, T: QStandardItem_data<RetType>>(&mut self, value: T) -> RetType {
    return value.data(self);
    // return 1;
  }
}

pub trait QStandardItem_data<RetType> {
  fn data(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QVariant QStandardItem::data(int role);
impl<'a> /*trait*/ QStandardItem_data<QVariant> for (i32) {
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
  pub fn child<RetType, T: QStandardItem_child<RetType>>(&mut self, value: T) -> RetType {
    return value.child(self);
    // return 1;
  }
}

pub trait QStandardItem_child<RetType> {
  fn child(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QStandardItem * QStandardItem::child(int row, int column);
impl<'a> /*trait*/ QStandardItem_child<QStandardItem> for (i32, i32) {
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
  pub fn isSelectable<RetType, T: QStandardItem_isSelectable<RetType>>(&mut self, value: T) -> RetType {
    return value.isSelectable(self);
    // return 1;
  }
}

pub trait QStandardItem_isSelectable<RetType> {
  fn isSelectable(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  bool QStandardItem::isSelectable();
impl<'a> /*trait*/ QStandardItem_isSelectable<i8> for () {
  fn isSelectable(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem12isSelectableEv()};
    let mut ret = unsafe {_ZNK13QStandardItem12isSelectableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn toolTip<RetType, T: QStandardItem_toolTip<RetType>>(&mut self, value: T) -> RetType {
    return value.toolTip(self);
    // return 1;
  }
}

pub trait QStandardItem_toolTip<RetType> {
  fn toolTip(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QString QStandardItem::toolTip();
impl<'a> /*trait*/ QStandardItem_toolTip<QString> for () {
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
  pub fn setRowCount<RetType, T: QStandardItem_setRowCount<RetType>>(&mut self, value: T) -> RetType {
    return value.setRowCount(self);
    // return 1;
  }
}

pub trait QStandardItem_setRowCount<RetType> {
  fn setRowCount(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setRowCount(int rows);
impl<'a> /*trait*/ QStandardItem_setRowCount<()> for (i32) {
  fn setRowCount(self, rsthis: &mut QStandardItem) -> () {
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
  pub fn write<RetType, T: QStandardItem_write<RetType>>(&mut self, value: T) -> RetType {
    return value.write(self);
    // return 1;
  }
}

pub trait QStandardItem_write<RetType> {
  fn write(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::write(QDataStream & out);
impl<'a> /*trait*/ QStandardItem_write<()> for (&'a mut QDataStream) {
  fn write(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK13QStandardItem5writeER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isDragEnabled<RetType, T: QStandardItem_isDragEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isDragEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_isDragEnabled<RetType> {
  fn isDragEnabled(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  bool QStandardItem::isDragEnabled();
impl<'a> /*trait*/ QStandardItem_isDragEnabled<i8> for () {
  fn isDragEnabled(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem13isDragEnabledEv()};
    let mut ret = unsafe {_ZNK13QStandardItem13isDragEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setAccessibleText<RetType, T: QStandardItem_setAccessibleText<RetType>>(&mut self, value: T) -> RetType {
    return value.setAccessibleText(self);
    // return 1;
  }
}

pub trait QStandardItem_setAccessibleText<RetType> {
  fn setAccessibleText(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setAccessibleText(const QString & accessibleText);
impl<'a> /*trait*/ QStandardItem_setAccessibleText<()> for (&'a  QString) {
  fn setAccessibleText(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem17setAccessibleTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem17setAccessibleTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn rowCount<RetType, T: QStandardItem_rowCount<RetType>>(&mut self, value: T) -> RetType {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QStandardItem_rowCount<RetType> {
  fn rowCount(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  int QStandardItem::rowCount();
impl<'a> /*trait*/ QStandardItem_rowCount<i32> for () {
  fn rowCount(self, rsthis: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem8rowCountEv()};
    let mut ret = unsafe {_ZNK13QStandardItem8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn removeColumn<RetType, T: QStandardItem_removeColumn<RetType>>(&mut self, value: T) -> RetType {
    return value.removeColumn(self);
    // return 1;
  }
}

pub trait QStandardItem_removeColumn<RetType> {
  fn removeColumn(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::removeColumn(int column);
impl<'a> /*trait*/ QStandardItem_removeColumn<()> for (i32) {
  fn removeColumn(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12removeColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QStandardItem12removeColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn removeRow<RetType, T: QStandardItem_removeRow<RetType>>(&mut self, value: T) -> RetType {
    return value.removeRow(self);
    // return 1;
  }
}

pub trait QStandardItem_removeRow<RetType> {
  fn removeRow(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::removeRow(int row);
impl<'a> /*trait*/ QStandardItem_removeRow<()> for (i32) {
  fn removeRow(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem9removeRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QStandardItem9removeRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn columnCount<RetType, T: QStandardItem_columnCount<RetType>>(&mut self, value: T) -> RetType {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QStandardItem_columnCount<RetType> {
  fn columnCount(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  int QStandardItem::columnCount();
impl<'a> /*trait*/ QStandardItem_columnCount<i32> for () {
  fn columnCount(self, rsthis: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem11columnCountEv()};
    let mut ret = unsafe {_ZNK13QStandardItem11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isTristate<RetType, T: QStandardItem_isTristate<RetType>>(&mut self, value: T) -> RetType {
    return value.isTristate(self);
    // return 1;
  }
}

pub trait QStandardItem_isTristate<RetType> {
  fn isTristate(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  bool QStandardItem::isTristate();
impl<'a> /*trait*/ QStandardItem_isTristate<i8> for () {
  fn isTristate(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10isTristateEv()};
    let mut ret = unsafe {_ZNK13QStandardItem10isTristateEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn parent<RetType, T: QStandardItem_parent<RetType>>(&mut self, value: T) -> RetType {
    return value.parent(self);
    // return 1;
  }
}

pub trait QStandardItem_parent<RetType> {
  fn parent(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QStandardItem * QStandardItem::parent();
impl<'a> /*trait*/ QStandardItem_parent<QStandardItem> for () {
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
  pub fn setFont<RetType, T: QStandardItem_setFont<RetType>>(&mut self, value: T) -> RetType {
    return value.setFont(self);
    // return 1;
  }
}

pub trait QStandardItem_setFont<RetType> {
  fn setFont(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setFont(const QFont & font);
impl<'a> /*trait*/ QStandardItem_setFont<()> for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn removeColumns<RetType, T: QStandardItem_removeColumns<RetType>>(&mut self, value: T) -> RetType {
    return value.removeColumns(self);
    // return 1;
  }
}

pub trait QStandardItem_removeColumns<RetType> {
  fn removeColumns(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::removeColumns(int column, int count);
impl<'a> /*trait*/ QStandardItem_removeColumns<()> for (i32, i32) {
  fn removeColumns(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13removeColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QStandardItem13removeColumnsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn FreeQStandardItem<RetType, T: QStandardItem_FreeQStandardItem<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQStandardItem(self);
    // return 1;
  }
}

pub trait QStandardItem_FreeQStandardItem<RetType> {
  fn FreeQStandardItem(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::FreeQStandardItem();
impl<'a> /*trait*/ QStandardItem_FreeQStandardItem<()> for () {
  fn FreeQStandardItem(self, rsthis: &mut QStandardItem) -> () {
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
  pub fn font<RetType, T: QStandardItem_font<RetType>>(&mut self, value: T) -> RetType {
    return value.font(self);
    // return 1;
  }
}

pub trait QStandardItem_font<RetType> {
  fn font(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QFont QStandardItem::font();
impl<'a> /*trait*/ QStandardItem_font<QFont> for () {
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
  pub fn setEditable<RetType, T: QStandardItem_setEditable<RetType>>(&mut self, value: T) -> RetType {
    return value.setEditable(self);
    // return 1;
  }
}

pub trait QStandardItem_setEditable<RetType> {
  fn setEditable(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setEditable(bool editable);
impl<'a> /*trait*/ QStandardItem_setEditable<()> for (i8) {
  fn setEditable(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setEditableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem11setEditableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setText<RetType, T: QStandardItem_setText<RetType>>(&mut self, value: T) -> RetType {
    return value.setText(self);
    // return 1;
  }
}

pub trait QStandardItem_setText<RetType> {
  fn setText(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setText(const QString & text);
impl<'a> /*trait*/ QStandardItem_setText<()> for (&'a  QString) {
  fn setText(self, rsthis: &mut QStandardItem) -> () {
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
  pub fn isEnabled<RetType, T: QStandardItem_isEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_isEnabled<RetType> {
  fn isEnabled(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  bool QStandardItem::isEnabled();
impl<'a> /*trait*/ QStandardItem_isEnabled<i8> for () {
  fn isEnabled(self, rsthis: &mut QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem9isEnabledEv()};
    let mut ret = unsafe {_ZNK13QStandardItem9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setDropEnabled<RetType, T: QStandardItem_setDropEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setDropEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_setDropEnabled<RetType> {
  fn setDropEnabled(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setDropEnabled(bool dropEnabled);
impl<'a> /*trait*/ QStandardItem_setDropEnabled<()> for (i8) {
  fn setDropEnabled(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem14setDropEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem14setDropEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setColumnCount<RetType, T: QStandardItem_setColumnCount<RetType>>(&mut self, value: T) -> RetType {
    return value.setColumnCount(self);
    // return 1;
  }
}

pub trait QStandardItem_setColumnCount<RetType> {
  fn setColumnCount(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setColumnCount(int columns);
impl<'a> /*trait*/ QStandardItem_setColumnCount<()> for (i32) {
  fn setColumnCount(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem14setColumnCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QStandardItem14setColumnCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn accessibleText<RetType, T: QStandardItem_accessibleText<RetType>>(&mut self, value: T) -> RetType {
    return value.accessibleText(self);
    // return 1;
  }
}

pub trait QStandardItem_accessibleText<RetType> {
  fn accessibleText(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QString QStandardItem::accessibleText();
impl<'a> /*trait*/ QStandardItem_accessibleText<QString> for () {
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
  pub fn read<RetType, T: QStandardItem_read<RetType>>(&mut self, value: T) -> RetType {
    return value.read(self);
    // return 1;
  }
}

pub trait QStandardItem_read<RetType> {
  fn read(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::read(QDataStream & in);
impl<'a> /*trait*/ QStandardItem_read<()> for (&'a mut QDataStream) {
  fn read(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem4readER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setCheckable<RetType, T: QStandardItem_setCheckable<RetType>>(&mut self, value: T) -> RetType {
    return value.setCheckable(self);
    // return 1;
  }
}

pub trait QStandardItem_setCheckable<RetType> {
  fn setCheckable(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setCheckable(bool checkable);
impl<'a> /*trait*/ QStandardItem_setCheckable<()> for (i8) {
  fn setCheckable(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12setCheckableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem12setCheckableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setDragEnabled<RetType, T: QStandardItem_setDragEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setDragEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_setDragEnabled<RetType> {
  fn setDragEnabled(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setDragEnabled(bool dragEnabled);
impl<'a> /*trait*/ QStandardItem_setDragEnabled<()> for (i8) {
  fn setDragEnabled(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem14setDragEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem14setDragEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn foreground<RetType, T: QStandardItem_foreground<RetType>>(&mut self, value: T) -> RetType {
    return value.foreground(self);
    // return 1;
  }
}

pub trait QStandardItem_foreground<RetType> {
  fn foreground(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QBrush QStandardItem::foreground();
impl<'a> /*trait*/ QStandardItem_foreground<QBrush> for () {
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
  pub fn clone<RetType, T: QStandardItem_clone<RetType>>(&mut self, value: T) -> RetType {
    return value.clone(self);
    // return 1;
  }
}

pub trait QStandardItem_clone<RetType> {
  fn clone(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QStandardItem * QStandardItem::clone();
impl<'a> /*trait*/ QStandardItem_clone<QStandardItem> for () {
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
  pub fn removeRows<RetType, T: QStandardItem_removeRows<RetType>>(&mut self, value: T) -> RetType {
    return value.removeRows(self);
    // return 1;
  }
}

pub trait QStandardItem_removeRows<RetType> {
  fn removeRows(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::removeRows(int row, int count);
impl<'a> /*trait*/ QStandardItem_removeRows<()> for (i32, i32) {
  fn removeRows(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10removeRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QStandardItem10removeRowsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn sizeHint<RetType, T: QStandardItem_sizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QStandardItem_sizeHint<RetType> {
  fn sizeHint(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QSize QStandardItem::sizeHint();
impl<'a> /*trait*/ QStandardItem_sizeHint<QSize> for () {
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
  pub fn setEnabled<RetType, T: QStandardItem_setEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_setEnabled<RetType> {
  fn setEnabled(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setEnabled(bool enabled);
impl<'a> /*trait*/ QStandardItem_setEnabled<()> for (i8) {
  fn setEnabled(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10setEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setBackground<RetType, T: QStandardItem_setBackground<RetType>>(&mut self, value: T) -> RetType {
    return value.setBackground(self);
    // return 1;
  }
}

pub trait QStandardItem_setBackground<RetType> {
  fn setBackground(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QStandardItem_setBackground<()> for (&'a  QBrush) {
  fn setBackground(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setAccessibleDescription<RetType, T: QStandardItem_setAccessibleDescription<RetType>>(&mut self, value: T) -> RetType {
    return value.setAccessibleDescription(self);
    // return 1;
  }
}

pub trait QStandardItem_setAccessibleDescription<RetType> {
  fn setAccessibleDescription(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setAccessibleDescription(const QString & accessibleDescription);
impl<'a> /*trait*/ QStandardItem_setAccessibleDescription<()> for (&'a  QString) {
  fn setAccessibleDescription(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem24setAccessibleDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem24setAccessibleDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setSizeHint<RetType, T: QStandardItem_setSizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.setSizeHint(self);
    // return 1;
  }
}

pub trait QStandardItem_setSizeHint<RetType> {
  fn setSizeHint(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setSizeHint(const QSize & sizeHint);
impl<'a> /*trait*/ QStandardItem_setSizeHint<()> for (&'a  QSize) {
  fn setSizeHint(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setSizeHintERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QStandardItem11setSizeHintERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn accessibleDescription<RetType, T: QStandardItem_accessibleDescription<RetType>>(&mut self, value: T) -> RetType {
    return value.accessibleDescription(self);
    // return 1;
  }
}

pub trait QStandardItem_accessibleDescription<RetType> {
  fn accessibleDescription(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  QString QStandardItem::accessibleDescription();
impl<'a> /*trait*/ QStandardItem_accessibleDescription<QString> for () {
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
  pub fn setTristate<RetType, T: QStandardItem_setTristate<RetType>>(&mut self, value: T) -> RetType {
    return value.setTristate(self);
    // return 1;
  }
}

pub trait QStandardItem_setTristate<RetType> {
  fn setTristate(self, rsthis: &mut QStandardItem) -> RetType;
}

// proto:  void QStandardItem::setTristate(bool tristate);
impl<'a> /*trait*/ QStandardItem_setTristate<()> for (i8) {
  fn setTristate(self, rsthis: &mut QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setTristateEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QStandardItem11setTristateEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

