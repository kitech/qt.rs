
// mod ::widgets::QUndoStack
// package qtwidgets
// /usr/include/qt/QtWidgets/qundostack.h
// #include <qundostack.h>
// #include <QtWidgets>

//  header block end

//  ext block begin

/*
#include <stdlib.h>
// extern C begin: 0
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin
pub struct QUndoStackList {
    pub qclsinst: usize /* *mut c_void*/,
}
// QList<T> & operator=(const QList<T> &)
impl QUndoStackList {
pub fn Operator_equal_0(&self) -> usize {
    // QUndoStackList_operator_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_operator_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator=(QList<T> &&)
impl QUndoStackList {
pub fn Operator_equal_1(&self) -> usize {
    // QUndoStackList_operator_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_operator_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void swap(QList<T> &)
impl QUndoStackList {
pub fn Swap_0(&self) -> (/*void*/) {
    // QUndoStackList_swap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_swap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool operator==(const QList<T> &)
impl QUndoStackList {
pub fn Operator_equal_equal_0(&self) -> bool {
    // QUndoStackList_operator_equal_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_operator_equal_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool operator!=(const QList<T> &)
impl QUndoStackList {
pub fn Operator_not_equal_0(&self) -> bool {
    // QUndoStackList_operator_not_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_operator_not_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int size()
impl QUndoStackList {
pub fn Size_0(&self) -> i32 {
    // QUndoStackList_size_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_size_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// void detach()
impl QUndoStackList {
pub fn Detach_0(&self) -> (/*void*/) {
    // QUndoStackList_detach_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_detach_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detachShared()
impl QUndoStackList {
pub fn DetachShared_0(&self) -> (/*void*/) {
    // QUndoStackList_detachShared_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_detachShared_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isDetached()
impl QUndoStackList {
pub fn IsDetached_0(&self) -> bool {
    // QUndoStackList_isDetached_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_isDetached_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void setSharable(bool)
impl QUndoStackList {
pub fn SetSharable_0(&self) -> (/*void*/) {
    // QUndoStackList_setSharable_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_setSharable_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isSharedWith(const QList<T> &)
impl QUndoStackList {
pub fn IsSharedWith_0(&self) -> bool {
    // QUndoStackList_isSharedWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_isSharedWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool isEmpty()
impl QUndoStackList {
pub fn IsEmpty_0(&self) -> bool {
    // QUndoStackList_isEmpty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_isEmpty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void clear()
impl QUndoStackList {
pub fn Clear_0(&self) -> (/*void*/) {
    // QUndoStackList_clear_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_clear_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// const T & at(int)
impl QUndoStackList {
pub fn At_0(&self) -> usize {
    // QUndoStackList_at_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_at_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & operator[](int)
impl QUndoStackList {
pub fn Operator_get_index_0(&self) -> usize {
    // QUndoStackList_operator_get_index_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_operator_get_index_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & operator[](int)
impl QUndoStackList {
pub fn Operator_get_index_1(&self) -> usize {
    // QUndoStackList_operator_get_index_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_operator_get_index_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void reserve(int)
impl QUndoStackList {
pub fn Reserve_0(&self) -> (/*void*/) {
    // QUndoStackList_reserve_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_reserve_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const T &)
impl QUndoStackList {
pub fn Append_0(&self) -> (/*void*/) {
    // QUndoStackList_append_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_append_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const QList<T> &)
impl QUndoStackList {
pub fn Append_1(&self) -> (/*void*/) {
    // QUndoStackList_append_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_append_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void prepend(const T &)
impl QUndoStackList {
pub fn Prepend_0(&self) -> (/*void*/) {
    // QUndoStackList_prepend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_prepend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void insert(int, const T &)
impl QUndoStackList {
pub fn Insert_0(&self) -> (/*void*/) {
    // QUndoStackList_insert_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_insert_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void replace(int, const T &)
impl QUndoStackList {
pub fn Replace_0(&self) -> (/*void*/) {
    // QUndoStackList_replace_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_replace_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeAt(int)
impl QUndoStackList {
pub fn RemoveAt_0(&self) -> (/*void*/) {
    // QUndoStackList_removeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_removeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int removeAll(const T &)
impl QUndoStackList {
pub fn RemoveAll_0(&self) -> i32 {
    // QUndoStackList_removeAll_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_removeAll_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool removeOne(const T &)
impl QUndoStackList {
pub fn RemoveOne_0(&self) -> bool {
    // QUndoStackList_removeOne_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_removeOne_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// T takeAt(int)
impl QUndoStackList {
pub fn TakeAt_0(&self) -> usize {
    // QUndoStackList_takeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_takeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeFirst()
impl QUndoStackList {
pub fn TakeFirst_0(&self) -> usize {
    // QUndoStackList_takeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_takeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeLast()
impl QUndoStackList {
pub fn TakeLast_0(&self) -> usize {
    // QUndoStackList_takeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_takeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void move(int, int)
impl QUndoStackList {
pub fn Move_0(&self) -> (/*void*/) {
    // QUndoStackList_move_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_move_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void swap(int, int)
impl QUndoStackList {
pub fn Swap_1(&self) -> (/*void*/) {
    // QUndoStackList_swap_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_swap_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int indexOf(const T &, int)
impl QUndoStackList {
pub fn IndexOf_0(&self) -> i32 {
    // QUndoStackList_indexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_indexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int lastIndexOf(const T &, int)
impl QUndoStackList {
pub fn LastIndexOf_0(&self) -> i32 {
    // QUndoStackList_lastIndexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_lastIndexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool contains(const T &)
impl QUndoStackList {
pub fn Contains_0(&self) -> bool {
    // QUndoStackList_contains_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_contains_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count(const T &)
impl QUndoStackList {
pub fn Count_0(&self) -> i32 {
    // QUndoStackList_count_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_count_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QList::iterator begin()
impl QUndoStackList {
pub fn Begin_0(&self) -> usize {
    // QUndoStackList_begin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_begin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator begin()
impl QUndoStackList {
pub fn Begin_1(&self) -> usize {
    // QUndoStackList_begin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_begin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cbegin()
impl QUndoStackList {
pub fn Cbegin_0(&self) -> usize {
    // QUndoStackList_cbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_cbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constBegin()
impl QUndoStackList {
pub fn ConstBegin_0(&self) -> usize {
    // QUndoStackList_constBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_constBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator end()
impl QUndoStackList {
pub fn End_0(&self) -> usize {
    // QUndoStackList_end_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_end_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator end()
impl QUndoStackList {
pub fn End_1(&self) -> usize {
    // QUndoStackList_end_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_end_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cend()
impl QUndoStackList {
pub fn Cend_0(&self) -> usize {
    // QUndoStackList_cend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_cend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constEnd()
impl QUndoStackList {
pub fn ConstEnd_0(&self) -> usize {
    // QUndoStackList_constEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_constEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rbegin()
impl QUndoStackList {
pub fn Rbegin_0(&self) -> usize {
    // QUndoStackList_rbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_rbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rend()
impl QUndoStackList {
pub fn Rend_0(&self) -> usize {
    // QUndoStackList_rend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_rend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rbegin()
impl QUndoStackList {
pub fn Rbegin_1(&self) -> usize {
    // QUndoStackList_rbegin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_rbegin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rend()
impl QUndoStackList {
pub fn Rend_1(&self) -> usize {
    // QUndoStackList_rend_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_rend_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crbegin()
impl QUndoStackList {
pub fn Crbegin_0(&self) -> usize {
    // QUndoStackList_crbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_crbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crend()
impl QUndoStackList {
pub fn Crend_0(&self) -> usize {
    // QUndoStackList_crend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_crend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator insert(QList::iterator, const T &)
impl QUndoStackList {
pub fn Insert_1(&self) -> usize {
    // QUndoStackList_insert_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_insert_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator)
impl QUndoStackList {
pub fn Erase_0(&self) -> usize {
    // QUndoStackList_erase_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_erase_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator, QList::iterator)
impl QUndoStackList {
pub fn Erase_1(&self) -> usize {
    // QUndoStackList_erase_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_erase_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count()
impl QUndoStackList {
pub fn Count_1(&self) -> i32 {
    // QUndoStackList_count_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_count_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int length()
impl QUndoStackList {
pub fn Length_0(&self) -> i32 {
    // QUndoStackList_length_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_length_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// T & first()
impl QUndoStackList {
pub fn First_0(&self) -> usize {
    // QUndoStackList_first_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_first_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constFirst()
impl QUndoStackList {
pub fn ConstFirst_0(&self) -> usize {
    // QUndoStackList_constFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_constFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & first()
impl QUndoStackList {
pub fn First_1(&self) -> usize {
    // QUndoStackList_first_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_first_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & last()
impl QUndoStackList {
pub fn Last_0(&self) -> usize {
    // QUndoStackList_last_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_last_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & last()
impl QUndoStackList {
pub fn Last_1(&self) -> usize {
    // QUndoStackList_last_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_last_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constLast()
impl QUndoStackList {
pub fn ConstLast_0(&self) -> usize {
    // QUndoStackList_constLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_constLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void removeFirst()
impl QUndoStackList {
pub fn RemoveFirst_0(&self) -> (/*void*/) {
    // QUndoStackList_removeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_removeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeLast()
impl QUndoStackList {
pub fn RemoveLast_0(&self) -> (/*void*/) {
    // QUndoStackList_removeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_removeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool startsWith(const T &)
impl QUndoStackList {
pub fn StartsWith_0(&self) -> bool {
    // QUndoStackList_startsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_startsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool endsWith(const T &)
impl QUndoStackList {
pub fn EndsWith_0(&self) -> bool {
    // QUndoStackList_endsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_endsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> mid(int, int)
impl QUndoStackList {
pub fn Mid_0(&self) -> usize {
    // QUndoStackList_mid_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_mid_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int)
impl QUndoStackList {
pub fn Value_0(&self) -> usize {
    // QUndoStackList_value_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_value_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int, const T &)
impl QUndoStackList {
pub fn Value_1(&self) -> usize {
    // QUndoStackList_value_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_value_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void push_back(const T &)
impl QUndoStackList {
pub fn Push_back_0(&self) -> (/*void*/) {
    // QUndoStackList_push_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_push_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void push_front(const T &)
impl QUndoStackList {
pub fn Push_front_0(&self) -> (/*void*/) {
    // QUndoStackList_push_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_push_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// T & front()
impl QUndoStackList {
pub fn Front_0(&self) -> usize {
    // QUndoStackList_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & front()
impl QUndoStackList {
pub fn Front_1(&self) -> usize {
    // QUndoStackList_front_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_front_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & back()
impl QUndoStackList {
pub fn Back_0(&self) -> usize {
    // QUndoStackList_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & back()
impl QUndoStackList {
pub fn Back_1(&self) -> usize {
    // QUndoStackList_back_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_back_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void pop_front()
impl QUndoStackList {
pub fn Pop_front_0(&self) -> (/*void*/) {
    // QUndoStackList_pop_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_pop_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void pop_back()
impl QUndoStackList {
pub fn Pop_back_0(&self) -> (/*void*/) {
    // QUndoStackList_pop_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_pop_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool empty()
impl QUndoStackList {
pub fn Empty_0(&self) -> bool {
    // QUndoStackList_empty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_empty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> & operator+=(const QList<T> &)
impl QUndoStackList {
pub fn Operator_add_equal_0(&self) -> usize {
    // QUndoStackList_operator_add_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_operator_add_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> operator+(const QList<T> &)
impl QUndoStackList {
pub fn Operator_add_0(&self) -> usize {
    // QUndoStackList_operator_add_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_operator_add_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator+=(const T &)
impl QUndoStackList {
pub fn Operator_add_equal_1(&self) -> usize {
    // QUndoStackList_operator_add_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_operator_add_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const T &)
impl QUndoStackList {
pub fn Operator_left_shift_0(&self) -> usize {
    // QUndoStackList_operator_left_shift_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_operator_left_shift_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const QList<T> &)
impl QUndoStackList {
pub fn Operator_left_shift_1(&self) -> usize {
    // QUndoStackList_operator_left_shift_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_operator_left_shift_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QVector<T> toVector()
impl QUndoStackList {
pub fn ToVector_0(&self) -> usize {
    // QUndoStackList_toVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_toVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> toSet()
impl QUndoStackList {
pub fn ToSet_0(&self) -> usize {
    // QUndoStackList_toSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_toSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromVector(const QVector<T> &)
impl QUndoStackList {
pub fn FromVector_0(&self) -> usize {
    // QUndoStackList_fromVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_fromVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromSet(const QSet<T> &)
impl QUndoStackList {
pub fn FromSet_0(&self) -> usize {
    // QUndoStackList_fromSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_fromSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromStdList(const std::list<T> &)
impl QUndoStackList {
pub fn FromStdList_0(&self) -> usize {
    // QUndoStackList_fromStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_fromStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// std::list<T> toStdList()
impl QUndoStackList {
pub fn ToStdList_0(&self) -> i32 {
    // QUndoStackList_toStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_toStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :i32 = Default::default();
    return dret;
  }
}
// QList::Node * detach_helper_grow(int, int)
impl QUndoStackList {
pub fn Detach_helper_grow_0(&self) -> usize {
    // QUndoStackList_detach_helper_grow_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_detach_helper_grow_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void detach_helper(int)
impl QUndoStackList {
pub fn Detach_helper_0(&self) -> (/*void*/) {
    // QUndoStackList_detach_helper_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_detach_helper_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detach_helper()
impl QUndoStackList {
pub fn Detach_helper_1(&self) -> (/*void*/) {
    // QUndoStackList_detach_helper_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_detach_helper_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void dealloc(QListData::Data *)
impl QUndoStackList {
pub fn Dealloc_0(&self) -> (/*void*/) {
    // QUndoStackList_dealloc_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_dealloc_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_construct(QList::Node *, const T &)
impl QUndoStackList {
pub fn Node_construct_0(&self) -> (/*void*/) {
    // QUndoStackList_node_construct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_node_construct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *)
impl QUndoStackList {
pub fn Node_destruct_0(&self) -> (/*void*/) {
    // QUndoStackList_node_destruct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_node_destruct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_copy(QList::Node *, QList::Node *, QList::Node *)
impl QUndoStackList {
pub fn Node_copy_0(&self) -> (/*void*/) {
    // QUndoStackList_node_copy_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_node_copy_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *, QList::Node *)
impl QUndoStackList {
pub fn Node_destruct_1(&self) -> (/*void*/) {
    // QUndoStackList_node_destruct_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_node_destruct_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isValidIterator(const QList::iterator &)
impl QUndoStackList {
pub fn IsValidIterator_0(&self) -> bool {
    // QUndoStackList_isValidIterator_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_isValidIterator_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::NotArrayCompatibleLayout)
impl QUndoStackList {
pub fn Op_eq_impl_0(&self) -> bool {
    // QUndoStackList_op_eq_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_op_eq_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::ArrayCompatibleLayout)
impl QUndoStackList {
pub fn Op_eq_impl_1(&self) -> bool {
    // QUndoStackList_op_eq_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_op_eq_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QUndoStackList {
pub fn Contains_impl_0(&self) -> bool {
    // QUndoStackList_contains_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_contains_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::ArrayCompatibleLayout)
impl QUndoStackList {
pub fn Contains_impl_1(&self) -> bool {
    // QUndoStackList_contains_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_contains_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QUndoStackList {
pub fn Count_impl_0(&self) -> i32 {
    // QUndoStackList_count_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_count_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int count_impl(const T &, QListData::ArrayCompatibleLayout)
impl QUndoStackList {
pub fn Count_impl_1(&self) -> i32 {
    // QUndoStackList_count_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QUndoStackList_count_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
//  body block end
