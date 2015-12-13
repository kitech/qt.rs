// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qdropevent::QDropEvent;
use super::qlistwidgetitem::QListWidgetItem;
use super::qwidget::QWidget;
use super::qpoint::QPoint;
use super::qstring::QString;
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QListWidget::dropEvent(QDropEvent * event);
  fn _ZN11QListWidget9dropEventEP10QDropEvent(arg0: *mut c_void) -> i32;
  // proto: void QListWidget::itemClicked(QListWidgetItem * item);
  fn _ZN11QListWidget11itemClickedEP15QListWidgetItem(arg0: *mut c_void) -> i32;
  // proto: QWidget * QListWidget::itemWidget(QListWidgetItem * item);
  fn _ZNK11QListWidget10itemWidgetEP15QListWidgetItem(arg0: *mut c_void) -> i32;
  // proto: void QListWidget::itemPressed(QListWidgetItem * item);
  fn _ZN11QListWidget11itemPressedEP15QListWidgetItem(arg0: *mut c_void) -> i32;
  // proto: void QListWidget::NewQListWidget(QWidget * parent);
  fn _ZN11QListWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QListWidget::currentRowChanged(int currentRow);
  fn _ZN11QListWidget17currentRowChangedEi(arg0: c_int) -> i32;
  // proto: int QListWidget::currentRow();
  fn _ZNK11QListWidget10currentRowEv() -> i32;
  // proto: QListWidgetItem * QListWidget::item(int row);
  fn _ZNK11QListWidget4itemEi(arg0: c_int) -> i32;
  // proto: QListWidgetItem * QListWidget::itemAt(const QPoint & p);
  fn _ZNK11QListWidget6itemAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QListWidget::insertItem(int row, const QString & label);
  fn _ZN11QListWidget10insertItemEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: int QListWidget::row(const QListWidgetItem * item);
  fn _ZNK11QListWidget3rowEPK15QListWidgetItem(arg0: *const c_void) -> i32;
  // proto: void QListWidget::openPersistentEditor(QListWidgetItem * item);
  fn _ZN11QListWidget20openPersistentEditorEP15QListWidgetItem(arg0: *mut c_void) -> i32;
  // proto: void QListWidget::clear();
  fn _ZN11QListWidget5clearEv() -> i32;
  // proto: void QListWidget::editItem(QListWidgetItem * item);
  fn _ZN11QListWidget8editItemEP15QListWidgetItem(arg0: *mut c_void) -> i32;
  // proto: int QListWidget::count();
  fn _ZNK11QListWidget5countEv() -> i32;
  // proto: void QListWidget::setItemHidden(const QListWidgetItem * item, bool hide);
  fn _ZN11QListWidget13setItemHiddenEPK15QListWidgetItemb(arg0: *const c_void, arg1: int8_t) -> i32;
  // proto: void QListWidget::FreeQListWidget();
  fn _ZN11QListWidgetD0Ev() -> i32;
  // proto: void QListWidget::addItem(QListWidgetItem * item);
  fn _ZN11QListWidget7addItemEP15QListWidgetItem(arg0: *mut c_void) -> i32;
  // proto: void QListWidget::itemSelectionChanged();
  fn _ZN11QListWidget20itemSelectionChangedEv() -> i32;
  // proto: QListWidgetItem * QListWidget::takeItem(int row);
  fn _ZN11QListWidget8takeItemEi(arg0: c_int) -> i32;
  // proto: bool QListWidget::isSortingEnabled();
  fn _ZNK11QListWidget16isSortingEnabledEv() -> i32;
  // proto: void QListWidget::addItems(const QStringList & labels);
  fn _ZN11QListWidget8addItemsERK11QStringList(arg0: *const c_void) -> i32;
  // proto: QList<QListWidgetItem *> QListWidget::selectedItems();
  fn _ZNK11QListWidget13selectedItemsEv() -> i32;
  // proto: const QMetaObject * QListWidget::metaObject();
  fn _ZNK11QListWidget10metaObjectEv() -> i32;
  // proto: void QListWidget::itemDoubleClicked(QListWidgetItem * item);
  fn _ZN11QListWidget17itemDoubleClickedEP15QListWidgetItem(arg0: *mut c_void) -> i32;
  // proto: void QListWidget::setItemSelected(const QListWidgetItem * item, bool select);
  fn _ZN11QListWidget15setItemSelectedEPK15QListWidgetItemb(arg0: *const c_void, arg1: int8_t) -> i32;
  // proto: void QListWidget::insertItem(int row, QListWidgetItem * item);
  fn _ZN11QListWidget10insertItemEiP15QListWidgetItem(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: void QListWidget::setCurrentRow(int row);
  fn _ZN11QListWidget13setCurrentRowEi(arg0: c_int) -> i32;
  // proto: void QListWidget::setSortingEnabled(bool enable);
  fn _ZN11QListWidget17setSortingEnabledEb(arg0: int8_t) -> i32;
  // proto: QRect QListWidget::visualItemRect(const QListWidgetItem * item);
  fn _ZNK11QListWidget14visualItemRectEPK15QListWidgetItem(arg0: *const c_void) -> i32;
  // proto: void QListWidget::removeItemWidget(QListWidgetItem * item);
  fn _ZN11QListWidget16removeItemWidgetEP15QListWidgetItem(arg0: *mut c_void) -> i32;
  // proto: void QListWidget::itemActivated(QListWidgetItem * item);
  fn _ZN11QListWidget13itemActivatedEP15QListWidgetItem(arg0: *mut c_void) -> i32;
  // proto: void QListWidget::closePersistentEditor(QListWidgetItem * item);
  fn _ZN11QListWidget21closePersistentEditorEP15QListWidgetItem(arg0: *mut c_void) -> i32;
  // proto: void QListWidget::itemEntered(QListWidgetItem * item);
  fn _ZN11QListWidget11itemEnteredEP15QListWidgetItem(arg0: *mut c_void) -> i32;
  // proto: bool QListWidget::isItemHidden(const QListWidgetItem * item);
  fn _ZNK11QListWidget12isItemHiddenEPK15QListWidgetItem(arg0: *const c_void) -> i32;
  // proto: void QListWidget::itemChanged(QListWidgetItem * item);
  fn _ZN11QListWidget11itemChangedEP15QListWidgetItem(arg0: *mut c_void) -> i32;
  // proto: QListWidgetItem * QListWidget::itemAt(int x, int y);
  fn _ZNK11QListWidget6itemAtEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QListWidget::currentItemChanged(QListWidgetItem * current, QListWidgetItem * previous);
  fn _ZN11QListWidget18currentItemChangedEP15QListWidgetItemS1_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: void QListWidget::addItem(const QString & label);
  fn _ZN11QListWidget7addItemERK7QString(arg0: *const c_void) -> i32;
  // proto: void QListWidget::insertItems(int row, const QStringList & labels);
  fn _ZN11QListWidget11insertItemsEiRK11QStringList(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QListWidget::currentTextChanged(const QString & currentText);
  fn _ZN11QListWidget18currentTextChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: QListWidgetItem * QListWidget::currentItem();
  fn _ZNK11QListWidget11currentItemEv() -> i32;
  // proto: void QListWidget::setCurrentItem(QListWidgetItem * item);
  fn _ZN11QListWidget14setCurrentItemEP15QListWidgetItem(arg0: *mut c_void) -> i32;
  // proto: void QListWidget::setItemWidget(QListWidgetItem * item, QWidget * widget);
  fn _ZN11QListWidget13setItemWidgetEP15QListWidgetItemP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: bool QListWidget::isItemSelected(const QListWidgetItem * item);
  fn _ZNK11QListWidget14isItemSelectedEPK15QListWidgetItem(arg0: *const c_void) -> i32;
  // proto: void QListWidget::NewQListWidget(const QListWidget & );
  fn _ZN11QListWidgetC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QListWidget)=1
pub struct QListWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QListWidget {
  pub fn dropEvent<T: QListWidget_dropEvent>(&mut self, value: T) -> i32 {
    value.dropEvent(self);
    return 1;
  }
}

pub trait QListWidget_dropEvent {
  fn dropEvent(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::dropEvent(QDropEvent * event);
impl<'a> /*trait*/ QListWidget_dropEvent for (&'a mut QDropEvent) {
  fn dropEvent(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget9dropEventEP10QDropEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget9dropEventEP10QDropEvent(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemClicked<T: QListWidget_itemClicked>(&mut self, value: T) -> i32 {
    value.itemClicked(self);
    return 1;
  }
}

pub trait QListWidget_itemClicked {
  fn itemClicked(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::itemClicked(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemClicked for (&'a mut QListWidgetItem) {
  fn itemClicked(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11itemClickedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget11itemClickedEP15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemWidget<T: QListWidget_itemWidget>(&mut self, value: T) -> i32 {
    value.itemWidget(self);
    return 1;
  }
}

pub trait QListWidget_itemWidget {
  fn itemWidget(self, this: &mut QListWidget) -> i32;
}

// proto: QWidget * QListWidget::itemWidget(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemWidget for (&'a mut QListWidgetItem) {
  fn itemWidget(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget10itemWidgetEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK11QListWidget10itemWidgetEP15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemPressed<T: QListWidget_itemPressed>(&mut self, value: T) -> i32 {
    value.itemPressed(self);
    return 1;
  }
}

pub trait QListWidget_itemPressed {
  fn itemPressed(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::itemPressed(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemPressed for (&'a mut QListWidgetItem) {
  fn itemPressed(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11itemPressedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget11itemPressedEP15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn NewQListWidget<T: QListWidget_NewQListWidget>(value: T) -> QListWidget {
    let rsthis = value.NewQListWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QListWidget_NewQListWidget {
  fn NewQListWidget(self) -> QListWidget;
}

// proto: void QListWidget::NewQListWidget(QWidget * parent);
impl<'a> /*trait*/ QListWidget_NewQListWidget for (&'a mut QWidget) {
  fn NewQListWidget(self) -> QListWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidgetC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidgetC1EP7QWidget(qthis, arg0)};
    let rsthis = QListWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn currentRowChanged<T: QListWidget_currentRowChanged>(&mut self, value: T) -> i32 {
    value.currentRowChanged(self);
    return 1;
  }
}

pub trait QListWidget_currentRowChanged {
  fn currentRowChanged(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::currentRowChanged(int currentRow);
impl<'a> /*trait*/ QListWidget_currentRowChanged for (i32) {
  fn currentRowChanged(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget17currentRowChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QListWidget17currentRowChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn currentRow<T: QListWidget_currentRow>(&mut self, value: T) -> i32 {
    value.currentRow(self);
    return 1;
  }
}

pub trait QListWidget_currentRow {
  fn currentRow(self, this: &mut QListWidget) -> i32;
}

// proto: int QListWidget::currentRow();
impl<'a> /*trait*/ QListWidget_currentRow for () {
  fn currentRow(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget10currentRowEv()};
    unsafe {_ZNK11QListWidget10currentRowEv()};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn item<T: QListWidget_item>(&mut self, value: T) -> i32 {
    value.item(self);
    return 1;
  }
}

pub trait QListWidget_item {
  fn item(self, this: &mut QListWidget) -> i32;
}

// proto: QListWidgetItem * QListWidget::item(int row);
impl<'a> /*trait*/ QListWidget_item for (i32) {
  fn item(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget4itemEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QListWidget4itemEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemAt<T: QListWidget_itemAt>(&mut self, value: T) -> i32 {
    value.itemAt(self);
    return 1;
  }
}

pub trait QListWidget_itemAt {
  fn itemAt(self, this: &mut QListWidget) -> i32;
}

// proto: QListWidgetItem * QListWidget::itemAt(const QPoint & p);
impl<'a> /*trait*/ QListWidget_itemAt for (&'a  QPoint) {
  fn itemAt(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget6itemAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK11QListWidget6itemAtERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn insertItem<T: QListWidget_insertItem>(&mut self, value: T) -> i32 {
    value.insertItem(self);
    return 1;
  }
}

pub trait QListWidget_insertItem {
  fn insertItem(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::insertItem(int row, const QString & label);
impl<'a> /*trait*/ QListWidget_insertItem for (i32, &'a  QString) {
  fn insertItem(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget10insertItemEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN11QListWidget10insertItemEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn row<T: QListWidget_row>(&mut self, value: T) -> i32 {
    value.row(self);
    return 1;
  }
}

pub trait QListWidget_row {
  fn row(self, this: &mut QListWidget) -> i32;
}

// proto: int QListWidget::row(const QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_row for (&'a  QListWidgetItem) {
  fn row(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget3rowEPK15QListWidgetItem()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK11QListWidget3rowEPK15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn openPersistentEditor<T: QListWidget_openPersistentEditor>(&mut self, value: T) -> i32 {
    value.openPersistentEditor(self);
    return 1;
  }
}

pub trait QListWidget_openPersistentEditor {
  fn openPersistentEditor(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::openPersistentEditor(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_openPersistentEditor for (&'a mut QListWidgetItem) {
  fn openPersistentEditor(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget20openPersistentEditorEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget20openPersistentEditorEP15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn clear<T: QListWidget_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QListWidget_clear {
  fn clear(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::clear();
impl<'a> /*trait*/ QListWidget_clear for () {
  fn clear(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget5clearEv()};
    unsafe {_ZN11QListWidget5clearEv()};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn editItem<T: QListWidget_editItem>(&mut self, value: T) -> i32 {
    value.editItem(self);
    return 1;
  }
}

pub trait QListWidget_editItem {
  fn editItem(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::editItem(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_editItem for (&'a mut QListWidgetItem) {
  fn editItem(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget8editItemEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget8editItemEP15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn count<T: QListWidget_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QListWidget_count {
  fn count(self, this: &mut QListWidget) -> i32;
}

// proto: int QListWidget::count();
impl<'a> /*trait*/ QListWidget_count for () {
  fn count(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget5countEv()};
    unsafe {_ZNK11QListWidget5countEv()};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn setItemHidden<T: QListWidget_setItemHidden>(&mut self, value: T) -> i32 {
    value.setItemHidden(self);
    return 1;
  }
}

pub trait QListWidget_setItemHidden {
  fn setItemHidden(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::setItemHidden(const QListWidgetItem * item, bool hide);
impl<'a> /*trait*/ QListWidget_setItemHidden for (&'a  QListWidgetItem, i8) {
  fn setItemHidden(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget13setItemHiddenEPK15QListWidgetItemb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN11QListWidget13setItemHiddenEPK15QListWidgetItemb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn FreeQListWidget<T: QListWidget_FreeQListWidget>(&mut self, value: T) -> i32 {
    value.FreeQListWidget(self);
    return 1;
  }
}

pub trait QListWidget_FreeQListWidget {
  fn FreeQListWidget(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::FreeQListWidget();
impl<'a> /*trait*/ QListWidget_FreeQListWidget for () {
  fn FreeQListWidget(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidgetD0Ev()};
    unsafe {_ZN11QListWidgetD0Ev()};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn addItem<T: QListWidget_addItem>(&mut self, value: T) -> i32 {
    value.addItem(self);
    return 1;
  }
}

pub trait QListWidget_addItem {
  fn addItem(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::addItem(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_addItem for (&'a mut QListWidgetItem) {
  fn addItem(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget7addItemEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget7addItemEP15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemSelectionChanged<T: QListWidget_itemSelectionChanged>(&mut self, value: T) -> i32 {
    value.itemSelectionChanged(self);
    return 1;
  }
}

pub trait QListWidget_itemSelectionChanged {
  fn itemSelectionChanged(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::itemSelectionChanged();
impl<'a> /*trait*/ QListWidget_itemSelectionChanged for () {
  fn itemSelectionChanged(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget20itemSelectionChangedEv()};
    unsafe {_ZN11QListWidget20itemSelectionChangedEv()};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn takeItem<T: QListWidget_takeItem>(&mut self, value: T) -> i32 {
    value.takeItem(self);
    return 1;
  }
}

pub trait QListWidget_takeItem {
  fn takeItem(self, this: &mut QListWidget) -> i32;
}

// proto: QListWidgetItem * QListWidget::takeItem(int row);
impl<'a> /*trait*/ QListWidget_takeItem for (i32) {
  fn takeItem(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget8takeItemEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QListWidget8takeItemEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn isSortingEnabled<T: QListWidget_isSortingEnabled>(&mut self, value: T) -> i32 {
    value.isSortingEnabled(self);
    return 1;
  }
}

pub trait QListWidget_isSortingEnabled {
  fn isSortingEnabled(self, this: &mut QListWidget) -> i32;
}

// proto: bool QListWidget::isSortingEnabled();
impl<'a> /*trait*/ QListWidget_isSortingEnabled for () {
  fn isSortingEnabled(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget16isSortingEnabledEv()};
    unsafe {_ZNK11QListWidget16isSortingEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn addItems<T: QListWidget_addItems>(&mut self, value: T) -> i32 {
    value.addItems(self);
    return 1;
  }
}

pub trait QListWidget_addItems {
  fn addItems(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::addItems(const QStringList & labels);
impl<'a> /*trait*/ QListWidget_addItems for (&'a  QStringList) {
  fn addItems(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget8addItemsERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QListWidget8addItemsERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn selectedItems<T: QListWidget_selectedItems>(&mut self, value: T) -> i32 {
    value.selectedItems(self);
    return 1;
  }
}

pub trait QListWidget_selectedItems {
  fn selectedItems(self, this: &mut QListWidget) -> i32;
}

// proto: QList<QListWidgetItem *> QListWidget::selectedItems();
impl<'a> /*trait*/ QListWidget_selectedItems for () {
  fn selectedItems(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget13selectedItemsEv()};
    unsafe {_ZNK11QListWidget13selectedItemsEv()};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn metaObject<T: QListWidget_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QListWidget_metaObject {
  fn metaObject(self, this: &mut QListWidget) -> i32;
}

// proto: const QMetaObject * QListWidget::metaObject();
impl<'a> /*trait*/ QListWidget_metaObject for () {
  fn metaObject(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget10metaObjectEv()};
    unsafe {_ZNK11QListWidget10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemDoubleClicked<T: QListWidget_itemDoubleClicked>(&mut self, value: T) -> i32 {
    value.itemDoubleClicked(self);
    return 1;
  }
}

pub trait QListWidget_itemDoubleClicked {
  fn itemDoubleClicked(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::itemDoubleClicked(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemDoubleClicked for (&'a mut QListWidgetItem) {
  fn itemDoubleClicked(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget17itemDoubleClickedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget17itemDoubleClickedEP15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn setItemSelected<T: QListWidget_setItemSelected>(&mut self, value: T) -> i32 {
    value.setItemSelected(self);
    return 1;
  }
}

pub trait QListWidget_setItemSelected {
  fn setItemSelected(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::setItemSelected(const QListWidgetItem * item, bool select);
impl<'a> /*trait*/ QListWidget_setItemSelected for (&'a  QListWidgetItem, i8) {
  fn setItemSelected(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget15setItemSelectedEPK15QListWidgetItemb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN11QListWidget15setItemSelectedEPK15QListWidgetItemb(arg0, arg1)};
    return 1;
  }
}

// proto: void QListWidget::insertItem(int row, QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_insertItem for (i32, &'a mut QListWidgetItem) {
  fn insertItem(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget10insertItemEiP15QListWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget10insertItemEiP15QListWidgetItem(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn setCurrentRow<T: QListWidget_setCurrentRow>(&mut self, value: T) -> i32 {
    value.setCurrentRow(self);
    return 1;
  }
}

pub trait QListWidget_setCurrentRow {
  fn setCurrentRow(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::setCurrentRow(int row);
impl<'a> /*trait*/ QListWidget_setCurrentRow for (i32) {
  fn setCurrentRow(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget13setCurrentRowEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QListWidget13setCurrentRowEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn setSortingEnabled<T: QListWidget_setSortingEnabled>(&mut self, value: T) -> i32 {
    value.setSortingEnabled(self);
    return 1;
  }
}

pub trait QListWidget_setSortingEnabled {
  fn setSortingEnabled(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::setSortingEnabled(bool enable);
impl<'a> /*trait*/ QListWidget_setSortingEnabled for (i8) {
  fn setSortingEnabled(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget17setSortingEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QListWidget17setSortingEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn visualItemRect<T: QListWidget_visualItemRect>(&mut self, value: T) -> i32 {
    value.visualItemRect(self);
    return 1;
  }
}

pub trait QListWidget_visualItemRect {
  fn visualItemRect(self, this: &mut QListWidget) -> i32;
}

// proto: QRect QListWidget::visualItemRect(const QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_visualItemRect for (&'a  QListWidgetItem) {
  fn visualItemRect(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget14visualItemRectEPK15QListWidgetItem()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK11QListWidget14visualItemRectEPK15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn removeItemWidget<T: QListWidget_removeItemWidget>(&mut self, value: T) -> i32 {
    value.removeItemWidget(self);
    return 1;
  }
}

pub trait QListWidget_removeItemWidget {
  fn removeItemWidget(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::removeItemWidget(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_removeItemWidget for (&'a mut QListWidgetItem) {
  fn removeItemWidget(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget16removeItemWidgetEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget16removeItemWidgetEP15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemActivated<T: QListWidget_itemActivated>(&mut self, value: T) -> i32 {
    value.itemActivated(self);
    return 1;
  }
}

pub trait QListWidget_itemActivated {
  fn itemActivated(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::itemActivated(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemActivated for (&'a mut QListWidgetItem) {
  fn itemActivated(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget13itemActivatedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget13itemActivatedEP15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn closePersistentEditor<T: QListWidget_closePersistentEditor>(&mut self, value: T) -> i32 {
    value.closePersistentEditor(self);
    return 1;
  }
}

pub trait QListWidget_closePersistentEditor {
  fn closePersistentEditor(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::closePersistentEditor(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_closePersistentEditor for (&'a mut QListWidgetItem) {
  fn closePersistentEditor(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget21closePersistentEditorEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget21closePersistentEditorEP15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemEntered<T: QListWidget_itemEntered>(&mut self, value: T) -> i32 {
    value.itemEntered(self);
    return 1;
  }
}

pub trait QListWidget_itemEntered {
  fn itemEntered(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::itemEntered(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemEntered for (&'a mut QListWidgetItem) {
  fn itemEntered(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11itemEnteredEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget11itemEnteredEP15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn isItemHidden<T: QListWidget_isItemHidden>(&mut self, value: T) -> i32 {
    value.isItemHidden(self);
    return 1;
  }
}

pub trait QListWidget_isItemHidden {
  fn isItemHidden(self, this: &mut QListWidget) -> i32;
}

// proto: bool QListWidget::isItemHidden(const QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_isItemHidden for (&'a  QListWidgetItem) {
  fn isItemHidden(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget12isItemHiddenEPK15QListWidgetItem()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK11QListWidget12isItemHiddenEPK15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemChanged<T: QListWidget_itemChanged>(&mut self, value: T) -> i32 {
    value.itemChanged(self);
    return 1;
  }
}

pub trait QListWidget_itemChanged {
  fn itemChanged(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::itemChanged(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemChanged for (&'a mut QListWidgetItem) {
  fn itemChanged(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11itemChangedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget11itemChangedEP15QListWidgetItem(arg0)};
    return 1;
  }
}

// proto: QListWidgetItem * QListWidget::itemAt(int x, int y);
impl<'a> /*trait*/ QListWidget_itemAt for (i32, i32) {
  fn itemAt(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK11QListWidget6itemAtEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn currentItemChanged<T: QListWidget_currentItemChanged>(&mut self, value: T) -> i32 {
    value.currentItemChanged(self);
    return 1;
  }
}

pub trait QListWidget_currentItemChanged {
  fn currentItemChanged(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::currentItemChanged(QListWidgetItem * current, QListWidgetItem * previous);
impl<'a> /*trait*/ QListWidget_currentItemChanged for (&'a mut QListWidgetItem, &'a mut QListWidgetItem) {
  fn currentItemChanged(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget18currentItemChangedEP15QListWidgetItemS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget18currentItemChangedEP15QListWidgetItemS1_(arg0, arg1)};
    return 1;
  }
}

// proto: void QListWidget::addItem(const QString & label);
impl<'a> /*trait*/ QListWidget_addItem for (&'a  QString) {
  fn addItem(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget7addItemERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QListWidget7addItemERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn insertItems<T: QListWidget_insertItems>(&mut self, value: T) -> i32 {
    value.insertItems(self);
    return 1;
  }
}

pub trait QListWidget_insertItems {
  fn insertItems(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::insertItems(int row, const QStringList & labels);
impl<'a> /*trait*/ QListWidget_insertItems for (i32, &'a  QStringList) {
  fn insertItems(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11insertItemsEiRK11QStringList()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN11QListWidget11insertItemsEiRK11QStringList(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn currentTextChanged<T: QListWidget_currentTextChanged>(&mut self, value: T) -> i32 {
    value.currentTextChanged(self);
    return 1;
  }
}

pub trait QListWidget_currentTextChanged {
  fn currentTextChanged(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::currentTextChanged(const QString & currentText);
impl<'a> /*trait*/ QListWidget_currentTextChanged for (&'a  QString) {
  fn currentTextChanged(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget18currentTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QListWidget18currentTextChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn currentItem<T: QListWidget_currentItem>(&mut self, value: T) -> i32 {
    value.currentItem(self);
    return 1;
  }
}

pub trait QListWidget_currentItem {
  fn currentItem(self, this: &mut QListWidget) -> i32;
}

// proto: QListWidgetItem * QListWidget::currentItem();
impl<'a> /*trait*/ QListWidget_currentItem for () {
  fn currentItem(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget11currentItemEv()};
    unsafe {_ZNK11QListWidget11currentItemEv()};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn setCurrentItem<T: QListWidget_setCurrentItem>(&mut self, value: T) -> i32 {
    value.setCurrentItem(self);
    return 1;
  }
}

pub trait QListWidget_setCurrentItem {
  fn setCurrentItem(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::setCurrentItem(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_setCurrentItem for (&'a mut QListWidgetItem) {
  fn setCurrentItem(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget14setCurrentItemEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget14setCurrentItemEP15QListWidgetItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn setItemWidget<T: QListWidget_setItemWidget>(&mut self, value: T) -> i32 {
    value.setItemWidget(self);
    return 1;
  }
}

pub trait QListWidget_setItemWidget {
  fn setItemWidget(self, this: &mut QListWidget) -> i32;
}

// proto: void QListWidget::setItemWidget(QListWidgetItem * item, QWidget * widget);
impl<'a> /*trait*/ QListWidget_setItemWidget for (&'a mut QListWidgetItem, &'a mut QWidget) {
  fn setItemWidget(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget13setItemWidgetEP15QListWidgetItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidget13setItemWidgetEP15QListWidgetItemP7QWidget(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn isItemSelected<T: QListWidget_isItemSelected>(&mut self, value: T) -> i32 {
    value.isItemSelected(self);
    return 1;
  }
}

pub trait QListWidget_isItemSelected {
  fn isItemSelected(self, this: &mut QListWidget) -> i32;
}

// proto: bool QListWidget::isItemSelected(const QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_isItemSelected for (&'a  QListWidgetItem) {
  fn isItemSelected(self, this: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget14isItemSelectedEPK15QListWidgetItem()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK11QListWidget14isItemSelectedEPK15QListWidgetItem(arg0)};
    return 1;
  }
}

// proto: void QListWidget::NewQListWidget(const QListWidget & );
impl<'a> /*trait*/ QListWidget_NewQListWidget for (&'a  QListWidget) {
  fn NewQListWidget(self) -> QListWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QListWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QListWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

