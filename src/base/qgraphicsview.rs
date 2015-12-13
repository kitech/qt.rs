// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qrectf::QRectF;
use super::qmatrix::QMatrix;
use super::qbrush::QBrush;
use super::qpointf::QPointF;
use super::qpoint::QPoint;
use super::qgraphicsscene::QGraphicsScene;
use super::qwidget::QWidget;
use super::qgraphicsitem::QGraphicsItem;
use super::qpolygon::QPolygon;
use super::qtransform::QTransform;
use super::qpolygonf::QPolygonF;
use super::qpainterpath::QPainterPath;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsView::scale(qreal sx, qreal sy);
  fn _ZN13QGraphicsView5scaleEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QPolygonF QGraphicsView::mapToScene(const QRect & rect);
  fn _ZNK13QGraphicsView10mapToSceneERK5QRect(arg0: *const c_void) -> i32;
  // proto: QPolygon QGraphicsView::mapFromScene(const QRectF & rect);
  fn _ZNK13QGraphicsView12mapFromSceneERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsView::translate(qreal dx, qreal dy);
  fn _ZN13QGraphicsView9translateEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QPointF QGraphicsView::mapToScene(int x, int y);
  fn _ZNK13QGraphicsView10mapToSceneEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: const QMetaObject * QGraphicsView::metaObject();
  fn _ZNK13QGraphicsView10metaObjectEv() -> i32;
  // proto: void QGraphicsView::setSceneRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN13QGraphicsView12setSceneRectEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: QRect QGraphicsView::rubberBandRect();
  fn _ZNK13QGraphicsView14rubberBandRectEv() -> i32;
  // proto: void QGraphicsView::setMatrix(const QMatrix & matrix, bool combine);
  fn _ZN13QGraphicsView9setMatrixERK7QMatrixb(arg0: *const c_void, arg1: int8_t) -> i32;
  // proto: bool QGraphicsView::isInteractive();
  fn _ZNK13QGraphicsView13isInteractiveEv() -> i32;
  // proto: void QGraphicsView::NewQGraphicsView(const QGraphicsView & );
  fn _ZN13QGraphicsViewC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QGraphicsView::setBackgroundBrush(const QBrush & brush);
  fn _ZN13QGraphicsView18setBackgroundBrushERK6QBrush(arg0: *const c_void) -> i32;
  // proto: bool QGraphicsView::isTransformed();
  fn _ZNK13QGraphicsView13isTransformedEv() -> i32;
  // proto: QGraphicsItem * QGraphicsView::itemAt(int x, int y);
  fn _ZNK13QGraphicsView6itemAtEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QGraphicsView::centerOn(const QPointF & pos);
  fn _ZN13QGraphicsView8centerOnERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsView::setForegroundBrush(const QBrush & brush);
  fn _ZN13QGraphicsView18setForegroundBrushERK6QBrush(arg0: *const c_void) -> i32;
  // proto: void QGraphicsView::shear(qreal sh, qreal sv);
  fn _ZN13QGraphicsView5shearEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QBrush QGraphicsView::foregroundBrush();
  fn _ZNK13QGraphicsView15foregroundBrushEv() -> i32;
  // proto: QGraphicsItem * QGraphicsView::itemAt(const QPoint & pos);
  fn _ZNK13QGraphicsView6itemAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QGraphicsView::updateSceneRect(const QRectF & rect);
  fn _ZN13QGraphicsView15updateSceneRectERK6QRectF(arg0: *const c_void) -> i32;
  // proto: QPointF QGraphicsView::mapToScene(const QPoint & point);
  fn _ZNK13QGraphicsView10mapToSceneERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QGraphicsView::setInteractive(bool allowed);
  fn _ZN13QGraphicsView14setInteractiveEb(arg0: int8_t) -> i32;
  // proto: QMatrix QGraphicsView::matrix();
  fn _ZNK13QGraphicsView6matrixEv() -> i32;
  // proto: QPolygonF QGraphicsView::mapToScene(int x, int y, int w, int h);
  fn _ZNK13QGraphicsView10mapToSceneEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QGraphicsView::NewQGraphicsView(QGraphicsScene * scene, QWidget * parent);
  fn _ZN13QGraphicsViewC1EP14QGraphicsSceneP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: void QGraphicsView::NewQGraphicsView(QWidget * parent);
  fn _ZN13QGraphicsViewC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QGraphicsView::centerOn(const QGraphicsItem * item);
  fn _ZN13QGraphicsView8centerOnEPK13QGraphicsItem(arg0: *const c_void) -> i32;
  // proto: QTransform QGraphicsView::viewportTransform();
  fn _ZNK13QGraphicsView17viewportTransformEv() -> i32;
  // proto: void QGraphicsView::resetCachedContent();
  fn _ZN13QGraphicsView18resetCachedContentEv() -> i32;
  // proto: QPolygonF QGraphicsView::mapToScene(const QPolygon & polygon);
  fn _ZNK13QGraphicsView10mapToSceneERK8QPolygon(arg0: *const c_void) -> i32;
  // proto: void QGraphicsView::ensureVisible(const QGraphicsItem * item, int xmargin, int ymargin);
  fn _ZN13QGraphicsView13ensureVisibleEPK13QGraphicsItemii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: QRectF QGraphicsView::sceneRect();
  fn _ZNK13QGraphicsView9sceneRectEv() -> i32;
  // proto: QGraphicsScene * QGraphicsView::scene();
  fn _ZNK13QGraphicsView5sceneEv() -> i32;
  // proto: QSize QGraphicsView::sizeHint();
  fn _ZNK13QGraphicsView8sizeHintEv() -> i32;
  // proto: QBrush QGraphicsView::backgroundBrush();
  fn _ZNK13QGraphicsView15backgroundBrushEv() -> i32;
  // proto: QPoint QGraphicsView::mapFromScene(qreal x, qreal y);
  fn _ZNK13QGraphicsView12mapFromSceneEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QGraphicsView::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
  fn _ZN13QGraphicsView13ensureVisibleERK6QRectFii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: QList<QGraphicsItem *> QGraphicsView::items();
  fn _ZNK13QGraphicsView5itemsEv() -> i32;
  // proto: QTransform QGraphicsView::transform();
  fn _ZNK13QGraphicsView9transformEv() -> i32;
  // proto: QList<QGraphicsItem *> QGraphicsView::items(int x, int y);
  fn _ZNK13QGraphicsView5itemsEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QGraphicsView::centerOn(qreal x, qreal y);
  fn _ZN13QGraphicsView8centerOnEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QGraphicsView::rubberBandChanged(QRect viewportRect, QPointF fromScenePoint, QPointF toScenePoint);
  fn _ZN13QGraphicsView17rubberBandChangedE5QRect7QPointFS1_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> i32;
  // proto: void QGraphicsView::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
  fn _ZN13QGraphicsView13ensureVisibleEddddii(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_int, arg5: c_int) -> i32;
  // proto: void QGraphicsView::rotate(qreal angle);
  fn _ZN13QGraphicsView6rotateEd(arg0: c_double) -> i32;
  // proto: QPolygon QGraphicsView::mapFromScene(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsView12mapFromSceneEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsView::setTransform(const QTransform & matrix, bool combine);
  fn _ZN13QGraphicsView12setTransformERK10QTransformb(arg0: *const c_void, arg1: int8_t) -> i32;
  // proto: void QGraphicsView::setSceneRect(const QRectF & rect);
  fn _ZN13QGraphicsView12setSceneRectERK6QRectF(arg0: *const c_void) -> i32;
  // proto: QPolygon QGraphicsView::mapFromScene(const QPolygonF & polygon);
  fn _ZNK13QGraphicsView12mapFromSceneERK9QPolygonF(arg0: *const c_void) -> i32;
  // proto: QPoint QGraphicsView::mapFromScene(const QPointF & point);
  fn _ZNK13QGraphicsView12mapFromSceneERK7QPointF(arg0: *const c_void) -> i32;
  // proto: QPainterPath QGraphicsView::mapFromScene(const QPainterPath & path);
  fn _ZNK13QGraphicsView12mapFromSceneERK12QPainterPath(arg0: *const c_void) -> i32;
  // proto: void QGraphicsView::FreeQGraphicsView();
  fn _ZN13QGraphicsViewD0Ev() -> i32;
  // proto: QList<QGraphicsItem *> QGraphicsView::items(const QPoint & pos);
  fn _ZNK13QGraphicsView5itemsERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QGraphicsView::resetMatrix();
  fn _ZN13QGraphicsView11resetMatrixEv() -> i32;
  // proto: void QGraphicsView::resetTransform();
  fn _ZN13QGraphicsView14resetTransformEv() -> i32;
  // proto: QPainterPath QGraphicsView::mapToScene(const QPainterPath & path);
  fn _ZNK13QGraphicsView10mapToSceneERK12QPainterPath(arg0: *const c_void) -> i32;
  // proto: void QGraphicsView::setScene(QGraphicsScene * scene);
  fn _ZN13QGraphicsView8setSceneEP14QGraphicsScene(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QGraphicsView)=1
pub struct QGraphicsView {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsView {
  pub fn scale<T: QGraphicsView_scale>(&mut self, value: T) -> i32 {
    value.scale(self);
    return 1;
  }
}

pub trait QGraphicsView_scale {
  fn scale(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::scale(qreal sx, qreal sy);
impl<'a> /*trait*/ QGraphicsView_scale for (f64, f64) {
  fn scale(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView5scaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN13QGraphicsView5scaleEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn mapToScene<T: QGraphicsView_mapToScene>(&mut self, value: T) -> i32 {
    value.mapToScene(self);
    return 1;
  }
}

pub trait QGraphicsView_mapToScene {
  fn mapToScene(self, this: &mut QGraphicsView) -> i32;
}

// proto: QPolygonF QGraphicsView::mapToScene(const QRect & rect);
impl<'a> /*trait*/ QGraphicsView_mapToScene for (&'a  QRect) {
  fn mapToScene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsView10mapToSceneERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn mapFromScene<T: QGraphicsView_mapFromScene>(&mut self, value: T) -> i32 {
    value.mapFromScene(self);
    return 1;
  }
}

pub trait QGraphicsView_mapFromScene {
  fn mapFromScene(self, this: &mut QGraphicsView) -> i32;
}

// proto: QPolygon QGraphicsView::mapFromScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsView_mapFromScene for (&'a  QRectF) {
  fn mapFromScene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsView12mapFromSceneERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn translate<T: QGraphicsView_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QGraphicsView_translate {
  fn translate(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsView_translate for (f64, f64) {
  fn translate(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN13QGraphicsView9translateEdd(arg0, arg1)};
    return 1;
  }
}

// proto: QPointF QGraphicsView::mapToScene(int x, int y);
impl<'a> /*trait*/ QGraphicsView_mapToScene for (i32, i32) {
  fn mapToScene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK13QGraphicsView10mapToSceneEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn metaObject<T: QGraphicsView_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGraphicsView_metaObject {
  fn metaObject(self, this: &mut QGraphicsView) -> i32;
}

// proto: const QMetaObject * QGraphicsView::metaObject();
impl<'a> /*trait*/ QGraphicsView_metaObject for () {
  fn metaObject(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10metaObjectEv()};
    unsafe {_ZNK13QGraphicsView10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setSceneRect<T: QGraphicsView_setSceneRect>(&mut self, value: T) -> i32 {
    value.setSceneRect(self);
    return 1;
  }
}

pub trait QGraphicsView_setSceneRect {
  fn setSceneRect(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::setSceneRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsView_setSceneRect for (f64, f64, f64, f64) {
  fn setSceneRect(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView12setSceneRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN13QGraphicsView12setSceneRectEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn rubberBandRect<T: QGraphicsView_rubberBandRect>(&mut self, value: T) -> i32 {
    value.rubberBandRect(self);
    return 1;
  }
}

pub trait QGraphicsView_rubberBandRect {
  fn rubberBandRect(self, this: &mut QGraphicsView) -> i32;
}

// proto: QRect QGraphicsView::rubberBandRect();
impl<'a> /*trait*/ QGraphicsView_rubberBandRect for () {
  fn rubberBandRect(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView14rubberBandRectEv()};
    unsafe {_ZNK13QGraphicsView14rubberBandRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setMatrix<T: QGraphicsView_setMatrix>(&mut self, value: T) -> i32 {
    value.setMatrix(self);
    return 1;
  }
}

pub trait QGraphicsView_setMatrix {
  fn setMatrix(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::setMatrix(const QMatrix & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsView_setMatrix for (&'a  QMatrix, i8) {
  fn setMatrix(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView9setMatrixERK7QMatrixb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN13QGraphicsView9setMatrixERK7QMatrixb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn isInteractive<T: QGraphicsView_isInteractive>(&mut self, value: T) -> i32 {
    value.isInteractive(self);
    return 1;
  }
}

pub trait QGraphicsView_isInteractive {
  fn isInteractive(self, this: &mut QGraphicsView) -> i32;
}

// proto: bool QGraphicsView::isInteractive();
impl<'a> /*trait*/ QGraphicsView_isInteractive for () {
  fn isInteractive(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView13isInteractiveEv()};
    unsafe {_ZNK13QGraphicsView13isInteractiveEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn NewQGraphicsView<T: QGraphicsView_NewQGraphicsView>(value: T) -> QGraphicsView {
    let rsthis = value.NewQGraphicsView();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsView_NewQGraphicsView {
  fn NewQGraphicsView(self) -> QGraphicsView;
}

// proto: void QGraphicsView::NewQGraphicsView(const QGraphicsView & );
impl<'a> /*trait*/ QGraphicsView_NewQGraphicsView for (&'a  QGraphicsView) {
  fn NewQGraphicsView(self) -> QGraphicsView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsViewC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsViewC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setBackgroundBrush<T: QGraphicsView_setBackgroundBrush>(&mut self, value: T) -> i32 {
    value.setBackgroundBrush(self);
    return 1;
  }
}

pub trait QGraphicsView_setBackgroundBrush {
  fn setBackgroundBrush(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::setBackgroundBrush(const QBrush & brush);
impl<'a> /*trait*/ QGraphicsView_setBackgroundBrush for (&'a  QBrush) {
  fn setBackgroundBrush(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView18setBackgroundBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsView18setBackgroundBrushERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn isTransformed<T: QGraphicsView_isTransformed>(&mut self, value: T) -> i32 {
    value.isTransformed(self);
    return 1;
  }
}

pub trait QGraphicsView_isTransformed {
  fn isTransformed(self, this: &mut QGraphicsView) -> i32;
}

// proto: bool QGraphicsView::isTransformed();
impl<'a> /*trait*/ QGraphicsView_isTransformed for () {
  fn isTransformed(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView13isTransformedEv()};
    unsafe {_ZNK13QGraphicsView13isTransformedEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn itemAt<T: QGraphicsView_itemAt>(&mut self, value: T) -> i32 {
    value.itemAt(self);
    return 1;
  }
}

pub trait QGraphicsView_itemAt {
  fn itemAt(self, this: &mut QGraphicsView) -> i32;
}

// proto: QGraphicsItem * QGraphicsView::itemAt(int x, int y);
impl<'a> /*trait*/ QGraphicsView_itemAt for (i32, i32) {
  fn itemAt(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK13QGraphicsView6itemAtEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn centerOn<T: QGraphicsView_centerOn>(&mut self, value: T) -> i32 {
    value.centerOn(self);
    return 1;
  }
}

pub trait QGraphicsView_centerOn {
  fn centerOn(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::centerOn(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsView_centerOn for (&'a  QPointF) {
  fn centerOn(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView8centerOnERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsView8centerOnERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setForegroundBrush<T: QGraphicsView_setForegroundBrush>(&mut self, value: T) -> i32 {
    value.setForegroundBrush(self);
    return 1;
  }
}

pub trait QGraphicsView_setForegroundBrush {
  fn setForegroundBrush(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::setForegroundBrush(const QBrush & brush);
impl<'a> /*trait*/ QGraphicsView_setForegroundBrush for (&'a  QBrush) {
  fn setForegroundBrush(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView18setForegroundBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsView18setForegroundBrushERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn shear<T: QGraphicsView_shear>(&mut self, value: T) -> i32 {
    value.shear(self);
    return 1;
  }
}

pub trait QGraphicsView_shear {
  fn shear(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::shear(qreal sh, qreal sv);
impl<'a> /*trait*/ QGraphicsView_shear for (f64, f64) {
  fn shear(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView5shearEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN13QGraphicsView5shearEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn foregroundBrush<T: QGraphicsView_foregroundBrush>(&mut self, value: T) -> i32 {
    value.foregroundBrush(self);
    return 1;
  }
}

pub trait QGraphicsView_foregroundBrush {
  fn foregroundBrush(self, this: &mut QGraphicsView) -> i32;
}

// proto: QBrush QGraphicsView::foregroundBrush();
impl<'a> /*trait*/ QGraphicsView_foregroundBrush for () {
  fn foregroundBrush(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView15foregroundBrushEv()};
    unsafe {_ZNK13QGraphicsView15foregroundBrushEv()};
    return 1;
  }
}

// proto: QGraphicsItem * QGraphicsView::itemAt(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsView_itemAt for (&'a  QPoint) {
  fn itemAt(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView6itemAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsView6itemAtERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn updateSceneRect<T: QGraphicsView_updateSceneRect>(&mut self, value: T) -> i32 {
    value.updateSceneRect(self);
    return 1;
  }
}

pub trait QGraphicsView_updateSceneRect {
  fn updateSceneRect(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::updateSceneRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsView_updateSceneRect for (&'a  QRectF) {
  fn updateSceneRect(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView15updateSceneRectERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsView15updateSceneRectERK6QRectF(arg0)};
    return 1;
  }
}

// proto: QPointF QGraphicsView::mapToScene(const QPoint & point);
impl<'a> /*trait*/ QGraphicsView_mapToScene for (&'a  QPoint) {
  fn mapToScene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsView10mapToSceneERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setInteractive<T: QGraphicsView_setInteractive>(&mut self, value: T) -> i32 {
    value.setInteractive(self);
    return 1;
  }
}

pub trait QGraphicsView_setInteractive {
  fn setInteractive(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::setInteractive(bool allowed);
impl<'a> /*trait*/ QGraphicsView_setInteractive for (i8) {
  fn setInteractive(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView14setInteractiveEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QGraphicsView14setInteractiveEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn matrix<T: QGraphicsView_matrix>(&mut self, value: T) -> i32 {
    value.matrix(self);
    return 1;
  }
}

pub trait QGraphicsView_matrix {
  fn matrix(self, this: &mut QGraphicsView) -> i32;
}

// proto: QMatrix QGraphicsView::matrix();
impl<'a> /*trait*/ QGraphicsView_matrix for () {
  fn matrix(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView6matrixEv()};
    unsafe {_ZNK13QGraphicsView6matrixEv()};
    return 1;
  }
}

// proto: QPolygonF QGraphicsView::mapToScene(int x, int y, int w, int h);
impl<'a> /*trait*/ QGraphicsView_mapToScene for (i32, i32, i32, i32) {
  fn mapToScene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZNK13QGraphicsView10mapToSceneEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QGraphicsView::NewQGraphicsView(QGraphicsScene * scene, QWidget * parent);
impl<'a> /*trait*/ QGraphicsView_NewQGraphicsView for (&'a mut QGraphicsScene, &'a mut QWidget) {
  fn NewQGraphicsView(self) -> QGraphicsView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsViewC1EP14QGraphicsSceneP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsViewC1EP14QGraphicsSceneP7QWidget(qthis, arg0, arg1)};
    let rsthis = QGraphicsView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QGraphicsView::NewQGraphicsView(QWidget * parent);
impl<'a> /*trait*/ QGraphicsView_NewQGraphicsView for (&'a mut QWidget) {
  fn NewQGraphicsView(self) -> QGraphicsView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsViewC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsViewC1EP7QWidget(qthis, arg0)};
    let rsthis = QGraphicsView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QGraphicsView::centerOn(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsView_centerOn for (&'a  QGraphicsItem) {
  fn centerOn(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView8centerOnEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsView8centerOnEPK13QGraphicsItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn viewportTransform<T: QGraphicsView_viewportTransform>(&mut self, value: T) -> i32 {
    value.viewportTransform(self);
    return 1;
  }
}

pub trait QGraphicsView_viewportTransform {
  fn viewportTransform(self, this: &mut QGraphicsView) -> i32;
}

// proto: QTransform QGraphicsView::viewportTransform();
impl<'a> /*trait*/ QGraphicsView_viewportTransform for () {
  fn viewportTransform(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView17viewportTransformEv()};
    unsafe {_ZNK13QGraphicsView17viewportTransformEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn resetCachedContent<T: QGraphicsView_resetCachedContent>(&mut self, value: T) -> i32 {
    value.resetCachedContent(self);
    return 1;
  }
}

pub trait QGraphicsView_resetCachedContent {
  fn resetCachedContent(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::resetCachedContent();
impl<'a> /*trait*/ QGraphicsView_resetCachedContent for () {
  fn resetCachedContent(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView18resetCachedContentEv()};
    unsafe {_ZN13QGraphicsView18resetCachedContentEv()};
    return 1;
  }
}

// proto: QPolygonF QGraphicsView::mapToScene(const QPolygon & polygon);
impl<'a> /*trait*/ QGraphicsView_mapToScene for (&'a  QPolygon) {
  fn mapToScene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneERK8QPolygon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsView10mapToSceneERK8QPolygon(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn ensureVisible<T: QGraphicsView_ensureVisible>(&mut self, value: T) -> i32 {
    value.ensureVisible(self);
    return 1;
  }
}

pub trait QGraphicsView_ensureVisible {
  fn ensureVisible(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::ensureVisible(const QGraphicsItem * item, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsView_ensureVisible for (&'a  QGraphicsItem, i32, i32) {
  fn ensureVisible(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView13ensureVisibleEPK13QGraphicsItemii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN13QGraphicsView13ensureVisibleEPK13QGraphicsItemii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn sceneRect<T: QGraphicsView_sceneRect>(&mut self, value: T) -> i32 {
    value.sceneRect(self);
    return 1;
  }
}

pub trait QGraphicsView_sceneRect {
  fn sceneRect(self, this: &mut QGraphicsView) -> i32;
}

// proto: QRectF QGraphicsView::sceneRect();
impl<'a> /*trait*/ QGraphicsView_sceneRect for () {
  fn sceneRect(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView9sceneRectEv()};
    unsafe {_ZNK13QGraphicsView9sceneRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn scene<T: QGraphicsView_scene>(&mut self, value: T) -> i32 {
    value.scene(self);
    return 1;
  }
}

pub trait QGraphicsView_scene {
  fn scene(self, this: &mut QGraphicsView) -> i32;
}

// proto: QGraphicsScene * QGraphicsView::scene();
impl<'a> /*trait*/ QGraphicsView_scene for () {
  fn scene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView5sceneEv()};
    unsafe {_ZNK13QGraphicsView5sceneEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn sizeHint<T: QGraphicsView_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QGraphicsView_sizeHint {
  fn sizeHint(self, this: &mut QGraphicsView) -> i32;
}

// proto: QSize QGraphicsView::sizeHint();
impl<'a> /*trait*/ QGraphicsView_sizeHint for () {
  fn sizeHint(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView8sizeHintEv()};
    unsafe {_ZNK13QGraphicsView8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn backgroundBrush<T: QGraphicsView_backgroundBrush>(&mut self, value: T) -> i32 {
    value.backgroundBrush(self);
    return 1;
  }
}

pub trait QGraphicsView_backgroundBrush {
  fn backgroundBrush(self, this: &mut QGraphicsView) -> i32;
}

// proto: QBrush QGraphicsView::backgroundBrush();
impl<'a> /*trait*/ QGraphicsView_backgroundBrush for () {
  fn backgroundBrush(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView15backgroundBrushEv()};
    unsafe {_ZNK13QGraphicsView15backgroundBrushEv()};
    return 1;
  }
}

// proto: QPoint QGraphicsView::mapFromScene(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsView_mapFromScene for (f64, f64) {
  fn mapFromScene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZNK13QGraphicsView12mapFromSceneEdd(arg0, arg1)};
    return 1;
  }
}

// proto: void QGraphicsView::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsView_ensureVisible for (&'a  QRectF, i32, i32) {
  fn ensureVisible(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView13ensureVisibleERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN13QGraphicsView13ensureVisibleERK6QRectFii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn items<T: QGraphicsView_items>(&mut self, value: T) -> i32 {
    value.items(self);
    return 1;
  }
}

pub trait QGraphicsView_items {
  fn items(self, this: &mut QGraphicsView) -> i32;
}

// proto: QList<QGraphicsItem *> QGraphicsView::items();
impl<'a> /*trait*/ QGraphicsView_items for () {
  fn items(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView5itemsEv()};
    unsafe {_ZNK13QGraphicsView5itemsEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn transform<T: QGraphicsView_transform>(&mut self, value: T) -> i32 {
    value.transform(self);
    return 1;
  }
}

pub trait QGraphicsView_transform {
  fn transform(self, this: &mut QGraphicsView) -> i32;
}

// proto: QTransform QGraphicsView::transform();
impl<'a> /*trait*/ QGraphicsView_transform for () {
  fn transform(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView9transformEv()};
    unsafe {_ZNK13QGraphicsView9transformEv()};
    return 1;
  }
}

// proto: QList<QGraphicsItem *> QGraphicsView::items(int x, int y);
impl<'a> /*trait*/ QGraphicsView_items for (i32, i32) {
  fn items(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView5itemsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK13QGraphicsView5itemsEii(arg0, arg1)};
    return 1;
  }
}

// proto: void QGraphicsView::centerOn(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsView_centerOn for (f64, f64) {
  fn centerOn(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView8centerOnEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN13QGraphicsView8centerOnEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn rubberBandChanged<T: QGraphicsView_rubberBandChanged>(&mut self, value: T) -> i32 {
    value.rubberBandChanged(self);
    return 1;
  }
}

pub trait QGraphicsView_rubberBandChanged {
  fn rubberBandChanged(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::rubberBandChanged(QRect viewportRect, QPointF fromScenePoint, QPointF toScenePoint);
impl<'a> /*trait*/ QGraphicsView_rubberBandChanged for (QRect, QPointF, QPointF) {
  fn rubberBandChanged(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView17rubberBandChangedE5QRect7QPointFS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsView17rubberBandChangedE5QRect7QPointFS1_(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QGraphicsView::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsView_ensureVisible for (f64, f64, f64, f64, i32, i32) {
  fn ensureVisible(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView13ensureVisibleEddddii()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    unsafe {_ZN13QGraphicsView13ensureVisibleEddddii(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn rotate<T: QGraphicsView_rotate>(&mut self, value: T) -> i32 {
    value.rotate(self);
    return 1;
  }
}

pub trait QGraphicsView_rotate {
  fn rotate(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::rotate(qreal angle);
impl<'a> /*trait*/ QGraphicsView_rotate for (f64) {
  fn rotate(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView6rotateEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN13QGraphicsView6rotateEd(arg0)};
    return 1;
  }
}

// proto: QPolygon QGraphicsView::mapFromScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsView_mapFromScene for (f64, f64, f64, f64) {
  fn mapFromScene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZNK13QGraphicsView12mapFromSceneEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setTransform<T: QGraphicsView_setTransform>(&mut self, value: T) -> i32 {
    value.setTransform(self);
    return 1;
  }
}

pub trait QGraphicsView_setTransform {
  fn setTransform(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::setTransform(const QTransform & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsView_setTransform for (&'a  QTransform, i8) {
  fn setTransform(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView12setTransformERK10QTransformb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN13QGraphicsView12setTransformERK10QTransformb(arg0, arg1)};
    return 1;
  }
}

// proto: void QGraphicsView::setSceneRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsView_setSceneRect for (&'a  QRectF) {
  fn setSceneRect(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView12setSceneRectERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QGraphicsView12setSceneRectERK6QRectF(arg0)};
    return 1;
  }
}

// proto: QPolygon QGraphicsView::mapFromScene(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsView_mapFromScene for (&'a  QPolygonF) {
  fn mapFromScene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneERK9QPolygonF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsView12mapFromSceneERK9QPolygonF(arg0)};
    return 1;
  }
}

// proto: QPoint QGraphicsView::mapFromScene(const QPointF & point);
impl<'a> /*trait*/ QGraphicsView_mapFromScene for (&'a  QPointF) {
  fn mapFromScene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsView12mapFromSceneERK7QPointF(arg0)};
    return 1;
  }
}

// proto: QPainterPath QGraphicsView::mapFromScene(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsView_mapFromScene for (&'a  QPainterPath) {
  fn mapFromScene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneERK12QPainterPath()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsView12mapFromSceneERK12QPainterPath(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn FreeQGraphicsView<T: QGraphicsView_FreeQGraphicsView>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsView(self);
    return 1;
  }
}

pub trait QGraphicsView_FreeQGraphicsView {
  fn FreeQGraphicsView(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::FreeQGraphicsView();
impl<'a> /*trait*/ QGraphicsView_FreeQGraphicsView for () {
  fn FreeQGraphicsView(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsViewD0Ev()};
    unsafe {_ZN13QGraphicsViewD0Ev()};
    return 1;
  }
}

// proto: QList<QGraphicsItem *> QGraphicsView::items(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsView_items for (&'a  QPoint) {
  fn items(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView5itemsERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsView5itemsERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn resetMatrix<T: QGraphicsView_resetMatrix>(&mut self, value: T) -> i32 {
    value.resetMatrix(self);
    return 1;
  }
}

pub trait QGraphicsView_resetMatrix {
  fn resetMatrix(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::resetMatrix();
impl<'a> /*trait*/ QGraphicsView_resetMatrix for () {
  fn resetMatrix(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView11resetMatrixEv()};
    unsafe {_ZN13QGraphicsView11resetMatrixEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn resetTransform<T: QGraphicsView_resetTransform>(&mut self, value: T) -> i32 {
    value.resetTransform(self);
    return 1;
  }
}

pub trait QGraphicsView_resetTransform {
  fn resetTransform(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::resetTransform();
impl<'a> /*trait*/ QGraphicsView_resetTransform for () {
  fn resetTransform(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView14resetTransformEv()};
    unsafe {_ZN13QGraphicsView14resetTransformEv()};
    return 1;
  }
}

// proto: QPainterPath QGraphicsView::mapToScene(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsView_mapToScene for (&'a  QPainterPath) {
  fn mapToScene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneERK12QPainterPath()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QGraphicsView10mapToSceneERK12QPainterPath(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setScene<T: QGraphicsView_setScene>(&mut self, value: T) -> i32 {
    value.setScene(self);
    return 1;
  }
}

pub trait QGraphicsView_setScene {
  fn setScene(self, this: &mut QGraphicsView) -> i32;
}

// proto: void QGraphicsView::setScene(QGraphicsScene * scene);
impl<'a> /*trait*/ QGraphicsView_setScene for (&'a mut QGraphicsScene) {
  fn setScene(self, this: &mut QGraphicsView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView8setSceneEP14QGraphicsScene()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsView8setSceneEP14QGraphicsScene(arg0)};
    return 1;
  }
}

