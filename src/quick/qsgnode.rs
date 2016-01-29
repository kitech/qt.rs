// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qsgnode.h
// dst-file: /src/quick/qsgnode.rs
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
// use super::qsgnode::QSGNode; // 773
use std::ops::Deref;
use super::super::gui::qmatrix4x4::QMatrix4x4; // 771
// use super::qsgnode::QSGBasicGeometryNode; // 773
use super::super::core::qrect::QRectF; // 771
use super::qsgmaterial::QSGMaterial; // 773
use super::qsggeometry::QSGGeometry; // 773
// use super::qsgnode::QSGClipNode; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSGTransformNode_Class_Size() -> c_int;
  // proto:  const QMatrix4x4 & QSGTransformNode::combinedMatrix();
  fn _ZNK16QSGTransformNode14combinedMatrixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGTransformNode::setCombinedMatrix(const QMatrix4x4 & matrix);
  fn _ZN16QSGTransformNode17setCombinedMatrixERK10QMatrix4x4(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMatrix4x4 & QSGTransformNode::matrix();
  fn _ZNK16QSGTransformNode6matrixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGTransformNode::setMatrix(const QMatrix4x4 & matrix);
  fn _ZN16QSGTransformNode9setMatrixERK10QMatrix4x4(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGTransformNode::~QSGTransformNode();
  fn _ZN16QSGTransformNodeD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGTransformNode::QSGTransformNode();
  fn _ZN16QSGTransformNodeC2Ev(qthis: u64 /* *mut c_void*/);
  fn QSGOpacityNode_Class_Size() -> c_int;
  // proto:  void QSGOpacityNode::~QSGOpacityNode();
  fn _ZN14QSGOpacityNodeD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGOpacityNode::QSGOpacityNode();
  fn _ZN14QSGOpacityNodeC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGOpacityNode::setOpacity(qreal opacity);
  fn _ZN14QSGOpacityNode10setOpacityEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  qreal QSGOpacityNode::opacity();
  fn _ZNK14QSGOpacityNode7opacityEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QSGOpacityNode::combinedOpacity();
  fn _ZNK14QSGOpacityNode15combinedOpacityEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QSGOpacityNode::isSubtreeBlocked();
  fn _ZNK14QSGOpacityNode16isSubtreeBlockedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSGOpacityNode::setCombinedOpacity(qreal opacity);
  fn _ZN14QSGOpacityNode18setCombinedOpacityEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  fn QSGClipNode_Class_Size() -> c_int;
  // proto:  void QSGClipNode::setClipRect(const QRectF & );
  fn _ZN11QSGClipNode11setClipRectERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGClipNode::QSGClipNode();
  fn _ZN11QSGClipNodeC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QSGClipNode::isRectangular();
  fn _ZNK11QSGClipNode13isRectangularEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSGClipNode::setIsRectangular(bool rectHint);
  fn _ZN11QSGClipNode16setIsRectangularEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QSGClipNode::~QSGClipNode();
  fn _ZN11QSGClipNodeD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QRectF QSGClipNode::clipRect();
  fn _ZNK11QSGClipNode8clipRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QSGGeometryNode_Class_Size() -> c_int;
  // proto:  QSGMaterial * QSGGeometryNode::opaqueMaterial();
  fn _ZNK15QSGGeometryNode14opaqueMaterialEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGGeometryNode::setInheritedOpacity(qreal opacity);
  fn _ZN15QSGGeometryNode19setInheritedOpacityEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QSGGeometryNode::setMaterial(QSGMaterial * material);
  fn _ZN15QSGGeometryNode11setMaterialEP11QSGMaterial(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QSGGeometryNode::renderOrder();
  fn _ZNK15QSGGeometryNode11renderOrderEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QSGGeometryNode::setRenderOrder(int order);
  fn _ZN15QSGGeometryNode14setRenderOrderEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QSGGeometryNode::~QSGGeometryNode();
  fn _ZN15QSGGeometryNodeD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGGeometryNode::setOpaqueMaterial(QSGMaterial * material);
  fn _ZN15QSGGeometryNode17setOpaqueMaterialEP11QSGMaterial(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSGMaterial * QSGGeometryNode::activeMaterial();
  fn _ZNK15QSGGeometryNode14activeMaterialEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSGMaterial * QSGGeometryNode::material();
  fn _ZNK15QSGGeometryNode8materialEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGGeometryNode::QSGGeometryNode();
  fn _ZN15QSGGeometryNodeC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  qreal QSGGeometryNode::inheritedOpacity();
  fn _ZNK15QSGGeometryNode16inheritedOpacityEv(qthis: u64 /* *mut c_void*/) -> c_double;
  fn QSGNodeVisitor_Class_Size() -> c_int;
  // proto:  void QSGNodeVisitor::~QSGNodeVisitor();
  fn _ZN14QSGNodeVisitorD2Ev(qthis: u64 /* *mut c_void*/);
  fn QSGNode_Class_Size() -> c_int;
  // proto:  void QSGNode::reparentChildNodesTo(QSGNode * newParent);
  fn _ZN7QSGNode20reparentChildNodesToEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGNode::prependChildNode(QSGNode * node);
  fn _ZN7QSGNode16prependChildNodeEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QSGNode::isSubtreeBlocked();
  fn _ZNK7QSGNode16isSubtreeBlockedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSGNode * QSGNode::previousSibling();
  fn _ZNK7QSGNode15previousSiblingEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSGNode * QSGNode::parent();
  fn _ZNK7QSGNode6parentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGNode::QSGNode();
  fn _ZN7QSGNodeC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGNode::removeAllChildNodes();
  fn _ZN7QSGNode19removeAllChildNodesEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSGNode * QSGNode::lastChild();
  fn _ZNK7QSGNode9lastChildEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGNode::clearDirty();
  fn _ZN7QSGNode10clearDirtyEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGNode::appendChildNode(QSGNode * node);
  fn _ZN7QSGNode15appendChildNodeEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGNode::removeChildNode(QSGNode * node);
  fn _ZN7QSGNode15removeChildNodeEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSGNode * QSGNode::childAtIndex(int i);
  fn _ZNK7QSGNode12childAtIndexEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QSGNode::preprocess();
  fn _ZN7QSGNode10preprocessEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSGNode * QSGNode::nextSibling();
  fn _ZNK7QSGNode11nextSiblingEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGNode::insertChildNodeAfter(QSGNode * node, QSGNode * after);
  fn _ZN7QSGNode20insertChildNodeAfterEPS_S0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QSGNode::insertChildNodeBefore(QSGNode * node, QSGNode * before);
  fn _ZN7QSGNode21insertChildNodeBeforeEPS_S0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  int QSGNode::childCount();
  fn _ZNK7QSGNode10childCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QSGNode::~QSGNode();
  fn _ZN7QSGNodeD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QSGNode * QSGNode::firstChild();
  fn _ZNK7QSGNode10firstChildEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QSGBasicGeometryNode_Class_Size() -> c_int;
  // proto:  QSGGeometry * QSGBasicGeometryNode::geometry();
  fn _ZN20QSGBasicGeometryNode8geometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMatrix4x4 * QSGBasicGeometryNode::matrix();
  fn _ZNK20QSGBasicGeometryNode6matrixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QSGClipNode * QSGBasicGeometryNode::clipList();
  fn _ZNK20QSGBasicGeometryNode8clipListEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGBasicGeometryNode::~QSGBasicGeometryNode();
  fn _ZN20QSGBasicGeometryNodeD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGBasicGeometryNode::setGeometry(QSGGeometry * geometry);
  fn _ZN20QSGBasicGeometryNode11setGeometryEP11QSGGeometry(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QSGRootNode_Class_Size() -> c_int;
  // proto:  void QSGRootNode::QSGRootNode();
  fn _ZN11QSGRootNodeC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGRootNode::~QSGRootNode();
  fn _ZN11QSGRootNodeD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QSGTransformNode)=1
#[derive(Default)]
pub struct QSGTransformNode {
  qbase: QSGNode,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QSGOpacityNode)=1
#[derive(Default)]
pub struct QSGOpacityNode {
  qbase: QSGNode,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QSGClipNode)=1
#[derive(Default)]
pub struct QSGClipNode {
  qbase: QSGBasicGeometryNode,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QSGGeometryNode)=1
#[derive(Default)]
pub struct QSGGeometryNode {
  qbase: QSGBasicGeometryNode,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QSGNodeVisitor)=8
#[derive(Default)]
pub struct QSGNodeVisitor {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QSGNode)=1
#[derive(Default)]
pub struct QSGNode {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QSGBasicGeometryNode)=1
#[derive(Default)]
pub struct QSGBasicGeometryNode {
  qbase: QSGNode,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QSGRootNode)=1
#[derive(Default)]
pub struct QSGRootNode {
  qbase: QSGNode,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSGTransformNode {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGTransformNode {
    return QSGTransformNode{qbase: QSGNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGTransformNode {
  type Target = QSGNode;

  fn deref(&self) -> &QSGNode {
    return & self.qbase;
  }
}
impl AsRef<QSGNode> for QSGTransformNode {
  fn as_ref(& self) -> & QSGNode {
    return & self.qbase;
  }
}
  // proto:  const QMatrix4x4 & QSGTransformNode::combinedMatrix();
impl /*struct*/ QSGTransformNode {
  pub fn combinedMatrix<RetType, T: QSGTransformNode_combinedMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.combinedMatrix(self);
    // return 1;
  }
}

pub trait QSGTransformNode_combinedMatrix<RetType> {
  fn combinedMatrix(self , rsthis: & QSGTransformNode) -> RetType;
}

  // proto:  const QMatrix4x4 & QSGTransformNode::combinedMatrix();
impl<'a> /*trait*/ QSGTransformNode_combinedMatrix<QMatrix4x4> for () {
  fn combinedMatrix(self , rsthis: & QSGTransformNode) -> QMatrix4x4 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QSGTransformNode14combinedMatrixEv()};
    let mut ret = unsafe {_ZNK16QSGTransformNode14combinedMatrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix4x4::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGTransformNode::setCombinedMatrix(const QMatrix4x4 & matrix);
impl /*struct*/ QSGTransformNode {
  pub fn setCombinedMatrix<RetType, T: QSGTransformNode_setCombinedMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCombinedMatrix(self);
    // return 1;
  }
}

pub trait QSGTransformNode_setCombinedMatrix<RetType> {
  fn setCombinedMatrix(self , rsthis: & QSGTransformNode) -> RetType;
}

  // proto:  void QSGTransformNode::setCombinedMatrix(const QMatrix4x4 & matrix);
impl<'a> /*trait*/ QSGTransformNode_setCombinedMatrix<()> for (&'a QMatrix4x4) {
  fn setCombinedMatrix(self , rsthis: & QSGTransformNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSGTransformNode17setCombinedMatrixERK10QMatrix4x4()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QSGTransformNode17setCombinedMatrixERK10QMatrix4x4(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMatrix4x4 & QSGTransformNode::matrix();
impl /*struct*/ QSGTransformNode {
  pub fn matrix<RetType, T: QSGTransformNode_matrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.matrix(self);
    // return 1;
  }
}

pub trait QSGTransformNode_matrix<RetType> {
  fn matrix(self , rsthis: & QSGTransformNode) -> RetType;
}

  // proto:  const QMatrix4x4 & QSGTransformNode::matrix();
impl<'a> /*trait*/ QSGTransformNode_matrix<QMatrix4x4> for () {
  fn matrix(self , rsthis: & QSGTransformNode) -> QMatrix4x4 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QSGTransformNode6matrixEv()};
    let mut ret = unsafe {_ZNK16QSGTransformNode6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix4x4::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGTransformNode::setMatrix(const QMatrix4x4 & matrix);
impl /*struct*/ QSGTransformNode {
  pub fn setMatrix<RetType, T: QSGTransformNode_setMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMatrix(self);
    // return 1;
  }
}

pub trait QSGTransformNode_setMatrix<RetType> {
  fn setMatrix(self , rsthis: & QSGTransformNode) -> RetType;
}

  // proto:  void QSGTransformNode::setMatrix(const QMatrix4x4 & matrix);
impl<'a> /*trait*/ QSGTransformNode_setMatrix<()> for (&'a QMatrix4x4) {
  fn setMatrix(self , rsthis: & QSGTransformNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSGTransformNode9setMatrixERK10QMatrix4x4()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QSGTransformNode9setMatrixERK10QMatrix4x4(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGTransformNode::~QSGTransformNode();
impl /*struct*/ QSGTransformNode {
  pub fn free<RetType, T: QSGTransformNode_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGTransformNode_free<RetType> {
  fn free(self , rsthis: & QSGTransformNode) -> RetType;
}

  // proto:  void QSGTransformNode::~QSGTransformNode();
impl<'a> /*trait*/ QSGTransformNode_free<()> for () {
  fn free(self , rsthis: & QSGTransformNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSGTransformNodeD2Ev()};
     unsafe {_ZN16QSGTransformNodeD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSGTransformNode::QSGTransformNode();
impl /*struct*/ QSGTransformNode {
  pub fn new<T: QSGTransformNode_new>(value: T) -> QSGTransformNode {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGTransformNode_new {
  fn new(self) -> QSGTransformNode;
}

  // proto:  void QSGTransformNode::QSGTransformNode();
impl<'a> /*trait*/ QSGTransformNode_new for () {
  fn new(self) -> QSGTransformNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSGTransformNodeC2Ev()};
    let ctysz: c_int = unsafe{QSGTransformNode_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN16QSGTransformNodeC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGTransformNode{qbase: QSGNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSGOpacityNode {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGOpacityNode {
    return QSGOpacityNode{qbase: QSGNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGOpacityNode {
  type Target = QSGNode;

  fn deref(&self) -> &QSGNode {
    return & self.qbase;
  }
}
impl AsRef<QSGNode> for QSGOpacityNode {
  fn as_ref(& self) -> & QSGNode {
    return & self.qbase;
  }
}
  // proto:  void QSGOpacityNode::~QSGOpacityNode();
impl /*struct*/ QSGOpacityNode {
  pub fn free<RetType, T: QSGOpacityNode_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGOpacityNode_free<RetType> {
  fn free(self , rsthis: & QSGOpacityNode) -> RetType;
}

  // proto:  void QSGOpacityNode::~QSGOpacityNode();
impl<'a> /*trait*/ QSGOpacityNode_free<()> for () {
  fn free(self , rsthis: & QSGOpacityNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSGOpacityNodeD2Ev()};
     unsafe {_ZN14QSGOpacityNodeD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSGOpacityNode::QSGOpacityNode();
impl /*struct*/ QSGOpacityNode {
  pub fn new<T: QSGOpacityNode_new>(value: T) -> QSGOpacityNode {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGOpacityNode_new {
  fn new(self) -> QSGOpacityNode;
}

  // proto:  void QSGOpacityNode::QSGOpacityNode();
impl<'a> /*trait*/ QSGOpacityNode_new for () {
  fn new(self) -> QSGOpacityNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSGOpacityNodeC2Ev()};
    let ctysz: c_int = unsafe{QSGOpacityNode_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN14QSGOpacityNodeC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGOpacityNode{qbase: QSGNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSGOpacityNode::setOpacity(qreal opacity);
impl /*struct*/ QSGOpacityNode {
  pub fn setOpacity<RetType, T: QSGOpacityNode_setOpacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpacity(self);
    // return 1;
  }
}

pub trait QSGOpacityNode_setOpacity<RetType> {
  fn setOpacity(self , rsthis: & QSGOpacityNode) -> RetType;
}

  // proto:  void QSGOpacityNode::setOpacity(qreal opacity);
impl<'a> /*trait*/ QSGOpacityNode_setOpacity<()> for (f64) {
  fn setOpacity(self , rsthis: & QSGOpacityNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSGOpacityNode10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QSGOpacityNode10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QSGOpacityNode::opacity();
impl /*struct*/ QSGOpacityNode {
  pub fn opacity<RetType, T: QSGOpacityNode_opacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opacity(self);
    // return 1;
  }
}

pub trait QSGOpacityNode_opacity<RetType> {
  fn opacity(self , rsthis: & QSGOpacityNode) -> RetType;
}

  // proto:  qreal QSGOpacityNode::opacity();
impl<'a> /*trait*/ QSGOpacityNode_opacity<f64> for () {
  fn opacity(self , rsthis: & QSGOpacityNode) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSGOpacityNode7opacityEv()};
    let mut ret = unsafe {_ZNK14QSGOpacityNode7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QSGOpacityNode::combinedOpacity();
impl /*struct*/ QSGOpacityNode {
  pub fn combinedOpacity<RetType, T: QSGOpacityNode_combinedOpacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.combinedOpacity(self);
    // return 1;
  }
}

pub trait QSGOpacityNode_combinedOpacity<RetType> {
  fn combinedOpacity(self , rsthis: & QSGOpacityNode) -> RetType;
}

  // proto:  qreal QSGOpacityNode::combinedOpacity();
impl<'a> /*trait*/ QSGOpacityNode_combinedOpacity<f64> for () {
  fn combinedOpacity(self , rsthis: & QSGOpacityNode) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSGOpacityNode15combinedOpacityEv()};
    let mut ret = unsafe {_ZNK14QSGOpacityNode15combinedOpacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QSGOpacityNode::isSubtreeBlocked();
impl /*struct*/ QSGOpacityNode {
  pub fn isSubtreeBlocked<RetType, T: QSGOpacityNode_isSubtreeBlocked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSubtreeBlocked(self);
    // return 1;
  }
}

pub trait QSGOpacityNode_isSubtreeBlocked<RetType> {
  fn isSubtreeBlocked(self , rsthis: & QSGOpacityNode) -> RetType;
}

  // proto:  bool QSGOpacityNode::isSubtreeBlocked();
impl<'a> /*trait*/ QSGOpacityNode_isSubtreeBlocked<i8> for () {
  fn isSubtreeBlocked(self , rsthis: & QSGOpacityNode) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QSGOpacityNode16isSubtreeBlockedEv()};
    let mut ret = unsafe {_ZNK14QSGOpacityNode16isSubtreeBlockedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSGOpacityNode::setCombinedOpacity(qreal opacity);
impl /*struct*/ QSGOpacityNode {
  pub fn setCombinedOpacity<RetType, T: QSGOpacityNode_setCombinedOpacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCombinedOpacity(self);
    // return 1;
  }
}

pub trait QSGOpacityNode_setCombinedOpacity<RetType> {
  fn setCombinedOpacity(self , rsthis: & QSGOpacityNode) -> RetType;
}

  // proto:  void QSGOpacityNode::setCombinedOpacity(qreal opacity);
impl<'a> /*trait*/ QSGOpacityNode_setCombinedOpacity<()> for (f64) {
  fn setCombinedOpacity(self , rsthis: & QSGOpacityNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSGOpacityNode18setCombinedOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QSGOpacityNode18setCombinedOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSGClipNode {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGClipNode {
    return QSGClipNode{qbase: QSGBasicGeometryNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGClipNode {
  type Target = QSGBasicGeometryNode;

  fn deref(&self) -> &QSGBasicGeometryNode {
    return & self.qbase;
  }
}
impl AsRef<QSGBasicGeometryNode> for QSGClipNode {
  fn as_ref(& self) -> & QSGBasicGeometryNode {
    return & self.qbase;
  }
}
  // proto:  void QSGClipNode::setClipRect(const QRectF & );
impl /*struct*/ QSGClipNode {
  pub fn setClipRect<RetType, T: QSGClipNode_setClipRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setClipRect(self);
    // return 1;
  }
}

pub trait QSGClipNode_setClipRect<RetType> {
  fn setClipRect(self , rsthis: & QSGClipNode) -> RetType;
}

  // proto:  void QSGClipNode::setClipRect(const QRectF & );
impl<'a> /*trait*/ QSGClipNode_setClipRect<()> for (&'a QRectF) {
  fn setClipRect(self , rsthis: & QSGClipNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSGClipNode11setClipRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QSGClipNode11setClipRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGClipNode::QSGClipNode();
impl /*struct*/ QSGClipNode {
  pub fn new<T: QSGClipNode_new>(value: T) -> QSGClipNode {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGClipNode_new {
  fn new(self) -> QSGClipNode;
}

  // proto:  void QSGClipNode::QSGClipNode();
impl<'a> /*trait*/ QSGClipNode_new for () {
  fn new(self) -> QSGClipNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSGClipNodeC2Ev()};
    let ctysz: c_int = unsafe{QSGClipNode_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN11QSGClipNodeC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGClipNode{qbase: QSGBasicGeometryNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QSGClipNode::isRectangular();
impl /*struct*/ QSGClipNode {
  pub fn isRectangular<RetType, T: QSGClipNode_isRectangular<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRectangular(self);
    // return 1;
  }
}

pub trait QSGClipNode_isRectangular<RetType> {
  fn isRectangular(self , rsthis: & QSGClipNode) -> RetType;
}

  // proto:  bool QSGClipNode::isRectangular();
impl<'a> /*trait*/ QSGClipNode_isRectangular<i8> for () {
  fn isRectangular(self , rsthis: & QSGClipNode) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSGClipNode13isRectangularEv()};
    let mut ret = unsafe {_ZNK11QSGClipNode13isRectangularEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSGClipNode::setIsRectangular(bool rectHint);
impl /*struct*/ QSGClipNode {
  pub fn setIsRectangular<RetType, T: QSGClipNode_setIsRectangular<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIsRectangular(self);
    // return 1;
  }
}

pub trait QSGClipNode_setIsRectangular<RetType> {
  fn setIsRectangular(self , rsthis: & QSGClipNode) -> RetType;
}

  // proto:  void QSGClipNode::setIsRectangular(bool rectHint);
impl<'a> /*trait*/ QSGClipNode_setIsRectangular<()> for (i8) {
  fn setIsRectangular(self , rsthis: & QSGClipNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSGClipNode16setIsRectangularEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QSGClipNode16setIsRectangularEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGClipNode::~QSGClipNode();
impl /*struct*/ QSGClipNode {
  pub fn free<RetType, T: QSGClipNode_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGClipNode_free<RetType> {
  fn free(self , rsthis: & QSGClipNode) -> RetType;
}

  // proto:  void QSGClipNode::~QSGClipNode();
impl<'a> /*trait*/ QSGClipNode_free<()> for () {
  fn free(self , rsthis: & QSGClipNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSGClipNodeD2Ev()};
     unsafe {_ZN11QSGClipNodeD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QSGClipNode::clipRect();
impl /*struct*/ QSGClipNode {
  pub fn clipRect<RetType, T: QSGClipNode_clipRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clipRect(self);
    // return 1;
  }
}

pub trait QSGClipNode_clipRect<RetType> {
  fn clipRect(self , rsthis: & QSGClipNode) -> RetType;
}

  // proto:  QRectF QSGClipNode::clipRect();
impl<'a> /*trait*/ QSGClipNode_clipRect<QRectF> for () {
  fn clipRect(self , rsthis: & QSGClipNode) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSGClipNode8clipRectEv()};
    let mut ret = unsafe {_ZNK11QSGClipNode8clipRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSGGeometryNode {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGGeometryNode {
    return QSGGeometryNode{qbase: QSGBasicGeometryNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGGeometryNode {
  type Target = QSGBasicGeometryNode;

  fn deref(&self) -> &QSGBasicGeometryNode {
    return & self.qbase;
  }
}
impl AsRef<QSGBasicGeometryNode> for QSGGeometryNode {
  fn as_ref(& self) -> & QSGBasicGeometryNode {
    return & self.qbase;
  }
}
  // proto:  QSGMaterial * QSGGeometryNode::opaqueMaterial();
impl /*struct*/ QSGGeometryNode {
  pub fn opaqueMaterial<RetType, T: QSGGeometryNode_opaqueMaterial<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueMaterial(self);
    // return 1;
  }
}

pub trait QSGGeometryNode_opaqueMaterial<RetType> {
  fn opaqueMaterial(self , rsthis: & QSGGeometryNode) -> RetType;
}

  // proto:  QSGMaterial * QSGGeometryNode::opaqueMaterial();
impl<'a> /*trait*/ QSGGeometryNode_opaqueMaterial<QSGMaterial> for () {
  fn opaqueMaterial(self , rsthis: & QSGGeometryNode) -> QSGMaterial {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSGGeometryNode14opaqueMaterialEv()};
    let mut ret = unsafe {_ZNK15QSGGeometryNode14opaqueMaterialEv(rsthis.qclsinst)};
    let mut ret1 = QSGMaterial::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGGeometryNode::setInheritedOpacity(qreal opacity);
impl /*struct*/ QSGGeometryNode {
  pub fn setInheritedOpacity<RetType, T: QSGGeometryNode_setInheritedOpacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setInheritedOpacity(self);
    // return 1;
  }
}

pub trait QSGGeometryNode_setInheritedOpacity<RetType> {
  fn setInheritedOpacity(self , rsthis: & QSGGeometryNode) -> RetType;
}

  // proto:  void QSGGeometryNode::setInheritedOpacity(qreal opacity);
impl<'a> /*trait*/ QSGGeometryNode_setInheritedOpacity<()> for (f64) {
  fn setInheritedOpacity(self , rsthis: & QSGGeometryNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSGGeometryNode19setInheritedOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QSGGeometryNode19setInheritedOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGGeometryNode::setMaterial(QSGMaterial * material);
impl /*struct*/ QSGGeometryNode {
  pub fn setMaterial<RetType, T: QSGGeometryNode_setMaterial<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaterial(self);
    // return 1;
  }
}

pub trait QSGGeometryNode_setMaterial<RetType> {
  fn setMaterial(self , rsthis: & QSGGeometryNode) -> RetType;
}

  // proto:  void QSGGeometryNode::setMaterial(QSGMaterial * material);
impl<'a> /*trait*/ QSGGeometryNode_setMaterial<()> for (&'a QSGMaterial) {
  fn setMaterial(self , rsthis: & QSGGeometryNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSGGeometryNode11setMaterialEP11QSGMaterial()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QSGGeometryNode11setMaterialEP11QSGMaterial(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSGGeometryNode::renderOrder();
impl /*struct*/ QSGGeometryNode {
  pub fn renderOrder<RetType, T: QSGGeometryNode_renderOrder<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.renderOrder(self);
    // return 1;
  }
}

pub trait QSGGeometryNode_renderOrder<RetType> {
  fn renderOrder(self , rsthis: & QSGGeometryNode) -> RetType;
}

  // proto:  int QSGGeometryNode::renderOrder();
impl<'a> /*trait*/ QSGGeometryNode_renderOrder<i32> for () {
  fn renderOrder(self , rsthis: & QSGGeometryNode) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSGGeometryNode11renderOrderEv()};
    let mut ret = unsafe {_ZNK15QSGGeometryNode11renderOrderEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSGGeometryNode::setRenderOrder(int order);
impl /*struct*/ QSGGeometryNode {
  pub fn setRenderOrder<RetType, T: QSGGeometryNode_setRenderOrder<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRenderOrder(self);
    // return 1;
  }
}

pub trait QSGGeometryNode_setRenderOrder<RetType> {
  fn setRenderOrder(self , rsthis: & QSGGeometryNode) -> RetType;
}

  // proto:  void QSGGeometryNode::setRenderOrder(int order);
impl<'a> /*trait*/ QSGGeometryNode_setRenderOrder<()> for (i32) {
  fn setRenderOrder(self , rsthis: & QSGGeometryNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSGGeometryNode14setRenderOrderEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QSGGeometryNode14setRenderOrderEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGGeometryNode::~QSGGeometryNode();
impl /*struct*/ QSGGeometryNode {
  pub fn free<RetType, T: QSGGeometryNode_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGGeometryNode_free<RetType> {
  fn free(self , rsthis: & QSGGeometryNode) -> RetType;
}

  // proto:  void QSGGeometryNode::~QSGGeometryNode();
impl<'a> /*trait*/ QSGGeometryNode_free<()> for () {
  fn free(self , rsthis: & QSGGeometryNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSGGeometryNodeD2Ev()};
     unsafe {_ZN15QSGGeometryNodeD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSGGeometryNode::setOpaqueMaterial(QSGMaterial * material);
impl /*struct*/ QSGGeometryNode {
  pub fn setOpaqueMaterial<RetType, T: QSGGeometryNode_setOpaqueMaterial<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpaqueMaterial(self);
    // return 1;
  }
}

pub trait QSGGeometryNode_setOpaqueMaterial<RetType> {
  fn setOpaqueMaterial(self , rsthis: & QSGGeometryNode) -> RetType;
}

  // proto:  void QSGGeometryNode::setOpaqueMaterial(QSGMaterial * material);
impl<'a> /*trait*/ QSGGeometryNode_setOpaqueMaterial<()> for (&'a QSGMaterial) {
  fn setOpaqueMaterial(self , rsthis: & QSGGeometryNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSGGeometryNode17setOpaqueMaterialEP11QSGMaterial()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QSGGeometryNode17setOpaqueMaterialEP11QSGMaterial(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSGMaterial * QSGGeometryNode::activeMaterial();
impl /*struct*/ QSGGeometryNode {
  pub fn activeMaterial<RetType, T: QSGGeometryNode_activeMaterial<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeMaterial(self);
    // return 1;
  }
}

pub trait QSGGeometryNode_activeMaterial<RetType> {
  fn activeMaterial(self , rsthis: & QSGGeometryNode) -> RetType;
}

  // proto:  QSGMaterial * QSGGeometryNode::activeMaterial();
impl<'a> /*trait*/ QSGGeometryNode_activeMaterial<QSGMaterial> for () {
  fn activeMaterial(self , rsthis: & QSGGeometryNode) -> QSGMaterial {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSGGeometryNode14activeMaterialEv()};
    let mut ret = unsafe {_ZNK15QSGGeometryNode14activeMaterialEv(rsthis.qclsinst)};
    let mut ret1 = QSGMaterial::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSGMaterial * QSGGeometryNode::material();
impl /*struct*/ QSGGeometryNode {
  pub fn material<RetType, T: QSGGeometryNode_material<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.material(self);
    // return 1;
  }
}

pub trait QSGGeometryNode_material<RetType> {
  fn material(self , rsthis: & QSGGeometryNode) -> RetType;
}

  // proto:  QSGMaterial * QSGGeometryNode::material();
impl<'a> /*trait*/ QSGGeometryNode_material<QSGMaterial> for () {
  fn material(self , rsthis: & QSGGeometryNode) -> QSGMaterial {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSGGeometryNode8materialEv()};
    let mut ret = unsafe {_ZNK15QSGGeometryNode8materialEv(rsthis.qclsinst)};
    let mut ret1 = QSGMaterial::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGGeometryNode::QSGGeometryNode();
impl /*struct*/ QSGGeometryNode {
  pub fn new<T: QSGGeometryNode_new>(value: T) -> QSGGeometryNode {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGGeometryNode_new {
  fn new(self) -> QSGGeometryNode;
}

  // proto:  void QSGGeometryNode::QSGGeometryNode();
impl<'a> /*trait*/ QSGGeometryNode_new for () {
  fn new(self) -> QSGGeometryNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSGGeometryNodeC2Ev()};
    let ctysz: c_int = unsafe{QSGGeometryNode_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN15QSGGeometryNodeC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGGeometryNode{qbase: QSGBasicGeometryNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QSGGeometryNode::inheritedOpacity();
impl /*struct*/ QSGGeometryNode {
  pub fn inheritedOpacity<RetType, T: QSGGeometryNode_inheritedOpacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.inheritedOpacity(self);
    // return 1;
  }
}

pub trait QSGGeometryNode_inheritedOpacity<RetType> {
  fn inheritedOpacity(self , rsthis: & QSGGeometryNode) -> RetType;
}

  // proto:  qreal QSGGeometryNode::inheritedOpacity();
impl<'a> /*trait*/ QSGGeometryNode_inheritedOpacity<f64> for () {
  fn inheritedOpacity(self , rsthis: & QSGGeometryNode) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSGGeometryNode16inheritedOpacityEv()};
    let mut ret = unsafe {_ZNK15QSGGeometryNode16inheritedOpacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QSGNodeVisitor {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGNodeVisitor {
    return QSGNodeVisitor{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QSGNodeVisitor::~QSGNodeVisitor();
impl /*struct*/ QSGNodeVisitor {
  pub fn free<RetType, T: QSGNodeVisitor_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGNodeVisitor_free<RetType> {
  fn free(self , rsthis: & QSGNodeVisitor) -> RetType;
}

  // proto:  void QSGNodeVisitor::~QSGNodeVisitor();
impl<'a> /*trait*/ QSGNodeVisitor_free<()> for () {
  fn free(self , rsthis: & QSGNodeVisitor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSGNodeVisitorD2Ev()};
     unsafe {_ZN14QSGNodeVisitorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSGNode {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGNode {
    return QSGNode{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QSGNode::reparentChildNodesTo(QSGNode * newParent);
impl /*struct*/ QSGNode {
  pub fn reparentChildNodesTo<RetType, T: QSGNode_reparentChildNodesTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reparentChildNodesTo(self);
    // return 1;
  }
}

pub trait QSGNode_reparentChildNodesTo<RetType> {
  fn reparentChildNodesTo(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  void QSGNode::reparentChildNodesTo(QSGNode * newParent);
impl<'a> /*trait*/ QSGNode_reparentChildNodesTo<()> for (&'a QSGNode) {
  fn reparentChildNodesTo(self , rsthis: & QSGNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSGNode20reparentChildNodesToEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QSGNode20reparentChildNodesToEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGNode::prependChildNode(QSGNode * node);
impl /*struct*/ QSGNode {
  pub fn prependChildNode<RetType, T: QSGNode_prependChildNode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prependChildNode(self);
    // return 1;
  }
}

pub trait QSGNode_prependChildNode<RetType> {
  fn prependChildNode(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  void QSGNode::prependChildNode(QSGNode * node);
impl<'a> /*trait*/ QSGNode_prependChildNode<()> for (&'a QSGNode) {
  fn prependChildNode(self , rsthis: & QSGNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSGNode16prependChildNodeEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QSGNode16prependChildNodeEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSGNode::isSubtreeBlocked();
impl /*struct*/ QSGNode {
  pub fn isSubtreeBlocked<RetType, T: QSGNode_isSubtreeBlocked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSubtreeBlocked(self);
    // return 1;
  }
}

pub trait QSGNode_isSubtreeBlocked<RetType> {
  fn isSubtreeBlocked(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  bool QSGNode::isSubtreeBlocked();
impl<'a> /*trait*/ QSGNode_isSubtreeBlocked<i8> for () {
  fn isSubtreeBlocked(self , rsthis: & QSGNode) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSGNode16isSubtreeBlockedEv()};
    let mut ret = unsafe {_ZNK7QSGNode16isSubtreeBlockedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSGNode * QSGNode::previousSibling();
impl /*struct*/ QSGNode {
  pub fn previousSibling<RetType, T: QSGNode_previousSibling<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.previousSibling(self);
    // return 1;
  }
}

pub trait QSGNode_previousSibling<RetType> {
  fn previousSibling(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  QSGNode * QSGNode::previousSibling();
impl<'a> /*trait*/ QSGNode_previousSibling<QSGNode> for () {
  fn previousSibling(self , rsthis: & QSGNode) -> QSGNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSGNode15previousSiblingEv()};
    let mut ret = unsafe {_ZNK7QSGNode15previousSiblingEv(rsthis.qclsinst)};
    let mut ret1 = QSGNode::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSGNode * QSGNode::parent();
impl /*struct*/ QSGNode {
  pub fn parent<RetType, T: QSGNode_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QSGNode_parent<RetType> {
  fn parent(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  QSGNode * QSGNode::parent();
impl<'a> /*trait*/ QSGNode_parent<QSGNode> for () {
  fn parent(self , rsthis: & QSGNode) -> QSGNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSGNode6parentEv()};
    let mut ret = unsafe {_ZNK7QSGNode6parentEv(rsthis.qclsinst)};
    let mut ret1 = QSGNode::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGNode::QSGNode();
impl /*struct*/ QSGNode {
  pub fn new<T: QSGNode_new>(value: T) -> QSGNode {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGNode_new {
  fn new(self) -> QSGNode;
}

  // proto:  void QSGNode::QSGNode();
impl<'a> /*trait*/ QSGNode_new for () {
  fn new(self) -> QSGNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSGNodeC2Ev()};
    let ctysz: c_int = unsafe{QSGNode_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN7QSGNodeC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGNode{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSGNode::removeAllChildNodes();
impl /*struct*/ QSGNode {
  pub fn removeAllChildNodes<RetType, T: QSGNode_removeAllChildNodes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeAllChildNodes(self);
    // return 1;
  }
}

pub trait QSGNode_removeAllChildNodes<RetType> {
  fn removeAllChildNodes(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  void QSGNode::removeAllChildNodes();
impl<'a> /*trait*/ QSGNode_removeAllChildNodes<()> for () {
  fn removeAllChildNodes(self , rsthis: & QSGNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSGNode19removeAllChildNodesEv()};
     unsafe {_ZN7QSGNode19removeAllChildNodesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSGNode * QSGNode::lastChild();
impl /*struct*/ QSGNode {
  pub fn lastChild<RetType, T: QSGNode_lastChild<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastChild(self);
    // return 1;
  }
}

pub trait QSGNode_lastChild<RetType> {
  fn lastChild(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  QSGNode * QSGNode::lastChild();
impl<'a> /*trait*/ QSGNode_lastChild<QSGNode> for () {
  fn lastChild(self , rsthis: & QSGNode) -> QSGNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSGNode9lastChildEv()};
    let mut ret = unsafe {_ZNK7QSGNode9lastChildEv(rsthis.qclsinst)};
    let mut ret1 = QSGNode::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGNode::clearDirty();
impl /*struct*/ QSGNode {
  pub fn clearDirty<RetType, T: QSGNode_clearDirty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearDirty(self);
    // return 1;
  }
}

pub trait QSGNode_clearDirty<RetType> {
  fn clearDirty(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  void QSGNode::clearDirty();
impl<'a> /*trait*/ QSGNode_clearDirty<()> for () {
  fn clearDirty(self , rsthis: & QSGNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSGNode10clearDirtyEv()};
     unsafe {_ZN7QSGNode10clearDirtyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSGNode::appendChildNode(QSGNode * node);
impl /*struct*/ QSGNode {
  pub fn appendChildNode<RetType, T: QSGNode_appendChildNode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.appendChildNode(self);
    // return 1;
  }
}

pub trait QSGNode_appendChildNode<RetType> {
  fn appendChildNode(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  void QSGNode::appendChildNode(QSGNode * node);
impl<'a> /*trait*/ QSGNode_appendChildNode<()> for (&'a QSGNode) {
  fn appendChildNode(self , rsthis: & QSGNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSGNode15appendChildNodeEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QSGNode15appendChildNodeEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGNode::removeChildNode(QSGNode * node);
impl /*struct*/ QSGNode {
  pub fn removeChildNode<RetType, T: QSGNode_removeChildNode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeChildNode(self);
    // return 1;
  }
}

pub trait QSGNode_removeChildNode<RetType> {
  fn removeChildNode(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  void QSGNode::removeChildNode(QSGNode * node);
impl<'a> /*trait*/ QSGNode_removeChildNode<()> for (&'a QSGNode) {
  fn removeChildNode(self , rsthis: & QSGNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSGNode15removeChildNodeEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QSGNode15removeChildNodeEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSGNode * QSGNode::childAtIndex(int i);
impl /*struct*/ QSGNode {
  pub fn childAtIndex<RetType, T: QSGNode_childAtIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childAtIndex(self);
    // return 1;
  }
}

pub trait QSGNode_childAtIndex<RetType> {
  fn childAtIndex(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  QSGNode * QSGNode::childAtIndex(int i);
impl<'a> /*trait*/ QSGNode_childAtIndex<QSGNode> for (i32) {
  fn childAtIndex(self , rsthis: & QSGNode) -> QSGNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSGNode12childAtIndexEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QSGNode12childAtIndexEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QSGNode::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGNode::preprocess();
impl /*struct*/ QSGNode {
  pub fn preprocess<RetType, T: QSGNode_preprocess<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.preprocess(self);
    // return 1;
  }
}

pub trait QSGNode_preprocess<RetType> {
  fn preprocess(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  void QSGNode::preprocess();
impl<'a> /*trait*/ QSGNode_preprocess<()> for () {
  fn preprocess(self , rsthis: & QSGNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSGNode10preprocessEv()};
     unsafe {_ZN7QSGNode10preprocessEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSGNode * QSGNode::nextSibling();
impl /*struct*/ QSGNode {
  pub fn nextSibling<RetType, T: QSGNode_nextSibling<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nextSibling(self);
    // return 1;
  }
}

pub trait QSGNode_nextSibling<RetType> {
  fn nextSibling(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  QSGNode * QSGNode::nextSibling();
impl<'a> /*trait*/ QSGNode_nextSibling<QSGNode> for () {
  fn nextSibling(self , rsthis: & QSGNode) -> QSGNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSGNode11nextSiblingEv()};
    let mut ret = unsafe {_ZNK7QSGNode11nextSiblingEv(rsthis.qclsinst)};
    let mut ret1 = QSGNode::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGNode::insertChildNodeAfter(QSGNode * node, QSGNode * after);
impl /*struct*/ QSGNode {
  pub fn insertChildNodeAfter<RetType, T: QSGNode_insertChildNodeAfter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertChildNodeAfter(self);
    // return 1;
  }
}

pub trait QSGNode_insertChildNodeAfter<RetType> {
  fn insertChildNodeAfter(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  void QSGNode::insertChildNodeAfter(QSGNode * node, QSGNode * after);
impl<'a> /*trait*/ QSGNode_insertChildNodeAfter<()> for (&'a QSGNode, &'a QSGNode) {
  fn insertChildNodeAfter(self , rsthis: & QSGNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSGNode20insertChildNodeAfterEPS_S0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QSGNode20insertChildNodeAfterEPS_S0_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QSGNode::insertChildNodeBefore(QSGNode * node, QSGNode * before);
impl /*struct*/ QSGNode {
  pub fn insertChildNodeBefore<RetType, T: QSGNode_insertChildNodeBefore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertChildNodeBefore(self);
    // return 1;
  }
}

pub trait QSGNode_insertChildNodeBefore<RetType> {
  fn insertChildNodeBefore(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  void QSGNode::insertChildNodeBefore(QSGNode * node, QSGNode * before);
impl<'a> /*trait*/ QSGNode_insertChildNodeBefore<()> for (&'a QSGNode, &'a QSGNode) {
  fn insertChildNodeBefore(self , rsthis: & QSGNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSGNode21insertChildNodeBeforeEPS_S0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QSGNode21insertChildNodeBeforeEPS_S0_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QSGNode::childCount();
impl /*struct*/ QSGNode {
  pub fn childCount<RetType, T: QSGNode_childCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childCount(self);
    // return 1;
  }
}

pub trait QSGNode_childCount<RetType> {
  fn childCount(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  int QSGNode::childCount();
impl<'a> /*trait*/ QSGNode_childCount<i32> for () {
  fn childCount(self , rsthis: & QSGNode) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSGNode10childCountEv()};
    let mut ret = unsafe {_ZNK7QSGNode10childCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSGNode::~QSGNode();
impl /*struct*/ QSGNode {
  pub fn free<RetType, T: QSGNode_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGNode_free<RetType> {
  fn free(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  void QSGNode::~QSGNode();
impl<'a> /*trait*/ QSGNode_free<()> for () {
  fn free(self , rsthis: & QSGNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSGNodeD2Ev()};
     unsafe {_ZN7QSGNodeD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSGNode * QSGNode::firstChild();
impl /*struct*/ QSGNode {
  pub fn firstChild<RetType, T: QSGNode_firstChild<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.firstChild(self);
    // return 1;
  }
}

pub trait QSGNode_firstChild<RetType> {
  fn firstChild(self , rsthis: & QSGNode) -> RetType;
}

  // proto:  QSGNode * QSGNode::firstChild();
impl<'a> /*trait*/ QSGNode_firstChild<QSGNode> for () {
  fn firstChild(self , rsthis: & QSGNode) -> QSGNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSGNode10firstChildEv()};
    let mut ret = unsafe {_ZNK7QSGNode10firstChildEv(rsthis.qclsinst)};
    let mut ret1 = QSGNode::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSGBasicGeometryNode {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGBasicGeometryNode {
    return QSGBasicGeometryNode{qbase: QSGNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGBasicGeometryNode {
  type Target = QSGNode;

  fn deref(&self) -> &QSGNode {
    return & self.qbase;
  }
}
impl AsRef<QSGNode> for QSGBasicGeometryNode {
  fn as_ref(& self) -> & QSGNode {
    return & self.qbase;
  }
}
  // proto:  QSGGeometry * QSGBasicGeometryNode::geometry();
impl /*struct*/ QSGBasicGeometryNode {
  pub fn geometry<RetType, T: QSGBasicGeometryNode_geometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QSGBasicGeometryNode_geometry<RetType> {
  fn geometry(self , rsthis: & QSGBasicGeometryNode) -> RetType;
}

  // proto:  QSGGeometry * QSGBasicGeometryNode::geometry();
impl<'a> /*trait*/ QSGBasicGeometryNode_geometry<QSGGeometry> for () {
  fn geometry(self , rsthis: & QSGBasicGeometryNode) -> QSGGeometry {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QSGBasicGeometryNode8geometryEv()};
    let mut ret = unsafe {_ZN20QSGBasicGeometryNode8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QSGGeometry::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMatrix4x4 * QSGBasicGeometryNode::matrix();
impl /*struct*/ QSGBasicGeometryNode {
  pub fn matrix<RetType, T: QSGBasicGeometryNode_matrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.matrix(self);
    // return 1;
  }
}

pub trait QSGBasicGeometryNode_matrix<RetType> {
  fn matrix(self , rsthis: & QSGBasicGeometryNode) -> RetType;
}

  // proto:  const QMatrix4x4 * QSGBasicGeometryNode::matrix();
impl<'a> /*trait*/ QSGBasicGeometryNode_matrix<QMatrix4x4> for () {
  fn matrix(self , rsthis: & QSGBasicGeometryNode) -> QMatrix4x4 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QSGBasicGeometryNode6matrixEv()};
    let mut ret = unsafe {_ZNK20QSGBasicGeometryNode6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix4x4::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QSGClipNode * QSGBasicGeometryNode::clipList();
impl /*struct*/ QSGBasicGeometryNode {
  pub fn clipList<RetType, T: QSGBasicGeometryNode_clipList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clipList(self);
    // return 1;
  }
}

pub trait QSGBasicGeometryNode_clipList<RetType> {
  fn clipList(self , rsthis: & QSGBasicGeometryNode) -> RetType;
}

  // proto:  const QSGClipNode * QSGBasicGeometryNode::clipList();
impl<'a> /*trait*/ QSGBasicGeometryNode_clipList<QSGClipNode> for () {
  fn clipList(self , rsthis: & QSGBasicGeometryNode) -> QSGClipNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QSGBasicGeometryNode8clipListEv()};
    let mut ret = unsafe {_ZNK20QSGBasicGeometryNode8clipListEv(rsthis.qclsinst)};
    let mut ret1 = QSGClipNode::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGBasicGeometryNode::~QSGBasicGeometryNode();
impl /*struct*/ QSGBasicGeometryNode {
  pub fn free<RetType, T: QSGBasicGeometryNode_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGBasicGeometryNode_free<RetType> {
  fn free(self , rsthis: & QSGBasicGeometryNode) -> RetType;
}

  // proto:  void QSGBasicGeometryNode::~QSGBasicGeometryNode();
impl<'a> /*trait*/ QSGBasicGeometryNode_free<()> for () {
  fn free(self , rsthis: & QSGBasicGeometryNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QSGBasicGeometryNodeD2Ev()};
     unsafe {_ZN20QSGBasicGeometryNodeD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSGBasicGeometryNode::setGeometry(QSGGeometry * geometry);
impl /*struct*/ QSGBasicGeometryNode {
  pub fn setGeometry<RetType, T: QSGBasicGeometryNode_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QSGBasicGeometryNode_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QSGBasicGeometryNode) -> RetType;
}

  // proto:  void QSGBasicGeometryNode::setGeometry(QSGGeometry * geometry);
impl<'a> /*trait*/ QSGBasicGeometryNode_setGeometry<()> for (&'a QSGGeometry) {
  fn setGeometry(self , rsthis: & QSGBasicGeometryNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QSGBasicGeometryNode11setGeometryEP11QSGGeometry()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QSGBasicGeometryNode11setGeometryEP11QSGGeometry(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSGRootNode {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGRootNode {
    return QSGRootNode{qbase: QSGNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGRootNode {
  type Target = QSGNode;

  fn deref(&self) -> &QSGNode {
    return & self.qbase;
  }
}
impl AsRef<QSGNode> for QSGRootNode {
  fn as_ref(& self) -> & QSGNode {
    return & self.qbase;
  }
}
  // proto:  void QSGRootNode::QSGRootNode();
impl /*struct*/ QSGRootNode {
  pub fn new<T: QSGRootNode_new>(value: T) -> QSGRootNode {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGRootNode_new {
  fn new(self) -> QSGRootNode;
}

  // proto:  void QSGRootNode::QSGRootNode();
impl<'a> /*trait*/ QSGRootNode_new for () {
  fn new(self) -> QSGRootNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSGRootNodeC2Ev()};
    let ctysz: c_int = unsafe{QSGRootNode_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN11QSGRootNodeC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGRootNode{qbase: QSGNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSGRootNode::~QSGRootNode();
impl /*struct*/ QSGRootNode {
  pub fn free<RetType, T: QSGRootNode_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGRootNode_free<RetType> {
  fn free(self , rsthis: & QSGRootNode) -> RetType;
}

  // proto:  void QSGRootNode::~QSGRootNode();
impl<'a> /*trait*/ QSGRootNode_free<()> for () {
  fn free(self , rsthis: & QSGRootNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSGRootNodeD2Ev()};
     unsafe {_ZN11QSGRootNodeD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

