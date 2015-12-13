// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qpointf::QPointF;
use super::qpaintdevice::QPaintDevice;
use super::qrectf::QRectF;
use super::qpixmap::QPixmap;
use super::qline::QLine;
use super::qpainterpath::QPainterPath;
use super::qlinef::QLineF;
use super::qpaintenginestate::QPaintEngineState;
use super::qregion::QRegion;
use super::qtextitem::QTextItem;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QPaintEngine::drawEllipse(const QRect & r);
  fn _ZN12QPaintEngine11drawEllipseERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QPaintEngine::NewQPaintEngine(const QPaintEngine & );
  fn _ZN12QPaintEngineC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QPaintEngine::isActive();
  fn _ZNK12QPaintEngine8isActiveEv() -> i32;
  // proto: void QPaintEngine::drawPoints(const QPointF * points, int pointCount);
  fn _ZN12QPaintEngine10drawPointsEPK7QPointFi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: QPoint QPaintEngine::coordinateOffset();
  fn _ZNK12QPaintEngine16coordinateOffsetEv() -> i32;
  // proto: void QPaintEngine::setPaintDevice(QPaintDevice * device);
  fn _ZN12QPaintEngine14setPaintDeviceEP12QPaintDevice(arg0: *mut c_void) -> i32;
  // proto: void QPaintEngine::setSystemRect(const QRect & rect);
  fn _ZN12QPaintEngine13setSystemRectERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QPaintEngine::FreeQPaintEngine();
  fn _ZN12QPaintEngineD0Ev() -> i32;
  // proto: bool QPaintEngine::end();
  fn _ZN12QPaintEngine3endEv() -> i32;
  // proto: void QPaintEngine::drawTiledPixmap(const QRectF & r, const QPixmap & pixmap, const QPointF & s);
  fn _ZN12QPaintEngine15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QPaintEngine::setActive(bool newState);
  fn _ZN12QPaintEngine9setActiveEb(arg0: int8_t) -> i32;
  // proto: void QPaintEngine::drawPixmap(const QRectF & r, const QPixmap & pm, const QRectF & sr);
  fn _ZN12QPaintEngine10drawPixmapERK6QRectFRK7QPixmapS2_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QPaintEngine::drawLines(const QLine * lines, int lineCount);
  fn _ZN12QPaintEngine9drawLinesEPK5QLinei(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPaintEngine::drawPath(const QPainterPath & path);
  fn _ZN12QPaintEngine8drawPathERK12QPainterPath(arg0: *const c_void) -> i32;
  // proto: void QPaintEngine::drawLines(const QLineF * lines, int lineCount);
  fn _ZN12QPaintEngine9drawLinesEPK6QLineFi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPaintEngine::updateState(const QPaintEngineState & state);
  fn _ZN12QPaintEngine11updateStateERK17QPaintEngineState(arg0: *const c_void) -> i32;
  // proto: bool QPaintEngine::begin(QPaintDevice * pdev);
  fn _ZN12QPaintEngine5beginEP12QPaintDevice(arg0: *mut c_void) -> i32;
  // proto: QRect QPaintEngine::systemRect();
  fn _ZNK12QPaintEngine10systemRectEv() -> i32;
  // proto: void QPaintEngine::drawRects(const QRectF * rects, int rectCount);
  fn _ZN12QPaintEngine9drawRectsEPK6QRectFi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPaintEngine::setSystemClip(const QRegion & baseClip);
  fn _ZN12QPaintEngine13setSystemClipERK7QRegion(arg0: *const c_void) -> i32;
  // proto: QRegion QPaintEngine::systemClip();
  fn _ZNK12QPaintEngine10systemClipEv() -> i32;
  // proto: QPaintDevice * QPaintEngine::paintDevice();
  fn _ZNK12QPaintEngine11paintDeviceEv() -> i32;
  // proto: void QPaintEngine::syncState();
  fn _ZN12QPaintEngine9syncStateEv() -> i32;
  // proto: QPainter * QPaintEngine::painter();
  fn _ZNK12QPaintEngine7painterEv() -> i32;
  // proto: void QPaintEngine::drawEllipse(const QRectF & r);
  fn _ZN12QPaintEngine11drawEllipseERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QPaintEngine::drawTextItem(const QPointF & p, const QTextItem & textItem);
  fn _ZN12QPaintEngine12drawTextItemERK7QPointFRK9QTextItem(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPaintEngine::fix_neg_rect(int * x, int * y, int * w, int * h);
  fn _ZN12QPaintEngine12fix_neg_rectEPiS0_S0_S0_(arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) -> i32;
  // proto: bool QPaintEngine::isExtended();
  fn _ZNK12QPaintEngine10isExtendedEv() -> i32;
  // proto: void QPaintEngine::drawRects(const QRect * rects, int rectCount);
  fn _ZN12QPaintEngine9drawRectsEPK5QRecti(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPaintEngine::drawPoints(const QPoint * points, int pointCount);
  fn _ZN12QPaintEngine10drawPointsEPK6QPointi(arg0: *const c_void, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QPaintEngine)=1
pub struct QPaintEngine {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPaintEngine {
  pub fn drawEllipse<T: QPaintEngine_drawEllipse>(&mut self, value: T) -> i32 {
    value.drawEllipse(self);
    return 1;
  }
}

pub trait QPaintEngine_drawEllipse {
  fn drawEllipse(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::drawEllipse(const QRect & r);
impl<'a> /*trait*/ QPaintEngine_drawEllipse for (&'a  QRect) {
  fn drawEllipse(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine11drawEllipseERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QPaintEngine11drawEllipseERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn NewQPaintEngine<T: QPaintEngine_NewQPaintEngine>(value: T) -> QPaintEngine {
    let rsthis = value.NewQPaintEngine();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintEngine_NewQPaintEngine {
  fn NewQPaintEngine(self) -> QPaintEngine;
}

// proto: void QPaintEngine::NewQPaintEngine(const QPaintEngine & );
impl<'a> /*trait*/ QPaintEngine_NewQPaintEngine for (&'a  QPaintEngine) {
  fn NewQPaintEngine(self) -> QPaintEngine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngineC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QPaintEngineC1ERKS_(qthis, arg0)};
    let rsthis = QPaintEngine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn isActive<T: QPaintEngine_isActive>(&mut self, value: T) -> i32 {
    value.isActive(self);
    return 1;
  }
}

pub trait QPaintEngine_isActive {
  fn isActive(self, this: &mut QPaintEngine) -> i32;
}

// proto: bool QPaintEngine::isActive();
impl<'a> /*trait*/ QPaintEngine_isActive for () {
  fn isActive(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine8isActiveEv()};
    unsafe {_ZNK12QPaintEngine8isActiveEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn drawPoints<T: QPaintEngine_drawPoints>(&mut self, value: T) -> i32 {
    value.drawPoints(self);
    return 1;
  }
}

pub trait QPaintEngine_drawPoints {
  fn drawPoints(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::drawPoints(const QPointF * points, int pointCount);
impl<'a> /*trait*/ QPaintEngine_drawPoints for (&'a  QPointF, i32) {
  fn drawPoints(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine10drawPointsEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QPaintEngine10drawPointsEPK7QPointFi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn coordinateOffset<T: QPaintEngine_coordinateOffset>(&mut self, value: T) -> i32 {
    value.coordinateOffset(self);
    return 1;
  }
}

pub trait QPaintEngine_coordinateOffset {
  fn coordinateOffset(self, this: &mut QPaintEngine) -> i32;
}

// proto: QPoint QPaintEngine::coordinateOffset();
impl<'a> /*trait*/ QPaintEngine_coordinateOffset for () {
  fn coordinateOffset(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine16coordinateOffsetEv()};
    unsafe {_ZNK12QPaintEngine16coordinateOffsetEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn setPaintDevice<T: QPaintEngine_setPaintDevice>(&mut self, value: T) -> i32 {
    value.setPaintDevice(self);
    return 1;
  }
}

pub trait QPaintEngine_setPaintDevice {
  fn setPaintDevice(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::setPaintDevice(QPaintDevice * device);
impl<'a> /*trait*/ QPaintEngine_setPaintDevice for (&'a mut QPaintDevice) {
  fn setPaintDevice(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine14setPaintDeviceEP12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QPaintEngine14setPaintDeviceEP12QPaintDevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn setSystemRect<T: QPaintEngine_setSystemRect>(&mut self, value: T) -> i32 {
    value.setSystemRect(self);
    return 1;
  }
}

pub trait QPaintEngine_setSystemRect {
  fn setSystemRect(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::setSystemRect(const QRect & rect);
impl<'a> /*trait*/ QPaintEngine_setSystemRect for (&'a  QRect) {
  fn setSystemRect(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine13setSystemRectERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QPaintEngine13setSystemRectERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn FreeQPaintEngine<T: QPaintEngine_FreeQPaintEngine>(&mut self, value: T) -> i32 {
    value.FreeQPaintEngine(self);
    return 1;
  }
}

pub trait QPaintEngine_FreeQPaintEngine {
  fn FreeQPaintEngine(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::FreeQPaintEngine();
impl<'a> /*trait*/ QPaintEngine_FreeQPaintEngine for () {
  fn FreeQPaintEngine(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngineD0Ev()};
    unsafe {_ZN12QPaintEngineD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn end<T: QPaintEngine_end>(&mut self, value: T) -> i32 {
    value.end(self);
    return 1;
  }
}

pub trait QPaintEngine_end {
  fn end(self, this: &mut QPaintEngine) -> i32;
}

// proto: bool QPaintEngine::end();
impl<'a> /*trait*/ QPaintEngine_end for () {
  fn end(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine3endEv()};
    unsafe {_ZN12QPaintEngine3endEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn drawTiledPixmap<T: QPaintEngine_drawTiledPixmap>(&mut self, value: T) -> i32 {
    value.drawTiledPixmap(self);
    return 1;
  }
}

pub trait QPaintEngine_drawTiledPixmap {
  fn drawTiledPixmap(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::drawTiledPixmap(const QRectF & r, const QPixmap & pixmap, const QPointF & s);
impl<'a> /*trait*/ QPaintEngine_drawTiledPixmap for (&'a  QRectF, &'a  QPixmap, &'a  QPointF) {
  fn drawTiledPixmap(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN12QPaintEngine15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn setActive<T: QPaintEngine_setActive>(&mut self, value: T) -> i32 {
    value.setActive(self);
    return 1;
  }
}

pub trait QPaintEngine_setActive {
  fn setActive(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::setActive(bool newState);
impl<'a> /*trait*/ QPaintEngine_setActive for (i8) {
  fn setActive(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9setActiveEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QPaintEngine9setActiveEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn drawPixmap<T: QPaintEngine_drawPixmap>(&mut self, value: T) -> i32 {
    value.drawPixmap(self);
    return 1;
  }
}

pub trait QPaintEngine_drawPixmap {
  fn drawPixmap(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::drawPixmap(const QRectF & r, const QPixmap & pm, const QRectF & sr);
impl<'a> /*trait*/ QPaintEngine_drawPixmap for (&'a  QRectF, &'a  QPixmap, &'a  QRectF) {
  fn drawPixmap(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine10drawPixmapERK6QRectFRK7QPixmapS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN12QPaintEngine10drawPixmapERK6QRectFRK7QPixmapS2_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn drawLines<T: QPaintEngine_drawLines>(&mut self, value: T) -> i32 {
    value.drawLines(self);
    return 1;
  }
}

pub trait QPaintEngine_drawLines {
  fn drawLines(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::drawLines(const QLine * lines, int lineCount);
impl<'a> /*trait*/ QPaintEngine_drawLines for (&'a  QLine, i32) {
  fn drawLines(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9drawLinesEPK5QLinei()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QPaintEngine9drawLinesEPK5QLinei(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn drawPath<T: QPaintEngine_drawPath>(&mut self, value: T) -> i32 {
    value.drawPath(self);
    return 1;
  }
}

pub trait QPaintEngine_drawPath {
  fn drawPath(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::drawPath(const QPainterPath & path);
impl<'a> /*trait*/ QPaintEngine_drawPath for (&'a  QPainterPath) {
  fn drawPath(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine8drawPathERK12QPainterPath()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QPaintEngine8drawPathERK12QPainterPath(arg0)};
    return 1;
  }
}

// proto: void QPaintEngine::drawLines(const QLineF * lines, int lineCount);
impl<'a> /*trait*/ QPaintEngine_drawLines for (&'a  QLineF, i32) {
  fn drawLines(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9drawLinesEPK6QLineFi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QPaintEngine9drawLinesEPK6QLineFi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn updateState<T: QPaintEngine_updateState>(&mut self, value: T) -> i32 {
    value.updateState(self);
    return 1;
  }
}

pub trait QPaintEngine_updateState {
  fn updateState(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::updateState(const QPaintEngineState & state);
impl<'a> /*trait*/ QPaintEngine_updateState for (&'a  QPaintEngineState) {
  fn updateState(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine11updateStateERK17QPaintEngineState()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QPaintEngine11updateStateERK17QPaintEngineState(arg0)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn begin<T: QPaintEngine_begin>(&mut self, value: T) -> i32 {
    value.begin(self);
    return 1;
  }
}

pub trait QPaintEngine_begin {
  fn begin(self, this: &mut QPaintEngine) -> i32;
}

// proto: bool QPaintEngine::begin(QPaintDevice * pdev);
impl<'a> /*trait*/ QPaintEngine_begin for (&'a mut QPaintDevice) {
  fn begin(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine5beginEP12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QPaintEngine5beginEP12QPaintDevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn systemRect<T: QPaintEngine_systemRect>(&mut self, value: T) -> i32 {
    value.systemRect(self);
    return 1;
  }
}

pub trait QPaintEngine_systemRect {
  fn systemRect(self, this: &mut QPaintEngine) -> i32;
}

// proto: QRect QPaintEngine::systemRect();
impl<'a> /*trait*/ QPaintEngine_systemRect for () {
  fn systemRect(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine10systemRectEv()};
    unsafe {_ZNK12QPaintEngine10systemRectEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn drawRects<T: QPaintEngine_drawRects>(&mut self, value: T) -> i32 {
    value.drawRects(self);
    return 1;
  }
}

pub trait QPaintEngine_drawRects {
  fn drawRects(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::drawRects(const QRectF * rects, int rectCount);
impl<'a> /*trait*/ QPaintEngine_drawRects for (&'a  QRectF, i32) {
  fn drawRects(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9drawRectsEPK6QRectFi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QPaintEngine9drawRectsEPK6QRectFi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn setSystemClip<T: QPaintEngine_setSystemClip>(&mut self, value: T) -> i32 {
    value.setSystemClip(self);
    return 1;
  }
}

pub trait QPaintEngine_setSystemClip {
  fn setSystemClip(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::setSystemClip(const QRegion & baseClip);
impl<'a> /*trait*/ QPaintEngine_setSystemClip for (&'a  QRegion) {
  fn setSystemClip(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine13setSystemClipERK7QRegion()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QPaintEngine13setSystemClipERK7QRegion(arg0)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn systemClip<T: QPaintEngine_systemClip>(&mut self, value: T) -> i32 {
    value.systemClip(self);
    return 1;
  }
}

pub trait QPaintEngine_systemClip {
  fn systemClip(self, this: &mut QPaintEngine) -> i32;
}

// proto: QRegion QPaintEngine::systemClip();
impl<'a> /*trait*/ QPaintEngine_systemClip for () {
  fn systemClip(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine10systemClipEv()};
    unsafe {_ZNK12QPaintEngine10systemClipEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn paintDevice<T: QPaintEngine_paintDevice>(&mut self, value: T) -> i32 {
    value.paintDevice(self);
    return 1;
  }
}

pub trait QPaintEngine_paintDevice {
  fn paintDevice(self, this: &mut QPaintEngine) -> i32;
}

// proto: QPaintDevice * QPaintEngine::paintDevice();
impl<'a> /*trait*/ QPaintEngine_paintDevice for () {
  fn paintDevice(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine11paintDeviceEv()};
    unsafe {_ZNK12QPaintEngine11paintDeviceEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn syncState<T: QPaintEngine_syncState>(&mut self, value: T) -> i32 {
    value.syncState(self);
    return 1;
  }
}

pub trait QPaintEngine_syncState {
  fn syncState(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::syncState();
impl<'a> /*trait*/ QPaintEngine_syncState for () {
  fn syncState(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9syncStateEv()};
    unsafe {_ZN12QPaintEngine9syncStateEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn painter<T: QPaintEngine_painter>(&mut self, value: T) -> i32 {
    value.painter(self);
    return 1;
  }
}

pub trait QPaintEngine_painter {
  fn painter(self, this: &mut QPaintEngine) -> i32;
}

// proto: QPainter * QPaintEngine::painter();
impl<'a> /*trait*/ QPaintEngine_painter for () {
  fn painter(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine7painterEv()};
    unsafe {_ZNK12QPaintEngine7painterEv()};
    return 1;
  }
}

// proto: void QPaintEngine::drawEllipse(const QRectF & r);
impl<'a> /*trait*/ QPaintEngine_drawEllipse for (&'a  QRectF) {
  fn drawEllipse(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine11drawEllipseERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QPaintEngine11drawEllipseERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn drawTextItem<T: QPaintEngine_drawTextItem>(&mut self, value: T) -> i32 {
    value.drawTextItem(self);
    return 1;
  }
}

pub trait QPaintEngine_drawTextItem {
  fn drawTextItem(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::drawTextItem(const QPointF & p, const QTextItem & textItem);
impl<'a> /*trait*/ QPaintEngine_drawTextItem for (&'a  QPointF, &'a  QTextItem) {
  fn drawTextItem(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine12drawTextItemERK7QPointFRK9QTextItem()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN12QPaintEngine12drawTextItemERK7QPointFRK9QTextItem(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn fix_neg_rect<T: QPaintEngine_fix_neg_rect>(&mut self, value: T) -> i32 {
    value.fix_neg_rect(self);
    return 1;
  }
}

pub trait QPaintEngine_fix_neg_rect {
  fn fix_neg_rect(self, this: &mut QPaintEngine) -> i32;
}

// proto: void QPaintEngine::fix_neg_rect(int * x, int * y, int * w, int * h);
impl<'a> /*trait*/ QPaintEngine_fix_neg_rect for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn fix_neg_rect(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine12fix_neg_rectEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZN12QPaintEngine12fix_neg_rectEPiS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn isExtended<T: QPaintEngine_isExtended>(&mut self, value: T) -> i32 {
    value.isExtended(self);
    return 1;
  }
}

pub trait QPaintEngine_isExtended {
  fn isExtended(self, this: &mut QPaintEngine) -> i32;
}

// proto: bool QPaintEngine::isExtended();
impl<'a> /*trait*/ QPaintEngine_isExtended for () {
  fn isExtended(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine10isExtendedEv()};
    unsafe {_ZNK12QPaintEngine10isExtendedEv()};
    return 1;
  }
}

// proto: void QPaintEngine::drawRects(const QRect * rects, int rectCount);
impl<'a> /*trait*/ QPaintEngine_drawRects for (&'a  QRect, i32) {
  fn drawRects(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9drawRectsEPK5QRecti()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QPaintEngine9drawRectsEPK5QRecti(arg0, arg1)};
    return 1;
  }
}

// proto: void QPaintEngine::drawPoints(const QPoint * points, int pointCount);
impl<'a> /*trait*/ QPaintEngine_drawPoints for (&'a  QPoint, i32) {
  fn drawPoints(self, this: &mut QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine10drawPointsEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QPaintEngine10drawPointsEPK6QPointi(arg0, arg1)};
    return 1;
  }
}

