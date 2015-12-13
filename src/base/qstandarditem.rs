// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbrush::QBrush;
use super::qstring::QString;
use super::qicon::QIcon;
use super::qvariant::QVariant;
use super::qdatastream::QDataStream;
use super::qfont::QFont;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QStandardItem::setChild(int row, QStandardItem * item);
  fn _ZN13QStandardItem8setChildEiPS_(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: QStandardItemModel * QStandardItem::model();
  fn _ZNK13QStandardItem5modelEv() -> i32;
  // proto: void QStandardItem::insertColumns(int column, int count);
  fn _ZN13QStandardItem13insertColumnsEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QStandardItem::setSelectable(bool selectable);
  fn _ZN13QStandardItem13setSelectableEb(arg0: int8_t) -> i32;
  // proto: int QStandardItem::column();
  fn _ZNK13QStandardItem6columnEv() -> i32;
  // proto: QString QStandardItem::whatsThis();
  fn _ZNK13QStandardItem9whatsThisEv() -> i32;
  // proto: QList<QStandardItem *> QStandardItem::takeColumn(int column);
  fn _ZN13QStandardItem10takeColumnEi(arg0: c_int) -> i32;
  // proto: void QStandardItem::setForeground(const QBrush & brush);
  fn _ZN13QStandardItem13setForegroundERK6QBrush(arg0: *const c_void) -> i32;
  // proto: bool QStandardItem::isEditable();
  fn _ZNK13QStandardItem10isEditableEv() -> i32;
  // proto: QIcon QStandardItem::icon();
  fn _ZNK13QStandardItem4iconEv() -> i32;
  // proto: void QStandardItem::setWhatsThis(const QString & whatsThis);
  fn _ZN13QStandardItem12setWhatsThisERK7QString(arg0: *const c_void) -> i32;
  // proto: QStandardItem * QStandardItem::takeChild(int row, int column);
  fn _ZN13QStandardItem9takeChildEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: int QStandardItem::type_();
  fn _ZNK13QStandardItem4typeEv() -> i32;
  // proto: QList<QStandardItem *> QStandardItem::takeRow(int row);
  fn _ZN13QStandardItem7takeRowEi(arg0: c_int) -> i32;
  // proto: int QStandardItem::row();
  fn _ZNK13QStandardItem3rowEv() -> i32;
  // proto: bool QStandardItem::isCheckable();
  fn _ZNK13QStandardItem11isCheckableEv() -> i32;
  // proto: QString QStandardItem::text();
  fn _ZNK13QStandardItem4textEv() -> i32;
  // proto: void QStandardItem::insertRows(int row, int count);
  fn _ZN13QStandardItem10insertRowsEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: bool QStandardItem::isDropEnabled();
  fn _ZNK13QStandardItem13isDropEnabledEv() -> i32;
  // proto: bool QStandardItem::hasChildren();
  fn _ZNK13QStandardItem11hasChildrenEv() -> i32;
  // proto: QString QStandardItem::statusTip();
  fn _ZNK13QStandardItem9statusTipEv() -> i32;
  // proto: void QStandardItem::setStatusTip(const QString & statusTip);
  fn _ZN13QStandardItem12setStatusTipERK7QString(arg0: *const c_void) -> i32;
  // proto: void QStandardItem::appendRow(QStandardItem * item);
  fn _ZN13QStandardItem9appendRowEPS_(arg0: *mut c_void) -> i32;
  // proto: void QStandardItem::setChild(int row, int column, QStandardItem * item);
  fn _ZN13QStandardItem8setChildEiiPS_(arg0: c_int, arg1: c_int, arg2: *mut c_void) -> i32;
  // proto: QModelIndex QStandardItem::index();
  fn _ZNK13QStandardItem5indexEv() -> i32;
  // proto: void QStandardItem::setIcon(const QIcon & icon);
  fn _ZN13QStandardItem7setIconERK5QIcon(arg0: *const c_void) -> i32;
  // proto: void QStandardItem::setToolTip(const QString & toolTip);
  fn _ZN13QStandardItem10setToolTipERK7QString(arg0: *const c_void) -> i32;
  // proto: void QStandardItem::setData(const QVariant & value, int role);
  fn _ZN13QStandardItem7setDataERK8QVarianti(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: QBrush QStandardItem::background();
  fn _ZNK13QStandardItem10backgroundEv() -> i32;
  // proto: QVariant QStandardItem::data(int role);
  fn _ZNK13QStandardItem4dataEi(arg0: c_int) -> i32;
  // proto: void QStandardItem::NewQStandardItem(const QStandardItem & other);
  fn _ZN13QStandardItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QStandardItem * QStandardItem::child(int row, int column);
  fn _ZNK13QStandardItem5childEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: bool QStandardItem::isSelectable();
  fn _ZNK13QStandardItem12isSelectableEv() -> i32;
  // proto: QString QStandardItem::toolTip();
  fn _ZNK13QStandardItem7toolTipEv() -> i32;
  // proto: void QStandardItem::setRowCount(int rows);
  fn _ZN13QStandardItem11setRowCountEi(arg0: c_int) -> i32;
  // proto: void QStandardItem::NewQStandardItem(const QString & text);
  fn _ZN13QStandardItemC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QStandardItem::write(QDataStream & out);
  fn _ZNK13QStandardItem5writeER11QDataStream(arg0: *mut c_void) -> i32;
  // proto: bool QStandardItem::isDragEnabled();
  fn _ZNK13QStandardItem13isDragEnabledEv() -> i32;
  // proto: void QStandardItem::setAccessibleText(const QString & accessibleText);
  fn _ZN13QStandardItem17setAccessibleTextERK7QString(arg0: *const c_void) -> i32;
  // proto: int QStandardItem::rowCount();
  fn _ZNK13QStandardItem8rowCountEv() -> i32;
  // proto: void QStandardItem::removeColumn(int column);
  fn _ZN13QStandardItem12removeColumnEi(arg0: c_int) -> i32;
  // proto: void QStandardItem::removeRow(int row);
  fn _ZN13QStandardItem9removeRowEi(arg0: c_int) -> i32;
  // proto: int QStandardItem::columnCount();
  fn _ZNK13QStandardItem11columnCountEv() -> i32;
  // proto: bool QStandardItem::isTristate();
  fn _ZNK13QStandardItem10isTristateEv() -> i32;
  // proto: QStandardItem * QStandardItem::parent();
  fn _ZNK13QStandardItem6parentEv() -> i32;
  // proto: void QStandardItem::insertRow(int row, QStandardItem * item);
  fn _ZN13QStandardItem9insertRowEiPS_(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: void QStandardItem::NewQStandardItem(const QIcon & icon, const QString & text);
  fn _ZN13QStandardItemC1ERK5QIconRK7QString(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QStandardItem::setFont(const QFont & font);
  fn _ZN13QStandardItem7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: void QStandardItem::removeColumns(int column, int count);
  fn _ZN13QStandardItem13removeColumnsEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QStandardItem::FreeQStandardItem();
  fn _ZN13QStandardItemD0Ev() -> i32;
  // proto: void QStandardItem::NewQStandardItem();
  fn _ZN13QStandardItemC1Ev(qthis: *mut c_void) -> i32;
  // proto: QFont QStandardItem::font();
  fn _ZNK13QStandardItem4fontEv() -> i32;
  // proto: void QStandardItem::setEditable(bool editable);
  fn _ZN13QStandardItem11setEditableEb(arg0: int8_t) -> i32;
  // proto: void QStandardItem::setText(const QString & text);
  fn _ZN13QStandardItem7setTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QStandardItem::NewQStandardItem(int rows, int columns);
  fn _ZN13QStandardItemC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> i32;
  // proto: bool QStandardItem::isEnabled();
  fn _ZNK13QStandardItem9isEnabledEv() -> i32;
  // proto: void QStandardItem::setDropEnabled(bool dropEnabled);
  fn _ZN13QStandardItem14setDropEnabledEb(arg0: int8_t) -> i32;
  // proto: void QStandardItem::setColumnCount(int columns);
  fn _ZN13QStandardItem14setColumnCountEi(arg0: c_int) -> i32;
  // proto: QString QStandardItem::accessibleText();
  fn _ZNK13QStandardItem14accessibleTextEv() -> i32;
  // proto: void QStandardItem::read(QDataStream & in);
  fn _ZN13QStandardItem4readER11QDataStream(arg0: *mut c_void) -> i32;
  // proto: void QStandardItem::setCheckable(bool checkable);
  fn _ZN13QStandardItem12setCheckableEb(arg0: int8_t) -> i32;
  // proto: void QStandardItem::setDragEnabled(bool dragEnabled);
  fn _ZN13QStandardItem14setDragEnabledEb(arg0: int8_t) -> i32;
  // proto: QBrush QStandardItem::foreground();
  fn _ZNK13QStandardItem10foregroundEv() -> i32;
  // proto: QStandardItem * QStandardItem::clone();
  fn _ZNK13QStandardItem5cloneEv() -> i32;
  // proto: void QStandardItem::removeRows(int row, int count);
  fn _ZN13QStandardItem10removeRowsEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QSize QStandardItem::sizeHint();
  fn _ZNK13QStandardItem8sizeHintEv() -> i32;
  // proto: void QStandardItem::setEnabled(bool enabled);
  fn _ZN13QStandardItem10setEnabledEb(arg0: int8_t) -> i32;
  // proto: void QStandardItem::setBackground(const QBrush & brush);
  fn _ZN13QStandardItem13setBackgroundERK6QBrush(arg0: *const c_void) -> i32;
  // proto: void QStandardItem::setAccessibleDescription(const QString & accessibleDescription);
  fn _ZN13QStandardItem24setAccessibleDescriptionERK7QString(arg0: *const c_void) -> i32;
  // proto: void QStandardItem::setSizeHint(const QSize & sizeHint);
  fn _ZN13QStandardItem11setSizeHintERK5QSize(arg0: *const c_void) -> i32;
  // proto: QString QStandardItem::accessibleDescription();
  fn _ZNK13QStandardItem21accessibleDescriptionEv() -> i32;
  // proto: void QStandardItem::setTristate(bool tristate);
  fn _ZN13QStandardItem11setTristateEb(arg0: int8_t) -> i32;
}

// body block begin
// class sizeof(QStandardItem)=1
pub struct QStandardItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStandardItem {
  pub fn setChild<T: QStandardItem_setChild>(&mut self, value: T) -> i32 {
    value.setChild(self);
    return 1;
  }
}

pub trait QStandardItem_setChild {
  fn setChild(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setChild(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_setChild for (i32, &'a mut QStandardItem) {
  fn setChild(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem8setChildEiPS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QStandardItem8setChildEiPS_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn model<T: QStandardItem_model>(&mut self, value: T) -> i32 {
    value.model(self);
    return 1;
  }
}

pub trait QStandardItem_model {
  fn model(self, this: &mut QStandardItem) -> i32;
}

// proto: QStandardItemModel * QStandardItem::model();
impl<'a> /*trait*/ QStandardItem_model for () {
  fn model(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5modelEv()};
    unsafe {_ZNK13QStandardItem5modelEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn insertColumns<T: QStandardItem_insertColumns>(&mut self, value: T) -> i32 {
    value.insertColumns(self);
    return 1;
  }
}

pub trait QStandardItem_insertColumns {
  fn insertColumns(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::insertColumns(int column, int count);
impl<'a> /*trait*/ QStandardItem_insertColumns for (i32, i32) {
  fn insertColumns(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13insertColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN13QStandardItem13insertColumnsEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setSelectable<T: QStandardItem_setSelectable>(&mut self, value: T) -> i32 {
    value.setSelectable(self);
    return 1;
  }
}

pub trait QStandardItem_setSelectable {
  fn setSelectable(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setSelectable(bool selectable);
impl<'a> /*trait*/ QStandardItem_setSelectable for (i8) {
  fn setSelectable(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13setSelectableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QStandardItem13setSelectableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn column<T: QStandardItem_column>(&mut self, value: T) -> i32 {
    value.column(self);
    return 1;
  }
}

pub trait QStandardItem_column {
  fn column(self, this: &mut QStandardItem) -> i32;
}

// proto: int QStandardItem::column();
impl<'a> /*trait*/ QStandardItem_column for () {
  fn column(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem6columnEv()};
    unsafe {_ZNK13QStandardItem6columnEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn whatsThis<T: QStandardItem_whatsThis>(&mut self, value: T) -> i32 {
    value.whatsThis(self);
    return 1;
  }
}

pub trait QStandardItem_whatsThis {
  fn whatsThis(self, this: &mut QStandardItem) -> i32;
}

// proto: QString QStandardItem::whatsThis();
impl<'a> /*trait*/ QStandardItem_whatsThis for () {
  fn whatsThis(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem9whatsThisEv()};
    unsafe {_ZNK13QStandardItem9whatsThisEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn takeColumn<T: QStandardItem_takeColumn>(&mut self, value: T) -> i32 {
    value.takeColumn(self);
    return 1;
  }
}

pub trait QStandardItem_takeColumn {
  fn takeColumn(self, this: &mut QStandardItem) -> i32;
}

// proto: QList<QStandardItem *> QStandardItem::takeColumn(int column);
impl<'a> /*trait*/ QStandardItem_takeColumn for (i32) {
  fn takeColumn(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10takeColumnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QStandardItem10takeColumnEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setForeground<T: QStandardItem_setForeground>(&mut self, value: T) -> i32 {
    value.setForeground(self);
    return 1;
  }
}

pub trait QStandardItem_setForeground {
  fn setForeground(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QStandardItem_setForeground for (&'a  QBrush) {
  fn setForeground(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItem13setForegroundERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isEditable<T: QStandardItem_isEditable>(&mut self, value: T) -> i32 {
    value.isEditable(self);
    return 1;
  }
}

pub trait QStandardItem_isEditable {
  fn isEditable(self, this: &mut QStandardItem) -> i32;
}

// proto: bool QStandardItem::isEditable();
impl<'a> /*trait*/ QStandardItem_isEditable for () {
  fn isEditable(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10isEditableEv()};
    unsafe {_ZNK13QStandardItem10isEditableEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn icon<T: QStandardItem_icon>(&mut self, value: T) -> i32 {
    value.icon(self);
    return 1;
  }
}

pub trait QStandardItem_icon {
  fn icon(self, this: &mut QStandardItem) -> i32;
}

// proto: QIcon QStandardItem::icon();
impl<'a> /*trait*/ QStandardItem_icon for () {
  fn icon(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4iconEv()};
    unsafe {_ZNK13QStandardItem4iconEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setWhatsThis<T: QStandardItem_setWhatsThis>(&mut self, value: T) -> i32 {
    value.setWhatsThis(self);
    return 1;
  }
}

pub trait QStandardItem_setWhatsThis {
  fn setWhatsThis(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setWhatsThis(const QString & whatsThis);
impl<'a> /*trait*/ QStandardItem_setWhatsThis for (&'a  QString) {
  fn setWhatsThis(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItem12setWhatsThisERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn takeChild<T: QStandardItem_takeChild>(&mut self, value: T) -> i32 {
    value.takeChild(self);
    return 1;
  }
}

pub trait QStandardItem_takeChild {
  fn takeChild(self, this: &mut QStandardItem) -> i32;
}

// proto: QStandardItem * QStandardItem::takeChild(int row, int column);
impl<'a> /*trait*/ QStandardItem_takeChild for (i32, i32) {
  fn takeChild(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem9takeChildEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN13QStandardItem9takeChildEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn type_<T: QStandardItem_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QStandardItem_type_ {
  fn type_(self, this: &mut QStandardItem) -> i32;
}

// proto: int QStandardItem::type_();
impl<'a> /*trait*/ QStandardItem_type_ for () {
  fn type_(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4typeEv()};
    unsafe {_ZNK13QStandardItem4typeEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn takeRow<T: QStandardItem_takeRow>(&mut self, value: T) -> i32 {
    value.takeRow(self);
    return 1;
  }
}

pub trait QStandardItem_takeRow {
  fn takeRow(self, this: &mut QStandardItem) -> i32;
}

// proto: QList<QStandardItem *> QStandardItem::takeRow(int row);
impl<'a> /*trait*/ QStandardItem_takeRow for (i32) {
  fn takeRow(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7takeRowEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QStandardItem7takeRowEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn row<T: QStandardItem_row>(&mut self, value: T) -> i32 {
    value.row(self);
    return 1;
  }
}

pub trait QStandardItem_row {
  fn row(self, this: &mut QStandardItem) -> i32;
}

// proto: int QStandardItem::row();
impl<'a> /*trait*/ QStandardItem_row for () {
  fn row(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem3rowEv()};
    unsafe {_ZNK13QStandardItem3rowEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isCheckable<T: QStandardItem_isCheckable>(&mut self, value: T) -> i32 {
    value.isCheckable(self);
    return 1;
  }
}

pub trait QStandardItem_isCheckable {
  fn isCheckable(self, this: &mut QStandardItem) -> i32;
}

// proto: bool QStandardItem::isCheckable();
impl<'a> /*trait*/ QStandardItem_isCheckable for () {
  fn isCheckable(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem11isCheckableEv()};
    unsafe {_ZNK13QStandardItem11isCheckableEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn text<T: QStandardItem_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QStandardItem_text {
  fn text(self, this: &mut QStandardItem) -> i32;
}

// proto: QString QStandardItem::text();
impl<'a> /*trait*/ QStandardItem_text for () {
  fn text(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4textEv()};
    unsafe {_ZNK13QStandardItem4textEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn insertRows<T: QStandardItem_insertRows>(&mut self, value: T) -> i32 {
    value.insertRows(self);
    return 1;
  }
}

pub trait QStandardItem_insertRows {
  fn insertRows(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::insertRows(int row, int count);
impl<'a> /*trait*/ QStandardItem_insertRows for (i32, i32) {
  fn insertRows(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10insertRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN13QStandardItem10insertRowsEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isDropEnabled<T: QStandardItem_isDropEnabled>(&mut self, value: T) -> i32 {
    value.isDropEnabled(self);
    return 1;
  }
}

pub trait QStandardItem_isDropEnabled {
  fn isDropEnabled(self, this: &mut QStandardItem) -> i32;
}

// proto: bool QStandardItem::isDropEnabled();
impl<'a> /*trait*/ QStandardItem_isDropEnabled for () {
  fn isDropEnabled(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem13isDropEnabledEv()};
    unsafe {_ZNK13QStandardItem13isDropEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn hasChildren<T: QStandardItem_hasChildren>(&mut self, value: T) -> i32 {
    value.hasChildren(self);
    return 1;
  }
}

pub trait QStandardItem_hasChildren {
  fn hasChildren(self, this: &mut QStandardItem) -> i32;
}

// proto: bool QStandardItem::hasChildren();
impl<'a> /*trait*/ QStandardItem_hasChildren for () {
  fn hasChildren(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem11hasChildrenEv()};
    unsafe {_ZNK13QStandardItem11hasChildrenEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn statusTip<T: QStandardItem_statusTip>(&mut self, value: T) -> i32 {
    value.statusTip(self);
    return 1;
  }
}

pub trait QStandardItem_statusTip {
  fn statusTip(self, this: &mut QStandardItem) -> i32;
}

// proto: QString QStandardItem::statusTip();
impl<'a> /*trait*/ QStandardItem_statusTip for () {
  fn statusTip(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem9statusTipEv()};
    unsafe {_ZNK13QStandardItem9statusTipEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setStatusTip<T: QStandardItem_setStatusTip>(&mut self, value: T) -> i32 {
    value.setStatusTip(self);
    return 1;
  }
}

pub trait QStandardItem_setStatusTip {
  fn setStatusTip(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QStandardItem_setStatusTip for (&'a  QString) {
  fn setStatusTip(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItem12setStatusTipERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn appendRow<T: QStandardItem_appendRow>(&mut self, value: T) -> i32 {
    value.appendRow(self);
    return 1;
  }
}

pub trait QStandardItem_appendRow {
  fn appendRow(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::appendRow(QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_appendRow for (&'a mut QStandardItem) {
  fn appendRow(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem9appendRowEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QStandardItem9appendRowEPS_(arg0)};
    return 1;
  }
}

// proto: void QStandardItem::setChild(int row, int column, QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_setChild for (i32, i32, &'a mut QStandardItem) {
  fn setChild(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem8setChildEiiPS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN13QStandardItem8setChildEiiPS_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn index<T: QStandardItem_index>(&mut self, value: T) -> i32 {
    value.index(self);
    return 1;
  }
}

pub trait QStandardItem_index {
  fn index(self, this: &mut QStandardItem) -> i32;
}

// proto: QModelIndex QStandardItem::index();
impl<'a> /*trait*/ QStandardItem_index for () {
  fn index(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5indexEv()};
    unsafe {_ZNK13QStandardItem5indexEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setIcon<T: QStandardItem_setIcon>(&mut self, value: T) -> i32 {
    value.setIcon(self);
    return 1;
  }
}

pub trait QStandardItem_setIcon {
  fn setIcon(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QStandardItem_setIcon for (&'a  QIcon) {
  fn setIcon(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItem7setIconERK5QIcon(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setToolTip<T: QStandardItem_setToolTip>(&mut self, value: T) -> i32 {
    value.setToolTip(self);
    return 1;
  }
}

pub trait QStandardItem_setToolTip {
  fn setToolTip(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QStandardItem_setToolTip for (&'a  QString) {
  fn setToolTip(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItem10setToolTipERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setData<T: QStandardItem_setData>(&mut self, value: T) -> i32 {
    value.setData(self);
    return 1;
  }
}

pub trait QStandardItem_setData {
  fn setData(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setData(const QVariant & value, int role);
impl<'a> /*trait*/ QStandardItem_setData for (&'a  QVariant, i32) {
  fn setData(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setDataERK8QVarianti()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN13QStandardItem7setDataERK8QVarianti(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn background<T: QStandardItem_background>(&mut self, value: T) -> i32 {
    value.background(self);
    return 1;
  }
}

pub trait QStandardItem_background {
  fn background(self, this: &mut QStandardItem) -> i32;
}

// proto: QBrush QStandardItem::background();
impl<'a> /*trait*/ QStandardItem_background for () {
  fn background(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10backgroundEv()};
    unsafe {_ZNK13QStandardItem10backgroundEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn data<T: QStandardItem_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QStandardItem_data {
  fn data(self, this: &mut QStandardItem) -> i32;
}

// proto: QVariant QStandardItem::data(int role);
impl<'a> /*trait*/ QStandardItem_data for (i32) {
  fn data(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4dataEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK13QStandardItem4dataEi(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItemC1ERKS_(qthis, arg0)};
    let rsthis = QStandardItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn child<T: QStandardItem_child>(&mut self, value: T) -> i32 {
    value.child(self);
    return 1;
  }
}

pub trait QStandardItem_child {
  fn child(self, this: &mut QStandardItem) -> i32;
}

// proto: QStandardItem * QStandardItem::child(int row, int column);
impl<'a> /*trait*/ QStandardItem_child for (i32, i32) {
  fn child(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5childEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK13QStandardItem5childEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isSelectable<T: QStandardItem_isSelectable>(&mut self, value: T) -> i32 {
    value.isSelectable(self);
    return 1;
  }
}

pub trait QStandardItem_isSelectable {
  fn isSelectable(self, this: &mut QStandardItem) -> i32;
}

// proto: bool QStandardItem::isSelectable();
impl<'a> /*trait*/ QStandardItem_isSelectable for () {
  fn isSelectable(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem12isSelectableEv()};
    unsafe {_ZNK13QStandardItem12isSelectableEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn toolTip<T: QStandardItem_toolTip>(&mut self, value: T) -> i32 {
    value.toolTip(self);
    return 1;
  }
}

pub trait QStandardItem_toolTip {
  fn toolTip(self, this: &mut QStandardItem) -> i32;
}

// proto: QString QStandardItem::toolTip();
impl<'a> /*trait*/ QStandardItem_toolTip for () {
  fn toolTip(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem7toolTipEv()};
    unsafe {_ZNK13QStandardItem7toolTipEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setRowCount<T: QStandardItem_setRowCount>(&mut self, value: T) -> i32 {
    value.setRowCount(self);
    return 1;
  }
}

pub trait QStandardItem_setRowCount {
  fn setRowCount(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setRowCount(int rows);
impl<'a> /*trait*/ QStandardItem_setRowCount for (i32) {
  fn setRowCount(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setRowCountEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QStandardItem11setRowCountEi(arg0)};
    return 1;
  }
}

// proto: void QStandardItem::NewQStandardItem(const QString & text);
impl<'a> /*trait*/ QStandardItem_NewQStandardItem for (&'a  QString) {
  fn NewQStandardItem(self) -> QStandardItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItemC1ERK7QString(qthis, arg0)};
    let rsthis = QStandardItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn write<T: QStandardItem_write>(&mut self, value: T) -> i32 {
    value.write(self);
    return 1;
  }
}

pub trait QStandardItem_write {
  fn write(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::write(QDataStream & out);
impl<'a> /*trait*/ QStandardItem_write for (&'a mut QDataStream) {
  fn write(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK13QStandardItem5writeER11QDataStream(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isDragEnabled<T: QStandardItem_isDragEnabled>(&mut self, value: T) -> i32 {
    value.isDragEnabled(self);
    return 1;
  }
}

pub trait QStandardItem_isDragEnabled {
  fn isDragEnabled(self, this: &mut QStandardItem) -> i32;
}

// proto: bool QStandardItem::isDragEnabled();
impl<'a> /*trait*/ QStandardItem_isDragEnabled for () {
  fn isDragEnabled(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem13isDragEnabledEv()};
    unsafe {_ZNK13QStandardItem13isDragEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setAccessibleText<T: QStandardItem_setAccessibleText>(&mut self, value: T) -> i32 {
    value.setAccessibleText(self);
    return 1;
  }
}

pub trait QStandardItem_setAccessibleText {
  fn setAccessibleText(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setAccessibleText(const QString & accessibleText);
impl<'a> /*trait*/ QStandardItem_setAccessibleText for (&'a  QString) {
  fn setAccessibleText(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem17setAccessibleTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItem17setAccessibleTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn rowCount<T: QStandardItem_rowCount>(&mut self, value: T) -> i32 {
    value.rowCount(self);
    return 1;
  }
}

pub trait QStandardItem_rowCount {
  fn rowCount(self, this: &mut QStandardItem) -> i32;
}

// proto: int QStandardItem::rowCount();
impl<'a> /*trait*/ QStandardItem_rowCount for () {
  fn rowCount(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem8rowCountEv()};
    unsafe {_ZNK13QStandardItem8rowCountEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn removeColumn<T: QStandardItem_removeColumn>(&mut self, value: T) -> i32 {
    value.removeColumn(self);
    return 1;
  }
}

pub trait QStandardItem_removeColumn {
  fn removeColumn(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::removeColumn(int column);
impl<'a> /*trait*/ QStandardItem_removeColumn for (i32) {
  fn removeColumn(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12removeColumnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QStandardItem12removeColumnEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn removeRow<T: QStandardItem_removeRow>(&mut self, value: T) -> i32 {
    value.removeRow(self);
    return 1;
  }
}

pub trait QStandardItem_removeRow {
  fn removeRow(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::removeRow(int row);
impl<'a> /*trait*/ QStandardItem_removeRow for (i32) {
  fn removeRow(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem9removeRowEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QStandardItem9removeRowEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn columnCount<T: QStandardItem_columnCount>(&mut self, value: T) -> i32 {
    value.columnCount(self);
    return 1;
  }
}

pub trait QStandardItem_columnCount {
  fn columnCount(self, this: &mut QStandardItem) -> i32;
}

// proto: int QStandardItem::columnCount();
impl<'a> /*trait*/ QStandardItem_columnCount for () {
  fn columnCount(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem11columnCountEv()};
    unsafe {_ZNK13QStandardItem11columnCountEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn isTristate<T: QStandardItem_isTristate>(&mut self, value: T) -> i32 {
    value.isTristate(self);
    return 1;
  }
}

pub trait QStandardItem_isTristate {
  fn isTristate(self, this: &mut QStandardItem) -> i32;
}

// proto: bool QStandardItem::isTristate();
impl<'a> /*trait*/ QStandardItem_isTristate for () {
  fn isTristate(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10isTristateEv()};
    unsafe {_ZNK13QStandardItem10isTristateEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn parent<T: QStandardItem_parent>(&mut self, value: T) -> i32 {
    value.parent(self);
    return 1;
  }
}

pub trait QStandardItem_parent {
  fn parent(self, this: &mut QStandardItem) -> i32;
}

// proto: QStandardItem * QStandardItem::parent();
impl<'a> /*trait*/ QStandardItem_parent for () {
  fn parent(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem6parentEv()};
    unsafe {_ZNK13QStandardItem6parentEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn insertRow<T: QStandardItem_insertRow>(&mut self, value: T) -> i32 {
    value.insertRow(self);
    return 1;
  }
}

pub trait QStandardItem_insertRow {
  fn insertRow(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::insertRow(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_insertRow for (i32, &'a mut QStandardItem) {
  fn insertRow(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem9insertRowEiPS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QStandardItem9insertRowEiPS_(arg0, arg1)};
    return 1;
  }
}

// proto: void QStandardItem::NewQStandardItem(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QStandardItem_NewQStandardItem for (&'a  QIcon, &'a  QString) {
  fn NewQStandardItem(self) -> QStandardItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemC1ERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItemC1ERK5QIconRK7QString(qthis, arg0, arg1)};
    let rsthis = QStandardItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setFont<T: QStandardItem_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QStandardItem_setFont {
  fn setFont(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setFont(const QFont & font);
impl<'a> /*trait*/ QStandardItem_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItem7setFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn removeColumns<T: QStandardItem_removeColumns>(&mut self, value: T) -> i32 {
    value.removeColumns(self);
    return 1;
  }
}

pub trait QStandardItem_removeColumns {
  fn removeColumns(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::removeColumns(int column, int count);
impl<'a> /*trait*/ QStandardItem_removeColumns for (i32, i32) {
  fn removeColumns(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13removeColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN13QStandardItem13removeColumnsEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn FreeQStandardItem<T: QStandardItem_FreeQStandardItem>(&mut self, value: T) -> i32 {
    value.FreeQStandardItem(self);
    return 1;
  }
}

pub trait QStandardItem_FreeQStandardItem {
  fn FreeQStandardItem(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::FreeQStandardItem();
impl<'a> /*trait*/ QStandardItem_FreeQStandardItem for () {
  fn FreeQStandardItem(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemD0Ev()};
    unsafe {_ZN13QStandardItemD0Ev()};
    return 1;
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
  pub fn font<T: QStandardItem_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QStandardItem_font {
  fn font(self, this: &mut QStandardItem) -> i32;
}

// proto: QFont QStandardItem::font();
impl<'a> /*trait*/ QStandardItem_font for () {
  fn font(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4fontEv()};
    unsafe {_ZNK13QStandardItem4fontEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setEditable<T: QStandardItem_setEditable>(&mut self, value: T) -> i32 {
    value.setEditable(self);
    return 1;
  }
}

pub trait QStandardItem_setEditable {
  fn setEditable(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setEditable(bool editable);
impl<'a> /*trait*/ QStandardItem_setEditable for (i8) {
  fn setEditable(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setEditableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QStandardItem11setEditableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setText<T: QStandardItem_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QStandardItem_setText {
  fn setText(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setText(const QString & text);
impl<'a> /*trait*/ QStandardItem_setText for (&'a  QString) {
  fn setText(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItem7setTextERK7QString(arg0)};
    return 1;
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
  pub fn isEnabled<T: QStandardItem_isEnabled>(&mut self, value: T) -> i32 {
    value.isEnabled(self);
    return 1;
  }
}

pub trait QStandardItem_isEnabled {
  fn isEnabled(self, this: &mut QStandardItem) -> i32;
}

// proto: bool QStandardItem::isEnabled();
impl<'a> /*trait*/ QStandardItem_isEnabled for () {
  fn isEnabled(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem9isEnabledEv()};
    unsafe {_ZNK13QStandardItem9isEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setDropEnabled<T: QStandardItem_setDropEnabled>(&mut self, value: T) -> i32 {
    value.setDropEnabled(self);
    return 1;
  }
}

pub trait QStandardItem_setDropEnabled {
  fn setDropEnabled(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setDropEnabled(bool dropEnabled);
impl<'a> /*trait*/ QStandardItem_setDropEnabled for (i8) {
  fn setDropEnabled(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem14setDropEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QStandardItem14setDropEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setColumnCount<T: QStandardItem_setColumnCount>(&mut self, value: T) -> i32 {
    value.setColumnCount(self);
    return 1;
  }
}

pub trait QStandardItem_setColumnCount {
  fn setColumnCount(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setColumnCount(int columns);
impl<'a> /*trait*/ QStandardItem_setColumnCount for (i32) {
  fn setColumnCount(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem14setColumnCountEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QStandardItem14setColumnCountEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn accessibleText<T: QStandardItem_accessibleText>(&mut self, value: T) -> i32 {
    value.accessibleText(self);
    return 1;
  }
}

pub trait QStandardItem_accessibleText {
  fn accessibleText(self, this: &mut QStandardItem) -> i32;
}

// proto: QString QStandardItem::accessibleText();
impl<'a> /*trait*/ QStandardItem_accessibleText for () {
  fn accessibleText(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem14accessibleTextEv()};
    unsafe {_ZNK13QStandardItem14accessibleTextEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn read<T: QStandardItem_read>(&mut self, value: T) -> i32 {
    value.read(self);
    return 1;
  }
}

pub trait QStandardItem_read {
  fn read(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::read(QDataStream & in);
impl<'a> /*trait*/ QStandardItem_read for (&'a mut QDataStream) {
  fn read(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QStandardItem4readER11QDataStream(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setCheckable<T: QStandardItem_setCheckable>(&mut self, value: T) -> i32 {
    value.setCheckable(self);
    return 1;
  }
}

pub trait QStandardItem_setCheckable {
  fn setCheckable(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setCheckable(bool checkable);
impl<'a> /*trait*/ QStandardItem_setCheckable for (i8) {
  fn setCheckable(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12setCheckableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QStandardItem12setCheckableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setDragEnabled<T: QStandardItem_setDragEnabled>(&mut self, value: T) -> i32 {
    value.setDragEnabled(self);
    return 1;
  }
}

pub trait QStandardItem_setDragEnabled {
  fn setDragEnabled(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setDragEnabled(bool dragEnabled);
impl<'a> /*trait*/ QStandardItem_setDragEnabled for (i8) {
  fn setDragEnabled(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem14setDragEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QStandardItem14setDragEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn foreground<T: QStandardItem_foreground>(&mut self, value: T) -> i32 {
    value.foreground(self);
    return 1;
  }
}

pub trait QStandardItem_foreground {
  fn foreground(self, this: &mut QStandardItem) -> i32;
}

// proto: QBrush QStandardItem::foreground();
impl<'a> /*trait*/ QStandardItem_foreground for () {
  fn foreground(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10foregroundEv()};
    unsafe {_ZNK13QStandardItem10foregroundEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn clone<T: QStandardItem_clone>(&mut self, value: T) -> i32 {
    value.clone(self);
    return 1;
  }
}

pub trait QStandardItem_clone {
  fn clone(self, this: &mut QStandardItem) -> i32;
}

// proto: QStandardItem * QStandardItem::clone();
impl<'a> /*trait*/ QStandardItem_clone for () {
  fn clone(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5cloneEv()};
    unsafe {_ZNK13QStandardItem5cloneEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn removeRows<T: QStandardItem_removeRows>(&mut self, value: T) -> i32 {
    value.removeRows(self);
    return 1;
  }
}

pub trait QStandardItem_removeRows {
  fn removeRows(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::removeRows(int row, int count);
impl<'a> /*trait*/ QStandardItem_removeRows for (i32, i32) {
  fn removeRows(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10removeRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN13QStandardItem10removeRowsEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn sizeHint<T: QStandardItem_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QStandardItem_sizeHint {
  fn sizeHint(self, this: &mut QStandardItem) -> i32;
}

// proto: QSize QStandardItem::sizeHint();
impl<'a> /*trait*/ QStandardItem_sizeHint for () {
  fn sizeHint(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem8sizeHintEv()};
    unsafe {_ZNK13QStandardItem8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setEnabled<T: QStandardItem_setEnabled>(&mut self, value: T) -> i32 {
    value.setEnabled(self);
    return 1;
  }
}

pub trait QStandardItem_setEnabled {
  fn setEnabled(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setEnabled(bool enabled);
impl<'a> /*trait*/ QStandardItem_setEnabled for (i8) {
  fn setEnabled(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10setEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QStandardItem10setEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setBackground<T: QStandardItem_setBackground>(&mut self, value: T) -> i32 {
    value.setBackground(self);
    return 1;
  }
}

pub trait QStandardItem_setBackground {
  fn setBackground(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QStandardItem_setBackground for (&'a  QBrush) {
  fn setBackground(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItem13setBackgroundERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setAccessibleDescription<T: QStandardItem_setAccessibleDescription>(&mut self, value: T) -> i32 {
    value.setAccessibleDescription(self);
    return 1;
  }
}

pub trait QStandardItem_setAccessibleDescription {
  fn setAccessibleDescription(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setAccessibleDescription(const QString & accessibleDescription);
impl<'a> /*trait*/ QStandardItem_setAccessibleDescription for (&'a  QString) {
  fn setAccessibleDescription(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem24setAccessibleDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItem24setAccessibleDescriptionERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setSizeHint<T: QStandardItem_setSizeHint>(&mut self, value: T) -> i32 {
    value.setSizeHint(self);
    return 1;
  }
}

pub trait QStandardItem_setSizeHint {
  fn setSizeHint(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setSizeHint(const QSize & sizeHint);
impl<'a> /*trait*/ QStandardItem_setSizeHint for (&'a  QSize) {
  fn setSizeHint(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setSizeHintERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QStandardItem11setSizeHintERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn accessibleDescription<T: QStandardItem_accessibleDescription>(&mut self, value: T) -> i32 {
    value.accessibleDescription(self);
    return 1;
  }
}

pub trait QStandardItem_accessibleDescription {
  fn accessibleDescription(self, this: &mut QStandardItem) -> i32;
}

// proto: QString QStandardItem::accessibleDescription();
impl<'a> /*trait*/ QStandardItem_accessibleDescription for () {
  fn accessibleDescription(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem21accessibleDescriptionEv()};
    unsafe {_ZNK13QStandardItem21accessibleDescriptionEv()};
    return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn setTristate<T: QStandardItem_setTristate>(&mut self, value: T) -> i32 {
    value.setTristate(self);
    return 1;
  }
}

pub trait QStandardItem_setTristate {
  fn setTristate(self, this: &mut QStandardItem) -> i32;
}

// proto: void QStandardItem::setTristate(bool tristate);
impl<'a> /*trait*/ QStandardItem_setTristate for (i8) {
  fn setTristate(self, this: &mut QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setTristateEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QStandardItem11setTristateEb(arg0)};
    return 1;
  }
}

