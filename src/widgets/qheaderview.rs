// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
// src-file: /QtWidgets/qheaderview.h
// dst-file: /src/widgets/qheaderview.rs
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
use super::qabstractitemview::QAbstractItemView; // 773
use std::ops::Deref;
use super::super::core::qsize::QSize; // 771
use super::super::core::qbytearray::QByteArray; // 771
use super::super::core::qpoint::QPoint; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qabstractitemmodel::QAbstractItemModel; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QHeaderView_Class_Size() -> c_int;
  // proto:  int QHeaderView::maximumSectionSize();
  fn _ZNK11QHeaderView18maximumSectionSizeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QSize QHeaderView::sizeHint();
  fn _ZNK11QHeaderView8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QHeaderView::sectionPosition(int logicalIndex);
  fn _ZNK11QHeaderView15sectionPositionEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QHeaderView::sectionResized(int logicalIndex, int oldSize, int newSize);
  fn _ZN11QHeaderView14sectionResizedEiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int);
  // proto:  int QHeaderView::sectionSize(int logicalIndex);
  fn _ZNK11QHeaderView11sectionSizeEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QHeaderView::QHeaderView(const QHeaderView & );
  fn dector_ZN11QHeaderViewC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QHeaderViewC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QHeaderView::setStretchLastSection(bool stretch);
  fn _ZN11QHeaderView21setStretchLastSectionEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QHeaderView::reset();
  fn _ZN11QHeaderView5resetEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QHeaderView::geometriesChanged();
  fn _ZN11QHeaderView17geometriesChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QHeaderView::resetDefaultSectionSize();
  fn _ZN11QHeaderView23resetDefaultSectionSizeEv(qthis: u64 /* *mut c_void*/);
  // proto:  QByteArray QHeaderView::saveState();
  fn _ZNK11QHeaderView9saveStateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QHeaderView::sectionsClickable();
  fn _ZNK11QHeaderView17sectionsClickableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QHeaderView::resizeContentsPrecision();
  fn _ZNK11QHeaderView23resizeContentsPrecisionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QHeaderView::setOffsetToSectionPosition(int visualIndex);
  fn _ZN11QHeaderView26setOffsetToSectionPositionEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QHeaderView::length();
  fn _ZNK11QHeaderView6lengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QHeaderView::hideSection(int logicalIndex);
  fn demth_ZN11QHeaderView11hideSectionEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QHeaderView::sortIndicatorSection();
  fn _ZNK11QHeaderView20sortIndicatorSectionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QHeaderView::cascadingSectionResizes();
  fn _ZNK11QHeaderView23cascadingSectionResizesEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QHeaderView::setMinimumSectionSize(int size);
  fn _ZN11QHeaderView21setMinimumSectionSizeEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QHeaderView::visualIndexAt(int position);
  fn _ZNK11QHeaderView13visualIndexAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QHeaderView::setOffset(int offset);
  fn _ZN11QHeaderView9setOffsetEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QHeaderView::logicalIndexAt(const QPoint & pos);
  fn demth_ZNK11QHeaderView14logicalIndexAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QHeaderView::~QHeaderView();
  fn _ZN11QHeaderViewD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QHeaderView::sectionViewportPosition(int logicalIndex);
  fn _ZNK11QHeaderView23sectionViewportPositionEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  bool QHeaderView::highlightSections();
  fn _ZNK11QHeaderView17highlightSectionsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QHeaderView::offset();
  fn _ZNK11QHeaderView6offsetEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QHeaderView::setSortIndicatorShown(bool show);
  fn _ZN11QHeaderView21setSortIndicatorShownEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  const QMetaObject * QHeaderView::metaObject();
  fn _ZNK11QHeaderView10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QHeaderView::showSection(int logicalIndex);
  fn demth_ZN11QHeaderView11showSectionEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QHeaderView::setVisible(bool v);
  fn _ZN11QHeaderView10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QHeaderView::hiddenSectionCount();
  fn _ZNK11QHeaderView18hiddenSectionCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QHeaderView::sectionMoved(int logicalIndex, int oldVisualIndex, int newVisualIndex);
  fn _ZN11QHeaderView12sectionMovedEiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int);
  // proto:  void QHeaderView::sectionHandleDoubleClicked(int logicalIndex);
  fn _ZN11QHeaderView26sectionHandleDoubleClickedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QHeaderView::setSectionsClickable(bool clickable);
  fn _ZN11QHeaderView20setSectionsClickableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QHeaderView::sectionPressed(int logicalIndex);
  fn _ZN11QHeaderView14sectionPressedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QHeaderView::setResizeContentsPrecision(int precision);
  fn _ZN11QHeaderView26setResizeContentsPrecisionEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QHeaderView::defaultSectionSize();
  fn _ZNK11QHeaderView18defaultSectionSizeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QHeaderView::setOffsetToLastSection();
  fn _ZN11QHeaderView22setOffsetToLastSectionEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QHeaderView::swapSections(int first, int second);
  fn _ZN11QHeaderView12swapSectionsEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  int QHeaderView::count();
  fn _ZNK11QHeaderView5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QHeaderView::visualIndex(int logicalIndex);
  fn _ZNK11QHeaderView11visualIndexEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QHeaderView::sectionClicked(int logicalIndex);
  fn _ZN11QHeaderView14sectionClickedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QHeaderView::sectionsMoved();
  fn _ZNK11QHeaderView13sectionsMovedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QHeaderView::stretchSectionCount();
  fn _ZNK11QHeaderView19stretchSectionCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QHeaderView::doItemsLayout();
  fn _ZN11QHeaderView13doItemsLayoutEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QHeaderView::setSectionsMovable(bool movable);
  fn _ZN11QHeaderView18setSectionsMovableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QHeaderView::sectionsHidden();
  fn _ZNK11QHeaderView14sectionsHiddenEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QHeaderView::minimumSectionSize();
  fn _ZNK11QHeaderView18minimumSectionSizeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QHeaderView::setCascadingSectionResizes(bool enable);
  fn _ZN11QHeaderView26setCascadingSectionResizesEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QHeaderView::setDefaultSectionSize(int size);
  fn _ZN11QHeaderView21setDefaultSectionSizeEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QHeaderView::sectionEntered(int logicalIndex);
  fn _ZN11QHeaderView14sectionEnteredEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QHeaderView::moveSection(int from, int to);
  fn _ZN11QHeaderView11moveSectionEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  bool QHeaderView::stretchLastSection();
  fn _ZNK11QHeaderView18stretchLastSectionEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QHeaderView::sectionSizeHint(int logicalIndex);
  fn _ZNK11QHeaderView15sectionSizeHintEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  bool QHeaderView::sectionsMovable();
  fn _ZNK11QHeaderView15sectionsMovableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QHeaderView::isSectionHidden(int logicalIndex);
  fn _ZNK11QHeaderView15isSectionHiddenEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  int QHeaderView::logicalIndexAt(int x, int y);
  fn demth_ZNK11QHeaderView14logicalIndexAtEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> c_int;
  // proto:  int QHeaderView::logicalIndexAt(int position);
  fn _ZNK11QHeaderView14logicalIndexAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  int QHeaderView::logicalIndex(int visualIndex);
  fn _ZNK11QHeaderView12logicalIndexEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QHeaderView::setMaximumSectionSize(int size);
  fn _ZN11QHeaderView21setMaximumSectionSizeEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QHeaderView::setHighlightSections(bool highlight);
  fn _ZN11QHeaderView20setHighlightSectionsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QHeaderView::setSectionHidden(int logicalIndex, bool hide);
  fn _ZN11QHeaderView16setSectionHiddenEib(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_char);
  // proto:  void QHeaderView::resizeSection(int logicalIndex, int size);
  fn _ZN11QHeaderView13resizeSectionEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  bool QHeaderView::restoreState(const QByteArray & state);
  fn _ZN11QHeaderView12restoreStateERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QHeaderView::sectionDoubleClicked(int logicalIndex);
  fn _ZN11QHeaderView20sectionDoubleClickedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QHeaderView::sectionCountChanged(int oldCount, int newCount);
  fn _ZN11QHeaderView19sectionCountChangedEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QHeaderView::setModel(QAbstractItemModel * model);
  fn _ZN11QHeaderView8setModelEP18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QHeaderView::isSortIndicatorShown();
  fn _ZNK11QHeaderView20isSortIndicatorShownEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QHeaderView_SlotProxy_connect__ZN11QHeaderView14sectionEnteredEi(qthis: *mut c_void, fptr: *mut c_void);
  fn QHeaderView_SlotProxy_connect__ZN11QHeaderView17geometriesChangedEv(qthis: *mut c_void, fptr: *mut c_void);
  fn QHeaderView_SlotProxy_connect__ZN11QHeaderView14sectionClickedEi(qthis: *mut c_void, fptr: *mut c_void);
  fn QHeaderView_SlotProxy_connect__ZN11QHeaderView12sectionMovedEiii(qthis: *mut c_void, fptr: *mut c_void);
  fn QHeaderView_SlotProxy_connect__ZN11QHeaderView14sectionPressedEi(qthis: *mut c_void, fptr: *mut c_void);
  fn QHeaderView_SlotProxy_connect__ZN11QHeaderView20sectionDoubleClickedEi(qthis: *mut c_void, fptr: *mut c_void);
  fn QHeaderView_SlotProxy_connect__ZN11QHeaderView26sectionHandleDoubleClickedEi(qthis: *mut c_void, fptr: *mut c_void);
  fn QHeaderView_SlotProxy_connect__ZN11QHeaderView14sectionResizedEiii(qthis: *mut c_void, fptr: *mut c_void);
  fn QHeaderView_SlotProxy_connect__ZN11QHeaderView19sectionCountChangedEii(qthis: *mut c_void, fptr: *mut c_void);
  fn QHeaderView_SlotProxy_connect__ZN11QHeaderView20sortIndicatorChangedEiN2Qt9SortOrderE(qthis: *mut c_void, fptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QHeaderView)=1
#[derive(Default)]
pub struct QHeaderView {
  qbase: QAbstractItemView,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _sectionHandleDoubleClicked_1: QHeaderView_sectionHandleDoubleClicked_signal,
  pub _sectionEntered_1: QHeaderView_sectionEntered_signal,
  pub _sortIndicatorChanged_1: QHeaderView_sortIndicatorChanged_signal,
  pub _sectionClicked_1: QHeaderView_sectionClicked_signal,
  pub _sectionCountChanged_1: QHeaderView_sectionCountChanged_signal,
  pub _geometriesChanged_1: QHeaderView_geometriesChanged_signal,
  pub _sectionMoved_1: QHeaderView_sectionMoved_signal,
  pub _sectionPressed_1: QHeaderView_sectionPressed_signal,
  pub _sectionDoubleClicked_1: QHeaderView_sectionDoubleClicked_signal,
  pub _sectionResized_1: QHeaderView_sectionResized_signal,
}

impl /*struct*/ QHeaderView {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QHeaderView {
    return QHeaderView{qbase: QAbstractItemView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QHeaderView {
  type Target = QAbstractItemView;

  fn deref(&self) -> &QAbstractItemView {
    return & self.qbase;
  }
}
impl AsRef<QAbstractItemView> for QHeaderView {
  fn as_ref(& self) -> & QAbstractItemView {
    return & self.qbase;
  }
}
  // proto:  int QHeaderView::maximumSectionSize();
impl /*struct*/ QHeaderView {
  pub fn maximumSectionSize<RetType, T: QHeaderView_maximumSectionSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_maximumSectionSize<RetType> {
  fn maximumSectionSize(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::maximumSectionSize();
impl<'a> /*trait*/ QHeaderView_maximumSectionSize<i32> for () {
  fn maximumSectionSize(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18maximumSectionSizeEv()};
    let mut ret = unsafe {_ZNK11QHeaderView18maximumSectionSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QHeaderView::sizeHint();
impl /*struct*/ QHeaderView {
  pub fn sizeHint<RetType, T: QHeaderView_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QHeaderView_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  QSize QHeaderView::sizeHint();
impl<'a> /*trait*/ QHeaderView_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QHeaderView) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QHeaderView8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QHeaderView::sectionPosition(int logicalIndex);
impl /*struct*/ QHeaderView {
  pub fn sectionPosition<RetType, T: QHeaderView_sectionPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionPosition(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionPosition<RetType> {
  fn sectionPosition(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::sectionPosition(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionPosition<i32> for (i32) {
  fn sectionPosition(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView15sectionPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView15sectionPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHeaderView::sectionResized(int logicalIndex, int oldSize, int newSize);
impl /*struct*/ QHeaderView {
  pub fn sectionResized<RetType, T: QHeaderView_sectionResized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionResized(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionResized<RetType> {
  fn sectionResized(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::sectionResized(int logicalIndex, int oldSize, int newSize);
impl<'a> /*trait*/ QHeaderView_sectionResized<()> for (i32, i32, i32) {
  fn sectionResized(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView14sectionResizedEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN11QHeaderView14sectionResizedEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  int QHeaderView::sectionSize(int logicalIndex);
impl /*struct*/ QHeaderView {
  pub fn sectionSize<RetType, T: QHeaderView_sectionSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionSize<RetType> {
  fn sectionSize(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::sectionSize(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionSize<i32> for (i32) {
  fn sectionSize(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView11sectionSizeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView11sectionSizeEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHeaderView::QHeaderView(const QHeaderView & );
impl /*struct*/ QHeaderView {
  pub fn New<T: QHeaderView_New>(value: T) -> QHeaderView {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QHeaderView_New {
  fn New(self) -> QHeaderView;
}

  // proto:  void QHeaderView::QHeaderView(const QHeaderView & );
impl<'a> /*trait*/ QHeaderView_New for (&'a QHeaderView) {
  fn New(self) -> QHeaderView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderViewC1ERKS_()};
    let ctysz: c_int = unsafe{QHeaderView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QHeaderViewC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QHeaderViewC1ERKS_(arg0)} as u64;
    let rsthis = QHeaderView{qbase: QAbstractItemView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHeaderView::setStretchLastSection(bool stretch);
impl /*struct*/ QHeaderView {
  pub fn setStretchLastSection<RetType, T: QHeaderView_setStretchLastSection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStretchLastSection(self);
    // return 1;
  }
}

pub trait QHeaderView_setStretchLastSection<RetType> {
  fn setStretchLastSection(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setStretchLastSection(bool stretch);
impl<'a> /*trait*/ QHeaderView_setStretchLastSection<()> for (i8) {
  fn setStretchLastSection(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setStretchLastSectionEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QHeaderView21setStretchLastSectionEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHeaderView::reset();
impl /*struct*/ QHeaderView {
  pub fn reset<RetType, T: QHeaderView_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QHeaderView_reset<RetType> {
  fn reset(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::reset();
impl<'a> /*trait*/ QHeaderView_reset<()> for () {
  fn reset(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView5resetEv()};
     unsafe {_ZN11QHeaderView5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHeaderView::geometriesChanged();
impl /*struct*/ QHeaderView {
  pub fn geometriesChanged<RetType, T: QHeaderView_geometriesChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.geometriesChanged(self);
    // return 1;
  }
}

pub trait QHeaderView_geometriesChanged<RetType> {
  fn geometriesChanged(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::geometriesChanged();
impl<'a> /*trait*/ QHeaderView_geometriesChanged<()> for () {
  fn geometriesChanged(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView17geometriesChangedEv()};
     unsafe {_ZN11QHeaderView17geometriesChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHeaderView::resetDefaultSectionSize();
impl /*struct*/ QHeaderView {
  pub fn resetDefaultSectionSize<RetType, T: QHeaderView_resetDefaultSectionSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetDefaultSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_resetDefaultSectionSize<RetType> {
  fn resetDefaultSectionSize(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::resetDefaultSectionSize();
impl<'a> /*trait*/ QHeaderView_resetDefaultSectionSize<()> for () {
  fn resetDefaultSectionSize(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView23resetDefaultSectionSizeEv()};
     unsafe {_ZN11QHeaderView23resetDefaultSectionSizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QByteArray QHeaderView::saveState();
impl /*struct*/ QHeaderView {
  pub fn saveState<RetType, T: QHeaderView_saveState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.saveState(self);
    // return 1;
  }
}

pub trait QHeaderView_saveState<RetType> {
  fn saveState(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  QByteArray QHeaderView::saveState();
impl<'a> /*trait*/ QHeaderView_saveState<QByteArray> for () {
  fn saveState(self , rsthis: & QHeaderView) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView9saveStateEv()};
    let mut ret = unsafe {_ZNK11QHeaderView9saveStateEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QHeaderView::sectionsClickable();
impl /*struct*/ QHeaderView {
  pub fn sectionsClickable<RetType, T: QHeaderView_sectionsClickable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionsClickable(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionsClickable<RetType> {
  fn sectionsClickable(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  bool QHeaderView::sectionsClickable();
impl<'a> /*trait*/ QHeaderView_sectionsClickable<i8> for () {
  fn sectionsClickable(self , rsthis: & QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView17sectionsClickableEv()};
    let mut ret = unsafe {_ZNK11QHeaderView17sectionsClickableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QHeaderView::resizeContentsPrecision();
impl /*struct*/ QHeaderView {
  pub fn resizeContentsPrecision<RetType, T: QHeaderView_resizeContentsPrecision<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resizeContentsPrecision(self);
    // return 1;
  }
}

pub trait QHeaderView_resizeContentsPrecision<RetType> {
  fn resizeContentsPrecision(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::resizeContentsPrecision();
impl<'a> /*trait*/ QHeaderView_resizeContentsPrecision<i32> for () {
  fn resizeContentsPrecision(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView23resizeContentsPrecisionEv()};
    let mut ret = unsafe {_ZNK11QHeaderView23resizeContentsPrecisionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHeaderView::setOffsetToSectionPosition(int visualIndex);
impl /*struct*/ QHeaderView {
  pub fn setOffsetToSectionPosition<RetType, T: QHeaderView_setOffsetToSectionPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOffsetToSectionPosition(self);
    // return 1;
  }
}

pub trait QHeaderView_setOffsetToSectionPosition<RetType> {
  fn setOffsetToSectionPosition(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setOffsetToSectionPosition(int visualIndex);
impl<'a> /*trait*/ QHeaderView_setOffsetToSectionPosition<()> for (i32) {
  fn setOffsetToSectionPosition(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView26setOffsetToSectionPositionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView26setOffsetToSectionPositionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QHeaderView::length();
impl /*struct*/ QHeaderView {
  pub fn length<RetType, T: QHeaderView_length<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QHeaderView_length<RetType> {
  fn length(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::length();
impl<'a> /*trait*/ QHeaderView_length<i32> for () {
  fn length(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView6lengthEv()};
    let mut ret = unsafe {_ZNK11QHeaderView6lengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHeaderView::hideSection(int logicalIndex);
impl /*struct*/ QHeaderView {
  pub fn hideSection<RetType, T: QHeaderView_hideSection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hideSection(self);
    // return 1;
  }
}

pub trait QHeaderView_hideSection<RetType> {
  fn hideSection(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::hideSection(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_hideSection<()> for (i32) {
  fn hideSection(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView11hideSectionEi()};
    let arg0 = self  as c_int;
     unsafe {demth_ZN11QHeaderView11hideSectionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QHeaderView::sortIndicatorSection();
impl /*struct*/ QHeaderView {
  pub fn sortIndicatorSection<RetType, T: QHeaderView_sortIndicatorSection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sortIndicatorSection(self);
    // return 1;
  }
}

pub trait QHeaderView_sortIndicatorSection<RetType> {
  fn sortIndicatorSection(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::sortIndicatorSection();
impl<'a> /*trait*/ QHeaderView_sortIndicatorSection<i32> for () {
  fn sortIndicatorSection(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView20sortIndicatorSectionEv()};
    let mut ret = unsafe {_ZNK11QHeaderView20sortIndicatorSectionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QHeaderView::cascadingSectionResizes();
impl /*struct*/ QHeaderView {
  pub fn cascadingSectionResizes<RetType, T: QHeaderView_cascadingSectionResizes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cascadingSectionResizes(self);
    // return 1;
  }
}

pub trait QHeaderView_cascadingSectionResizes<RetType> {
  fn cascadingSectionResizes(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  bool QHeaderView::cascadingSectionResizes();
impl<'a> /*trait*/ QHeaderView_cascadingSectionResizes<i8> for () {
  fn cascadingSectionResizes(self , rsthis: & QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView23cascadingSectionResizesEv()};
    let mut ret = unsafe {_ZNK11QHeaderView23cascadingSectionResizesEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QHeaderView::setMinimumSectionSize(int size);
impl /*struct*/ QHeaderView {
  pub fn setMinimumSectionSize<RetType, T: QHeaderView_setMinimumSectionSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_setMinimumSectionSize<RetType> {
  fn setMinimumSectionSize(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setMinimumSectionSize(int size);
impl<'a> /*trait*/ QHeaderView_setMinimumSectionSize<()> for (i32) {
  fn setMinimumSectionSize(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setMinimumSectionSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView21setMinimumSectionSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QHeaderView::visualIndexAt(int position);
impl /*struct*/ QHeaderView {
  pub fn visualIndexAt<RetType, T: QHeaderView_visualIndexAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visualIndexAt(self);
    // return 1;
  }
}

pub trait QHeaderView_visualIndexAt<RetType> {
  fn visualIndexAt(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::visualIndexAt(int position);
impl<'a> /*trait*/ QHeaderView_visualIndexAt<i32> for (i32) {
  fn visualIndexAt(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView13visualIndexAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView13visualIndexAtEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHeaderView::setOffset(int offset);
impl /*struct*/ QHeaderView {
  pub fn setOffset<RetType, T: QHeaderView_setOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOffset(self);
    // return 1;
  }
}

pub trait QHeaderView_setOffset<RetType> {
  fn setOffset(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setOffset(int offset);
impl<'a> /*trait*/ QHeaderView_setOffset<()> for (i32) {
  fn setOffset(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView9setOffsetEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView9setOffsetEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QHeaderView::logicalIndexAt(const QPoint & pos);
impl /*struct*/ QHeaderView {
  pub fn logicalIndexAt<RetType, T: QHeaderView_logicalIndexAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.logicalIndexAt(self);
    // return 1;
  }
}

pub trait QHeaderView_logicalIndexAt<RetType> {
  fn logicalIndexAt(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::logicalIndexAt(const QPoint & pos);
impl<'a> /*trait*/ QHeaderView_logicalIndexAt<i32> for (&'a QPoint) {
  fn logicalIndexAt(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView14logicalIndexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZNK11QHeaderView14logicalIndexAtERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHeaderView::~QHeaderView();
impl /*struct*/ QHeaderView {
  pub fn Free<RetType, T: QHeaderView_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QHeaderView_Free<RetType> {
  fn Free(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::~QHeaderView();
impl<'a> /*trait*/ QHeaderView_Free<()> for () {
  fn Free(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderViewD0Ev()};
     unsafe {_ZN11QHeaderViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QHeaderView::sectionViewportPosition(int logicalIndex);
impl /*struct*/ QHeaderView {
  pub fn sectionViewportPosition<RetType, T: QHeaderView_sectionViewportPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionViewportPosition(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionViewportPosition<RetType> {
  fn sectionViewportPosition(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::sectionViewportPosition(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionViewportPosition<i32> for (i32) {
  fn sectionViewportPosition(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView23sectionViewportPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView23sectionViewportPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QHeaderView::highlightSections();
impl /*struct*/ QHeaderView {
  pub fn highlightSections<RetType, T: QHeaderView_highlightSections<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.highlightSections(self);
    // return 1;
  }
}

pub trait QHeaderView_highlightSections<RetType> {
  fn highlightSections(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  bool QHeaderView::highlightSections();
impl<'a> /*trait*/ QHeaderView_highlightSections<i8> for () {
  fn highlightSections(self , rsthis: & QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView17highlightSectionsEv()};
    let mut ret = unsafe {_ZNK11QHeaderView17highlightSectionsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QHeaderView::offset();
impl /*struct*/ QHeaderView {
  pub fn offset<RetType, T: QHeaderView_offset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.offset(self);
    // return 1;
  }
}

pub trait QHeaderView_offset<RetType> {
  fn offset(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::offset();
impl<'a> /*trait*/ QHeaderView_offset<i32> for () {
  fn offset(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView6offsetEv()};
    let mut ret = unsafe {_ZNK11QHeaderView6offsetEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHeaderView::setSortIndicatorShown(bool show);
impl /*struct*/ QHeaderView {
  pub fn setSortIndicatorShown<RetType, T: QHeaderView_setSortIndicatorShown<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSortIndicatorShown(self);
    // return 1;
  }
}

pub trait QHeaderView_setSortIndicatorShown<RetType> {
  fn setSortIndicatorShown(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setSortIndicatorShown(bool show);
impl<'a> /*trait*/ QHeaderView_setSortIndicatorShown<()> for (i8) {
  fn setSortIndicatorShown(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setSortIndicatorShownEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QHeaderView21setSortIndicatorShownEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QHeaderView::metaObject();
impl /*struct*/ QHeaderView {
  pub fn metaObject<RetType, T: QHeaderView_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QHeaderView_metaObject<RetType> {
  fn metaObject(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  const QMetaObject * QHeaderView::metaObject();
impl<'a> /*trait*/ QHeaderView_metaObject<()> for () {
  fn metaObject(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView10metaObjectEv()};
     unsafe {_ZNK11QHeaderView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHeaderView::showSection(int logicalIndex);
impl /*struct*/ QHeaderView {
  pub fn showSection<RetType, T: QHeaderView_showSection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showSection(self);
    // return 1;
  }
}

pub trait QHeaderView_showSection<RetType> {
  fn showSection(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::showSection(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_showSection<()> for (i32) {
  fn showSection(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView11showSectionEi()};
    let arg0 = self  as c_int;
     unsafe {demth_ZN11QHeaderView11showSectionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHeaderView::setVisible(bool v);
impl /*struct*/ QHeaderView {
  pub fn setVisible<RetType, T: QHeaderView_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QHeaderView_setVisible<RetType> {
  fn setVisible(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setVisible(bool v);
impl<'a> /*trait*/ QHeaderView_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QHeaderView10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QHeaderView::hiddenSectionCount();
impl /*struct*/ QHeaderView {
  pub fn hiddenSectionCount<RetType, T: QHeaderView_hiddenSectionCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hiddenSectionCount(self);
    // return 1;
  }
}

pub trait QHeaderView_hiddenSectionCount<RetType> {
  fn hiddenSectionCount(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::hiddenSectionCount();
impl<'a> /*trait*/ QHeaderView_hiddenSectionCount<i32> for () {
  fn hiddenSectionCount(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18hiddenSectionCountEv()};
    let mut ret = unsafe {_ZNK11QHeaderView18hiddenSectionCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHeaderView::sectionMoved(int logicalIndex, int oldVisualIndex, int newVisualIndex);
impl /*struct*/ QHeaderView {
  pub fn sectionMoved<RetType, T: QHeaderView_sectionMoved<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionMoved(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionMoved<RetType> {
  fn sectionMoved(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::sectionMoved(int logicalIndex, int oldVisualIndex, int newVisualIndex);
impl<'a> /*trait*/ QHeaderView_sectionMoved<()> for (i32, i32, i32) {
  fn sectionMoved(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView12sectionMovedEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN11QHeaderView12sectionMovedEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QHeaderView::sectionHandleDoubleClicked(int logicalIndex);
impl /*struct*/ QHeaderView {
  pub fn sectionHandleDoubleClicked<RetType, T: QHeaderView_sectionHandleDoubleClicked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionHandleDoubleClicked(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionHandleDoubleClicked<RetType> {
  fn sectionHandleDoubleClicked(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::sectionHandleDoubleClicked(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionHandleDoubleClicked<()> for (i32) {
  fn sectionHandleDoubleClicked(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView26sectionHandleDoubleClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView26sectionHandleDoubleClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHeaderView::setSectionsClickable(bool clickable);
impl /*struct*/ QHeaderView {
  pub fn setSectionsClickable<RetType, T: QHeaderView_setSectionsClickable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSectionsClickable(self);
    // return 1;
  }
}

pub trait QHeaderView_setSectionsClickable<RetType> {
  fn setSectionsClickable(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setSectionsClickable(bool clickable);
impl<'a> /*trait*/ QHeaderView_setSectionsClickable<()> for (i8) {
  fn setSectionsClickable(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView20setSectionsClickableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QHeaderView20setSectionsClickableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHeaderView::sectionPressed(int logicalIndex);
impl /*struct*/ QHeaderView {
  pub fn sectionPressed<RetType, T: QHeaderView_sectionPressed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionPressed(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionPressed<RetType> {
  fn sectionPressed(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::sectionPressed(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionPressed<()> for (i32) {
  fn sectionPressed(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView14sectionPressedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView14sectionPressedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHeaderView::setResizeContentsPrecision(int precision);
impl /*struct*/ QHeaderView {
  pub fn setResizeContentsPrecision<RetType, T: QHeaderView_setResizeContentsPrecision<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setResizeContentsPrecision(self);
    // return 1;
  }
}

pub trait QHeaderView_setResizeContentsPrecision<RetType> {
  fn setResizeContentsPrecision(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setResizeContentsPrecision(int precision);
impl<'a> /*trait*/ QHeaderView_setResizeContentsPrecision<()> for (i32) {
  fn setResizeContentsPrecision(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView26setResizeContentsPrecisionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView26setResizeContentsPrecisionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QHeaderView::defaultSectionSize();
impl /*struct*/ QHeaderView {
  pub fn defaultSectionSize<RetType, T: QHeaderView_defaultSectionSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_defaultSectionSize<RetType> {
  fn defaultSectionSize(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::defaultSectionSize();
impl<'a> /*trait*/ QHeaderView_defaultSectionSize<i32> for () {
  fn defaultSectionSize(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18defaultSectionSizeEv()};
    let mut ret = unsafe {_ZNK11QHeaderView18defaultSectionSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHeaderView::setOffsetToLastSection();
impl /*struct*/ QHeaderView {
  pub fn setOffsetToLastSection<RetType, T: QHeaderView_setOffsetToLastSection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOffsetToLastSection(self);
    // return 1;
  }
}

pub trait QHeaderView_setOffsetToLastSection<RetType> {
  fn setOffsetToLastSection(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setOffsetToLastSection();
impl<'a> /*trait*/ QHeaderView_setOffsetToLastSection<()> for () {
  fn setOffsetToLastSection(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView22setOffsetToLastSectionEv()};
     unsafe {_ZN11QHeaderView22setOffsetToLastSectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHeaderView::swapSections(int first, int second);
impl /*struct*/ QHeaderView {
  pub fn swapSections<RetType, T: QHeaderView_swapSections<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swapSections(self);
    // return 1;
  }
}

pub trait QHeaderView_swapSections<RetType> {
  fn swapSections(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::swapSections(int first, int second);
impl<'a> /*trait*/ QHeaderView_swapSections<()> for (i32, i32) {
  fn swapSections(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView12swapSectionsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QHeaderView12swapSectionsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QHeaderView::count();
impl /*struct*/ QHeaderView {
  pub fn count<RetType, T: QHeaderView_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QHeaderView_count<RetType> {
  fn count(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::count();
impl<'a> /*trait*/ QHeaderView_count<i32> for () {
  fn count(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView5countEv()};
    let mut ret = unsafe {_ZNK11QHeaderView5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QHeaderView::visualIndex(int logicalIndex);
impl /*struct*/ QHeaderView {
  pub fn visualIndex<RetType, T: QHeaderView_visualIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visualIndex(self);
    // return 1;
  }
}

pub trait QHeaderView_visualIndex<RetType> {
  fn visualIndex(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::visualIndex(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_visualIndex<i32> for (i32) {
  fn visualIndex(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView11visualIndexEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView11visualIndexEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHeaderView::sectionClicked(int logicalIndex);
impl /*struct*/ QHeaderView {
  pub fn sectionClicked<RetType, T: QHeaderView_sectionClicked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionClicked(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionClicked<RetType> {
  fn sectionClicked(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::sectionClicked(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionClicked<()> for (i32) {
  fn sectionClicked(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView14sectionClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView14sectionClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QHeaderView::sectionsMoved();
impl /*struct*/ QHeaderView {
  pub fn sectionsMoved<RetType, T: QHeaderView_sectionsMoved<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionsMoved(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionsMoved<RetType> {
  fn sectionsMoved(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  bool QHeaderView::sectionsMoved();
impl<'a> /*trait*/ QHeaderView_sectionsMoved<i8> for () {
  fn sectionsMoved(self , rsthis: & QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView13sectionsMovedEv()};
    let mut ret = unsafe {_ZNK11QHeaderView13sectionsMovedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QHeaderView::stretchSectionCount();
impl /*struct*/ QHeaderView {
  pub fn stretchSectionCount<RetType, T: QHeaderView_stretchSectionCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stretchSectionCount(self);
    // return 1;
  }
}

pub trait QHeaderView_stretchSectionCount<RetType> {
  fn stretchSectionCount(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::stretchSectionCount();
impl<'a> /*trait*/ QHeaderView_stretchSectionCount<i32> for () {
  fn stretchSectionCount(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView19stretchSectionCountEv()};
    let mut ret = unsafe {_ZNK11QHeaderView19stretchSectionCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHeaderView::doItemsLayout();
impl /*struct*/ QHeaderView {
  pub fn doItemsLayout<RetType, T: QHeaderView_doItemsLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doItemsLayout(self);
    // return 1;
  }
}

pub trait QHeaderView_doItemsLayout<RetType> {
  fn doItemsLayout(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::doItemsLayout();
impl<'a> /*trait*/ QHeaderView_doItemsLayout<()> for () {
  fn doItemsLayout(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView13doItemsLayoutEv()};
     unsafe {_ZN11QHeaderView13doItemsLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHeaderView::setSectionsMovable(bool movable);
impl /*struct*/ QHeaderView {
  pub fn setSectionsMovable<RetType, T: QHeaderView_setSectionsMovable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSectionsMovable(self);
    // return 1;
  }
}

pub trait QHeaderView_setSectionsMovable<RetType> {
  fn setSectionsMovable(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setSectionsMovable(bool movable);
impl<'a> /*trait*/ QHeaderView_setSectionsMovable<()> for (i8) {
  fn setSectionsMovable(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView18setSectionsMovableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QHeaderView18setSectionsMovableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QHeaderView::sectionsHidden();
impl /*struct*/ QHeaderView {
  pub fn sectionsHidden<RetType, T: QHeaderView_sectionsHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionsHidden(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionsHidden<RetType> {
  fn sectionsHidden(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  bool QHeaderView::sectionsHidden();
impl<'a> /*trait*/ QHeaderView_sectionsHidden<i8> for () {
  fn sectionsHidden(self , rsthis: & QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView14sectionsHiddenEv()};
    let mut ret = unsafe {_ZNK11QHeaderView14sectionsHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QHeaderView::minimumSectionSize();
impl /*struct*/ QHeaderView {
  pub fn minimumSectionSize<RetType, T: QHeaderView_minimumSectionSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_minimumSectionSize<RetType> {
  fn minimumSectionSize(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::minimumSectionSize();
impl<'a> /*trait*/ QHeaderView_minimumSectionSize<i32> for () {
  fn minimumSectionSize(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18minimumSectionSizeEv()};
    let mut ret = unsafe {_ZNK11QHeaderView18minimumSectionSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHeaderView::setCascadingSectionResizes(bool enable);
impl /*struct*/ QHeaderView {
  pub fn setCascadingSectionResizes<RetType, T: QHeaderView_setCascadingSectionResizes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCascadingSectionResizes(self);
    // return 1;
  }
}

pub trait QHeaderView_setCascadingSectionResizes<RetType> {
  fn setCascadingSectionResizes(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setCascadingSectionResizes(bool enable);
impl<'a> /*trait*/ QHeaderView_setCascadingSectionResizes<()> for (i8) {
  fn setCascadingSectionResizes(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView26setCascadingSectionResizesEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QHeaderView26setCascadingSectionResizesEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHeaderView::setDefaultSectionSize(int size);
impl /*struct*/ QHeaderView {
  pub fn setDefaultSectionSize<RetType, T: QHeaderView_setDefaultSectionSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDefaultSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_setDefaultSectionSize<RetType> {
  fn setDefaultSectionSize(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setDefaultSectionSize(int size);
impl<'a> /*trait*/ QHeaderView_setDefaultSectionSize<()> for (i32) {
  fn setDefaultSectionSize(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setDefaultSectionSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView21setDefaultSectionSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHeaderView::sectionEntered(int logicalIndex);
impl /*struct*/ QHeaderView {
  pub fn sectionEntered<RetType, T: QHeaderView_sectionEntered<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionEntered(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionEntered<RetType> {
  fn sectionEntered(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::sectionEntered(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionEntered<()> for (i32) {
  fn sectionEntered(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView14sectionEnteredEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView14sectionEnteredEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHeaderView::moveSection(int from, int to);
impl /*struct*/ QHeaderView {
  pub fn moveSection<RetType, T: QHeaderView_moveSection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.moveSection(self);
    // return 1;
  }
}

pub trait QHeaderView_moveSection<RetType> {
  fn moveSection(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::moveSection(int from, int to);
impl<'a> /*trait*/ QHeaderView_moveSection<()> for (i32, i32) {
  fn moveSection(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView11moveSectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QHeaderView11moveSectionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QHeaderView::stretchLastSection();
impl /*struct*/ QHeaderView {
  pub fn stretchLastSection<RetType, T: QHeaderView_stretchLastSection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stretchLastSection(self);
    // return 1;
  }
}

pub trait QHeaderView_stretchLastSection<RetType> {
  fn stretchLastSection(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  bool QHeaderView::stretchLastSection();
impl<'a> /*trait*/ QHeaderView_stretchLastSection<i8> for () {
  fn stretchLastSection(self , rsthis: & QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18stretchLastSectionEv()};
    let mut ret = unsafe {_ZNK11QHeaderView18stretchLastSectionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QHeaderView::sectionSizeHint(int logicalIndex);
impl /*struct*/ QHeaderView {
  pub fn sectionSizeHint<RetType, T: QHeaderView_sectionSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionSizeHint(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionSizeHint<RetType> {
  fn sectionSizeHint(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::sectionSizeHint(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionSizeHint<i32> for (i32) {
  fn sectionSizeHint(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView15sectionSizeHintEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView15sectionSizeHintEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QHeaderView::sectionsMovable();
impl /*struct*/ QHeaderView {
  pub fn sectionsMovable<RetType, T: QHeaderView_sectionsMovable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionsMovable(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionsMovable<RetType> {
  fn sectionsMovable(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  bool QHeaderView::sectionsMovable();
impl<'a> /*trait*/ QHeaderView_sectionsMovable<i8> for () {
  fn sectionsMovable(self , rsthis: & QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView15sectionsMovableEv()};
    let mut ret = unsafe {_ZNK11QHeaderView15sectionsMovableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QHeaderView::isSectionHidden(int logicalIndex);
impl /*struct*/ QHeaderView {
  pub fn isSectionHidden<RetType, T: QHeaderView_isSectionHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSectionHidden(self);
    // return 1;
  }
}

pub trait QHeaderView_isSectionHidden<RetType> {
  fn isSectionHidden(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  bool QHeaderView::isSectionHidden(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_isSectionHidden<i8> for (i32) {
  fn isSectionHidden(self , rsthis: & QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView15isSectionHiddenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView15isSectionHiddenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QHeaderView::logicalIndexAt(int x, int y);
impl<'a> /*trait*/ QHeaderView_logicalIndexAt<i32> for (i32, i32) {
  fn logicalIndexAt(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView14logicalIndexAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {demth_ZNK11QHeaderView14logicalIndexAtEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QHeaderView::logicalIndexAt(int position);
impl<'a> /*trait*/ QHeaderView_logicalIndexAt<i32> for (i32) {
  fn logicalIndexAt(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView14logicalIndexAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView14logicalIndexAtEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QHeaderView::logicalIndex(int visualIndex);
impl /*struct*/ QHeaderView {
  pub fn logicalIndex<RetType, T: QHeaderView_logicalIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.logicalIndex(self);
    // return 1;
  }
}

pub trait QHeaderView_logicalIndex<RetType> {
  fn logicalIndex(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  int QHeaderView::logicalIndex(int visualIndex);
impl<'a> /*trait*/ QHeaderView_logicalIndex<i32> for (i32) {
  fn logicalIndex(self , rsthis: & QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView12logicalIndexEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView12logicalIndexEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QHeaderView::setMaximumSectionSize(int size);
impl /*struct*/ QHeaderView {
  pub fn setMaximumSectionSize<RetType, T: QHeaderView_setMaximumSectionSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_setMaximumSectionSize<RetType> {
  fn setMaximumSectionSize(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setMaximumSectionSize(int size);
impl<'a> /*trait*/ QHeaderView_setMaximumSectionSize<()> for (i32) {
  fn setMaximumSectionSize(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setMaximumSectionSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView21setMaximumSectionSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHeaderView::setHighlightSections(bool highlight);
impl /*struct*/ QHeaderView {
  pub fn setHighlightSections<RetType, T: QHeaderView_setHighlightSections<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHighlightSections(self);
    // return 1;
  }
}

pub trait QHeaderView_setHighlightSections<RetType> {
  fn setHighlightSections(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setHighlightSections(bool highlight);
impl<'a> /*trait*/ QHeaderView_setHighlightSections<()> for (i8) {
  fn setHighlightSections(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView20setHighlightSectionsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QHeaderView20setHighlightSectionsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHeaderView::setSectionHidden(int logicalIndex, bool hide);
impl /*struct*/ QHeaderView {
  pub fn setSectionHidden<RetType, T: QHeaderView_setSectionHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSectionHidden(self);
    // return 1;
  }
}

pub trait QHeaderView_setSectionHidden<RetType> {
  fn setSectionHidden(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setSectionHidden(int logicalIndex, bool hide);
impl<'a> /*trait*/ QHeaderView_setSectionHidden<()> for (i32, i8) {
  fn setSectionHidden(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView16setSectionHiddenEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {_ZN11QHeaderView16setSectionHiddenEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QHeaderView::resizeSection(int logicalIndex, int size);
impl /*struct*/ QHeaderView {
  pub fn resizeSection<RetType, T: QHeaderView_resizeSection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resizeSection(self);
    // return 1;
  }
}

pub trait QHeaderView_resizeSection<RetType> {
  fn resizeSection(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::resizeSection(int logicalIndex, int size);
impl<'a> /*trait*/ QHeaderView_resizeSection<()> for (i32, i32) {
  fn resizeSection(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView13resizeSectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QHeaderView13resizeSectionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QHeaderView::restoreState(const QByteArray & state);
impl /*struct*/ QHeaderView {
  pub fn restoreState<RetType, T: QHeaderView_restoreState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.restoreState(self);
    // return 1;
  }
}

pub trait QHeaderView_restoreState<RetType> {
  fn restoreState(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  bool QHeaderView::restoreState(const QByteArray & state);
impl<'a> /*trait*/ QHeaderView_restoreState<i8> for (&'a QByteArray) {
  fn restoreState(self , rsthis: & QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView12restoreStateERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QHeaderView12restoreStateERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QHeaderView::sectionDoubleClicked(int logicalIndex);
impl /*struct*/ QHeaderView {
  pub fn sectionDoubleClicked<RetType, T: QHeaderView_sectionDoubleClicked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionDoubleClicked(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionDoubleClicked<RetType> {
  fn sectionDoubleClicked(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::sectionDoubleClicked(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionDoubleClicked<()> for (i32) {
  fn sectionDoubleClicked(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView20sectionDoubleClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView20sectionDoubleClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHeaderView::sectionCountChanged(int oldCount, int newCount);
impl /*struct*/ QHeaderView {
  pub fn sectionCountChanged<RetType, T: QHeaderView_sectionCountChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionCountChanged(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionCountChanged<RetType> {
  fn sectionCountChanged(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::sectionCountChanged(int oldCount, int newCount);
impl<'a> /*trait*/ QHeaderView_sectionCountChanged<()> for (i32, i32) {
  fn sectionCountChanged(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView19sectionCountChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QHeaderView19sectionCountChangedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QHeaderView::setModel(QAbstractItemModel * model);
impl /*struct*/ QHeaderView {
  pub fn setModel<RetType, T: QHeaderView_setModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModel(self);
    // return 1;
  }
}

pub trait QHeaderView_setModel<RetType> {
  fn setModel(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  void QHeaderView::setModel(QAbstractItemModel * model);
impl<'a> /*trait*/ QHeaderView_setModel<()> for (&'a QAbstractItemModel) {
  fn setModel(self , rsthis: & QHeaderView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView8setModelEP18QAbstractItemModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QHeaderView8setModelEP18QAbstractItemModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QHeaderView::isSortIndicatorShown();
impl /*struct*/ QHeaderView {
  pub fn isSortIndicatorShown<RetType, T: QHeaderView_isSortIndicatorShown<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSortIndicatorShown(self);
    // return 1;
  }
}

pub trait QHeaderView_isSortIndicatorShown<RetType> {
  fn isSortIndicatorShown(self , rsthis: & QHeaderView) -> RetType;
}

  // proto:  bool QHeaderView::isSortIndicatorShown();
impl<'a> /*trait*/ QHeaderView_isSortIndicatorShown<i8> for () {
  fn isSortIndicatorShown(self , rsthis: & QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView20isSortIndicatorShownEv()};
    let mut ret = unsafe {_ZNK11QHeaderView20isSortIndicatorShownEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

#[derive(Default)] // for QHeaderView_sectionHandleDoubleClicked
pub struct QHeaderView_sectionHandleDoubleClicked_signal{poi:u64}
impl /* struct */ QHeaderView {
  pub fn sectionHandleDoubleClicked_1(self) -> QHeaderView_sectionHandleDoubleClicked_signal {
     return QHeaderView_sectionHandleDoubleClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QHeaderView_sectionHandleDoubleClicked_signal {
  pub fn connect<T: QHeaderView_sectionHandleDoubleClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QHeaderView_sectionHandleDoubleClicked_signal_connect {
  fn connect(self, sigthis: QHeaderView_sectionHandleDoubleClicked_signal);
}

#[derive(Default)] // for QHeaderView_sectionEntered
pub struct QHeaderView_sectionEntered_signal{poi:u64}
impl /* struct */ QHeaderView {
  pub fn sectionEntered_1(self) -> QHeaderView_sectionEntered_signal {
     return QHeaderView_sectionEntered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QHeaderView_sectionEntered_signal {
  pub fn connect<T: QHeaderView_sectionEntered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QHeaderView_sectionEntered_signal_connect {
  fn connect(self, sigthis: QHeaderView_sectionEntered_signal);
}

#[derive(Default)] // for QHeaderView_sortIndicatorChanged
pub struct QHeaderView_sortIndicatorChanged_signal{poi:u64}
impl /* struct */ QHeaderView {
  pub fn sortIndicatorChanged_1(self) -> QHeaderView_sortIndicatorChanged_signal {
     return QHeaderView_sortIndicatorChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QHeaderView_sortIndicatorChanged_signal {
  pub fn connect<T: QHeaderView_sortIndicatorChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QHeaderView_sortIndicatorChanged_signal_connect {
  fn connect(self, sigthis: QHeaderView_sortIndicatorChanged_signal);
}

#[derive(Default)] // for QHeaderView_sectionClicked
pub struct QHeaderView_sectionClicked_signal{poi:u64}
impl /* struct */ QHeaderView {
  pub fn sectionClicked_1(self) -> QHeaderView_sectionClicked_signal {
     return QHeaderView_sectionClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QHeaderView_sectionClicked_signal {
  pub fn connect<T: QHeaderView_sectionClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QHeaderView_sectionClicked_signal_connect {
  fn connect(self, sigthis: QHeaderView_sectionClicked_signal);
}

#[derive(Default)] // for QHeaderView_sectionCountChanged
pub struct QHeaderView_sectionCountChanged_signal{poi:u64}
impl /* struct */ QHeaderView {
  pub fn sectionCountChanged_1(self) -> QHeaderView_sectionCountChanged_signal {
     return QHeaderView_sectionCountChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QHeaderView_sectionCountChanged_signal {
  pub fn connect<T: QHeaderView_sectionCountChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QHeaderView_sectionCountChanged_signal_connect {
  fn connect(self, sigthis: QHeaderView_sectionCountChanged_signal);
}

#[derive(Default)] // for QHeaderView_geometriesChanged
pub struct QHeaderView_geometriesChanged_signal{poi:u64}
impl /* struct */ QHeaderView {
  pub fn geometriesChanged_1(self) -> QHeaderView_geometriesChanged_signal {
     return QHeaderView_geometriesChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QHeaderView_geometriesChanged_signal {
  pub fn connect<T: QHeaderView_geometriesChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QHeaderView_geometriesChanged_signal_connect {
  fn connect(self, sigthis: QHeaderView_geometriesChanged_signal);
}

#[derive(Default)] // for QHeaderView_sectionMoved
pub struct QHeaderView_sectionMoved_signal{poi:u64}
impl /* struct */ QHeaderView {
  pub fn sectionMoved_1(self) -> QHeaderView_sectionMoved_signal {
     return QHeaderView_sectionMoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QHeaderView_sectionMoved_signal {
  pub fn connect<T: QHeaderView_sectionMoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QHeaderView_sectionMoved_signal_connect {
  fn connect(self, sigthis: QHeaderView_sectionMoved_signal);
}

#[derive(Default)] // for QHeaderView_sectionPressed
pub struct QHeaderView_sectionPressed_signal{poi:u64}
impl /* struct */ QHeaderView {
  pub fn sectionPressed_1(self) -> QHeaderView_sectionPressed_signal {
     return QHeaderView_sectionPressed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QHeaderView_sectionPressed_signal {
  pub fn connect<T: QHeaderView_sectionPressed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QHeaderView_sectionPressed_signal_connect {
  fn connect(self, sigthis: QHeaderView_sectionPressed_signal);
}

#[derive(Default)] // for QHeaderView_sectionDoubleClicked
pub struct QHeaderView_sectionDoubleClicked_signal{poi:u64}
impl /* struct */ QHeaderView {
  pub fn sectionDoubleClicked_1(self) -> QHeaderView_sectionDoubleClicked_signal {
     return QHeaderView_sectionDoubleClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QHeaderView_sectionDoubleClicked_signal {
  pub fn connect<T: QHeaderView_sectionDoubleClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QHeaderView_sectionDoubleClicked_signal_connect {
  fn connect(self, sigthis: QHeaderView_sectionDoubleClicked_signal);
}

#[derive(Default)] // for QHeaderView_sectionResized
pub struct QHeaderView_sectionResized_signal{poi:u64}
impl /* struct */ QHeaderView {
  pub fn sectionResized_1(self) -> QHeaderView_sectionResized_signal {
     return QHeaderView_sectionResized_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QHeaderView_sectionResized_signal {
  pub fn connect<T: QHeaderView_sectionResized_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QHeaderView_sectionResized_signal_connect {
  fn connect(self, sigthis: QHeaderView_sectionResized_signal);
}

// sectionEntered(int)
extern fn QHeaderView_sectionEntered_signal_connect_cb_0(arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QHeaderView_sectionEntered_signal_connect for (extern fn(i32)) {
  fn connect(self, sigthis: QHeaderView_sectionEntered_signal) {
    // do smth...
    unsafe {QHeaderView_SlotProxy_connect__ZN11QHeaderView14sectionEnteredEi(sigthis.poi as *mut c_void, QHeaderView_sectionEntered_signal_connect_cb_0 as *mut c_void)};
  }
}
// geometriesChanged()
extern fn QHeaderView_geometriesChanged_signal_connect_cb_1() {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QHeaderView_geometriesChanged_signal_connect for (extern fn()) {
  fn connect(self, sigthis: QHeaderView_geometriesChanged_signal) {
    // do smth...
    unsafe {QHeaderView_SlotProxy_connect__ZN11QHeaderView17geometriesChangedEv(sigthis.poi as *mut c_void, QHeaderView_geometriesChanged_signal_connect_cb_1 as *mut c_void)};
  }
}
// sectionClicked(int)
extern fn QHeaderView_sectionClicked_signal_connect_cb_2(arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QHeaderView_sectionClicked_signal_connect for (extern fn(i32)) {
  fn connect(self, sigthis: QHeaderView_sectionClicked_signal) {
    // do smth...
    unsafe {QHeaderView_SlotProxy_connect__ZN11QHeaderView14sectionClickedEi(sigthis.poi as *mut c_void, QHeaderView_sectionClicked_signal_connect_cb_2 as *mut c_void)};
  }
}
// sectionMoved(int, int, int)
extern fn QHeaderView_sectionMoved_signal_connect_cb_3(arg0: c_int, arg1: c_int, arg2: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QHeaderView_sectionMoved_signal_connect for (extern fn(i32, i32, i32)) {
  fn connect(self, sigthis: QHeaderView_sectionMoved_signal) {
    // do smth...
    unsafe {QHeaderView_SlotProxy_connect__ZN11QHeaderView12sectionMovedEiii(sigthis.poi as *mut c_void, QHeaderView_sectionMoved_signal_connect_cb_3 as *mut c_void)};
  }
}
// sectionPressed(int)
extern fn QHeaderView_sectionPressed_signal_connect_cb_4(arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QHeaderView_sectionPressed_signal_connect for (extern fn(i32)) {
  fn connect(self, sigthis: QHeaderView_sectionPressed_signal) {
    // do smth...
    unsafe {QHeaderView_SlotProxy_connect__ZN11QHeaderView14sectionPressedEi(sigthis.poi as *mut c_void, QHeaderView_sectionPressed_signal_connect_cb_4 as *mut c_void)};
  }
}
// sectionDoubleClicked(int)
extern fn QHeaderView_sectionDoubleClicked_signal_connect_cb_5(arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QHeaderView_sectionDoubleClicked_signal_connect for (extern fn(i32)) {
  fn connect(self, sigthis: QHeaderView_sectionDoubleClicked_signal) {
    // do smth...
    unsafe {QHeaderView_SlotProxy_connect__ZN11QHeaderView20sectionDoubleClickedEi(sigthis.poi as *mut c_void, QHeaderView_sectionDoubleClicked_signal_connect_cb_5 as *mut c_void)};
  }
}
// sectionHandleDoubleClicked(int)
extern fn QHeaderView_sectionHandleDoubleClicked_signal_connect_cb_6(arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QHeaderView_sectionHandleDoubleClicked_signal_connect for (extern fn(i32)) {
  fn connect(self, sigthis: QHeaderView_sectionHandleDoubleClicked_signal) {
    // do smth...
    unsafe {QHeaderView_SlotProxy_connect__ZN11QHeaderView26sectionHandleDoubleClickedEi(sigthis.poi as *mut c_void, QHeaderView_sectionHandleDoubleClicked_signal_connect_cb_6 as *mut c_void)};
  }
}
// sectionResized(int, int, int)
extern fn QHeaderView_sectionResized_signal_connect_cb_7(arg0: c_int, arg1: c_int, arg2: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QHeaderView_sectionResized_signal_connect for (extern fn(i32, i32, i32)) {
  fn connect(self, sigthis: QHeaderView_sectionResized_signal) {
    // do smth...
    unsafe {QHeaderView_SlotProxy_connect__ZN11QHeaderView14sectionResizedEiii(sigthis.poi as *mut c_void, QHeaderView_sectionResized_signal_connect_cb_7 as *mut c_void)};
  }
}
// sectionCountChanged(int, int)
extern fn QHeaderView_sectionCountChanged_signal_connect_cb_8(arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QHeaderView_sectionCountChanged_signal_connect for (extern fn(i32, i32)) {
  fn connect(self, sigthis: QHeaderView_sectionCountChanged_signal) {
    // do smth...
    unsafe {QHeaderView_SlotProxy_connect__ZN11QHeaderView19sectionCountChangedEii(sigthis.poi as *mut c_void, QHeaderView_sectionCountChanged_signal_connect_cb_8 as *mut c_void)};
  }
}
// sortIndicatorChanged(int, Qt::SortOrder)
extern fn QHeaderView_sortIndicatorChanged_signal_connect_cb_9(arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QHeaderView_sortIndicatorChanged_signal_connect for (extern fn(i32, i32)) {
  fn connect(self, sigthis: QHeaderView_sortIndicatorChanged_signal) {
    // do smth...
    unsafe {QHeaderView_SlotProxy_connect__ZN11QHeaderView20sortIndicatorChangedEiN2Qt9SortOrderE(sigthis.poi as *mut c_void, QHeaderView_sortIndicatorChanged_signal_connect_cb_9 as *mut c_void)};
  }
}
// <= body block end

