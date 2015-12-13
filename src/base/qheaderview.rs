// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QHeaderView::maximumSectionSize();
  fn _ZNK11QHeaderView18maximumSectionSizeEv() -> i32;
  // proto: QSize QHeaderView::sizeHint();
  fn _ZNK11QHeaderView8sizeHintEv() -> i32;
  // proto: int QHeaderView::sectionPosition(int logicalIndex);
  fn _ZNK11QHeaderView15sectionPositionEi(arg0: c_int) -> i32;
  // proto: void QHeaderView::sectionResized(int logicalIndex, int oldSize, int newSize);
  fn _ZN11QHeaderView14sectionResizedEiii(arg0: c_int, arg1: c_int, arg2: c_int) -> i32;
  // proto: int QHeaderView::sectionSize(int logicalIndex);
  fn _ZNK11QHeaderView11sectionSizeEi(arg0: c_int) -> i32;
  // proto: void QHeaderView::NewQHeaderView(const QHeaderView & );
  fn _ZN11QHeaderViewC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QHeaderView::setStretchLastSection(bool stretch);
  fn _ZN11QHeaderView21setStretchLastSectionEb(arg0: int8_t) -> i32;
  // proto: void QHeaderView::reset();
  fn _ZN11QHeaderView5resetEv() -> i32;
  // proto: void QHeaderView::geometriesChanged();
  fn _ZN11QHeaderView17geometriesChangedEv() -> i32;
  // proto: void QHeaderView::resetDefaultSectionSize();
  fn _ZN11QHeaderView23resetDefaultSectionSizeEv() -> i32;
  // proto: QByteArray QHeaderView::saveState();
  fn _ZNK11QHeaderView9saveStateEv() -> i32;
  // proto: bool QHeaderView::sectionsClickable();
  fn _ZNK11QHeaderView17sectionsClickableEv() -> i32;
  // proto: int QHeaderView::resizeContentsPrecision();
  fn _ZNK11QHeaderView23resizeContentsPrecisionEv() -> i32;
  // proto: void QHeaderView::setOffsetToSectionPosition(int visualIndex);
  fn _ZN11QHeaderView26setOffsetToSectionPositionEi(arg0: c_int) -> i32;
  // proto: int QHeaderView::length();
  fn _ZNK11QHeaderView6lengthEv() -> i32;
  // proto: void QHeaderView::hideSection(int logicalIndex);
  fn _ZN11QHeaderView11hideSectionEi(arg0: c_int) -> i32;
  // proto: int QHeaderView::sortIndicatorSection();
  fn _ZNK11QHeaderView20sortIndicatorSectionEv() -> i32;
  // proto: bool QHeaderView::cascadingSectionResizes();
  fn _ZNK11QHeaderView23cascadingSectionResizesEv() -> i32;
  // proto: void QHeaderView::setMinimumSectionSize(int size);
  fn _ZN11QHeaderView21setMinimumSectionSizeEi(arg0: c_int) -> i32;
  // proto: int QHeaderView::visualIndexAt(int position);
  fn _ZNK11QHeaderView13visualIndexAtEi(arg0: c_int) -> i32;
  // proto: void QHeaderView::setOffset(int offset);
  fn _ZN11QHeaderView9setOffsetEi(arg0: c_int) -> i32;
  // proto: int QHeaderView::logicalIndexAt(const QPoint & pos);
  fn _ZNK11QHeaderView14logicalIndexAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QHeaderView::FreeQHeaderView();
  fn _ZN11QHeaderViewD0Ev() -> i32;
  // proto: int QHeaderView::sectionViewportPosition(int logicalIndex);
  fn _ZNK11QHeaderView23sectionViewportPositionEi(arg0: c_int) -> i32;
  // proto: bool QHeaderView::highlightSections();
  fn _ZNK11QHeaderView17highlightSectionsEv() -> i32;
  // proto: int QHeaderView::offset();
  fn _ZNK11QHeaderView6offsetEv() -> i32;
  // proto: void QHeaderView::setSortIndicatorShown(bool show);
  fn _ZN11QHeaderView21setSortIndicatorShownEb(arg0: int8_t) -> i32;
  // proto: const QMetaObject * QHeaderView::metaObject();
  fn _ZNK11QHeaderView10metaObjectEv() -> i32;
  // proto: void QHeaderView::showSection(int logicalIndex);
  fn _ZN11QHeaderView11showSectionEi(arg0: c_int) -> i32;
  // proto: void QHeaderView::setVisible(bool v);
  fn _ZN11QHeaderView10setVisibleEb(arg0: int8_t) -> i32;
  // proto: int QHeaderView::hiddenSectionCount();
  fn _ZNK11QHeaderView18hiddenSectionCountEv() -> i32;
  // proto: void QHeaderView::sectionMoved(int logicalIndex, int oldVisualIndex, int newVisualIndex);
  fn _ZN11QHeaderView12sectionMovedEiii(arg0: c_int, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QHeaderView::sectionHandleDoubleClicked(int logicalIndex);
  fn _ZN11QHeaderView26sectionHandleDoubleClickedEi(arg0: c_int) -> i32;
  // proto: void QHeaderView::setSectionsClickable(bool clickable);
  fn _ZN11QHeaderView20setSectionsClickableEb(arg0: int8_t) -> i32;
  // proto: void QHeaderView::sectionPressed(int logicalIndex);
  fn _ZN11QHeaderView14sectionPressedEi(arg0: c_int) -> i32;
  // proto: void QHeaderView::setResizeContentsPrecision(int precision);
  fn _ZN11QHeaderView26setResizeContentsPrecisionEi(arg0: c_int) -> i32;
  // proto: int QHeaderView::defaultSectionSize();
  fn _ZNK11QHeaderView18defaultSectionSizeEv() -> i32;
  // proto: void QHeaderView::setOffsetToLastSection();
  fn _ZN11QHeaderView22setOffsetToLastSectionEv() -> i32;
  // proto: void QHeaderView::swapSections(int first, int second);
  fn _ZN11QHeaderView12swapSectionsEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: int QHeaderView::count();
  fn _ZNK11QHeaderView5countEv() -> i32;
  // proto: int QHeaderView::visualIndex(int logicalIndex);
  fn _ZNK11QHeaderView11visualIndexEi(arg0: c_int) -> i32;
  // proto: void QHeaderView::sectionClicked(int logicalIndex);
  fn _ZN11QHeaderView14sectionClickedEi(arg0: c_int) -> i32;
  // proto: bool QHeaderView::sectionsMoved();
  fn _ZNK11QHeaderView13sectionsMovedEv() -> i32;
  // proto: int QHeaderView::stretchSectionCount();
  fn _ZNK11QHeaderView19stretchSectionCountEv() -> i32;
  // proto: void QHeaderView::doItemsLayout();
  fn _ZN11QHeaderView13doItemsLayoutEv() -> i32;
  // proto: void QHeaderView::setSectionsMovable(bool movable);
  fn _ZN11QHeaderView18setSectionsMovableEb(arg0: int8_t) -> i32;
  // proto: bool QHeaderView::sectionsHidden();
  fn _ZNK11QHeaderView14sectionsHiddenEv() -> i32;
  // proto: int QHeaderView::minimumSectionSize();
  fn _ZNK11QHeaderView18minimumSectionSizeEv() -> i32;
  // proto: void QHeaderView::setCascadingSectionResizes(bool enable);
  fn _ZN11QHeaderView26setCascadingSectionResizesEb(arg0: int8_t) -> i32;
  // proto: void QHeaderView::setDefaultSectionSize(int size);
  fn _ZN11QHeaderView21setDefaultSectionSizeEi(arg0: c_int) -> i32;
  // proto: void QHeaderView::sectionEntered(int logicalIndex);
  fn _ZN11QHeaderView14sectionEnteredEi(arg0: c_int) -> i32;
  // proto: void QHeaderView::moveSection(int from, int to);
  fn _ZN11QHeaderView11moveSectionEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: bool QHeaderView::stretchLastSection();
  fn _ZNK11QHeaderView18stretchLastSectionEv() -> i32;
  // proto: int QHeaderView::sectionSizeHint(int logicalIndex);
  fn _ZNK11QHeaderView15sectionSizeHintEi(arg0: c_int) -> i32;
  // proto: bool QHeaderView::sectionsMovable();
  fn _ZNK11QHeaderView15sectionsMovableEv() -> i32;
  // proto: bool QHeaderView::isSectionHidden(int logicalIndex);
  fn _ZNK11QHeaderView15isSectionHiddenEi(arg0: c_int) -> i32;
  // proto: int QHeaderView::logicalIndexAt(int x, int y);
  fn _ZNK11QHeaderView14logicalIndexAtEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: int QHeaderView::logicalIndexAt(int position);
  fn _ZNK11QHeaderView14logicalIndexAtEi(arg0: c_int) -> i32;
  // proto: int QHeaderView::logicalIndex(int visualIndex);
  fn _ZNK11QHeaderView12logicalIndexEi(arg0: c_int) -> i32;
  // proto: void QHeaderView::setMaximumSectionSize(int size);
  fn _ZN11QHeaderView21setMaximumSectionSizeEi(arg0: c_int) -> i32;
  // proto: void QHeaderView::setHighlightSections(bool highlight);
  fn _ZN11QHeaderView20setHighlightSectionsEb(arg0: int8_t) -> i32;
  // proto: void QHeaderView::setSectionHidden(int logicalIndex, bool hide);
  fn _ZN11QHeaderView16setSectionHiddenEib(arg0: c_int, arg1: int8_t) -> i32;
  // proto: void QHeaderView::resizeSection(int logicalIndex, int size);
  fn _ZN11QHeaderView13resizeSectionEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: bool QHeaderView::restoreState(const QByteArray & state);
  fn _ZN11QHeaderView12restoreStateERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: void QHeaderView::sectionDoubleClicked(int logicalIndex);
  fn _ZN11QHeaderView20sectionDoubleClickedEi(arg0: c_int) -> i32;
  // proto: void QHeaderView::sectionCountChanged(int oldCount, int newCount);
  fn _ZN11QHeaderView19sectionCountChangedEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: bool QHeaderView::isSortIndicatorShown();
  fn _ZNK11QHeaderView20isSortIndicatorShownEv() -> i32;
}

// body block begin
// class sizeof(QHeaderView)=1
pub struct QHeaderView {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QHeaderView {
  pub fn maximumSectionSize<T: QHeaderView_maximumSectionSize>(&mut self, value: T) -> i32 {
    value.maximumSectionSize(self);
    return 1;
  }
}

pub trait QHeaderView_maximumSectionSize {
  fn maximumSectionSize(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::maximumSectionSize();
impl<'a> /*trait*/ QHeaderView_maximumSectionSize for () {
  fn maximumSectionSize(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18maximumSectionSizeEv()};
    unsafe {_ZNK11QHeaderView18maximumSectionSizeEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sizeHint<T: QHeaderView_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QHeaderView_sizeHint {
  fn sizeHint(self, this: &mut QHeaderView) -> i32;
}

// proto: QSize QHeaderView::sizeHint();
impl<'a> /*trait*/ QHeaderView_sizeHint for () {
  fn sizeHint(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView8sizeHintEv()};
    unsafe {_ZNK11QHeaderView8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionPosition<T: QHeaderView_sectionPosition>(&mut self, value: T) -> i32 {
    value.sectionPosition(self);
    return 1;
  }
}

pub trait QHeaderView_sectionPosition {
  fn sectionPosition(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::sectionPosition(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionPosition for (i32) {
  fn sectionPosition(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView15sectionPositionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QHeaderView15sectionPositionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionResized<T: QHeaderView_sectionResized>(&mut self, value: T) -> i32 {
    value.sectionResized(self);
    return 1;
  }
}

pub trait QHeaderView_sectionResized {
  fn sectionResized(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::sectionResized(int logicalIndex, int oldSize, int newSize);
impl<'a> /*trait*/ QHeaderView_sectionResized for (i32, i32, i32) {
  fn sectionResized(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView14sectionResizedEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN11QHeaderView14sectionResizedEiii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionSize<T: QHeaderView_sectionSize>(&mut self, value: T) -> i32 {
    value.sectionSize(self);
    return 1;
  }
}

pub trait QHeaderView_sectionSize {
  fn sectionSize(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::sectionSize(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionSize for (i32) {
  fn sectionSize(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView11sectionSizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QHeaderView11sectionSizeEi(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QHeaderViewC1ERKS_(qthis, arg0)};
    let rsthis = QHeaderView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setStretchLastSection<T: QHeaderView_setStretchLastSection>(&mut self, value: T) -> i32 {
    value.setStretchLastSection(self);
    return 1;
  }
}

pub trait QHeaderView_setStretchLastSection {
  fn setStretchLastSection(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setStretchLastSection(bool stretch);
impl<'a> /*trait*/ QHeaderView_setStretchLastSection for (i8) {
  fn setStretchLastSection(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setStretchLastSectionEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QHeaderView21setStretchLastSectionEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn reset<T: QHeaderView_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QHeaderView_reset {
  fn reset(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::reset();
impl<'a> /*trait*/ QHeaderView_reset for () {
  fn reset(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView5resetEv()};
    unsafe {_ZN11QHeaderView5resetEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn geometriesChanged<T: QHeaderView_geometriesChanged>(&mut self, value: T) -> i32 {
    value.geometriesChanged(self);
    return 1;
  }
}

pub trait QHeaderView_geometriesChanged {
  fn geometriesChanged(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::geometriesChanged();
impl<'a> /*trait*/ QHeaderView_geometriesChanged for () {
  fn geometriesChanged(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView17geometriesChangedEv()};
    unsafe {_ZN11QHeaderView17geometriesChangedEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn resetDefaultSectionSize<T: QHeaderView_resetDefaultSectionSize>(&mut self, value: T) -> i32 {
    value.resetDefaultSectionSize(self);
    return 1;
  }
}

pub trait QHeaderView_resetDefaultSectionSize {
  fn resetDefaultSectionSize(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::resetDefaultSectionSize();
impl<'a> /*trait*/ QHeaderView_resetDefaultSectionSize for () {
  fn resetDefaultSectionSize(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView23resetDefaultSectionSizeEv()};
    unsafe {_ZN11QHeaderView23resetDefaultSectionSizeEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn saveState<T: QHeaderView_saveState>(&mut self, value: T) -> i32 {
    value.saveState(self);
    return 1;
  }
}

pub trait QHeaderView_saveState {
  fn saveState(self, this: &mut QHeaderView) -> i32;
}

// proto: QByteArray QHeaderView::saveState();
impl<'a> /*trait*/ QHeaderView_saveState for () {
  fn saveState(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView9saveStateEv()};
    unsafe {_ZNK11QHeaderView9saveStateEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionsClickable<T: QHeaderView_sectionsClickable>(&mut self, value: T) -> i32 {
    value.sectionsClickable(self);
    return 1;
  }
}

pub trait QHeaderView_sectionsClickable {
  fn sectionsClickable(self, this: &mut QHeaderView) -> i32;
}

// proto: bool QHeaderView::sectionsClickable();
impl<'a> /*trait*/ QHeaderView_sectionsClickable for () {
  fn sectionsClickable(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView17sectionsClickableEv()};
    unsafe {_ZNK11QHeaderView17sectionsClickableEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn resizeContentsPrecision<T: QHeaderView_resizeContentsPrecision>(&mut self, value: T) -> i32 {
    value.resizeContentsPrecision(self);
    return 1;
  }
}

pub trait QHeaderView_resizeContentsPrecision {
  fn resizeContentsPrecision(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::resizeContentsPrecision();
impl<'a> /*trait*/ QHeaderView_resizeContentsPrecision for () {
  fn resizeContentsPrecision(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView23resizeContentsPrecisionEv()};
    unsafe {_ZNK11QHeaderView23resizeContentsPrecisionEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setOffsetToSectionPosition<T: QHeaderView_setOffsetToSectionPosition>(&mut self, value: T) -> i32 {
    value.setOffsetToSectionPosition(self);
    return 1;
  }
}

pub trait QHeaderView_setOffsetToSectionPosition {
  fn setOffsetToSectionPosition(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setOffsetToSectionPosition(int visualIndex);
impl<'a> /*trait*/ QHeaderView_setOffsetToSectionPosition for (i32) {
  fn setOffsetToSectionPosition(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView26setOffsetToSectionPositionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QHeaderView26setOffsetToSectionPositionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn length<T: QHeaderView_length>(&mut self, value: T) -> i32 {
    value.length(self);
    return 1;
  }
}

pub trait QHeaderView_length {
  fn length(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::length();
impl<'a> /*trait*/ QHeaderView_length for () {
  fn length(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView6lengthEv()};
    unsafe {_ZNK11QHeaderView6lengthEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn hideSection<T: QHeaderView_hideSection>(&mut self, value: T) -> i32 {
    value.hideSection(self);
    return 1;
  }
}

pub trait QHeaderView_hideSection {
  fn hideSection(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::hideSection(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_hideSection for (i32) {
  fn hideSection(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView11hideSectionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QHeaderView11hideSectionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sortIndicatorSection<T: QHeaderView_sortIndicatorSection>(&mut self, value: T) -> i32 {
    value.sortIndicatorSection(self);
    return 1;
  }
}

pub trait QHeaderView_sortIndicatorSection {
  fn sortIndicatorSection(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::sortIndicatorSection();
impl<'a> /*trait*/ QHeaderView_sortIndicatorSection for () {
  fn sortIndicatorSection(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView20sortIndicatorSectionEv()};
    unsafe {_ZNK11QHeaderView20sortIndicatorSectionEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn cascadingSectionResizes<T: QHeaderView_cascadingSectionResizes>(&mut self, value: T) -> i32 {
    value.cascadingSectionResizes(self);
    return 1;
  }
}

pub trait QHeaderView_cascadingSectionResizes {
  fn cascadingSectionResizes(self, this: &mut QHeaderView) -> i32;
}

// proto: bool QHeaderView::cascadingSectionResizes();
impl<'a> /*trait*/ QHeaderView_cascadingSectionResizes for () {
  fn cascadingSectionResizes(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView23cascadingSectionResizesEv()};
    unsafe {_ZNK11QHeaderView23cascadingSectionResizesEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setMinimumSectionSize<T: QHeaderView_setMinimumSectionSize>(&mut self, value: T) -> i32 {
    value.setMinimumSectionSize(self);
    return 1;
  }
}

pub trait QHeaderView_setMinimumSectionSize {
  fn setMinimumSectionSize(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setMinimumSectionSize(int size);
impl<'a> /*trait*/ QHeaderView_setMinimumSectionSize for (i32) {
  fn setMinimumSectionSize(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setMinimumSectionSizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QHeaderView21setMinimumSectionSizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn visualIndexAt<T: QHeaderView_visualIndexAt>(&mut self, value: T) -> i32 {
    value.visualIndexAt(self);
    return 1;
  }
}

pub trait QHeaderView_visualIndexAt {
  fn visualIndexAt(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::visualIndexAt(int position);
impl<'a> /*trait*/ QHeaderView_visualIndexAt for (i32) {
  fn visualIndexAt(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView13visualIndexAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QHeaderView13visualIndexAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setOffset<T: QHeaderView_setOffset>(&mut self, value: T) -> i32 {
    value.setOffset(self);
    return 1;
  }
}

pub trait QHeaderView_setOffset {
  fn setOffset(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setOffset(int offset);
impl<'a> /*trait*/ QHeaderView_setOffset for (i32) {
  fn setOffset(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView9setOffsetEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QHeaderView9setOffsetEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn logicalIndexAt<T: QHeaderView_logicalIndexAt>(&mut self, value: T) -> i32 {
    value.logicalIndexAt(self);
    return 1;
  }
}

pub trait QHeaderView_logicalIndexAt {
  fn logicalIndexAt(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::logicalIndexAt(const QPoint & pos);
impl<'a> /*trait*/ QHeaderView_logicalIndexAt for (&'a  QPoint) {
  fn logicalIndexAt(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView14logicalIndexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK11QHeaderView14logicalIndexAtERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn FreeQHeaderView<T: QHeaderView_FreeQHeaderView>(&mut self, value: T) -> i32 {
    value.FreeQHeaderView(self);
    return 1;
  }
}

pub trait QHeaderView_FreeQHeaderView {
  fn FreeQHeaderView(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::FreeQHeaderView();
impl<'a> /*trait*/ QHeaderView_FreeQHeaderView for () {
  fn FreeQHeaderView(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderViewD0Ev()};
    unsafe {_ZN11QHeaderViewD0Ev()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionViewportPosition<T: QHeaderView_sectionViewportPosition>(&mut self, value: T) -> i32 {
    value.sectionViewportPosition(self);
    return 1;
  }
}

pub trait QHeaderView_sectionViewportPosition {
  fn sectionViewportPosition(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::sectionViewportPosition(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionViewportPosition for (i32) {
  fn sectionViewportPosition(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView23sectionViewportPositionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QHeaderView23sectionViewportPositionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn highlightSections<T: QHeaderView_highlightSections>(&mut self, value: T) -> i32 {
    value.highlightSections(self);
    return 1;
  }
}

pub trait QHeaderView_highlightSections {
  fn highlightSections(self, this: &mut QHeaderView) -> i32;
}

// proto: bool QHeaderView::highlightSections();
impl<'a> /*trait*/ QHeaderView_highlightSections for () {
  fn highlightSections(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView17highlightSectionsEv()};
    unsafe {_ZNK11QHeaderView17highlightSectionsEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn offset<T: QHeaderView_offset>(&mut self, value: T) -> i32 {
    value.offset(self);
    return 1;
  }
}

pub trait QHeaderView_offset {
  fn offset(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::offset();
impl<'a> /*trait*/ QHeaderView_offset for () {
  fn offset(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView6offsetEv()};
    unsafe {_ZNK11QHeaderView6offsetEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setSortIndicatorShown<T: QHeaderView_setSortIndicatorShown>(&mut self, value: T) -> i32 {
    value.setSortIndicatorShown(self);
    return 1;
  }
}

pub trait QHeaderView_setSortIndicatorShown {
  fn setSortIndicatorShown(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setSortIndicatorShown(bool show);
impl<'a> /*trait*/ QHeaderView_setSortIndicatorShown for (i8) {
  fn setSortIndicatorShown(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setSortIndicatorShownEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QHeaderView21setSortIndicatorShownEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn metaObject<T: QHeaderView_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QHeaderView_metaObject {
  fn metaObject(self, this: &mut QHeaderView) -> i32;
}

// proto: const QMetaObject * QHeaderView::metaObject();
impl<'a> /*trait*/ QHeaderView_metaObject for () {
  fn metaObject(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView10metaObjectEv()};
    unsafe {_ZNK11QHeaderView10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn showSection<T: QHeaderView_showSection>(&mut self, value: T) -> i32 {
    value.showSection(self);
    return 1;
  }
}

pub trait QHeaderView_showSection {
  fn showSection(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::showSection(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_showSection for (i32) {
  fn showSection(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView11showSectionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QHeaderView11showSectionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setVisible<T: QHeaderView_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QHeaderView_setVisible {
  fn setVisible(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setVisible(bool v);
impl<'a> /*trait*/ QHeaderView_setVisible for (i8) {
  fn setVisible(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QHeaderView10setVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn hiddenSectionCount<T: QHeaderView_hiddenSectionCount>(&mut self, value: T) -> i32 {
    value.hiddenSectionCount(self);
    return 1;
  }
}

pub trait QHeaderView_hiddenSectionCount {
  fn hiddenSectionCount(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::hiddenSectionCount();
impl<'a> /*trait*/ QHeaderView_hiddenSectionCount for () {
  fn hiddenSectionCount(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18hiddenSectionCountEv()};
    unsafe {_ZNK11QHeaderView18hiddenSectionCountEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionMoved<T: QHeaderView_sectionMoved>(&mut self, value: T) -> i32 {
    value.sectionMoved(self);
    return 1;
  }
}

pub trait QHeaderView_sectionMoved {
  fn sectionMoved(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::sectionMoved(int logicalIndex, int oldVisualIndex, int newVisualIndex);
impl<'a> /*trait*/ QHeaderView_sectionMoved for (i32, i32, i32) {
  fn sectionMoved(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView12sectionMovedEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN11QHeaderView12sectionMovedEiii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionHandleDoubleClicked<T: QHeaderView_sectionHandleDoubleClicked>(&mut self, value: T) -> i32 {
    value.sectionHandleDoubleClicked(self);
    return 1;
  }
}

pub trait QHeaderView_sectionHandleDoubleClicked {
  fn sectionHandleDoubleClicked(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::sectionHandleDoubleClicked(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionHandleDoubleClicked for (i32) {
  fn sectionHandleDoubleClicked(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView26sectionHandleDoubleClickedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QHeaderView26sectionHandleDoubleClickedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setSectionsClickable<T: QHeaderView_setSectionsClickable>(&mut self, value: T) -> i32 {
    value.setSectionsClickable(self);
    return 1;
  }
}

pub trait QHeaderView_setSectionsClickable {
  fn setSectionsClickable(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setSectionsClickable(bool clickable);
impl<'a> /*trait*/ QHeaderView_setSectionsClickable for (i8) {
  fn setSectionsClickable(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView20setSectionsClickableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QHeaderView20setSectionsClickableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionPressed<T: QHeaderView_sectionPressed>(&mut self, value: T) -> i32 {
    value.sectionPressed(self);
    return 1;
  }
}

pub trait QHeaderView_sectionPressed {
  fn sectionPressed(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::sectionPressed(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionPressed for (i32) {
  fn sectionPressed(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView14sectionPressedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QHeaderView14sectionPressedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setResizeContentsPrecision<T: QHeaderView_setResizeContentsPrecision>(&mut self, value: T) -> i32 {
    value.setResizeContentsPrecision(self);
    return 1;
  }
}

pub trait QHeaderView_setResizeContentsPrecision {
  fn setResizeContentsPrecision(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setResizeContentsPrecision(int precision);
impl<'a> /*trait*/ QHeaderView_setResizeContentsPrecision for (i32) {
  fn setResizeContentsPrecision(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView26setResizeContentsPrecisionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QHeaderView26setResizeContentsPrecisionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn defaultSectionSize<T: QHeaderView_defaultSectionSize>(&mut self, value: T) -> i32 {
    value.defaultSectionSize(self);
    return 1;
  }
}

pub trait QHeaderView_defaultSectionSize {
  fn defaultSectionSize(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::defaultSectionSize();
impl<'a> /*trait*/ QHeaderView_defaultSectionSize for () {
  fn defaultSectionSize(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18defaultSectionSizeEv()};
    unsafe {_ZNK11QHeaderView18defaultSectionSizeEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setOffsetToLastSection<T: QHeaderView_setOffsetToLastSection>(&mut self, value: T) -> i32 {
    value.setOffsetToLastSection(self);
    return 1;
  }
}

pub trait QHeaderView_setOffsetToLastSection {
  fn setOffsetToLastSection(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setOffsetToLastSection();
impl<'a> /*trait*/ QHeaderView_setOffsetToLastSection for () {
  fn setOffsetToLastSection(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView22setOffsetToLastSectionEv()};
    unsafe {_ZN11QHeaderView22setOffsetToLastSectionEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn swapSections<T: QHeaderView_swapSections>(&mut self, value: T) -> i32 {
    value.swapSections(self);
    return 1;
  }
}

pub trait QHeaderView_swapSections {
  fn swapSections(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::swapSections(int first, int second);
impl<'a> /*trait*/ QHeaderView_swapSections for (i32, i32) {
  fn swapSections(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView12swapSectionsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QHeaderView12swapSectionsEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn count<T: QHeaderView_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QHeaderView_count {
  fn count(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::count();
impl<'a> /*trait*/ QHeaderView_count for () {
  fn count(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView5countEv()};
    unsafe {_ZNK11QHeaderView5countEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn visualIndex<T: QHeaderView_visualIndex>(&mut self, value: T) -> i32 {
    value.visualIndex(self);
    return 1;
  }
}

pub trait QHeaderView_visualIndex {
  fn visualIndex(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::visualIndex(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_visualIndex for (i32) {
  fn visualIndex(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView11visualIndexEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QHeaderView11visualIndexEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionClicked<T: QHeaderView_sectionClicked>(&mut self, value: T) -> i32 {
    value.sectionClicked(self);
    return 1;
  }
}

pub trait QHeaderView_sectionClicked {
  fn sectionClicked(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::sectionClicked(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionClicked for (i32) {
  fn sectionClicked(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView14sectionClickedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QHeaderView14sectionClickedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionsMoved<T: QHeaderView_sectionsMoved>(&mut self, value: T) -> i32 {
    value.sectionsMoved(self);
    return 1;
  }
}

pub trait QHeaderView_sectionsMoved {
  fn sectionsMoved(self, this: &mut QHeaderView) -> i32;
}

// proto: bool QHeaderView::sectionsMoved();
impl<'a> /*trait*/ QHeaderView_sectionsMoved for () {
  fn sectionsMoved(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView13sectionsMovedEv()};
    unsafe {_ZNK11QHeaderView13sectionsMovedEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn stretchSectionCount<T: QHeaderView_stretchSectionCount>(&mut self, value: T) -> i32 {
    value.stretchSectionCount(self);
    return 1;
  }
}

pub trait QHeaderView_stretchSectionCount {
  fn stretchSectionCount(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::stretchSectionCount();
impl<'a> /*trait*/ QHeaderView_stretchSectionCount for () {
  fn stretchSectionCount(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView19stretchSectionCountEv()};
    unsafe {_ZNK11QHeaderView19stretchSectionCountEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn doItemsLayout<T: QHeaderView_doItemsLayout>(&mut self, value: T) -> i32 {
    value.doItemsLayout(self);
    return 1;
  }
}

pub trait QHeaderView_doItemsLayout {
  fn doItemsLayout(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::doItemsLayout();
impl<'a> /*trait*/ QHeaderView_doItemsLayout for () {
  fn doItemsLayout(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView13doItemsLayoutEv()};
    unsafe {_ZN11QHeaderView13doItemsLayoutEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setSectionsMovable<T: QHeaderView_setSectionsMovable>(&mut self, value: T) -> i32 {
    value.setSectionsMovable(self);
    return 1;
  }
}

pub trait QHeaderView_setSectionsMovable {
  fn setSectionsMovable(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setSectionsMovable(bool movable);
impl<'a> /*trait*/ QHeaderView_setSectionsMovable for (i8) {
  fn setSectionsMovable(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView18setSectionsMovableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QHeaderView18setSectionsMovableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionsHidden<T: QHeaderView_sectionsHidden>(&mut self, value: T) -> i32 {
    value.sectionsHidden(self);
    return 1;
  }
}

pub trait QHeaderView_sectionsHidden {
  fn sectionsHidden(self, this: &mut QHeaderView) -> i32;
}

// proto: bool QHeaderView::sectionsHidden();
impl<'a> /*trait*/ QHeaderView_sectionsHidden for () {
  fn sectionsHidden(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView14sectionsHiddenEv()};
    unsafe {_ZNK11QHeaderView14sectionsHiddenEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn minimumSectionSize<T: QHeaderView_minimumSectionSize>(&mut self, value: T) -> i32 {
    value.minimumSectionSize(self);
    return 1;
  }
}

pub trait QHeaderView_minimumSectionSize {
  fn minimumSectionSize(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::minimumSectionSize();
impl<'a> /*trait*/ QHeaderView_minimumSectionSize for () {
  fn minimumSectionSize(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18minimumSectionSizeEv()};
    unsafe {_ZNK11QHeaderView18minimumSectionSizeEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setCascadingSectionResizes<T: QHeaderView_setCascadingSectionResizes>(&mut self, value: T) -> i32 {
    value.setCascadingSectionResizes(self);
    return 1;
  }
}

pub trait QHeaderView_setCascadingSectionResizes {
  fn setCascadingSectionResizes(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setCascadingSectionResizes(bool enable);
impl<'a> /*trait*/ QHeaderView_setCascadingSectionResizes for (i8) {
  fn setCascadingSectionResizes(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView26setCascadingSectionResizesEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QHeaderView26setCascadingSectionResizesEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setDefaultSectionSize<T: QHeaderView_setDefaultSectionSize>(&mut self, value: T) -> i32 {
    value.setDefaultSectionSize(self);
    return 1;
  }
}

pub trait QHeaderView_setDefaultSectionSize {
  fn setDefaultSectionSize(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setDefaultSectionSize(int size);
impl<'a> /*trait*/ QHeaderView_setDefaultSectionSize for (i32) {
  fn setDefaultSectionSize(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setDefaultSectionSizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QHeaderView21setDefaultSectionSizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionEntered<T: QHeaderView_sectionEntered>(&mut self, value: T) -> i32 {
    value.sectionEntered(self);
    return 1;
  }
}

pub trait QHeaderView_sectionEntered {
  fn sectionEntered(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::sectionEntered(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionEntered for (i32) {
  fn sectionEntered(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView14sectionEnteredEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QHeaderView14sectionEnteredEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn moveSection<T: QHeaderView_moveSection>(&mut self, value: T) -> i32 {
    value.moveSection(self);
    return 1;
  }
}

pub trait QHeaderView_moveSection {
  fn moveSection(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::moveSection(int from, int to);
impl<'a> /*trait*/ QHeaderView_moveSection for (i32, i32) {
  fn moveSection(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView11moveSectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QHeaderView11moveSectionEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn stretchLastSection<T: QHeaderView_stretchLastSection>(&mut self, value: T) -> i32 {
    value.stretchLastSection(self);
    return 1;
  }
}

pub trait QHeaderView_stretchLastSection {
  fn stretchLastSection(self, this: &mut QHeaderView) -> i32;
}

// proto: bool QHeaderView::stretchLastSection();
impl<'a> /*trait*/ QHeaderView_stretchLastSection for () {
  fn stretchLastSection(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView18stretchLastSectionEv()};
    unsafe {_ZNK11QHeaderView18stretchLastSectionEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionSizeHint<T: QHeaderView_sectionSizeHint>(&mut self, value: T) -> i32 {
    value.sectionSizeHint(self);
    return 1;
  }
}

pub trait QHeaderView_sectionSizeHint {
  fn sectionSizeHint(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::sectionSizeHint(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionSizeHint for (i32) {
  fn sectionSizeHint(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView15sectionSizeHintEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QHeaderView15sectionSizeHintEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionsMovable<T: QHeaderView_sectionsMovable>(&mut self, value: T) -> i32 {
    value.sectionsMovable(self);
    return 1;
  }
}

pub trait QHeaderView_sectionsMovable {
  fn sectionsMovable(self, this: &mut QHeaderView) -> i32;
}

// proto: bool QHeaderView::sectionsMovable();
impl<'a> /*trait*/ QHeaderView_sectionsMovable for () {
  fn sectionsMovable(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView15sectionsMovableEv()};
    unsafe {_ZNK11QHeaderView15sectionsMovableEv()};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn isSectionHidden<T: QHeaderView_isSectionHidden>(&mut self, value: T) -> i32 {
    value.isSectionHidden(self);
    return 1;
  }
}

pub trait QHeaderView_isSectionHidden {
  fn isSectionHidden(self, this: &mut QHeaderView) -> i32;
}

// proto: bool QHeaderView::isSectionHidden(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_isSectionHidden for (i32) {
  fn isSectionHidden(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView15isSectionHiddenEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QHeaderView15isSectionHiddenEi(arg0)};
    return 1;
  }
}

// proto: int QHeaderView::logicalIndexAt(int x, int y);
impl<'a> /*trait*/ QHeaderView_logicalIndexAt for (i32, i32) {
  fn logicalIndexAt(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView14logicalIndexAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK11QHeaderView14logicalIndexAtEii(arg0, arg1)};
    return 1;
  }
}

// proto: int QHeaderView::logicalIndexAt(int position);
impl<'a> /*trait*/ QHeaderView_logicalIndexAt for (i32) {
  fn logicalIndexAt(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView14logicalIndexAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QHeaderView14logicalIndexAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn logicalIndex<T: QHeaderView_logicalIndex>(&mut self, value: T) -> i32 {
    value.logicalIndex(self);
    return 1;
  }
}

pub trait QHeaderView_logicalIndex {
  fn logicalIndex(self, this: &mut QHeaderView) -> i32;
}

// proto: int QHeaderView::logicalIndex(int visualIndex);
impl<'a> /*trait*/ QHeaderView_logicalIndex for (i32) {
  fn logicalIndex(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView12logicalIndexEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QHeaderView12logicalIndexEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setMaximumSectionSize<T: QHeaderView_setMaximumSectionSize>(&mut self, value: T) -> i32 {
    value.setMaximumSectionSize(self);
    return 1;
  }
}

pub trait QHeaderView_setMaximumSectionSize {
  fn setMaximumSectionSize(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setMaximumSectionSize(int size);
impl<'a> /*trait*/ QHeaderView_setMaximumSectionSize for (i32) {
  fn setMaximumSectionSize(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView21setMaximumSectionSizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QHeaderView21setMaximumSectionSizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setHighlightSections<T: QHeaderView_setHighlightSections>(&mut self, value: T) -> i32 {
    value.setHighlightSections(self);
    return 1;
  }
}

pub trait QHeaderView_setHighlightSections {
  fn setHighlightSections(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setHighlightSections(bool highlight);
impl<'a> /*trait*/ QHeaderView_setHighlightSections for (i8) {
  fn setHighlightSections(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView20setHighlightSectionsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QHeaderView20setHighlightSectionsEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn setSectionHidden<T: QHeaderView_setSectionHidden>(&mut self, value: T) -> i32 {
    value.setSectionHidden(self);
    return 1;
  }
}

pub trait QHeaderView_setSectionHidden {
  fn setSectionHidden(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::setSectionHidden(int logicalIndex, bool hide);
impl<'a> /*trait*/ QHeaderView_setSectionHidden for (i32, i8) {
  fn setSectionHidden(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView16setSectionHiddenEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN11QHeaderView16setSectionHiddenEib(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn resizeSection<T: QHeaderView_resizeSection>(&mut self, value: T) -> i32 {
    value.resizeSection(self);
    return 1;
  }
}

pub trait QHeaderView_resizeSection {
  fn resizeSection(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::resizeSection(int logicalIndex, int size);
impl<'a> /*trait*/ QHeaderView_resizeSection for (i32, i32) {
  fn resizeSection(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView13resizeSectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QHeaderView13resizeSectionEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn restoreState<T: QHeaderView_restoreState>(&mut self, value: T) -> i32 {
    value.restoreState(self);
    return 1;
  }
}

pub trait QHeaderView_restoreState {
  fn restoreState(self, this: &mut QHeaderView) -> i32;
}

// proto: bool QHeaderView::restoreState(const QByteArray & state);
impl<'a> /*trait*/ QHeaderView_restoreState for (&'a  QByteArray) {
  fn restoreState(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView12restoreStateERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QHeaderView12restoreStateERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionDoubleClicked<T: QHeaderView_sectionDoubleClicked>(&mut self, value: T) -> i32 {
    value.sectionDoubleClicked(self);
    return 1;
  }
}

pub trait QHeaderView_sectionDoubleClicked {
  fn sectionDoubleClicked(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::sectionDoubleClicked(int logicalIndex);
impl<'a> /*trait*/ QHeaderView_sectionDoubleClicked for (i32) {
  fn sectionDoubleClicked(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView20sectionDoubleClickedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QHeaderView20sectionDoubleClickedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn sectionCountChanged<T: QHeaderView_sectionCountChanged>(&mut self, value: T) -> i32 {
    value.sectionCountChanged(self);
    return 1;
  }
}

pub trait QHeaderView_sectionCountChanged {
  fn sectionCountChanged(self, this: &mut QHeaderView) -> i32;
}

// proto: void QHeaderView::sectionCountChanged(int oldCount, int newCount);
impl<'a> /*trait*/ QHeaderView_sectionCountChanged for (i32, i32) {
  fn sectionCountChanged(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHeaderView19sectionCountChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QHeaderView19sectionCountChangedEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QHeaderView {
  pub fn isSortIndicatorShown<T: QHeaderView_isSortIndicatorShown>(&mut self, value: T) -> i32 {
    value.isSortIndicatorShown(self);
    return 1;
  }
}

pub trait QHeaderView_isSortIndicatorShown {
  fn isSortIndicatorShown(self, this: &mut QHeaderView) -> i32;
}

// proto: bool QHeaderView::isSortIndicatorShown();
impl<'a> /*trait*/ QHeaderView_isSortIndicatorShown for () {
  fn isSortIndicatorShown(self, this: &mut QHeaderView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHeaderView20isSortIndicatorShownEv()};
    unsafe {_ZNK11QHeaderView20isSortIndicatorShownEv()};
    return 1;
  }
}

