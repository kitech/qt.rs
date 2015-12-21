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
use super::qstring::QString;
use super::qstringlist::QStringList;
use super::qpoint::QPoint;
use super::qitemselectionmodel::QItemSelectionModel;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTreeWidget::itemDoubleClicked(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget17itemDoubleClickedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QTreeWidget::itemExpanded(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget12itemExpandedEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::setColumnCount(int columns);
  fn _ZN11QTreeWidget14setColumnCountEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTreeWidget::~QTreeWidget();
  fn _ZN11QTreeWidgetD0Ev(qthis: *mut c_void);
  // proto:  QList<QTreeWidgetItem *> QTreeWidget::selectedItems();
  fn _ZNK11QTreeWidget13selectedItemsEv(qthis: *mut c_void);
  // proto:  bool QTreeWidget::isItemExpanded(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget14isItemExpandedEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QTreeWidget::QTreeWidget(const QTreeWidget & );
  fn _ZN11QTreeWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::setItemHidden(const QTreeWidgetItem * item, bool hide);
  fn _ZN11QTreeWidget13setItemHiddenEPK15QTreeWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  int QTreeWidget::indexOfTopLevelItem(QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget19indexOfTopLevelItemEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QTreeWidget::insertTopLevelItem(int index, QTreeWidgetItem * item);
  fn _ZN11QTreeWidget18insertTopLevelItemEiP15QTreeWidgetItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTreeWidget::setItemWidget(QTreeWidgetItem * item, int column, QWidget * widget);
  fn _ZN11QTreeWidget13setItemWidgetEP15QTreeWidgetItemiP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  bool QTreeWidget::isItemSelected(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget14isItemSelectedEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  int QTreeWidget::currentColumn();
  fn _ZNK11QTreeWidget13currentColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTreeWidget::isFirstItemColumnSpanned(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget24isFirstItemColumnSpannedEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QTreeWidget::clear();
  fn _ZN11QTreeWidget5clearEv(qthis: *mut c_void);
  // proto:  void QTreeWidget::itemPressed(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget11itemPressedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QTreeWidget::setHeaderLabels(const QStringList & labels);
  fn _ZN11QTreeWidget15setHeaderLabelsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTreeWidgetItem * QTreeWidget::invisibleRootItem();
  fn _ZNK11QTreeWidget17invisibleRootItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QTreeWidget::metaObject();
  fn _ZNK11QTreeWidget10metaObjectEv(qthis: *mut c_void);
  // proto:  void QTreeWidget::itemEntered(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget11itemEnteredEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QTreeWidgetItem * QTreeWidget::itemBelow(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget9itemBelowEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QTreeWidget::sortColumn();
  fn _ZNK11QTreeWidget10sortColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  QTreeWidgetItem * QTreeWidget::itemAt(int x, int y);
  fn _ZNK11QTreeWidget6itemAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QTreeWidgetItem * QTreeWidget::currentItem();
  fn _ZNK11QTreeWidget11currentItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::itemCollapsed(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget13itemCollapsedEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTreeWidgetItem * QTreeWidget::itemAt(const QPoint & p);
  fn _ZNK11QTreeWidget6itemAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QTreeWidget::itemClicked(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget11itemClickedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QTreeWidgetItem * QTreeWidget::topLevelItem(int index);
  fn _ZNK11QTreeWidget12topLevelItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QTreeWidget::topLevelItemCount();
  fn _ZNK11QTreeWidget17topLevelItemCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QTreeWidgetItem * QTreeWidget::headerItem();
  fn _ZNK11QTreeWidget10headerItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::setFirstItemColumnSpanned(const QTreeWidgetItem * item, bool span);
  fn _ZN11QTreeWidget25setFirstItemColumnSpannedEPK15QTreeWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QTreeWidget::removeItemWidget(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget16removeItemWidgetEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QTreeWidgetItem * QTreeWidget::itemAbove(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget9itemAboveEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::expandItem(const QTreeWidgetItem * item);
  fn _ZN11QTreeWidget10expandItemEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::itemSelectionChanged();
  fn _ZN11QTreeWidget20itemSelectionChangedEv(qthis: *mut c_void);
  // proto:  void QTreeWidget::setHeaderItem(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget13setHeaderItemEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::collapseItem(const QTreeWidgetItem * item);
  fn _ZN11QTreeWidget12collapseItemEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::itemChanged(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget11itemChangedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QTreeWidgetItem * QTreeWidget::takeTopLevelItem(int index);
  fn _ZN11QTreeWidget16takeTopLevelItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QWidget * QTreeWidget::itemWidget(QTreeWidgetItem * item, int column);
  fn _ZNK11QTreeWidget10itemWidgetEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  void QTreeWidget::editItem(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget8editItemEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QTreeWidget::setItemExpanded(const QTreeWidgetItem * item, bool expand);
  fn _ZN11QTreeWidget15setItemExpandedEPK15QTreeWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QTreeWidget::addTopLevelItem(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget15addTopLevelItemEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::closePersistentEditor(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget21closePersistentEditorEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QTreeWidget::QTreeWidget(QWidget * parent);
  fn _ZN11QTreeWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::setSelectionModel(QItemSelectionModel * selectionModel);
  fn _ZN11QTreeWidget17setSelectionModelEP19QItemSelectionModel(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRect QTreeWidget::visualItemRect(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget14visualItemRectEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::setHeaderLabel(const QString & label);
  fn _ZN11QTreeWidget14setHeaderLabelERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::currentItemChanged(QTreeWidgetItem * current, QTreeWidgetItem * previous);
  fn _ZN11QTreeWidget18currentItemChangedEP15QTreeWidgetItemS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QTreeWidget::isItemHidden(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget12isItemHiddenEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QTreeWidget::openPersistentEditor(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget20openPersistentEditorEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  int QTreeWidget::columnCount();
  fn _ZNK11QTreeWidget11columnCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::setItemSelected(const QTreeWidgetItem * item, bool select);
  fn _ZN11QTreeWidget15setItemSelectedEPK15QTreeWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QTreeWidget::itemActivated(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget13itemActivatedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
}

// body block begin
// class sizeof(QTreeWidget)=1
pub struct QTreeWidget {
  pub qclsinst: *mut c_void,
}

  // proto:  void QTreeWidget::itemDoubleClicked(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemDoubleClicked<RetType, T: QTreeWidget_itemDoubleClicked<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemDoubleClicked(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemDoubleClicked<RetType> {
  fn itemDoubleClicked(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemDoubleClicked(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemDoubleClicked<()> for (QTreeWidgetItem, i32) {
  fn itemDoubleClicked(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget17itemDoubleClickedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget17itemDoubleClickedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemExpanded(QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn itemExpanded<RetType, T: QTreeWidget_itemExpanded<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemExpanded(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemExpanded<RetType> {
  fn itemExpanded(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemExpanded(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_itemExpanded<()> for (QTreeWidgetItem) {
  fn itemExpanded(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget12itemExpandedEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget12itemExpandedEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::setColumnCount(int columns);
impl /*struct*/ QTreeWidget {
  pub fn setColumnCount<RetType, T: QTreeWidget_setColumnCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColumnCount(self);
    // return 1;
  }
}

pub trait QTreeWidget_setColumnCount<RetType> {
  fn setColumnCount(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setColumnCount(int columns);
impl<'a> /*trait*/ QTreeWidget_setColumnCount<()> for (i32) {
  fn setColumnCount(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget14setColumnCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTreeWidget14setColumnCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::~QTreeWidget();
impl /*struct*/ QTreeWidget {
  pub fn FreeQTreeWidget<RetType, T: QTreeWidget_FreeQTreeWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTreeWidget(self);
    // return 1;
  }
}

pub trait QTreeWidget_FreeQTreeWidget<RetType> {
  fn FreeQTreeWidget(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::~QTreeWidget();
impl<'a> /*trait*/ QTreeWidget_FreeQTreeWidget<()> for () {
  fn FreeQTreeWidget(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidgetD0Ev()};
     unsafe {_ZN11QTreeWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QTreeWidgetItem *> QTreeWidget::selectedItems();
impl /*struct*/ QTreeWidget {
  pub fn selectedItems<RetType, T: QTreeWidget_selectedItems<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedItems(self);
    // return 1;
  }
}

pub trait QTreeWidget_selectedItems<RetType> {
  fn selectedItems(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  QList<QTreeWidgetItem *> QTreeWidget::selectedItems();
impl<'a> /*trait*/ QTreeWidget_selectedItems<()> for () {
  fn selectedItems(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget13selectedItemsEv()};
     unsafe {_ZNK11QTreeWidget13selectedItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTreeWidget::isItemExpanded(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn isItemExpanded<RetType, T: QTreeWidget_isItemExpanded<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isItemExpanded(self);
    // return 1;
  }
}

pub trait QTreeWidget_isItemExpanded<RetType> {
  fn isItemExpanded(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  bool QTreeWidget::isItemExpanded(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_isItemExpanded<i8> for (QTreeWidgetItem) {
  fn isItemExpanded(self , rsthis: &mut QTreeWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget14isItemExpandedEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget14isItemExpandedEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeWidget::QTreeWidget(const QTreeWidget & );
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

  // proto:  void QTreeWidget::QTreeWidget(const QTreeWidget & );
impl<'a> /*trait*/ QTreeWidget_NewQTreeWidget for (QTreeWidget) {
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

  // proto:  void QTreeWidget::setItemHidden(const QTreeWidgetItem * item, bool hide);
impl /*struct*/ QTreeWidget {
  pub fn setItemHidden<RetType, T: QTreeWidget_setItemHidden<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setItemHidden(self);
    // return 1;
  }
}

pub trait QTreeWidget_setItemHidden<RetType> {
  fn setItemHidden(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setItemHidden(const QTreeWidgetItem * item, bool hide);
impl<'a> /*trait*/ QTreeWidget_setItemHidden<()> for (QTreeWidgetItem, i8) {
  fn setItemHidden(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13setItemHiddenEPK15QTreeWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN11QTreeWidget13setItemHiddenEPK15QTreeWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QTreeWidget::indexOfTopLevelItem(QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn indexOfTopLevelItem<RetType, T: QTreeWidget_indexOfTopLevelItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indexOfTopLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_indexOfTopLevelItem<RetType> {
  fn indexOfTopLevelItem(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  int QTreeWidget::indexOfTopLevelItem(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_indexOfTopLevelItem<i32> for (QTreeWidgetItem) {
  fn indexOfTopLevelItem(self , rsthis: &mut QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget19indexOfTopLevelItemEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget19indexOfTopLevelItemEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTreeWidget::insertTopLevelItem(int index, QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn insertTopLevelItem<RetType, T: QTreeWidget_insertTopLevelItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertTopLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_insertTopLevelItem<RetType> {
  fn insertTopLevelItem(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::insertTopLevelItem(int index, QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_insertTopLevelItem<()> for (i32, QTreeWidgetItem) {
  fn insertTopLevelItem(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget18insertTopLevelItemEiP15QTreeWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget18insertTopLevelItemEiP15QTreeWidgetItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::setItemWidget(QTreeWidgetItem * item, int column, QWidget * widget);
impl /*struct*/ QTreeWidget {
  pub fn setItemWidget<RetType, T: QTreeWidget_setItemWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setItemWidget(self);
    // return 1;
  }
}

pub trait QTreeWidget_setItemWidget<RetType> {
  fn setItemWidget(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setItemWidget(QTreeWidgetItem * item, int column, QWidget * widget);
impl<'a> /*trait*/ QTreeWidget_setItemWidget<()> for (QTreeWidgetItem, i32, QWidget) {
  fn setItemWidget(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13setItemWidgetEP15QTreeWidgetItemiP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget13setItemWidgetEP15QTreeWidgetItemiP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QTreeWidget::isItemSelected(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn isItemSelected<RetType, T: QTreeWidget_isItemSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isItemSelected(self);
    // return 1;
  }
}

pub trait QTreeWidget_isItemSelected<RetType> {
  fn isItemSelected(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  bool QTreeWidget::isItemSelected(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_isItemSelected<i8> for (QTreeWidgetItem) {
  fn isItemSelected(self , rsthis: &mut QTreeWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget14isItemSelectedEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget14isItemSelectedEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTreeWidget::currentColumn();
impl /*struct*/ QTreeWidget {
  pub fn currentColumn<RetType, T: QTreeWidget_currentColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentColumn(self);
    // return 1;
  }
}

pub trait QTreeWidget_currentColumn<RetType> {
  fn currentColumn(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  int QTreeWidget::currentColumn();
impl<'a> /*trait*/ QTreeWidget_currentColumn<i32> for () {
  fn currentColumn(self , rsthis: &mut QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget13currentColumnEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget13currentColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTreeWidget::isFirstItemColumnSpanned(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn isFirstItemColumnSpanned<RetType, T: QTreeWidget_isFirstItemColumnSpanned<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isFirstItemColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeWidget_isFirstItemColumnSpanned<RetType> {
  fn isFirstItemColumnSpanned(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  bool QTreeWidget::isFirstItemColumnSpanned(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_isFirstItemColumnSpanned<i8> for (QTreeWidgetItem) {
  fn isFirstItemColumnSpanned(self , rsthis: &mut QTreeWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget24isFirstItemColumnSpannedEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget24isFirstItemColumnSpannedEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeWidget::clear();
impl /*struct*/ QTreeWidget {
  pub fn clear<RetType, T: QTreeWidget_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QTreeWidget_clear<RetType> {
  fn clear(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::clear();
impl<'a> /*trait*/ QTreeWidget_clear<()> for () {
  fn clear(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget5clearEv()};
     unsafe {_ZN11QTreeWidget5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemPressed(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemPressed<RetType, T: QTreeWidget_itemPressed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemPressed(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemPressed<RetType> {
  fn itemPressed(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemPressed(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemPressed<()> for (QTreeWidgetItem, i32) {
  fn itemPressed(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget11itemPressedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget11itemPressedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::setHeaderLabels(const QStringList & labels);
impl /*struct*/ QTreeWidget {
  pub fn setHeaderLabels<RetType, T: QTreeWidget_setHeaderLabels<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeaderLabels(self);
    // return 1;
  }
}

pub trait QTreeWidget_setHeaderLabels<RetType> {
  fn setHeaderLabels(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QTreeWidget_setHeaderLabels<()> for (QStringList) {
  fn setHeaderLabels(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget15setHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget15setHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::invisibleRootItem();
impl /*struct*/ QTreeWidget {
  pub fn invisibleRootItem<RetType, T: QTreeWidget_invisibleRootItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.invisibleRootItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_invisibleRootItem<RetType> {
  fn invisibleRootItem(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::invisibleRootItem();
impl<'a> /*trait*/ QTreeWidget_invisibleRootItem<QTreeWidgetItem> for () {
  fn invisibleRootItem(self , rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget17invisibleRootItemEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget17invisibleRootItemEv(rsthis.qclsinst)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTreeWidget::metaObject();
impl /*struct*/ QTreeWidget {
  pub fn metaObject<RetType, T: QTreeWidget_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTreeWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  const QMetaObject * QTreeWidget::metaObject();
impl<'a> /*trait*/ QTreeWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget10metaObjectEv()};
     unsafe {_ZNK11QTreeWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemEntered(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemEntered<RetType, T: QTreeWidget_itemEntered<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemEntered(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemEntered<RetType> {
  fn itemEntered(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemEntered(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemEntered<()> for (QTreeWidgetItem, i32) {
  fn itemEntered(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget11itemEnteredEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget11itemEnteredEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemBelow(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn itemBelow<RetType, T: QTreeWidget_itemBelow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemBelow(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemBelow<RetType> {
  fn itemBelow(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemBelow(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_itemBelow<QTreeWidgetItem> for (QTreeWidgetItem) {
  fn itemBelow(self , rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget9itemBelowEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget9itemBelowEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QTreeWidget::sortColumn();
impl /*struct*/ QTreeWidget {
  pub fn sortColumn<RetType, T: QTreeWidget_sortColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sortColumn(self);
    // return 1;
  }
}

pub trait QTreeWidget_sortColumn<RetType> {
  fn sortColumn(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  int QTreeWidget::sortColumn();
impl<'a> /*trait*/ QTreeWidget_sortColumn<i32> for () {
  fn sortColumn(self , rsthis: &mut QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget10sortColumnEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget10sortColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemAt(int x, int y);
impl /*struct*/ QTreeWidget {
  pub fn itemAt<RetType, T: QTreeWidget_itemAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemAt<RetType> {
  fn itemAt(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemAt(int x, int y);
impl<'a> /*trait*/ QTreeWidget_itemAt<QTreeWidgetItem> for (i32, i32) {
  fn itemAt(self , rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
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

  // proto:  QTreeWidgetItem * QTreeWidget::currentItem();
impl /*struct*/ QTreeWidget {
  pub fn currentItem<RetType, T: QTreeWidget_currentItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_currentItem<RetType> {
  fn currentItem(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::currentItem();
impl<'a> /*trait*/ QTreeWidget_currentItem<QTreeWidgetItem> for () {
  fn currentItem(self , rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget11currentItemEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget11currentItemEv(rsthis.qclsinst)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemCollapsed(QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn itemCollapsed<RetType, T: QTreeWidget_itemCollapsed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemCollapsed(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemCollapsed<RetType> {
  fn itemCollapsed(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemCollapsed(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_itemCollapsed<()> for (QTreeWidgetItem) {
  fn itemCollapsed(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13itemCollapsedEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget13itemCollapsedEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemAt(const QPoint & p);
impl<'a> /*trait*/ QTreeWidget_itemAt<QTreeWidgetItem> for (QPoint) {
  fn itemAt(self , rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget6itemAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget6itemAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn setCurrentItem<RetType, T: QTreeWidget_setCurrentItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_setCurrentItem<RetType> {
  fn setCurrentItem(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_setCurrentItem<()> for (QTreeWidgetItem, i32) {
  fn setCurrentItem(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemClicked(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemClicked<RetType, T: QTreeWidget_itemClicked<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemClicked(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemClicked<RetType> {
  fn itemClicked(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemClicked(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemClicked<()> for (QTreeWidgetItem, i32) {
  fn itemClicked(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget11itemClickedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget11itemClickedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::topLevelItem(int index);
impl /*struct*/ QTreeWidget {
  pub fn topLevelItem<RetType, T: QTreeWidget_topLevelItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_topLevelItem<RetType> {
  fn topLevelItem(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::topLevelItem(int index);
impl<'a> /*trait*/ QTreeWidget_topLevelItem<QTreeWidgetItem> for (i32) {
  fn topLevelItem(self , rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget12topLevelItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTreeWidget12topLevelItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QTreeWidget::topLevelItemCount();
impl /*struct*/ QTreeWidget {
  pub fn topLevelItemCount<RetType, T: QTreeWidget_topLevelItemCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topLevelItemCount(self);
    // return 1;
  }
}

pub trait QTreeWidget_topLevelItemCount<RetType> {
  fn topLevelItemCount(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  int QTreeWidget::topLevelItemCount();
impl<'a> /*trait*/ QTreeWidget_topLevelItemCount<i32> for () {
  fn topLevelItemCount(self , rsthis: &mut QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget17topLevelItemCountEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget17topLevelItemCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::headerItem();
impl /*struct*/ QTreeWidget {
  pub fn headerItem<RetType, T: QTreeWidget_headerItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.headerItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_headerItem<RetType> {
  fn headerItem(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::headerItem();
impl<'a> /*trait*/ QTreeWidget_headerItem<QTreeWidgetItem> for () {
  fn headerItem(self , rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget10headerItemEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget10headerItemEv(rsthis.qclsinst)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidget::setFirstItemColumnSpanned(const QTreeWidgetItem * item, bool span);
impl /*struct*/ QTreeWidget {
  pub fn setFirstItemColumnSpanned<RetType, T: QTreeWidget_setFirstItemColumnSpanned<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFirstItemColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeWidget_setFirstItemColumnSpanned<RetType> {
  fn setFirstItemColumnSpanned(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setFirstItemColumnSpanned(const QTreeWidgetItem * item, bool span);
impl<'a> /*trait*/ QTreeWidget_setFirstItemColumnSpanned<()> for (QTreeWidgetItem, i8) {
  fn setFirstItemColumnSpanned(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget25setFirstItemColumnSpannedEPK15QTreeWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN11QTreeWidget25setFirstItemColumnSpannedEPK15QTreeWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::removeItemWidget(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn removeItemWidget<RetType, T: QTreeWidget_removeItemWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeItemWidget(self);
    // return 1;
  }
}

pub trait QTreeWidget_removeItemWidget<RetType> {
  fn removeItemWidget(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::removeItemWidget(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_removeItemWidget<()> for (QTreeWidgetItem, i32) {
  fn removeItemWidget(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget16removeItemWidgetEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget16removeItemWidgetEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemAbove(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn itemAbove<RetType, T: QTreeWidget_itemAbove<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemAbove(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemAbove<RetType> {
  fn itemAbove(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemAbove(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_itemAbove<QTreeWidgetItem> for (QTreeWidgetItem) {
  fn itemAbove(self , rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget9itemAboveEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget9itemAboveEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidget::expandItem(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn expandItem<RetType, T: QTreeWidget_expandItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.expandItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_expandItem<RetType> {
  fn expandItem(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::expandItem(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_expandItem<()> for (QTreeWidgetItem) {
  fn expandItem(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget10expandItemEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget10expandItemEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemSelectionChanged();
impl /*struct*/ QTreeWidget {
  pub fn itemSelectionChanged<RetType, T: QTreeWidget_itemSelectionChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemSelectionChanged(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemSelectionChanged<RetType> {
  fn itemSelectionChanged(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemSelectionChanged();
impl<'a> /*trait*/ QTreeWidget_itemSelectionChanged<()> for () {
  fn itemSelectionChanged(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget20itemSelectionChangedEv()};
     unsafe {_ZN11QTreeWidget20itemSelectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::setHeaderItem(QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn setHeaderItem<RetType, T: QTreeWidget_setHeaderItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeaderItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_setHeaderItem<RetType> {
  fn setHeaderItem(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setHeaderItem(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_setHeaderItem<()> for (QTreeWidgetItem) {
  fn setHeaderItem(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13setHeaderItemEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget13setHeaderItemEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::collapseItem(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn collapseItem<RetType, T: QTreeWidget_collapseItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.collapseItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_collapseItem<RetType> {
  fn collapseItem(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::collapseItem(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_collapseItem<()> for (QTreeWidgetItem) {
  fn collapseItem(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget12collapseItemEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget12collapseItemEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemChanged(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemChanged<RetType, T: QTreeWidget_itemChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemChanged(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemChanged<RetType> {
  fn itemChanged(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemChanged(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemChanged<()> for (QTreeWidgetItem, i32) {
  fn itemChanged(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget11itemChangedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget11itemChangedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::takeTopLevelItem(int index);
impl /*struct*/ QTreeWidget {
  pub fn takeTopLevelItem<RetType, T: QTreeWidget_takeTopLevelItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.takeTopLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_takeTopLevelItem<RetType> {
  fn takeTopLevelItem(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::takeTopLevelItem(int index);
impl<'a> /*trait*/ QTreeWidget_takeTopLevelItem<QTreeWidgetItem> for (i32) {
  fn takeTopLevelItem(self , rsthis: &mut QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget16takeTopLevelItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN11QTreeWidget16takeTopLevelItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QTreeWidget::itemWidget(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemWidget<RetType, T: QTreeWidget_itemWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemWidget(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemWidget<RetType> {
  fn itemWidget(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  QWidget * QTreeWidget::itemWidget(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemWidget<QWidget> for (QTreeWidgetItem, i32) {
  fn itemWidget(self , rsthis: &mut QTreeWidget) -> QWidget {
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

  // proto:  void QTreeWidget::editItem(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn editItem<RetType, T: QTreeWidget_editItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.editItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_editItem<RetType> {
  fn editItem(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::editItem(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_editItem<()> for (QTreeWidgetItem, i32) {
  fn editItem(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget8editItemEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget8editItemEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::setItemExpanded(const QTreeWidgetItem * item, bool expand);
impl /*struct*/ QTreeWidget {
  pub fn setItemExpanded<RetType, T: QTreeWidget_setItemExpanded<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setItemExpanded(self);
    // return 1;
  }
}

pub trait QTreeWidget_setItemExpanded<RetType> {
  fn setItemExpanded(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setItemExpanded(const QTreeWidgetItem * item, bool expand);
impl<'a> /*trait*/ QTreeWidget_setItemExpanded<()> for (QTreeWidgetItem, i8) {
  fn setItemExpanded(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget15setItemExpandedEPK15QTreeWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN11QTreeWidget15setItemExpandedEPK15QTreeWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::addTopLevelItem(QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn addTopLevelItem<RetType, T: QTreeWidget_addTopLevelItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addTopLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_addTopLevelItem<RetType> {
  fn addTopLevelItem(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::addTopLevelItem(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_addTopLevelItem<()> for (QTreeWidgetItem) {
  fn addTopLevelItem(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget15addTopLevelItemEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget15addTopLevelItemEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::closePersistentEditor(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn closePersistentEditor<RetType, T: QTreeWidget_closePersistentEditor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.closePersistentEditor(self);
    // return 1;
  }
}

pub trait QTreeWidget_closePersistentEditor<RetType> {
  fn closePersistentEditor(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::closePersistentEditor(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_closePersistentEditor<()> for (QTreeWidgetItem, i32) {
  fn closePersistentEditor(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget21closePersistentEditorEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget21closePersistentEditorEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::QTreeWidget(QWidget * parent);
impl<'a> /*trait*/ QTreeWidget_NewQTreeWidget for (QWidget) {
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

  // proto:  void QTreeWidget::setSelectionModel(QItemSelectionModel * selectionModel);
impl /*struct*/ QTreeWidget {
  pub fn setSelectionModel<RetType, T: QTreeWidget_setSelectionModel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSelectionModel(self);
    // return 1;
  }
}

pub trait QTreeWidget_setSelectionModel<RetType> {
  fn setSelectionModel(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setSelectionModel(QItemSelectionModel * selectionModel);
impl<'a> /*trait*/ QTreeWidget_setSelectionModel<()> for (QItemSelectionModel) {
  fn setSelectionModel(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget17setSelectionModelEP19QItemSelectionModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget17setSelectionModelEP19QItemSelectionModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QTreeWidget::visualItemRect(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn visualItemRect<RetType, T: QTreeWidget_visualItemRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.visualItemRect(self);
    // return 1;
  }
}

pub trait QTreeWidget_visualItemRect<RetType> {
  fn visualItemRect(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  QRect QTreeWidget::visualItemRect(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_visualItemRect<QRect> for (QTreeWidgetItem) {
  fn visualItemRect(self , rsthis: &mut QTreeWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget14visualItemRectEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget14visualItemRectEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidget::setHeaderLabel(const QString & label);
impl /*struct*/ QTreeWidget {
  pub fn setHeaderLabel<RetType, T: QTreeWidget_setHeaderLabel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeaderLabel(self);
    // return 1;
  }
}

pub trait QTreeWidget_setHeaderLabel<RetType> {
  fn setHeaderLabel(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setHeaderLabel(const QString & label);
impl<'a> /*trait*/ QTreeWidget_setHeaderLabel<()> for (QString) {
  fn setHeaderLabel(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget14setHeaderLabelERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget14setHeaderLabelERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::currentItemChanged(QTreeWidgetItem * current, QTreeWidgetItem * previous);
impl /*struct*/ QTreeWidget {
  pub fn currentItemChanged<RetType, T: QTreeWidget_currentItemChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentItemChanged(self);
    // return 1;
  }
}

pub trait QTreeWidget_currentItemChanged<RetType> {
  fn currentItemChanged(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::currentItemChanged(QTreeWidgetItem * current, QTreeWidgetItem * previous);
impl<'a> /*trait*/ QTreeWidget_currentItemChanged<()> for (QTreeWidgetItem, QTreeWidgetItem) {
  fn currentItemChanged(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget18currentItemChangedEP15QTreeWidgetItemS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget18currentItemChangedEP15QTreeWidgetItemS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QTreeWidget::isItemHidden(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn isItemHidden<RetType, T: QTreeWidget_isItemHidden<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isItemHidden(self);
    // return 1;
  }
}

pub trait QTreeWidget_isItemHidden<RetType> {
  fn isItemHidden(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  bool QTreeWidget::isItemHidden(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_isItemHidden<i8> for (QTreeWidgetItem) {
  fn isItemHidden(self , rsthis: &mut QTreeWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget12isItemHiddenEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget12isItemHiddenEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeWidget::openPersistentEditor(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn openPersistentEditor<RetType, T: QTreeWidget_openPersistentEditor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.openPersistentEditor(self);
    // return 1;
  }
}

pub trait QTreeWidget_openPersistentEditor<RetType> {
  fn openPersistentEditor(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::openPersistentEditor(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_openPersistentEditor<()> for (QTreeWidgetItem, i32) {
  fn openPersistentEditor(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget20openPersistentEditorEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget20openPersistentEditorEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QTreeWidget::columnCount();
impl /*struct*/ QTreeWidget {
  pub fn columnCount<RetType, T: QTreeWidget_columnCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QTreeWidget_columnCount<RetType> {
  fn columnCount(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  int QTreeWidget::columnCount();
impl<'a> /*trait*/ QTreeWidget_columnCount<i32> for () {
  fn columnCount(self , rsthis: &mut QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget11columnCountEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_setCurrentItem<()> for (QTreeWidgetItem) {
  fn setCurrentItem(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::setItemSelected(const QTreeWidgetItem * item, bool select);
impl /*struct*/ QTreeWidget {
  pub fn setItemSelected<RetType, T: QTreeWidget_setItemSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setItemSelected(self);
    // return 1;
  }
}

pub trait QTreeWidget_setItemSelected<RetType> {
  fn setItemSelected(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setItemSelected(const QTreeWidgetItem * item, bool select);
impl<'a> /*trait*/ QTreeWidget_setItemSelected<()> for (QTreeWidgetItem, i8) {
  fn setItemSelected(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget15setItemSelectedEPK15QTreeWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN11QTreeWidget15setItemSelectedEPK15QTreeWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemActivated(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemActivated<RetType, T: QTreeWidget_itemActivated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemActivated(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemActivated<RetType> {
  fn itemActivated(self , rsthis: &mut QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemActivated(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemActivated<()> for (QTreeWidgetItem, i32) {
  fn itemActivated(self , rsthis: &mut QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13itemActivatedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget13itemActivatedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

