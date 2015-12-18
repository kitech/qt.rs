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
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QListWidget::dropEvent(QDropEvent * event);
  fn _ZN11QListWidget9dropEventEP10QDropEvent(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidget::itemClicked(QListWidgetItem * item);
  fn _ZN11QListWidget11itemClickedEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QListWidget::itemWidget(QListWidgetItem * item);
  fn _ZNK11QListWidget10itemWidgetEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QListWidget::itemPressed(QListWidgetItem * item);
  fn _ZN11QListWidget11itemPressedEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidget::NewQListWidget(QWidget * parent);
  fn _ZN11QListWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidget::currentRowChanged(int currentRow);
  fn _ZN11QListWidget17currentRowChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QListWidget::currentRow();
  fn _ZNK11QListWidget10currentRowEv(qthis: *mut c_void) -> c_int;
  // proto:  QListWidgetItem * QListWidget::item(int row);
  fn _ZNK11QListWidget4itemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QListWidgetItem * QListWidget::itemAt(const QPoint & p);
  fn _ZNK11QListWidget6itemAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QListWidget::insertItem(int row, const QString & label);
  fn _ZN11QListWidget10insertItemEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  int QListWidget::row(const QListWidgetItem * item);
  fn _ZNK11QListWidget3rowEPK15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QListWidget::openPersistentEditor(QListWidgetItem * item);
  fn _ZN11QListWidget20openPersistentEditorEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidget::clear();
  fn _ZN11QListWidget5clearEv(qthis: *mut c_void) ;
  // proto:  void QListWidget::editItem(QListWidgetItem * item);
  fn _ZN11QListWidget8editItemEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QListWidget::count();
  fn _ZNK11QListWidget5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QListWidget::setItemHidden(const QListWidgetItem * item, bool hide);
  fn _ZN11QListWidget13setItemHiddenEPK15QListWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  void QListWidget::FreeQListWidget();
  fn _ZN11QListWidgetD0Ev(qthis: *mut c_void) ;
  // proto:  void QListWidget::addItem(QListWidgetItem * item);
  fn _ZN11QListWidget7addItemEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidget::itemSelectionChanged();
  fn _ZN11QListWidget20itemSelectionChangedEv(qthis: *mut c_void) ;
  // proto:  QListWidgetItem * QListWidget::takeItem(int row);
  fn _ZN11QListWidget8takeItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QListWidget::isSortingEnabled();
  fn _ZNK11QListWidget16isSortingEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QListWidget::addItems(const QStringList & labels);
  fn _ZN11QListWidget8addItemsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QList<QListWidgetItem *> QListWidget::selectedItems();
  fn _ZNK11QListWidget13selectedItemsEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QListWidget::metaObject();
  fn _ZNK11QListWidget10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QListWidget::itemDoubleClicked(QListWidgetItem * item);
  fn _ZN11QListWidget17itemDoubleClickedEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidget::setItemSelected(const QListWidgetItem * item, bool select);
  fn _ZN11QListWidget15setItemSelectedEPK15QListWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  void QListWidget::insertItem(int row, QListWidgetItem * item);
  fn _ZN11QListWidget10insertItemEiP15QListWidgetItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QListWidget::setCurrentRow(int row);
  fn _ZN11QListWidget13setCurrentRowEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QListWidget::setSortingEnabled(bool enable);
  fn _ZN11QListWidget17setSortingEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QRect QListWidget::visualItemRect(const QListWidgetItem * item);
  fn _ZNK11QListWidget14visualItemRectEPK15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QListWidget::removeItemWidget(QListWidgetItem * item);
  fn _ZN11QListWidget16removeItemWidgetEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidget::itemActivated(QListWidgetItem * item);
  fn _ZN11QListWidget13itemActivatedEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidget::closePersistentEditor(QListWidgetItem * item);
  fn _ZN11QListWidget21closePersistentEditorEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidget::itemEntered(QListWidgetItem * item);
  fn _ZN11QListWidget11itemEnteredEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QListWidget::isItemHidden(const QListWidgetItem * item);
  fn _ZNK11QListWidget12isItemHiddenEPK15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QListWidget::itemChanged(QListWidgetItem * item);
  fn _ZN11QListWidget11itemChangedEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QListWidgetItem * QListWidget::itemAt(int x, int y);
  fn _ZNK11QListWidget6itemAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QListWidget::currentItemChanged(QListWidgetItem * current, QListWidgetItem * previous);
  fn _ZN11QListWidget18currentItemChangedEP15QListWidgetItemS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QListWidget::addItem(const QString & label);
  fn _ZN11QListWidget7addItemERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidget::insertItems(int row, const QStringList & labels);
  fn _ZN11QListWidget11insertItemsEiRK11QStringList(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QListWidget::currentTextChanged(const QString & currentText);
  fn _ZN11QListWidget18currentTextChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QListWidgetItem * QListWidget::currentItem();
  fn _ZNK11QListWidget11currentItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidget::setCurrentItem(QListWidgetItem * item);
  fn _ZN11QListWidget14setCurrentItemEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListWidget::setItemWidget(QListWidgetItem * item, QWidget * widget);
  fn _ZN11QListWidget13setItemWidgetEP15QListWidgetItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QListWidget::isItemSelected(const QListWidgetItem * item);
  fn _ZNK11QListWidget14isItemSelectedEPK15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QListWidget::NewQListWidget(const QListWidget & );
  fn _ZN11QListWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QListWidget)=1
pub struct QListWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QListWidget {
  pub fn dropEvent<RetType, T: QListWidget_dropEvent<RetType>>(&mut self, value: T) -> RetType {
    return value.dropEvent(self);
    // return 1;
  }
}

pub trait QListWidget_dropEvent<RetType> {
  fn dropEvent(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::dropEvent(QDropEvent * event);
impl<'a> /*trait*/ QListWidget_dropEvent<()> for (&'a mut QDropEvent) {
  fn dropEvent(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget9dropEventEP10QDropEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget9dropEventEP10QDropEvent(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemClicked<RetType, T: QListWidget_itemClicked<RetType>>(&mut self, value: T) -> RetType {
    return value.itemClicked(self);
    // return 1;
  }
}

pub trait QListWidget_itemClicked<RetType> {
  fn itemClicked(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::itemClicked(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemClicked<()> for (&'a mut QListWidgetItem) {
  fn itemClicked(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11itemClickedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget11itemClickedEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemWidget<RetType, T: QListWidget_itemWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.itemWidget(self);
    // return 1;
  }
}

pub trait QListWidget_itemWidget<RetType> {
  fn itemWidget(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  QWidget * QListWidget::itemWidget(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemWidget<QWidget> for (&'a mut QListWidgetItem) {
  fn itemWidget(self, rsthis: &mut QListWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget10itemWidgetEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QListWidget10itemWidgetEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemPressed<RetType, T: QListWidget_itemPressed<RetType>>(&mut self, value: T) -> RetType {
    return value.itemPressed(self);
    // return 1;
  }
}

pub trait QListWidget_itemPressed<RetType> {
  fn itemPressed(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::itemPressed(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemPressed<()> for (&'a mut QListWidgetItem) {
  fn itemPressed(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11itemPressedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget11itemPressedEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn currentRowChanged<RetType, T: QListWidget_currentRowChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.currentRowChanged(self);
    // return 1;
  }
}

pub trait QListWidget_currentRowChanged<RetType> {
  fn currentRowChanged(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::currentRowChanged(int currentRow);
impl<'a> /*trait*/ QListWidget_currentRowChanged<()> for (i32) {
  fn currentRowChanged(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget17currentRowChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QListWidget17currentRowChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn currentRow<RetType, T: QListWidget_currentRow<RetType>>(&mut self, value: T) -> RetType {
    return value.currentRow(self);
    // return 1;
  }
}

pub trait QListWidget_currentRow<RetType> {
  fn currentRow(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  int QListWidget::currentRow();
impl<'a> /*trait*/ QListWidget_currentRow<i32> for () {
  fn currentRow(self, rsthis: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget10currentRowEv()};
    let mut ret = unsafe {_ZNK11QListWidget10currentRowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn item<RetType, T: QListWidget_item<RetType>>(&mut self, value: T) -> RetType {
    return value.item(self);
    // return 1;
  }
}

pub trait QListWidget_item<RetType> {
  fn item(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  QListWidgetItem * QListWidget::item(int row);
impl<'a> /*trait*/ QListWidget_item<QListWidgetItem> for (i32) {
  fn item(self, rsthis: &mut QListWidget) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget4itemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QListWidget4itemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QListWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemAt<RetType, T: QListWidget_itemAt<RetType>>(&mut self, value: T) -> RetType {
    return value.itemAt(self);
    // return 1;
  }
}

pub trait QListWidget_itemAt<RetType> {
  fn itemAt(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  QListWidgetItem * QListWidget::itemAt(const QPoint & p);
impl<'a> /*trait*/ QListWidget_itemAt<QListWidgetItem> for (&'a  QPoint) {
  fn itemAt(self, rsthis: &mut QListWidget) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget6itemAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QListWidget6itemAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QListWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn insertItem<RetType, T: QListWidget_insertItem<RetType>>(&mut self, value: T) -> RetType {
    return value.insertItem(self);
    // return 1;
  }
}

pub trait QListWidget_insertItem<RetType> {
  fn insertItem(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::insertItem(int row, const QString & label);
impl<'a> /*trait*/ QListWidget_insertItem<()> for (i32, &'a  QString) {
  fn insertItem(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget10insertItemEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget10insertItemEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn row<RetType, T: QListWidget_row<RetType>>(&mut self, value: T) -> RetType {
    return value.row(self);
    // return 1;
  }
}

pub trait QListWidget_row<RetType> {
  fn row(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  int QListWidget::row(const QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_row<i32> for (&'a  QListWidgetItem) {
  fn row(self, rsthis: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget3rowEPK15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QListWidget3rowEPK15QListWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn openPersistentEditor<RetType, T: QListWidget_openPersistentEditor<RetType>>(&mut self, value: T) -> RetType {
    return value.openPersistentEditor(self);
    // return 1;
  }
}

pub trait QListWidget_openPersistentEditor<RetType> {
  fn openPersistentEditor(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::openPersistentEditor(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_openPersistentEditor<()> for (&'a mut QListWidgetItem) {
  fn openPersistentEditor(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget20openPersistentEditorEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget20openPersistentEditorEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn clear<RetType, T: QListWidget_clear<RetType>>(&mut self, value: T) -> RetType {
    return value.clear(self);
    // return 1;
  }
}

pub trait QListWidget_clear<RetType> {
  fn clear(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::clear();
impl<'a> /*trait*/ QListWidget_clear<()> for () {
  fn clear(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget5clearEv()};
     unsafe {_ZN11QListWidget5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn editItem<RetType, T: QListWidget_editItem<RetType>>(&mut self, value: T) -> RetType {
    return value.editItem(self);
    // return 1;
  }
}

pub trait QListWidget_editItem<RetType> {
  fn editItem(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::editItem(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_editItem<()> for (&'a mut QListWidgetItem) {
  fn editItem(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget8editItemEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget8editItemEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn count<RetType, T: QListWidget_count<RetType>>(&mut self, value: T) -> RetType {
    return value.count(self);
    // return 1;
  }
}

pub trait QListWidget_count<RetType> {
  fn count(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  int QListWidget::count();
impl<'a> /*trait*/ QListWidget_count<i32> for () {
  fn count(self, rsthis: &mut QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget5countEv()};
    let mut ret = unsafe {_ZNK11QListWidget5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn setItemHidden<RetType, T: QListWidget_setItemHidden<RetType>>(&mut self, value: T) -> RetType {
    return value.setItemHidden(self);
    // return 1;
  }
}

pub trait QListWidget_setItemHidden<RetType> {
  fn setItemHidden(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::setItemHidden(const QListWidgetItem * item, bool hide);
impl<'a> /*trait*/ QListWidget_setItemHidden<()> for (&'a  QListWidgetItem, i8) {
  fn setItemHidden(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget13setItemHiddenEPK15QListWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN11QListWidget13setItemHiddenEPK15QListWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn FreeQListWidget<RetType, T: QListWidget_FreeQListWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQListWidget(self);
    // return 1;
  }
}

pub trait QListWidget_FreeQListWidget<RetType> {
  fn FreeQListWidget(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::FreeQListWidget();
impl<'a> /*trait*/ QListWidget_FreeQListWidget<()> for () {
  fn FreeQListWidget(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidgetD0Ev()};
     unsafe {_ZN11QListWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn addItem<RetType, T: QListWidget_addItem<RetType>>(&mut self, value: T) -> RetType {
    return value.addItem(self);
    // return 1;
  }
}

pub trait QListWidget_addItem<RetType> {
  fn addItem(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::addItem(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_addItem<()> for (&'a mut QListWidgetItem) {
  fn addItem(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget7addItemEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget7addItemEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemSelectionChanged<RetType, T: QListWidget_itemSelectionChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.itemSelectionChanged(self);
    // return 1;
  }
}

pub trait QListWidget_itemSelectionChanged<RetType> {
  fn itemSelectionChanged(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::itemSelectionChanged();
impl<'a> /*trait*/ QListWidget_itemSelectionChanged<()> for () {
  fn itemSelectionChanged(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget20itemSelectionChangedEv()};
     unsafe {_ZN11QListWidget20itemSelectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn takeItem<RetType, T: QListWidget_takeItem<RetType>>(&mut self, value: T) -> RetType {
    return value.takeItem(self);
    // return 1;
  }
}

pub trait QListWidget_takeItem<RetType> {
  fn takeItem(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  QListWidgetItem * QListWidget::takeItem(int row);
impl<'a> /*trait*/ QListWidget_takeItem<QListWidgetItem> for (i32) {
  fn takeItem(self, rsthis: &mut QListWidget) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget8takeItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN11QListWidget8takeItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QListWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn isSortingEnabled<RetType, T: QListWidget_isSortingEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isSortingEnabled(self);
    // return 1;
  }
}

pub trait QListWidget_isSortingEnabled<RetType> {
  fn isSortingEnabled(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  bool QListWidget::isSortingEnabled();
impl<'a> /*trait*/ QListWidget_isSortingEnabled<i8> for () {
  fn isSortingEnabled(self, rsthis: &mut QListWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget16isSortingEnabledEv()};
    let mut ret = unsafe {_ZNK11QListWidget16isSortingEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn addItems<RetType, T: QListWidget_addItems<RetType>>(&mut self, value: T) -> RetType {
    return value.addItems(self);
    // return 1;
  }
}

pub trait QListWidget_addItems<RetType> {
  fn addItems(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::addItems(const QStringList & labels);
impl<'a> /*trait*/ QListWidget_addItems<()> for (&'a  QStringList) {
  fn addItems(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget8addItemsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget8addItemsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn selectedItems<RetType, T: QListWidget_selectedItems<RetType>>(&mut self, value: T) -> RetType {
    return value.selectedItems(self);
    // return 1;
  }
}

pub trait QListWidget_selectedItems<RetType> {
  fn selectedItems(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  QList<QListWidgetItem *> QListWidget::selectedItems();
impl<'a> /*trait*/ QListWidget_selectedItems<()> for () {
  fn selectedItems(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget13selectedItemsEv()};
     unsafe {_ZNK11QListWidget13selectedItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn metaObject<RetType, T: QListWidget_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QListWidget_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  const QMetaObject * QListWidget::metaObject();
impl<'a> /*trait*/ QListWidget_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget10metaObjectEv()};
     unsafe {_ZNK11QListWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemDoubleClicked<RetType, T: QListWidget_itemDoubleClicked<RetType>>(&mut self, value: T) -> RetType {
    return value.itemDoubleClicked(self);
    // return 1;
  }
}

pub trait QListWidget_itemDoubleClicked<RetType> {
  fn itemDoubleClicked(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::itemDoubleClicked(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemDoubleClicked<()> for (&'a mut QListWidgetItem) {
  fn itemDoubleClicked(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget17itemDoubleClickedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget17itemDoubleClickedEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn setItemSelected<RetType, T: QListWidget_setItemSelected<RetType>>(&mut self, value: T) -> RetType {
    return value.setItemSelected(self);
    // return 1;
  }
}

pub trait QListWidget_setItemSelected<RetType> {
  fn setItemSelected(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::setItemSelected(const QListWidgetItem * item, bool select);
impl<'a> /*trait*/ QListWidget_setItemSelected<()> for (&'a  QListWidgetItem, i8) {
  fn setItemSelected(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget15setItemSelectedEPK15QListWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN11QListWidget15setItemSelectedEPK15QListWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QListWidget::insertItem(int row, QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_insertItem<()> for (i32, &'a mut QListWidgetItem) {
  fn insertItem(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget10insertItemEiP15QListWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget10insertItemEiP15QListWidgetItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn setCurrentRow<RetType, T: QListWidget_setCurrentRow<RetType>>(&mut self, value: T) -> RetType {
    return value.setCurrentRow(self);
    // return 1;
  }
}

pub trait QListWidget_setCurrentRow<RetType> {
  fn setCurrentRow(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::setCurrentRow(int row);
impl<'a> /*trait*/ QListWidget_setCurrentRow<()> for (i32) {
  fn setCurrentRow(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget13setCurrentRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QListWidget13setCurrentRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn setSortingEnabled<RetType, T: QListWidget_setSortingEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setSortingEnabled(self);
    // return 1;
  }
}

pub trait QListWidget_setSortingEnabled<RetType> {
  fn setSortingEnabled(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::setSortingEnabled(bool enable);
impl<'a> /*trait*/ QListWidget_setSortingEnabled<()> for (i8) {
  fn setSortingEnabled(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget17setSortingEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QListWidget17setSortingEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn visualItemRect<RetType, T: QListWidget_visualItemRect<RetType>>(&mut self, value: T) -> RetType {
    return value.visualItemRect(self);
    // return 1;
  }
}

pub trait QListWidget_visualItemRect<RetType> {
  fn visualItemRect(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  QRect QListWidget::visualItemRect(const QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_visualItemRect<QRect> for (&'a  QListWidgetItem) {
  fn visualItemRect(self, rsthis: &mut QListWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget14visualItemRectEPK15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QListWidget14visualItemRectEPK15QListWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn removeItemWidget<RetType, T: QListWidget_removeItemWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.removeItemWidget(self);
    // return 1;
  }
}

pub trait QListWidget_removeItemWidget<RetType> {
  fn removeItemWidget(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::removeItemWidget(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_removeItemWidget<()> for (&'a mut QListWidgetItem) {
  fn removeItemWidget(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget16removeItemWidgetEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget16removeItemWidgetEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemActivated<RetType, T: QListWidget_itemActivated<RetType>>(&mut self, value: T) -> RetType {
    return value.itemActivated(self);
    // return 1;
  }
}

pub trait QListWidget_itemActivated<RetType> {
  fn itemActivated(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::itemActivated(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemActivated<()> for (&'a mut QListWidgetItem) {
  fn itemActivated(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget13itemActivatedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget13itemActivatedEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn closePersistentEditor<RetType, T: QListWidget_closePersistentEditor<RetType>>(&mut self, value: T) -> RetType {
    return value.closePersistentEditor(self);
    // return 1;
  }
}

pub trait QListWidget_closePersistentEditor<RetType> {
  fn closePersistentEditor(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::closePersistentEditor(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_closePersistentEditor<()> for (&'a mut QListWidgetItem) {
  fn closePersistentEditor(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget21closePersistentEditorEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget21closePersistentEditorEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemEntered<RetType, T: QListWidget_itemEntered<RetType>>(&mut self, value: T) -> RetType {
    return value.itemEntered(self);
    // return 1;
  }
}

pub trait QListWidget_itemEntered<RetType> {
  fn itemEntered(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::itemEntered(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemEntered<()> for (&'a mut QListWidgetItem) {
  fn itemEntered(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11itemEnteredEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget11itemEnteredEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn isItemHidden<RetType, T: QListWidget_isItemHidden<RetType>>(&mut self, value: T) -> RetType {
    return value.isItemHidden(self);
    // return 1;
  }
}

pub trait QListWidget_isItemHidden<RetType> {
  fn isItemHidden(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  bool QListWidget::isItemHidden(const QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_isItemHidden<i8> for (&'a  QListWidgetItem) {
  fn isItemHidden(self, rsthis: &mut QListWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget12isItemHiddenEPK15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QListWidget12isItemHiddenEPK15QListWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn itemChanged<RetType, T: QListWidget_itemChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.itemChanged(self);
    // return 1;
  }
}

pub trait QListWidget_itemChanged<RetType> {
  fn itemChanged(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::itemChanged(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemChanged<()> for (&'a mut QListWidgetItem) {
  fn itemChanged(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11itemChangedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget11itemChangedEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QListWidgetItem * QListWidget::itemAt(int x, int y);
impl<'a> /*trait*/ QListWidget_itemAt<QListWidgetItem> for (i32, i32) {
  fn itemAt(self, rsthis: &mut QListWidget) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QListWidget6itemAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QListWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn currentItemChanged<RetType, T: QListWidget_currentItemChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.currentItemChanged(self);
    // return 1;
  }
}

pub trait QListWidget_currentItemChanged<RetType> {
  fn currentItemChanged(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::currentItemChanged(QListWidgetItem * current, QListWidgetItem * previous);
impl<'a> /*trait*/ QListWidget_currentItemChanged<()> for (&'a mut QListWidgetItem, &'a mut QListWidgetItem) {
  fn currentItemChanged(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget18currentItemChangedEP15QListWidgetItemS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget18currentItemChangedEP15QListWidgetItemS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QListWidget::addItem(const QString & label);
impl<'a> /*trait*/ QListWidget_addItem<()> for (&'a  QString) {
  fn addItem(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget7addItemERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget7addItemERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn insertItems<RetType, T: QListWidget_insertItems<RetType>>(&mut self, value: T) -> RetType {
    return value.insertItems(self);
    // return 1;
  }
}

pub trait QListWidget_insertItems<RetType> {
  fn insertItems(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::insertItems(int row, const QStringList & labels);
impl<'a> /*trait*/ QListWidget_insertItems<()> for (i32, &'a  QStringList) {
  fn insertItems(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11insertItemsEiRK11QStringList()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget11insertItemsEiRK11QStringList(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn currentTextChanged<RetType, T: QListWidget_currentTextChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.currentTextChanged(self);
    // return 1;
  }
}

pub trait QListWidget_currentTextChanged<RetType> {
  fn currentTextChanged(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::currentTextChanged(const QString & currentText);
impl<'a> /*trait*/ QListWidget_currentTextChanged<()> for (&'a  QString) {
  fn currentTextChanged(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget18currentTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget18currentTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn currentItem<RetType, T: QListWidget_currentItem<RetType>>(&mut self, value: T) -> RetType {
    return value.currentItem(self);
    // return 1;
  }
}

pub trait QListWidget_currentItem<RetType> {
  fn currentItem(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  QListWidgetItem * QListWidget::currentItem();
impl<'a> /*trait*/ QListWidget_currentItem<QListWidgetItem> for () {
  fn currentItem(self, rsthis: &mut QListWidget) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget11currentItemEv()};
    let mut ret = unsafe {_ZNK11QListWidget11currentItemEv(rsthis.qclsinst)};
    let mut ret1 = QListWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn setCurrentItem<RetType, T: QListWidget_setCurrentItem<RetType>>(&mut self, value: T) -> RetType {
    return value.setCurrentItem(self);
    // return 1;
  }
}

pub trait QListWidget_setCurrentItem<RetType> {
  fn setCurrentItem(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::setCurrentItem(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_setCurrentItem<()> for (&'a mut QListWidgetItem) {
  fn setCurrentItem(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget14setCurrentItemEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget14setCurrentItemEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn setItemWidget<RetType, T: QListWidget_setItemWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.setItemWidget(self);
    // return 1;
  }
}

pub trait QListWidget_setItemWidget<RetType> {
  fn setItemWidget(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  void QListWidget::setItemWidget(QListWidgetItem * item, QWidget * widget);
impl<'a> /*trait*/ QListWidget_setItemWidget<()> for (&'a mut QListWidgetItem, &'a mut QWidget) {
  fn setItemWidget(self, rsthis: &mut QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget13setItemWidgetEP15QListWidgetItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget13setItemWidgetEP15QListWidgetItemP7QWidget(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn isItemSelected<RetType, T: QListWidget_isItemSelected<RetType>>(&mut self, value: T) -> RetType {
    return value.isItemSelected(self);
    // return 1;
  }
}

pub trait QListWidget_isItemSelected<RetType> {
  fn isItemSelected(self, rsthis: &mut QListWidget) -> RetType;
}

// proto:  bool QListWidget::isItemSelected(const QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_isItemSelected<i8> for (&'a  QListWidgetItem) {
  fn isItemSelected(self, rsthis: &mut QListWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget14isItemSelectedEPK15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QListWidget14isItemSelectedEPK15QListWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QListWidget::NewQListWidget(const QListWidget & );
impl<'a> /*trait*/ QListWidget_NewQListWidget for (&'a  QListWidget) {
  fn NewQListWidget(self) -> QListWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QListWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

