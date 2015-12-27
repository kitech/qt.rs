// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
// src-file: /QtWidgets/qabstractitemview.h
// dst-file: /src/widgets/qabstractitemview.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qabstractscrollarea::QAbstractScrollArea; // 773
use std::ops::Deref;
use super::super::core::qabstractitemmodel::QModelIndex; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qsize::QSize; // 771
use super::qabstractitemdelegate::QAbstractItemDelegate; // 773
use super::super::core::qstring::QString; // 771
use super::super::core::qitemselectionmodel::QItemSelectionModel; // 771
use super::super::core::qrect::QRect; // 771
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qabstractitemmodel::QAbstractItemModel; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractItemView_Class_Size() -> c_int;
  // proto:  QWidget * QAbstractItemView::indexWidget(const QModelIndex & index);
  fn _ZNK17QAbstractItemView11indexWidgetERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractItemView::scrollToBottom();
  fn _ZN17QAbstractItemView14scrollToBottomEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractItemView::setDropIndicatorShown(bool enable);
  fn _ZN17QAbstractItemView21setDropIndicatorShownEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QSize QAbstractItemView::sizeHintForIndex(const QModelIndex & index);
  fn _ZNK17QAbstractItemView16sizeHintForIndexERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractItemView::setItemDelegateForColumn(int column, QAbstractItemDelegate * delegate);
  fn _ZN17QAbstractItemView24setItemDelegateForColumnEiP21QAbstractItemDelegate(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  bool QAbstractItemView::dragEnabled();
  fn _ZNK17QAbstractItemView11dragEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QAbstractItemDelegate * QAbstractItemView::itemDelegate(const QModelIndex & index);
  fn _ZNK17QAbstractItemView12itemDelegateERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::keyboardSearch(const QString & search);
  fn _ZN17QAbstractItemView14keyboardSearchERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QAbstractItemView::rootIndex();
  fn _ZNK17QAbstractItemView9rootIndexEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAbstractItemDelegate * QAbstractItemView::itemDelegateForColumn(int column);
  fn _ZNK17QAbstractItemView21itemDelegateForColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QItemSelectionModel * QAbstractItemView::selectionModel();
  fn _ZNK17QAbstractItemView14selectionModelEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractItemView::viewportEntered();
  fn _ZN17QAbstractItemView15viewportEnteredEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractItemView::entered(const QModelIndex & index);
  fn _ZN17QAbstractItemView7enteredERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::reset();
  fn _ZN17QAbstractItemView5resetEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractItemView::setItemDelegateForRow(int row, QAbstractItemDelegate * delegate);
  fn _ZN17QAbstractItemView21setItemDelegateForRowEiP21QAbstractItemDelegate(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QAbstractItemView::setRootIndex(const QModelIndex & index);
  fn _ZN17QAbstractItemView12setRootIndexERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::setAutoScrollMargin(int margin);
  fn _ZN17QAbstractItemView19setAutoScrollMarginEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QRect QAbstractItemView::visualRect(const QModelIndex & index);
  fn _ZNK17QAbstractItemView10visualRectERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractItemView::doubleClicked(const QModelIndex & index);
  fn _ZN17QAbstractItemView13doubleClickedERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::doItemsLayout();
  fn _ZN17QAbstractItemView13doItemsLayoutEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractItemView::~QAbstractItemView();
  fn _ZN17QAbstractItemViewD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QAbstractItemModel * QAbstractItemView::model();
  fn _ZNK17QAbstractItemView5modelEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QAbstractItemView::iconSize();
  fn _ZNK17QAbstractItemView8iconSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractItemView::setItemDelegate(QAbstractItemDelegate * delegate);
  fn _ZN17QAbstractItemView15setItemDelegateEP21QAbstractItemDelegate(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::setDragEnabled(bool enable);
  fn _ZN17QAbstractItemView14setDragEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QModelIndex QAbstractItemView::currentIndex();
  fn _ZNK17QAbstractItemView12currentIndexEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QAbstractItemView::sizeHintForRow(int row);
  fn _ZNK17QAbstractItemView14sizeHintForRowEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QAbstractItemView::QAbstractItemView(QWidget * parent);
  fn dector_ZN17QAbstractItemViewC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN17QAbstractItemViewC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QAbstractItemView::showDropIndicator();
  fn _ZNK17QAbstractItemView17showDropIndicatorEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QAbstractItemView::hasAutoScroll();
  fn _ZNK17QAbstractItemView13hasAutoScrollEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAbstractItemView::QAbstractItemView(const QAbstractItemView & );
  fn dector_ZN17QAbstractItemViewC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN17QAbstractItemViewC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::selectAll();
  fn _ZN17QAbstractItemView9selectAllEv(qthis: u64 /* *mut c_void*/);
  // proto:  QAbstractItemDelegate * QAbstractItemView::itemDelegate();
  fn _ZNK17QAbstractItemView12itemDelegateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractItemView::edit(const QModelIndex & index);
  fn _ZN17QAbstractItemView4editERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::setAlternatingRowColors(bool enable);
  fn _ZN17QAbstractItemView23setAlternatingRowColorsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QAbstractItemView::sizeHintForColumn(int column);
  fn _ZNK17QAbstractItemView17sizeHintForColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QAbstractItemView::setIconSize(const QSize & size);
  fn _ZN17QAbstractItemView11setIconSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::closePersistentEditor(const QModelIndex & index);
  fn _ZN17QAbstractItemView21closePersistentEditorERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::setDragDropOverwriteMode(bool overwrite);
  fn _ZN17QAbstractItemView24setDragDropOverwriteModeEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QAbstractItemView::clearSelection();
  fn _ZN17QAbstractItemView14clearSelectionEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractItemView::scrollToTop();
  fn _ZN17QAbstractItemView11scrollToTopEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractItemView::setSelectionModel(QItemSelectionModel * selectionModel);
  fn _ZN17QAbstractItemView17setSelectionModelEP19QItemSelectionModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::pressed(const QModelIndex & index);
  fn _ZN17QAbstractItemView7pressedERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::setCurrentIndex(const QModelIndex & index);
  fn _ZN17QAbstractItemView15setCurrentIndexERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::clicked(const QModelIndex & index);
  fn _ZN17QAbstractItemView7clickedERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QAbstractItemView::indexAt(const QPoint & point);
  fn _ZNK17QAbstractItemView7indexAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractItemView::setTabKeyNavigation(bool enable);
  fn _ZN17QAbstractItemView19setTabKeyNavigationEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QAbstractItemView::setIndexWidget(const QModelIndex & index, QWidget * widget);
  fn _ZN17QAbstractItemView14setIndexWidgetERK11QModelIndexP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QAbstractItemView::setAutoScroll(bool enable);
  fn _ZN17QAbstractItemView13setAutoScrollEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QAbstractItemView::setModel(QAbstractItemModel * model);
  fn _ZN17QAbstractItemView8setModelEP18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::update(const QModelIndex & index);
  fn _ZN17QAbstractItemView6updateERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractItemView::iconSizeChanged(const QSize & size);
  fn _ZN17QAbstractItemView15iconSizeChangedERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QAbstractItemView::metaObject();
  fn _ZNK17QAbstractItemView10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractItemView::openPersistentEditor(const QModelIndex & index);
  fn _ZN17QAbstractItemView20openPersistentEditorERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QAbstractItemDelegate * QAbstractItemView::itemDelegateForRow(int row);
  fn _ZNK17QAbstractItemView18itemDelegateForRowEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QAbstractItemView::dragDropOverwriteMode();
  fn _ZNK17QAbstractItemView21dragDropOverwriteModeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QAbstractItemView::tabKeyNavigation();
  fn _ZNK17QAbstractItemView16tabKeyNavigationEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QAbstractItemView::autoScrollMargin();
  fn _ZNK17QAbstractItemView16autoScrollMarginEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QAbstractItemView::activated(const QModelIndex & index);
  fn _ZN17QAbstractItemView9activatedERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QAbstractItemView::alternatingRowColors();
  fn _ZNK17QAbstractItemView20alternatingRowColorsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView7enteredERK11QModelIndex(qthis: *mut c_void, fptr: *mut c_void);
  fn QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView13doubleClickedERK11QModelIndex(qthis: *mut c_void, fptr: *mut c_void);
  fn QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView7pressedERK11QModelIndex(qthis: *mut c_void, fptr: *mut c_void);
  fn QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView15viewportEnteredEv(qthis: *mut c_void, fptr: *mut c_void);
  fn QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView15iconSizeChangedERK5QSize(qthis: *mut c_void, fptr: *mut c_void);
  fn QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView9activatedERK11QModelIndex(qthis: *mut c_void, fptr: *mut c_void);
  fn QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView7clickedERK11QModelIndex(qthis: *mut c_void, fptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractItemView)=1
#[derive(Default)]
pub struct QAbstractItemView {
  qbase: QAbstractScrollArea,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _iconSizeChanged_1: QAbstractItemView_iconSizeChanged_signal,
  pub _clicked_1: QAbstractItemView_clicked_signal,
  pub _viewportEntered_1: QAbstractItemView_viewportEntered_signal,
  pub _activated_1: QAbstractItemView_activated_signal,
  pub _pressed_1: QAbstractItemView_pressed_signal,
  pub _entered_1: QAbstractItemView_entered_signal,
  pub _doubleClicked_1: QAbstractItemView_doubleClicked_signal,
}

impl /*struct*/ QAbstractItemView {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractItemView {
    return QAbstractItemView{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractItemView {
  type Target = QAbstractScrollArea;

  fn deref(&self) -> &QAbstractScrollArea {
    return & self.qbase;
  }
}
impl AsRef<QAbstractScrollArea> for QAbstractItemView {
  fn as_ref(& self) -> & QAbstractScrollArea {
    return & self.qbase;
  }
}
  // proto:  QWidget * QAbstractItemView::indexWidget(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn indexWidget<RetType, T: QAbstractItemView_indexWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexWidget(self);
    // return 1;
  }
}

pub trait QAbstractItemView_indexWidget<RetType> {
  fn indexWidget(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  QWidget * QAbstractItemView::indexWidget(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_indexWidget<QWidget> for (&'a QModelIndex) {
  fn indexWidget(self , rsthis: & QAbstractItemView) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView11indexWidgetERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QAbstractItemView11indexWidgetERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractItemView::scrollToBottom();
impl /*struct*/ QAbstractItemView {
  pub fn scrollToBottom<RetType, T: QAbstractItemView_scrollToBottom<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scrollToBottom(self);
    // return 1;
  }
}

pub trait QAbstractItemView_scrollToBottom<RetType> {
  fn scrollToBottom(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::scrollToBottom();
impl<'a> /*trait*/ QAbstractItemView_scrollToBottom<()> for () {
  fn scrollToBottom(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView14scrollToBottomEv()};
     unsafe {_ZN17QAbstractItemView14scrollToBottomEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setDropIndicatorShown(bool enable);
impl /*struct*/ QAbstractItemView {
  pub fn setDropIndicatorShown<RetType, T: QAbstractItemView_setDropIndicatorShown<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDropIndicatorShown(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setDropIndicatorShown<RetType> {
  fn setDropIndicatorShown(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setDropIndicatorShown(bool enable);
impl<'a> /*trait*/ QAbstractItemView_setDropIndicatorShown<()> for (i8) {
  fn setDropIndicatorShown(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView21setDropIndicatorShownEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN17QAbstractItemView21setDropIndicatorShownEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QAbstractItemView::sizeHintForIndex(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn sizeHintForIndex<RetType, T: QAbstractItemView_sizeHintForIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHintForIndex(self);
    // return 1;
  }
}

pub trait QAbstractItemView_sizeHintForIndex<RetType> {
  fn sizeHintForIndex(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  QSize QAbstractItemView::sizeHintForIndex(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_sizeHintForIndex<QSize> for (&'a QModelIndex) {
  fn sizeHintForIndex(self , rsthis: & QAbstractItemView) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView16sizeHintForIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QAbstractItemView16sizeHintForIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setItemDelegateForColumn(int column, QAbstractItemDelegate * delegate);
impl /*struct*/ QAbstractItemView {
  pub fn setItemDelegateForColumn<RetType, T: QAbstractItemView_setItemDelegateForColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemDelegateForColumn(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setItemDelegateForColumn<RetType> {
  fn setItemDelegateForColumn(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setItemDelegateForColumn(int column, QAbstractItemDelegate * delegate);
impl<'a> /*trait*/ QAbstractItemView_setItemDelegateForColumn<()> for (i32, &'a QAbstractItemDelegate) {
  fn setItemDelegateForColumn(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView24setItemDelegateForColumnEiP21QAbstractItemDelegate()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView24setItemDelegateForColumnEiP21QAbstractItemDelegate(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QAbstractItemView::dragEnabled();
impl /*struct*/ QAbstractItemView {
  pub fn dragEnabled<RetType, T: QAbstractItemView_dragEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dragEnabled(self);
    // return 1;
  }
}

pub trait QAbstractItemView_dragEnabled<RetType> {
  fn dragEnabled(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  bool QAbstractItemView::dragEnabled();
impl<'a> /*trait*/ QAbstractItemView_dragEnabled<i8> for () {
  fn dragEnabled(self , rsthis: & QAbstractItemView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView11dragEnabledEv()};
    let mut ret = unsafe {_ZNK17QAbstractItemView11dragEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QAbstractItemDelegate * QAbstractItemView::itemDelegate(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn itemDelegate<RetType, T: QAbstractItemView_itemDelegate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemDelegate(self);
    // return 1;
  }
}

pub trait QAbstractItemView_itemDelegate<RetType> {
  fn itemDelegate(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  QAbstractItemDelegate * QAbstractItemView::itemDelegate(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_itemDelegate<()> for (&'a QModelIndex) {
  fn itemDelegate(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView12itemDelegateERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK17QAbstractItemView12itemDelegateERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::keyboardSearch(const QString & search);
impl /*struct*/ QAbstractItemView {
  pub fn keyboardSearch<RetType, T: QAbstractItemView_keyboardSearch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyboardSearch(self);
    // return 1;
  }
}

pub trait QAbstractItemView_keyboardSearch<RetType> {
  fn keyboardSearch(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::keyboardSearch(const QString & search);
impl<'a> /*trait*/ QAbstractItemView_keyboardSearch<()> for (&'a QString) {
  fn keyboardSearch(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView14keyboardSearchERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView14keyboardSearchERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractItemView::rootIndex();
impl /*struct*/ QAbstractItemView {
  pub fn rootIndex<RetType, T: QAbstractItemView_rootIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootIndex(self);
    // return 1;
  }
}

pub trait QAbstractItemView_rootIndex<RetType> {
  fn rootIndex(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  QModelIndex QAbstractItemView::rootIndex();
impl<'a> /*trait*/ QAbstractItemView_rootIndex<QModelIndex> for () {
  fn rootIndex(self , rsthis: & QAbstractItemView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView9rootIndexEv()};
    let mut ret = unsafe {_ZNK17QAbstractItemView9rootIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAbstractItemDelegate * QAbstractItemView::itemDelegateForColumn(int column);
impl /*struct*/ QAbstractItemView {
  pub fn itemDelegateForColumn<RetType, T: QAbstractItemView_itemDelegateForColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemDelegateForColumn(self);
    // return 1;
  }
}

pub trait QAbstractItemView_itemDelegateForColumn<RetType> {
  fn itemDelegateForColumn(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  QAbstractItemDelegate * QAbstractItemView::itemDelegateForColumn(int column);
impl<'a> /*trait*/ QAbstractItemView_itemDelegateForColumn<()> for (i32) {
  fn itemDelegateForColumn(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView21itemDelegateForColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK17QAbstractItemView21itemDelegateForColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QItemSelectionModel * QAbstractItemView::selectionModel();
impl /*struct*/ QAbstractItemView {
  pub fn selectionModel<RetType, T: QAbstractItemView_selectionModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectionModel(self);
    // return 1;
  }
}

pub trait QAbstractItemView_selectionModel<RetType> {
  fn selectionModel(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  QItemSelectionModel * QAbstractItemView::selectionModel();
impl<'a> /*trait*/ QAbstractItemView_selectionModel<QItemSelectionModel> for () {
  fn selectionModel(self , rsthis: & QAbstractItemView) -> QItemSelectionModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView14selectionModelEv()};
    let mut ret = unsafe {_ZNK17QAbstractItemView14selectionModelEv(rsthis.qclsinst)};
    let mut ret1 = QItemSelectionModel::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractItemView::viewportEntered();
impl /*struct*/ QAbstractItemView {
  pub fn viewportEntered<RetType, T: QAbstractItemView_viewportEntered<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.viewportEntered(self);
    // return 1;
  }
}

pub trait QAbstractItemView_viewportEntered<RetType> {
  fn viewportEntered(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::viewportEntered();
impl<'a> /*trait*/ QAbstractItemView_viewportEntered<()> for () {
  fn viewportEntered(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView15viewportEnteredEv()};
     unsafe {_ZN17QAbstractItemView15viewportEnteredEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::entered(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn entered<RetType, T: QAbstractItemView_entered<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.entered(self);
    // return 1;
  }
}

pub trait QAbstractItemView_entered<RetType> {
  fn entered(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::entered(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_entered<()> for (&'a QModelIndex) {
  fn entered(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView7enteredERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView7enteredERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::reset();
impl /*struct*/ QAbstractItemView {
  pub fn reset<RetType, T: QAbstractItemView_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QAbstractItemView_reset<RetType> {
  fn reset(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::reset();
impl<'a> /*trait*/ QAbstractItemView_reset<()> for () {
  fn reset(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView5resetEv()};
     unsafe {_ZN17QAbstractItemView5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setItemDelegateForRow(int row, QAbstractItemDelegate * delegate);
impl /*struct*/ QAbstractItemView {
  pub fn setItemDelegateForRow<RetType, T: QAbstractItemView_setItemDelegateForRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemDelegateForRow(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setItemDelegateForRow<RetType> {
  fn setItemDelegateForRow(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setItemDelegateForRow(int row, QAbstractItemDelegate * delegate);
impl<'a> /*trait*/ QAbstractItemView_setItemDelegateForRow<()> for (i32, &'a QAbstractItemDelegate) {
  fn setItemDelegateForRow(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView21setItemDelegateForRowEiP21QAbstractItemDelegate()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView21setItemDelegateForRowEiP21QAbstractItemDelegate(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setRootIndex(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn setRootIndex<RetType, T: QAbstractItemView_setRootIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRootIndex(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setRootIndex<RetType> {
  fn setRootIndex(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_setRootIndex<()> for (&'a QModelIndex) {
  fn setRootIndex(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setAutoScrollMargin(int margin);
impl /*struct*/ QAbstractItemView {
  pub fn setAutoScrollMargin<RetType, T: QAbstractItemView_setAutoScrollMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoScrollMargin(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setAutoScrollMargin<RetType> {
  fn setAutoScrollMargin(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setAutoScrollMargin(int margin);
impl<'a> /*trait*/ QAbstractItemView_setAutoScrollMargin<()> for (i32) {
  fn setAutoScrollMargin(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView19setAutoScrollMarginEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN17QAbstractItemView19setAutoScrollMarginEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QAbstractItemView::visualRect(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn visualRect<RetType, T: QAbstractItemView_visualRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visualRect(self);
    // return 1;
  }
}

pub trait QAbstractItemView_visualRect<RetType> {
  fn visualRect(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  QRect QAbstractItemView::visualRect(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_visualRect<QRect> for (&'a QModelIndex) {
  fn visualRect(self , rsthis: & QAbstractItemView) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView10visualRectERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QAbstractItemView10visualRectERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractItemView::doubleClicked(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn doubleClicked<RetType, T: QAbstractItemView_doubleClicked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doubleClicked(self);
    // return 1;
  }
}

pub trait QAbstractItemView_doubleClicked<RetType> {
  fn doubleClicked(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::doubleClicked(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_doubleClicked<()> for (&'a QModelIndex) {
  fn doubleClicked(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView13doubleClickedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView13doubleClickedERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::doItemsLayout();
impl /*struct*/ QAbstractItemView {
  pub fn doItemsLayout<RetType, T: QAbstractItemView_doItemsLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doItemsLayout(self);
    // return 1;
  }
}

pub trait QAbstractItemView_doItemsLayout<RetType> {
  fn doItemsLayout(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::doItemsLayout();
impl<'a> /*trait*/ QAbstractItemView_doItemsLayout<()> for () {
  fn doItemsLayout(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView13doItemsLayoutEv()};
     unsafe {_ZN17QAbstractItemView13doItemsLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::~QAbstractItemView();
impl /*struct*/ QAbstractItemView {
  pub fn Free<RetType, T: QAbstractItemView_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QAbstractItemView_Free<RetType> {
  fn Free(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::~QAbstractItemView();
impl<'a> /*trait*/ QAbstractItemView_Free<()> for () {
  fn Free(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemViewD0Ev()};
     unsafe {_ZN17QAbstractItemViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAbstractItemModel * QAbstractItemView::model();
impl /*struct*/ QAbstractItemView {
  pub fn model<RetType, T: QAbstractItemView_model<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QAbstractItemView_model<RetType> {
  fn model(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  QAbstractItemModel * QAbstractItemView::model();
impl<'a> /*trait*/ QAbstractItemView_model<()> for () {
  fn model(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView5modelEv()};
     unsafe {_ZNK17QAbstractItemView5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QAbstractItemView::iconSize();
impl /*struct*/ QAbstractItemView {
  pub fn iconSize<RetType, T: QAbstractItemView_iconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconSize(self);
    // return 1;
  }
}

pub trait QAbstractItemView_iconSize<RetType> {
  fn iconSize(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  QSize QAbstractItemView::iconSize();
impl<'a> /*trait*/ QAbstractItemView_iconSize<QSize> for () {
  fn iconSize(self , rsthis: & QAbstractItemView) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView8iconSizeEv()};
    let mut ret = unsafe {_ZNK17QAbstractItemView8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setItemDelegate(QAbstractItemDelegate * delegate);
impl /*struct*/ QAbstractItemView {
  pub fn setItemDelegate<RetType, T: QAbstractItemView_setItemDelegate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemDelegate(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setItemDelegate<RetType> {
  fn setItemDelegate(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setItemDelegate(QAbstractItemDelegate * delegate);
impl<'a> /*trait*/ QAbstractItemView_setItemDelegate<()> for (&'a QAbstractItemDelegate) {
  fn setItemDelegate(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView15setItemDelegateEP21QAbstractItemDelegate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView15setItemDelegateEP21QAbstractItemDelegate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setDragEnabled(bool enable);
impl /*struct*/ QAbstractItemView {
  pub fn setDragEnabled<RetType, T: QAbstractItemView_setDragEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDragEnabled(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setDragEnabled<RetType> {
  fn setDragEnabled(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setDragEnabled(bool enable);
impl<'a> /*trait*/ QAbstractItemView_setDragEnabled<()> for (i8) {
  fn setDragEnabled(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView14setDragEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN17QAbstractItemView14setDragEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractItemView::currentIndex();
impl /*struct*/ QAbstractItemView {
  pub fn currentIndex<RetType, T: QAbstractItemView_currentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QAbstractItemView_currentIndex<RetType> {
  fn currentIndex(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  QModelIndex QAbstractItemView::currentIndex();
impl<'a> /*trait*/ QAbstractItemView_currentIndex<QModelIndex> for () {
  fn currentIndex(self , rsthis: & QAbstractItemView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView12currentIndexEv()};
    let mut ret = unsafe {_ZNK17QAbstractItemView12currentIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QAbstractItemView::sizeHintForRow(int row);
impl /*struct*/ QAbstractItemView {
  pub fn sizeHintForRow<RetType, T: QAbstractItemView_sizeHintForRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHintForRow(self);
    // return 1;
  }
}

pub trait QAbstractItemView_sizeHintForRow<RetType> {
  fn sizeHintForRow(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  int QAbstractItemView::sizeHintForRow(int row);
impl<'a> /*trait*/ QAbstractItemView_sizeHintForRow<i32> for (i32) {
  fn sizeHintForRow(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView14sizeHintForRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK17QAbstractItemView14sizeHintForRowEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAbstractItemView::QAbstractItemView(QWidget * parent);
impl /*struct*/ QAbstractItemView {
  pub fn New<T: QAbstractItemView_New>(value: T) -> QAbstractItemView {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractItemView_New {
  fn New(self) -> QAbstractItemView;
}

  // proto:  void QAbstractItemView::QAbstractItemView(QWidget * parent);
impl<'a> /*trait*/ QAbstractItemView_New for (&'a QWidget) {
  fn New(self) -> QAbstractItemView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemViewC1EP7QWidget()};
    let ctysz: c_int = unsafe{QAbstractItemView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN17QAbstractItemViewC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN17QAbstractItemViewC1EP7QWidget(arg0)} as u64;
    let rsthis = QAbstractItemView{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QAbstractItemView::showDropIndicator();
impl /*struct*/ QAbstractItemView {
  pub fn showDropIndicator<RetType, T: QAbstractItemView_showDropIndicator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showDropIndicator(self);
    // return 1;
  }
}

pub trait QAbstractItemView_showDropIndicator<RetType> {
  fn showDropIndicator(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  bool QAbstractItemView::showDropIndicator();
impl<'a> /*trait*/ QAbstractItemView_showDropIndicator<i8> for () {
  fn showDropIndicator(self , rsthis: & QAbstractItemView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView17showDropIndicatorEv()};
    let mut ret = unsafe {_ZNK17QAbstractItemView17showDropIndicatorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractItemView::hasAutoScroll();
impl /*struct*/ QAbstractItemView {
  pub fn hasAutoScroll<RetType, T: QAbstractItemView_hasAutoScroll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasAutoScroll(self);
    // return 1;
  }
}

pub trait QAbstractItemView_hasAutoScroll<RetType> {
  fn hasAutoScroll(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  bool QAbstractItemView::hasAutoScroll();
impl<'a> /*trait*/ QAbstractItemView_hasAutoScroll<i8> for () {
  fn hasAutoScroll(self , rsthis: & QAbstractItemView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView13hasAutoScrollEv()};
    let mut ret = unsafe {_ZNK17QAbstractItemView13hasAutoScrollEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractItemView::QAbstractItemView(const QAbstractItemView & );
impl<'a> /*trait*/ QAbstractItemView_New for (&'a QAbstractItemView) {
  fn New(self) -> QAbstractItemView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemViewC1ERKS_()};
    let ctysz: c_int = unsafe{QAbstractItemView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN17QAbstractItemViewC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN17QAbstractItemViewC1ERKS_(arg0)} as u64;
    let rsthis = QAbstractItemView{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractItemView::selectAll();
impl /*struct*/ QAbstractItemView {
  pub fn selectAll<RetType, T: QAbstractItemView_selectAll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectAll(self);
    // return 1;
  }
}

pub trait QAbstractItemView_selectAll<RetType> {
  fn selectAll(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::selectAll();
impl<'a> /*trait*/ QAbstractItemView_selectAll<()> for () {
  fn selectAll(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView9selectAllEv()};
     unsafe {_ZN17QAbstractItemView9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAbstractItemDelegate * QAbstractItemView::itemDelegate();
impl<'a> /*trait*/ QAbstractItemView_itemDelegate<()> for () {
  fn itemDelegate(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView12itemDelegateEv()};
     unsafe {_ZNK17QAbstractItemView12itemDelegateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::edit(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn edit<RetType, T: QAbstractItemView_edit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.edit(self);
    // return 1;
  }
}

pub trait QAbstractItemView_edit<RetType> {
  fn edit(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::edit(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_edit<()> for (&'a QModelIndex) {
  fn edit(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView4editERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView4editERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setAlternatingRowColors(bool enable);
impl /*struct*/ QAbstractItemView {
  pub fn setAlternatingRowColors<RetType, T: QAbstractItemView_setAlternatingRowColors<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAlternatingRowColors(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setAlternatingRowColors<RetType> {
  fn setAlternatingRowColors(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setAlternatingRowColors(bool enable);
impl<'a> /*trait*/ QAbstractItemView_setAlternatingRowColors<()> for (i8) {
  fn setAlternatingRowColors(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView23setAlternatingRowColorsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN17QAbstractItemView23setAlternatingRowColorsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAbstractItemView::sizeHintForColumn(int column);
impl /*struct*/ QAbstractItemView {
  pub fn sizeHintForColumn<RetType, T: QAbstractItemView_sizeHintForColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHintForColumn(self);
    // return 1;
  }
}

pub trait QAbstractItemView_sizeHintForColumn<RetType> {
  fn sizeHintForColumn(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  int QAbstractItemView::sizeHintForColumn(int column);
impl<'a> /*trait*/ QAbstractItemView_sizeHintForColumn<i32> for (i32) {
  fn sizeHintForColumn(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView17sizeHintForColumnEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK17QAbstractItemView17sizeHintForColumnEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setIconSize(const QSize & size);
impl /*struct*/ QAbstractItemView {
  pub fn setIconSize<RetType, T: QAbstractItemView_setIconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIconSize(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setIconSize<RetType> {
  fn setIconSize(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setIconSize(const QSize & size);
impl<'a> /*trait*/ QAbstractItemView_setIconSize<()> for (&'a QSize) {
  fn setIconSize(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::closePersistentEditor(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn closePersistentEditor<RetType, T: QAbstractItemView_closePersistentEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.closePersistentEditor(self);
    // return 1;
  }
}

pub trait QAbstractItemView_closePersistentEditor<RetType> {
  fn closePersistentEditor(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::closePersistentEditor(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_closePersistentEditor<()> for (&'a QModelIndex) {
  fn closePersistentEditor(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView21closePersistentEditorERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView21closePersistentEditorERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setDragDropOverwriteMode(bool overwrite);
impl /*struct*/ QAbstractItemView {
  pub fn setDragDropOverwriteMode<RetType, T: QAbstractItemView_setDragDropOverwriteMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDragDropOverwriteMode(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setDragDropOverwriteMode<RetType> {
  fn setDragDropOverwriteMode(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setDragDropOverwriteMode(bool overwrite);
impl<'a> /*trait*/ QAbstractItemView_setDragDropOverwriteMode<()> for (i8) {
  fn setDragDropOverwriteMode(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView24setDragDropOverwriteModeEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN17QAbstractItemView24setDragDropOverwriteModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::clearSelection();
impl /*struct*/ QAbstractItemView {
  pub fn clearSelection<RetType, T: QAbstractItemView_clearSelection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearSelection(self);
    // return 1;
  }
}

pub trait QAbstractItemView_clearSelection<RetType> {
  fn clearSelection(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::clearSelection();
impl<'a> /*trait*/ QAbstractItemView_clearSelection<()> for () {
  fn clearSelection(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView14clearSelectionEv()};
     unsafe {_ZN17QAbstractItemView14clearSelectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::scrollToTop();
impl /*struct*/ QAbstractItemView {
  pub fn scrollToTop<RetType, T: QAbstractItemView_scrollToTop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scrollToTop(self);
    // return 1;
  }
}

pub trait QAbstractItemView_scrollToTop<RetType> {
  fn scrollToTop(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::scrollToTop();
impl<'a> /*trait*/ QAbstractItemView_scrollToTop<()> for () {
  fn scrollToTop(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView11scrollToTopEv()};
     unsafe {_ZN17QAbstractItemView11scrollToTopEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setSelectionModel(QItemSelectionModel * selectionModel);
impl /*struct*/ QAbstractItemView {
  pub fn setSelectionModel<RetType, T: QAbstractItemView_setSelectionModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelectionModel(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setSelectionModel<RetType> {
  fn setSelectionModel(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setSelectionModel(QItemSelectionModel * selectionModel);
impl<'a> /*trait*/ QAbstractItemView_setSelectionModel<()> for (&'a QItemSelectionModel) {
  fn setSelectionModel(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView17setSelectionModelEP19QItemSelectionModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView17setSelectionModelEP19QItemSelectionModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::pressed(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn pressed<RetType, T: QAbstractItemView_pressed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pressed(self);
    // return 1;
  }
}

pub trait QAbstractItemView_pressed<RetType> {
  fn pressed(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::pressed(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_pressed<()> for (&'a QModelIndex) {
  fn pressed(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView7pressedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView7pressedERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setCurrentIndex(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn setCurrentIndex<RetType, T: QAbstractItemView_setCurrentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setCurrentIndex<RetType> {
  fn setCurrentIndex(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setCurrentIndex(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_setCurrentIndex<()> for (&'a QModelIndex) {
  fn setCurrentIndex(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView15setCurrentIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView15setCurrentIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::clicked(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn clicked<RetType, T: QAbstractItemView_clicked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clicked(self);
    // return 1;
  }
}

pub trait QAbstractItemView_clicked<RetType> {
  fn clicked(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::clicked(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_clicked<()> for (&'a QModelIndex) {
  fn clicked(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView7clickedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView7clickedERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractItemView::indexAt(const QPoint & point);
impl /*struct*/ QAbstractItemView {
  pub fn indexAt<RetType, T: QAbstractItemView_indexAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexAt(self);
    // return 1;
  }
}

pub trait QAbstractItemView_indexAt<RetType> {
  fn indexAt(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  QModelIndex QAbstractItemView::indexAt(const QPoint & point);
impl<'a> /*trait*/ QAbstractItemView_indexAt<QModelIndex> for (&'a QPoint) {
  fn indexAt(self , rsthis: & QAbstractItemView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView7indexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QAbstractItemView7indexAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setTabKeyNavigation(bool enable);
impl /*struct*/ QAbstractItemView {
  pub fn setTabKeyNavigation<RetType, T: QAbstractItemView_setTabKeyNavigation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabKeyNavigation(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setTabKeyNavigation<RetType> {
  fn setTabKeyNavigation(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setTabKeyNavigation(bool enable);
impl<'a> /*trait*/ QAbstractItemView_setTabKeyNavigation<()> for (i8) {
  fn setTabKeyNavigation(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView19setTabKeyNavigationEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN17QAbstractItemView19setTabKeyNavigationEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setIndexWidget(const QModelIndex & index, QWidget * widget);
impl /*struct*/ QAbstractItemView {
  pub fn setIndexWidget<RetType, T: QAbstractItemView_setIndexWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIndexWidget(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setIndexWidget<RetType> {
  fn setIndexWidget(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setIndexWidget(const QModelIndex & index, QWidget * widget);
impl<'a> /*trait*/ QAbstractItemView_setIndexWidget<()> for (&'a QModelIndex, &'a QWidget) {
  fn setIndexWidget(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView14setIndexWidgetERK11QModelIndexP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView14setIndexWidgetERK11QModelIndexP7QWidget(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setAutoScroll(bool enable);
impl /*struct*/ QAbstractItemView {
  pub fn setAutoScroll<RetType, T: QAbstractItemView_setAutoScroll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoScroll(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setAutoScroll<RetType> {
  fn setAutoScroll(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setAutoScroll(bool enable);
impl<'a> /*trait*/ QAbstractItemView_setAutoScroll<()> for (i8) {
  fn setAutoScroll(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView13setAutoScrollEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN17QAbstractItemView13setAutoScrollEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::setModel(QAbstractItemModel * model);
impl /*struct*/ QAbstractItemView {
  pub fn setModel<RetType, T: QAbstractItemView_setModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModel(self);
    // return 1;
  }
}

pub trait QAbstractItemView_setModel<RetType> {
  fn setModel(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::setModel(QAbstractItemModel * model);
impl<'a> /*trait*/ QAbstractItemView_setModel<()> for (&'a QAbstractItemModel) {
  fn setModel(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView8setModelEP18QAbstractItemModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView8setModelEP18QAbstractItemModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::update(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn update<RetType, T: QAbstractItemView_update<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.update(self);
    // return 1;
  }
}

pub trait QAbstractItemView_update<RetType> {
  fn update(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::update(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_update<()> for (&'a QModelIndex) {
  fn update(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView6updateERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView6updateERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::iconSizeChanged(const QSize & size);
impl /*struct*/ QAbstractItemView {
  pub fn iconSizeChanged<RetType, T: QAbstractItemView_iconSizeChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconSizeChanged(self);
    // return 1;
  }
}

pub trait QAbstractItemView_iconSizeChanged<RetType> {
  fn iconSizeChanged(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::iconSizeChanged(const QSize & size);
impl<'a> /*trait*/ QAbstractItemView_iconSizeChanged<()> for (&'a QSize) {
  fn iconSizeChanged(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView15iconSizeChangedERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView15iconSizeChangedERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractItemView::metaObject();
impl /*struct*/ QAbstractItemView {
  pub fn metaObject<RetType, T: QAbstractItemView_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractItemView_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  const QMetaObject * QAbstractItemView::metaObject();
impl<'a> /*trait*/ QAbstractItemView_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView10metaObjectEv()};
     unsafe {_ZNK17QAbstractItemView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractItemView::openPersistentEditor(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn openPersistentEditor<RetType, T: QAbstractItemView_openPersistentEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.openPersistentEditor(self);
    // return 1;
  }
}

pub trait QAbstractItemView_openPersistentEditor<RetType> {
  fn openPersistentEditor(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::openPersistentEditor(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_openPersistentEditor<()> for (&'a QModelIndex) {
  fn openPersistentEditor(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView20openPersistentEditorERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView20openPersistentEditorERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAbstractItemDelegate * QAbstractItemView::itemDelegateForRow(int row);
impl /*struct*/ QAbstractItemView {
  pub fn itemDelegateForRow<RetType, T: QAbstractItemView_itemDelegateForRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemDelegateForRow(self);
    // return 1;
  }
}

pub trait QAbstractItemView_itemDelegateForRow<RetType> {
  fn itemDelegateForRow(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  QAbstractItemDelegate * QAbstractItemView::itemDelegateForRow(int row);
impl<'a> /*trait*/ QAbstractItemView_itemDelegateForRow<()> for (i32) {
  fn itemDelegateForRow(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView18itemDelegateForRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK17QAbstractItemView18itemDelegateForRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAbstractItemView::dragDropOverwriteMode();
impl /*struct*/ QAbstractItemView {
  pub fn dragDropOverwriteMode<RetType, T: QAbstractItemView_dragDropOverwriteMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dragDropOverwriteMode(self);
    // return 1;
  }
}

pub trait QAbstractItemView_dragDropOverwriteMode<RetType> {
  fn dragDropOverwriteMode(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  bool QAbstractItemView::dragDropOverwriteMode();
impl<'a> /*trait*/ QAbstractItemView_dragDropOverwriteMode<i8> for () {
  fn dragDropOverwriteMode(self , rsthis: & QAbstractItemView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView21dragDropOverwriteModeEv()};
    let mut ret = unsafe {_ZNK17QAbstractItemView21dragDropOverwriteModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractItemView::tabKeyNavigation();
impl /*struct*/ QAbstractItemView {
  pub fn tabKeyNavigation<RetType, T: QAbstractItemView_tabKeyNavigation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabKeyNavigation(self);
    // return 1;
  }
}

pub trait QAbstractItemView_tabKeyNavigation<RetType> {
  fn tabKeyNavigation(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  bool QAbstractItemView::tabKeyNavigation();
impl<'a> /*trait*/ QAbstractItemView_tabKeyNavigation<i8> for () {
  fn tabKeyNavigation(self , rsthis: & QAbstractItemView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView16tabKeyNavigationEv()};
    let mut ret = unsafe {_ZNK17QAbstractItemView16tabKeyNavigationEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QAbstractItemView::autoScrollMargin();
impl /*struct*/ QAbstractItemView {
  pub fn autoScrollMargin<RetType, T: QAbstractItemView_autoScrollMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoScrollMargin(self);
    // return 1;
  }
}

pub trait QAbstractItemView_autoScrollMargin<RetType> {
  fn autoScrollMargin(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  int QAbstractItemView::autoScrollMargin();
impl<'a> /*trait*/ QAbstractItemView_autoScrollMargin<i32> for () {
  fn autoScrollMargin(self , rsthis: & QAbstractItemView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView16autoScrollMarginEv()};
    let mut ret = unsafe {_ZNK17QAbstractItemView16autoScrollMarginEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAbstractItemView::activated(const QModelIndex & index);
impl /*struct*/ QAbstractItemView {
  pub fn activated<RetType, T: QAbstractItemView_activated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activated(self);
    // return 1;
  }
}

pub trait QAbstractItemView_activated<RetType> {
  fn activated(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  void QAbstractItemView::activated(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemView_activated<()> for (&'a QModelIndex) {
  fn activated(self , rsthis: & QAbstractItemView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractItemView9activatedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAbstractItemView9activatedERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAbstractItemView::alternatingRowColors();
impl /*struct*/ QAbstractItemView {
  pub fn alternatingRowColors<RetType, T: QAbstractItemView_alternatingRowColors<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.alternatingRowColors(self);
    // return 1;
  }
}

pub trait QAbstractItemView_alternatingRowColors<RetType> {
  fn alternatingRowColors(self , rsthis: & QAbstractItemView) -> RetType;
}

  // proto:  bool QAbstractItemView::alternatingRowColors();
impl<'a> /*trait*/ QAbstractItemView_alternatingRowColors<i8> for () {
  fn alternatingRowColors(self , rsthis: & QAbstractItemView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAbstractItemView20alternatingRowColorsEv()};
    let mut ret = unsafe {_ZNK17QAbstractItemView20alternatingRowColorsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

#[derive(Default)] // for QAbstractItemView_iconSizeChanged
pub struct QAbstractItemView_iconSizeChanged_signal{poi:u64}
impl /* struct */ QAbstractItemView {
  pub fn iconSizeChanged_1(self) -> QAbstractItemView_iconSizeChanged_signal {
     return QAbstractItemView_iconSizeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemView_iconSizeChanged_signal {
  pub fn connect<T: QAbstractItemView_iconSizeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemView_iconSizeChanged_signal_connect {
  fn connect(self, sigthis: QAbstractItemView_iconSizeChanged_signal);
}

#[derive(Default)] // for QAbstractItemView_clicked
pub struct QAbstractItemView_clicked_signal{poi:u64}
impl /* struct */ QAbstractItemView {
  pub fn clicked_1(self) -> QAbstractItemView_clicked_signal {
     return QAbstractItemView_clicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemView_clicked_signal {
  pub fn connect<T: QAbstractItemView_clicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemView_clicked_signal_connect {
  fn connect(self, sigthis: QAbstractItemView_clicked_signal);
}

#[derive(Default)] // for QAbstractItemView_viewportEntered
pub struct QAbstractItemView_viewportEntered_signal{poi:u64}
impl /* struct */ QAbstractItemView {
  pub fn viewportEntered_1(self) -> QAbstractItemView_viewportEntered_signal {
     return QAbstractItemView_viewportEntered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemView_viewportEntered_signal {
  pub fn connect<T: QAbstractItemView_viewportEntered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemView_viewportEntered_signal_connect {
  fn connect(self, sigthis: QAbstractItemView_viewportEntered_signal);
}

#[derive(Default)] // for QAbstractItemView_activated
pub struct QAbstractItemView_activated_signal{poi:u64}
impl /* struct */ QAbstractItemView {
  pub fn activated_1(self) -> QAbstractItemView_activated_signal {
     return QAbstractItemView_activated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemView_activated_signal {
  pub fn connect<T: QAbstractItemView_activated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemView_activated_signal_connect {
  fn connect(self, sigthis: QAbstractItemView_activated_signal);
}

#[derive(Default)] // for QAbstractItemView_pressed
pub struct QAbstractItemView_pressed_signal{poi:u64}
impl /* struct */ QAbstractItemView {
  pub fn pressed_1(self) -> QAbstractItemView_pressed_signal {
     return QAbstractItemView_pressed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemView_pressed_signal {
  pub fn connect<T: QAbstractItemView_pressed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemView_pressed_signal_connect {
  fn connect(self, sigthis: QAbstractItemView_pressed_signal);
}

#[derive(Default)] // for QAbstractItemView_entered
pub struct QAbstractItemView_entered_signal{poi:u64}
impl /* struct */ QAbstractItemView {
  pub fn entered_1(self) -> QAbstractItemView_entered_signal {
     return QAbstractItemView_entered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemView_entered_signal {
  pub fn connect<T: QAbstractItemView_entered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemView_entered_signal_connect {
  fn connect(self, sigthis: QAbstractItemView_entered_signal);
}

#[derive(Default)] // for QAbstractItemView_doubleClicked
pub struct QAbstractItemView_doubleClicked_signal{poi:u64}
impl /* struct */ QAbstractItemView {
  pub fn doubleClicked_1(self) -> QAbstractItemView_doubleClicked_signal {
     return QAbstractItemView_doubleClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemView_doubleClicked_signal {
  pub fn connect<T: QAbstractItemView_doubleClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemView_doubleClicked_signal_connect {
  fn connect(self, sigthis: QAbstractItemView_doubleClicked_signal);
}

// entered(const class QModelIndex &)
extern fn QAbstractItemView_entered_signal_connect_cb_0(arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QAbstractItemView_entered_signal_connect for (extern fn(QModelIndex)) {
  fn connect(self, sigthis: QAbstractItemView_entered_signal) {
    // do smth...
    unsafe {QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView7enteredERK11QModelIndex(sigthis.poi as *mut c_void, QAbstractItemView_entered_signal_connect_cb_0 as *mut c_void)};
  }
}
// doubleClicked(const class QModelIndex &)
extern fn QAbstractItemView_doubleClicked_signal_connect_cb_1(arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QAbstractItemView_doubleClicked_signal_connect for (extern fn(QModelIndex)) {
  fn connect(self, sigthis: QAbstractItemView_doubleClicked_signal) {
    // do smth...
    unsafe {QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView13doubleClickedERK11QModelIndex(sigthis.poi as *mut c_void, QAbstractItemView_doubleClicked_signal_connect_cb_1 as *mut c_void)};
  }
}
// pressed(const class QModelIndex &)
extern fn QAbstractItemView_pressed_signal_connect_cb_2(arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QAbstractItemView_pressed_signal_connect for (extern fn(QModelIndex)) {
  fn connect(self, sigthis: QAbstractItemView_pressed_signal) {
    // do smth...
    unsafe {QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView7pressedERK11QModelIndex(sigthis.poi as *mut c_void, QAbstractItemView_pressed_signal_connect_cb_2 as *mut c_void)};
  }
}
// viewportEntered()
extern fn QAbstractItemView_viewportEntered_signal_connect_cb_3() {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QAbstractItemView_viewportEntered_signal_connect for (extern fn()) {
  fn connect(self, sigthis: QAbstractItemView_viewportEntered_signal) {
    // do smth...
    unsafe {QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView15viewportEnteredEv(sigthis.poi as *mut c_void, QAbstractItemView_viewportEntered_signal_connect_cb_3 as *mut c_void)};
  }
}
// iconSizeChanged(const class QSize &)
extern fn QAbstractItemView_iconSizeChanged_signal_connect_cb_4(arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QAbstractItemView_iconSizeChanged_signal_connect for (extern fn(QSize)) {
  fn connect(self, sigthis: QAbstractItemView_iconSizeChanged_signal) {
    // do smth...
    unsafe {QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView15iconSizeChangedERK5QSize(sigthis.poi as *mut c_void, QAbstractItemView_iconSizeChanged_signal_connect_cb_4 as *mut c_void)};
  }
}
// activated(const class QModelIndex &)
extern fn QAbstractItemView_activated_signal_connect_cb_5(arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QAbstractItemView_activated_signal_connect for (extern fn(QModelIndex)) {
  fn connect(self, sigthis: QAbstractItemView_activated_signal) {
    // do smth...
    unsafe {QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView9activatedERK11QModelIndex(sigthis.poi as *mut c_void, QAbstractItemView_activated_signal_connect_cb_5 as *mut c_void)};
  }
}
// clicked(const class QModelIndex &)
extern fn QAbstractItemView_clicked_signal_connect_cb_6(arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QAbstractItemView_clicked_signal_connect for (extern fn(QModelIndex)) {
  fn connect(self, sigthis: QAbstractItemView_clicked_signal) {
    // do smth...
    unsafe {QAbstractItemView_SlotProxy_connect__ZN17QAbstractItemView7clickedERK11QModelIndex(sigthis.poi as *mut c_void, QAbstractItemView_clicked_signal_connect_cb_6 as *mut c_void)};
  }
}
// <= body block end

