// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtreewidgetitem::QTreeWidgetItem;
use super::qwidget::QWidget;
use super::qstringlist::QStringList;
use super::qpoint::QPoint;
use super::qitemselectionmodel::QItemSelectionModel;
use super::qrect::QRect;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTreeWidget::itemDoubleClicked(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget17itemDoubleClickedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QTreeWidget::itemExpanded(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget12itemExpandedEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeWidget::setColumnCount(int columns);
  fn _ZN11QTreeWidget14setColumnCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTreeWidget::FreeQTreeWidget();
  fn _ZN11QTreeWidgetD0Ev(qthis: *mut c_void) ;
  // proto:  QList<QTreeWidgetItem *> QTreeWidget::selectedItems();
  fn _ZNK11QTreeWidget13selectedItemsEv(qthis: *mut c_void) ;
  // proto:  bool QTreeWidget::isItemExpanded(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget14isItemExpandedEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QTreeWidget::NewQTreeWidget(const QTreeWidget & );
  fn _ZN11QTreeWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeWidget::setItemHidden(const QTreeWidgetItem * item, bool hide);
  fn _ZN11QTreeWidget13setItemHiddenEPK15QTreeWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  int QTreeWidget::indexOfTopLevelItem(QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget19indexOfTopLevelItemEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QTreeWidget::insertTopLevelItem(int index, QTreeWidgetItem * item);
  fn _ZN11QTreeWidget18insertTopLevelItemEiP15QTreeWidgetItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTreeWidget::setItemWidget(QTreeWidgetItem * item, int column, QWidget * widget);
  fn _ZN11QTreeWidget13setItemWidgetEP15QTreeWidgetItemiP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) ;
  // proto:  bool QTreeWidget::isItemSelected(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget14isItemSelectedEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  int QTreeWidget::currentColumn();
  fn _ZNK11QTreeWidget13currentColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTreeWidget::isFirstItemColumnSpanned(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget24isFirstItemColumnSpannedEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QTreeWidget::clear();
  fn _ZN11QTreeWidget5clearEv(qthis: *mut c_void) ;
  // proto:  void QTreeWidget::itemPressed(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget11itemPressedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QTreeWidget::setHeaderLabels(const QStringList & labels);
  fn _ZN11QTreeWidget15setHeaderLabelsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTreeWidgetItem * QTreeWidget::invisibleRootItem();
  fn _ZNK11QTreeWidget17invisibleRootItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QTreeWidget::metaObject();
  fn _ZNK11QTreeWidget10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTreeWidget::itemEntered(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget11itemEnteredEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  QTreeWidgetItem * QTreeWidget::itemBelow(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget9itemBelowEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QTreeWidget::sortColumn();
  fn _ZNK11QTreeWidget10sortColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  QTreeWidgetItem * QTreeWidget::itemAt(int x, int y);
  fn _ZNK11QTreeWidget6itemAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QTreeWidgetItem * QTreeWidget::currentItem();
  fn _ZNK11QTreeWidget11currentItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::itemCollapsed(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget13itemCollapsedEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTreeWidgetItem * QTreeWidget::itemAt(const QPoint & p);
  fn _ZNK11QTreeWidget6itemAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QTreeWidget::itemClicked(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget11itemClickedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  QTreeWidgetItem * QTreeWidget::topLevelItem(int index);
  fn _ZNK11QTreeWidget12topLevelItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QTreeWidget::topLevelItemCount();
  fn _ZNK11QTreeWidget17topLevelItemCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QTreeWidgetItem * QTreeWidget::headerItem();
  fn _ZNK11QTreeWidget10headerItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::setFirstItemColumnSpanned(const QTreeWidgetItem * item, bool span);
  fn _ZN11QTreeWidget25setFirstItemColumnSpannedEPK15QTreeWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  void QTreeWidget::removeItemWidget(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget16removeItemWidgetEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  QTreeWidgetItem * QTreeWidget::itemAbove(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget9itemAboveEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::expandItem(const QTreeWidgetItem * item);
  fn _ZN11QTreeWidget10expandItemEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeWidget::itemSelectionChanged();
  fn _ZN11QTreeWidget20itemSelectionChangedEv(qthis: *mut c_void) ;
  // proto:  void QTreeWidget::setHeaderItem(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget13setHeaderItemEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeWidget::collapseItem(const QTreeWidgetItem * item);
  fn _ZN11QTreeWidget12collapseItemEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeWidget::itemChanged(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget11itemChangedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  QTreeWidgetItem * QTreeWidget::takeTopLevelItem(int index);
  fn _ZN11QTreeWidget16takeTopLevelItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QWidget * QTreeWidget::itemWidget(QTreeWidgetItem * item, int column);
  fn _ZNK11QTreeWidget10itemWidgetEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  void QTreeWidget::editItem(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget8editItemEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QTreeWidget::setItemExpanded(const QTreeWidgetItem * item, bool expand);
  fn _ZN11QTreeWidget15setItemExpandedEPK15QTreeWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  void QTreeWidget::addTopLevelItem(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget15addTopLevelItemEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeWidget::closePersistentEditor(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget21closePersistentEditorEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QTreeWidget::NewQTreeWidget(QWidget * parent);
  fn _ZN11QTreeWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeWidget::setSelectionModel(QItemSelectionModel * selectionModel);
  fn _ZN11QTreeWidget17setSelectionModelEP19QItemSelectionModel(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRect QTreeWidget::visualItemRect(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget14visualItemRectEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::setHeaderLabel(const QString & label);
  fn _ZN11QTreeWidget14setHeaderLabelERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeWidget::currentItemChanged(QTreeWidgetItem * current, QTreeWidgetItem * previous);
  fn _ZN11QTreeWidget18currentItemChangedEP15QTreeWidgetItemS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QTreeWidget::isItemHidden(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget12isItemHiddenEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QTreeWidget::openPersistentEditor(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget20openPersistentEditorEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  int QTreeWidget::columnCount();
  fn _ZNK11QTreeWidget11columnCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeWidget::setItemSelected(const QTreeWidgetItem * item, bool select);
  fn _ZN11QTreeWidget15setItemSelectedEPK15QTreeWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  void QTreeWidget::itemActivated(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget13itemActivatedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
}

// body block begin
// class sizeof(QTreeWidget)=1
pub struct QTreeWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTreeWidget {
  pub fn itemDoubleClicked<T: QTreeWidget_itemDoubleClicked>(&mut self, value: T)  {
     value.itemDoubleClicked(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemDoubleClicked {
  fn itemDoubleClicked(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::itemDoubleClicked(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemDoubleClicked for (&'a mut QTreeWidgetItem, i32) {
  fn itemDoubleClicked(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget17itemDoubleClickedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget17itemDoubleClickedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn itemExpanded<T: QTreeWidget_itemExpanded>(&mut self, value: T)  {
     value.itemExpanded(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemExpanded {
  fn itemExpanded(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::itemExpanded(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_itemExpanded for (&'a mut QTreeWidgetItem) {
  fn itemExpanded(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget12itemExpandedEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget12itemExpandedEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn setColumnCount<T: QTreeWidget_setColumnCount>(&mut self, value: T)  {
     value.setColumnCount(self);
    // return 1;
  }
}

pub trait QTreeWidget_setColumnCount {
  fn setColumnCount(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::setColumnCount(int columns);
impl<'a> /*trait*/ QTreeWidget_setColumnCount for (i32) {
  fn setColumnCount(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget14setColumnCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTreeWidget14setColumnCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn FreeQTreeWidget<T: QTreeWidget_FreeQTreeWidget>(&mut self, value: T)  {
     value.FreeQTreeWidget(self);
    // return 1;
  }
}

pub trait QTreeWidget_FreeQTreeWidget {
  fn FreeQTreeWidget(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::FreeQTreeWidget();
impl<'a> /*trait*/ QTreeWidget_FreeQTreeWidget for () {
  fn FreeQTreeWidget(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidgetD0Ev()};
     unsafe {_ZN11QTreeWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn selectedItems<T: QTreeWidget_selectedItems>(&mut self, value: T)  {
     value.selectedItems(self);
    // return 1;
  }
}

pub trait QTreeWidget_selectedItems {
  fn selectedItems(self, rsthis: &mut QTreeWidget) ;
}

// proto:  QList<QTreeWidgetItem *> QTreeWidget::selectedItems();
impl<'a> /*trait*/ QTreeWidget_selectedItems for () {
  fn selectedItems(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget13selectedItemsEv()};
     unsafe {_ZNK11QTreeWidget13selectedItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn isItemExpanded<T: QTreeWidget_isItemExpanded>(&mut self, value: T) -> i8 {
    return value.isItemExpanded(self);
    // return 1;
  }
}

pub trait QTreeWidget_isItemExpanded {
  fn isItemExpanded(self, rsthis: &mut QTreeWidget) -> i8;
}

// proto:  bool QTreeWidget::isItemExpanded(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_isItemExpanded for (&'a  QTreeWidgetItem) {
  fn isItemExpanded(self, rsthis: &mut QTreeWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget14isItemExpandedEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget14isItemExpandedEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn NewQTreeWidget<T: QTreeWidget_NewQTreeWidget>(value: T) -> QTreeWidget {
    let rsthis = value.NewQTreeWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidget_NewQTreeWidget {
  fn NewQTreeWidget(self) -> QTreeWidget;
}

// proto: void QTreeWidget::NewQTreeWidget(const QTreeWidget & );
impl<'a> /*trait*/ QTreeWidget_NewQTreeWidget for (&'a  QTreeWidget) {
  fn NewQTreeWidget(self) -> QTreeWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTreeWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QTreeWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn setItemHidden<T: QTreeWidget_setItemHidden>(&mut self, value: T)  {
     value.setItemHidden(self);
    // return 1;
  }
}

pub trait QTreeWidget_setItemHidden {
  fn setItemHidden(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::setItemHidden(const QTreeWidgetItem * item, bool hide);
impl<'a> /*trait*/ QTreeWidget_setItemHidden for (&'a  QTreeWidgetItem, i8) {
  fn setItemHidden(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13setItemHiddenEPK15QTreeWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN11QTreeWidget13setItemHiddenEPK15QTreeWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn indexOfTopLevelItem<T: QTreeWidget_indexOfTopLevelItem>(&mut self, value: T) -> i32 {
    return value.indexOfTopLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_indexOfTopLevelItem {
  fn indexOfTopLevelItem(self, rsthis: &mut QTreeWidget) -> i32;
}

// proto:  int QTreeWidget::indexOfTopLevelItem(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_indexOfTopLevelItem for (&'a mut QTreeWidgetItem) {
  fn indexOfTopLevelItem(self, rsthis: &mut QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget19indexOfTopLevelItemEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget19indexOfTopLevelItemEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn insertTopLevelItem<T: QTreeWidget_insertTopLevelItem>(&mut self, value: T)  {
     value.insertTopLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_insertTopLevelItem {
  fn insertTopLevelItem(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::insertTopLevelItem(int index, QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_insertTopLevelItem for (i32, &'a mut QTreeWidgetItem) {
  fn insertTopLevelItem(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget18insertTopLevelItemEiP15QTreeWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget18insertTopLevelItemEiP15QTreeWidgetItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn setItemWidget<T: QTreeWidget_setItemWidget>(&mut self, value: T)  {
     value.setItemWidget(self);
    // return 1;
  }
}

pub trait QTreeWidget_setItemWidget {
  fn setItemWidget(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::setItemWidget(QTreeWidgetItem * item, int column, QWidget * widget);
impl<'a> /*trait*/ QTreeWidget_setItemWidget for (&'a mut QTreeWidgetItem, i32, &'a mut QWidget) {
  fn setItemWidget(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13setItemWidgetEP15QTreeWidgetItemiP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget13setItemWidgetEP15QTreeWidgetItemiP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn isItemSelected<T: QTreeWidget_isItemSelected>(&mut self, value: T) -> i8 {
    return value.isItemSelected(self);
    // return 1;
  }
}

pub trait QTreeWidget_isItemSelected {
  fn isItemSelected(self, rsthis: &mut QTreeWidget) -> i8;
}

// proto:  bool QTreeWidget::isItemSelected(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_isItemSelected for (&'a  QTreeWidgetItem) {
  fn isItemSelected(self, rsthis: &mut QTreeWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget14isItemSelectedEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget14isItemSelectedEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn currentColumn<T: QTreeWidget_currentColumn>(&mut self, value: T) -> i32 {
    return value.currentColumn(self);
    // return 1;
  }
}

pub trait QTreeWidget_currentColumn {
  fn currentColumn(self, rsthis: &mut QTreeWidget) -> i32;
}

// proto:  int QTreeWidget::currentColumn();
impl<'a> /*trait*/ QTreeWidget_currentColumn for () {
  fn currentColumn(self, rsthis: &mut QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget13currentColumnEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget13currentColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn isFirstItemColumnSpanned<T: QTreeWidget_isFirstItemColumnSpanned>(&mut self, value: T) -> i8 {
    return value.isFirstItemColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeWidget_isFirstItemColumnSpanned {
  fn isFirstItemColumnSpanned(self, rsthis: &mut QTreeWidget) -> i8;
}

// proto:  bool QTreeWidget::isFirstItemColumnSpanned(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_isFirstItemColumnSpanned for (&'a  QTreeWidgetItem) {
  fn isFirstItemColumnSpanned(self, rsthis: &mut QTreeWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget24isFirstItemColumnSpannedEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget24isFirstItemColumnSpannedEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn clear<T: QTreeWidget_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QTreeWidget_clear {
  fn clear(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::clear();
impl<'a> /*trait*/ QTreeWidget_clear for () {
  fn clear(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget5clearEv()};
     unsafe {_ZN11QTreeWidget5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn itemPressed<T: QTreeWidget_itemPressed>(&mut self, value: T)  {
     value.itemPressed(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemPressed {
  fn itemPressed(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::itemPressed(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemPressed for (&'a mut QTreeWidgetItem, i32) {
  fn itemPressed(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget11itemPressedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget11itemPressedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn setHeaderLabels<T: QTreeWidget_setHeaderLabels>(&mut self, value: T)  {
     value.setHeaderLabels(self);
    // return 1;
  }
}

pub trait QTreeWidget_setHeaderLabels {
  fn setHeaderLabels(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::setHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QTreeWidget_setHeaderLabels for (&'a  QStringList) {
  fn setHeaderLabels(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget15setHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget15setHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn invisibleRootItem<T: QTreeWidget_invisibleRootItem>(&mut self, value: T) -> QTreeWidgetItem {
    return value.invisibleRootItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_invisibleRootItem {
  fn invisibleRootItem(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem;
}

// proto:  QTreeWidgetItem * QTreeWidget::invisibleRootItem();
impl<'a> /*trait*/ QTreeWidget_invisibleRootItem for () {
  fn invisibleRootItem(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget17invisibleRootItemEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget17invisibleRootItemEv(rsthis.qclsinst)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn metaObject<T: QTreeWidget_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QTreeWidget_metaObject {
  fn metaObject(self, rsthis: &mut QTreeWidget) ;
}

// proto:  const QMetaObject * QTreeWidget::metaObject();
impl<'a> /*trait*/ QTreeWidget_metaObject for () {
  fn metaObject(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget10metaObjectEv()};
     unsafe {_ZNK11QTreeWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn itemEntered<T: QTreeWidget_itemEntered>(&mut self, value: T)  {
     value.itemEntered(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemEntered {
  fn itemEntered(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::itemEntered(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemEntered for (&'a mut QTreeWidgetItem, i32) {
  fn itemEntered(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget11itemEnteredEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget11itemEnteredEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn itemBelow<T: QTreeWidget_itemBelow>(&mut self, value: T) -> QTreeWidgetItem {
    return value.itemBelow(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemBelow {
  fn itemBelow(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem;
}

// proto:  QTreeWidgetItem * QTreeWidget::itemBelow(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_itemBelow for (&'a  QTreeWidgetItem) {
  fn itemBelow(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget9itemBelowEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget9itemBelowEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn sortColumn<T: QTreeWidget_sortColumn>(&mut self, value: T) -> i32 {
    return value.sortColumn(self);
    // return 1;
  }
}

pub trait QTreeWidget_sortColumn {
  fn sortColumn(self, rsthis: &mut QTreeWidget) -> i32;
}

// proto:  int QTreeWidget::sortColumn();
impl<'a> /*trait*/ QTreeWidget_sortColumn for () {
  fn sortColumn(self, rsthis: &mut QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget10sortColumnEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget10sortColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn itemAt<T: QTreeWidget_itemAt>(&mut self, value: T) -> QTreeWidgetItem {
    return value.itemAt(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemAt {
  fn itemAt(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem;
}

// proto:  QTreeWidgetItem * QTreeWidget::itemAt(int x, int y);
impl<'a> /*trait*/ QTreeWidget_itemAt for (i32, i32) {
  fn itemAt(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QTreeWidget6itemAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn currentItem<T: QTreeWidget_currentItem>(&mut self, value: T) -> QTreeWidgetItem {
    return value.currentItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_currentItem {
  fn currentItem(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem;
}

// proto:  QTreeWidgetItem * QTreeWidget::currentItem();
impl<'a> /*trait*/ QTreeWidget_currentItem for () {
  fn currentItem(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget11currentItemEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget11currentItemEv(rsthis.qclsinst)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn itemCollapsed<T: QTreeWidget_itemCollapsed>(&mut self, value: T)  {
     value.itemCollapsed(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemCollapsed {
  fn itemCollapsed(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::itemCollapsed(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_itemCollapsed for (&'a mut QTreeWidgetItem) {
  fn itemCollapsed(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13itemCollapsedEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget13itemCollapsedEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QTreeWidgetItem * QTreeWidget::itemAt(const QPoint & p);
impl<'a> /*trait*/ QTreeWidget_itemAt for (&'a  QPoint) {
  fn itemAt(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget6itemAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget6itemAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn setCurrentItem<T: QTreeWidget_setCurrentItem>(&mut self, value: T)  {
     value.setCurrentItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_setCurrentItem {
  fn setCurrentItem(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_setCurrentItem for (&'a mut QTreeWidgetItem, i32) {
  fn setCurrentItem(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn itemClicked<T: QTreeWidget_itemClicked>(&mut self, value: T)  {
     value.itemClicked(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemClicked {
  fn itemClicked(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::itemClicked(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemClicked for (&'a mut QTreeWidgetItem, i32) {
  fn itemClicked(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget11itemClickedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget11itemClickedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn topLevelItem<T: QTreeWidget_topLevelItem>(&mut self, value: T) -> QTreeWidgetItem {
    return value.topLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_topLevelItem {
  fn topLevelItem(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem;
}

// proto:  QTreeWidgetItem * QTreeWidget::topLevelItem(int index);
impl<'a> /*trait*/ QTreeWidget_topLevelItem for (i32) {
  fn topLevelItem(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget12topLevelItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTreeWidget12topLevelItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn topLevelItemCount<T: QTreeWidget_topLevelItemCount>(&mut self, value: T) -> i32 {
    return value.topLevelItemCount(self);
    // return 1;
  }
}

pub trait QTreeWidget_topLevelItemCount {
  fn topLevelItemCount(self, rsthis: &mut QTreeWidget) -> i32;
}

// proto:  int QTreeWidget::topLevelItemCount();
impl<'a> /*trait*/ QTreeWidget_topLevelItemCount for () {
  fn topLevelItemCount(self, rsthis: &mut QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget17topLevelItemCountEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget17topLevelItemCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn headerItem<T: QTreeWidget_headerItem>(&mut self, value: T) -> QTreeWidgetItem {
    return value.headerItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_headerItem {
  fn headerItem(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem;
}

// proto:  QTreeWidgetItem * QTreeWidget::headerItem();
impl<'a> /*trait*/ QTreeWidget_headerItem for () {
  fn headerItem(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget10headerItemEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget10headerItemEv(rsthis.qclsinst)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn setFirstItemColumnSpanned<T: QTreeWidget_setFirstItemColumnSpanned>(&mut self, value: T)  {
     value.setFirstItemColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeWidget_setFirstItemColumnSpanned {
  fn setFirstItemColumnSpanned(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::setFirstItemColumnSpanned(const QTreeWidgetItem * item, bool span);
impl<'a> /*trait*/ QTreeWidget_setFirstItemColumnSpanned for (&'a  QTreeWidgetItem, i8) {
  fn setFirstItemColumnSpanned(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget25setFirstItemColumnSpannedEPK15QTreeWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN11QTreeWidget25setFirstItemColumnSpannedEPK15QTreeWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn removeItemWidget<T: QTreeWidget_removeItemWidget>(&mut self, value: T)  {
     value.removeItemWidget(self);
    // return 1;
  }
}

pub trait QTreeWidget_removeItemWidget {
  fn removeItemWidget(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::removeItemWidget(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_removeItemWidget for (&'a mut QTreeWidgetItem, i32) {
  fn removeItemWidget(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget16removeItemWidgetEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget16removeItemWidgetEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn itemAbove<T: QTreeWidget_itemAbove>(&mut self, value: T) -> QTreeWidgetItem {
    return value.itemAbove(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemAbove {
  fn itemAbove(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem;
}

// proto:  QTreeWidgetItem * QTreeWidget::itemAbove(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_itemAbove for (&'a  QTreeWidgetItem) {
  fn itemAbove(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget9itemAboveEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget9itemAboveEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn expandItem<T: QTreeWidget_expandItem>(&mut self, value: T)  {
     value.expandItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_expandItem {
  fn expandItem(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::expandItem(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_expandItem for (&'a  QTreeWidgetItem) {
  fn expandItem(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget10expandItemEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget10expandItemEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn itemSelectionChanged<T: QTreeWidget_itemSelectionChanged>(&mut self, value: T)  {
     value.itemSelectionChanged(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemSelectionChanged {
  fn itemSelectionChanged(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::itemSelectionChanged();
impl<'a> /*trait*/ QTreeWidget_itemSelectionChanged for () {
  fn itemSelectionChanged(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget20itemSelectionChangedEv()};
     unsafe {_ZN11QTreeWidget20itemSelectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn setHeaderItem<T: QTreeWidget_setHeaderItem>(&mut self, value: T)  {
     value.setHeaderItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_setHeaderItem {
  fn setHeaderItem(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::setHeaderItem(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_setHeaderItem for (&'a mut QTreeWidgetItem) {
  fn setHeaderItem(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13setHeaderItemEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget13setHeaderItemEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn collapseItem<T: QTreeWidget_collapseItem>(&mut self, value: T)  {
     value.collapseItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_collapseItem {
  fn collapseItem(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::collapseItem(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_collapseItem for (&'a  QTreeWidgetItem) {
  fn collapseItem(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget12collapseItemEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget12collapseItemEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn itemChanged<T: QTreeWidget_itemChanged>(&mut self, value: T)  {
     value.itemChanged(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemChanged {
  fn itemChanged(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::itemChanged(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemChanged for (&'a mut QTreeWidgetItem, i32) {
  fn itemChanged(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget11itemChangedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget11itemChangedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn takeTopLevelItem<T: QTreeWidget_takeTopLevelItem>(&mut self, value: T) -> QTreeWidgetItem {
    return value.takeTopLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_takeTopLevelItem {
  fn takeTopLevelItem(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem;
}

// proto:  QTreeWidgetItem * QTreeWidget::takeTopLevelItem(int index);
impl<'a> /*trait*/ QTreeWidget_takeTopLevelItem for (i32) {
  fn takeTopLevelItem(self, rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget16takeTopLevelItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN11QTreeWidget16takeTopLevelItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn itemWidget<T: QTreeWidget_itemWidget>(&mut self, value: T) -> QWidget {
    return value.itemWidget(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemWidget {
  fn itemWidget(self, rsthis: &mut QTreeWidget) -> QWidget;
}

// proto:  QWidget * QTreeWidget::itemWidget(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemWidget for (&'a mut QTreeWidgetItem, i32) {
  fn itemWidget(self, rsthis: &mut QTreeWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget10itemWidgetEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QTreeWidget10itemWidgetEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn editItem<T: QTreeWidget_editItem>(&mut self, value: T)  {
     value.editItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_editItem {
  fn editItem(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::editItem(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_editItem for (&'a mut QTreeWidgetItem, i32) {
  fn editItem(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget8editItemEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget8editItemEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn setItemExpanded<T: QTreeWidget_setItemExpanded>(&mut self, value: T)  {
     value.setItemExpanded(self);
    // return 1;
  }
}

pub trait QTreeWidget_setItemExpanded {
  fn setItemExpanded(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::setItemExpanded(const QTreeWidgetItem * item, bool expand);
impl<'a> /*trait*/ QTreeWidget_setItemExpanded for (&'a  QTreeWidgetItem, i8) {
  fn setItemExpanded(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget15setItemExpandedEPK15QTreeWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN11QTreeWidget15setItemExpandedEPK15QTreeWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn addTopLevelItem<T: QTreeWidget_addTopLevelItem>(&mut self, value: T)  {
     value.addTopLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_addTopLevelItem {
  fn addTopLevelItem(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::addTopLevelItem(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_addTopLevelItem for (&'a mut QTreeWidgetItem) {
  fn addTopLevelItem(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget15addTopLevelItemEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget15addTopLevelItemEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn closePersistentEditor<T: QTreeWidget_closePersistentEditor>(&mut self, value: T)  {
     value.closePersistentEditor(self);
    // return 1;
  }
}

pub trait QTreeWidget_closePersistentEditor {
  fn closePersistentEditor(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::closePersistentEditor(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_closePersistentEditor for (&'a mut QTreeWidgetItem, i32) {
  fn closePersistentEditor(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget21closePersistentEditorEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget21closePersistentEditorEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto: void QTreeWidget::NewQTreeWidget(QWidget * parent);
impl<'a> /*trait*/ QTreeWidget_NewQTreeWidget for (&'a mut QWidget) {
  fn NewQTreeWidget(self) -> QTreeWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidgetC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTreeWidgetC1EP7QWidget(qthis, arg0)};
    let rsthis = QTreeWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn setSelectionModel<T: QTreeWidget_setSelectionModel>(&mut self, value: T)  {
     value.setSelectionModel(self);
    // return 1;
  }
}

pub trait QTreeWidget_setSelectionModel {
  fn setSelectionModel(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::setSelectionModel(QItemSelectionModel * selectionModel);
impl<'a> /*trait*/ QTreeWidget_setSelectionModel for (&'a mut QItemSelectionModel) {
  fn setSelectionModel(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget17setSelectionModelEP19QItemSelectionModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget17setSelectionModelEP19QItemSelectionModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn visualItemRect<T: QTreeWidget_visualItemRect>(&mut self, value: T) -> QRect {
    return value.visualItemRect(self);
    // return 1;
  }
}

pub trait QTreeWidget_visualItemRect {
  fn visualItemRect(self, rsthis: &mut QTreeWidget) -> QRect;
}

// proto:  QRect QTreeWidget::visualItemRect(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_visualItemRect for (&'a  QTreeWidgetItem) {
  fn visualItemRect(self, rsthis: &mut QTreeWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget14visualItemRectEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget14visualItemRectEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn setHeaderLabel<T: QTreeWidget_setHeaderLabel>(&mut self, value: T)  {
     value.setHeaderLabel(self);
    // return 1;
  }
}

pub trait QTreeWidget_setHeaderLabel {
  fn setHeaderLabel(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::setHeaderLabel(const QString & label);
impl<'a> /*trait*/ QTreeWidget_setHeaderLabel for (&'a  QString) {
  fn setHeaderLabel(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget14setHeaderLabelERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget14setHeaderLabelERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn currentItemChanged<T: QTreeWidget_currentItemChanged>(&mut self, value: T)  {
     value.currentItemChanged(self);
    // return 1;
  }
}

pub trait QTreeWidget_currentItemChanged {
  fn currentItemChanged(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::currentItemChanged(QTreeWidgetItem * current, QTreeWidgetItem * previous);
impl<'a> /*trait*/ QTreeWidget_currentItemChanged for (&'a mut QTreeWidgetItem, &'a mut QTreeWidgetItem) {
  fn currentItemChanged(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget18currentItemChangedEP15QTreeWidgetItemS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget18currentItemChangedEP15QTreeWidgetItemS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn isItemHidden<T: QTreeWidget_isItemHidden>(&mut self, value: T) -> i8 {
    return value.isItemHidden(self);
    // return 1;
  }
}

pub trait QTreeWidget_isItemHidden {
  fn isItemHidden(self, rsthis: &mut QTreeWidget) -> i8;
}

// proto:  bool QTreeWidget::isItemHidden(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_isItemHidden for (&'a  QTreeWidgetItem) {
  fn isItemHidden(self, rsthis: &mut QTreeWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget12isItemHiddenEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget12isItemHiddenEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn openPersistentEditor<T: QTreeWidget_openPersistentEditor>(&mut self, value: T)  {
     value.openPersistentEditor(self);
    // return 1;
  }
}

pub trait QTreeWidget_openPersistentEditor {
  fn openPersistentEditor(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::openPersistentEditor(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_openPersistentEditor for (&'a mut QTreeWidgetItem, i32) {
  fn openPersistentEditor(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget20openPersistentEditorEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget20openPersistentEditorEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn columnCount<T: QTreeWidget_columnCount>(&mut self, value: T) -> i32 {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QTreeWidget_columnCount {
  fn columnCount(self, rsthis: &mut QTreeWidget) -> i32;
}

// proto:  int QTreeWidget::columnCount();
impl<'a> /*trait*/ QTreeWidget_columnCount for () {
  fn columnCount(self, rsthis: &mut QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget11columnCountEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_setCurrentItem for (&'a mut QTreeWidgetItem) {
  fn setCurrentItem(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn setItemSelected<T: QTreeWidget_setItemSelected>(&mut self, value: T)  {
     value.setItemSelected(self);
    // return 1;
  }
}

pub trait QTreeWidget_setItemSelected {
  fn setItemSelected(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::setItemSelected(const QTreeWidgetItem * item, bool select);
impl<'a> /*trait*/ QTreeWidget_setItemSelected for (&'a  QTreeWidgetItem, i8) {
  fn setItemSelected(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget15setItemSelectedEPK15QTreeWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN11QTreeWidget15setItemSelectedEPK15QTreeWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidget {
  pub fn itemActivated<T: QTreeWidget_itemActivated>(&mut self, value: T)  {
     value.itemActivated(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemActivated {
  fn itemActivated(self, rsthis: &mut QTreeWidget) ;
}

// proto:  void QTreeWidget::itemActivated(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemActivated for (&'a mut QTreeWidgetItem, i32) {
  fn itemActivated(self, rsthis: &mut QTreeWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13itemActivatedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget13itemActivatedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

