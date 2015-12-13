// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qbytearray::QByteArray;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QHeaderView::maximumSectionSize();
  fn _ZNK11QHeaderView18maximumSectionSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QHeaderView::sizeHint();
  fn _ZNK11QHeaderView8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QHeaderView::sectionPosition(int logicalIndex);
  fn _ZNK11QHeaderView15sectionPositionEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QHeaderView::sectionResized(int logicalIndex, int oldSize, int newSize);
  fn _ZN11QHeaderView14sectionResizedEiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int) ;
  // proto:  int QHeaderView::sectionSize(int logicalIndex);
  fn _ZNK11QHeaderView11sectionSizeEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QHeaderView::NewQHeaderView(const QHeaderView & );
  fn _ZN11QHeaderViewC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QHeaderView::setStretchLastSection(bool stretch);
  fn _ZN11QHeaderView21setStretchLastSectionEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QHeaderView::reset();
  fn _ZN11QHeaderView5resetEv(qthis: *mut c_void) ;
  // proto:  void QHeaderView::geometriesChanged();
  fn _ZN11QHeaderView17geometriesChangedEv(qthis: *mut c_void) ;
  // proto:  void QHeaderView::resetDefaultSectionSize();
  fn _ZN11QHeaderView23resetDefaultSectionSizeEv(qthis: *mut c_void) ;
  // proto:  QByteArray QHeaderView::saveState();
  fn _ZNK11QHeaderView9saveStateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QHeaderView::sectionsClickable();
  fn _ZNK11QHeaderView17sectionsClickableEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QHeaderView::resizeContentsPrecision();
  fn _ZNK11QHeaderView23resizeContentsPrecisionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QHeaderView::setOffsetToSectionPosition(int visualIndex);
  fn _ZN11QHeaderView26setOffsetToSectionPositionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QHeaderView::length();
  fn _ZNK11QHeaderView6lengthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QHeaderView::hideSection(int logicalIndex);
  fn _ZN11QHeaderView11hideSectionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QHeaderView::sortIndicatorSection();
  fn _ZNK11QHeaderView20sortIndicatorSectionEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QHeaderView::cascadingSectionResizes();
  fn _ZNK11QHeaderView23cascadingSectionResizesEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QHeaderView::setMinimumSectionSize(int size);
  fn _ZN11QHeaderView21setMinimumSectionSizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QHeaderView::visualIndexAt(int position);
  fn _ZNK11QHeaderView13visualIndexAtEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QHeaderView::setOffset(int offset);
  fn _ZN11QHeaderView9setOffsetEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QHeaderView::logicalIndexAt(const QPoint & pos);
  fn _ZNK11QHeaderView14logicalIndexAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QHeaderView::FreeQHeaderView();
  fn _ZN11QHeaderViewD0Ev(qthis: *mut c_void) ;
  // proto:  int QHeaderView::sectionViewportPosition(int logicalIndex);
  fn _ZNK11QHeaderView23sectionViewportPositionEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  bool QHeaderView::highlightSections();
  fn _ZNK11QHeaderView17highlightSectionsEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QHeaderView::offset();
  fn _ZNK11QHeaderView6offsetEv(qthis: *mut c_void) -> c_int;
  // proto:  void QHeaderView::setSortIndicatorShown(bool show);
  fn _ZN11QHeaderView21setSortIndicatorShownEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  const QMetaObject * QHeaderView::metaObject();
  fn _ZNK11QHeaderView10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QHeaderView::showSection(int logicalIndex);
  fn _ZN11QHeaderView11showSectionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QHeaderView::setVisible(bool v);
  fn _ZN11QHeaderView10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QHeaderView::hiddenSectionCount();
  fn _ZNK11QHeaderView18hiddenSectionCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QHeaderView::sectionMoved(int logicalIndex, int oldVisualIndex, int newVisualIndex);
  fn _ZN11QHeaderView12sectionMovedEiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int) ;
  // proto:  void QHeaderView::sectionHandleDoubleClicked(int logicalIndex);
  fn _ZN11QHeaderView26sectionHandleDoubleClickedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QHeaderView::setSectionsClickable(bool clickable);
  fn _ZN11QHeaderView20setSectionsClickableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QHeaderView::sectionPressed(int logicalIndex);
  fn _ZN11QHeaderView14sectionPressedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QHeaderView::setResizeContentsPrecision(int precision);
  fn _ZN11QHeaderView26setResizeContentsPrecisionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QHeaderView::defaultSectionSize();
  fn _ZNK11QHeaderView18defaultSectionSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QHeaderView::setOffsetToLastSection();
  fn _ZN11QHeaderView22setOffsetToLastSectionEv(qthis: *mut c_void) ;
  // proto:  void QHeaderView::swapSections(int first, int second);
  fn _ZN11QHeaderView12swapSectionsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  int QHeaderView::count();
  fn _ZNK11QHeaderView5countEv(qthis: *mut c_void) -> c_int;
  // proto:  int QHeaderView::visualIndex(int logicalIndex);
  fn _ZNK11QHeaderView11visualIndexEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QHeaderView::sectionClicked(int logicalIndex);
  fn _ZN11QHeaderView14sectionClickedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QHeaderView::sectionsMoved();
  fn _ZNK11QHeaderView13sectionsMovedEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QHeaderView::stretchSectionCount();
  fn _ZNK11QHeaderView19stretchSectionCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QHeaderView::doItemsLayout();
  fn _ZN11QHeaderView13doItemsLayoutEv(qthis: *mut c_void) ;
  // proto:  void QHeaderView::setSectionsMovable(bool movable);
  fn _ZN11QHeaderView18setSectionsMovableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QHeaderView::sectionsHidden();
  fn _ZNK11QHeaderView14sectionsHiddenEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QHeaderView::minimumSectionSize();
  fn _ZNK11QHeaderView18minimumSectionSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QHeaderView::setCascadingSectionResizes(bool enable);
  fn _ZN11QHeaderView26setCascadingSectionResizesEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QHeaderView::setDefaultSectionSize(int size);
  fn _ZN11QHeaderView21setDefaultSectionSizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QHeaderView::sectionEntered(int logicalIndex);
  fn _ZN11QHeaderView14sectionEnteredEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QHeaderView::moveSection(int from, int to);
  fn _ZN11QHeaderView11moveSectionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  bool QHeaderView::stretchLastSection();
  fn _ZNK11QHeaderView18stretchLastSectionEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QHeaderView::sectionSizeHint(int logicalIndex);
  fn _ZNK11QHeaderView15sectionSizeHintEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  bool QHeaderView::sectionsMovable();
  fn _ZNK11QHeaderView15sectionsMovableEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QHeaderView::isSectionHidden(int logicalIndex);
  fn _ZNK11QHeaderView15isSectionHiddenEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  int QHeaderView::logicalIndexAt(int x, int y);
  fn _ZNK11QHeaderView14logicalIndexAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> c_int;
  // proto:  int QHeaderView::logicalIndexAt(int position);
  fn _ZNK11QHeaderView14logicalIndexAtEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QHeaderView::logicalIndex(int visualIndex);
  fn _ZNK11QHeaderView12logicalIndexEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QHeaderView::setMaximumSectionSize(int size);
  fn _ZN11QHeaderView21setMaximumSectionSizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QHeaderView::setHighlightSections(bool highlight);
  fn _ZN11QHeaderView20setHighlightSectionsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QHeaderView::setSectionHidden(int logicalIndex, bool hide);
  fn _ZN11QHeaderView16setSectionHiddenEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  void QHeaderView::resizeSection(int logicalIndex, int size);
  fn _ZN11QHeaderView13resizeSectionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  bool QHeaderView::restoreState(const QByteArray & state);
  fn _ZN11QHeaderView12restoreStateERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QHeaderView::sectionDoubleClicked(int logicalIndex);
  fn _ZN11QHeaderView20sectionDoubleClickedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QHeaderView::sectionCountChanged(int oldCount, int newCount);
  fn _ZN11QHeaderView19sectionCountChangedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  bool QHeaderView::isSortIndicatorShown();
  fn _ZNK11QHeaderView20isSortIndicatorShownEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QHeaderView)=1
pub struct QHeaderView {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QHeaderView {
  pub fn maximumSectionSize<T: QHeaderView_maximumSectionSize>(&mut self, value: T) -> i32 {
    return value.maximumSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_maximumSectionSize {
  fn maximumSectionSize(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::maximumSectionSize();
impl<'a> /*trait*/ QHeaderView_maximumSectionSize for () {
  fn maximumSectionSize(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18maximumSectionSizeEv()};
    let mut ret = unsafe {_ZNK11QHeaderView18maximumSectionSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sizeHint<T: QHeaderView_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QHeaderView_sizeHint {
  fn sizeHint(self, rsthis: &mut QHeaderView) -> QSize;
}

// proto:  QSize QHeaderView::sizeHint();
impl<'a> /*trait*/ QHeaderView_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QHeaderView) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QHeaderView8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionPosition<T: QHeaderView_sectionPosition>(&mut self, value: T) -> i32 {
    return value.sectionPosition(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionPosition {
  fn sectionPosition(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::sectionPosition(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionPosition for (i32) {
  fn sectionPosition(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView15sectionPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView15sectionPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionResized<T: QHeaderView_sectionResized>(&mut self, value: T)  {
     value.sectionResized(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionResized {
  fn sectionResized(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::sectionResized(int logicalIndex, int oldSize, int newSize);
impl<'a> /*trait*/ QHeaderView_sectionResized for (i32, i32, i32) {
  fn sectionResized(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView14sectionResizedEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN11QHeaderView14sectionResizedEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionSize<T: QHeaderView_sectionSize>(&mut self, value: T) -> i32 {
    return value.sectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionSize {
  fn sectionSize(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::sectionSize(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionSize for (i32) {
  fn sectionSize(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView11sectionSizeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView11sectionSizeEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn NewQHeaderView<T: QHeaderView_NewQHeaderView>(value: T) -> QHeaderView {
    let rsthis = value.NewQHeaderView();
    return rsthis;
    // return 1;
  }
}

pub trait QHeaderView_NewQHeaderView {
  fn NewQHeaderView(self) -> QHeaderView;
}

// proto: void QHeaderView::NewQHeaderView(const QHeaderView & );
impl<'a> /*trait*/ QHeaderView_NewQHeaderView for (&'a  QHeaderView) {
  fn NewQHeaderView(self) -> QHeaderView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderViewC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QHeaderViewC1ERKS_(qthis, arg0)};
    let rsthis = QHeaderView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setStretchLastSection<T: QHeaderView_setStretchLastSection>(&mut self, value: T)  {
     value.setStretchLastSection(self);
    // return 1;
  }
}

pub trait QHeaderView_setStretchLastSection {
  fn setStretchLastSection(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setStretchLastSection(bool stretch);
impl<'a> /*trait*/ QHeaderView_setStretchLastSection for (i8) {
  fn setStretchLastSection(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setStretchLastSectionEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QHeaderView21setStretchLastSectionEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn reset<T: QHeaderView_reset>(&mut self, value: T)  {
     value.reset(self);
    // return 1;
  }
}

pub trait QHeaderView_reset {
  fn reset(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::reset();
impl<'a> /*trait*/ QHeaderView_reset for () {
  fn reset(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView5resetEv()};
     unsafe {_ZN11QHeaderView5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn geometriesChanged<T: QHeaderView_geometriesChanged>(&mut self, value: T)  {
     value.geometriesChanged(self);
    // return 1;
  }
}

pub trait QHeaderView_geometriesChanged {
  fn geometriesChanged(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::geometriesChanged();
impl<'a> /*trait*/ QHeaderView_geometriesChanged for () {
  fn geometriesChanged(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView17geometriesChangedEv()};
     unsafe {_ZN11QHeaderView17geometriesChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn resetDefaultSectionSize<T: QHeaderView_resetDefaultSectionSize>(&mut self, value: T)  {
     value.resetDefaultSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_resetDefaultSectionSize {
  fn resetDefaultSectionSize(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::resetDefaultSectionSize();
impl<'a> /*trait*/ QHeaderView_resetDefaultSectionSize for () {
  fn resetDefaultSectionSize(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView23resetDefaultSectionSizeEv()};
     unsafe {_ZN11QHeaderView23resetDefaultSectionSizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn saveState<T: QHeaderView_saveState>(&mut self, value: T) -> QByteArray {
    return value.saveState(self);
    // return 1;
  }
}

pub trait QHeaderView_saveState {
  fn saveState(self, rsthis: &mut QHeaderView) -> QByteArray;
}

// proto:  QByteArray QHeaderView::saveState();
impl<'a> /*trait*/ QHeaderView_saveState for () {
  fn saveState(self, rsthis: &mut QHeaderView) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView9saveStateEv()};
    let mut ret = unsafe {_ZNK11QHeaderView9saveStateEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionsClickable<T: QHeaderView_sectionsClickable>(&mut self, value: T) -> i8 {
    return value.sectionsClickable(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionsClickable {
  fn sectionsClickable(self, rsthis: &mut QHeaderView) -> i8;
}

// proto:  bool QHeaderView::sectionsClickable();
impl<'a> /*trait*/ QHeaderView_sectionsClickable for () {
  fn sectionsClickable(self, rsthis: &mut QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView17sectionsClickableEv()};
    let mut ret = unsafe {_ZNK11QHeaderView17sectionsClickableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn resizeContentsPrecision<T: QHeaderView_resizeContentsPrecision>(&mut self, value: T) -> i32 {
    return value.resizeContentsPrecision(self);
    // return 1;
  }
}

pub trait QHeaderView_resizeContentsPrecision {
  fn resizeContentsPrecision(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::resizeContentsPrecision();
impl<'a> /*trait*/ QHeaderView_resizeContentsPrecision for () {
  fn resizeContentsPrecision(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView23resizeContentsPrecisionEv()};
    let mut ret = unsafe {_ZNK11QHeaderView23resizeContentsPrecisionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setOffsetToSectionPosition<T: QHeaderView_setOffsetToSectionPosition>(&mut self, value: T)  {
     value.setOffsetToSectionPosition(self);
    // return 1;
  }
}

pub trait QHeaderView_setOffsetToSectionPosition {
  fn setOffsetToSectionPosition(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setOffsetToSectionPosition(int visualIndex);
impl<'a> /*trait*/ QHeaderView_setOffsetToSectionPosition for (i32) {
  fn setOffsetToSectionPosition(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView26setOffsetToSectionPositionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView26setOffsetToSectionPositionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn length<T: QHeaderView_length>(&mut self, value: T) -> i32 {
    return value.length(self);
    // return 1;
  }
}

pub trait QHeaderView_length {
  fn length(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::length();
impl<'a> /*trait*/ QHeaderView_length for () {
  fn length(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView6lengthEv()};
    let mut ret = unsafe {_ZNK11QHeaderView6lengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn hideSection<T: QHeaderView_hideSection>(&mut self, value: T)  {
     value.hideSection(self);
    // return 1;
  }
}

pub trait QHeaderView_hideSection {
  fn hideSection(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::hideSection(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_hideSection for (i32) {
  fn hideSection(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView11hideSectionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView11hideSectionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sortIndicatorSection<T: QHeaderView_sortIndicatorSection>(&mut self, value: T) -> i32 {
    return value.sortIndicatorSection(self);
    // return 1;
  }
}

pub trait QHeaderView_sortIndicatorSection {
  fn sortIndicatorSection(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::sortIndicatorSection();
impl<'a> /*trait*/ QHeaderView_sortIndicatorSection for () {
  fn sortIndicatorSection(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView20sortIndicatorSectionEv()};
    let mut ret = unsafe {_ZNK11QHeaderView20sortIndicatorSectionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn cascadingSectionResizes<T: QHeaderView_cascadingSectionResizes>(&mut self, value: T) -> i8 {
    return value.cascadingSectionResizes(self);
    // return 1;
  }
}

pub trait QHeaderView_cascadingSectionResizes {
  fn cascadingSectionResizes(self, rsthis: &mut QHeaderView) -> i8;
}

// proto:  bool QHeaderView::cascadingSectionResizes();
impl<'a> /*trait*/ QHeaderView_cascadingSectionResizes for () {
  fn cascadingSectionResizes(self, rsthis: &mut QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView23cascadingSectionResizesEv()};
    let mut ret = unsafe {_ZNK11QHeaderView23cascadingSectionResizesEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setMinimumSectionSize<T: QHeaderView_setMinimumSectionSize>(&mut self, value: T)  {
     value.setMinimumSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_setMinimumSectionSize {
  fn setMinimumSectionSize(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setMinimumSectionSize(int size);
impl<'a> /*trait*/ QHeaderView_setMinimumSectionSize for (i32) {
  fn setMinimumSectionSize(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setMinimumSectionSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView21setMinimumSectionSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn visualIndexAt<T: QHeaderView_visualIndexAt>(&mut self, value: T) -> i32 {
    return value.visualIndexAt(self);
    // return 1;
  }
}

pub trait QHeaderView_visualIndexAt {
  fn visualIndexAt(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::visualIndexAt(int position);
impl<'a> /*trait*/ QHeaderView_visualIndexAt for (i32) {
  fn visualIndexAt(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView13visualIndexAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView13visualIndexAtEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setOffset<T: QHeaderView_setOffset>(&mut self, value: T)  {
     value.setOffset(self);
    // return 1;
  }
}

pub trait QHeaderView_setOffset {
  fn setOffset(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setOffset(int offset);
impl<'a> /*trait*/ QHeaderView_setOffset for (i32) {
  fn setOffset(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView9setOffsetEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView9setOffsetEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn logicalIndexAt<T: QHeaderView_logicalIndexAt>(&mut self, value: T) -> i32 {
    return value.logicalIndexAt(self);
    // return 1;
  }
}

pub trait QHeaderView_logicalIndexAt {
  fn logicalIndexAt(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::logicalIndexAt(const QPoint & pos);
impl<'a> /*trait*/ QHeaderView_logicalIndexAt for (&'a  QPoint) {
  fn logicalIndexAt(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView14logicalIndexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QHeaderView14logicalIndexAtERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn FreeQHeaderView<T: QHeaderView_FreeQHeaderView>(&mut self, value: T)  {
     value.FreeQHeaderView(self);
    // return 1;
  }
}

pub trait QHeaderView_FreeQHeaderView {
  fn FreeQHeaderView(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::FreeQHeaderView();
impl<'a> /*trait*/ QHeaderView_FreeQHeaderView for () {
  fn FreeQHeaderView(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderViewD0Ev()};
     unsafe {_ZN11QHeaderViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionViewportPosition<T: QHeaderView_sectionViewportPosition>(&mut self, value: T) -> i32 {
    return value.sectionViewportPosition(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionViewportPosition {
  fn sectionViewportPosition(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::sectionViewportPosition(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionViewportPosition for (i32) {
  fn sectionViewportPosition(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView23sectionViewportPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView23sectionViewportPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn highlightSections<T: QHeaderView_highlightSections>(&mut self, value: T) -> i8 {
    return value.highlightSections(self);
    // return 1;
  }
}

pub trait QHeaderView_highlightSections {
  fn highlightSections(self, rsthis: &mut QHeaderView) -> i8;
}

// proto:  bool QHeaderView::highlightSections();
impl<'a> /*trait*/ QHeaderView_highlightSections for () {
  fn highlightSections(self, rsthis: &mut QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView17highlightSectionsEv()};
    let mut ret = unsafe {_ZNK11QHeaderView17highlightSectionsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn offset<T: QHeaderView_offset>(&mut self, value: T) -> i32 {
    return value.offset(self);
    // return 1;
  }
}

pub trait QHeaderView_offset {
  fn offset(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::offset();
impl<'a> /*trait*/ QHeaderView_offset for () {
  fn offset(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView6offsetEv()};
    let mut ret = unsafe {_ZNK11QHeaderView6offsetEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setSortIndicatorShown<T: QHeaderView_setSortIndicatorShown>(&mut self, value: T)  {
     value.setSortIndicatorShown(self);
    // return 1;
  }
}

pub trait QHeaderView_setSortIndicatorShown {
  fn setSortIndicatorShown(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setSortIndicatorShown(bool show);
impl<'a> /*trait*/ QHeaderView_setSortIndicatorShown for (i8) {
  fn setSortIndicatorShown(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setSortIndicatorShownEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QHeaderView21setSortIndicatorShownEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn metaObject<T: QHeaderView_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QHeaderView_metaObject {
  fn metaObject(self, rsthis: &mut QHeaderView) ;
}

// proto:  const QMetaObject * QHeaderView::metaObject();
impl<'a> /*trait*/ QHeaderView_metaObject for () {
  fn metaObject(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView10metaObjectEv()};
     unsafe {_ZNK11QHeaderView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn showSection<T: QHeaderView_showSection>(&mut self, value: T)  {
     value.showSection(self);
    // return 1;
  }
}

pub trait QHeaderView_showSection {
  fn showSection(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::showSection(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_showSection for (i32) {
  fn showSection(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView11showSectionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView11showSectionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setVisible<T: QHeaderView_setVisible>(&mut self, value: T)  {
     value.setVisible(self);
    // return 1;
  }
}

pub trait QHeaderView_setVisible {
  fn setVisible(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setVisible(bool v);
impl<'a> /*trait*/ QHeaderView_setVisible for (i8) {
  fn setVisible(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QHeaderView10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn hiddenSectionCount<T: QHeaderView_hiddenSectionCount>(&mut self, value: T) -> i32 {
    return value.hiddenSectionCount(self);
    // return 1;
  }
}

pub trait QHeaderView_hiddenSectionCount {
  fn hiddenSectionCount(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::hiddenSectionCount();
impl<'a> /*trait*/ QHeaderView_hiddenSectionCount for () {
  fn hiddenSectionCount(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18hiddenSectionCountEv()};
    let mut ret = unsafe {_ZNK11QHeaderView18hiddenSectionCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionMoved<T: QHeaderView_sectionMoved>(&mut self, value: T)  {
     value.sectionMoved(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionMoved {
  fn sectionMoved(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::sectionMoved(int logicalIndex, int oldVisualIndex, int newVisualIndex);
impl<'a> /*trait*/ QHeaderView_sectionMoved for (i32, i32, i32) {
  fn sectionMoved(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView12sectionMovedEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN11QHeaderView12sectionMovedEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionHandleDoubleClicked<T: QHeaderView_sectionHandleDoubleClicked>(&mut self, value: T)  {
     value.sectionHandleDoubleClicked(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionHandleDoubleClicked {
  fn sectionHandleDoubleClicked(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::sectionHandleDoubleClicked(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionHandleDoubleClicked for (i32) {
  fn sectionHandleDoubleClicked(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView26sectionHandleDoubleClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView26sectionHandleDoubleClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setSectionsClickable<T: QHeaderView_setSectionsClickable>(&mut self, value: T)  {
     value.setSectionsClickable(self);
    // return 1;
  }
}

pub trait QHeaderView_setSectionsClickable {
  fn setSectionsClickable(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setSectionsClickable(bool clickable);
impl<'a> /*trait*/ QHeaderView_setSectionsClickable for (i8) {
  fn setSectionsClickable(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView20setSectionsClickableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QHeaderView20setSectionsClickableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionPressed<T: QHeaderView_sectionPressed>(&mut self, value: T)  {
     value.sectionPressed(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionPressed {
  fn sectionPressed(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::sectionPressed(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionPressed for (i32) {
  fn sectionPressed(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView14sectionPressedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView14sectionPressedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setResizeContentsPrecision<T: QHeaderView_setResizeContentsPrecision>(&mut self, value: T)  {
     value.setResizeContentsPrecision(self);
    // return 1;
  }
}

pub trait QHeaderView_setResizeContentsPrecision {
  fn setResizeContentsPrecision(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setResizeContentsPrecision(int precision);
impl<'a> /*trait*/ QHeaderView_setResizeContentsPrecision for (i32) {
  fn setResizeContentsPrecision(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView26setResizeContentsPrecisionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView26setResizeContentsPrecisionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn defaultSectionSize<T: QHeaderView_defaultSectionSize>(&mut self, value: T) -> i32 {
    return value.defaultSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_defaultSectionSize {
  fn defaultSectionSize(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::defaultSectionSize();
impl<'a> /*trait*/ QHeaderView_defaultSectionSize for () {
  fn defaultSectionSize(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18defaultSectionSizeEv()};
    let mut ret = unsafe {_ZNK11QHeaderView18defaultSectionSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setOffsetToLastSection<T: QHeaderView_setOffsetToLastSection>(&mut self, value: T)  {
     value.setOffsetToLastSection(self);
    // return 1;
  }
}

pub trait QHeaderView_setOffsetToLastSection {
  fn setOffsetToLastSection(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setOffsetToLastSection();
impl<'a> /*trait*/ QHeaderView_setOffsetToLastSection for () {
  fn setOffsetToLastSection(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView22setOffsetToLastSectionEv()};
     unsafe {_ZN11QHeaderView22setOffsetToLastSectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn swapSections<T: QHeaderView_swapSections>(&mut self, value: T)  {
     value.swapSections(self);
    // return 1;
  }
}

pub trait QHeaderView_swapSections {
  fn swapSections(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::swapSections(int first, int second);
impl<'a> /*trait*/ QHeaderView_swapSections for (i32, i32) {
  fn swapSections(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView12swapSectionsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QHeaderView12swapSectionsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn count<T: QHeaderView_count>(&mut self, value: T) -> i32 {
    return value.count(self);
    // return 1;
  }
}

pub trait QHeaderView_count {
  fn count(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::count();
impl<'a> /*trait*/ QHeaderView_count for () {
  fn count(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView5countEv()};
    let mut ret = unsafe {_ZNK11QHeaderView5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn visualIndex<T: QHeaderView_visualIndex>(&mut self, value: T) -> i32 {
    return value.visualIndex(self);
    // return 1;
  }
}

pub trait QHeaderView_visualIndex {
  fn visualIndex(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::visualIndex(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_visualIndex for (i32) {
  fn visualIndex(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView11visualIndexEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView11visualIndexEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionClicked<T: QHeaderView_sectionClicked>(&mut self, value: T)  {
     value.sectionClicked(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionClicked {
  fn sectionClicked(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::sectionClicked(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionClicked for (i32) {
  fn sectionClicked(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView14sectionClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView14sectionClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionsMoved<T: QHeaderView_sectionsMoved>(&mut self, value: T) -> i8 {
    return value.sectionsMoved(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionsMoved {
  fn sectionsMoved(self, rsthis: &mut QHeaderView) -> i8;
}

// proto:  bool QHeaderView::sectionsMoved();
impl<'a> /*trait*/ QHeaderView_sectionsMoved for () {
  fn sectionsMoved(self, rsthis: &mut QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView13sectionsMovedEv()};
    let mut ret = unsafe {_ZNK11QHeaderView13sectionsMovedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn stretchSectionCount<T: QHeaderView_stretchSectionCount>(&mut self, value: T) -> i32 {
    return value.stretchSectionCount(self);
    // return 1;
  }
}

pub trait QHeaderView_stretchSectionCount {
  fn stretchSectionCount(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::stretchSectionCount();
impl<'a> /*trait*/ QHeaderView_stretchSectionCount for () {
  fn stretchSectionCount(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView19stretchSectionCountEv()};
    let mut ret = unsafe {_ZNK11QHeaderView19stretchSectionCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn doItemsLayout<T: QHeaderView_doItemsLayout>(&mut self, value: T)  {
     value.doItemsLayout(self);
    // return 1;
  }
}

pub trait QHeaderView_doItemsLayout {
  fn doItemsLayout(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::doItemsLayout();
impl<'a> /*trait*/ QHeaderView_doItemsLayout for () {
  fn doItemsLayout(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView13doItemsLayoutEv()};
     unsafe {_ZN11QHeaderView13doItemsLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setSectionsMovable<T: QHeaderView_setSectionsMovable>(&mut self, value: T)  {
     value.setSectionsMovable(self);
    // return 1;
  }
}

pub trait QHeaderView_setSectionsMovable {
  fn setSectionsMovable(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setSectionsMovable(bool movable);
impl<'a> /*trait*/ QHeaderView_setSectionsMovable for (i8) {
  fn setSectionsMovable(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView18setSectionsMovableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QHeaderView18setSectionsMovableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionsHidden<T: QHeaderView_sectionsHidden>(&mut self, value: T) -> i8 {
    return value.sectionsHidden(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionsHidden {
  fn sectionsHidden(self, rsthis: &mut QHeaderView) -> i8;
}

// proto:  bool QHeaderView::sectionsHidden();
impl<'a> /*trait*/ QHeaderView_sectionsHidden for () {
  fn sectionsHidden(self, rsthis: &mut QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView14sectionsHiddenEv()};
    let mut ret = unsafe {_ZNK11QHeaderView14sectionsHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn minimumSectionSize<T: QHeaderView_minimumSectionSize>(&mut self, value: T) -> i32 {
    return value.minimumSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_minimumSectionSize {
  fn minimumSectionSize(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::minimumSectionSize();
impl<'a> /*trait*/ QHeaderView_minimumSectionSize for () {
  fn minimumSectionSize(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18minimumSectionSizeEv()};
    let mut ret = unsafe {_ZNK11QHeaderView18minimumSectionSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setCascadingSectionResizes<T: QHeaderView_setCascadingSectionResizes>(&mut self, value: T)  {
     value.setCascadingSectionResizes(self);
    // return 1;
  }
}

pub trait QHeaderView_setCascadingSectionResizes {
  fn setCascadingSectionResizes(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setCascadingSectionResizes(bool enable);
impl<'a> /*trait*/ QHeaderView_setCascadingSectionResizes for (i8) {
  fn setCascadingSectionResizes(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView26setCascadingSectionResizesEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QHeaderView26setCascadingSectionResizesEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setDefaultSectionSize<T: QHeaderView_setDefaultSectionSize>(&mut self, value: T)  {
     value.setDefaultSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_setDefaultSectionSize {
  fn setDefaultSectionSize(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setDefaultSectionSize(int size);
impl<'a> /*trait*/ QHeaderView_setDefaultSectionSize for (i32) {
  fn setDefaultSectionSize(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setDefaultSectionSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView21setDefaultSectionSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionEntered<T: QHeaderView_sectionEntered>(&mut self, value: T)  {
     value.sectionEntered(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionEntered {
  fn sectionEntered(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::sectionEntered(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionEntered for (i32) {
  fn sectionEntered(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView14sectionEnteredEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView14sectionEnteredEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn moveSection<T: QHeaderView_moveSection>(&mut self, value: T)  {
     value.moveSection(self);
    // return 1;
  }
}

pub trait QHeaderView_moveSection {
  fn moveSection(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::moveSection(int from, int to);
impl<'a> /*trait*/ QHeaderView_moveSection for (i32, i32) {
  fn moveSection(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView11moveSectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QHeaderView11moveSectionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn stretchLastSection<T: QHeaderView_stretchLastSection>(&mut self, value: T) -> i8 {
    return value.stretchLastSection(self);
    // return 1;
  }
}

pub trait QHeaderView_stretchLastSection {
  fn stretchLastSection(self, rsthis: &mut QHeaderView) -> i8;
}

// proto:  bool QHeaderView::stretchLastSection();
impl<'a> /*trait*/ QHeaderView_stretchLastSection for () {
  fn stretchLastSection(self, rsthis: &mut QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18stretchLastSectionEv()};
    let mut ret = unsafe {_ZNK11QHeaderView18stretchLastSectionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionSizeHint<T: QHeaderView_sectionSizeHint>(&mut self, value: T) -> i32 {
    return value.sectionSizeHint(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionSizeHint {
  fn sectionSizeHint(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::sectionSizeHint(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionSizeHint for (i32) {
  fn sectionSizeHint(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView15sectionSizeHintEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView15sectionSizeHintEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionsMovable<T: QHeaderView_sectionsMovable>(&mut self, value: T) -> i8 {
    return value.sectionsMovable(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionsMovable {
  fn sectionsMovable(self, rsthis: &mut QHeaderView) -> i8;
}

// proto:  bool QHeaderView::sectionsMovable();
impl<'a> /*trait*/ QHeaderView_sectionsMovable for () {
  fn sectionsMovable(self, rsthis: &mut QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView15sectionsMovableEv()};
    let mut ret = unsafe {_ZNK11QHeaderView15sectionsMovableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn isSectionHidden<T: QHeaderView_isSectionHidden>(&mut self, value: T) -> i8 {
    return value.isSectionHidden(self);
    // return 1;
  }
}

pub trait QHeaderView_isSectionHidden {
  fn isSectionHidden(self, rsthis: &mut QHeaderView) -> i8;
}

// proto:  bool QHeaderView::isSectionHidden(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_isSectionHidden for (i32) {
  fn isSectionHidden(self, rsthis: &mut QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView15isSectionHiddenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView15isSectionHiddenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  int QHeaderView::logicalIndexAt(int x, int y);
impl<'a> /*trait*/ QHeaderView_logicalIndexAt for (i32, i32) {
  fn logicalIndexAt(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView14logicalIndexAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView14logicalIndexAtEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QHeaderView::logicalIndexAt(int position);
impl<'a> /*trait*/ QHeaderView_logicalIndexAt for (i32) {
  fn logicalIndexAt(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView14logicalIndexAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView14logicalIndexAtEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn logicalIndex<T: QHeaderView_logicalIndex>(&mut self, value: T) -> i32 {
    return value.logicalIndex(self);
    // return 1;
  }
}

pub trait QHeaderView_logicalIndex {
  fn logicalIndex(self, rsthis: &mut QHeaderView) -> i32;
}

// proto:  int QHeaderView::logicalIndex(int visualIndex);
impl<'a> /*trait*/ QHeaderView_logicalIndex for (i32) {
  fn logicalIndex(self, rsthis: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView12logicalIndexEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QHeaderView12logicalIndexEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setMaximumSectionSize<T: QHeaderView_setMaximumSectionSize>(&mut self, value: T)  {
     value.setMaximumSectionSize(self);
    // return 1;
  }
}

pub trait QHeaderView_setMaximumSectionSize {
  fn setMaximumSectionSize(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setMaximumSectionSize(int size);
impl<'a> /*trait*/ QHeaderView_setMaximumSectionSize for (i32) {
  fn setMaximumSectionSize(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setMaximumSectionSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView21setMaximumSectionSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setHighlightSections<T: QHeaderView_setHighlightSections>(&mut self, value: T)  {
     value.setHighlightSections(self);
    // return 1;
  }
}

pub trait QHeaderView_setHighlightSections {
  fn setHighlightSections(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setHighlightSections(bool highlight);
impl<'a> /*trait*/ QHeaderView_setHighlightSections for (i8) {
  fn setHighlightSections(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView20setHighlightSectionsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QHeaderView20setHighlightSectionsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setSectionHidden<T: QHeaderView_setSectionHidden>(&mut self, value: T)  {
     value.setSectionHidden(self);
    // return 1;
  }
}

pub trait QHeaderView_setSectionHidden {
  fn setSectionHidden(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::setSectionHidden(int logicalIndex, bool hide);
impl<'a> /*trait*/ QHeaderView_setSectionHidden for (i32, i8) {
  fn setSectionHidden(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView16setSectionHiddenEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN11QHeaderView16setSectionHiddenEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn resizeSection<T: QHeaderView_resizeSection>(&mut self, value: T)  {
     value.resizeSection(self);
    // return 1;
  }
}

pub trait QHeaderView_resizeSection {
  fn resizeSection(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::resizeSection(int logicalIndex, int size);
impl<'a> /*trait*/ QHeaderView_resizeSection for (i32, i32) {
  fn resizeSection(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView13resizeSectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QHeaderView13resizeSectionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn restoreState<T: QHeaderView_restoreState>(&mut self, value: T) -> i8 {
    return value.restoreState(self);
    // return 1;
  }
}

pub trait QHeaderView_restoreState {
  fn restoreState(self, rsthis: &mut QHeaderView) -> i8;
}

// proto:  bool QHeaderView::restoreState(const QByteArray & state);
impl<'a> /*trait*/ QHeaderView_restoreState for (&'a  QByteArray) {
  fn restoreState(self, rsthis: &mut QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView12restoreStateERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QHeaderView12restoreStateERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionDoubleClicked<T: QHeaderView_sectionDoubleClicked>(&mut self, value: T)  {
     value.sectionDoubleClicked(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionDoubleClicked {
  fn sectionDoubleClicked(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::sectionDoubleClicked(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionDoubleClicked for (i32) {
  fn sectionDoubleClicked(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView20sectionDoubleClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QHeaderView20sectionDoubleClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionCountChanged<T: QHeaderView_sectionCountChanged>(&mut self, value: T)  {
     value.sectionCountChanged(self);
    // return 1;
  }
}

pub trait QHeaderView_sectionCountChanged {
  fn sectionCountChanged(self, rsthis: &mut QHeaderView) ;
}

// proto:  void QHeaderView::sectionCountChanged(int oldCount, int newCount);
impl<'a> /*trait*/ QHeaderView_sectionCountChanged for (i32, i32) {
  fn sectionCountChanged(self, rsthis: &mut QHeaderView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView19sectionCountChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QHeaderView19sectionCountChangedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn isSortIndicatorShown<T: QHeaderView_isSortIndicatorShown>(&mut self, value: T) -> i8 {
    return value.isSortIndicatorShown(self);
    // return 1;
  }
}

pub trait QHeaderView_isSortIndicatorShown {
  fn isSortIndicatorShown(self, rsthis: &mut QHeaderView) -> i8;
}

// proto:  bool QHeaderView::isSortIndicatorShown();
impl<'a> /*trait*/ QHeaderView_isSortIndicatorShown for () {
  fn isSortIndicatorShown(self, rsthis: &mut QHeaderView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView20isSortIndicatorShownEv()};
    let mut ret = unsafe {_ZNK11QHeaderView20isSortIndicatorShownEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

